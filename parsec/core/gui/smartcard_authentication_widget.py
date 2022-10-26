# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations
from typing import Literal

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget

from parsec.core.gui.ui.smartcard_authentication_widget import Ui_SmartcardAuthenticationWidget


class SmartCardAuthenticationWidget(QWidget, Ui_SmartcardAuthenticationWidget):
    authentication_state_changed = pyqtSignal(bool)

    def __init__(self, parent: QWidget | None = None) -> None:
        super().__init__(parent)
        self.setupUi(self)

    def is_auth_valid(self) -> Literal[True]:
        return True
