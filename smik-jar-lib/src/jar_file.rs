use crate::ReadVersion;
use semver::Version;
use std::collections::BTreeMap;
use std::io::{Read, Seek};
use std::path::PathBuf;
use zip::ZipArchive;
use zip::result::ZipResult;

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
