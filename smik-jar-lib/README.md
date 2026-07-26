# smik-jar-lib

`smik-jar-lib` is the reusable library behind `smik-jar-tool`. It reads and updates the `softwareVersion` value in
selected Spring Boot properties files stored below `BOOT-INF/classes/` in a JAR archive.

## Reading versions

Wrap any readable, seekable JAR source in `JarFile`:

```rust,no_run
use std::fs::File;

use smik_jar_lib::JarFile;

let source = File::open("application.jar")?;
let mut jar = JarFile::new(source);

for (path, version) in jar.versions()? {
    println!("{}: {:?}", path.display(), version);
}

# Ok::<(), Box<dyn std::error::Error>>(())
```

The result is ordered by path. A value of `None` means that a discovered properties file did not contain
`softwareVersion`. Missing or malformed files are logged and omitted.

## Updating a version

Updating requires readable, writable, seekable storage. `set_version` builds and returns a complete replacement archive;
it does not overwrite the original storage:

```rust,no_run
use std::fs::{File, OpenOptions};
use std::io::Write;

use smik_jar_lib::JarFile;

let source = File::open("application.jar")?;
let mut jar = JarFile::new(source);
let replacement = jar.set_version(&"2.4.0")?;

let mut destination = OpenOptions::new()
    .write(true)
    .truncate(true)
    .open("application.jar")?;
destination.write_all(&replacement)?;

# Ok::<(), Box<dyn std::error::Error>>(())
```

Every supported properties file found in the archive receives the new value. Other regular files and directories are
copied into the replacement archive.

## Supported paths

The recognized filenames are:

- `application.properties`
- `application-dev.properties`
- `application-int.properties`
- `application-local.properties`
- `application-prod.properties`

Each filename is resolved relative to `BOOT-INF/classes/`.

## Errors and diagnostics

`JarFile::versions` returns ZIP errors that prevent the archive from being opened. `JarFile::set_version` returns
`JarError`, which represents I/O, ZIP, and Java properties failures. Non-fatal discovery and parsing diagnostics use the
`log` facade, so applications can choose their preferred logger.

The crate is an internal workspace package and is not configured for publication.
