from PyQt5.QtCore import pyqtSignal, QCoreApplication
from PyQt5.QtWidgets import QToolButton, QMessageBox


def show_info(parent, text):
    QMessageBox.info(parent, QCoreApplication.translate("messages", "Info"), text)


def show_warning(parent, text):
    QMessageBox.warning(parent, QCoreApplication.translate("messages", "Warning"), text)


def show_error(parent, text):
    QMessageBox.error(parent, QCoreApplication.translate("messages", "Error"), text)


class ToolButton(QToolButton):
    clicked_name = pyqtSignal(str)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.clicked.connect(self.emit_clicked)

    def emit_clicked(self):
        self.clicked_name.emit(self.text())
