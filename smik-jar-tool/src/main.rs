//! CLI tool for managing SMIK JAR file versions.

use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use log::error;
use smik_jar_lib::JarFile;

#[derive(Debug, Parser)]
struct Args {
    #[clap(index = 1, help = "Path to the JAR file")]
    jar_file: PathBuf,
    #[clap(long, short, help = "The version to set in the JAR file")]
    version: Option<String>,
}

fn main() -> ExitCode {
    env_logger::init();
    let args = Args::parse();

    let Ok(src) = OpenOptions::new()
        .read(true)
        .open(&args.jar_file)
        .inspect_err(|error| error!("Error opening file: {error}"))
    else {
        return ExitCode::FAILURE;
    };

    let mut jar_file = JarFile::new(src);

    if let Some(version) = args.version {
        let Ok(new_file) = jar_file
            .set_version(&version)
            .inspect_err(|error| error!("Error setting version: {error}"))
        else {
            return ExitCode::FAILURE;
        };

        let Ok(mut dst) = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&args.jar_file)
            .inspect_err(|error| error!("Error opening file: {error}"))
        else {
            return ExitCode::FAILURE;
        };

        if let Err(error) = dst.write_all(&new_file) {
            error!("Error writing to file: {error}");
            return ExitCode::FAILURE;
        }
    } else {
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
    }

    ExitCode::SUCCESS
}
