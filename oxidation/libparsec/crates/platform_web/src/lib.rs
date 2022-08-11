// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

mod transport;

pub use transport::*;

pub fn create_context() -> RuntimeContext {
    unimplemented!()
}

pub fn destroy_context(_ctx: RuntimeContext) {
    unimplemented!()
}

type JobFn = Box<dyn FnOnce() + Send>;

pub struct RuntimeContext {}

impl RuntimeContext {
    pub fn submit_job(&mut self, _func: JobFn) -> Result<(), &'static str> {
        unimplemented!()
    }
}
