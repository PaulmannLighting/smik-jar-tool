#![doc = include_str!("../README.md")]

use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use log::error;
use smik_jar_lib::JarFile;

/// Command-line arguments.
#[derive(Debug, Parser)]
#[clap(
    version,
    about,
    author,
    long_about = "Tool to read and update the version in a JAR file."
)]
struct Args {
    /// Path to the JAR archive.
    #[clap(index = 1, help = "Path to the JAR file")]
    jar_file: PathBuf,
    /// Version to write, or `None` to read the current versions.
    #[clap(
        index = 2,
        value_name = "VERSION",
        help = "The version to set in the JAR file"
    )]
    new_version: Option<String>,
}

/// Runs the requested read or update operation.
fn main() -> ExitCode {
    env_logger::init();
    let args = Args::parse();

    let Ok(jar_file) = OpenOptions::new()
        .read(true)
        .write(args.new_version.is_some())
        .open(&args.jar_file)
        .inspect_err(|error| error!("Error opening file: {error}"))
        .map(JarFile::new)
    else {
        return ExitCode::FAILURE;
    };

    if let Some(version) = args.new_version {
        replace_version(jar_file, &version)
    } else {
        read_versions(jar_file)
    }
}

/// Writes `version` to the JAR's supported properties files.
fn replace_version(mut jar_file: JarFile<File>, version: &str) -> ExitCode {
    let Ok(()) = jar_file
        .set_version(version)
        .inspect_err(|error| error!("Error setting version: {error}"))
    else {
        return ExitCode::FAILURE;
    };

    ExitCode::SUCCESS
}

/// Prints versions from the supported properties files in `path`.
fn read_versions(mut jar_file: JarFile<File>) -> ExitCode {
    let Ok(versions) = jar_file
        .versions()
        .inspect_err(|error| error!("Error reading versions: {error}"))
    else {
        return ExitCode::FAILURE;
    };

    for (path, version) in versions {
        if let Some(version) = version {
            println!("{}: {version}", path.display());
        } else {
            error!("{} does not have a version", path.display());
        }
    }

    ExitCode::SUCCESS
}
