import os
import pathlib
import signal
from structlog import get_logger

from PyQt5.QtCore import QTimer
from PyQt5.QtWidgets import QApplication

from parsec.core.config import CoreConfig

try:
    from parsec.core.gui import lang
    from parsec.core.gui import settings
    from parsec.core.gui.core_call import init_core_call
    from parsec.core.gui.main_window import MainWindow
except ImportError as exc:
    raise ModuleNotFoundError(
        """PyQt forms haven't been generated.
You must install the parsec package or run `python setup.py generate_pyqt_forms`
"""
    ) from exc


logger = get_logger()


def kill_window(window):
    def _inner_kill_window(*args):
        window.force_close = True
        window.close_app()
        QApplication.quit()

    return _inner_kill_window


def run_gui(config: CoreConfig):
    # def run_gui(parsec_core, trio_portal, cancel_scope):
    logger.info("Starting UI")

    # init_core_call(parsec_core=parsec_core, trio_portal=trio_portal, cancel_scope=cancel_scope)

    app = QApplication([])
    app.setOrganizationName("Scille")
    app.setOrganizationDomain("parsec.cloud")
    app.setApplicationName("Parsec")

    if not settings.get_value("global/mountpoint", None):
        settings.set_value("global/mountpoint", os.path.join(str(pathlib.Path.home()), "parsec"))

    # splash = QSplashScreen(QPixmap(':/logos/images/logos/parsec.png'))
    # splash.show()
    # app.processEvents()

    lang.switch_language()

    win = MainWindow()

    # QTimer wakes up the event loop periodically which allows us to close
    # the window even when it is in background.
    signal.signal(signal.SIGINT, kill_window(win))
    timer = QTimer()
    timer.start(400)
    timer.timeout.connect(lambda: None)

    win.showMaximized()
    # splash.finish(win)

    return app.exec_()
