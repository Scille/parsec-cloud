# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import inspect
import functools
from parsec.core.gui.app import run_gui
from PyQt5.QtCore import pyqtBoundSignal

__all__ = ("run_gui",)


def patch_pyqtboundsignal_connect():
    """Allow qt signals to connect to async slot as long as the slot
    is bounded to an instance with a `jobs_ctx` atrribute to schedule
    trio jobs
    """
    if getattr(pyqtBoundSignal, "_connect_patched", False):
        return

    original_connect = pyqtBoundSignal.connect

    @functools.wraps(original_connect)
    def new_connect(self, method):
        bounded_original_connect = original_connect.__get__(self)
        if not inspect.iscoroutinefunction(method):
            return bounded_original_connect(method)
        instance = getattr(method, "__self__", None)
        if instance is None:
            raise ValueError("No instance for async qt callback")
        jobs_ctx = getattr(instance, "jobs_ctx", None)
        if jobs_ctx is None:
            raise ValueError("No job context for async qt callback")

        signature = inspect.signature(method)
        nb_args = len(signature.parameters)

        @functools.wraps(method)
        def slot(*args):
            args = args[:nb_args]
            jobs_ctx.submit_job(None, None, method, *args)

        return bounded_original_connect(slot)

    pyqtBoundSignal.connect = new_connect
    pyqtBoundSignal._connect_patched = True


# Patch
patch_pyqtboundsignal_connect()
