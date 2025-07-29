//! Library to read JAR files and extract the software version from them.

use entries_mut::EntriesMut;
pub use error::JarError;
pub use jar_file::JarFile;
use read_version::ReadVersion;

mod entries_mut;
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
