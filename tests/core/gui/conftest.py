# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from unittest.mock import patch
from functools import wraps, partial
import trio
from trio.testing import trio_test as vanilla_trio_test
import queue
import threading

from parsec.event_bus import EventBus
from parsec.core.gui.main_window import MainWindow
from parsec.core.gui.trio_thread import QtToTrioJobScheduler
from parsec.core.gui.login_widget import LoginWidget, LoginLoginWidget
from parsec.core.gui.bootstrap_organization_widget import BootstrapOrganizationWidget
from parsec.core.gui.claim_user_widget import ClaimUserWidget
from parsec.core.gui.claim_device_widget import ClaimDeviceWidget
from parsec.core.gui.central_widget import CentralWidget
from parsec.core.gui.users_widget import UsersWidget
from parsec.core.gui.devices_widget import DevicesWidget
from parsec.core.gui.mount_widget import MountWidget
from parsec.core.gui.lang import switch_language


_qt_thread_gateway = None


# Must be an async fixture to be actually called after
# `_qt_thread_gateway` has been configured
@pytest.fixture
async def qt_thread_gateway(trio_in_side_thread):
    return _qt_thread_gateway


class ThreadAsyncGateway:
    def __init__(self):
        self._portal = None
        self._action_req_queue = queue.Queue(1)
        self._lock = trio.Lock()

    def _send_action_req(self, type, action):
        return self._action_req_queue.put_nowait((type, action))

    def _recv_action_req(self):
        return self._action_req_queue.get()

    def run_action_pump(self):
        while True:
            action, rep_callback = self._recv_action_req()
            if action == "stop":
                type, value = rep_callback
                if type == "exc":
                    raise value
                else:
                    return value

            try:
                rep = action()
                rep_callback("ret", rep)

            except Exception as exc:
                rep_callback("exc", exc)

    async def stop_action_pump(self, type, value):
        async with self._lock:
            self._send_action_req("stop", (type, value))

    async def send_action(self, action):
        portal = trio.BlockingTrioPortal()
        rep_sender, rep_receiver = trio.open_memory_channel(1)

        def _rep_callback(type, value):
            portal.run_sync(rep_sender.send_nowait, (type, value))

        async with self._lock:
            self._send_action_req(action, _rep_callback)
            type, value = await rep_receiver.receive()
            if type == "exc":
                raise value
            else:
                return value


# TODO: Running the trio loop in a QThread shouldn't be needed
# make sure it's the case, then remove this dead code
# class TrioQThread(QThread):
#     def __init__(self, fn, *args):
#         super().__init__()
#         self.fn = fn
#         self.args = args

#     def run(self):
#         self.fn(*self.args)


# Not an async fixture (and doesn't depend on an async fixture either)
# this fixture will actually be executed *before* pytest-trio setup
# the trio loop, giving us a chance to monkeypatch it !
@pytest.fixture
def trio_in_side_thread():
    def _trio_test(fn):
        @vanilla_trio_test
        @wraps(fn)
        async def inner_wrapper(**kwargs):
            try:
                ret = await fn(**kwargs)

            except BaseException as exc:
                await _qt_thread_gateway.stop_action_pump("exc", exc)
                raise

            else:
                await _qt_thread_gateway.stop_action_pump("value", ret)

        @wraps(fn)
        def outer_wrapper(**kwargs):
            global _qt_thread_gateway
            assert not _qt_thread_gateway
            _qt_thread_gateway = ThreadAsyncGateway()

            thread = threading.Thread(target=partial(inner_wrapper, **kwargs))
            thread.start()
            # thread = TrioQThread(partial(inner_wrapper, **kwargs))
            # thread.start()
            try:
                _qt_thread_gateway.run_action_pump()

            finally:
                # thread.wait()
                thread.join()
                _qt_thread_gateway = None

        return outer_wrapper

    with patch("pytest_trio.plugin.trio_test", new=_trio_test):
        yield


