# smik-jar-tool

`smik-jar-tool` is a command-line application for reading and updating the
`softwareVersion` property in supported Spring Boot property files inside a
JAR.

## Build

From the workspace root:

```console
cargo build --release --bin smik-jar-tool
```

## Read versions

Supply only the archive path:

```console
smik-jar-tool application.jar
```

The command prints one `path: version` line for each discovered properties
file that contains `softwareVersion`. Missing versions and operational errors
are logged to standard error. The command fails if the archive itself cannot
be opened.

## Update a version

Supply the replacement version as the second positional argument:

```console
smik-jar-tool application.jar 2.4.0
```

The command updates every supported properties file found in the archive, then
overwrites the input path with the reconstructed JAR. A missing
`softwareVersion` property is added. The update is not a dry run, and no backup
is created automatically.

The supported files are `application.properties` and its `dev`, `int`,
`local`, and `prod` variants below `BOOT-INF/classes/`.

## Logging

The application uses `env_logger`. Set `RUST_LOG` to select a diagnostic level:

```console
RUST_LOG=debug smik-jar-tool application.jar
```

Use `smik-jar-tool --help` for the generated command reference.

The crate is an internal workspace package and is not configured for
publication.
