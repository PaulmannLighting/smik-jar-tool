use std::collections::{BTreeMap, HashMap};
use std::io::{Read, Seek};
use std::path::PathBuf;

use log::{error, warn};
use zip::ZipArchive;

use crate::by_path::ByPath;
use crate::{SOFTWARE_VERSION, properties_files};

/// Extension trait to represent a JAR file.
pub trait ReadVersion<T> {
    /// Returns the JAR file's properties files as a map of path to properties.
    #[must_use]
    fn properties(&mut self) -> BTreeMap<PathBuf, HashMap<String, String>>;

    /// Returns a map of the properties files' names and versions stored therein.
    #[must_use]
    fn versions(&mut self) -> BTreeMap<PathBuf, Option<String>> {
        self.properties()
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
    fn properties(&mut self) -> BTreeMap<PathBuf, HashMap<String, String>> {
        let mut properties_files_contents = BTreeMap::new();

        for path in properties_files() {
            let Ok(entry) = self.by_path(&path).inspect_err(|error| {
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

            properties_files_contents.insert(path, properties);
        }

        if properties_files_contents.is_empty() {
            warn!("No properties files found in JAR archive.");
        }

        properties_files_contents
    }
}
