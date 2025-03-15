# Steganography Implementation in nHale

This document provides detailed information about the steganography implementations in nHale.

## Overview

nHale currently supports embedding and extracting hidden data in the following file formats:

- **PNG**: Uses LSB (Least Significant Bit) steganography with configurable bit depth (1-4 bits)
- **JPEG**: Uses DCT coefficient modification with Reed-Solomon error correction
- **PDF**: Embeds data in document metadata with HMAC integrity checking

Future planned formats (not yet implemented):
- **WAV**: LSB in audio samples (planned)
- **MP3**: (planned)
- **MP4**: (planned)
- **BMP**: (planned)
- **GIF**: (planned)

## PNG Steganography

### Implementation Details

1. **LSB Embedding**:
   - Configurable bit depth (1-4 bits per color channel)
   - Uses RGB channels for data storage
   - Supports encryption (AES-256, ChaCha20, RSA)

2. **Data Format**:
   - First 4 bytes: Length prefix (big-endian u32)
   - Remaining bytes: Actual data (optionally encrypted)

3. **Capacity**:
   - Each pixel provides 3 channels (RGB)
   - With 1-bit LSB: 3 bits per pixel
   - With 4-bit LSB: 12 bits per pixel
   - Example: 512x512 image with 1-bit LSB = ~98KB storage

## JPEG Steganography

### Implementation Details

1. **DCT Coefficient Modification**:
   - Works with JPEG's DCT transform blocks
   - Modifies coefficients to store data
   - Includes Reed-Solomon error correction

2. **Data Format**:
   - Error correction encoding
   - Length prefix
   - Actual data

3. **Limitations**:
   - Subject to JPEG compression artifacts
   - Limited capacity compared to PNG
   - Requires high quality settings for reliability

## PDF Steganography

### Implementation Details

1. **Metadata Embedding**:
   - Embeds data in PDF metadata stream
   - Uses HMAC for integrity verification
   - Supports encryption

2. **Data Format**:
   - HMAC (32 bytes)
   - HMAC Key (32 bytes)
   - Actual data

3. **Security Features**:
   - HMAC integrity checking
   - Optional encryption
   - Metadata stream isolation

## Working with the API

### Embedding Data

```rust
use nhale::{EmbedConfig, embed_in_jpg, embed_in_png, embed_in_pdf};

// Create configuration
let config = EmbedConfig {
    input_path: "input.jpg".to_string(),
    output_path: "output_with_hidden_data.jpg".to_string(),
    data: "Secret message".as_bytes().to_vec(),
    encryption: None, // Optional encryption
};

// Embed data
let result = embed_in_jpg(config);
```

### Extracting Data

```rust
use nhale::{ExtractConfig, extract_from_jpg};

// Create configuration
let config = ExtractConfig {
    input_path: "image_with_hidden_data.jpg".to_string(),
    encryption: None, // Must match embedding encryption
};

// Extract data
let result = extract_from_jpg(config);
if let Ok(data) = result {
    println!("Extracted: {}", String::from_utf8_lossy(&data));
}
```

## Best Practices

1. **Format Selection**:
   - Use PNG for maximum reliability and capacity
   - Use JPEG when storage size is a concern
   - Use PDF when document-based steganography is needed

2. **Data Size**:
   - Keep messages small relative to carrier capacity
   - Use compression for large messages
   - Consider error correction overhead

3. **Security**:
   - Always use encryption for sensitive data
   - Enable HMAC integrity checking
   - Use strong passwords for encryption

4. **Error Handling**:
   - Always check return values
   - Handle extraction errors gracefully
   - Verify extracted data integrity 