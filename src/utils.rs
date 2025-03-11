//! Utility Module
//! 
//! This module provides utility functions for the application.

use crate::{Error, Result};
use std::path::Path;
use std::fs;

/// Checks if a file exists and has the correct extension
pub fn validate_image_file(path: &Path) -> Result<()> {
    if !path.exists() {
        return Err(Error::Io(format!("File not found: {}", path.display())));
    }
    
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => match ext.to_lowercase().as_str() {
            "png" | "jpg" | "jpeg" | "bmp" | "gif" => Ok(()),
            _ => Err(Error::InvalidInput("Unsupported image format".into())),
        },
        None => Err(Error::InvalidInput("File has no extension".into())),
    }
}

/// Checks if a file exists and has the correct extension for audio files
pub fn validate_audio_file(path: &Path) -> Result<()> {
    if !path.exists() {
        return Err(Error::Io(format!("File not found: {}", path.display())));
    }
    
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => match ext.to_lowercase().as_str() {
            "wav" | "mp3" => Ok(()),
            _ => Err(Error::InvalidInput("Unsupported audio format".into())),
        },
        None => Err(Error::InvalidInput("File has no extension".into())),
    }
}

/// Calculates the maximum amount of data that can be embedded in an image
pub fn calculate_image_capacity(width: u32, height: u32) -> usize {
    // We can use 1 bit per color channel (RGB), so 3 bits per pixel
    // Subtract 32 bits (4 bytes) for storing the data length
    let total_bits = (width * height * 3) as usize;
    let available_bits = total_bits.saturating_sub(32);
    available_bits / 8 // Convert to bytes
}

/// Maximum allowed data size (100MB)
const MAX_DATA_SIZE: usize = 100 * 1024 * 1024;

/// Validates input data
pub fn validate_data(data: &[u8]) -> Result<()> {
    if data.is_empty() {
        return Err(Error::InvalidData("Data cannot be empty".to_string()));
    }
    
    if data.len() > MAX_DATA_SIZE {
        return Err(Error::InvalidData(format!(
            "Data size exceeds maximum allowed size of {}MB",
            MAX_DATA_SIZE / (1024 * 1024)
        )));
    }
    
    Ok(())
}

/// Checks if a file exists and is accessible
pub fn check_file_exists(path: &str) -> Result<()> {
    if !Path::new(path).exists() {
        return Err(Error::Io(format!("File not found: {}", path)));
    }
    Ok(())
}

/// Formats a size in bytes to a human-readable string
pub fn format_size(size: u64) -> String {
    const UNITS: [&str; 4] = ["B", "KB", "MB", "GB"];
    let mut size = size as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    format!("{:.2} {}", size, UNITS[unit_index])
}

/// Reads a file into a byte vector
pub fn read_file(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    fs::read(path.as_ref())
        .map_err(|e| Error::Io(e.to_string()))
}

/// Writes bytes to a file
pub fn write_file(path: impl AsRef<Path>, data: &[u8]) -> Result<()> {
    fs::write(path.as_ref(), data)
        .map_err(|e| Error::Io(e.to_string()))
}

/// Checks if a file exists
pub fn file_exists(path: impl AsRef<Path>) -> bool {
    path.as_ref().exists()
}

/// Gets the file size in bytes
pub fn file_size(path: impl AsRef<Path>) -> Result<u64> {
    fs::metadata(path.as_ref())
        .map(|m| m.len())
        .map_err(|e| Error::Io(e.to_string()))
}

/// Supported file formats
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileFormat {
    Png,
    Jpg,
    Bmp,
    Gif,
    Wav,
    Mp3,
    Mp4,
    Pdf,
    Unknown,
}

/// Detects the file format from the given path
pub fn detect_file_format(path: &Path) -> FileFormat {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => match ext.to_lowercase().as_str() {
            "png" => FileFormat::Png,
            "jpg" | "jpeg" => FileFormat::Jpg,
            "bmp" => FileFormat::Bmp,
            "gif" => FileFormat::Gif,
            "wav" => FileFormat::Wav,
            "mp3" => FileFormat::Mp3,
            "mp4" => FileFormat::Mp4,
            "pdf" => FileFormat::Pdf,
            _ => FileFormat::Unknown,
        },
        None => FileFormat::Unknown,
    }
}

/// Detects the file format from bytes (magic numbers)
pub fn detect_file_format_from_bytes(bytes: &[u8]) -> FileFormat {
    if bytes.len() < 8 {
        return FileFormat::Unknown;
    }
    
    // Check file signatures
    if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        FileFormat::Png
    } else if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
        FileFormat::Jpg
    } else if bytes.starts_with(b"BM") {
        FileFormat::Bmp
    } else if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
        FileFormat::Gif
    } else if bytes.starts_with(b"RIFF") && bytes[8..].starts_with(b"WAVE") {
        FileFormat::Wav
    } else if bytes.starts_with(&[0x49, 0x44, 0x33]) || bytes.starts_with(&[0xFF, 0xFB]) {
        FileFormat::Mp3
    } else if bytes.starts_with(&[0x00, 0x00, 0x00]) && 
             (bytes[4..].starts_with(b"ftyp") || bytes[4..].starts_with(b"moov")) {
        FileFormat::Mp4
    } else if bytes.starts_with(b"%PDF") {
        FileFormat::Pdf
    } else {
        FileFormat::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::fs;
    use tempfile::tempdir;
    
    #[test]
    fn test_validate_image_file() {
        // Test with non-existent files with valid extensions
        let valid_extensions = ["png", "jpg", "jpeg", "bmp", "gif"];
        for ext in valid_extensions.iter() {
            let path = PathBuf::from(format!("test.{}", ext));
            assert!(matches!(
                validate_image_file(&path),
                Err(Error::Io(_))
            ));
        }
        
        // Create a temporary file with an invalid extension
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.txt");
        fs::write(&path, b"test").unwrap();
        
        assert!(matches!(
            validate_image_file(&path),
            Err(Error::InvalidInput(_))
        ));
    }
    
    #[test]
    fn test_calculate_image_capacity() {
        assert_eq!(calculate_image_capacity(100, 100), 3746);
        assert_eq!(calculate_image_capacity(1920, 1080), 777_596);
    }
    
    #[test]
    fn test_format_size() {
        assert_eq!(format_size(0), "0.00 B");
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1024 * 1024), "1.00 MB");
        assert_eq!(format_size(1024 * 1024 * 1024), "1.00 GB");
    }
    
    #[test]
    fn test_validate_data() {
        let data = vec![1, 2, 3, 4, 5];
        assert!(validate_data(&data).is_ok());
        
        let empty_data: Vec<u8> = vec![];
        assert!(validate_data(&empty_data).is_err());
        
        let large_data = vec![0; MAX_DATA_SIZE + 1];
        assert!(validate_data(&large_data).is_err());
    }
    
    #[test]
    fn test_check_file_exists() {
        assert!(check_file_exists("Cargo.toml").is_ok());
        assert!(check_file_exists("nonexistent_file.txt").is_err());
    }
} 