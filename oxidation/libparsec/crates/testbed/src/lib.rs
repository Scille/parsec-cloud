// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use lazy_static::lazy_static;
use std::sync::Arc;

mod coolorg;
mod empty;
mod env;
mod types;

pub use env::*;
pub use types::*;

// Templates are generated only once, then copied for fast initialization of testbed envs
lazy_static! {
    static ref TESTBED_TEMPLATES: Vec<Arc<TestbedTemplate>> =
        vec![Arc::new(empty::generate()), Arc::new(coolorg::generate()),];
}

/// Only used to expose templates to the test server through Python bindings
pub fn test_get_testbed_templates() -> Vec<Arc<TestbedTemplate>> {
    TESTBED_TEMPLATES.clone()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_stability() {
        let a1 = coolorg::generate();
        let a2 = coolorg::generate();
        let a3 = empty::generate();

        assert_eq!(a1.crc, a2.crc);
        assert_ne!(a1.crc, a3.crc);
    }
}
