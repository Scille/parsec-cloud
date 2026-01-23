// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod identity;
mod opaque_key;
mod sign;

use libparsec_types::prelude::*;

// Matches what is done in `server/parsec/cli/testbed.py`
fn mocked_openbao_new_token_entity_id_and_mail() -> (String, String, EmailAddress) {
    let token = format!("s.{}", libparsec_types::uuid::Uuid::new_v4().simple());

    let entity_id = {
        use sha2::{Digest, Sha256};
        let hash = Sha256::digest(token.as_bytes());
        let b: [u8; 16] = hash[..16].try_into().unwrap();
        libparsec_types::uuid::Uuid::from_bytes(b).to_string()
    };

    let email: EmailAddress = format!("{entity_id}@example.invalid").parse().unwrap();

    (token, entity_id, email)
}
