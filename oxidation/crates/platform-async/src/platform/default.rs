use std::{
    future::Future,
    sync::{Arc, Mutex},
};

pub fn spawn<F>(future: F) -> Task<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let shared_state = SharedState::default();
    let shared_state = Arc::new(Mutex::new(shared_state));

    let task = Task::new(shared_state.clone());
    let runnable = Runnable::new(future, shared_state);

    let _handle = tokio::spawn(runnable);

    task
}

struct SharedState<T> {
    value: Option<T>,
}

impl<T> Default for SharedState<T> {
    fn default() -> Self {
        Self { value: None }
    }
}

pub struct Task<T> {
    shared_state: Arc<Mutex<SharedState<T>>>,
}

impl<T> Task<T> {
    fn new(shared_state: Arc<Mutex<SharedState<T>>>) -> Self {
        Self { shared_state }
    }
}

pub struct Runnable<F, T> {
    future: F,
    shared_state: Arc<Mutex<SharedState<T>>>,
}

impl<F, T> Runnable<F, T> {
    fn new(future: F, shared_state: Arc<Mutex<SharedState<T>>>) -> Self {
        Self {
            future,
            shared_state,
        }
    }
}
