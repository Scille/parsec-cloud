from PyQt5.QtCore import Qt
from PyQt5.QtWidgets import QDialog

from parsec.core.gui.ui.loading_dialog import Ui_LoadingDialog


class LoadingDialog(QDialog, Ui_LoadingDialog):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.setWindowFlags(Qt.SplashScreen)
