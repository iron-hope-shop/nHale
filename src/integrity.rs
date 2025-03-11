//! Integrity Module
//! 
//! This module provides functionality for verifying the integrity of embedded data
//! using HMAC-SHA256.

use crate::{Error, Result};
use hmac::{Hmac, Mac};
use sha2::Sha256;

const HMAC_KEY_LENGTH: usize = 32;
const HMAC_OUTPUT_LENGTH: usize = 32;

type HmacSha256 = Hmac<Sha256>;

/// Adds an HMAC to the data for integrity verification
pub fn add_integrity_check(data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    if key.len() != HMAC_KEY_LENGTH {
        return Err(Error::Integrity("Invalid key length".into()));
    }
    
    let mut mac = HmacSha256::new_from_slice(key)
        .map_err(|e| Error::Integrity(e.to_string()))?;
    
    mac.update(data);
    let result = mac.finalize();
    let mac_bytes = result.into_bytes();
    
    let mut output = Vec::with_capacity(data.len() + HMAC_OUTPUT_LENGTH);
    output.extend_from_slice(&mac_bytes);
    output.extend_from_slice(data);
    
    Ok(output)
}

/// Verifies the integrity of data using its HMAC
pub fn verify_integrity(data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    if data.len() < HMAC_OUTPUT_LENGTH {
        return Err(Error::Integrity("Data too short".into()));
    }
    
    if key.len() != HMAC_KEY_LENGTH {
        return Err(Error::Integrity("Invalid key length".into()));
    }
    
    let (mac_bytes, message) = data.split_at(HMAC_OUTPUT_LENGTH);
    
    let mut mac = HmacSha256::new_from_slice(key)
        .map_err(|e| Error::Integrity(e.to_string()))?;
    
    mac.update(message);
    
    mac.verify_slice(mac_bytes)
        .map_err(|_| Error::Integrity("Invalid HMAC".into()))?;
    
    Ok(message.to_vec())
}

/// Generates a random key suitable for HMAC operations
pub fn generate_integrity_key() -> Result<[u8; HMAC_KEY_LENGTH]> {
    let mut key = [0u8; HMAC_KEY_LENGTH];
    getrandom::getrandom(&mut key)
        .map_err(|e| Error::Integrity(e.to_string()))?;
    Ok(key)
}

/// Generates an HMAC for the given data
pub fn generate_hmac(data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let mut mac = HmacSha256::new_from_slice(key)
        .map_err(|e| Error::Integrity(format!("Invalid HMAC key: {}", e)))?;
    
    mac.update(data);
    Ok(mac.finalize().into_bytes().to_vec())
}

/// Verifies the HMAC for the given data
pub fn verify_hmac(data: &[u8], key: &[u8], hmac: &[u8]) -> Result<bool> {
    let mut mac = HmacSha256::new_from_slice(key)
        .map_err(|e| Error::Integrity(format!("Invalid HMAC key: {}", e)))?;
    
    mac.update(data);
    
    Ok(mac.verify_slice(hmac).is_ok())
} 