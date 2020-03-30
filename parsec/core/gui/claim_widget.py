# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget

from parsec.core.gui.ui.claim_widget import Ui_ClaimWidget


class ClaimWidget(QWidget, Ui_ClaimWidget):
    cancel_clicked = pyqtSignal()

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.button_cancel.clicked.connect(self.cancel_clicked.emit)

    def setText(self, text):
        self.label.setText(text)
