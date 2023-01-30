// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use hex_literal::hex;

use libparsec_crypto::SigningKey;

use crate::TestbedTemplate;

pub(crate) fn generate() -> TestbedTemplate {
    TestbedTemplate::new(
        "empty",
        SigningKey::from(hex!(
            "62fdf2e15b7e7a43e8f1c4e38ec25f11f285691497f14f082d769fa2d0d2d1e6"
        )),
        vec![],
        vec![],
        vec![],
    )
}