@pytest.fixture
async def aqtbot(qtbot, qt_thread_gateway):
    return AsyncQtBot(qtbot, qt_thread_gateway)


class CtxManagerAsyncWrapped:
    def __init__(self, qtbot, qt_thread_gateway, fnname, *args, **kwargs):
        self.qtbot = qtbot
        self.qt_thread_gateway = qt_thread_gateway
        self.fnname = fnname
        self.args = args
        self.kwargs = kwargs
        self.ctx = None

    async def __aenter__(self):
        def _action_enter():
            self.ctx = getattr(self.qtbot, self.fnname)(*self.args, **self.kwargs)
            self.ctx.__enter__()

        await self.qt_thread_gateway.send_action(_action_enter)

    async def __aexit__(self, exc_type, exc, tb):
        def _action_exit():
            self.ctx.__exit__(exc_type, exc, tb)

        await self.qt_thread_gateway.send_action(_action_exit)


class AsyncQtBot:
    def __init__(self, qtbot, qt_thread_gateway):
        self.qtbot = qtbot
        self.qt_thread_gateway = qt_thread_gateway

        def _autowrap(fnname):
            async def wrapper(*args, **kwargs):
                def _action():
                    return getattr(self.qtbot, fnname)(*args, **kwargs)

                return await self.qt_thread_gateway.send_action(_action)

            wrapper.__name__ = f"{fnname}"
            return wrapper

        self.key_click = _autowrap("keyClick")
        self.key_clicks = _autowrap("keyClicks")
        self.key_event = _autowrap("keyEvent")
        self.key_press = _autowrap("keyPress")
        self.key_release = _autowrap("keyRelease")
        # self.key_to_ascii = self.qtbot.keyToAscii  # available ?
        self.mouse_click = _autowrap("mouseClick")
        self.mouse_d_click = _autowrap("mouseDClick")
        self.mouse_move = _autowrap("mouseMove")
        self.mouse_press = _autowrap("mousePress")
        self.mouse_release = _autowrap("mouseRelease")

        self.add_widget = _autowrap("add_widget")
        self.stop = _autowrap("stop")
        self.wait = _autowrap("wait")

        def _autowrap_ctx_manager(fnname):
            def wrapper(*args, **kwargs):
                return CtxManagerAsyncWrapped(
                    self.qtbot, self.qt_thread_gateway, fnname, *args, **kwargs
                )

            wrapper.__name__ = f"{fnname}"
            return wrapper

        self.wait_signal = _autowrap_ctx_manager("wait_signal")
        self.wait_signals = _autowrap_ctx_manager("wait_signals")
        self.assert_not_emitted = _autowrap_ctx_manager("assert_not_emitted")
        self.wait_active = _autowrap_ctx_manager("wait_active")
        self.wait_exposed = _autowrap_ctx_manager("wait_exposed")
        self.capture_exceptions = _autowrap_ctx_manager("capture_exceptions")


@pytest.fixture
def autoclose_dialog(monkeypatch):
    class DialogSpy:
        def __init__(self):
            self.dialogs = []

    spy = DialogSpy()

    def _dialog_exec(dialog):
        spy.dialogs.append((dialog.label_title.text(), dialog.label_message.text()))

    monkeypatch.setattr(
        "parsec.core.gui.custom_widgets.MessageDialog.exec_", _dialog_exec, raising=False
    )
    return spy


@pytest.fixture
async def jobs_ctx():
    async with trio.open_nursery() as nursery:
        job_scheduler = QtToTrioJobScheduler()
        await nursery.start(job_scheduler._start)
        try:
            yield job_scheduler

        finally:
            await job_scheduler._stop()


