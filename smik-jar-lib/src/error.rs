use std::fmt::Display;
use std::io;
use std::path::PathBuf;

use java_properties::PropertiesError;
use zip::result::ZipError;

/// Error type for the JAR library.
#[derive(Debug)]
pub enum JarError {
    /// And I/O error occurred.
    Io(io::Error),
    /// An error occurred while reading or writing the ZIP archive.
    Zip(ZipError),
    /// An error occurred while parsing Java properties.
    JavaProperties(PropertiesError),
    /// A UTF-8 error occurred.
    Utf8(PathBuf),
}

impl Display for JarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "I/O error: {error}"),
            Self::Zip(error) => write!(f, "ZIP error: {error}"),
            Self::JavaProperties(error) => write!(f, "Error parsing Java properties: {error}"),
            Self::Utf8(path) => write!(f, "UTF-8 error: {}", path.display()),
        }
    }
}

impl std::error::Error for JarError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Zip(error) => Some(error),
            Self::JavaProperties(error) => Some(error),
            Self::Utf8(_) => None,
        }
    }
}

impl From<io::Error> for JarError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<ZipError> for JarError {
    fn from(error: ZipError) -> Self {
        Self::Zip(error)
    }
}

impl From<PropertiesError> for JarError {
    fn from(error: PropertiesError) -> Self {
        Self::JavaProperties(error)
    }
}

impl From<PathBuf> for JarError {
    fn from(path: PathBuf) -> Self {
        Self::Utf8(path)
    }
}
