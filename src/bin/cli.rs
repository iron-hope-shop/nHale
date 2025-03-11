use clap::{Parser, Subcommand};
use nhale::{
    embedding::{embed_data, EmbedConfig, EmbeddingConfig, MediaType},
    encryption::{Algorithm, CryptoConfig},
    extraction::ExtractConfig,
    utils::{detect_file_format, FileFormat},
    Error, Result,
};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "nHale Contributors")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Embed data in a file
    Embed {
        /// Input file
        #[clap(short, long)]
        input: PathBuf,

        /// Output file
        #[clap(short, long)]
        output: PathBuf,

        /// Data to embed
        #[clap(short, long)]
        data: String,

        /// Optional password for encryption
        #[clap(short, long)]
        password: Option<String>,

        /// Encryption algorithm (aes256, chacha20, or rsa)
        #[clap(short, long, default_value = "aes256")]
        algorithm: String,

        /// Force a specific file format
        #[clap(short, long)]
        format: Option<String>,

        /// LSB bit depth for image steganography (1-4, default: 1)
        #[clap(long, default_value = "1")]
        bit_depth: u8,

        /// Compression level for data (0-9, default: 6)
        #[clap(long, default_value = "6")]
        compression: u8,

        /// Advanced configuration options in key=value format
        #[clap(short, long, value_parser = parse_key_val)]
        config: Vec<(String, String)>,
    },

    /// Extract data from a file
    Extract {
        /// Input file
        #[clap(short, long)]
        input: PathBuf,

        /// Optional password for decryption
        #[clap(short, long)]
        password: Option<String>,

        /// Encryption algorithm (aes256, chacha20, or rsa)
        #[clap(short, long, default_value = "aes256")]
        algorithm: String,

        /// Force a specific file format
        #[clap(short, long)]
        format: Option<String>,

        /// LSB bit depth for image steganography (1-4, default: 1)
        #[clap(long, default_value = "1")]
        bit_depth: u8,

        /// Output file (optional, otherwise print to stdout)
        #[clap(short, long)]
        output: Option<PathBuf>,

        /// Advanced configuration options in key=value format
        #[clap(short, long, value_parser = parse_key_val)]
        config: Vec<(String, String)>,
    },

    /// Create watermark
    Watermark {
        /// Input file
        #[clap(short, long)]
        input: PathBuf,

        /// Output file
        #[clap(short, long)]
        output: PathBuf,

        /// Watermark data
        #[clap(short, long)]
        data: String,

        /// Watermark strength (0.0-1.0)
        #[clap(short, long, default_value = "0.5")]
        strength: f32,

        /// Visible watermark (default is invisible)
        #[clap(short, long)]
        visible: bool,
    },

    /// Verify watermark
    VerifyWatermark {
        /// Input file
        #[clap(short, long)]
        input: PathBuf,

        /// Expected watermark data
        #[clap(short, long)]
        data: String,
    },

    /// Detect steganography in files
    Detect {
        /// Input file
        #[clap(short, long)]
        input: PathBuf,

        /// Detection sensitivity (0.0-1.0)
        #[clap(short, long, default_value = "0.5")]
        sensitivity: f32,
    },
}

/// Parse a key-value pair in the format "key=value"
fn parse_key_val(s: &str) -> Result<(String, String)> {
    let pos = s.find('=').ok_or_else(|| {
        Error::InvalidInput(format!("Invalid key=value: no `=` found in `{}`", s))
    })?;
    Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
}

