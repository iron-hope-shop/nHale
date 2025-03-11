# Error Correction in nHale

This document explains how error correction is implemented in nHale to improve the robustness of steganography, particularly for lossy formats like JPEG.

## Overview

Error correction is a critical component for reliable steganography in lossy formats. When embedding data in JPEG images, compression artifacts can corrupt the embedded bits, leading to data loss. nHale implements error correction to mitigate this issue.

## Parity-Based Error Correction

nHale uses a simple but effective parity-based error correction system:

### How It Works

1. **Data Chunking**: The input data is divided into chunks (default: 8 bytes per chunk).
2. **Parity Generation**: For each chunk, a parity byte is calculated by XORing all bytes in the chunk.
3. **Checksum**: A CRC-32 checksum of the original data is calculated and stored.
4. **Format**:
   - Header: Data-to-parity ratio (1 byte), Flags (1 byte), Original data length (4 bytes)
   - Optional: CRC-32 checksum (4 bytes)
   - Data: Interleaved data chunks and parity bytes

### Error Detection

During extraction:

1. **Parity Verification**: The parity of each chunk is recalculated and compared with the stored parity byte.
2. **Checksum Verification**: The CRC-32 checksum of the extracted data is calculated and compared with the stored checksum.
3. **Reporting**: Warnings are issued for any detected corruption.

### Configuration

The error correction can be configured with:

- `data_to_parity_ratio`: The number of data bytes per parity byte. Lower values provide more protection but reduce capacity.
- `use_checksum`: Whether to include a CRC-32 checksum for additional integrity checking.

## Integration with JPEG Steganography

Error correction is automatically applied when embedding data in JPEG images:

1. **Embedding Process**:
   - Original data → Encryption (if enabled) → Error Correction → Embedding
   - The error-corrected data is embedded in the image's blue channel by adjusting the parity of average values in 8x8 blocks.

2. **Extraction Process**:
   - Raw bits → Assembled bytes → Error Correction Decoding → Decryption (if enabled) → Original data
   - Any detected corruption is reported through warnings.

## Limitations

- The current implementation focuses on error detection rather than correction.
- For JPEG images with significant compression, some data loss may still occur.
- The overhead (extra bytes needed) is approximately 1/8 of the original data size by default.

## Usage Example

```rust
use nhale::{EmbedConfig, embed_in_jpg, ErrorCorrectionConfig};

// Create a custom error correction configuration
let error_correction_config = ErrorCorrectionConfig {
    data_to_parity_ratio: 4,  // More protection (1 parity byte for every 4 data bytes)
    use_checksum: true,      
};

// This configuration can be passed to the embedding function
// The JPEG steganography will automatically use error correction
let config = EmbedConfig {
    input_path: "input.jpg".to_string(),
    output_path: "output.jpg".to_string(),
    data: "Secret message".as_bytes().to_vec(),
    encryption: None,
};

// The data will be protected with error correction
let result = embed_in_jpg(config);
```

## Future Improvements

Future versions of nHale may include more sophisticated error correction:

1. **Reed-Solomon Codes**: Implement true error correction capable of repairing damaged data.
2. **Adaptive Error Correction**: Adjust the level of protection based on image characteristics.
3. **Selective Protection**: Apply stronger protection to critical data segments. 