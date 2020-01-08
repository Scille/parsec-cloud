# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
import signal
from structlog import get_logger
from queue import Queue

from PyQt5.QtCore import QTimer, Qt
from PyQt5.QtGui import QFont, QFontDatabase
from PyQt5.QtWidgets import QApplication

from parsec.core.config import CoreConfig
from parsec.event_bus import EventBus
from parsec.core.ipcinterface import (
    run_ipc_server,
    send_to_ipc_server,
    IPCServerAlreadyRunning,
    IPCServerNotRunning,
)

try:
    from parsec.core.gui import lang
    from parsec.core.gui.new_version import CheckNewVersion
    from parsec.core.gui.systray import systray_available, Systray
    from parsec.core.gui.main_window import MainWindow
    from parsec.core.gui.trio_thread import ThreadSafeQtSignal, run_trio_thread
except ImportError as exc:
    raise ModuleNotFoundError(
        """PyQt forms haven't been generated.
You must install the parsec package or run `python setup.py generate_pyqt_forms`
"""
    ) from exc


logger = get_logger()


def before_quit(systray):
    def _before_quit():
        systray.hide()

    return _before_quit


async def _start_ipc_server(config, main_window, start_arg, result_queue):
    new_instance_needed_qt = ThreadSafeQtSignal(main_window, "new_instance_needed", object)
    foreground_needed_qt = ThreadSafeQtSignal(main_window, "foreground_needed")

    async def cmd_handler(cmd):
        if cmd["cmd"] == "foreground":
            foreground_needed_qt.emit()
        elif cmd["cmd"] == "new_instance":
            new_instance_needed_qt.emit(cmd.get("start_arg"))
        return {"status": "ok"}

    while True:
        try:
            async with run_ipc_server(
                cmd_handler, config.ipc_socket_file, win32_mutex_name=config.ipc_win32_mutex_name
            ):
                result_queue.put("started")
                await trio.sleep_forever()

        except IPCServerAlreadyRunning:
            # Parsec is already started, give it our work then
            try:
                try:
                    if start_arg:
                        await send_to_ipc_server(
                            config.ipc_socket_file, "new_instance", start_arg=start_arg
                        )
                    else:
                        await send_to_ipc_server(config.ipc_socket_file, "foreground")
                finally:
                    result_queue.put("already_running")
                return

            except IPCServerNotRunning:
                # IPC server has closed, retry to create our own
                continue


def run_gui(config: CoreConfig, start_arg: str = None):
    logger.info("Starting UI")

    # Needed for High DPI usage of QIcons, otherwise only QImages are well scaled
    QApplication.setAttribute(Qt.AA_EnableHighDpiScaling)
    QApplication.setAttribute(Qt.AA_UseHighDpiPixmaps)

    app = QApplication([])
    app.setOrganizationName("Scille")
    app.setOrganizationDomain("parsec.cloud")
    app.setApplicationName("Parsec")

    QFontDatabase.addApplicationFont(":/fonts/fonts/Roboto-Regular.ttf")
    f = QFont("Roboto")
    app.setFont(f)

    lang_key = lang.switch_language(config)

    event_bus = EventBus()
    with run_trio_thread() as jobs_ctx:
        win = MainWindow(
            jobs_ctx=jobs_ctx,
            event_bus=event_bus,
            config=config,
            minimize_on_close=config.gui_tray_enabled and systray_available(),
        )

        result_queue = Queue(maxsize=1)

        class ThreadSafeNoQtSignal(ThreadSafeQtSignal):
            def __init__(self):
                self.qobj = None
                self.signal_name = ""
                self.args_types = ()

            def emit(self, *args):
                pass

        jobs_ctx.submit_job(
            ThreadSafeNoQtSignal(),
            ThreadSafeNoQtSignal(),
            _start_ipc_server,
            config,
            win,
            start_arg,
            result_queue,
        )
        if result_queue.get() == "already_running":
            # Another instance of Parsec already started, nothing more to do
            return

        if systray_available():
            systray = Systray(parent=win)
            win.systray_notification.connect(systray.on_systray_notification)
            systray.on_close.connect(win.close_app)
            systray.on_show.connect(win.show_top)
            app.aboutToQuit.connect(before_quit(systray))

        if config.gui_check_version_at_startup:
            CheckNewVersion(jobs_ctx=jobs_ctx, event_bus=event_bus, config=config, parent=win)

        win.showMaximized()
        win.show_top()
        win.new_instance_needed.emit(start_arg)

        def kill_window(*args):
            win.close_app(force=True)
            QApplication.quit()

        signal.signal(signal.SIGINT, kill_window)
        # QTimer wakes up the event loop periodically which allows us to close
        # the window even when it is in background.
        timer = QTimer()
        timer.start(400)
        timer.timeout.connect(lambda: None)
        if lang_key:
            event_bus.send("gui.config.changed", gui_language=lang_key)

        return app.exec_()
