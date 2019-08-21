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
    from parsec.core.gui.lang import translate as _
    from parsec.core.gui.new_version import CheckNewVersion
    from parsec.core.gui.systray import systray_available, Systray
    from parsec.core.gui.main_window import MainWindow
    from parsec.core.gui.trio_thread import run_trio_thread
    from parsec.core.gui.custom_dialogs import show_error
    from parsec.core.gui import desktop
    from parsec.core.gui import win_registry
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


def run_gui(config: CoreConfig):
    logger.info("Starting UI")

    app = QApplication([])
    app.setOrganizationName("Scille")
    app.setOrganizationDomain("parsec.cloud")
    app.setApplicationName("Parsec")

    f = QFont("Arial")
    app.setFont(f)

    lang.switch_language(config)

    if not config.gui_allow_multiple_instances and desktop.parsec_instances_count() > 1:
        show_error(None, _("PARSEC_ALREADY_RUNNING"))
        return

    event_bus = EventBus()
    with run_trio_thread() as jobs_ctx:
        systray = None
        if systray_available() and config.gui_tray_enabled:
            win = MainWindow(
                jobs_ctx=jobs_ctx, event_bus=event_bus, config=config, minimize_on_close=True
            )
            systray = Systray(parent=win)
            systray.on_close.connect(win.close_app)
            systray.on_show.connect(win.show_top)
            app.aboutToQuit.connect(before_quit(systray))
        else:
            win = MainWindow(jobs_ctx=jobs_ctx, event_bus=event_bus, config=config)

        if config.gui_check_version_at_startup:
            CheckNewVersion(jobs_ctx=jobs_ctx, event_bus=event_bus, config=config, parent=win)

        win.showMaximized()

        if config.gui_windows_left_panel:
            win_registry.add_nav_link(config.mountpoint_base_dir)
        else:
            win_registry.remove_nav_link()

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
