// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use futures::{future::BoxFuture, FutureExt};
use pyo3::{
    import_exception, once_cell::GILOnceCell, pyclass, pyfunction, pymethods, types::PyString,
    wrap_pyfunction, IntoPy, Py, PyAny, PyObject, PyResult, Python,
};
use std::future::Future;
use tokio::task::JoinHandle;

import_exception!(trio, RunFinishedError);

// We have one global Tokio runtime that is lazily initialized
struct Stuff {
    tokio_runtime: tokio::runtime::Runtime,
    // Cache Python function to save time when we need them
    // Stuff needed when we will have to wake up the trio coroutine. Note we do as much work
    // has possible now given we can turn error into Python exception, while we will have to
    // raise a Rust panic! in the code waking up trio (given error there means we cannot reach Python !)
    trio_current_trio_token_fn: PyObject,
    trio_reschedule_fn: PyObject,
    trio_current_task_fn: PyObject,
    trio_wait_task_rescheduled: PyObject,
    trio_abort_succeeded: PyObject,
    outcome_value_fn: PyObject,
    outcome_error_fn: PyObject,
}

static STUFF: GILOnceCell<Stuff> = GILOnceCell::new();

fn get_stuff(py: Python<'_>) -> &Stuff {
    // We accept unwrap because it runs once and it avoids the Result everytime
    STUFF.get_or_init(py, || {
        let trio_lowlevel = py.import("trio").unwrap().getattr("lowlevel").unwrap();
        let outcome = py.import("outcome").unwrap();
        Stuff {
            tokio_runtime: tokio::runtime::Runtime::new().unwrap(),
            trio_current_trio_token_fn: trio_lowlevel
                .getattr("current_trio_token")
                .unwrap()
                .into_py(py),
            trio_reschedule_fn: trio_lowlevel.getattr("reschedule").unwrap().into_py(py),
            trio_current_task_fn: trio_lowlevel.getattr("current_task").unwrap().into_py(py),
            trio_wait_task_rescheduled: trio_lowlevel
                .getattr("wait_task_rescheduled")
                .unwrap()
                .into_py(py),
            trio_abort_succeeded: trio_lowlevel
                .getattr("Abort")
                .unwrap()
                .getattr("SUCCEEDED")
                .unwrap()
                .into_py(py),
            outcome_value_fn: outcome.getattr("Value").unwrap().into_py(py),
            outcome_error_fn: outcome.getattr("Error").unwrap().into_py(py),
        }
    })
}

#[pyfunction]
fn safe_trio_reschedule_fn(
    py: Python,
    task: &PyAny,
    value: &PyAny,
    future_id: &str,
) -> PyResult<()> {
    // Check `task.custom_sleep_data` to know if scheduling is required or forbidden
    let rescheduling_required = rescheduling_required(task, future_id);
    if !rescheduling_required {
        return Ok(());
    }
    let stuff = get_stuff(py);
    stuff.trio_reschedule_fn.call1(py, (task, value))?;
    Ok(())
}

fn rescheduling_required(task: &PyAny, future_id: &str) -> bool {
    task.getattr("custom_sleep_data")
        .and_then(|v| v.extract::<&str>())
        .map(|x| x == future_id)
        .unwrap_or_default()
}

#[pyclass]
struct TokioTaskAborterFromTrio {
    handle: JoinHandle<()>,
    task: PyObject,
}

#[pymethods]
impl TokioTaskAborterFromTrio {
    fn __call__(&self, py: Python, _raise_cancel: &PyAny) -> PyResult<PyObject> {
        // Given we return `trio.lowlevel.Abort.SUCCEEDED` we have given our word to trio we won't call reschedule for this task.
        // So we clear the `task.custom_sleep_data` attribute to indicate the rescheduling is no longer required.
        self.task.setattr(py, "custom_sleep_data", py.None())?;
        self.handle.abort();
        Ok(get_stuff(py).trio_abort_succeeded.clone())
    }
}

