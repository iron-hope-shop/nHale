//! Metadata Module
//! 
//! This module provides functionality for handling metadata associated with
//! steganographic data, including timestamps, authorship, and custom fields.

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::{Error, Result};

/// Metadata associated with embedded data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    /// Unix timestamp of when the data was embedded
    pub timestamp: i64,
    /// Author of the embedded data
    pub author: String,
    /// Description of the embedded data
    pub description: String,
    /// Custom metadata fields
    pub custom: HashMap<String, String>,
}

impl Metadata {
    /// Create new metadata with the given author and description
    pub fn new(author: String, description: String) -> Self {
        Self {
            timestamp: chrono::Utc::now().timestamp(),
            author,
            description,
            custom: HashMap::new(),
        }
    }

    /// Add a custom field to the metadata
    pub fn add_custom(&mut self, key: String, value: String) {
        self.custom.insert(key, value);
    }

    /// Serialize metadata to bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        serde_json::to_vec(self)
            .map_err(|e| Error::Serialization(e.to_string()))
    }

    /// Deserialize metadata from bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        serde_json::from_slice(data)
            .map_err(|e| Error::Serialization(e.to_string()))
    }
} 