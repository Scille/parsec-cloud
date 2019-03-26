# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import signal
from structlog import get_logger

from PyQt5.QtCore import QTimer
from PyQt5.QtGui import QFont
from PyQt5.QtWidgets import QApplication

from parsec.core.config import CoreConfig
from parsec.event_bus import EventBus

try:
    from parsec.core.gui import lang
    from parsec.core.gui.new_version import CheckNewVersion
    from parsec.core.gui.systray import systray_available, Systray
    from parsec.core.gui.main_window import MainWindow
    from parsec.core.gui.trio_thread import run_trio_thread
except ImportError as exc:
    raise ModuleNotFoundError(
        """PyQt forms haven't been generated.
You must install the parsec package or run `python setup.py generate_pyqt_forms`
"""
    ) from exc


logger = get_logger()


def run_gui(config: CoreConfig):
    logger.info("Starting UI")

    app = QApplication([])
    app.setOrganizationName("Scille")
    app.setOrganizationDomain("parsec.cloud")
    app.setApplicationName("Parsec")

    f = QFont("Arial")
    app.setFont(f)

    lang.switch_language(config)

    event_bus = EventBus()
    with run_trio_thread() as jobs_ctx:

        if systray_available() and config.gui_tray_enabled:
            win = MainWindow(
                jobs_ctx=jobs_ctx, event_bus=event_bus, config=config, minimize_on_close=True
            )
            systray = Systray(parent=win)
            systray.on_close.connect(win.close_app)
            systray.on_show.connect(win.show_top)

        else:
            win = MainWindow(jobs_ctx=jobs_ctx, event_bus=event_bus, config=config)

        if config.gui_check_version_at_startup:
            CheckNewVersion(jobs_ctx=jobs_ctx, event_bus=event_bus, config=config, parent=win)

        win.showMaximized()

        def kill_window(*args):
            win.close_app(force=True)
            QApplication.quit()

        signal.signal(signal.SIGINT, kill_window)
        # QTimer wakes up the event loop periodically which allows us to close
        # the window even when it is in background.
        timer = QTimer()
        timer.start(400)
        timer.timeout.connect(lambda: None)

        return app.exec_()
