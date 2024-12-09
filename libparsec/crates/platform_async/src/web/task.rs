// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use futures::FutureExt;
use pin_project::{pin_project, pinned_drop};
use std::{
    any::Any,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use crate::{future::Future, oneshot};

/*
 * JoinError
 *
 * Code adapted from Tokio (MIT License).
 */

pub struct JoinError {
    repr: Repr,
}

/// `SyncWrapper` can make `Send + !Sync` types `Sync` by
/// disallowing all immutable access to the value.
pub struct SyncWrapper<T> {
    value: T,
}

// safety: The SyncWrapper being send allows you to send the inner value across
// thread boundaries.
unsafe impl<T: Send> Send for SyncWrapper<T> {}

// safety: An immutable reference to a SyncWrapper is useless, so moving such an
// immutable reference across threads is safe.
unsafe impl<T> Sync for SyncWrapper<T> {}

impl<T> SyncWrapper<T> {
    pub(crate) fn new(value: T) -> Self {
        Self { value }
    }

    pub(crate) fn into_inner(self) -> T {
        self.value
    }
}

impl SyncWrapper<Box<dyn Any + Send>> {
    /// Attempt to downcast using `Any::downcast_ref()` to a type that is known to be `Sync`.
    pub(crate) fn downcast_ref_sync<T: Any + Sync>(&self) -> Option<&T> {
        // SAFETY: if the downcast fails, the inner value is not touched,
        // so no thread-safety violation can occur.
        self.value.downcast_ref()
    }
}

pub enum Repr {
    Cancelled,
    Panic(SyncWrapper<Box<dyn Any + Send + 'static>>),
}

impl JoinError {
    pub(crate) fn cancelled() -> JoinError {
        JoinError {
            repr: Repr::Cancelled,
        }
    }
    pub(crate) fn panic(err: Box<dyn Any + Send + 'static>) -> JoinError {
        JoinError {
            repr: Repr::Panic(SyncWrapper::new(err)),
        }
    }
    pub fn is_cancelled(&self) -> bool {
        matches!(self.repr, Repr::Cancelled)
    }
    pub fn is_panic(&self) -> bool {
        matches!(self.repr, Repr::Panic(_))
    }
    pub fn into_panic(self) -> Box<dyn std::any::Any> {
        self.try_into_panic()
            .expect("`JoinError` reason is not a panic.")
    }
    pub fn try_into_panic(self) -> Result<Box<dyn std::any::Any>, JoinError> {
        match self.repr {
            Repr::Panic(p) => Ok(p.into_inner()),
            _ => Err(self),
        }
    }
}

impl std::error::Error for JoinError {}

impl From<JoinError> for std::io::Error {
    fn from(src: JoinError) -> std::io::Error {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            match src.repr {
                Repr::Cancelled => "task was cancelled",
                Repr::Panic(_) => "task panicked",
            },
        )
    }
}

fn panic_payload_as_str(payload: &SyncWrapper<Box<dyn Any + Send>>) -> Option<&str> {
    // Panic payloads are almost always `String` (if invoked with formatting arguments)
    // or `&'static str` (if invoked with a string literal).
    //
    // Non-string panic payloads have niche use-cases,
    // so we don't really need to worry about those.
    if let Some(s) = payload.downcast_ref_sync::<String>() {
        return Some(s);
    }

    if let Some(s) = payload.downcast_ref_sync::<&'static str>() {
        return Some(s);
    }

    None
}

impl std::fmt::Display for JoinError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.repr {
            Repr::Cancelled => write!(fmt, "task was cancelled"),
            Repr::Panic(p) => match panic_payload_as_str(p) {
                Some(panic_str) => {
                    write!(fmt, "task panicked with message {:?}", panic_str)
                }
                None => {
                    write!(fmt, "task panicked")
                }
            },
        }
    }
}

impl std::fmt::Debug for JoinError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.repr {
            Repr::Cancelled => write!(fmt, "JoinError::Cancelled()"),
            Repr::Panic(p) => match panic_payload_as_str(p) {
                Some(panic_str) => {
                    write!(fmt, "JoinError::Panic({:?}, ...)", panic_str)
                }
                None => write!(fmt, "JoinError::Panic(...)"),
            },
        }
    }
}

/*
 * JoinHandle
 */

pub struct JoinHandle<R> {
    task_outcome_rx: oneshot::Receiver<Result<R, JoinError>>,
    task_flags: Arc<TaskSharedFlags>,
}

