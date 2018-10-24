import os
import pathlib
import signal

from PyQt5.QtCore import QTimer
from PyQt5.QtWidgets import QApplication

from parsec.core.gui import lang
from parsec.core.gui import settings
from parsec.core.gui.core_call import init_core_call
from parsec.core.gui.main_window import MainWindow


def kill_window(window):
    def _inner_kill_window(*args):
        window.force_close = True
        window.close_app()
        QApplication.quit()

    return _inner_kill_window


def run_gui(parsec_core, trio_portal, cancel_scope):
    print("Starting UI")

    app = QApplication([])
    app.setOrganizationName("Scille")
    app.setOrganizationDomain("parsec.cloud")
    app.setApplicationName("Parsec")

    if not settings.get_value("mountpoint", None):
        settings.set_value("mountpoint", os.path.join(str(pathlib.Path.home()), "parsec"))

    # splash = QSplashScreen(QPixmap(':/logos/images/logos/parsec.png'))
    # splash.show()
    # app.processEvents()

    init_core_call(parsec_core=parsec_core, trio_portal=trio_portal, cancel_scope=cancel_scope)

    lang.switch_to_locale()

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
