//! Extraction Module
//!
//! This module provides functionality for extracting embedded data from files.

use crate::encryption::{Algorithm, CryptoConfig};
use crate::pdf::PdfHandler;
use crate::Error;
use crate::Result;
use image;
use image::DynamicImage;
use image::ImageBuffer;
use image::Rgba;
use jpeg_decoder::{Decoder, PixelFormat};
use std::fs::File;
use std::io::BufReader;

/// Configuration for data extraction
#[derive(Debug)]
pub struct ExtractConfig {
    /// Path to the file containing embedded data
    pub input_path: String,
    /// Optional decryption configuration
    pub encryption: Option<CryptoConfig>,
    /// Additional extraction parameters
    pub parameters: Option<std::collections::HashMap<String, String>>,
}

/// Extracts embedded data from a PDF file
pub fn extract_data(config: ExtractConfig) -> Result<Vec<u8>> {
    // TODO: Detect file type and call appropriate function
    extract_from_pdf(config)
}

/// Extracts embedded data from a PDF file
pub fn extract_from_pdf(config: ExtractConfig) -> Result<Vec<u8>> {
    // Initialize PDF handler
    let handler = PdfHandler::new(&config.input_path)?;

    // Extract raw data
    let raw_data = handler.extract_data()?;

    // Decrypt if needed
    if let Some(crypto_config) = config.encryption {
        match crypto_config.algorithm {
            Algorithm::Aes256 => crate::encryption::decrypt(&raw_data, &crypto_config),
            Algorithm::ChaCha20 => crate::encryption::decrypt(&raw_data, &crypto_config),
            Algorithm::Rsa => crate::encryption::decrypt(&raw_data, &crypto_config),
        }
    } else {
        Ok(raw_data)
    }
}

/// Extracts embedded data from a PNG image
pub fn extract_from_png(config: ExtractConfig) -> Result<Vec<u8>> {
    // Load the image
    let img = image::open(&config.input_path)
        .map_err(|e| Error::InvalidInput(format!("Failed to open image: {}", e)))?;

    // Get bit depth from parameters if provided, default to 1
    let bit_depth = if let Some(params) = &config.parameters {
        params
            .get("bit_depth")
            .and_then(|v| v.parse::<u8>().ok())
            .unwrap_or(1)
    } else {
        1 // Default bit depth
    };

    // Validate bit depth
    if !(1..=4).contains(&bit_depth) {
        return Err(Error::InvalidInput(format!(
            "Bit depth must be between 1 and 4, got {}",
            bit_depth
        )));
    }

    // Extract raw data from the image
    let raw_data = extract_from_image(&img, bit_depth)?;

    // Decrypt if needed
    if let Some(crypto_config) = config.encryption {
        match crypto_config.algorithm {
            Algorithm::Aes256 => crate::encryption::decrypt(&raw_data, &crypto_config),
            Algorithm::ChaCha20 => crate::encryption::decrypt(&raw_data, &crypto_config),
            Algorithm::Rsa => crate::encryption::decrypt(&raw_data, &crypto_config),
        }
    } else {
        Ok(raw_data)
    }
}

/// Extracts data from an image using LSB steganography
fn extract_from_image(image: &DynamicImage, bit_depth: u8) -> Result<Vec<u8>> {
    let buffer = image.to_rgba8();
    let (width, height) = buffer.dimensions();

    // First extract the length (4 bytes at the beginning)
    let mut len_bytes = [0u8; 4];
    extract_bytes(&buffer, 0, &mut len_bytes, bit_depth)?;

    let data_len = u32::from_be_bytes(len_bytes) as usize;

    // Check if the image has enough capacity
    let max_bytes = (width * height * bit_depth as u32) / 8;
    if data_len as u32 > max_bytes {
        return Err(Error::InvalidData(format!(
            "Data length ({}) exceeds image capacity ({})",
            data_len, max_bytes
        )));
    }

    // Extract actual data
    let mut data = vec![0u8; data_len];
    extract_bytes(&buffer, 32, &mut data, bit_depth)?;

    Ok(data)
}

