# nHale - Issues, Bugs, and Development Priorities

This document tracks the current issues, bugs, and development priorities in the nHale steganography toolkit. It serves as a reference for both users and developers.

## Implemented Features vs. README Claims

| Feature in README | Status | Notes |
|-------------------|--------|-------|
| Rust-based LSB embedding and extraction | ✅ Implemented | Working for PNG images with variable bit depth (1-4) |
| AES, ChaCha20, and RSA encryption modules | ✅ Implemented | All encryption algorithms are fully functional |
| Integrity checking and HMAC verification | ✅ Implemented | Working for PDF files |
| Metadata manipulation utilities | ⚠️ Partial | Basic structure exists, but limited functionality |
| PNG image processing | ✅ Implemented | Functional LSB steganography |
| JPG image processing | ⚠️ Partial | Basic implementation exists but needs reliability improvements |
| BMP and GIF image processing | ❌ Missing | Mentioned in README but not implemented |
| Audio processing (WAV, MP3) | ❌ Missing | Function stubs exist but not implemented |
| Video processing (MP4) | ❌ Missing | Function stubs exist but not implemented |
| PDF embedding and extraction | ✅ Implemented | Working with integrity checking |
| Basic CLI commands | ✅ Implemented | Commands for embed, extract, watermark, verify, detect |
| Advanced configuration options | ⚠️ Partial | Basic options exist, more needed for different formats |
| Batch processing | ❌ Missing | Mentioned but not implemented |
| Watermarking | ⚠️ Skeleton | Module structure exists, but functionality not implemented |
| Error correction | ✅ Implemented | Reed-Solomon and parity-based options available |

## Current Issues

### 1. JPEG Steganography Reliability

- **Issue**: JPEG steganography is unreliable due to lossy compression
- **Current Status**: Basic implementation exists using DCT coefficient modification
- **Development Needs**:
  - Better integration of Reed-Solomon error correction
  - Improved handling of JPEG compression artifacts
  - More robust approach to bit embedding in DCT coefficients
  - Testing with various JPEG encoders and compression levels
- **Relevant Files**: `src/embedding.rs`, `src/extraction.rs`, `src/error_correction.rs`

### 2. File Format Support Gaps

- **Issue**: Several file formats mentioned in README are not implemented
- **Current Status**: Only PNG, JPG (partial), and PDF are implemented
- **Development Needs**:
  - Implement BMP steganography
  - Implement GIF steganography
  - Implement WAV steganography (simplest audio format to start with)
  - Implement MP3 steganography
  - Implement MP4 steganography (most complex)
- **Relevant Files**: `src/embedding.rs`, `src/extraction.rs`

### 3. Watermarking Implementation

- **Issue**: Watermarking module is just a skeleton
- **Current Status**: Module structure exists but all functions return "Not implemented"
- **Development Needs**:
  - Implement visible watermarking (simpler)
  - Implement invisible DCT-based watermarking (complex)
  - Add watermark detection and verification
  - Add tests for watermarking functionality
- **Relevant Files**: `src/watermarking.rs`

### 4. Metadata Manipulation Limitations

- **Issue**: Limited metadata functionality
- **Current Status**: Basic structure exists but functionality is limited
- **Development Needs**:
  - Enhance metadata extraction for different file formats
  - Implement metadata modification capabilities
  - Add format-specific metadata handlers
- **Relevant Files**: `src/metadata.rs`

### 5. Testing Limitations

- **Issue**: Limited test coverage
- **Current Status**: Basic unit tests exist, one JPEG test is ignored
- **Development Needs**:
  - Add more test fixtures for different file formats
  - Create comprehensive integration tests
  - Add tests for error cases and edge conditions
  - Add benchmarking tests for performance optimization
- **Relevant Files**: Tests in various `*_test.rs` sections

### 6. CLI Enhancements

- **Issue**: CLI needs improvement
- **Current Status**: Basic commands work but some features are missing
- **Development Needs**:
  - Improve help documentation with examples
  - Add configuration file support
  - Implement batch processing for multiple files
  - Add support for reading data from stdin/files
- **Relevant Files**: `src/bin/cli.rs`

### 7. Error Handling

- **Issue**: Error handling could be improved
- **Current Status**: Basic error types exist
- **Development Needs**:
  - More specific error messages
  - Better error context
  - Improved validation of inputs
- **Relevant Files**: `src/lib.rs` (Error enum), various modules

## Specific Bugs

### 1. JPEG Reed-Solomon Integration

- **Bug**: JPEG with Reed-Solomon error correction is unreliable
- **Status**: Test is currently ignored (`test_jpg_reed_solomon_error_correction`)
- **Fix**: Needs overhaul of JPEG embedding and extraction with proper error correction integration
- **Relevant Files**: `src/embedding.rs`, `src/extraction.rs`, `src/error_correction.rs`

### 2. Extraction Debug Messages

- **Bug**: Debug messages are printed during extraction in production code
- **Status**: Affects user experience
- **Fix**: Replace `println!` debug statements with proper logging
- **Relevant Files**: `src/extraction.rs`

### 3. Key Derivation Function

- **Bug**: Current key derivation function is a simplified version due to compatibility issues
- **Status**: Works but not optimal for security
- **Fix**: Replace with proper PBKDF2 implementation
- **Relevant Files**: `src/encryption.rs`

### 4. RSA Key Handling

- **Bug**: RSA implementation generates new keys for each operation instead of using persistent keys
- **Status**: Makes the RSA encryption/decryption impractical for real use
- **Fix**: Implement proper key storage and loading
- **Relevant Files**: `src/encryption.rs`

## Development Priorities

Based on our analysis, here are the recommended development priorities:

### Immediate Priorities (Critical for Basic Functionality)

1. **Improve JPEG Steganography Reliability**
   - Fix Reed-Solomon integration
   - Improve compression artifact handling
   - Update tests to verify functionality

2. **Complete Basic Watermarking Implementation**
   - Implement visible watermarking first
   - Add basic watermark detection

3. **Enhance Error Handling**
   - Replace debug prints with proper logging
   - Improve error messages and context

### Short-term Priorities (Important for Feature Completeness)

4. **Implement BMP and WAV Support**
   - These are simpler formats to implement
   - Will increase the supported format coverage

5. **Enhance Metadata Functionality**
   - Complete metadata extraction and modification
   - Support for different file formats

6. **Improve CLI Documentation and Help**
   - Better examples and usage guidance
   - Configuration file support

### Medium-term Priorities (Feature Enhancement)

7. **Implement GIF and MP3 Support**
   - More complex but still achievable
   - Add appropriate tests

8. **Invisible Watermarking Implementation**
   - More complex DCT-based watermarking
   - Add robust detection and verification

9. **Batch Processing**
   - Support for processing multiple files
   - Directory-based operations

### Long-term Priorities (Advanced Features)

10. **Implement MP4 Support**
    - Most complex format
    - May require specialized libraries

11. **Security Auditing**
    - Comprehensive security review
    - Fix any identified issues

12. **Performance Optimization**
    - Benchmark different approaches
    - Optimize for speed and memory usage

13. **Packaging and Distribution**
    - Create platform-specific packages
    - Automate release process
    - Enable CI/CD workflow

## Conclusion

The nHale project has made significant progress implementing core steganography features for PNG and PDF files with strong encryption support. The JPEG implementation needs improvement, and several planned features are not yet implemented. Following the priorities outlined above will help close the gap between the current implementation and the ambitious vision described in the README. 