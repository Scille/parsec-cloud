# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import sys
import signal
from types import TracebackType
from typing import Any, Callable, Iterator, Type
from contextlib import contextmanager

import trio
import trio_typing
import qtrio
from structlog import get_logger
from PyQt5.QtCore import QTimer, Qt
from PyQt5.QtWidgets import QApplication

from parsec.event_bus import EventBus
from parsec.core.core_events import CoreEvent
from parsec.core.config import CoreConfig
from parsec.core.ipcinterface import (
    run_ipc_server,
    send_to_ipc_server,
    IPCServerAlreadyRunning,
    IPCServerNotRunning,
    IPCCommand,
)

try:
    from parsec.core.gui import lang
    from parsec.core.gui.parsec_application import ParsecApp
    from parsec.core.gui.new_version import CheckNewVersion
    from parsec.core.gui.systray import systray_available, Systray
    from parsec.core.gui.main_window import MainWindow
    from parsec.core.gui.trio_jobs import run_trio_job_scheduler
    from parsec.core.gui.custom_dialogs import QDialogInProcess
except ImportError as exc:
    raise ModuleNotFoundError(
        """PyQt forms haven't been generated.
Running `python misc/generate_pyqt.py build` should fix the issue
"""
    ) from exc


logger = get_logger()


def before_quit(systray: Systray) -> Callable[[], None]:
    def _before_quit() -> None:
        systray.hide()

    return _before_quit


async def _run_ipc_server(
    config: CoreConfig,
    main_window: MainWindow,
    start_arg: str | None,
    task_status: trio_typing.TaskStatus[None] = trio.TASK_STATUS_IGNORED,
) -> None:
    new_instance_needed = main_window.new_instance_needed
    foreground_needed = main_window.foreground_needed

    async def _cmd_handler(cmd: dict[str, object]) -> dict[str, str]:
        if cmd["cmd"] == IPCCommand.FOREGROUND:
            foreground_needed.emit()
        elif cmd["cmd"] == IPCCommand.NEW_INSTANCE:
            new_instance_needed.emit(cmd.get("start_arg"))
        return {"status": "ok"}

    # Loop over attemps at running an IPC server or sending the command to an existing one
    while True:

        # Attempt to run an IPC server if Parsec is not already started
        try:
            async with run_ipc_server(
                _cmd_handler, config.ipc_socket_file, win32_mutex_name=config.ipc_win32_mutex_name
            ):
                task_status.started()
                await trio.sleep_forever()

        # Parsec is already started, give it our work then
        except IPCServerAlreadyRunning:

            # Protect against race conditions, in case the server was shutting down
            try:
                if start_arg:
                    await send_to_ipc_server(
                        config.ipc_socket_file, IPCCommand.NEW_INSTANCE, start_arg=start_arg
                    )
                else:
                    await send_to_ipc_server(config.ipc_socket_file, IPCCommand.FOREGROUND)

            # IPC server has closed, retry to create our own
            except IPCServerNotRunning:
                continue

            # We have successfuly noticed the other running application
            # We can now forward the exception to the caller
            raise


@contextmanager
def fail_on_first_exception(kill_window: Callable[[], None]) -> Iterator[None]:
    exceptions = []

    def excepthook(
        etype: Type[BaseException], exception: BaseException, traceback: TracebackType | None
    ) -> object:
        exceptions.append(exception)
        kill_window()
        return previous_hook(etype, exception, traceback)

    sys.excepthook, previous_hook = excepthook, sys.excepthook

    try:
        yield
    finally:
        sys.excepthook = previous_hook
        if exceptions:
            raise exceptions[0]


@contextmanager
def log_pyqt_exceptions() -> Iterator[None]:
    # Override sys.excepthook to be able to properly log exceptions occuring in Qt slots.
    # Exceptions occuring in the core while in the Qt app should be catched sooner by the
    # job.

    def log_except(
        etype: Type[BaseException], exception: BaseException, traceback: TracebackType | None
    ) -> None:
        logger.exception("Exception in Qt slot", exc_info=(etype, exception, traceback))

    sys.excepthook, previous_hook = log_except, sys.excepthook
    try:
        yield
    finally:
        sys.excepthook = previous_hook


