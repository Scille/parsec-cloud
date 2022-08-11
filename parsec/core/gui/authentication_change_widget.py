# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from PyQt5.QtWidgets import QWidget, QApplication

from structlog import get_logger

from parsec.core.local_device import (
    load_device_with_password,
    load_device_with_smartcard,
    save_device_with_password_in_config,
    save_device_with_smartcard_in_config,
    LocalDeviceError,
    LocalDeviceNotFoundError,
    LocalDeviceCryptoError,
    DeviceFileType,
    get_available_device,
)

from parsec.core.gui.custom_dialogs import GreyedDialog, get_text_input, show_error, show_info
from parsec.core.gui.lang import translate as _

from parsec.core.gui.ui.authentication_change_widget import Ui_AuthenticationChangeWidget


logger = get_logger()


class AuthenticationChangeWidget(QWidget, Ui_AuthenticationChangeWidget):
    def __init__(self, core, jobs_ctx, loaded_device):
        super().__init__()
        self.setupUi(self)
        self.core = core
        self.jobs_ctx = jobs_ctx
        self.dialog = None
        self.button_validate.clicked.connect(self._on_validate_clicked)
        self.widget_auth.authentication_state_changed.connect(self._on_info_filled)
        self.button_validate.setEnabled(False)
        self.loaded_device = loaded_device

    def _on_info_filled(self, auth_method, valid):
        self.button_validate.setEnabled(valid)

    async def _on_validate_clicked(self):
        self.button_validate.setEnabled(False)
        auth_method = self.widget_auth.get_auth_method()
        try:
            if auth_method == DeviceFileType.PASSWORD:
                save_device_with_password_in_config(
                    self.core.config.config_dir, self.loaded_device, self.widget_auth.get_auth()
                )
            elif auth_method == DeviceFileType.SMARTCARD:
                await save_device_with_smartcard_in_config(
                    self.core.config.config_dir, self.loaded_device
                )
            show_info(self, _("TEXT_AUTH_CHANGE_SUCCESS"))
            if self.dialog:
                self.dialog.accept()
            elif QApplication.activeModalWidget():
                QApplication.activeModalWidget().accept()
            else:
                logger.warning("Cannot close dialog when changing password info")
        except LocalDeviceCryptoError as exc:
            self.button_validate.setEnabled(True)
            if auth_method == DeviceFileType.SMARTCARD:
                show_error(self, _("TEXT_INVALID_SMARTCARD"), exception=exc)
        except LocalDeviceNotFoundError as exc:
            self.button_validate.setEnabled(True)
            if auth_method == DeviceFileType.PASSWORD:
                show_error(self, _("TEXT_CANNOT_SAVE_DEVICE"), exception=exc)
        except LocalDeviceError as exc:
            self.button_validate.setEnabled(True)
            show_error(self, _("TEXT_CANNOT_SAVE_DEVICE"), exception=exc)

    @classmethod
    async def show_modal(cls, core, jobs_ctx, parent, on_finished=None):
        available_device = get_available_device(core.config.config_dir, core.device)
        loaded_device = None

        try:
            if available_device.type == DeviceFileType.PASSWORD:
                password = get_text_input(
                    parent,
                    _("TEXT_DEVICE_UNLOCK_TITLE"),
                    _("TEXT_DEVICE_UNLOCK_FOR_AUTH_CHANGE_LABEL"),
                    placeholder="",
                    default_text="",
                    completion=None,
                    button_text=None,
                    validator=None,
                    hidden=True,
                )
                if not password:
                    return
                loaded_device = load_device_with_password(available_device.key_file_path, password)
            else:
                loaded_device = await load_device_with_smartcard(available_device.key_file_path)
        except LocalDeviceError:
            show_error(parent, _("TEXT_LOGIN_ERROR_AUTHENTICATION_FAILED"))
            return

        w = cls(core=core, jobs_ctx=jobs_ctx, loaded_device=loaded_device)
        d = GreyedDialog(w, title=_("TEXT_CHANGE_AUTHENTICATION_TITLE"), parent=parent)
        w.dialog = d

        if on_finished:
            d.finished.connect(on_finished)
        # Unlike exec_, show is asynchronous and works within the main Qt loop
        d.show()
        return w
