use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use tempfile::tempfile;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

use super::JarFile;

const ASSET_CONTENT: &[u8] = b"unchanged";
const ASSET_PATH: &str = "BOOT-INF/classes/static/asset.txt";
const EXPECTED_ENTRY_COUNT: usize = 2;
const LONG_VERSION_LENGTH: usize = 8_192;
const NEW_VERSION: &str = "2.4.0";
const PROPERTIES_PATH: &str = "BOOT-INF/classes/application.properties";
const RETAINED_PROPERTY: &str = "retained";
const RETAINED_VALUE: &str = "value";
const SOFTWARE_VERSION: &str = "softwareVersion";

#[test]
fn set_version_overwrites_owned_storage() -> Result<(), Box<dyn Error>> {
    let old_version = "x".repeat(LONG_VERSION_LENGTH);
    let storage = archive_with_version(&old_version)?;
    let original_length = storage.metadata()?.len();
    let mut jar_file = JarFile::new(storage);

    jar_file.set_version(NEW_VERSION)?;

    let versions = jar_file.versions()?;
    assert_eq!(
        versions.get(&PathBuf::from(PROPERTIES_PATH)),
        Some(&Some(NEW_VERSION.to_string()))
    );

    let storage = jar_file.into_inner();
    assert!(storage.metadata()?.len() < original_length);

    let mut archive = ZipArchive::new(storage)?;
    assert_eq!(archive.len(), EXPECTED_ENTRY_COUNT);

    let properties_entry = archive.by_name(PROPERTIES_PATH)?;
    assert_eq!(properties_entry.compression(), CompressionMethod::Stored);
    let properties = java_properties::read(properties_entry)?;
    assert_eq!(
        properties.get(SOFTWARE_VERSION).map(String::as_str),
        Some(NEW_VERSION)
    );
    assert_eq!(
        properties.get(RETAINED_PROPERTY).map(String::as_str),
        Some(RETAINED_VALUE)
    );

    let mut asset_entry = archive.by_name(ASSET_PATH)?;
    assert_eq!(asset_entry.compression(), CompressionMethod::Deflated);
    let mut asset = Vec::new();
    asset_entry.read_to_end(&mut asset)?;
    assert_eq!(asset, ASSET_CONTENT);

    Ok(())
}

fn archive_with_version(version: &str) -> Result<File, Box<dyn Error>> {
    let storage = tempfile()?;
    let mut writer = ZipWriter::new(storage);
    let properties_options =
        SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
    let asset_options =
        SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
    let properties =
        format!("{SOFTWARE_VERSION}={version}\n{RETAINED_PROPERTY}={RETAINED_VALUE}\n");

    writer.start_file(PROPERTIES_PATH, properties_options)?;
    writer.write_all(properties.as_bytes())?;
    writer.start_file(ASSET_PATH, asset_options)?;
    writer.write_all(ASSET_CONTENT)?;

    Ok(writer.finish()?)
}
