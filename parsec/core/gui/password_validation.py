# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import re
from typing import List

from PyQt5.QtWidgets import QWidget
from zxcvbn import zxcvbn

from parsec.core.gui.lang import translate as _
from parsec.core.gui.ui.password_strength_widget import Ui_PasswordStrengthWidget


PASSWORD_CSS = {
    0: "color: #333333; background-color: none;",
    1: "color: #F44336; background-color: none;",
    2: "color: #F44336; background-color: none;",
    3: "color: #F1962B; background-color: none;",
    4: "color: #8BC34A; background-color: none;",
    5: "color: #8BC34A; background-color: none;",
}


def get_password_strength_text(password_score: int) -> str:
    PASSWORD_STRENGTH_TEXTS = [
        _("TEXT_PASSWORD_VALIDATION_TOO_SHORT"),
        _("TEXT_PASSWORD_VALIDATION_VERY_WEAK"),
        _("TEXT_PASSWORD_VALIDATION_WEAK"),
        _("TEXT_PASSWORD_VALIDATION_AVERAGE"),
        _("TEXT_PASSWORD_VALIDATION_GOOD"),
        _("TEXT_PASSWORD_VALIDATION_EXCELLENT"),
    ]
    return PASSWORD_STRENGTH_TEXTS[password_score]


def get_password_strength(password: str, excluded_strings: list[str] | None = None) -> int:
    if len(password) < 8:
        return 0
    result = zxcvbn(password, user_inputs=excluded_strings)
    return result["score"] + 1


class PasswordStrengthWidget(QWidget, Ui_PasswordStrengthWidget):
    def __init__(self, parent: QWidget | None = None) -> None:
        super().__init__(parent)
        self.setupUi(self)
        self._excluded_strings: list[str] = []

    def set_excluded_strings(self, excluded_strings: List[str]) -> None:
        self._excluded_strings = []
        # User inputs are passed raw, we have to clean them up a bit
        # We split them up on the most common characters
        for exc_str in excluded_strings:
            self._excluded_strings += [x for x in re.split(r"\W+", exc_str) if x and len(x) > 3]

    def on_password_change(self, text: str | None) -> None:
        if not text:
            self.hide()
            return
        score = get_password_strength(text, self._excluded_strings)
        self.label.setText(
            _("TEXT_PASSWORD_VALIDATION_PASSWORD_STRENGTH_strength").format(
                strength=get_password_strength_text(score)
            )
        )
        self.label.setStyleSheet(PASSWORD_CSS[score])
        self.show()

    def get_password_strength(self, password: str) -> int:
        return get_password_strength(password, self._excluded_strings)
