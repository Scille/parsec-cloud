// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::sync::{Arc, Mutex};

mod coolorg;
mod empty;
mod env;
mod minimal;
mod scope;
mod types;

pub use env::*;
pub use scope::{Run, TestbedScope};
pub use types::*;

enum TemplateState {
    NotGenerated(fn() -> TestbedTemplate),
    Generated(Arc<TestbedTemplate>),
}

// Templates are generated only once, then copied for fast initialization of testbed envs
// On top of that we generated them lazily to further improve speed of single-test runs
// given cargo-nextest relies on that even when running multiple tests.
static TESTBED_TEMPLATES: [(&str, Mutex<TemplateState>); 3] = [
    (
        "empty",
        Mutex::new(TemplateState::NotGenerated(empty::generate)),
    ),
    (
        "minimal",
        Mutex::new(TemplateState::NotGenerated(minimal::generate)),
    ),
    (
        "coolorg",
        Mutex::new(TemplateState::NotGenerated(coolorg::generate)),
    ),
];

fn get_template(template: &str) -> Arc<TestbedTemplate> {
    for (id, state) in TESTBED_TEMPLATES.as_ref() {
        if *id == template {
            let mut guard = state.lock().expect("Mutex is poisoned");
            match &*guard {
                TemplateState::NotGenerated(generate) => {
                    let testbed = Arc::new(generate());
                    assert_eq!(
                        testbed.id, *id,
                        "Mismatch in testbed template ID `{}` vs `{}`",
                        testbed.id, *id
                    );
                    *guard = TemplateState::Generated(testbed.clone());
                    return testbed;
                }
                TemplateState::Generated(testbed) => return testbed.clone(),
            }
        }
    }
    panic!("No testbed template with ID `{}`", template);
}

/// Only used to expose templates to the test server through Python bindings
pub fn test_get_testbed_templates() -> Vec<Arc<TestbedTemplate>> {
    TESTBED_TEMPLATES
        .iter()
        .map(|(id, _)| get_template(id))
        .collect()
}

#[cfg(test)]
#[path = "../tests/unit/testbed.rs"]
mod tests;
