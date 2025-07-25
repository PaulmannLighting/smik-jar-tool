use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::path::PathBuf;

use log::info;
use semver::Version;
use zip::result::ZipResult;
use zip::write::SimpleFileOptions;
use zip::{ZipArchive, ZipWriter};

use crate::{JarError, ReadVersion, SOFTWARE_VERSION};

/// API to a JAR file.
pub struct JarFile<T> {
    inner: T,
}

impl<T> JarFile<T> {
    /// Create a new JAR file.
    pub const fn new(inner: T) -> Self {
        Self { inner }
    }

    /// Returns the inner file object.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> JarFile<T>
where
    T: Read + Seek,
{
    /// Return the versions stored in the JAR file's properties files.
    ///
    /// # Errors
    ///
    /// Returns a [`ZipError`](zip::result::ZipError) if the JAR file could not be read.
    pub fn versions(&mut self) -> ZipResult<BTreeMap<PathBuf, Version>> {
        ZipArchive::new(&mut self.inner).map(|mut zip_archive| zip_archive.versions())
    }
}

impl<T> JarFile<T>
where
    T: Write + Read + Seek,
{
    /// Set the version in the JAR file's properties files.
    ///
    /// # Errors
    ///
    /// Returns a [`JarError`] if the JAR file could not be written to or if the properties could not be read.
    pub fn set_version(&mut self, version: &Version) -> Result<(), JarError> {
        let properties =
            ZipArchive::new(&mut self.inner).map(|mut zip_archive| zip_archive.get_properties())?;
        let mut zip_writer = ZipWriter::new_append(&mut self.inner)?;
        let options = SimpleFileOptions::default();

        for (path, properties) in properties {
            if let Some(current_version) = properties.get(SOFTWARE_VERSION) {
                info!(
                    "Updating version in {}: {current_version} -> {version}",
                    path.display()
                );
            }

            zip_writer.start_file(path.to_string_lossy(), options)?;
            java_properties::write(&mut zip_writer, &properties)?;
        }

        zip_writer.finish()?;
        Ok(())
    }
}
