// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use super::{templates, test_get_template};

#[test]
pub fn generate_stability() {
    let a1 = templates::minimal::generate();
    let a2 = templates::minimal::generate();
    let a3 = templates::empty::generate();

    assert_eq!(a1.compute_crc(), a2.compute_crc());
    assert_ne!(a1.compute_crc(), a3.compute_crc());
}

#[test]
pub fn get_template_stability() {
    let a1 = test_get_template("minimal").unwrap();
    let a2 = test_get_template("minimal").unwrap();
    assert!(Arc::ptr_eq(&a1, &a2));
}
