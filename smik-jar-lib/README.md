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

Updating is available for `JarFile<File>`. `set_version` overwrites the owned
file and returns no archive buffer:

```rust,no_run
use std::fs::OpenOptions;

use smik_jar_lib::JarFile;

let storage = OpenOptions::new()
    .read(true)
    .write(true)
    .open("application.jar")?;
let mut jar = JarFile::new(storage);
jar.set_version("2.4.0")?;

# Ok::<(), Box<dyn std::error::Error>>(())
```

Every supported properties file found in the archive receives the new value. Other regular files and directories are
preserved. Because ZIP entries cannot be resized in place, the archive is
streamed into an anonymous file created by the `tempfile` crate first. After
reconstruction succeeds, the temporary file is streamed into the wrapped
file, which is truncated and rewound. The temporary file is removed
automatically when it is dropped, and the `JarFile` remains usable after the
update.

## Supported paths

The recognized filenames are:

- `application.properties`
- `application-dev.properties`
- `application-int.properties`
- `application-local.properties`
- `application-prod.properties`

Each filename is resolved relative to `BOOT-INF/classes/`.

## Errors and diagnostics

`JarFile::versions` returns ZIP errors that prevent the archive from being
opened. `JarFile::set_version` returns `JarError`, which represents storage
I/O, ZIP, and Java properties failures. Non-fatal discovery and parsing
diagnostics use the `log` facade, so applications can choose their preferred
logger.

The crate is an internal workspace package and is not configured for publication.
