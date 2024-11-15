# JVM/tools/sj

The binary crate for the JVM implementation.

This crate houses no actual logic for the JVM, its purpose is to spin up the main thread with the user's
arguments. The actual runtime implementation is housed in [the runtime crate](../../runtime).

### Running the JVM

**If lost, check `--help` for more indepth explanations**

```console
$ sj <ClassName>
```

#### Example

```console
# Set the main class to "Main.class"
# NOTE: The `class` extension MUST be omitted!
$ sj Main
```