# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import threading
from contextlib import contextmanager
from inspect import iscoroutinefunction, signature

import trio
from structlog import get_logger
from parsec.core.fs import FSError
from parsec.core.mountpoint import MountpointError
from parsec.utils import trio_run, split_multi_error
from PyQt5.QtCore import pyqtBoundSignal, Q_ARG, QMetaObject, Qt


logger = get_logger()


class JobResultError(Exception):
    def __init__(self, status, **kwargs):
        self.status = status
        self.params = kwargs
        super().__init__(status, kwargs)


class JobSchedulerNotAvailable(Exception):
    pass


class ThreadSafeQtSignal:
    def __init__(self, qobj, signal_name, *args_types):
        signal = getattr(qobj, signal_name)
        assert isinstance(signal, pyqtBoundSignal)
        self.qobj = qobj
        self.signal_name = signal_name
        self.args_types = args_types

    def emit(self, *args):
        assert len(self.args_types) == len(args)
        cooked_args = [Q_ARG(t, v) for t, v in zip(self.args_types, args)]
        QMetaObject.invokeMethod(self.qobj, self.signal_name, Qt.QueuedConnection, *cooked_args)

    def __repr__(self):
        return f"<ThreadSafeQtSignal {self.signal_name}>"


class QtToTrioJob:
    def __init__(self, trio_token, fn, args, kwargs, qt_on_success, qt_on_error):
        self._trio_token = trio_token
        assert isinstance(qt_on_success, ThreadSafeQtSignal)
        assert qt_on_success.args_types in ((), (QtToTrioJob,))
        self._qt_on_success = qt_on_success
        assert isinstance(qt_on_error, ThreadSafeQtSignal)
        assert qt_on_error.args_types in ((), (QtToTrioJob,))
        self._qt_on_error = qt_on_error
        self._fn = fn
        self._args = args
        self._kwargs = kwargs
        self.cancel_scope = None
        self._started = trio.Event()
        self._done = threading.Event()
        self.status = None
        self.ret = None
        self.exc = None

    def __str__(self):
        return f"{self._fn.__name__}"

    @property
    def arguments(self):
        bound_arguments = signature(self._fn).bind(*self._args, **self._kwargs)
        bound_arguments.apply_defaults()
        return bound_arguments.arguments

    def is_finished(self):
        return self._done.is_set()

    def is_cancelled(self):
        return self.status == "cancelled"

    def is_ok(self):
        return self.status == "ok"

    def is_errored(self):
        return self.status == "ko"

    def is_crashed(self):
        return self.status == "crashed"

    async def __call__(self, *, task_status=trio.TASK_STATUS_IGNORED):
        assert self.status is None
        with trio.CancelScope() as self.cancel_scope:
            task_status.started()
            self._started.set()

            try:
                if not iscoroutinefunction(self._fn):
                    result = self._fn(*self._args, **self._kwargs)
                else:
                    result = await self._fn(*self._args, **self._kwargs)
                self.set_result(result)

            except Exception as exc:
                self.set_exception(exc)

            except trio.Cancelled as exc:
                self.set_cancelled(exc)
                raise

            except trio.MultiError as exc:
                cancelled_errors, other_exceptions = split_multi_error(exc)
                if other_exceptions:
                    self.set_exception(other_exceptions)
                else:
                    self.set_cancelled(cancelled_errors)
                if cancelled_errors:
                    raise cancelled_errors

    def set_result(self, result):
        self.status = "ok"
        self.ret = result
        self._set_done()

    def set_cancelled(self, exc):
        self.status = "cancelled"
        self.exc = exc
        self._set_done()

    def set_exception(self, exc):
        if isinstance(exc, JobResultError):
            self.status = exc.status
            self.exc = exc
        elif isinstance(exc, (FSError, MountpointError)):
            self.status = "ko"
            self.exc = exc
        else:
            logger.exception("Uncatched error", exc_info=exc)
            wrapped = JobResultError("crashed", exc=exc, info=f"Unexpected error: {repr(exc)}")
            wrapped.__traceback__ = exc.__traceback__
            self.status = wrapped.status
            self.exc = wrapped
        self._set_done()

    def _set_done(self):
        self._done.set()
        signal = self._qt_on_success if self.is_ok() else self._qt_on_error
        signal.emit(self) if signal.args_types else signal.emit()

    def cancel_and_join(self):
        assert self.cancel_scope
        try:
            trio.from_thread.run_sync(self.cancel_scope.cancel, trio_token=self._trio_token)
        except trio.RunFinishedError:
            pass
        self._done.wait()


