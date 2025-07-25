use crate::{BOOT_INF_CLASSES, EntriesMut};
use log::{error, warn};
use semver::Version;
use std::collections::BTreeMap;
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

    /// Returns the JAR file's version.
    ///
    /// # Errors
    /// This function will return `None` if the JAR file's version could not be parsed.
    #[must_use]
    fn versions(&mut self) -> BTreeMap<PathBuf, Version>
    where
        T: Read + Seek,
    {
        let mut versions = BTreeMap::new();
        let mut properties_files = self.properties_files();

        while let Some((path, version)) = next_properties_file(&mut properties_files) {
            versions.insert(path.clone(), version.clone());
        }

        versions
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

fn next_properties_file<T>(properties_files: &mut EntriesMut<'_, T>) -> Option<(PathBuf, Version)>
where
    T: Read + Seek,
{
    let (path, properties_file) = properties_files.next()?;

    let Ok(entry) = properties_file.inspect_err(|error| {
        warn!(
            "Error while reading file {} from ZIP archive: {error}",
            path.display()
        );
    }) else {
        return None;
    };

    let Ok(properties) = java_properties::read(entry)
        .inspect_err(|error| error!("Error parsing properties: {error}"))
    else {
        return None;
    };

    let Some(version) = properties.get("softwareVersion") else {
        error!("Missing softwareVersion in JAR file: {}", path.display());
        return None;
    };

    let Ok(version) = Version::parse(version)
        .inspect_err(|error| error!("Error parsing version {version}: {error}"))
    else {
        return None;
    };

    Some((path, version))
}
