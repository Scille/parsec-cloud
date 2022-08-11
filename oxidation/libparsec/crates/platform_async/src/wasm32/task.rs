// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
};

use wasm_bindgen_futures::spawn_local;

/// Spawns a new asynchronous task, returning a [Task] for it.
///
/// ```
/// use libparsec_platform_async::spawn;
/// use futures::ready;
///
/// # futures_lite::future::block_on(async {
/// let future = ready(42);
/// let task = spawn(future);
/// let result = task.await;
/// assert_eq!(result, 42);
/// # });
/// ```
pub fn spawn<F>(future: F) -> Task<F::Output>
where
    F: Future + 'static,
    F::Output: 'static,
{
    let shared_state = SharedState::default();
    let shared_state = Arc::new(Mutex::new(shared_state));

    let task = Task::new(shared_state.clone());
    let runnable = Runnable::new(future, shared_state);

    spawn_local(runnable);

    task
}

struct SharedState<T> {
    canceled: bool,
    finished: bool,
    detached: bool,
    value: Option<T>,
    task_waker: Option<Waker>,
    runnable_waker: Option<Waker>,
}

impl<T> Default for SharedState<T> {
    fn default() -> Self {
        Self {
            canceled: false,
            finished: false,
            detached: false,
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

    /// Cancels the task
    pub fn abort(&self) -> Option<T> {
        let mut state = self.shared_state.lock().expect("mutex poisoned");

        state.canceled = true;
        if let Some(ref waker) = state.runnable_waker {
            waker.wake_by_ref()
        }
        state.value.take()
    }

    /// Detaches the task to let it keep running in the background
    pub fn detach(self) {
        let mut state = self.shared_state.lock().expect("mutex poisoned");

        state.detached = true;
        state.task_waker = None;
    }

    /// Return `true` if the current task is finished
    pub fn is_finished(&self) -> bool {
        self.shared_state.lock().expect("mutex poisoned").finished
    }

    /// Return `true` if the current task is canceled
    pub fn is_canceled(&self) -> bool {
        self.shared_state.lock().expect("mutex poisoned").canceled
    }
}

impl<T> Future for Task<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let s = self.as_mut();
        let mut state = s.shared_state.lock().expect("mutex poisoned");

        state.task_waker = Some(cx.waker().clone());
        if state.finished {
            Poll::Ready(
                state
                    .value
                    .take()
                    .expect("return value from runnable should have been set, but wasn't"),
            )
        } else {
            Poll::Pending
        }
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

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let mut s = self.as_mut();
        let mut state = s.shared_state.lock().expect("Mutex poisoned");

        state.runnable_waker = Some(cx.waker().clone());
        if state.canceled {
            state.runnable_waker = None;
            Poll::Ready(())
        } else {
            drop(state);
            match s.future.as_mut().poll(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(value) => {
                    let mut state = s.shared_state.lock().expect("Mutex poisoned");

                    state.value = Some(value);
                    state.finished = true;
                    state.runnable_waker = None;
                    if let Some(ref waker) = state.task_waker {
                        waker.wake_by_ref();
                    }
                    Poll::Ready(())
                }
            }
        }
    }
}
