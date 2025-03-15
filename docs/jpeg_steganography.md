# JPEG Steganography in nHale

This document details the JPEG steganography implementation in nHale, focusing on the DCT coefficient modification approach and error correction.

## Technical Overview

The JPEG steganography implementation in nHale uses DCT (Discrete Cosine Transform) coefficient modification to embed data. This approach is chosen for its resilience to JPEG compression while maintaining reasonable image quality.

### Implementation Details

1. **Data Preprocessing**:
   - Input data is processed through Reed-Solomon error correction
   - Length prefix is added (4 bytes, big-endian u32)
   - Optional encryption (AES-256, ChaCha20, or RSA)

2. **Embedding Process**:
   - JPEG is decoded to access DCT coefficients
   - Data is embedded by modifying selected coefficients
   - Reed-Solomon error correction provides robustness
   - Image is re-encoded with embedded data

3. **Extraction Process**:
   - JPEG is decoded to access DCT coefficients
   - Embedded data is extracted from coefficients
   - Error correction is applied to recover data
   - Length prefix is used to extract exact message

## Current Limitations

1. **Compression Sensitivity**:
   - High JPEG compression can corrupt embedded data
   - Quality settings below 90 may cause data loss
   - Multiple saves/edits can destroy hidden data

2. **Capacity Constraints**:
   - Limited by number of usable coefficients
   - Error correction reduces effective capacity
   - Typical capacity is lower than PNG steganography

3. **Implementation Status**:
   - Basic DCT coefficient modification working
   - Reed-Solomon integration partially complete
   - Further optimization needed for reliability

## Best Practices

1. **Image Settings**:
   - Use JPEG quality 90 or higher
   - Avoid recompressing images with hidden data
   - Use larger images for more capacity

2. **Data Handling**:
   - Keep messages small relative to image size
   - Enable error correction for important data
   - Use encryption for sensitive information

3. **Alternative Considerations**:
   - Consider PNG for maximum reliability
   - Use PDF for document-based steganography
   - Test extraction before relying on embedded data

## API Usage

```rust
use nhale::{EmbedConfig, embed_in_jpg};

// Configuration for embedding
let config = EmbedConfig {
    input_path: "input.jpg".to_string(),
    output_path: "output.jpg".to_string(),
    data: "Secret message".as_bytes().to_vec(),
    encryption: None, // Optional encryption
};

// Embed the data
match embed_in_jpg(config) {
    Ok(_) => println!("Data embedded successfully"),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Error Correction

The current implementation uses Reed-Solomon error correction:

1. **Configuration**:
   - Default configuration uses RS(255,223)
   - 32 bytes of error correction per 223 bytes of data
   - Can correct up to 16 byte errors per block

2. **Integration Status**:
   - Basic Reed-Solomon encoding implemented
   - Decoding support available
   - Integration with JPEG needs optimization

## Future Improvements

1. **Planned Enhancements**:
   - Improved coefficient selection
   - Better error correction integration
   - Adaptive capacity estimation
   - Quality preservation techniques

2. **Research Areas**:
   - Alternative coefficient selection methods
   - Advanced error correction schemes
   - Compression resistance techniques 