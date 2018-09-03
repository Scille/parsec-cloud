from PyQt5.QtCore import QTranslator, QLibraryInfo, QLocale, QFile
from PyQt5.QtWidgets import QApplication

from .main_window import MainWindow


def run_gui(parsec_core, trio_portal, cancel_scope):
    print("Starting UI")

    app = QApplication([])

    translator = QTranslator()
    translator.load(':/translations/parsec_{}'.format(QLocale.system().name()[:2]))
    app.installTranslator(translator)

    win = MainWindow(parsec_core, trio_portal, cancel_scope)
    win.show()

    return app.exec_()
