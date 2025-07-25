use std::collections::{BTreeMap, HashMap};
use std::io::{Read, Seek, Write};
use std::path::PathBuf;

use log::{debug, warn};
use zip::result::ZipError;
use zip::write::SimpleFileOptions;
use zip::{ZipArchive, ZipWriter};

use crate::JarError;

pub trait CopyPartial {
    /// Copies the specified properties files from the JAR archive to a new buffer.
    fn copy_partial<T>(
        &mut self,
        src: &mut ZipArchive<T>,
        exclude: Vec<String>,
    ) -> Result<BTreeMap<PathBuf, SimpleFileOptions>, ZipError>
    where
        T: Read + Seek;

    /// Adds a file with the specified properties to the JAR archive.
    fn add_files(
        &mut self,
        properties: BTreeMap<PathBuf, HashMap<String, String>>,
        options: BTreeMap<PathBuf, SimpleFileOptions>,
    ) -> Result<(), JarError>;

    /// Replaces the contents of the JAR archive with the specified properties files.
    fn replace<T>(
        self,
        src: &mut ZipArchive<T>,
        properties: BTreeMap<PathBuf, HashMap<String, String>>,
    ) -> Result<(), JarError>
    where
        Self: Sized,
        T: Read + Seek;
}

impl<W> CopyPartial for ZipWriter<W>
where
    W: Write + Seek,
{
    fn copy_partial<T>(
        &mut self,
        src: &mut ZipArchive<T>,
        exclude: Vec<String>,
    ) -> Result<BTreeMap<PathBuf, SimpleFileOptions>, ZipError>
    where
        T: Read + Seek,
    {
        let mut file_buffer = Vec::new();
        let mut options = BTreeMap::new();
        let files: Vec<_> = src.file_names().map(ToOwned::to_owned).collect();

        for file in files {
            let mut entry = src.by_name(&file)?;

            if exclude.contains(&file) {
                debug!("Excluding file: {file}");
                options.insert(PathBuf::from(file), entry.options());
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
            self.start_file(
                path.to_string_lossy(),
                options.remove(&path).unwrap_or_default(),
            )?;
            java_properties::write(&mut *self, &properties)?;
        }

        Ok(())
    }

    fn replace<T>(
        mut self,
        src: &mut ZipArchive<T>,
        properties: BTreeMap<PathBuf, HashMap<String, String>>,
    ) -> Result<(), JarError>
    where
        T: Read + Seek,
    {
        let options = self.copy_partial(
            src,
            properties
                .keys()
                .filter_map(|path| path.to_str().map(ToOwned::to_owned))
                .collect(),
        )?;
        self.add_files(properties, options)?;
        self.finish()?;
        Ok(())
    }
}
