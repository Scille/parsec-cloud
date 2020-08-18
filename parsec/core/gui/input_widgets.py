# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QLineEdit
from PyQt5.QtGui import QValidator


# Qt's validators suck. They only allow characters to be entered
# if they pass the validation, which may be confusing to a user.
# Instead we want to allow any character, but show the user
# that what they entered is invalid (by changing the field's background
# color for example). This is done by setting a dynamic property
# and using the state of this property in CSS.
class ValidatedLineEdit(QLineEdit):
    validity_changed = pyqtSignal(bool)

    def __init__(self, parent=None):
        super().__init__(parent)
        self._validator = None
        self.textEdited.connect(self._on_text_edited)

    # Override of Qt just to avoid mistakes
    def setValidator(self, validator):
        self._validator = validator

    def set_validator(self, validator):
        self._validator = validator

    def _on_text_edited(self, text):
        if self._validator:
            r, _, _ = self._validator.validate(text, 0)
            if r == QValidator.Invalid:
                self.setProperty("valid", False)
                self.validity_changed.emit(False)
            else:
                self.setProperty("valid", True)
                self.validity_changed.emit(True)
            self.style().polish(self)

    def is_input_valid(self):
        if not self._validator:
            return True
        r, _, _ = self._validator.validate(self.text(), 0)
        if r == QValidator.Invalid:
            return False
        return True
