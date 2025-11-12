// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Generate serialization test layout for all protocol commands
// Concrete test code can be found in tests/<cmd_family>/<api_version>/<cmd>.rs
use libparsec_serialization_format::protocol_cmds_tests;

protocol_cmds_tests!("schema/anonymous_cmds");
protocol_cmds_tests!("schema/invited_cmds");
protocol_cmds_tests!("schema/authenticated_cmds");
protocol_cmds_tests!("schema/tos_cmds");
protocol_cmds_tests!("schema/anonymous_server_cmds");
protocol_cmds_tests!("schema/authenticated_account_cmds");
