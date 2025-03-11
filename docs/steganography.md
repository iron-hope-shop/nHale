# Steganography Implementation in nHale

This document provides detailed information about the steganography implementations in nHale, with a special focus on the JPEG steganography functionality.

## Overview

nHale supports embedding and extracting hidden data in various file formats:

- **PNG**: Uses LSB (Least Significant Bit) steganography with configurable bit depth
- **JPEG**: Uses a block-based approach to survive compression
- **PDF**: Embeds data in document structure
- **WAV**: (Planned) LSB in audio samples
- **MP3**: (Planned) 
- **MP4**: (Planned)

## JPEG Steganography

### Implementation Details

JPEG steganography is particularly challenging due to the lossy compression used in the JPEG format. Our implementation uses the following approach:

1. **Block-Based Embedding**:
   - We work with 8x8 pixel blocks, which aligns with JPEG's DCT transform blocks
   - For each block, we embed a single bit by modifying the average blue channel value
   - Even average values represent '0', odd average values represent '1'

2. **Data Format**:
   - First 4 bytes: Length prefix (big-endian u32)
   - Remaining bytes: Actual data
   
3. **Capacity**:
   - Each 8x8 block can store 1 bit
   - A 256x256 JPEG image contains 32x32 = 1024 blocks
   - This gives a theoretical capacity of 128 bytes (1024 bits)

### Limitations

Due to the lossy nature of JPEG compression, the following limitations apply:

1. **Compression Artifacts**: JPEG compression can alter pixel values, potentially corrupting embedded data
2. **Quality Settings**: Higher quality settings (90-100) improve reliability but don't guarantee perfect extraction
3. **Limited Capacity**: Only 1 bit per 8x8 block, much less than formats like PNG
4. **Blue Channel Focus**: We use the blue channel as human eyes are less sensitive to changes in blue

### Best Practices

For more reliable JPEG steganography:

1. **Use High Quality**: Set JPEG quality to 95-100 when saving
2. **Keep Messages Small**: The smaller the message, the more likely it will survive compression
3. **Consider Image Size**: Larger images provide more capacity
4. **Use Error Correction**: For critical data, consider implementing error correction codes
5. **Alternative Formats**: For maximum reliability, consider using PNG or other lossless formats

## PNG Steganography

PNG steganography in nHale uses traditional LSB (Least Significant Bit) embedding:

1. **Configurable Bit Depth**: Can use 1-4 bits per color channel
2. **Multiple Channels**: Can embed across R, G, and B channels
3. **High Capacity**: A 512x512 PNG using 1-bit LSB provides approximately 98KB of storage

## PDF Steganography

PDF steganography works by embedding data in the document structure:

1. **Metadata Fields**: Embeds data in custom metadata fields
2. **Object Streams**: Hides data in object streams
3. **Capacity**: Varies based on document structure and size

## Working with the API

### Embedding Data

```rust
use nhale::{EmbedConfig, embed_in_jpg, embed_in_png};

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
    parameters: None,
};

// Extract data
let result = extract_from_jpg(config);
if let Ok(data) = result {
    println!("Extracted: {}", String::from_utf8_lossy(&data));
}
``` 