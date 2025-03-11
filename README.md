<div align="center">
  <img src="./images/banner.gif" alt="nHale - A Modular, High-Performance Steganography Library" width="100%">
</div>

<div align="center">
  
  [![CI](https://github.com/iron-hope-shop/nHale/actions/workflows/ci.yml/badge.svg)](https://github.com/iron-hope-shop/nHale/actions/workflows/ci.yml)
  [![GitHub Release](https://img.shields.io/github/v/release/iron-hope-shop/nHale?include_prereleases&label=Release)](https://github.com/iron-hope-shop/nHale/releases)
  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
  
</div>

---

## **1. Introduction**
### **1.1 Overview**
nHale is an open-source advanced steganography toolkit designed for secure message embedding, extraction, and analysis across various file formats. It is a Rust-first project, providing a lightweight, efficient, and modular implementation of steganographic algorithms. This tool is aimed at cybersecurity professionals, privacy advocates, and digital forensics experts.

### **1.2 Objectives**
- Develop a high-performance, lightweight Rust-based steganography library.
- Provide native embedding and extraction capabilities for multiple file types.
- Implement strong encryption before embedding data for enhanced security.
- Ensure robust metadata and watermarking functionalities.
- Maintain strict software development best practices.

---

## **2. System Architecture**
### **2.1 High-Level Design**
- **Core Engine:** Rust-based steganography algorithms.
- **CLI Interface:** Rust-based command-line interface with comprehensive options.

### **2.2 Supported File Formats**
- **Images:** BMP, PNG, JPG, GIF.
- **Audio:** WAV, MP3.
- **Video:** MP4.
- **Documents:** PDF.

### **2.3 Core Modules**
- **Embedding Module:** Hides encrypted data inside media files.
- **Extraction Module:** Recovers hidden messages from media.
- **Integrity Checker Module:** Ensures hidden data has not been modified.
- **Watermarking Module:** Embeds and verifies digital watermarks.
- **Metadata Module:** Reads, modifies, and analyzes metadata.
- **Encryption Module:** AES, ChaCha20, and RSA encryption support.
- **PDF Analysis Module:** Detects hidden scripts and anomalies.

---

## **3. Quick Start**

### **Installation**

#### From Binaries
Download the latest binary for your platform from the [Releases](https://github.com/iron-hope-shop/nHale/releases) page.

#### From Source
```bash
# Clone the repository
git clone https://github.com/iron-hope-shop/nHale.git
cd nhale

# Build the project
cargo build --release

# Run the CLI
./target/release/nhale-cli --help
```

### **Basic Usage**

#### Embedding data in an image
```bash
nhale embed -i input.png -o output.png -d "Secret message"
```

#### Extracting data from an image
```bash
nhale extract -i output.png
```

#### Using encryption
```bash
nhale embed -i input.png -o output.png -d "Secret message" --encrypt --password "your-secure-password"
```

#### More examples
See the [documentation](./docs/steganography.md) for more detailed examples and advanced usage.

---

## **4. Development**

### **Prerequisites**
- Rust 1.67.0 or higher
- Cargo

### **Testing**
```bash
cargo test
```

### **Contributing**
Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

---

## **5. License**
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## **6. Project Execution Plan (Kanban Tasks)**

### **Phase 1: Core Development**
- [x] Implement Rust-based LSB embedding and extraction.
- [x] Implement AES, ChaCha20, and RSA encryption modules.
- [x] Develop integrity checking and HMAC verification.
- [ ] Implement metadata manipulation utilities.
- [x] Implement PNG image processing with LSB steganography and variable bit depth.
- [ ] Implement JPG, BMP, and GIF image processing.
- [ ] Implement audio processing utilities (WAV, MP3).
- [ ] Implement video processing utilities (MP4).
- [x] Implement PDF embedding and extraction with integrity checking.

### **Phase 2: CLI Enhancement**
- [x] Develop basic CLI commands and argument parsing.
- [x] Add advanced configuration options for steganography techniques.
- [ ] Implement batch processing capabilities.
- [ ] Create user-friendly CLI help and documentation.
- [ ] Write comprehensive Rust documentation for all CLI commands.
- [ ] Write automated tests for CLI.

### **Phase 3: Testing & Deployment**
- [ ] Perform security audits on steganographic algorithms.
- [ ] Optimize performance and reduce binary size.
- [ ] Write end-to-end integration tests.
- [ ] Package and release for various platforms.

---

## **7. Code Structure**
```plaintext
nHale/
│── src/
│   │── main.rs  # CLI entry point
│   │── lib.rs   # Core library functionality
│   │── embedding.rs  # Embedding module
│   │── extraction.rs  # Extraction module
│   │── encryption.rs  # Encryption (AES, ChaCha20, RSA)
│   │── integrity.rs  # Integrity checking with HMAC
│   │── metadata.rs  # Metadata manipulation
│   │── pdf.rs  # PDF analysis module
│   │── utils.rs  # Helper functions and utilities
│   │── bin/
│   │   │── cli.rs  # CLI implementation
│
│── tests/
│   │── integration_tests.rs  # Integration testing
│   │── unit_tests.rs  # Unit tests
│   │── fixtures/  # Test fixtures
│
│── docs/
│   │── architecture.md  # Technical documentation
│   │── api.md  # API documentation
│   │── usage.md  # User guide
│
│── .github/
│   │── workflows/
│   │   │── ci.yml  # CI/CD pipeline
│
│── Cargo.toml  # Rust dependencies
│── README.md  # Project overview
│── LICENSE  # Open-source license