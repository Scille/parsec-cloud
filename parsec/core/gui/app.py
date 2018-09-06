from PyQt5.QtWidgets import QApplication

from parsec.core.gui import lang
from parsec.core.gui.core_call import init_core_call, core_call
from parsec.core.gui.main_window import MainWindow


def run_gui(parsec_core, trio_portal, cancel_scope):
    print("Starting UI")

    app = QApplication([])
    app.setOrganizationName('Scille')
    app.setOrganizationDomain('parsec.cloud')
    app.setApplicationName('Parsec')

    # splash = QSplashScreen(QPixmap(':/logos/images/logos/parsec.png'))
    # splash.show()
    # app.processEvents()

    init_core_call(parsec_core=parsec_core, trio_portal=trio_portal,
                   cancel_scope=cancel_scope)

    lang.switch_to_locale()

    win = MainWindow()
    win.show()
    # splash.finish(win)

    return app.exec_()
