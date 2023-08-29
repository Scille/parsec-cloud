# Libparsec Protocol

This crate provides the schema definition, implementation and serialization
tests for Parsec protocol commands.

The JSON schemas defining the protocol commands can be found in the [`schema`]
directory. The commands implementation and tests are generated with procedural
macros from the [`libparsec serialization format`] crate based on these schemas.

[`schema`]: ./schema
[`libparsec serialization format`]: ../serialization_format
