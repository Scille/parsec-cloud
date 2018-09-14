from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QToolButton


class ToolButton(QToolButton):
    clicked_name = pyqtSignal(str)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.clicked.connect(self.emit_clicked)

    def emit_clicked(self):
        self.clicked_name.emit(self.text())
