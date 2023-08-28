# Libparsec Serialization Format

This crate provides function-like [`procedural macros`] for generating
serialization code for libparsec protocol and data types, based on JSON schemas.

The JSON schema formats for libparsec protocol and data types are defined in
the [`json_schema`] directory. The concrete schemas can be found in the
[`/libparsec/types/schema`] and [`/libparsec/protocol/schema`] directories.

## Examples

Generate protocol commands code for all the json schemas in the
`schema/invited_cmds` directory:

```rust
parsec_protocol_cmds_family!("schema/invited_cmds");
```

Generate serialization code for the user certificate data type:

```rust
parsec_data!("schema/certif/user_certificate.json5");
```

## Internals

[`Procedural macros`] allow generating code by executing a function receiving a
stream of tokens as input and producing a stream of tokens as output to compile
into the caller’s crate.
The [`syn`] and [`quote`] crates are used respectively for parsing the input
token stream and generating the output token stream. Please refer to the crates
docs to learn more about it.

[`json_schema`]: https://github.com/Scille/parsec-cloud/tree/master/json_schema
[`/libparsec/types/schema`]: https://github.com/Scille/parsec-cloud/tree/master/libparsec/types/schema
[`/libparsec/protocol/schema`]: https://github.com/Scille/parsec-cloud/tree/master/libparsec/protocol/schema
[`procedural macros`]: https://doc.rust-lang.org/reference/procedural-macros.html#function-like-procedural-macros
[`syn`]: https://docs.rs/syn/latest/syn/
[`quote`]: https://docs.rs/crate/quote/latest
