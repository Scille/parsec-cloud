// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use super::{coolorg, empty};

#[test]
pub fn test_stability() {
    let a1 = coolorg::generate();
    let a2 = coolorg::generate();
    let a3 = empty::generate();

    assert_eq!(a1.crc, a2.crc);
    assert_ne!(a1.crc, a3.crc);
}