class QtToTrioJobScheduler:
    def __init__(self):
        self._trio_token = None
        self._cancel_scope = None
        self.started = threading.Event()
        self._stopped = trio.Event()
        self._throttling_scheduled_jobs = {}
        self._throttling_last_executed = {}

    async def _start(self, *, task_status=trio.TASK_STATUS_IGNORED):
        assert not self.started.is_set()
        self._trio_token = trio.lowlevel.current_trio_token()
        self._send_job_channel, recv_job_channel = trio.open_memory_channel(1)
        try:
            async with trio.open_service_nursery() as nursery, recv_job_channel:
                self._cancel_scope = nursery.cancel_scope
                self.started.set()
                task_status.started()
                while True:
                    job = await recv_job_channel.receive()
                    await nursery.start(job)

        finally:
            self._stopped.set()

    async def _stop(self):
        self._cancel_scope.cancel()
        await self._send_job_channel.aclose()
        await self._stopped.wait()

    def stop(self):
        try:
            trio.from_thread.run(self._stop, trio_token=self._trio_token)
        except trio.RunFinishedError:
            pass

    def _run_job(self, job, *args, sync=False):
        try:
            if sync:
                return trio.from_thread.run_sync(job, *args, trio_token=self._trio_token)
            else:
                return trio.from_thread.run(job, *args, trio_token=self._trio_token)

        except trio.BrokenResourceError:
            logger.info(f"The submitted job `{job}` won't run as the scheduler is stopped")
            raise JobSchedulerNotAvailable("The job scheduler is stopped")

        except trio.RunFinishedError:
            logger.info(f"The submitted job `{job}` won't run as the trio loop is not running")
            raise JobSchedulerNotAvailable("The trio loop is not running")

    def submit_throttled_job(
        self, throttling_id: str, delay: float, qt_on_success, qt_on_error, fn, *args, **kwargs
    ):
        """
        Throttle execution: immediatly execute `fn` unless a job with a similar
        `throttling_id` it has been already executed in the last `delay` seconds,
        in which case we schedule the execution to occur when the delay is elapsed.
        Submitting a job with an already sheduled `throttling_id` will lead to
        a single execution of the last provided job parameters at the soonest delay.
        """
        # Fool-proof sanity check, signals must be wrapped in `ThreadSafeQtSignal`
        assert not [x for x in args if isinstance(x, pyqtBoundSignal)]
        assert not [v for v in kwargs.values() if isinstance(v, pyqtBoundSignal)]

        async def _throttled_execute(task_status=trio.TASK_STATUS_IGNORED):
            # Create the job but don't execute it: we have to handle throttle first !
            job = QtToTrioJob(self._trio_token, fn, args, kwargs, qt_on_success, qt_on_error)

            # Only modify `_throttling_scheduled_jobs` from the trio
            # thread to avoid concurrent acces with the Qt thread
            # Note we might be overwritting another job here, it is fine given
            # we only want to execute the last scheduled one
            self._throttling_scheduled_jobs[throttling_id] = job
            task_status.started()

            # Sleep the throttle delay, if last execution occurred long enough
            # this will equivalent of doing `trio.sleep(0)`
            last_executed = self._throttling_last_executed.get(throttling_id, 0)
            await trio.sleep_until(last_executed + delay)

            # It is possible our job has been executed by another `_throttled_execute`
            # that had a shorter delay. In such case we have nothing more to do.
            last_executed = self._throttling_last_executed.get(throttling_id, 0)
            if last_executed + delay > trio.current_time():
                return
            job = self._throttling_scheduled_jobs.pop(throttling_id, None)
            if not job:
                return

            # Actually start the job execution
            self._throttling_last_executed[throttling_id] = trio.current_time()
            try:
                await self._send_job_channel.send(job)
            except trio.BrokenResourceError:
                # Job scheduler has been closed, nothing more can be done
                pass

        async def _send_throttle_job():
            # While inside this async function we are blocking the Qt thread
            # hence we just submit `_throttled_execute`
            await self._send_job_channel.send(_throttled_execute)

        try:
            self._run_job(_send_throttle_job)
        except JobSchedulerNotAvailable:
            pass

    def submit_job(self, qt_on_success, qt_on_error, fn, *args, **kwargs):
        # Fool-proof sanity check, signals must be wrapped in `ThreadSafeQtSignal`
        assert not [x for x in args if isinstance(x, pyqtBoundSignal)]
        assert not [v for v in kwargs.values() if isinstance(v, pyqtBoundSignal)]
        job = QtToTrioJob(self._trio_token, fn, args, kwargs, qt_on_success, qt_on_error)

        async def _submit_job():
            # While inside this async function we are blocking the Qt thread
            # hence we just wait for the job to start (to avoid concurrent
            # crash if the job is cancelled)
            await self._send_job_channel.send(job)
            await job._started.wait()

        try:
            self._run_job(_submit_job)
        except JobSchedulerNotAvailable as exc:
            job.set_cancelled(exc)

        return job

    # This method is only here for legacy purposes.
    # It shouldn't NOT be used as running an async job synchronously
    # might block the Qt loop for too long and cause the application
    # to freeze. TODO: remove it later

    def run(self, afn, *args):
        return self._run_job(afn, *args)

    # In contrast to the `run` method, it is acceptable to block
    # the Qt loop while waiting for a synchronous job to finish
    # as it shouldn't take too long (it might simply wait for
    # a few scheduled trio task steps to finish). However,
    # it shouln't be used too aggressively as it might still slow
    # down the application.

    def run_sync(self, fn, *args):
        return self._run_job(fn, *args, sync=True)


@contextmanager
def run_trio_thread():
    job_scheduler = QtToTrioJobScheduler()
    thread = threading.Thread(target=trio_run, args=[job_scheduler._start])
    thread.setName("TrioLoop")
    thread.start()
    job_scheduler.started.wait()

    try:
        yield job_scheduler

    finally:
        job_scheduler.stop()
        thread.join()
