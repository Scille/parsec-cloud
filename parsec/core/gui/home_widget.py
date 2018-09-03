from PyQt5.QtWidgets import QWidget

from parsec.core.gui.ui.home_widget import Ui_HomeWidget


class HomeWidget(QWidget, Ui_HomeWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.setupUi(self)
