# Libparsec Platform Mountpoint

This crate provides a cross platform interface for mountpoint to simplify the implementation.

## Examples

[`memfs`]

## Internals

### Windows

- [`build.rs`] allows delayload for [`winfsp`], it links the `dll` for the binary.
- [`winfsp_tests`] tests the file system

There is a special type `ParsecEntryInfo` which is used for `Icon Overlay Handler` to show if a file is sync or not.

[`build.rs`]: https://github.com/Scille/parsec-cloud/tree/master/libparsec/crates/platform_mountpoint/build.rs
[`winfsp`]: https://winfsp.dev/
[`winfsp_tests`]: https://github.com/winfsp/winfsp/tree/master/tst/winfsp-tests
[`memfs`]: https://github.com/Scille/parsec-cloud/tree/master/libparsec/crates/platform_mountpoint/memfs/mod.rs
