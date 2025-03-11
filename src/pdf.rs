//! PDF Module
//!
//! This module provides functionality for working with PDF files.

use crate::integrity;
use crate::{Error, Result};
use lopdf::{Dictionary, Document, Object, Stream};

/// Handler for PDF operations
pub struct PdfHandler {
    doc: Document,
}

impl PdfHandler {
    /// Creates a new PDF handler
    pub fn new(path: &str) -> Result<Self> {
        let doc = Document::load(path)
            .map_err(|e| Error::InvalidInput(format!("Failed to load PDF: {}", e)))?;
        Ok(Self { doc })
    }

    /// Embeds data into the PDF
    pub fn embed_data(&mut self, data: &[u8]) -> Result<()> {
        // Add HMAC for integrity checking
        let hmac_key = integrity::generate_integrity_key()?;
        let hmac = integrity::generate_hmac(data, &hmac_key)?;

        // Combine HMAC, key, and data
        let mut payload = Vec::with_capacity(hmac.len() + hmac_key.len() + data.len());

        // Format: [HMAC (32 bytes)][HMAC Key (32 bytes)][Original Data]
        payload.extend_from_slice(&hmac);
        payload.extend_from_slice(&hmac_key);
        payload.extend_from_slice(data);

        // Prepare metadata dictionary
        let mut metadata = Dictionary::new();
        metadata.set("Type", Object::Name("Metadata".as_bytes().to_vec()));
        metadata.set("Subtype", Object::Name("XML".as_bytes().to_vec()));

        let mut stream = Stream::new(Dictionary::new(), payload);
        stream
            .dict
            .set("Type", Object::Name("EmbeddedFile".as_bytes().to_vec()));

        let metadata_ref = self.doc.add_object(Object::Stream(stream));
        let catalog = self
            .doc
            .catalog_mut()
            .map_err(|e| Error::InvalidInput(format!("Failed to get PDF catalog: {}", e)))?;
        catalog.set("Metadata", Object::Reference(metadata_ref));

        Ok(())
    }

    /// Extracts embedded data from the PDF
    pub fn extract_data(&self) -> Result<Vec<u8>> {
        let catalog = self
            .doc
            .catalog()
            .map_err(|e| Error::InvalidInput(format!("Failed to get PDF catalog: {}", e)))?;

        let metadata = catalog
            .get(b"Metadata")
            .map_or(Err(Error::InvalidInput("Metadata not found".into())), Ok)?;

        // Get reference from metadata object
        if let Object::Reference(reference) = metadata {
            // Get the actual object using the reference
            match self
                .doc
                .get_object(*reference)
                .map_err(|e| Error::InvalidInput(format!("Failed to get metadata object: {}", e)))?
            {
                Object::Stream(ref stream) => {
                    let payload = &stream.content;

                    // The payload should be at least 64 bytes (32 for HMAC, 32 for key)
                    if payload.len() < 64 {
                        return Err(Error::Integrity("Invalid data format".into()));
                    }

                    // Extract HMAC, key, and data
                    let hmac = &payload[0..32];
                    let hmac_key = &payload[32..64];
                    let data = &payload[64..];

                    // Verify HMAC
                    if !integrity::verify_hmac(data, hmac_key, hmac)? {
                        return Err(Error::Integrity("Data integrity check failed".into()));
                    }

                    Ok(data.to_vec())
                }
                _ => Err(Error::InvalidInput("Invalid metadata format".into())),
            }
        } else {
            Err(Error::InvalidInput("Metadata is not a reference".into()))
        }
    }

    /// Saves the PDF to a file
    pub fn save(&mut self, path: &str) -> Result<()> {
        self.doc
            .save(path)
            .map_err(|e| Error::InvalidInput(format!("Failed to save PDF: {}", e)))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_pdf_embed_extract() {
        // Create a temp PDF file
        let pdf_data = include_bytes!("../tests/fixtures/test.pdf");
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(pdf_data).unwrap();
        let temp_path = temp_file.path().to_str().unwrap().to_string();

        // Test data
        let test_data = b"This is a test message for PDF steganography";

        // Embed data
        let mut handler = PdfHandler::new(&temp_path).unwrap();
        handler.embed_data(test_data).unwrap();

        let output_path = temp_file
            .path()
            .with_extension("output.pdf")
            .to_str()
            .unwrap()
            .to_string();
        handler.save(&output_path).unwrap();

        // Extract data
        let handler = PdfHandler::new(&output_path).unwrap();
        let extracted = handler.extract_data().unwrap();

        assert_eq!(extracted, test_data);
    }
}
