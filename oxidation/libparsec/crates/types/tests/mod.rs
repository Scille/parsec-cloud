// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

// Needed to expose `rstest_reuse::template` proc macro
#![allow(clippy::single_component_path_imports)]
use rstest_reuse;

mod test_addr;
mod test_certif;
mod test_invite;
mod test_manifest;
mod test_message;
mod test_sas_code;
