# Error Correction in nHale

This document details the error correction mechanisms implemented in nHale, with a focus on the Reed-Solomon implementation used for JPEG steganography.

## Overview

nHale implements Reed-Solomon error correction to provide robustness against data corruption, particularly in lossy formats like JPEG. The implementation is designed to be modular and configurable.

## Reed-Solomon Implementation

### Current Configuration

1. **Default Parameters**:
   - RS(255,223) encoding
   - 32 bytes of error correction per 223 bytes of data
   - Can correct up to 16 byte errors per block
   - Uses GF(2^8) finite field arithmetic

2. **Implementation Details**:
   - Based on the standard Reed-Solomon algorithm
   - Uses systematic encoding (data bytes appear unchanged)
   - Optimized for 8-bit symbol size
   - Supports variable message sizes

### Usage in JPEG Steganography

1. **Integration**:
   - Applied before DCT coefficient modification
   - Each data block is protected independently
   - Error correction bytes are embedded alongside data

2. **Current Status**:
   - Basic encoding and decoding implemented
   - Integration with JPEG steganography needs optimization
   - Performance improvements planned

## API Usage

```rust
use nhale::error_correction::{ReedSolomonConfig, encode_reed_solomon};

// Create default configuration
let config = ReedSolomonConfig::default();

// Encode data with error correction
let data = b"Example message";
let encoded = encode_reed_solomon(data, &config)?;

// The encoded data includes both the original message and error correction bytes
assert!(encoded.len() > data.len());
```

## Error Handling

1. **Detection Capabilities**:
   - Can detect corrupted data blocks
   - Identifies uncorrectable errors
   - Reports correction success/failure

2. **Recovery Process**:
   - Attempts to correct detected errors
   - Falls back gracefully if correction impossible
   - Provides detailed error information

## Performance Considerations

1. **Current Performance**:
   - Encoding: O(n) for message size n
   - Decoding: O(n) for successful correction
   - Memory usage: O(n) additional space

2. **Optimization Opportunities**:
   - SIMD acceleration possible
   - Parallel processing for large messages
   - Lookup table optimizations

## Future Improvements

1. **Planned Enhancements**:
   - Better JPEG integration
   - Adaptive error correction levels
   - Performance optimizations
   - Extended error statistics

2. **Research Areas**:
   - Alternative error correction codes
   - Format-specific optimizations
   - Adaptive correction strategies 