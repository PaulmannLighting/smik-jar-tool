use std::path::Path;

const SEPARATOR: &str = "/";

/// Extension trait to convert Paths to ZIP file paths.
pub trait ZipFilePath {
    /// Returns the path to a file in a ZIP archive.
    fn zip_file_path(&self) -> Option<String>;
}

impl<T> ZipFilePath for T
where
    T: AsRef<Path>,
{
    fn zip_file_path(&self) -> Option<String> {
        let mut components = Vec::new();

        for component in self.as_ref().components() {
            components.push(component.as_os_str().to_str()?);
        }

        Some(components.join(SEPARATOR))
    }
}
