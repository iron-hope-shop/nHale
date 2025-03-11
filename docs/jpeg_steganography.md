# JPEG Steganography in nHale

This document provides a comprehensive explanation of how JPEG steganography is implemented in nHale, the challenges faced, and the solutions implemented.

## Overview

JPEG steganography is particularly challenging due to the lossy nature of JPEG compression. Unlike formats like PNG, which preserve pixel values exactly, JPEG alters pixel values during compression, potentially corrupting embedded data.

## Implementation Details

### Embedding Approach

nHale uses a block-based approach for JPEG steganography:

1. **Block Structure**: The image is divided into 8x8 pixel blocks, which aligns with JPEG's DCT (Discrete Cosine Transform) blocks.

2. **Blue Channel Focus**: We modify the blue channel as human eyes are less sensitive to changes in blue than red or green.

3. **Parity-Based Embedding**:
   - For each 8x8 block, we calculate the average blue value
   - To embed a '0' bit, we ensure the average is even
   - To embed a '1' bit, we ensure the average is odd
   - We adjust all pixels in the block slightly to achieve the desired parity

4. **Data Format**:
   - First 4 bytes: Length prefix (big-endian u32)
   - Remaining bytes: Data with error correction

5. **Error Correction**:
   - Data is protected using parity bytes (1 byte per 8 data bytes by default)
   - A CRC-32 checksum is included for integrity verification
   - This helps detect (though not fully correct) corruption caused by compression

### Capacity

- Each 8x8 block can store 1 bit
- A 256x256 JPEG image contains 32x32 = 1024 blocks
- This gives a theoretical capacity of 128 bytes (1024 bits)
- After accounting for error correction overhead, the effective capacity is around 110 bytes

## Challenges and Solutions

### Challenge 1: Compression Artifacts

**Problem**: JPEG compression alters pixel values, potentially changing the parity of block averages.

**Solutions**:
1. **Block-Based Approach**: Using the average of an 8x8 block is more robust than individual pixels
2. **Error Correction**: Implementing parity-based error correction helps detect corruption
3. **Quality Settings**: Using higher JPEG quality settings (95-100) minimizes corruption

### Challenge 2: Capacity vs. Robustness

**Problem**: More robust approaches typically reduce capacity.

**Solutions**:
1. **Configurable Error Correction**: Users can adjust the data-to-parity ratio
2. **Blue Channel Focus**: Concentrating on one channel maximizes capacity while maintaining robustness
3. **Quality Recommendations**: Documentation advises on optimal capacity based on image size and quality

### Challenge 3: Error Detection and Correction

**Problem**: JPEG compression can cause unrecoverable data loss.

**Solutions**:
1. **Parity Bytes**: Each chunk of data is protected by a parity byte
2. **CRC-32 Checksum**: Validates the integrity of the entire message
3. **Robust Reporting**: The system provides detailed warnings when corruption is detected

## Usage Guidelines

### Best Practices

1. **Use High Quality**: Set JPEG quality to 95-100 when saving images with embedded data
2. **Size Matters**: Larger images provide more capacity and better reliability
3. **Keep Messages Small**: Smaller messages have a better chance of surviving compression
4. **Consider Alternatives**: For critical data, consider using PNG or other lossless formats

### API Example

```rust
// Embedding data
let config = EmbedConfig {
    input_path: "input.jpg".to_string(),
    output_path: "output_with_hidden_data.jpg".to_string(),
    data: "Secret message".as_bytes().to_vec(),
    encryption: None, // Optional encryption
};

// The data will be automatically protected with error correction
let result = embed_in_jpg(config);

// Extracting data
let extract_config = ExtractConfig {
    input_path: "image_with_hidden_data.jpg".to_string(),
    encryption: None,
    parameters: None,
};

let result = extract_from_jpg(extract_config);
```

## Technical Details

### Embedding Algorithm

```
1. Apply error correction to the input data
2. Divide the image into 8x8 pixel blocks
3. For each bit to embed:
   a. Calculate the average blue value of the current block
   b. Determine the target parity (even for '0', odd for '1')
   c. If the current parity doesn't match the target:
      i. Calculate the adjustment needed (+1 or -1)
      ii. Apply the adjustment to all pixels in the block
   d. Move to the next block
4. Save the modified image with high quality settings
```

### Extraction Algorithm

```
1. Divide the image into 8x8 pixel blocks
2. For each block:
   a. Calculate the average blue value
   b. Extract a bit based on parity (even = '0', odd = '1')
3. Convert the bits to bytes
4. Extract the length prefix and data portion
5. Apply error correction decoding
6. Decrypt if necessary
7. Return the extracted data
```

## Future Improvements

1. **DCT Coefficient Modification**: Implement direct modification of DCT coefficients for better compression resistance
2. **Reed-Solomon Codes**: Implement true error correction capable of repairing damaged data
3. **Adaptive Embedding**: Adjust embedding strength based on image characteristics
4. **Perceptual Models**: Use human visual system models to maximize capacity while remaining imperceptible 