// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

// This lib provide helpers that are used for testing purpose.
// To simplify the writing of those helpers, we use the same rule for when writing tests.
#![allow(clippy::unwrap_used)]

use std::sync::{Arc, Mutex};

mod env;
mod scope;
mod template;
mod templates;

pub use env::*;
pub use scope::{Run, TestbedScope};
pub use template::*;

enum TemplateState {
    NotGenerated(fn() -> Arc<TestbedTemplate>),
    Generated(Arc<TestbedTemplate>),
}

// Templates are generated only once, then copied for fast initialization of testbed envs
// On top of that we generated them lazily to further improve speed of single-test runs
// given cargo-nextest relies on that even when running multiple tests.
static TESTBED_TEMPLATES: [(&str, Mutex<TemplateState>); 4] = [
    (
        "empty",
        Mutex::new(TemplateState::NotGenerated(templates::empty::generate)),
    ),
    (
        "minimal",
        Mutex::new(TemplateState::NotGenerated(templates::minimal::generate)),
    ),
    (
        "minimal_client_ready",
        Mutex::new(TemplateState::NotGenerated(
            templates::minimal_client_ready::generate,
        )),
    ),
    (
        "coolorg",
        Mutex::new(TemplateState::NotGenerated(templates::coolorg::generate)),
    ),
];

pub fn test_get_template(id: &str) -> Option<Arc<TestbedTemplate>> {
    for (candidate, state) in TESTBED_TEMPLATES.as_ref() {
        if *candidate == id {
            let mut guard = state.lock().expect("Mutex is poisoned");
            let template = match &*guard {
                TemplateState::NotGenerated(generate) => {
                    let template = generate();
                    assert_eq!(
                        template.id, id,
                        "Mismatch in testbed template ID `{}` vs `{}`",
                        template.id, id
                    );
                    *guard = TemplateState::Generated(template.clone());
                    template
                }
                TemplateState::Generated(template) => template.to_owned(),
            };
            return Some(template);
        }
    }
    None
}

#[cfg(test)]
#[path = "../tests/unit/testbed.rs"]
mod tests;
