// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
};

/// Spawns a new asynchronous task, returning a [Task] for it.
///
/// ```
/// use libparsec_platform_async::{spawn, sleep};
/// use std::time::Duration;
///
/// # #[tokio::main]
/// # async fn main() {
/// let task = spawn(async {
///     libparsec_platform_async::sleep(Duration::from_millis(10)).await;
///     42
/// });
/// let result = task.await;
/// assert_eq!(result, 42);
/// # }
/// ```
pub fn spawn<F>(future: F) -> Task<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let shared_state = SharedState::default();
    let shared_state = Arc::new(Mutex::new(shared_state));

    let task = Task::new(shared_state.clone());
    let runnable = Runnable::new(future, shared_state);

    tokio::spawn(runnable);

    task
}

/// It's the state shared between [Task] and [Runnable].
/// It's configured inside of [spawn]'s function

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

/// Task is a *task controller*, meaning that it allow to control the spawned future.
/// You can wait for it to finish or abort it.
///
/// A `Task` *detaches* the associated future when it is dropped, meaning that the future will now run in background.
///
/// This `struct` is created by the [spawn] functions;
pub struct Task<T> {
    shared_state: Arc<Mutex<SharedState<T>>>,
}

impl<T> Task<T> {
    fn new(shared_state: Arc<Mutex<SharedState<T>>>) -> Self {
        Self { shared_state }
    }

    /// Abort the task. This will stop the spawned [Task] to make any new progress.
    /// It may return the value returned by the future if it as finished before aborting it.
    ///
    /// ```
    /// use libparsec_platform_async::{spawn, Notify};
    /// use std::{
    ///     time::Duration,
    ///     sync::{Arc, atomic::{AtomicBool, Ordering}}
    /// };
    /// # use tokio::time::sleep;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let notify = Arc::new(Notify::default());
    /// let notify2 = notify.clone();
    /// let finished = Arc::new(AtomicBool::new(false));
    /// let finished2 = finished.clone();
    ///
    /// let task = spawn(async move {
    ///     notify2.notified().await;
    ///     finished2.store(true, Ordering::SeqCst);
    /// });
    ///
    /// task.abort();
    /// notify.notify_one();
    /// sleep(Duration::from_millis(10)).await;
    /// assert_eq!(finished.load(Ordering::SeqCst), false, "task shouldn't have finished");
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// Awaiting a canceled task result in a panic
    ///
    /// ```should_panic
    /// use libparsec_platform_async::spawn;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let task = spawn(futures::future::ready(42));
    ///
    /// task.abort();
    /// task.await; // should panic here!
    /// # }
    /// ```
    pub fn abort(&self) -> Option<T> {
        let mut state = self.shared_state.lock().expect("mutex is poisoned");

        state.canceled = true;
        if let Some(ref waker) = state.runnable_waker {
            waker.wake_by_ref();
        }
        state.value.take()
    }

    /// Detaches the task to let it keep running in the background
    ///
    /// ```
    /// use libparsec_platform_async::{spawn, Notify};
    /// use std::{
    ///     time::Duration,
    ///     sync::{Arc, atomic::{AtomicBool, Ordering}},
    /// };
    /// # use tokio::time::sleep;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let notify = Arc::new(Notify::default());
    /// let notify2 = notify.clone();
    /// let finished = Arc::new(AtomicBool::new(false));
    /// let finished2 = finished.clone();
    ///
    /// let task = spawn(async move {
    ///     notify2.notified().await;
    ///     finished2.store(true, Ordering::SeqCst);
    /// });
    ///
    /// task.detach();
    /// notify.notify_one();
    /// sleep(Duration::from_millis(10)).await;
    /// assert_eq!(finished.load(Ordering::SeqCst), true, "task should have finished in background");
    /// # }
    pub fn detach(self) {
        let mut state = self.shared_state.lock().expect("mutex is poisoned");

        state.detached = true;
        state.task_waker = None;
    }

    /// Return `true` if the current task is finished
    ///
    /// ```
    /// use libparsec_platform_async::spawn;
    /// use std::time::Duration;
    /// # use tokio::time::sleep;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let task = spawn(futures::future::ready(42));
    ///
    /// sleep(Duration::from_millis(10)).await;
    /// assert!(task.is_finished(), "task should have finished");
    /// # }
    /// ```
    pub fn is_finished(&self) -> bool {
        self.shared_state
            .lock()
            .expect("mutex is poisoned")
            .finished
    }

    /// Return `true` if the current task is canceled
    ///
    /// ```
    /// use libparsec_platform_async::spawn;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let task = spawn(futures::future::ready(42));
    ///
    /// task.abort();
    /// assert!(task.is_canceled(), "task should have been canceled");
    /// # }
    /// ```
    pub fn is_canceled(&self) -> bool {
        self.shared_state
            .lock()
            .expect("mutex is poisoned")
            .canceled
    }
}

impl<T> Future for Task<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let s = self.as_mut();
        let mut state = s.shared_state.lock().expect("mutex is poisoned");

        if state.canceled {
            panic!("awaiting a canceled task");
        }
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

/// Background Task that will Poll the future given to [spawn].
///
/// Listen for message from [Task] passed in the [SharedState]
struct Runnable<F, T> {
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
        let mut state = s.shared_state.lock().expect("mutex is poisoned");

        state.runnable_waker = Some(cx.waker().clone());
        if state.canceled {
            state.runnable_waker = None;
            Poll::Ready(())
        } else {
            drop(state);
            match s.future.as_mut().poll(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(value) => {
                    let mut state = s.shared_state.lock().expect("mutex is poisoned");

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
