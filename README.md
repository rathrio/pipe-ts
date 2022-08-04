# | ts

Replaces anything that looks like a UNIX timestamp with a human readable
representation in the system's local time, e.g.:

```sh
echo "foo bar 1659605680 blabla" | ts
# => foo bar 2022-08-04 11:34:40 blabla
```

This comes in handy when inspecting the output of tools like
[kcat](https://github.com/edenhill/kcat) for instance.

## Installation

From source with the [Rust toochain installed](https://www.rust-lang.org/tools/install):

```sh
cargo install --path .
```

You should now be able to run `ts` from everywhere:

```sh
ts --help
```