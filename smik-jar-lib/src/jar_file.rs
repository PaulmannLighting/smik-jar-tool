use std::collections::BTreeMap;
use std::io::{Cursor, Read, Seek, Write};
use std::path::{Path, PathBuf};

use log::info;
use zip::result::ZipResult;
use zip::{ZipArchive, ZipWriter};

use crate::copy_partial::CopyPartial;
use crate::{BOOT_INF_CLASSES, JarError, PROPERTIES_FILES, ReadVersion, SOFTWARE_VERSION};

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
    pub fn versions(&mut self) -> ZipResult<BTreeMap<PathBuf, Option<String>>> {
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
    pub fn set_version(&mut self, version: &impl ToString) -> Result<(), JarError> {
        let mut zip_archive = ZipArchive::new(&mut self.inner)?;
        let properties_files: Vec<String> = PROPERTIES_FILES
            .iter()
            .map(|file_name| Path::new(BOOT_INF_CLASSES).join(file_name))
            .filter_map(|path| path.to_str().map(ToOwned::to_owned))
            .collect();
        let mut buffer: Vec<u8> = Vec::new();

        let mut properties = zip_archive.get_properties();
        let mut zip_writer = ZipWriter::new(Cursor::new(&mut buffer));
        let options = zip_writer.copy_partial(&mut zip_archive, properties_files)?;

        for (path, properties) in &mut properties {
            if let Some(current_version) = properties.get(SOFTWARE_VERSION) {
                info!(
                    "Updating version in {}: {current_version} -> {}",
                    path.display(),
                    version.to_string()
                );
            }

            properties.insert(SOFTWARE_VERSION.into(), version.to_string());
        }

        zip_writer.add_files(properties, options)?;
        zip_writer.finish()?;
        Ok(())
    }
}
