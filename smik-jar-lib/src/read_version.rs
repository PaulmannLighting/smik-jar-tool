use crate::{BOOT_INF_CLASSES, EntriesMut, SOFTWARE_VERSION};
use log::{error, warn};
use semver::Version;
use std::collections::{BTreeMap, HashMap};
use std::io::{Read, Seek};
use std::path::{Path, PathBuf};
use zip::ZipArchive;

const PROPERTIES_FILES: [&str; 5] = [
    "application.properties",
    "application-dev.properties",
    "application-int.properties",
    "application-local.properties",
    "application-prod.properties",
];

/// Extension trait to represent a JAR file.
pub trait ReadVersion<T> {
    /// Returns an iterator over the JAR file's properties files.
    #[must_use]
    fn properties_files(&mut self) -> EntriesMut<'_, T>;

    /// Returns the JAR file's properties files as a map of path to properties.
    fn get_properties(&mut self) -> BTreeMap<PathBuf, HashMap<String, String>>
    where
        T: Read + Seek,
    {
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

    /// Returns the JAR file's version.
    ///
    /// # Errors
    /// This function will return `None` if the JAR file's version could not be parsed.
    #[must_use]
    fn versions(&mut self) -> BTreeMap<PathBuf, Version>
    where
        T: Read + Seek,
    {
        self.get_properties()
            .into_iter()
            .filter_map(|(path, properties)| {
                properties
                    .get(SOFTWARE_VERSION)
                    .map(|version_string| (path, version_string))
                    .and_then(|(path, version)| {
                        Version::parse(version)
                            .inspect_err(|error| error!("Invalid version: {error}"))
                            .ok()
                            .map(|version| (path, version))
                    })
            })
            .collect()
    }
}

impl<T> ReadVersion<T> for ZipArchive<T>
where
    T: Read + Seek,
{
    fn properties_files(&mut self) -> EntriesMut<'_, T> {
        let file_names = PROPERTIES_FILES
            .iter()
            .map(|properties_file| Path::new(BOOT_INF_CLASSES).join(properties_file))
            .filter_map(|path| path.to_str().map(ToOwned::to_owned))
            .filter_map(|file_name| {
                self.by_name(&file_name)
                    .ok()
                    .map(|_| PathBuf::from(file_name))
            })
            .collect();

        EntriesMut::new(self, file_names)
    }
}
