# smik JAR tool

`smik-jar-tool` reads and updates the `softwareVersion` property in a fixed set
of Spring Boot property files inside a JAR archive.

## Workspace

The workspace contains two crates:

- [`smik-jar-tool`](smik-jar-tool/README.md) provides the command-line
  application.
- [`smik-jar-lib`](smik-jar-lib/README.md) provides the reusable archive API.

Dependency versions, features, and workspace crate paths are defined once in
the root `Cargo.toml` under `[workspace.dependencies]`. Member crates inherit
their dependencies with `workspace = true`.

## Build

The workspace requires Rust with support for the 2024 edition.

```console
cargo build --release
```

The release binary is written to `target/release/smik-jar-tool`.

## Usage

Pass a JAR path without a version to display every version found:

```console
cargo run --bin smik-jar-tool -- application.jar
```

Pass a version as the second argument to update the JAR in place:

```console
cargo run --bin smik-jar-tool -- application.jar 2.4.0
```

The tool looks below `BOOT-INF/classes/` for these files:

- `application.properties`
- `application-dev.properties`
- `application-int.properties`
- `application-local.properties`
- `application-prod.properties`

Missing property files are skipped. During an update, every discovered file is
rewritten with the requested `softwareVersion`; a missing property is added.
Other archive entries are copied to the replacement JAR.

The update command overwrites the supplied archive after constructing its
replacement in memory. Keep a backup when modifying an artifact that cannot be
recreated. Rewriting a signed JAR invalidates signatures that cover changed
content.

Set `RUST_LOG` to control diagnostics:

```console
RUST_LOG=info cargo run --bin smik-jar-tool -- application.jar
```

## Development

Run the workspace checks from the repository root:

```console
cargo +nightly fmt --check
cargo clippy --all-features
cargo test --all-features
cargo build --all-features --release
```

See [`ARCHITECTURE.md`](ARCHITECTURE.md) for the internal design and extension
points.

## License

This workspace is licensed under the [MIT License](LICENSE).
