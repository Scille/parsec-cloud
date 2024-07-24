# Why do we need our own Zstd crate here ?

Zstd is a very efficient compression library, and it C-based implementation works
well in Rust thanks to the `zstd` crate.

However compiling the `zstd` crate for WASM can be cumbersome on Windows/MacOS,
as it requires to install a specific version of llvm with WebAssembly support
(see https://github.com/gyscos/zstd-rs/issues/93).

Hence this `libparsec_zstd` crates that act as a façade over two implementations:

- The actual `zstd` crate. This is the implementation used by default.
- A pure-Rust alternative. It can be enabled by setting the `use_pure_rust_but_dirty_zstd`
  cfg option.

Note the pure-Rust implementation is based on the `ruzstd` crate that provide
a decompressor, and a custom compressor (see `src/dirty.rs`) that relies on
the fact uncompressed data can be considered valid zstd by simply adding some
headers.
Hence a dead simple but very inefficient zstd implementation that should never
be used for anything else than development purposes !

## Example

Compiling with Zstd support provided by the `zstd` crate:

```shell
cargo build -p libparsec_whatever
```

Compiling with Zstd support provided by the pure-Rust dirty implementation (i.e.
only do that for dev).

```shell
RUSTFLAGS='--cfg use_pure_rust_but_dirty_zstd cargo build -p libparsec_whatever
```

Notes:

- There is nothing to do to have the "good" Zstd implementation ;-)
- You should never need to use this `use_pure_rust_but_dirty_zstd` by hand:
  instead it is used by the `make.py` script which is already responsible to
  do libparsec compilation with the correct flags.

## Why not using features instead of cfg ?

Features are supposed to be additive, while we want to switch between two
implementations.

Features must be explicitly enabled, however here we have one case that should
be used for the vast majority of the cases and a weird corner case that should
under no circumstances end up being used in production !

So using a feature would require more configuration in multiple places, and may
lead to the wrong implementation being used by mistake while being hard to
detect (this has already happened when generating test data !).

Finally features cannot be enabled for transitive dependencies, this is an issue
when using `wasm-pack test` on `libparsec_platform_storage` (given a feature must
be passed to `libparsec_zstd`, but only `libparsec_types` depends on it so it
cannot be done from `libparsec_platform_storage`).
