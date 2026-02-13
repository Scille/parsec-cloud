// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{AnonymousCmds, AuthenticatedCmds, ProxyConfig};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

pub(crate) fn alice_cmds_factory(env: &TestbedEnv) -> AuthenticatedCmds {
    let device = env.local_device("alice@dev1");
    AuthenticatedCmds::new(&env.discriminant_dir, device, ProxyConfig::default()).unwrap()
}

pub(crate) fn anonymous_cmds_factory(env: &TestbedEnv) -> AnonymousCmds {
    let addr =
        ParsecPkiEnrollmentAddr::new(env.server_addr.clone(), env.organization_id.to_owned());
    AnonymousCmds::new(&env.discriminant_dir, addr.into(), ProxyConfig::default()).unwrap()
}
