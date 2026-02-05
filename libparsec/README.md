# Welcome to libparsec \o/

## Crates layout

- `libparsec` (i.e. code in `./src`): crate exposed to the outside world. The JS bindings (i.e. `../bindings`)
  access the content of this crate.
- `libparsec_*` (i.e. code in `./crates/`): internal crates, `libparsec` itself mostly
  re-expose them.

> Notes:
>
> - Currently the CLI relies on the internal `libparsec_*` crates instead of the
>   `libparsec`. This is for historical reasons and should eventually be fixed.
> - For simplicity, internal crates don't have their `libparsec_` suffix in the
>   folder hierarchy (e.g. `libparsec_types` is located at `./crates/types`).

## Internal crates

### Client crates

The client is the component that runs a device.
It is currently divided into two crates:

- `client_connection`: Handles connection to the server.
- `client`: Contains everything else: invitation & enrollment, certificate & manifest
  validation, workspace operations, monitors running background tasks, etc.

> Notes:
>
> The `client` crate can be seen as the "main" crate implementing most of the
> functional logic of the application.

### Account crate

As it name suggest, the `account` crate implements the client-side part of Parsec Account
(e.g. create an account, login, list available registration devices and invitations etc.).

### Base types crates

The base types can be divided into:

1. What is used to communicate between the client and the server.
   We call this the "protocol" and it is defined in `protocol/schema`.
2. What is used only by the client, but ends up serialized on disk.
   We call this simply the "data" and it is defined in `types/schema`.
3. What is used only internally (i.e. in-memory and never serialized).

The key point here is backward/forward compatibility as serialized data are
produced and consumed by different version of Parsec.

- `protocol`: Implements the protocol, and the API version.
- `types`: Implements the data, and the types that are only for internal use.
- `serialization_format`: Defines the format used for serialization (for both
  protocol and data).

> Notes:
>
> - `serialization_format` also provides a macro to generate Python bindings
>   for the protocol (used by the server). Note this code is beyond mere mortal
>   comprehension, debugging requires sacrificing your soul to Satan.

### Platform crates

Crates with platform in their name have different implementations depending on
the platform they are compiled for.

For instance `libparsec_platform_mountpoint` uses FUSE on UNIX, WinFSP on Windows,
and is not compatible with web.

The corollary is that crates without `platform` in their name are platform-independent,
and hence can be tested on a single platform.

The platform crates:

- `platform_async`: Asynchronous programming, works differently between web and native
  (the former has to integrate with the javascript runtime, the latter uses Tokio), so
  this crate exposes platform-agnostic utilities (e.g. spawn a future, async lock etc.).
- `platform_device_loader`: Manages the device encryption and where they are stored.
- `platform_filesystem` : Manages (device) data on disk, as files.
- `platform_http_proxy`: Handles HTTP proxy.
- `platform_ipc`: Allows communication between processes to ensure a device is not
  concurrently used.
- `platform_mountpoint`: FUSE/WinFSP to expose the workspaces as a OS drive.
- `platform_storage`: Database to store on disk manifest/block/certificates etc.

### Test helpers crates

Here, "testing" refers to two use-cases:

1. Running tests on a crate (i.e. `cargo test -p my_crate`)
2. Running the application with test feature (e.g. we want to run the GUI with an
   dummy organization already configured for quick testing purpose)

- `testbed`: Defines common organization templates to prevent code duplication in tests.
- `tests_macros`: Defines the `#[parsec_test]` macro that is used to decorate each
  test function. Among other things, this macro conveniently handles setting up
  Tokio context (so that our test can be an asynchronous function) and the testbed
  (configuring a test organization, and starting a testbed server if needed).
- `tests_lite`: Re-expose utilities useful to write tests (e.g. `pretty_assertions`,
  `rstest`, the `parsec_test` macro etc.). This is a lighter version of `tests_fixture`
  that is needed to tests `types` crate (since `tests_fixture` internally uses `types`
  crate).
- `tests_fixtures`: This crate re-expose the content of `test_lite` and provides additional
  helpers such as temporary folder fixture, injection hook, testbed types etc.

> Notes:
>
> - From an external point of view, the `tests_fixture` re-expose everything needed.
>   So when writing tests, `use libparsec_tests_fixtures::prelude::*` is the single-no-brainer-one-liner™.
> - `tests_fixtures`'s name is mostly an historical artifact, as it contains far
>   more than only fixtures !

### Special crates

- `crypto` Implements all the crypto algorithms (signing, encryption, hashing etc.). There
  are two implementations for those: Rustcrypto (pure Rust) and Sodiumoxide (based on libsodium).
- `zstd`: That's a funny one, you should have a look at [its README](./crates/zstd/README.md) ;-)

## FAQ

### Why most tests are in `tests/unit` ?

#### TL;DR

Stick to the following rule:

- All tests are unit tests.
- All test code must be in `tests/unit` (this works by using `#[path = "../../tests/unit/my_test.rs"]`
  to overwrite the test module location).
- Try to mimic the first-level hierarchy of `src` in the `tests/unit` (e.g. `src/foo.rs` -> `tests/unit/foo.rs`).

#### Long story

In Rust there is concept of unit vs integration testing:

- Unit test are within the tested crate.
- Integration are code outside the tested crate.

> "But I don't care about your “crate” concept! Defining a test as unit or integration
> should depend of what it does, not where it comes from !"

Well tough luck! That's how Rust works, so you'd better play along ¯\\_(ツ)_/¯

More seriously, this behavior comes from the fact the crate is the unit of compilation
in Rust. Hence a "unit" test is simply code added to the crate, which allows it to
access private components.

In practice this cause the following issues:

- It's complicated to know where to put new tests. For instance two related tests can
  end up in two separate places because one need access to internal and not the other
  (with additional fun if you want them to share helpers !).
- An integration test might need to be moved to unit when adding to it an additional
  check (if the check access internals).
- It's very common to access internals when testing (e.g. define a dump method for test
  only that returns an internal type).
- Unit tests code is located in the `src` directory and might even be inlined with
  the code to test. This add a lot of noise when reading the code :(

So to solve this we stick to the "all test code is unit test located in `tests/unit` folder".

The drawback to this approach is that it [increases compilation time](https://xkcd.com/303/)
(because the crate is the unit of compilation in Rust, remember? ^^) since 1) all
the test code is part of the crate and 2) modifying a test recompile this big crate
instead of a very small crate depending on a bigger crate that haven't changed.
