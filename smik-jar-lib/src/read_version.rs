use std::collections::{BTreeMap, HashMap};
use std::io::{Read, Seek};
use std::path::{Path, PathBuf};

use log::{error, warn};
use zip::ZipArchive;

use crate::{BOOT_INF_CLASSES, EntriesMut, PROPERTIES_FILES, SOFTWARE_VERSION};

/// Extension trait to represent a JAR file.
pub trait ReadVersion<T> {
    /// Returns an iterator over the JAR's `application*.properties` files.
    #[must_use]
    fn properties_files(&mut self) -> EntriesMut<'_, T>;

    /// Returns the JAR file's properties files as a map of path to properties.
    #[must_use]
    fn get_properties(&mut self) -> BTreeMap<PathBuf, HashMap<String, String>>;

    /// Returns a map of the properties files' names and versions stored therein.
    #[must_use]
    fn versions(&mut self) -> BTreeMap<PathBuf, Option<String>> {
        self.get_properties()
            .into_iter()
            .map(|(path, properties)| {
                (
                    path,
                    properties.get(SOFTWARE_VERSION).map(ToString::to_string),
                )
            })
            .collect()
    }
}

impl<T> ReadVersion<T> for ZipArchive<T>
where
    T: Read + Seek,
{
    /// Returns an iterator over the JAR file's properties files.
    fn properties_files(&mut self) -> EntriesMut<'_, T> {
        let file_names = PROPERTIES_FILES
            .iter()
            .map(|properties_file| Path::new(BOOT_INF_CLASSES).join(properties_file))
            .filter_map(|path| {
                path.to_str().map(ToOwned::to_owned).map_or_else(
                    || {
                        error!("Invalid UTF-8 in properties file path: {}", path.display());
                        None
                    },
                    Some,
                )
            })
            .filter_map(|file_name| {
                self.by_name(&file_name)
                    .inspect_err(|error| warn!("Missing file {file_name} in ZIP archive: {error}"))
                    .ok()
                    .map(|_| PathBuf::from(file_name))
            })
            .collect();

        EntriesMut::new(self, file_names)
    }

    fn get_properties(&mut self) -> BTreeMap<PathBuf, HashMap<String, String>> {
        let mut zip_files = self.properties_files();
        let mut properties_files = BTreeMap::new();

        while let Some((path, zip_file)) = zip_files.next() {
            let Ok(entry) = zip_file.inspect_err(|error| {
                warn!(
                    "Error while reading file {} from ZIP archive: {error}",
                    path.display()
                );
            }) else {
                continue;
            };

            let Ok(properties) = java_properties::read(entry)
                .inspect_err(|error| error!("Error parsing properties: {error}"))
            else {
                continue;
            };

            properties_files.insert(path, properties);
        }

        properties_files
    }
}
