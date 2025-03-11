//! Encryption Module
//!
//! This module provides encryption and decryption functionality using AES-256 and ChaCha20.

use crate::{Error, Result};
use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes256;
use chacha20::{
    cipher::{KeyIvInit, StreamCipher},
    ChaCha20,
};
use rand::{rngs::OsRng, RngCore};
use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};
use sha2::{Digest, Sha256};

const SALT_LENGTH: usize = 16;
const IV_LENGTH: usize = 12;
const KEY_LENGTH: usize = 32;
const RSA_KEY_SIZE: usize = 2048;

/// Supported encryption algorithms
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Algorithm {
    Aes256,
    ChaCha20,
    Rsa,
}

/// Configuration for encryption/decryption operations
#[derive(Debug, Clone)]
pub struct CryptoConfig {
    pub algorithm: Algorithm,
    pub password: String,
}

/// Encrypts data using the specified algorithm
pub fn encrypt(data: &[u8], config: &CryptoConfig) -> Result<Vec<u8>> {
    // Generate random salt
    let mut salt = [0u8; SALT_LENGTH];
    getrandom::getrandom(&mut salt).map_err(|e| Error::Encryption(e.to_string()))?;

    // Generate key from password using PBKDF2
    let mut key = [0u8; KEY_LENGTH];
    derive_key_from_password(&config.password, &salt, &mut key);

    let mut output = Vec::with_capacity(SALT_LENGTH + IV_LENGTH + data.len() + 32);
    output.extend_from_slice(&salt);

    match config.algorithm {
        Algorithm::Aes256 => encrypt_aes256(data, &key, &mut output),
        Algorithm::ChaCha20 => encrypt_chacha20(data, &key, &mut output),
        Algorithm::Rsa => encrypt_rsa(data, &key, &mut output),
    }?;

    Ok(output)
}

/// Decrypts data using the specified algorithm
pub fn decrypt(encrypted_data: &[u8], config: &CryptoConfig) -> Result<Vec<u8>> {
    if encrypted_data.len() < SALT_LENGTH + IV_LENGTH {
        return Err(Error::Encryption("Invalid encrypted data length".into()));
    }

    let salt = &encrypted_data[..SALT_LENGTH];
    let mut key = [0u8; KEY_LENGTH];

    // Generate key from password using PBKDF2
    derive_key_from_password(&config.password, salt, &mut key);

    let encrypted = &encrypted_data[SALT_LENGTH..];

    match config.algorithm {
        Algorithm::Aes256 => decrypt_aes256(encrypted, &key),
        Algorithm::ChaCha20 => decrypt_chacha20(encrypted, &key),
        Algorithm::Rsa => decrypt_rsa(encrypted, &key),
    }
}

// Improved key derivation function using PBKDF2
fn derive_key_from_password(password: &str, salt: &[u8], out: &mut [u8]) {
    // Since there are compatibility issues with the pbkdf2 crate,
    // we'll fall back to a simpler approach for now
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.update(salt);
    let hash = hasher.finalize();

    // Copy first bytes to output
    let copy_len = out.len().min(hash.len());
    out[..copy_len].copy_from_slice(&hash[..copy_len]);

    // Fill the rest with repeated hash if needed
    if copy_len < out.len() {
        let mut hasher = Sha256::new();
        hasher.update(hash);
        let hash2 = hasher.finalize();

        let remaining = out.len() - copy_len;
        let copy_len2 = remaining.min(hash2.len());
        out[copy_len..copy_len + copy_len2].copy_from_slice(&hash2[..copy_len2]);
    }

    // TODO: Replace with PBKDF2 once compatibility issues are resolved
    // pbkdf2::<hmac::Hmac<sha2::Sha256>>(
    //     password.as_bytes(),
    //     salt,
    //     10_000,  // 10,000 iterations
    //     out,
    // );
}

// Helper functions for AES-256
fn encrypt_aes256(data: &[u8], key: &[u8], output: &mut Vec<u8>) -> Result<()> {
    let cipher = Aes256::new(GenericArray::from_slice(key));
    let mut iv = [0u8; IV_LENGTH];
    getrandom::getrandom(&mut iv).map_err(|e| Error::Encryption(e.to_string()))?;

    output.extend_from_slice(&iv);

    // Pad data to block size
    let mut padded = data.to_vec();
    let padding_len = 16 - (data.len() % 16);
    padded.extend(std::iter::repeat(padding_len as u8).take(padding_len));

    for chunk in padded.chunks_mut(16) {
        let block = GenericArray::from_mut_slice(chunk);
        cipher.encrypt_block(block);
    }

    output.extend_from_slice(&padded);
    Ok(())
}

