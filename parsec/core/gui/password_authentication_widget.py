# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import List

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtGui import QValidator
from PyQt5.QtWidgets import QWidget

from parsec.core.gui.lang import translate as _
from parsec.core.gui.password_validation import PasswordStrengthWidget
from parsec.core.gui.ui.password_authentication_widget import Ui_PasswordAuthenticationWidget


class PasswordAuthenticationWidget(QWidget, Ui_PasswordAuthenticationWidget):
    authentication_state_changed = pyqtSignal(bool)

    def __init__(self, parent: QWidget | None = None) -> None:
        super().__init__(parent)
        self.setupUi(self)
        self.pwd_str_widget = PasswordStrengthWidget()
        self.layout_password_strength.addWidget(self.pwd_str_widget)
        self.line_edit_password.textChanged.connect(self.pwd_str_widget.on_password_change)
        self.line_edit_password.textChanged.connect(self._check_match)
        self.line_edit_password_check.textChanged.connect(self._check_match)
        self.label_mismatch.hide()

    def set_excluded_strings(self, excluded_strings: List[str]) -> None:
        self.pwd_str_widget.set_excluded_strings(excluded_strings)

    def _check_match(self, text: str = "") -> None:
        password = self.line_edit_password.text()
        password_check = self.line_edit_password_check.text()
        if (
            password
            and password_check
            and password == password_check
            and self.pwd_str_widget.get_password_strength(password) > 0
        ):
            self.line_edit_password_check.setProperty("validity", QValidator.Acceptable)
            self.line_edit_password_check.setToolTip(_("TEXT_PASSWORD_CHECK_MATCH"))
            self.line_edit_password_check.style().polish(self.line_edit_password_check)
            self.label_mismatch.hide()
            self.authentication_state_changed.emit(True)
        else:
            if password and password_check and password != password_check:
                self.line_edit_password_check.setProperty("validity", QValidator.Intermediate)
                self.line_edit_password_check.setToolTip(_("TEXT_PASSWORD_CHECK_NO_MATCH"))
                self.line_edit_password_check.style().polish(self.line_edit_password_check)
                self.label_mismatch.show()
            else:
                self.line_edit_password_check.setProperty("validity", QValidator.Acceptable)
                self.line_edit_password_check.setToolTip(_("TEXT_PASSWORD_CHECK_MATCH"))
                self.line_edit_password_check.style().polish(self.line_edit_password_check)
                self.label_mismatch.hide()
            self.authentication_state_changed.emit(False)

    def is_auth_valid(self) -> bool:
        password = self.line_edit_password.text()
        password_check = self.line_edit_password_check.text()
        return bool(
            password
            and password_check
            and password == password_check
            and self.pwd_str_widget.get_password_strength(password) > 0
        )

    @property
    def password(self) -> str:
        return self.line_edit_password.text()
