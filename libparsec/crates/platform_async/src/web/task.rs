// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use futures::FutureExt;
use pin_project::{pin_project, pinned_drop};
use std::num::NonZeroU64;
use std::sync::{atomic::AtomicU64, Mutex};
use std::{
    any::Any,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use crate::{future::Future, oneshot};

// Most of the code in this file has been adapted from Tokio (MIT License).

/*
 * TaskID
 */

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct TaskID(pub(crate) NonZeroU64);

impl std::fmt::Display for TaskID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

pub fn try_task_id() -> Option<TaskID> {
    current_task_id()
}

impl TaskID {
    pub(crate) fn next() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(1);

        loop {
            let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
            if let Some(id) = NonZeroU64::new(id) {
                return Self(id);
            }
        }
    }
}

/*
 * JoinError
 */

pub struct JoinError {
    repr: Repr,
    id: TaskID,
}

pub enum Repr {
    Cancelled,
    Panic(SyncWrapper<Box<dyn Any + Send + 'static>>),
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

impl JoinError {
    pub(crate) fn cancelled(id: TaskID) -> JoinError {
        JoinError {
            repr: Repr::Cancelled,
            id,
        }
    }

    pub(crate) fn panic(id: TaskID, err: Box<dyn Any + Send + 'static>) -> JoinError {
        JoinError {
            repr: Repr::Panic(SyncWrapper::new(err)),
            id,
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

    pub fn id(&self) -> TaskID {
        self.id
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
            Repr::Cancelled => write!(fmt, "task {} was cancelled", self.id),
            Repr::Panic(p) => match panic_payload_as_str(p) {
                Some(panic_str) => {
                    write!(
                        fmt,
                        "task {} panicked with message {:?}",
                        self.id, panic_str
                    )
                }
                None => {
                    write!(fmt, "task {} panicked", self.id)
                }
            },
        }
    }
}

impl std::fmt::Debug for JoinError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.repr {
            Repr::Cancelled => write!(fmt, "JoinError::Cancelled({:?})", self.id),
            Repr::Panic(p) => match panic_payload_as_str(p) {
                Some(panic_str) => {
                    write!(fmt, "JoinError::Panic({:?}, {:?}, ...)", self.id, panic_str)
                }
                None => write!(fmt, "JoinError::Panic({:?}, ...)", self.id),
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
        fmt.debug_struct("JoinHandle")
            .field("id", &self.task_flags.id)
            .finish()
    }
}

impl<R> JoinHandle<R> {
    #[inline(always)]
    pub fn abort(&self) {
        if let Some(sx) = self
            .task_flags
            .abort_required_sx
            .lock()
            .expect("Mutex is poisoned")
            .take()
        {
            // Error here means the receiver has been dropped, which means the task is already finished.
            let _ = sx.send(());
        }
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

    #[inline(always)]
    pub fn id(&self) -> TaskID {
        self.task_flags.id
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

pub struct AbortHandle {
    task_flags: Arc<TaskSharedFlags>,
}

impl AbortHandle {
    #[inline(always)]
    pub fn abort(&self) {
        if let Some(sx) = self
            .task_flags
            .abort_required_sx
            .lock()
            .expect("Mutex is poisoned")
            .take()
        {
            // Error here means the receiver has been dropped, which means the task is already finished.
            let _ = sx.send(());
        }
    }

    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        self.task_flags.is_finished.load(Ordering::Relaxed)
    }

    #[inline(always)]
    pub fn id(&self) -> TaskID {
        self.task_flags.id
    }
}

impl std::fmt::Debug for AbortHandle {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("AbortHandle")
            .field("id", &self.task_flags.id)
            .finish()
    }
}

/*
 * Task
 */

#[derive(Debug)]
struct TaskSharedFlags {
    id: TaskID,
    is_finished: AtomicBool,
    abort_required_sx: Mutex<Option<oneshot::Sender<()>>>,
}

#[pin_project(PinnedDrop)]
struct Task<T>
where
    T: Future + 'static,
    T::Output: 'static,
{
    outcome_sx: Option<oneshot::Sender<Result<T::Output, JoinError>>>,
    #[pin]
    abort_required_rx: oneshot::Receiver<()>,
    flags: Arc<TaskSharedFlags>,
    #[pin]
    future: Option<T>,
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
        let mut this = self.project();

        macro_rules! finish {
            ($outcome:expr) => {{
                let outcome_sx = this.outcome_sx.take().expect("called after complete");
                // Drop the future
                this.future.set(None);
                // If this errors, it means the handle has been dropped, so no big deal.
                let _ = outcome_sx.send($outcome);
                this.flags.is_finished.store(true, Ordering::Relaxed);
                std::task::Poll::Ready(())
            }};
        }

        match this.abort_required_rx.poll(cx) {
            std::task::Poll::Pending => (),
            std::task::Poll::Ready(_) => {
                // The task has been aborted !
                return finish!(Err(JoinError::cancelled(this.flags.id)));
            }
        }

        let previous_task_id = set_current_task_id(Some(this.flags.id));
        // `previous_task_id` is expected to be `None` since it is only configured
        // when polling the future of a task (i.e. what we do here !), and this is
        // not supposed to lead to recursive calls.
        assert!(previous_task_id.is_none());
        let poll_outcome = {
            let future = this
                .future
                .as_mut()
                .as_pin_mut()
                .expect("called after complete");
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| future.poll(cx)))
        };
        set_current_task_id(None);
        match poll_outcome {
            Ok(std::task::Poll::Pending) => std::task::Poll::Pending,
            Ok(std::task::Poll::Ready(result)) => finish!(Ok(result)),
            // The future has panicked !
            Err(err) => finish!(Err(JoinError::panic(this.flags.id, err))),
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
        // The task is always passed to wasm bindgen's `spawn` right after being created,
        // hence it should be polled until completion, at which point `outcome_sx`&`future`
        // have been used.
        assert!(this.outcome_sx.is_none());
        assert!(this.future.is_none());
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
    let (abort_required_sx, abort_required_rx) = oneshot::channel::<()>();
    let task_flags = Arc::new(TaskSharedFlags {
        id: TaskID::next(),
        is_finished: AtomicBool::new(false),
        abort_required_sx: Mutex::new(Some(abort_required_sx)),
    });

    let future = Task {
        outcome_sx: Some(task_outcome_sx),
        abort_required_rx,
        flags: task_flags.clone(),
        future: Some(future),
    };
    wasm_bindgen_futures::spawn_local(future);

    JoinHandle {
        task_outcome_rx,
        task_flags,
    }
}

/*
 * WithTaskIDFuture
 */

/// On native platform everything is simple:
/// - Tokio runs the futures.
/// - Either a futures runs as a task, in which case Tokio gave it a task ID...
/// - ...or the futures runs from something like `tokio::block_on` (e.g. from
///   `#[tokio::main]`), in which case this is not a task so it does not have
///   a task ID.
///
/// However on web, we have to interface ourselves with the JS event loop:
/// - There is no `tokio::block_on` equivalent, one less thing to worry about ;-)
/// - Futures explicitly spawned from our code are run as tasks and have a task ID...
/// - ...but futures running from JS code (i.e. from the bindings) have no concept
///   of task even if they are very similar in practice (i.e. they are isolated
///   piece of asynchronous work).
///
/// This is especially an issue since we rely on task ID to track errors such as
/// invalid locking.
///
/// Hence this special future wrapper that should be used by the bindings to give
/// a task ID to the futures coming from the JS event loop.
#[pin_project]
pub struct WithTaskIDFuture<T>
where
    T: Future + 'static,
    T::Output: 'static,
{
    task_id: TaskID,
    #[pin]
    future: T,
}

impl<T> From<T> for WithTaskIDFuture<T>
where
    T: Future + 'static,
    T::Output: 'static,
{
    fn from(future: T) -> Self {
        Self {
            task_id: TaskID::next(),
            future,
        }
    }
}

impl<T> Future for WithTaskIDFuture<T>
where
    T: Future + 'static,
    T::Output: 'static,
{
    type Output = T::Output;

    #[inline(always)]
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();

        let previous_task_id = set_current_task_id(Some(*this.task_id));
        // `previous_task_id` is expected to be `None` since it is only configured
        // when polling the future of a task (i.e. what we do here !), and this is
        // not supposed to lead to recursive calls.
        assert!(previous_task_id.is_none());

        let poll_outcome =
            { std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| this.future.poll(cx))) };

        set_current_task_id(None);

        match poll_outcome {
            Ok(res) => res,
            Err(err) => {
                std::panic::resume_unwind(err);
            }
        }
    }
}

/*
 * Thread-local context (to implement coroutine local)
 */

struct Context {
    current_task_id: std::cell::Cell<Option<TaskID>>,
}

std::thread_local!(static CONTEXT: Context = const { Context { current_task_id: std::cell::Cell::new(None) } });

fn set_current_task_id(id: Option<TaskID>) -> Option<TaskID> {
    CONTEXT
        .try_with(|ctx| ctx.current_task_id.replace(id))
        .unwrap_or(None)
}

fn current_task_id() -> Option<TaskID> {
    CONTEXT
        .try_with(|ctx| ctx.current_task_id.get())
        .unwrap_or(None)
}
