//! CLI tool for managing smik JAR file versions.

use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::Parser;
use log::error;
use smik_jar_lib::JarFile;

#[derive(Debug, Parser)]
#[clap(
    version,
    about,
    author,
    long_about = "Tool to read and update the version in a JAR file."
)]
struct Args {
    #[clap(index = 1, help = "Path to the JAR file")]
    jar_file: PathBuf,
    #[clap(index = 2, help = "The version to set in the JAR file")]
    version: Option<String>,
}

fn main() -> ExitCode {
    env_logger::init();
    let args = Args::parse();

    if let Some(version) = args.version {
        replace_version(&args.jar_file, &version)
    } else {
        read_versions(&args.jar_file)
    }
}

fn replace_version(path: &Path, version: &str) -> ExitCode {
    let Ok(src) = OpenOptions::new()
        .read(true)
        .open(path)
        .inspect_err(|error| error!("Error opening file: {error}"))
    else {
        return ExitCode::FAILURE;
    };

    let mut jar_file = JarFile::new(src);

    let Ok(new_file) = jar_file
        .set_version(&version)
        .inspect_err(|error| error!("Error setting version: {error}"))
    else {
        return ExitCode::FAILURE;
    };

    // Close the source file to ensure it is not blocked.
    drop(jar_file);

    let Ok(mut dst) = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .inspect_err(|error| error!("Error opening file: {error}"))
    else {
        return ExitCode::FAILURE;
    };

    if let Err(error) = dst.write_all(&new_file) {
        error!("Error writing to file: {error}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn read_versions(path: &Path) -> ExitCode {
    let Ok(src) = OpenOptions::new()
        .read(true)
        .open(path)
        .inspect_err(|error| error!("Error opening file: {error}"))
    else {
        return ExitCode::FAILURE;
    };

    let mut jar_file = JarFile::new(src);

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
