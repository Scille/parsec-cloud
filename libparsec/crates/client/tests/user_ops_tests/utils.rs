// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client::{certificates_ops::CertificatesOps, user_ops::UserOps, EventBus};
use libparsec_client_connection::{AuthenticatedCmds, ProxyConfig};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

pub(crate) async fn user_ops_factory(env: &TestbedEnv, device: &Arc<LocalDevice>) -> UserOps {
    let event_bus = EventBus::default();
    let cmds = Arc::new(
        AuthenticatedCmds::new(
            &env.discriminant_dir,
            device.clone(),
            ProxyConfig::default(),
        )
        .unwrap(),
    );
    let certificates_ops = Arc::new(
        CertificatesOps::start(
            &env.discriminant_dir,
            device.clone(),
            event_bus.clone(),
            cmds.clone(),
        )
        .await
        .unwrap(),
    );
    UserOps::start(
        env.discriminant_dir.clone(),
        device.clone(),
        cmds,
        certificates_ops,
        event_bus,
    )
    .await
    .unwrap()
}
