// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use lazy_static::lazy_static;
use std::sync::Arc;

mod coolorg;
mod empty;
mod env;
mod minimal;
mod scope;
mod types;

pub use env::*;
pub use scope::{Run, TestbedScope};
pub use types::*;

// Templates are generated only once, then copied for fast initialization of testbed envs
lazy_static! {
    static ref TESTBED_TEMPLATES: Vec<Arc<TestbedTemplate>> = vec![
        Arc::new(empty::generate()),
        Arc::new(minimal::generate()),
        Arc::new(coolorg::generate())
    ];
}

/// Only used to expose templates to the test server through Python bindings
pub fn test_get_testbed_templates() -> Vec<Arc<TestbedTemplate>> {
    TESTBED_TEMPLATES.clone()
}

#[cfg(test)]
#[path = "../tests/unit/testbed.rs"]
mod tests;
