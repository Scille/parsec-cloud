// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_crypto::{PrivateKey, SecretKey, SigningKey};

#[test]
fn anti_leak_debug() {
    assert_eq!(format!("{:?}", SecretKey::generate()), "SecretKey(****)");
    assert_eq!(format!("{:?}", PrivateKey::generate()), "PrivateKey(****)");
    assert_eq!(format!("{:?}", SigningKey::generate()), "SigningKey(****)");
}