def run_gui(config: CoreConfig, start_arg: str | None = None, diagnose: bool = False) -> object:
    logger.info("Starting UI")

    # Needed for High DPI usage of QIcons, otherwise only QImages are well scaled
    QApplication.setAttribute(Qt.AA_EnableHighDpiScaling)
    QApplication.setAttribute(Qt.AA_UseHighDpiPixmaps)
    QApplication.setHighDpiScaleFactorRoundingPolicy(
        Qt.HighDpiScaleFactorRoundingPolicy.PassThrough
    )

    # The parsec app needs to be instanciated before qtrio runs in order
    # to be the default QApplication instance
    app = ParsecApp()
    assert QApplication.instance() is app
    return qtrio.run(_run_gui, app, config, start_arg, diagnose)


async def _run_gui(
    app: ParsecApp, config: CoreConfig, start_arg: str | None = None, diagnose: bool = False
) -> None:
    app.load_stylesheet()
    app.load_font()

    lang_key = lang.switch_language(config)

    event_bus = EventBus()
    async with run_trio_job_scheduler() as jobs_ctx:
        win = MainWindow(
            jobs_ctx=jobs_ctx,
            quit_callback=jobs_ctx.close,
            event_bus=event_bus,
            config=config,
            minimize_on_close=config.gui_tray_enabled and systray_available(),
        )

        # Attempt to run an IPC server if Parsec is not already started
        try:
            await jobs_ctx.nursery.start(_run_ipc_server, config, win, start_arg)
        # Another instance of Parsec already started, nothing more to do
        except IPCServerAlreadyRunning:
            return

        # If we are here, it's either the IPC server has successfully started
        # or it has crashed without being able to communicate with an existing
        # IPC server. Such case is of course not supposed to happen but if it
        # does we nevertheless keep the application running as a kind of
        # failsafe mode (and the crash reason is logged and sent to telemetry).

        # Systray is not displayed on MacOS, having natively a menu with similar functions.
        if systray_available() and sys.platform != "darwin":
            systray = Systray(parent=win)
            win.systray_notification.connect(systray.on_systray_notification)
            systray.on_close.connect(win.close_app)
            systray.on_show.connect(win.show_top)
            app.aboutToQuit.connect(before_quit(systray))
            if config.gui_tray_enabled:
                app.setQuitOnLastWindowClosed(False)

        if config.gui_check_version_at_startup and not diagnose:
            CheckNewVersion(jobs_ctx=jobs_ctx, event_bus=event_bus, config=config, parent=win)

        win.show_window(skip_dialogs=diagnose)
        win.show_top()
        win.new_instance_needed.emit(start_arg)

        if sys.platform == "darwin":
            # macFUSE is not bundled with Parsec and must be manually installed by the user
            # so we detect early such need to provide a help dialogue ;-)
            # TODO: provide a similar mechanism on Windows&Linux to handle mountpoint runner not available
            from parsec.core.gui.instance_widget import ensure_macfuse_available_or_show_dialogue

            ensure_macfuse_available_or_show_dialogue(win)

        def kill_window(*args: Any) -> None:
            win.close_app(force=True)

        signal.signal(signal.SIGINT, kill_window)

        # QTimer wakes up the event loop periodically which allows us to close
        # the window even when it is in background.
        timer = QTimer()
        timer.start(400)
        timer.timeout.connect(lambda: None)

        if diagnose:
            diagnose_timer = QTimer()
            diagnose_timer.start(1000)
            diagnose_timer.timeout.connect(kill_window)

        if lang_key:
            event_bus.send(CoreEvent.GUI_CONFIG_CHANGED, gui_language=lang_key)

        with QDialogInProcess.manage_pools():
            if diagnose:
                with fail_on_first_exception(kill_window):
                    await trio.sleep_forever()
            else:
                with log_pyqt_exceptions():
                    await trio.sleep_forever()