/// Helper function to extract bytes from image
fn extract_bytes(
    buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    start_bit: usize,
    data: &mut [u8],
    bit_depth: u8,
) -> Result<()> {
    let mut bit_index = start_bit;
    let bit_depth = bit_depth as usize;

    for byte in data.iter_mut() {
        let mut new_byte = 0u8;

        for bit in 0..8 {
            let x = ((bit_index + bit) / 8) as u32 % buffer.width();
            let y = ((bit_index + bit) / 8) as u32 / buffer.width();

            let pixel = buffer.get_pixel(x, y);
            let color_index = (bit_index + bit) % 3;

            // Get LSB bits based on bit_depth
            let bit_mask = ((1 << bit_depth) - 1) as u8;
            let extracted_bits = pixel[color_index] & bit_mask;

            // Set the corresponding bit in the byte
            new_byte |= extracted_bits << (7 - bit);
        }

        *byte = new_byte;
        bit_index += 8;
    }

    Ok(())
}

/// Extracts embedded data from a JPG image using DCT coefficient technique
pub fn extract_from_jpg(config: ExtractConfig) -> Result<Vec<u8>> {
    // Open the input JPEG file
    let file = File::open(&config.input_path)
        .map_err(|e| Error::Io(format!("Failed to open JPEG file: {}", e)))?;

    // Create a decoder to read the JPEG metadata
    let mut decoder = Decoder::new(BufReader::new(file));

    // Read info without decoding the image
    decoder
        .read_info()
        .map_err(|e| Error::InvalidInput(format!("Failed to read JPEG info: {}", e)))?;

    // Get the metadata
    let jpeg_info = match decoder.info() {
        Some(info) => info,
        None => {
            return Err(Error::InvalidInput(
                "Failed to get JPEG information".to_string(),
            ))
        }
    };

    // Decode the image
    let pixels = decoder
        .decode()
        .map_err(|e| Error::InvalidInput(format!("Failed to decode JPEG image: {}", e)))?;

    // Check the pixel format
    let (width, height) = (jpeg_info.width as usize, jpeg_info.height as usize);
    let pixel_format = jpeg_info.pixel_format;

    println!(
        "DEBUG: Image dimensions: {}x{}, pixel format: {:?}",
        width, height, pixel_format
    );

    // Only support RGB24 and CMYK32 formats
    if pixel_format != PixelFormat::RGB24 && pixel_format != PixelFormat::CMYK32 {
        return Err(Error::InvalidInput(format!(
            "Unsupported pixel format: {:?}, only RGB24 and CMYK32 are supported",
            pixel_format
        )));
    }

    // Determine number of color channels
    let channels = match pixel_format {
        PixelFormat::RGB24 => 3,
        PixelFormat::CMYK32 => 4,
        _ => unreachable!(), // We already checked for RGB24 and CMYK32
    };

    println!("DEBUG: Using {} channels", channels);

    // Calculate the maximum data we can extract
    let max_blocks_x = width / 8;
    let max_blocks_y = height / 8;
    let capacity_bytes = (max_blocks_x * max_blocks_y) / 8;

    println!(
        "DEBUG: Max blocks: {}x{}, capacity: {} bytes",
        max_blocks_x, max_blocks_y, capacity_bytes
    );

    // Extract bits from the blue channel average of every 8x8 block
    let mut bits = Vec::with_capacity(max_blocks_x * max_blocks_y);

    for by in 0..max_blocks_y {
        for bx in 0..max_blocks_x {
            // Calculate the average blue value for this 8x8 block
            let mut blue_sum = 0;
            let mut pixel_count = 0;

            for y in by * 8..(by + 1) * 8 {
                for x in bx * 8..(bx + 1) * 8 {
                    if y < height && x < width {
                        let pixel_pos = (y * width + x) * channels + 2; // +2 for blue channel
                        if pixel_pos < pixels.len() {
                            blue_sum += pixels[pixel_pos] as u32;
                            pixel_count += 1;
                        }
                    }
                }
            }

            if pixel_count > 0 {
                // Calculate average
                let avg_blue = (blue_sum / pixel_count) as u8;

                // Extract bit based on parity (odd = 1, even = 0)
                bits.push(avg_blue % 2 == 1);
            }
        }
    }

    println!("DEBUG: Extracted {} bits", bits.len());

    // Convert bits to bytes
    let raw_bytes = bits_to_bytes(&bits);

    println!("DEBUG: Converted to {} bytes", raw_bytes.len());

    // Print the first 16 bytes for debugging
    if raw_bytes.len() >= 16 {
        println!("DEBUG: First 16 bytes: {:?}", &raw_bytes[0..16]);
    } else {
        println!("DEBUG: All bytes: {:?}", &raw_bytes);
    }

    // We need at least 4 bytes for the length prefix
    if raw_bytes.len() < 4 {
        return Err(Error::InvalidData(
            "Not enough data in the image".to_string(),
        ));
    }

    // Extract the length prefix (first 4 bytes)
    let mut length_bytes = [0u8; 4];
    length_bytes.copy_from_slice(&raw_bytes[0..4]);
    let data_length = u32::from_be_bytes(length_bytes) as usize;

    println!("DEBUG: Extracted length prefix: {} bytes", data_length);

    // Validate the length
    if data_length + 4 > raw_bytes.len() || data_length > capacity_bytes {
        return Err(Error::InvalidData(format!(
            "Invalid data length: {}. Max capacity: {}",
            data_length,
            capacity_bytes - 4
        )));
    }

    // Extract the error-corrected data
    let error_corrected_data = raw_bytes[4..4 + data_length].to_vec();

    println!(
        "DEBUG: Extracted {} bytes of error-corrected data",
        error_corrected_data.len()
    );

    // Apply Reed-Solomon error correction decoding to recover the original data
    let actual_data = match crate::error_correction::decode_reed_solomon(&error_corrected_data) {
        Ok(data) => data,
        Err(e) => {
            println!("DEBUG: Reed-Solomon error correction failed: {}", e);
            // Try the simpler parity-based correction as a fallback
            match crate::error_correction::decode(&error_corrected_data) {
                Ok(data) => data,
                Err(_) => {
                    println!("DEBUG: All error correction failed, using raw data");
                    // If all error correction fails, return the raw data as a last resort
                    error_corrected_data
                }
            }
        }
    };

    println!("DEBUG: After error correction: {} bytes", actual_data.len());

    // Decrypt if necessary
    if let Some(crypto_config) = &config.encryption {
        crate::encryption::decrypt(&actual_data, crypto_config)
    } else {
        Ok(actual_data)
    }
}

