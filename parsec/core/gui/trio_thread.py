# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from contextlib import contextmanager
import trio
import threading
from inspect import iscoroutinefunction
from structlog import get_logger
from parsec.core.fs import FSError
from PyQt5.QtCore import pyqtBoundSignal, Q_ARG, QMetaObject, Qt


logger = get_logger()


class JobResultError(Exception):
    def __init__(self, status, **kwargs):
        self.status = status
        self.params = kwargs


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


class QtToTrioJob:
    def __init__(self, portal, fn, args, kwargs, qt_on_success, qt_on_error):
        self._portal = portal
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

    def is_finished(self):
        return self._done.is_set()

    async def _run_fn(self, *, task_status=trio.TASK_STATUS_IGNORED):
        try:
            with trio.CancelScope() as self.cancel_scope:
                task_status.started()
                self._started.set()

                try:
                    if iscoroutinefunction(self._fn):
                        self.ret = await self._fn(*self._args, **self._kwargs)
                    else:
                        self.ret = self._fn(*self._args, **self._kwargs)
                    self.status = "ok"

                except JobResultError as exc:
                    self.status = exc.status
                    self.exc = exc

                except trio.Cancelled as exc:
                    self.status = "cancelled"
                    self.exc = exc
                    raise

                except FSError as exc:
                    self.status = "ko"
                    self.exc = exc

                except Exception as exc:
                    logger.exception("Uncatched error", exc_info=exc)
                    self.status = "crashed"
                    self.exc = JobResultError(
                        "crashed", exc=exc, info=f"Unexpected error: {repr(exc)}"
                    )
                    self.exc.__traceback__ = exc.__traceback__

        finally:
            self._done.set()
            signal = self._qt_on_success if self.status == "ok" else self._qt_on_error
            if signal.args_types:
                signal.emit(self)
            else:
                signal.emit()

    def cancel_and_join(self):
        assert self.cancel_scope
        self._portal.run_sync(self.cancel_scope.cancel)
        self._done.wait()


class QtToTrioJobScheduler:
    def __init__(self):
        self._portal = None
        self._cancel_scope = None
        self.started = threading.Event()
        self._stopped = trio.Event()

    async def _start(self, *, task_status=trio.TASK_STATUS_IGNORED):
        assert not self.started.is_set()
        self._portal = trio.BlockingTrioPortal()
        self._send_job_channel, recv_job_channel = trio.open_memory_channel(1)
        try:
            async with trio.open_nursery() as nursery, recv_job_channel:
                self._cancel_scope = nursery.cancel_scope
                self.started.set()
                task_status.started()
                while True:
                    job = await recv_job_channel.receive()
                    assert job.status is None
                    await nursery.start(job._run_fn)

        finally:
            self._stopped.set()

    async def _stop(self):
        self._cancel_scope.cancel()
        await self._send_job_channel.aclose()
        await self._stopped.wait()

    def stop(self):
        self._portal.run(self._stop)

    def submit_job(self, qt_on_success, qt_on_error, fn, *args, **kwargs):
        # Fool-proof sanity check, signals must be wrapped in `ThreadSafeQtSignal`
        assert not [x for x in args if isinstance(x, pyqtBoundSignal)]
        assert not [v for v in kwargs.values() if isinstance(v, pyqtBoundSignal)]
        job = QtToTrioJob(self._portal, fn, args, kwargs, qt_on_success, qt_on_error)

        async def _submit_job():
            # While inside this async function we are blocking the Qt thread
            # hence we just wait for the job to start (to avoid concurrent
            # crash if the job is cancelled)
            await self._send_job_channel.send(job)
            await job._started.wait()

        self._portal.run(_submit_job)
        return job

    # TODO: needed by legacy widget
    def run(self, afn, *args):
        return self._portal.run(afn, *args)

    # TODO: needed by legacy widget
    def run_sync(self, fn, *args):
        return self._portal.run_sync(fn, *args)


# TODO: Running the trio loop in a QThread shouldn't be needed
# make sure it's the case, then remove this dead code
# class QtToTrioJobSchedulerThread(QThread):
#     def __init__(self, job_scheduler):
#         super().__init__()
#         self.job_scheduler = job_scheduler

#     def run(self):
#         trio.run(self.job_scheduler._start)


@contextmanager
def run_trio_thread():
    job_scheduler = QtToTrioJobScheduler()
    thread = threading.Thread(target=trio.run, args=[job_scheduler._start])
    thread.setName("TrioLoop")
    thread.start()
    # thread = QtToTrioJobSchedulerThread(job_scheduler)
    # thread.start()
    job_scheduler.started.wait()

    try:
        yield job_scheduler

    finally:
        job_scheduler.stop()
        thread.join()
        # thread.wait()