impl<T> std::fmt::Debug for JoinHandle<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // // Safety: The header pointer is valid.
        // let id_ptr = unsafe { Header::get_id_ptr(self.raw.header_ptr()) };
        // let id = unsafe { id_ptr.as_ref() };
        // fmt.debug_struct("JoinHandle").field("id", id).finish()
        // Safety: The header pointer is valid.
        fmt.debug_struct("JoinHandle").finish()
    }
}

impl<R> JoinHandle<R> {
    #[inline(always)]
    pub fn abort(&self) {
        self.task_flags
            .abort_required
            .store(true, Ordering::Relaxed);
    }
    #[inline(always)]
    pub fn abort_handle(&self) -> AbortHandle {
        AbortHandle {
            task_flags: self.task_flags.clone(),
        }
    }
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        self.task_flags.is_finished.load(Ordering::Relaxed)
    }
}

impl<T> Future for JoinHandle<T> {
    type Output = std::result::Result<T, JoinError>;

    #[inline(always)]
    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Self::Output> {
        match self.task_outcome_rx.poll_unpin(cx) {
            std::task::Poll::Pending => std::task::Poll::Pending,
            std::task::Poll::Ready(Ok(result)) => std::task::Poll::Ready(result),
            // An error here means the oneshot sender has been dropped without
            // setting a value, which is something the task is never supposed to do.
            std::task::Poll::Ready(Err(_)) => unreachable!(),
        }
    }
}

/*
 * AbortHandle
 */

#[derive(Debug)]
pub struct AbortHandle {
    task_flags: Arc<TaskSharedFlags>,
}

impl AbortHandle {
    pub fn abort(&self) {
        self.task_flags
            .abort_required
            .store(true, Ordering::Relaxed);
    }
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        self.task_flags.is_finished.load(Ordering::Relaxed)
    }
}

/*
 * Task
 */

#[derive(Debug)]
struct TaskSharedFlags {
    is_finished: AtomicBool,
    abort_required: AtomicBool,
}

#[pin_project(PinnedDrop)]
struct Task<T>
where
    T: Future + 'static,
    T::Output: 'static,
{
    outcome_sx: Option<oneshot::Sender<Result<T::Output, JoinError>>>,
    flags: Arc<TaskSharedFlags>,
    #[pin]
    future: T,
}

impl<T> Future for Task<T>
where
    T: Future + 'static,
    T::Output: 'static,
{
    type Output = ();

    #[inline(always)]
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();

        macro_rules! finish {
            ($outcome:expr) => {{
                let outcome_sx = this.outcome_sx.take().expect("called after complete");
                // If this errors, it means the handle has been dropped, so no big deal.
                let _ = outcome_sx.send($outcome);
                this.flags.is_finished.store(true, Ordering::Relaxed);
                std::task::Poll::Ready(())
            }};
        }

        if this.flags.abort_required.load(Ordering::Relaxed) {
            // TODO: should we drop `this.future` here ?
            return finish!(Err(JoinError::cancelled()));
        }

        let poll_outcome =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| this.future.poll(cx)));
        match poll_outcome {
            Ok(std::task::Poll::Pending) => std::task::Poll::Pending,
            Ok(std::task::Poll::Ready(result)) => finish!(Ok(result)),
            // The future has panicked !
            Err(err) => finish!(Err(JoinError::panic(err))),
        }
    }
}

#[pinned_drop]
impl<T> PinnedDrop for Task<T>
where
    T: Future + 'static,
    T::Output: 'static,
{
    fn drop(self: std::pin::Pin<&mut Self>) {
        let this = self.project();
        if let Some(outcome_sx) = this.outcome_sx.take() {
            // If this errors, it means the handle has been dropped, so no big deal.
            let _ = outcome_sx.send(Err(JoinError::cancelled()));
        }
    }
}

/*
 * spawn
 */

#[inline(always)]
pub fn spawn<T>(future: T) -> JoinHandle<T::Output>
where
    T: Future + 'static,
    T::Output: 'static,
{
    let (task_outcome_sx, task_outcome_rx) = oneshot::channel::<Result<T::Output, JoinError>>();
    let task_flags = Arc::new(TaskSharedFlags {
        is_finished: AtomicBool::new(false),
        abort_required: AtomicBool::new(false),
    });

    let future = Task {
        outcome_sx: Some(task_outcome_sx),
        flags: task_flags.clone(),
        future,
    };
    wasm_bindgen_futures::spawn_local(future);

    JoinHandle {
        task_outcome_rx,
        task_flags,
    }
}
