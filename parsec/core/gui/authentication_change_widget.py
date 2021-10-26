# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from PyQt5.QtWidgets import QWidget, QLineEdit
from PyQt5.QtCore import pyqtSignal

from structlog import get_logger

from parsec.core.local_device import (
    get_key_file,
    load_device_with_password,
    load_device_with_smartcard,
    save_device_with_password,
    save_device_with_smartcard,
    DeviceFileType,
)

from parsec.core.gui.custom_dialogs import GreyedDialog  # show_error, show_info, GreyedDialog
from parsec.core.gui.lang import translate as _

from parsec.core.gui.ui.authentication_change_widget import Ui_AuthenticationChangeWidget
from parsec.core.gui.ui.authentication_change_page1_widget import Ui_AuthenticationChangePage1Widget
from parsec.core.gui.ui.authentication_change_page2_widget import Ui_AuthenticationChangePage2Widget


logger = get_logger()


class AuthenticationChangePage2Widget(QWidget, Ui_AuthenticationChangePage2Widget):
    info_filled = pyqtSignal(bool)

    def __init__(self, device):
        super().__init__()
        self.setupUi(self)
        self.widget_auth.authentication_state_changed.connect(self.check_infos)
        self.widget_auth.excluded_strings = [
            device.organization_id,
            device.user_display,
            device.short_user_display,
            device.device_display,
        ]
        self.widget_auth.authentication_state_changed.connect(
            lambda _, valid: self.info_filled.emit(valid)
        )

    def set_excluded_strings(self, excluded_strings):
        self.widget_auth.excluded_strings = excluded_strings


class AuthenticationChangePage1Widget(QWidget, Ui_AuthenticationChangePage1Widget):
    info_filled = pyqtSignal(bool)

    def __init__(self):
        super().__init__()
        self.setupUi(self)
        self.widget = None
        if True:
            self.widget = QLineEdit()
            self.widget.setEchoMode(QLineEdit.Password)
            self.widget.textChanged.connect(self._on_password_changed)
            self.layout_old_auth.insertWidget(0, self.widget)

    def _on_password_changed(self, text):
        self.info_filled.emit(bool(len(text)))

    def get_current_authentication(self):
        if True:  # current_auth == PASSWORD
            return self.widget.text()


class AuthenticationChangeWidget(QWidget, Ui_AuthenticationChangeWidget):
    def __init__(self, core, jobs_ctx):
        super().__init__()
        self.setupUi(self)
        self.core = core
        self.jobs_ctx = jobs_ctx
        self.dialog = None
        self.button_validate.clicked.connect(self._on_validate_clicked)
        self.current_page = AuthenticationChangePage1Widget()
        self.current_page.info_filled.connect(self._on_info_filled)
        self.main_layout.addWidget(self.current_page)
        self.button_validate.setEnabled(False)

    def _on_info_filled(self, valid):
        self.button_validate.setEnabled(valid)

    def _on_validate_clicked(self):
        if isinstance(self.current_page, AuthenticationChangePage1Widget):
            if True:  # if device.auth_type == PASSWORD
                _ = get_key_file(self.core.config.config_dir, self.core.device)
                # load device
        else:
            pass

    @classmethod
    def show_modal(cls, core, jobs_ctx, parent, on_finished=None):
        w = cls(core=core, jobs_ctx=jobs_ctx)
        d = GreyedDialog(w, title=_("TEXT_CHANGE_PASSWORD_TITLE"), parent=parent)
        w.dialog = d

        if on_finished:
            d.finished.connect(on_finished)
        # Unlike exec_, show is asynchronous and works within the main Qt loop
        d.show()
        return w


"""  def change_password(self):
        key_file = get_key_file(self.core.config.config_dir, self.core.device)
        try:
            change_device_password(
                key_file, self.line_edit_old_password.text(), self.widget_new_auth.get_auth()
            )
            show_info(self, _("TEXT_CHANGE_PASSWORD_SUCCESS"))
            if self.dialog:
                self.dialog.accept()
            elif QApplication.activeModalWidget():
                QApplication.activeModalWidget().accept()
            else:
                logger.warning("Cannot close dialog when changing password info")
        except LocalDeviceCryptoError as exc:
            show_error(self, _("TEXT_CHANGE_PASSWORD_INVALID_PASSWORD"), exception=exc) """
