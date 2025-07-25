use java_properties::PropertiesError;
use std::fmt::Display;

/// Error type for the JAR library.
#[derive(Debug)]
pub enum JarError {
    /// And I/O error occurred.
    Io(std::io::Error),
    /// An error occurred while reading or writing the ZIP archive.
    Zip(zip::result::ZipError),
    /// An error occurred while parsing Java properties.
    JavaProperties(PropertiesError),
}

impl Display for JarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "I/O error: {error}"),
            Self::Zip(error) => write!(f, "ZIP error: {error}"),
            Self::JavaProperties(error) => write!(f, "Error parsing Java properties: {error}"),
        }
    }
}

impl std::error::Error for JarError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Zip(error) => Some(error),
            Self::JavaProperties(error) => Some(error),
        }
    }
}

impl From<std::io::Error> for JarError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<zip::result::ZipError> for JarError {
    fn from(error: zip::result::ZipError) -> Self {
        Self::Zip(error)
    }
}

impl From<PropertiesError> for JarError {
    fn from(error: PropertiesError) -> Self {
        Self::JavaProperties(error)
    }
}
