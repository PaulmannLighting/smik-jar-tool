# Architecture

This document describes the internal structure of the `smik-jar-tool`
workspace. The root [`README.md`](README.md) contains user-facing build and
usage instructions.

## Workspace layout

```mermaid
flowchart LR
    User[User] --> Cli[smik-jar-tool]
    Cli --> Library[smik-jar-lib]
    Library --> Zip[zip]
    Library --> Properties[java-properties]
    Cli --> Logging[env_logger and log]
    Library --> Logging
```

The binary crate owns argument parsing, filesystem access, logging setup, and
exit codes. The library crate owns knowledge of the supported paths, Java
properties parsing, and ZIP archive reconstruction.

All dependency versions and features are declared in the root
`[workspace.dependencies]` table. Member manifests only select the workspace
dependencies they use.

## Library modules

The `smik-jar-lib` crate exposes `JarFile` and `JarError`. Its remaining
modules are implementation details:

| Module | Responsibility |
| --- | --- |
| `lib` | Defines supported property paths and the public crate surface. |
| `jar_file` | Coordinates version reads and archive updates. |
| `read_version` | Locates and parses supported Java properties files. |
| `update_jar` | Copies ZIP entries and rewrites changed properties files. |
| `by_path` | Resolves a ZIP entry from a filesystem-style path. |
| `error` | Combines I/O, ZIP, and Java properties failures. |

`JarFile<T>` is generic over its storage. Reading requires `T: Read + Seek`;
updating additionally requires `T: Write`. The CLI opens a regular file for
reading, while the updated archive is first written to an in-memory buffer.

## Read flow

```mermaid
sequenceDiagram
    participant CLI as smik-jar-tool
    participant JAR as JarFile
    participant ZIP as ZipArchive
    participant Props as java-properties

    CLI->>JAR: versions()
    JAR->>ZIP: open archive
    loop Each supported path
        ZIP->>Props: read matching entry
        Props-->>JAR: property map
    end
    JAR-->>CLI: path to optional version map
```

Missing or malformed supported property files are logged and omitted. Failure
to open the ZIP archive is returned to the caller. Versions are stored in a
`BTreeMap`, giving callers deterministic path ordering.

## Update flow

```mermaid
sequenceDiagram
    participant CLI as smik-jar-tool
    participant JAR as JarFile
    participant Reader as ZipArchive
    participant Writer as ZipWriter

    CLI->>JAR: set_version(version)
    JAR->>Reader: read supported properties
    JAR->>JAR: set softwareVersion
    JAR->>Writer: copy unchanged entries
    JAR->>Writer: write changed property files
    Writer-->>CLI: replacement bytes
    CLI->>CLI: overwrite source path
```

The writer preserves each replaced entry's ZIP options when available.
Unchanged regular files and directories are copied. Unsupported ZIP entry
types are skipped with a warning. The library returns the complete replacement
archive as bytes; it does not modify the original storage itself.

## Error and logging model

Structural archive errors from `versions` use `ZipError`. Updates use
`JarError`, which retains I/O, ZIP, and Java properties errors as sources.
Recoverable discovery problems, such as an absent supported properties file,
are reported through `log` and do not abort the scan.

The binary translates operation failures into `ExitCode::FAILURE` and emits
the underlying error. Successful reads print versions to standard output.

## Extension points

To support another properties filename, add it to `PROPERTIES_FILES` in
`smik-jar-lib/src/lib.rs`. To support a different archive layout or property
key, update the corresponding path constants or `SOFTWARE_VERSION`. These are
currently compile-time policy choices rather than runtime configuration.

When changing archive reconstruction, preserve the separation between
selection/parsing in `read_version` and ZIP output in `update_jar`. Add tests
covering both retained archive entries and rewritten properties metadata.
