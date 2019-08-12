# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import Qt
from PyQt5.QtWidgets import QDialog

from parsec.core.local_device import get_key_file, change_device_password, LocalDeviceCryptoError

from parsec.core.gui.password_validation import (
    get_password_strength,
    get_password_strength_text,
    PASSWORD_CSS,
)
from parsec.core.gui.custom_dialogs import show_error, show_info
from parsec.core.gui.lang import translate as _

from parsec.core.gui.ui.password_change_dialog import Ui_PasswordChangeDialog


class PasswordChangeDialog(QDialog, Ui_PasswordChangeDialog):
    def __init__(self, core, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.core = core
        self.line_edit_old_password.textChanged.connect(self.check_infos)
        self.line_edit_password.textChanged.connect(self.password_changed)
        self.line_edit_password.textChanged.connect(self.check_infos)
        self.line_edit_password_check.textChanged.connect(self.check_infos)
        self.button_change.clicked.connect(self.change_password)
        self.button_change.setDisabled(True)
        self.setWindowFlags(Qt.SplashScreen)

    def check_infos(self):
        if (
            len(self.line_edit_old_password.text())
            and len(self.line_edit_password.text())
            and get_password_strength(self.line_edit_password.text()) > 0
            and len(self.line_edit_password_check.text())
        ):
            self.button_change.setDisabled(False)
        else:
            self.button_change.setDisabled(True)

    def password_changed(self, text):
        if text:
            self.label_password_strength.show()
            score = get_password_strength(text)
            self.label_password_strength.setText(
                _("LABEL_PASSWORD_STRENGTH_{}").format(get_password_strength_text(score))
            )
            self.label_password_strength.setStyleSheet(PASSWORD_CSS[score])
        else:
            self.label_password_strength.hide()

    def change_password(self):
        if self.line_edit_password.text() != self.line_edit_password_check.text():
            show_error(self, _("ERR_PASSWORD_MISMATCH"))
        else:
            key_file = get_key_file(self.core.config.config_dir, self.core.device)
            try:
                change_device_password(
                    key_file, self.line_edit_old_password.text(), self.line_edit_password.text()
                )
                show_info(self, _("INFO_CHANGE_PASSWORD_SUCCESS"))
                self.accept()
            except LocalDeviceCryptoError as exc:
                show_error(self, _("ERR_PASSWORD_CHANGE_INVALID_PASSWORD"), exception=exc)
