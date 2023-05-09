// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::sync::Arc;

use super::{coolorg, empty, get_template};

#[test]
pub fn generate_stability() {
    let a1 = coolorg::generate();
    let a2 = coolorg::generate();
    let a3 = empty::generate();

    assert_eq!(a1.crc, a2.crc);
    assert_ne!(a1.crc, a3.crc);
}

#[test]
pub fn get_template_stability() {
    let a1 = get_template("minimal");
    let a2 = get_template("minimal");
    assert!(Arc::ptr_eq(&a1, &a2));
}
