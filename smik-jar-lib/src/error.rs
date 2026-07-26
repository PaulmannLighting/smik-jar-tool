use std::io;

use java_properties::PropertiesError;
use thiserror::Error;
use zip::result::ZipError;

/// Error type for the JAR library.
#[derive(Debug, Error)]
pub enum JarError {
    /// An I/O error occurred.
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// An error occurred while reading or writing the ZIP archive.
    #[error("ZIP error: {0}")]
    Zip(#[from] ZipError),

    /// An error occurred while parsing Java properties.
    #[error("Error parsing Java properties: {0}")]
    JavaProperties(#[from] PropertiesError),
}
