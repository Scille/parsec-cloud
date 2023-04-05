// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

// Functions using rstest parametrize ignores `#[warn(clippy::too_many_arguments)]`
// decorator, so we must do global ignore instead :(
#![allow(clippy::too_many_arguments)]
#[allow(clippy::single_component_path_imports)]
/// Needed to expose `rstest_reuse::template` proc macro
use rstest_reuse;

use libparsec_types::*;

#[path = "../src/fixtures.rs"]
mod fixtures;

mod test_addr;
mod test_certif;
mod test_id;
mod test_invite;
mod test_local_device;
mod test_local_device_file;
mod test_local_manifest;
mod test_manifest;
mod test_message;
mod test_pki;
mod test_sas_code;
mod test_time;
