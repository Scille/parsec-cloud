# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtGui import QValidator
from PyQt5.QtWidgets import QLineEdit, QWidget

from parsec.core.gui.validators import trim_string


# Qt's validators suck. They only allow characters to be entered
# if they pass the validation, which may be confusing to a user.
# Instead we want to allow any character, but show the user
# that what they entered is invalid (by changing the field's background
# color for example). This is done by setting a dynamic property
# and using the state of this property in CSS.
class ValidatedLineEdit(QLineEdit):
    validity_changed = pyqtSignal(QValidator.State)

    def __init__(self, parent: QWidget | None = None) -> None:
        super().__init__(parent)
        self._validator: QValidator | None = None
        self.textChanged.connect(self._on_text_edited)

    # Override of Qt just to avoid mistakes
    def setValidator(self, validator: QValidator) -> None:
        self._validator = validator

    def set_validator(self, validator: QValidator) -> None:
        self._validator = validator

    def clean_text(self) -> str:
        return trim_string(self.text())

    def _on_text_edited(self, text: str) -> None:
        if self._validator:
            r, _, _ = self._validator.validate(text, 0)
            self.setProperty("validity", r)
            self.validity_changed.emit(r)
            self.style().polish(self)

    def is_input_valid(self) -> bool:
        if not self._validator:
            return True
        r, _, _ = self._validator.validate(self.text(), 0)
        return r == QValidator.Acceptable
