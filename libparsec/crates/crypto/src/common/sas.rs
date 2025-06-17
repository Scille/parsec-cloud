// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub const SAS_CODE_NONCE_SIZE: usize = 64;

pub fn generate_sas_code_nonce() -> Vec<u8> {
    let mut nonce = vec![0; SAS_CODE_NONCE_SIZE];
    crate::generate_rand(&mut nonce);
    nonce
}
