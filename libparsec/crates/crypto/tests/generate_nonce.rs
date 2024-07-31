// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use pretty_assertions::assert_eq;

use libparsec_crypto::generate_nonce;

#[test]
fn length() {
    let nonce = generate_nonce();
    assert_eq!(nonce.len(), 64);
}

#[test]
fn stability() {
    let nonce1 = generate_nonce();
    let nonce2 = generate_nonce();

    assert_ne!(nonce1, nonce2);
}
