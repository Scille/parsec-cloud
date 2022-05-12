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
}

impl<T> crate::task::Task<T> for Task<T> {
    fn cancel(&self) -> Option<T> {
        let mut state = self.shared_state.lock().unwrap();

        state.canceled = true;
        if let Some(ref waker) = state.runnable_waker {
            waker.wake_by_ref();
        }
        state.value.take()
    }

    fn detach(self) {
        self.shared_state.lock().unwrap().detached = true;
    }

    fn is_finised(&self) -> bool {
        self.shared_state.lock().unwrap().finished
    }

    fn is_canceled(&self) -> bool {
        self.shared_state.lock().unwrap().canceled
    }
}

impl<T> Future for Task<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let s = self.as_mut();
        let mut state = s.shared_state.lock().unwrap();

        state.task_waker = Some(cx.waker().clone());
        if state.finished {
            Poll::Ready(
                state
                    .value
                    .take()
                    .expect("return value from runnable sohould have been set, but wasn't"),
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
