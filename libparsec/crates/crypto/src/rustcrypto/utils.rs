// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use rand::{rngs::OsRng, RngCore};

pub(crate) fn generate_rand(out: &mut [u8]) {
    OsRng.fill_bytes(out);
}
