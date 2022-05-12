use futures::FutureExt;
use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
};

/// Spawns a new asynchronous task, returning a [Task] for it.
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
    canceled: bool,
    finished: bool,
    value: Option<T>,
    task_waker: Option<Waker>,
    runnable_waker: Option<Waker>,
}

impl<T> Default for SharedState<T> {
    fn default() -> Self {
        Self {
            canceled: false,
            finished: false,
            value: None,
            task_waker: None,
            runnable_waker: None,
        }
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
    future: Pin<Box<F>>,
    shared_state: Arc<Mutex<SharedState<T>>>,
}

impl<F, T> Runnable<F, T> {
    fn new(future: F, shared_state: Arc<Mutex<SharedState<T>>>) -> Self {
        Self {
            future: Box::pin(future),
            shared_state,
        }
    }
}

impl<F, T> Future for Runnable<F, T>
where
    F: Future<Output = T>,
{
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut s = self.as_mut();
        let mut state = s.shared_state.lock().unwrap();

        state.runnable_waker = Some(cx.waker().clone());
        if state.canceled {
            Poll::Ready(())
        } else {
            drop(state);
            match s.future.poll_unpin(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(value) => {
                    let mut state = s.shared_state.lock().unwrap();

                    state.value = Some(value);
                    state.finished = true;
                    if let Some(ref waker) = state.task_waker {
                        waker.wake_by_ref();
                    }
                    Poll::Ready(())
                }
            }
        }
    }
}
