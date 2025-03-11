//! nHale - Advanced Steganography Toolkit
//! 
//! This library provides a comprehensive set of tools for steganographic operations,
//! including data embedding, extraction, and analysis across various file formats.

pub mod embedding;
pub mod extraction;
pub mod encryption;
pub mod integrity;
pub mod metadata;
pub mod pdf;
pub mod utils;
pub mod watermarking;
pub mod error_correction;

// Re-export commonly used items
pub use embedding::*;
pub use extraction::*;
pub use encryption::*;
pub use integrity::*;
pub use watermarking::*;
pub use error_correction::*;

use std::fmt;

/// Error types for the nHale library
#[derive(Debug)]
pub enum Error {
    /// Input validation error
    InvalidInput(String),
    /// I/O error
    Io(String),
    /// Encryption error
    Encryption(String),
    /// Integrity error
    Integrity(String),
    /// Serialization error
    Serialization(String),
    /// Invalid data
    InvalidData(String),
    /// Feature not implemented
    NotImplemented(String),
    /// Encoding error
    Encoding(String),
    /// Extraction error
    Extraction(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            Error::Io(msg) => write!(f, "I/O error: {}", msg),
            Error::Encryption(msg) => write!(f, "Encryption error: {}", msg),
            Error::Integrity(msg) => write!(f, "Integrity error: {}", msg),
            Error::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            Error::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            Error::NotImplemented(msg) => write!(f, "Not implemented: {}", msg),
            Error::Encoding(msg) => write!(f, "Encoding error: {}", msg),
            Error::Extraction(msg) => write!(f, "Extraction error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

/// Result type for nHale operations
pub type Result<T> = std::result::Result<T, Error>;

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION"); 