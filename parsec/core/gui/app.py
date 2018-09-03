from PyQt5.QtWidgets import QApplication

from .main_window import MainWindow


def run_gui(parsec_core, trio_portal, cancel_scope):
    app = QApplication([])

    print("Starting UI")

    win = MainWindow(parsec_core, trio_portal, cancel_scope)
    win.show()

    return app.exec_()
