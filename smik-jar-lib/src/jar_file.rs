use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read, Seek, Write, copy};
use std::path::PathBuf;

use log::{info, warn};
use tempfile::tempfile;
use zip::result::ZipResult;
use zip::{ZipArchive, ZipWriter};

use crate::update_jar::UpdateJar;
use crate::{JarError, ReadVersion, SOFTWARE_VERSION};

#[cfg(test)]
mod tests;

/// A JAR archive that can be inspected or reconstructed.
///
/// Reading requires the wrapped value to implement [`Read`] and [`Seek`].
/// Updating is available when the wrapped value is a [`File`].
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

impl JarFile<File> {
    /// Sets the version in every recognized properties file.
    ///
    /// The updated archive is reconstructed in an anonymous temporary file.
    /// After reconstruction succeeds, its contents replace the complete
    /// contents of the wrapped file. The temporary file is removed
    /// automatically, while the wrapped file is rewound and remains owned by
    /// this [`JarFile`].
    ///
    /// # Errors
    ///
    /// Returns a [`JarError`] if the archive cannot be read or reconstructed,
    /// if a properties file cannot be serialized, or if the wrapped file
    /// cannot be overwritten.
    pub fn set_version(&mut self, version: &str) -> Result<(), JarError> {
        let mut replacement = tempfile()?;

        {
            let mut zip_archive = ZipArchive::new(&mut self.inner)?;
            let mut properties = zip_archive.properties();

            for (path, properties) in &mut properties {
                if let Some(current_version) = properties.get(SOFTWARE_VERSION) {
                    info!(
                        "Updating version in {}: {current_version} -> {version}",
                        path.display()
                    );
                } else {
                    warn!(
                        "No version found in {}. Adding version: {version}",
                        path.display()
                    );
                }

                properties.insert(SOFTWARE_VERSION.into(), version.to_string());
            }

            let mut zip_writer = ZipWriter::new(&mut replacement);
            zip_writer.replace(&mut zip_archive, properties)?;
            zip_writer.finish()?;
        }

        replacement.rewind()?;
        self.inner.rewind()?;
        let replacement_length = copy(&mut replacement, &mut self.inner)?;
        self.inner.set_len(replacement_length)?;
        self.inner.rewind()?;
        self.inner.flush()?;

        Ok(())
    }
}
