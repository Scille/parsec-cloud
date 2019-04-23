# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QDialog

from parsec.core.invite_claim import (
    InviteClaimBackendOfflineError,
    InviteClaimError,
    generate_invitation_token as core_generate_invitation_token,
    invite_and_create_device as core_invite_and_create_device,
)
from parsec.types import BackendOrganizationAddr, DeviceName
from parsec.core.gui import desktop
from parsec.core.gui import validators
from parsec.core.gui.custom_widgets import show_warning, show_info, show_error
from parsec.core.gui.lang import translate as _
from parsec.core.gui.ui.register_device_dialog import Ui_RegisterDeviceDialog
from parsec.core.gui.trio_thread import JobResultError, ThreadSafeQtSignal


STATUS_TO_ERRMSG = {
    "registration-invite-bad-value": _("Bad device name."),
    "already_exists": _("This device already exists."),
    "registration-invite-offline": _("Cannot invite a device without being online."),
    "timeout": _("Device took too much time to register."),
}

DEFAULT_ERRMSG = _("Cannot register this device ({info}).")


async def _do_registration(device, new_device_name, token):
    try:
        new_device_name = DeviceName(new_device_name)
    except ValueError:
        raise JobResultError("registration-invite-bad-value")

    try:
        await core_invite_and_create_device(device, new_device_name, token)
    except InviteClaimBackendOfflineError:
        raise JobResultError("registration-invite-offline")
    except InviteClaimError as exc:
        raise JobResultError("registration-invite-error", info=str(exc))

    return new_device_name, token


class RegisterDeviceDialog(QDialog, Ui_RegisterDeviceDialog):
    device_registered = pyqtSignal(BackendOrganizationAddr, DeviceName, str)
    registration_success = pyqtSignal()
    registration_error = pyqtSignal()

    def __init__(self, portal, core, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.core = core
        self.portal = portal
        self.registration_job = None
        self.widget_registration.hide()
        self.button_cancel.hide()
        self.button_register.clicked.connect(self.register_device)
        self.button_cancel.clicked.connect(self.cancel_registration)
        self.button_copy_device.clicked.connect(self.copy_field(self.line_edit_device))
        self.button_copy_token.clicked.connect(self.copy_field(self.line_edit_token))
        self.button_copy_url.clicked.connect(self.copy_field(self.line_edit_url))
        self.registration_success.connect(self.on_registration_success)
        self.registration_error.connect(self.on_registration_error)
        self.line_edit_device_name.setValidator(validators.DeviceNameValidator())
        self.closing_allowed = True

    def copy_field(self, widget):
        def _inner_copy_field():
            desktop.copy_to_clipboard(widget.text())

        return _inner_copy_field

    def on_registration_error(self):
        self.line_edit_token.setText("")
        self.line_edit_url.setText("")
        self.line_edit_device.setText("")
        self.widget_registration.hide()
        self.button_cancel.hide()
        self.button_register.show()
        self.line_edit_device_name.show()
        self.closing_allowed = True

        if self.registration_job:
            assert self.registration_job.is_finished()
            if self.registration_job.status == "cancelled":
                return
            assert self.registration_job.status != "ok"
            errmsg = STATUS_TO_ERRMSG.get(self.registration_job.status, DEFAULT_ERRMSG)
            show_error(self, errmsg.format(**self.registration_job.exc.params))

    def on_registration_success(self):
        assert self.registration_job.is_finished()
        assert self.registration_job.status == "ok"
        show_info(self, _("Device has been registered. You may now close this window."))
        new_device_name, token = self.registration_job.ret
        self.device_registered.emit(self.core.device.organization_addr, new_device_name, token)
        self.registration_job = None
        self.line_edit_token.setText("")
        self.line_edit_url.setText("")
        self.line_edit_device.setText("")
        self.widget_registration.hide()
        self.button_cancel.hide()
        self.button_register.show()
        self.line_edit_device_name.show()
        self.closing_allowed = True

    def cancel_registration(self):
        if self.registration_job:
            self.registration_job.cancel_and_join()
            self.registration_job = None
        self.line_edit_token.setText("")
        self.line_edit_url.setText("")
        self.line_edit_device.setText("")
        self.widget_registration.hide()
        self.button_cancel.hide()
        self.button_register.show()
        self.line_edit_device_name.show()
        self.closing_allowed = True

    def closeEvent(self, event):
        if not self.closing_allowed:
            show_warning(
                self,
                _(
                    "Cannot close this window while waiting for the new device to register. "
                    "Please cancel first."
                ),
            )
            event.ignore()
        else:
            event.accept()

    def register_device(self):
        if not self.line_edit_device_name.text():
            show_warning(self, _("Please enter a device name."))
            return

        token = core_generate_invitation_token()
        self.line_edit_device.setText(self.line_edit_device_name.text())
        self.line_edit_device.setCursorPosition(0)
        self.line_edit_token.setText(token)
        self.line_edit_token.setCursorPosition(0)
        self.line_edit_url.setText(self.core.device.organization_addr)
        self.line_edit_url.setCursorPosition(0)
        self.button_cancel.setFocus()
        self.widget_registration.show()
        self.registration_job = self.portal.submit_job(
            ThreadSafeQtSignal(self, "registration_success"),
            ThreadSafeQtSignal(self, "registration_error"),
            _do_registration,
            device=self.core.device,
            new_device_name=self.line_edit_device_name.text(),
            token=token,
        )
        self.button_cancel.show()
        self.line_edit_device_name.hide()
        self.button_register.hide()
        self.closing_allowed = False
