// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use pretty_assertions::assert_eq as p_assert_eq;

use libparsec_crypto::generate_sas_code_nonce;

#[test]
fn sas_code_length() {
    let nonce = generate_sas_code_nonce();
    p_assert_eq!(nonce.len(), 64);
}

#[test]
fn sas_code_stability() {
    let nonce1 = generate_sas_code_nonce();
    let nonce2 = generate_sas_code_nonce();

    assert_ne!(nonce1, nonce2);
}