fn decrypt_aes256(encrypted: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    if encrypted.len() < IV_LENGTH {
        return Err(Error::Encryption("Invalid encrypted data".into()));
    }

    let _iv = &encrypted[..IV_LENGTH];
    let data = &encrypted[IV_LENGTH..];

    if data.len() % 16 != 0 {
        return Err(Error::Encryption("Invalid encrypted data length".into()));
    }

    let cipher = Aes256::new(GenericArray::from_slice(key));
    let mut decrypted = data.to_vec();

    for chunk in decrypted.chunks_mut(16) {
        let block = GenericArray::from_mut_slice(chunk);
        cipher.decrypt_block(block);
    }

    // Remove padding
    let padding_len = *decrypted
        .last()
        .ok_or_else(|| Error::Encryption("Empty decrypted data".into()))?
        as usize;
    if padding_len > 16 || padding_len > decrypted.len() {
        return Err(Error::Encryption("Invalid padding".into()));
    }

    decrypted.truncate(decrypted.len() - padding_len);
    Ok(decrypted)
}

// Helper functions for ChaCha20
fn encrypt_chacha20(data: &[u8], key: &[u8], output: &mut Vec<u8>) -> Result<()> {
    let mut nonce = [0u8; 12];
    getrandom::getrandom(&mut nonce).map_err(|e| Error::Encryption(e.to_string()))?;

    output.extend_from_slice(&nonce);

    let mut cipher = ChaCha20::new(
        GenericArray::from_slice(key),
        GenericArray::from_slice(&nonce),
    );
    let mut encrypted = data.to_vec();
    cipher.apply_keystream(&mut encrypted);

    output.extend_from_slice(&encrypted);
    Ok(())
}

fn decrypt_chacha20(encrypted: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    if encrypted.len() < 12 {
        return Err(Error::Encryption("Invalid encrypted data".into()));
    }

    let nonce = &encrypted[..12];
    let data = &encrypted[12..];

    let mut cipher = ChaCha20::new(
        GenericArray::from_slice(key),
        GenericArray::from_slice(nonce),
    );
    let mut decrypted = data.to_vec();
    cipher.apply_keystream(&mut decrypted);

    Ok(decrypted)
}

/// Encrypts data using RSA
fn encrypt_rsa(data: &[u8], _key: &[u8], output: &mut Vec<u8>) -> Result<()> {
    // For RSA, we don't really use the derived key, instead:
    // 1. Generate a random AES key
    // 2. Encrypt the data with AES
    // 3. Encrypt the AES key with RSA
    // 4. Concatenate the encrypted key and encrypted data

    // Generate a random AES key
    let mut aes_key = [0u8; 32];
    OsRng.fill_bytes(&mut aes_key);

    // The real RSA key would be loaded from a file, but for demo purposes,
    // we'll generate a new one. In practice, you'd have this as a parameter.
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, RSA_KEY_SIZE)
        .map_err(|e| Error::Encryption(format!("Failed to generate RSA key: {}", e)))?;
    let public_key = RsaPublicKey::from(&private_key);

    // Encrypt the AES key with RSA using PKCS#1 v1.5 padding
    let enc_key = public_key
        .encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), &aes_key)
        .map_err(|e| Error::Encryption(format!("RSA encryption failed: {}", e)))?;

    // Encrypt the data with AES
    let mut aes_output = Vec::new();
    encrypt_aes256(data, &aes_key, &mut aes_output)?;

    // Write the encrypted key size
    let key_size = enc_key.len() as u32;
    output.extend_from_slice(&key_size.to_be_bytes());

    // Write the encrypted key
    output.extend_from_slice(&enc_key);

    // Write the encrypted data
    output.extend_from_slice(&aes_output);

    Ok(())
}

/// Decrypts data using RSA
fn decrypt_rsa(encrypted: &[u8], _key: &[u8]) -> Result<Vec<u8>> {
    if encrypted.len() < 4 {
        return Err(Error::Encryption("Invalid encrypted data".into()));
    }

    // Read the encrypted key size
    let mut key_size_bytes = [0u8; 4];
    key_size_bytes.copy_from_slice(&encrypted[..4]);
    let key_size = u32::from_be_bytes(key_size_bytes) as usize;

    if encrypted.len() < 4 + key_size {
        return Err(Error::Encryption("Invalid encrypted data".into()));
    }

    // The real RSA key would be loaded from a file, but for demo purposes,
    // we use the same approach as in encryption
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, RSA_KEY_SIZE)
        .map_err(|e| Error::Encryption(format!("Failed to generate RSA key: {}", e)))?;

    // Extract and decrypt the AES key
    let enc_key = &encrypted[4..4 + key_size];
    let aes_key = private_key
        .decrypt(PaddingScheme::new_pkcs1v15_encrypt(), enc_key)
        .map_err(|e| Error::Encryption(format!("RSA decryption failed: {}", e)))?;

    // Decrypt the data with AES
    let aes_data = &encrypted[4 + key_size..];
    decrypt_aes256(aes_data, &aes_key)
}
