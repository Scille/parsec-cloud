# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget

from parsec.core.gui.ui.smartcard_authentication_widget import Ui_SmartcardAuthenticationWidget


class SmartCardAuthenticationWidget(QWidget, Ui_SmartcardAuthenticationWidget):
    authentication_state_changed = pyqtSignal(bool)

    def __init__(self, parent=None):
        super().__init__(parent)
        self.setupUi(self)

    def is_auth_valid(self):
        return True
