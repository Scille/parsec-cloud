# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import pyqtSignal, Qt
from PyQt5.QtWidgets import QDialog

from parsec.core.gui.ui.claim_dialog import Ui_ClaimDialog


class ClaimDialog(QDialog, Ui_ClaimDialog):
    cancel_clicked = pyqtSignal()

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.setWindowFlags(Qt.SplashScreen)
        self.button_cancel.clicked.connect(self.cancel_clicked.emit)

    def setText(self, text):
        self.label.setText(text)
