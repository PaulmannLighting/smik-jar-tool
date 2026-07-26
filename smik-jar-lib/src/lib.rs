#![doc = include_str!("../README.md")]

use std::path::{Path, PathBuf};

pub use self::error::JarError;
pub use self::jar_file::JarFile;
use self::read_version::ReadVersion;

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

/// Returns the recognized properties-file paths within a JAR archive.
fn properties_files() -> impl Iterator<Item = PathBuf> {
    PROPERTIES_FILES.iter().map(|properties_file| {
        Path::new(BOOT_INF)
            .join(Path::new(CLASSES))
            .join(properties_file)
    })
}