// Helper function to convert bits to bytes
fn bits_to_bytes(bits: &[bool]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(bits.len() / 8 + 1);

    for chunk in bits.chunks(8) {
        let mut byte = 0u8;
        for (i, &bit) in chunk.iter().enumerate() {
            if i < 8 && bit {
                byte |= 1 << (7 - i);
            }
        }
        bytes.push(byte);
    }

    bytes
}

/// Extracts embedded data from a WAV audio file
pub fn extract_from_wav(_config: ExtractConfig) -> Result<Vec<u8>> {
    // TODO: Implement WAV steganography extraction
    Err(Error::NotImplemented(
        "WAV extraction not yet implemented".into(),
    ))
}

/// Extracts embedded data from an MP3 audio file
pub fn extract_from_mp3(_config: ExtractConfig) -> Result<Vec<u8>> {
    // TODO: Implement MP3 steganography extraction
    Err(Error::NotImplemented(
        "MP3 extraction not yet implemented".into(),
    ))
}

/// Extracts embedded data from an MP4 video file
pub fn extract_from_mp4(_config: ExtractConfig) -> Result<Vec<u8>> {
    // TODO: Implement MP4 steganography extraction
    Err(Error::NotImplemented(
        "MP4 extraction not yet implemented".into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdf::PdfHandler;
    use crate::Error;
    use lopdf::{dictionary, Document, Object};
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn test_extract_data() -> Result<()> {
        let dir = tempdir().map_err(|e| Error::Io(e.to_string()))?;
        let input_path = dir.path().join("test.pdf");
        let output_path = dir.path().join("embedded.pdf");

        // Create a properly initialized test PDF
        let mut doc = Document::new();

        // Create a simple PDF structure
        let pages_id = doc.new_object_id();
        let page_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
        });

        let pages = dictionary! {
            "Type" => "Pages",
            "Kids" => vec![Object::Reference(page_id)],
            "Count" => 1,
            "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
        };
        doc.objects.insert(pages_id, Object::Dictionary(pages));

        let catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });
        doc.trailer.set("Root", catalog_id);

        doc.save(&input_path)
            .map_err(|e| Error::Io(e.to_string()))?;

        // Now use PdfHandler to embed data with integrity checking
        let test_data = b"Test data";
        let mut handler = PdfHandler::new(input_path.to_str().unwrap())?;
        handler.embed_data(test_data)?;
        handler.save(output_path.to_str().unwrap())?;

        // Test extraction using our extract_data function
        let config = ExtractConfig {
            input_path: output_path.to_str().unwrap().to_string(),
            encryption: None,
            parameters: None,
        };

        let data = extract_data(config)?;
        assert!(!data.is_empty());
        assert_eq!(data, test_data);
        Ok(())
    }
}
