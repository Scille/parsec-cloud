// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::sync::Arc;

use hex_literal::hex;

use crate::TestbedTemplate;

/// Minimal organization:
/// - Only a single alice user & device
/// - No data in local storage
pub(crate) fn generate() -> Arc<TestbedTemplate> {
    TestbedTemplate::from_builder("minimal")
        .bootstrap_organization(
            hex!("b62e7d2a9ed95187975294a1afb1ba345a79e4beb873389366d6c836d20ec5bc"),
            None,
            "alice@dev1",
            Some("Alicey McAliceFace <alice@example.com>"),
            hex!("74e860967fd90d063ebd64fb1ba6824c4c010099dd37508b7f2875a5db2ef8c9"),
            Some("My dev1 machine"),
            hex!("d544f66ece9c85d5b80275db9124b5f04bb038081622bed139c1e789c5217400"),
            hex!("a4031e8bcdd84df8ae12bd3d05e6e20f"),
            hex!("26bf35a98c1e54e90215e154af92a1af2d1142cdd0dba25b990426b0b30b0f9a"),
            hex!("323614fc6bd2d300f42d6731059f542f89534cdca11b2cb13d5a9a5a6b19b6ac"),
            "P@ssw0rd.",
        )
        .finalize()
}
