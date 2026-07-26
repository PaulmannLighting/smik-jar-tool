use std::collections::{BTreeMap, HashMap};
use std::io::{Read, Seek, Write};
use std::path::PathBuf;

use log::{debug, warn};
use zip::result::ZipError;
use zip::write::SimpleFileOptions;
use zip::{ZipArchive, ZipWriter};

use crate::JarError;

/// Reconstructs a JAR archive with selected properties files replaced.
pub trait UpdateJar {
    /// Copies entries from the given [`ZipArchive`] except for `exclude`.
    fn copy_partial<T>(
        &mut self,
        src: &mut ZipArchive<T>,
        exclude: Vec<PathBuf>,
    ) -> Result<BTreeMap<PathBuf, SimpleFileOptions>, ZipError>
    where
        T: Read + Seek;

    /// Adds properties files with their respective ZIP options.
    fn add_files(
        &mut self,
        properties: BTreeMap<PathBuf, HashMap<String, String>>,
        options: BTreeMap<PathBuf, SimpleFileOptions>,
    ) -> Result<(), JarError>;

    /// Copies an archive while replacing the specified properties files.
    fn replace<T>(
        &mut self,
        src: &mut ZipArchive<T>,
        properties: BTreeMap<PathBuf, HashMap<String, String>>,
    ) -> Result<(), JarError>
    where
        T: Read + Seek,
    {
        let options = self.copy_partial(src, properties.keys().cloned().collect())?;
        self.add_files(properties, options)?;
        Ok(())
    }
}

impl<W> UpdateJar for ZipWriter<W>
where
    W: Write + Seek,
{
    fn copy_partial<T>(
        &mut self,
        src: &mut ZipArchive<T>,
        exclude: Vec<PathBuf>,
    ) -> Result<BTreeMap<PathBuf, SimpleFileOptions>, ZipError>
    where
        T: Read + Seek,
    {
        let mut file_buffer = Vec::new();
        let mut options = BTreeMap::new();
        let files: Vec<_> = src.file_names().map(ToOwned::to_owned).collect();

        for file in files {
            let mut entry = src.by_name(&file)?;

            if let Some(path) = entry.enclosed_name()
                && exclude.contains(&path)
            {
                debug!("Excluding file: {file}");
                options.insert(path, entry.options());
                continue;
            }

            if entry.is_file() {
                debug!("Copying file: {}", entry.name());
                file_buffer.clear();
                entry.read_to_end(&mut file_buffer)?;
                self.start_file(entry.name(), entry.options())?;
                self.write_all(&file_buffer)?;
            } else if entry.is_dir() {
                debug!("Creating directory: {}", entry.name());
                self.add_directory(entry.name(), entry.options())?;
            } else {
                warn!("Skipping unsupported entry: {}", entry.name());
            }
        }

        Ok(options)
    }

    fn add_files(
        &mut self,
        properties: BTreeMap<PathBuf, HashMap<String, String>>,
        mut options: BTreeMap<PathBuf, SimpleFileOptions>,
    ) -> Result<(), JarError> {
        for (path, properties) in properties {
            self.start_file_from_path(&path, options.remove(&path).unwrap_or_default())?;
            java_properties::write(&mut *self, &properties)?;
        }

        Ok(())
    }
}
