//! Library to read JAR files and extract the software version from them.

use std::path::{Path, PathBuf};

pub use error::JarError;
pub use jar_file::JarFile;
use read_version::ReadVersion;

mod by_path;
mod error;
mod jar_file;
mod read_version;
mod update_jar;

const BOOT_INF: &str = "BOOT-INF";
const CLASSES: &str = "classes";
const SOFTWARE_VERSION: &str = "softwareVersion";
const PROPERTIES_FILES: [&str; 5] = [
    "application.properties",
    "application-dev.properties",
    "application-int.properties",
    "application-local.properties",
    "application-prod.properties",
];

/// Returns an iterator over the JAR file's properties files.
fn properties_files() -> impl Iterator<Item = PathBuf> {
    PROPERTIES_FILES.iter().map(|properties_file| {
        Path::new(BOOT_INF)
            .join(Path::new(CLASSES))
            .join(properties_file)
    })
}