#[pyclass]
/// A wrapper for a rust `future` that will be polled in the trio context.
pub(crate) struct FutureIntoCoroutine(Option<BoxFuture<'static, PyResult<PyObject>>>);

impl FutureIntoCoroutine {
    pub fn from_raw<F>(fut: F) -> Self
    where
        F: Future<Output = PyResult<PyObject>> + Send + 'static,
    {
        FutureIntoCoroutine(Some(fut.boxed()))
    }

    pub fn from<F, R>(fut: F) -> Self
    where
        F: Future<Output = PyResult<R>> + Send + 'static,
        R: IntoPy<PyObject>,
    {
        Self::from_raw(async move {
            let res = fut.await?;
            Python::with_gil(|py| Ok(res.into_py(py)))
        })
    }
}

#[pymethods]
impl FutureIntoCoroutine {
    fn __await__(&mut self, py: Python<'_>) -> PyResult<PyObject> {
        let fut = self.0.take().expect("Already awaited coroutine");

        let stuff = get_stuff(py);
        let trio_token = stuff.trio_current_trio_token_fn.call0(py)?;
        let trio_reschedule_fn: Py<PyAny> =
            wrap_pyfunction!(safe_trio_reschedule_fn, py)?.into_py(py);
        let outcome_value_fn = stuff.outcome_value_fn.clone();
        let outcome_error_fn = stuff.outcome_error_fn.clone();

        let trio_current_task = stuff.trio_current_task_fn.call0(py)?;
        let task = trio_current_task.clone();
        let future_id = uuid::Uuid::new_v4().to_string();

        // Set `custom_sleep_data` to indicate the rescheduling is required.
        // This will be cleared if the task is rescheduled or cancelled.
        trio_current_task.setattr(py, "custom_sleep_data", &future_id)?;

        // Schedule the Tokio future
        let handle = stuff.tokio_runtime.spawn(async move {
            // Here we have left the trio thread and are inside a thread provided by the Tokio runtime

            // Actual run of the Tokio future
            let ret = std::panic::AssertUnwindSafe(fut).catch_unwind().await;
            let ret = ret.unwrap_or_else(|panic_err| {
                let msg = match panic_err.downcast::<&str>() {
                    Ok(msg) => msg.to_string(),
                    Err(panic_err) => match panic_err.downcast::<String>() {
                        Ok(msg) => *msg,
                        Err(panic_err) => format!("Unknown error type {:?}", panic_err.type_id()),
                    },
                };
                Err(pyo3::panic::PanicException::new_err(msg))
            });

            // Now that our job is done we have to call trio's reschedule function and
            // pass it the result as an outcome object
            Python::with_gil(|py| {
                let trio_current_task = trio_current_task.as_ref(py);

                // Create the outcome object
                let outcome = process_outcome_value(py, ret, outcome_value_fn, outcome_error_fn);
                notify_trio(
                    py,
                    trio_token,
                    trio_reschedule_fn,
                    trio_current_task,
                    outcome,
                    &future_id,
                );
            })
        });

        // This aborter is callback that will be called by trio if our coroutine gets
        // cancelled, this allow us to cancel the related tokio task
        let aborter = TokioTaskAborterFromTrio { handle, task };
        // Return the special coroutine object that tells trio loop to block our task
        // until it is manually rescheduled
        stuff
            .trio_wait_task_rescheduled
            .as_ref(py)
            .call1((aborter,))?
            .call_method0("__await__")
            .map(|value| value.into_py(py))
    }
}

fn process_outcome_value(
    py: Python<'_>,
    value: PyResult<PyObject>,
    outcome_value_fn: PyObject,
    outcome_error_fn: PyObject,
) -> PyObject {
    match value {
        Ok(val) => outcome_value_fn
            .call1(py, (val,))
            .expect("Cannot create `outcome.Value(<result>)`"),
        Err(err) => outcome_error_fn
            .call1(py, (err,))
            .expect("Cannot create `outcome.Error(<result>)`"),
    }
}

fn notify_trio(
    py: Python<'_>,
    trio_token: PyObject,
    trio_reschedule_fn: PyObject,
    trio_current_task: &PyAny,
    outcome: PyObject,
    future_id: &str,
) {
    // Reschedule the coroutine, this must be done from within the trio thread so we have to use
    // the trio token to schedule a synchronous function that will do the actual schedule call
    match trio_token.call_method1(
        py,
        "run_sync_soon",
        (
            trio_reschedule_fn,
            trio_current_task,
            outcome,
            PyString::new(py, future_id),
        ),
    ) {
        Ok(_) => (),
        // We can ignore the error if the trio loop has been closed...
        Err(err) if err.is_instance_of::<RunFinishedError>(py) => (),
        // ...but for any other exception there is not much we can do :'(
        Err(err) => {
            panic!(
                "Cannot call `TrioToken.run_sync_soon(trio.lowlevel.reschedule, <outcome>)`: {:?}",
                err
            );
        }
    }
}
