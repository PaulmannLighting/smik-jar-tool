use semver::Version;

pub trait WriteVersion<T> {
    /// Returns the JAR file's version.
    ///
    /// # Errors
    /// This function will return `None` if the JAR file's version could not be parsed.
    #[must_use]
    fn set_version(&mut self, version: Version);
}
