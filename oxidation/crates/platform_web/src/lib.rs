pub fn create_context() -> RuntimeContext {
    unimplemented!()
}

pub fn destroy_context(_ctx: RuntimeContext) {
    unimplemented!()
}

type JobFn = Box<dyn FnOnce() + Send>;

pub struct RuntimeContext {
}

impl RuntimeContext {
    pub fn submit_job(&mut self, _func: JobFn) -> Result<(), &'static str> {
        unimplemented!()
    }
}