@pytest.fixture
def gui_factory(qtbot, qt_thread_gateway, jobs_ctx, core_config):
    async def _gui_factory(event_bus=None, core_config=core_config):
        # First start popup blocks the test
        # Check version and mountpoint are useless for most tests
        core_config = core_config.evolve(
            gui_check_version_at_startup=False, gui_first_launch=False, mountpoint_enabled=False
        )
        event_bus = event_bus or EventBus()
        # Language config rely on global var, must reset it for each test !
        switch_language(core_config)

        def _create_main_window():
            # Pass minimize_on_close to avoid having test blocked by the
            # closing confirmation prompt

            main_w = MainWindow(jobs_ctx, event_bus, core_config, minimize_on_close=True)
            qtbot.add_widget(main_w)
            main_w.showMaximized()
            return main_w

        return await qt_thread_gateway.send_action(_create_main_window)

    return _gui_factory


@pytest.fixture
async def gui(gui_factory, event_bus, core_config):
    return await gui_factory(event_bus, core_config)


# Decorator to add a method to a class
def add_method(cls):
    def _add_method(func):
        @wraps(func)
        def wrapper(self, *args, **kwargs):
            return func(*args, **kwargs)

        setattr(cls, func.__name__, func)
        return func

    return _add_method


# Since widgets are not longer persistant and are instancied only when needed,
# we can no longer simply access them.
# These methods help to retrieve a widget according to the current state of the GUI.
# They're prefixed by "test_" to ensure that they do not erase any "real" methods.


@add_method(MainWindow)
def test_get_main_widget(self):
    item = self.widget_center.layout().itemAt(0)
    return item.widget()


@add_method(MainWindow)
def test_get_central_widget(self):
    main_widget = self.test_get_main_widget()
    if not isinstance(main_widget, CentralWidget):
        return None
    return main_widget


@add_method(MainWindow)
def test_get_login_widget(self):
    main_widget = self.test_get_main_widget()
    if not isinstance(main_widget, LoginWidget):
        return None
    return main_widget


@add_method(MainWindow)
def test_get_login_login_widget(self):
    login_w = self.test_get_login_widget()
    if not login_w:
        return
    item = login_w.layout.itemAt(0)
    w = item.widget()
    if not isinstance(w, LoginLoginWidget):
        return None
    return w


@add_method(MainWindow)
def test_get_claim_user_widget(self):
    login_w = self.test_get_login_widget()
    if not login_w:
        return
    item = login_w.layout.itemAt(0)
    w = item.widget()
    if not isinstance(w, ClaimUserWidget):
        return None
    return w


@add_method(MainWindow)
def test_get_claim_device_widget(self):
    login_w = self.test_get_login_widget()
    if not login_w:
        return
    item = login_w.layout.itemAt(0)
    w = item.widget()
    if not isinstance(w, ClaimDeviceWidget):
        return None
    return w


@add_method(MainWindow)
def test_get_bootstrap_organization_widget(self):
    login_w = self.test_get_login_widget()
    if not login_w:
        return
    item = login_w.layout.itemAt(0)
    w = item.widget()
    if not isinstance(w, BootstrapOrganizationWidget):
        return None
    return w


@add_method(MainWindow)
def test_get_users_widget(self):
    central_widget = self.test_get_central_widget()
    item = central_widget.widget_central.layout().itemAt(0)
    w = item.widget()
    if not isinstance(w, UsersWidget):
        return None
    return w


@add_method(MainWindow)
def test_get_devices_widget(self):
    central_widget = self.test_get_central_widget()
    item = central_widget.widget_central.layout().itemAt(0)
    w = item.widget()
    if not isinstance(w, DevicesWidget):
        return None
    return w


@add_method(MainWindow)
def test_get_mount_widget(self):
    central_widget = self.test_get_central_widget()
    item = central_widget.widget_central.layout().itemAt(0)
    w = item.widget()
    if not isinstance(w, MountWidget):
        return None
    return w


@add_method(MainWindow)
def test_get_workspaces_widget(self):
    mount_widget = self.test_get_mount_widget()
    return mount_widget.workspaces_widget


@add_method(MainWindow)
def test_get_files_widget(self):
    mount_widget = self.test_get_mount_widget()
    return mount_widget.files_widget
