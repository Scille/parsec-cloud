# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

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
