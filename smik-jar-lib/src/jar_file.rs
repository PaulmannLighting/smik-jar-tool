use std::collections::BTreeMap;
use std::io::{Cursor, Read, Seek, Write};
use std::path::PathBuf;

use log::{info, warn};
use zip::result::ZipResult;
use zip::{ZipArchive, ZipWriter};

use crate::update_jar::UpdateJar;
use crate::{JarError, ReadVersion, SOFTWARE_VERSION};

/// A JAR archive that can be inspected or reconstructed.
///
/// Reading requires the wrapped value to implement [`Read`] and [`Seek`].
/// Updating additionally requires [`Write`].
pub struct JarFile<T> {
    inner: T,
}

impl<T> JarFile<T> {
    /// Wraps a JAR archive source.
    pub const fn new(inner: T) -> Self {
        Self { inner }
    }

    /// Consumes the wrapper and returns the archive source.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> JarFile<T>
where
    T: Read + Seek,
{
    /// Returns versions stored in the recognized properties files.
    ///
    /// Each map value is `None` when its properties file exists but does not
    /// define `softwareVersion`.
    ///
    /// # Errors
    ///
    /// Returns a [`ZipError`](zip::result::ZipError) if the JAR archive cannot
    /// be opened.
    pub fn versions(&mut self) -> ZipResult<BTreeMap<PathBuf, Option<String>>> {
        ZipArchive::new(&mut self.inner).map(|mut zip_archive| zip_archive.versions())
    }
}

impl<T> JarFile<T>
where
    T: Write + Read + Seek,
{
    /// Sets the version in every recognized properties file.
    ///
    /// Returns the complete reconstructed archive as bytes. This method does
    /// not overwrite the wrapped source.
    ///
    /// # Errors
    ///
    /// Returns a [`JarError`] if the archive cannot be read or reconstructed,
    /// or if a properties file cannot be serialized.
    pub fn set_version(&mut self, version: &impl ToString) -> Result<Vec<u8>, JarError> {
        let mut zip_archive = ZipArchive::new(&mut self.inner)?;
        let mut buffer: Vec<u8> = Vec::new();

        let mut properties = zip_archive.properties();

        for (path, properties) in &mut properties {
            if let Some(current_version) = properties.get(SOFTWARE_VERSION) {
                info!(
                    "Updating version in {}: {current_version} -> {}",
                    path.display(),
                    version.to_string()
                );
            } else {
                warn!(
                    "No version found in {}. Adding version: {}",
                    path.display(),
                    version.to_string()
                );
            }

            properties.insert(SOFTWARE_VERSION.into(), version.to_string());
        }

        let mut zip_writer = ZipWriter::new(Cursor::new(&mut buffer));
        zip_writer.replace(&mut zip_archive, properties)?;
        zip_writer.finish()?;
        Ok(buffer)
    }
}
