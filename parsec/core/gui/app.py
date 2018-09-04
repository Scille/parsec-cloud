from PyQt5.QtCore import QSettings
from PyQt5.QtWidgets import QApplication

from parsec.core.gui import lang
from parsec.core.gui.core_call import init_core_call
from parsec.core.gui.main_window import MainWindow


def run_gui(parsec_core, trio_portal, cancel_scope):
    print("Starting UI")

    app = QApplication([])
    app.setOrganizationName('Scille')
    app.setOrganizationDomain('parsec.cloud')
    app.setApplicationName('Parsec')

    init_core_call(parsec_core=parsec_core, trio_portal=trio_portal,
                   cancel_scope=cancel_scope)

    lang.switch_to_locale()

    win = MainWindow()
    win.show()

    return app.exec_()
