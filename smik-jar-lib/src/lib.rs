//! Library to read JAR files and extract the software version from them.

use entries_mut::EntriesMut;
pub use jar_file::JarFile;
use read_version::ReadVersion;
use write_version::WriteVersion;

mod entries_mut;
mod jar_file;
mod read_version;
mod write_version;

const BOOT_INF_CLASSES: &str = "BOOT-INF/classes";