fn main() -> Result<()> {
    // Initialize the logger
    env_logger::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Embed {
            input,
            output,
            data,
            password,
            algorithm,
            format,
            bit_depth,
            compression,
            config,
        } => {
            if !(1..=4).contains(&bit_depth) {
                eprintln!("Error: Bit depth must be between 1 and 4, got {}", bit_depth);
                return Ok(());
            }

            if compression > 9 {
                return Err(Error::InvalidInput(
                    "Compression level must be between 0 and 9".into(),
                ));
            }

            let file_format = if let Some(fmt) = format {
                match fmt.to_lowercase().as_str() {
                    "png" => FileFormat::Png,
                    "jpg" | "jpeg" => FileFormat::Jpg,
                    "bmp" => FileFormat::Bmp,
                    "gif" => FileFormat::Gif,
                    "wav" => FileFormat::Wav,
                    "mp3" => FileFormat::Mp3,
                    "mp4" => FileFormat::Mp4,
                    "pdf" => FileFormat::Pdf,
                    _ => return Err(Error::InvalidInput("Unsupported file format".into())),
                }
            } else {
                detect_file_format(&input)
            };

            let encryption = password.map(|pass| CryptoConfig {
                algorithm: match algorithm.as_str() {
                    "aes256" => Algorithm::Aes256,
                    "chacha20" => Algorithm::ChaCha20,
                    "rsa" => Algorithm::Rsa,
                    _ => panic!("Invalid algorithm"),
                },
                password: pass,
            });

            // Create a parameters map
            let mut parameters = HashMap::new();
            parameters.insert("bit_depth".to_string(), bit_depth.to_string());
            parameters.insert("compression".to_string(), compression.to_string());

            // Add any additional config parameters
            for (key, value) in config {
                parameters.insert(key, value);
            }

            let _embedding_config = EmbeddingConfig {
                media_type: match file_format {
                    FileFormat::Png | FileFormat::Jpg | FileFormat::Bmp | FileFormat::Gif => {
                        MediaType::Image
                    }
                    FileFormat::Wav | FileFormat::Mp3 => MediaType::Audio,
                    FileFormat::Mp4 => MediaType::Video,
                    FileFormat::Pdf => MediaType::Pdf,
                    _ => return Err(Error::InvalidInput("Unsupported media type".into())),
                },
                use_encryption: encryption.is_some(),
                password: encryption.as_ref().map(|c| c.password.clone()),
                parameters,
            };

            let config = EmbedConfig {
                input_path: input.to_str().unwrap().to_string(),
                output_path: output.to_str().unwrap().to_string(),
                data: data.into_bytes(),
                encryption,
            };

            match file_format {
                FileFormat::Pdf => embed_data(config)?,
                FileFormat::Png => nhale::embedding::embed_in_png(config)?,
                FileFormat::Jpg => nhale::embedding::embed_in_jpg(config)?,
                FileFormat::Wav => nhale::embedding::embed_in_wav(config)?,
                FileFormat::Mp3 => nhale::embedding::embed_in_mp3(config)?,
                FileFormat::Mp4 => nhale::embedding::embed_in_mp4(config)?,
                _ => return Err(Error::InvalidInput("Unsupported file format".into())),
            }

            println!("Data successfully embedded in {}", output.display());
            Ok(())
        }

        Commands::Extract {
            input,
            password,
            algorithm,
            format,
            bit_depth,
            output,
            config,
        } => {
            if !(1..=4).contains(&bit_depth) {
                eprintln!("Error: Bit depth must be between 1 and 4, got {}", bit_depth);
                return Ok(());
            }

            let file_format = if let Some(fmt) = format {
                match fmt.to_lowercase().as_str() {
                    "png" => FileFormat::Png,
                    "jpg" | "jpeg" => FileFormat::Jpg,
                    "bmp" => FileFormat::Bmp,
                    "gif" => FileFormat::Gif,
                    "wav" => FileFormat::Wav,
                    "mp3" => FileFormat::Mp3,
                    "mp4" => FileFormat::Mp4,
                    "pdf" => FileFormat::Pdf,
                    _ => return Err(Error::InvalidInput("Unsupported file format".into())),
                }
            } else {
                detect_file_format(&input)
            };

            // Create a parameters map
            let mut parameters = HashMap::new();
            parameters.insert("bit_depth".to_string(), bit_depth.to_string());

            // Add any additional config parameters
            for (key, value) in config {
                parameters.insert(key, value);
            }

            let config = ExtractConfig {
                input_path: input.to_str().unwrap().to_string(),
                encryption: password.map(|pass| CryptoConfig {
                    algorithm: match algorithm.as_str() {
                        "aes256" => Algorithm::Aes256,
                        "chacha20" => Algorithm::ChaCha20,
                        "rsa" => Algorithm::Rsa,
                        _ => panic!("Invalid algorithm"),
                    },
                    password: pass,
                }),
                parameters: Some(parameters),
            };

            let final_data = match file_format {
                FileFormat::Pdf => nhale::extraction::extract_from_pdf(config)?,
                FileFormat::Png => nhale::extraction::extract_from_png(config)?,
                FileFormat::Jpg => nhale::extraction::extract_from_jpg(config)?,
                FileFormat::Wav => nhale::extraction::extract_from_wav(config)?,
                FileFormat::Mp3 => nhale::extraction::extract_from_mp3(config)?,
                FileFormat::Mp4 => nhale::extraction::extract_from_mp4(config)?,
                _ => return Err(Error::InvalidInput("Unsupported file format".into())),
            };

            // If an output file is specified, write the extracted data to it
            if let Some(output_path) = output {
                std::fs::write(&output_path, &final_data)
                    .map_err(|e| Error::Io(format!("Failed to write output file: {}", e)))?;
                println!("Extracted data written to {}", output_path.display());
            } else {
                // Otherwise print to stdout
                match String::from_utf8(final_data.clone()) {
                    Ok(text) => println!("Extracted message: {}", text),
                    Err(_) => println!("Extracted binary data of {} bytes", final_data.len()),
                }
            }

            Ok(())
        }

        Commands::Watermark {
            input: _,
            output: _,
            data: _,
            strength: _,
            visible: _,
        } => {
            println!("Watermarking feature is not implemented yet");
            Err(Error::NotImplemented(
                "Watermarking not yet implemented".into(),
            ))
        }

        Commands::VerifyWatermark { input: _, data: _ } => {
            println!("Watermark verification feature is not implemented yet");
            Err(Error::NotImplemented(
                "Watermark verification not yet implemented".into(),
            ))
        }

        Commands::Detect { input: _, sensitivity: _ } => {
            println!("Steganography detection feature is not implemented yet");
            Err(Error::NotImplemented(
                "Steganography detection not yet implemented".into(),
            ))
        }
    }
}
