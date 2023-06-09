// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use rand_08::{rngs::OsRng, RngCore};

pub fn generate_nonce() -> Vec<u8> {
    let mut nonce = vec![0; 64];
    OsRng.fill_bytes(&mut nonce);

    nonce
}
