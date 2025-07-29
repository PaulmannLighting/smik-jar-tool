//! CLI tool for managing smik JAR file versions.

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};
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
    src: PathBuf,
    #[clap(subcommand)]
    action: Action,
}

#[derive(Debug, Subcommand)]
enum Action {
    #[clap(name = "read", about = "Read versions from the JAR file")]
    Read,
    #[clap(name = "write", about = "Write versions to the JAR file")]
    Write {
        #[clap(index = 1, help = "The version to set in the JAR file")]
        version: String,
        #[clap(index = 2, help = "The destination JAR file")]
        dst: PathBuf,
    },
}

fn main() -> ExitCode {
    env_logger::init();
    let args = Args::parse();
    let Ok(src) = OpenOptions::new()
        .read(true)
        .open(args.src)
        .inspect_err(|error| error!("Error opening file: {error}"))
    else {
        return ExitCode::FAILURE;
    };

    match args.action {
        Action::Read => read_versions(src),
        Action::Write { version, dst } => replace_version(src, &version, &dst),
    }
}

fn replace_version(src: File, version: &str, dst: &Path) -> ExitCode {
    let mut jar_file = JarFile::new(src);

    let Ok(new_file) = jar_file
        .set_version(&version)
        .inspect_err(|error| error!("Error setting version: {error}"))
    else {
        return ExitCode::FAILURE;
    };

    drop(jar_file);

    let Ok(mut dst) = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .open(dst)
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

fn read_versions(src: File) -> ExitCode {
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
