// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// TODO: Remove me when tests are implemented.
//       Currently it is required to avoid warnings since ALL the test files
//       contain unused code such as `use super::authenticated_cmds;`.
//       Once implemented, we can safely remove this (or move it
//       to the tests files)
#![allow(unused)]

// Generate serialization test layout for all protocol commands
// Concrete test code can be found in tests/<cmd_family>/<api_version>/<cmd>.rs
use libparsec_serialization_format::protocol_cmds_tests;

// TODO: This is broken since APIv4 schema have changed a lot, should fix this asap !
protocol_cmds_tests!("schema/invited_cmds");
protocol_cmds_tests!("schema/anonymous_cmds");
protocol_cmds_tests!("schema/authenticated_cmds");
