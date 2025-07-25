//! Library to read JAR files and extract the software version from them.

use entries_mut::EntriesMut;
pub use error::JarError;
pub use jar_file::JarFile;
use read_version::ReadVersion;

mod copy_partial;
mod entries_mut;
mod error;
mod jar_file;
mod read_version;

const BOOT_INF_CLASSES: &str = "BOOT-INF/classes";
const SOFTWARE_VERSION: &str = "softwareVersion";
const PROPERTIES_FILES: [&str; 5] = [
    "application.properties",
    "application-dev.properties",
    "application-int.properties",
    "application-local.properties",
    "application-prod.properties",
];
