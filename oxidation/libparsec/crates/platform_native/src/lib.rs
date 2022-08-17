// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

mod transport;

pub use transport::*;

use std::thread::JoinHandle;

pub fn create_context() -> RuntimeContext {
    let (sender, receiver) = std::sync::mpsc::channel::<JobFn>();

    // Simple thread to execute jobs
    let thread_join_handle = std::thread::spawn(move || {
        while let Ok(func) = receiver.recv() {
            func();
        }
        // Peer has closed the channel
    });

    RuntimeContext {
        job_sender: sender,
        thread_join_handle,
    }
}

pub fn destroy_context(ctx: RuntimeContext) {
    drop(ctx.job_sender);
    ctx.thread_join_handle
        .join()
        .expect("Cannot job runtime context thread !");
}

type JobFn = Box<dyn FnOnce() + Send>;

pub struct RuntimeContext {
    job_sender: std::sync::mpsc::Sender<JobFn>,
    thread_join_handle: JoinHandle<()>,
}

impl RuntimeContext {
    pub fn submit_job(&mut self, func: JobFn) -> Result<(), &'static str> {
        self.job_sender.send(func).map_err(|_| "submit_failed")
    }
}
