# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import QCoreApplication, pyqtSignal
from PyQt5.QtWidgets import QDialog

from parsec.core.invite_claim import (
    InviteClaimError,
    generate_invitation_token as core_generate_invitation_token,
    invite_and_create_user as core_invite_and_create_user,
)
from parsec.core.backend_connection.exceptions import BackendNotAvailable
from parsec.types import BackendOrganizationAddr, UserID
from parsec.core.gui import desktop
from parsec.core.gui import validators
from parsec.core.gui.custom_widgets import show_warning, show_info, show_error
from parsec.core.gui.lang import translate as _
from parsec.core.gui.ui.register_user_dialog import Ui_RegisterUserDialog
from parsec.core.gui.trio_thread import JobResultError, ThreadSafeQtSignal


STATUS_TO_ERRMSG = {
    "registration-invite-error": _("Only admins can invite a new user."),
    "timeout": _("User took too much time to register."),
}

DEFAULT_ERRMSG = _("Can not register this user ({info}).")


async def _do_registration(device, new_user_id, token, is_admin):
    try:
        await core_invite_and_create_user(device, new_user_id, token, is_admin)
    except InviteClaimError as exc:
        raise JobResultError("registration-invite-error", info=str(exc))


class RegisterUserDialog(QDialog, Ui_RegisterUserDialog):
    user_registered = pyqtSignal(BackendOrganizationAddr, UserID, str)
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
        self.button_register.clicked.connect(self.register_user)
        self.button_cancel.clicked.connect(self.cancel_registration)
        self.button_copy_username.clicked.connect(self.copy_field(self.line_edit_user))
        self.button_copy_token.clicked.connect(self.copy_field(self.line_edit_token))
        self.button_copy_url.clicked.connect(self.copy_field(self.line_edit_url))
        self.registration_success.connect(self.on_registration_success)
        self.registration_error.connect(self.on_registration_error)
        self.line_edit_username.setValidator(validators.UserIDValidator())
        self.closing_allowed = True

    def copy_field(self, widget):
        def _inner_copy_field():
            desktop.copy_to_clipboard(widget.text())

        return _inner_copy_field

    def on_registration_error(self):
        self.line_edit_token.setText("")
        self.line_edit_url.setText("")
        self.line_edit_user.setText("")
        self.widget_registration.hide()
        self.button_cancel.hide()
        self.checkbox_is_admin.show()
        self.button_register.show()
        self.line_edit_username.show()
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
        show_info(
            self,
            QCoreApplication.translate(
                "RegisterUserDialog", "User has been registered. You may now close this window."
            ),
        )
        self.registration_job = None
        self.user_registered.emit(
            BackendOrganizationAddr(self.line_edit_url.text()),
            UserID(self.line_edit_user.text()),
            self.line_edit_token.text(),
        )
        self.line_edit_token.setText("")
        self.line_edit_url.setText("")
        self.line_edit_user.setText("")
        self.widget_registration.hide()
        self.button_cancel.hide()
        self.button_register.show()
        self.line_edit_username.show()
        self.checkbox_is_admin.show()
        self.closing_allowed = True

    def cancel_registration(self):
        if self.registration_job:
            self.registration_job.cancel_and_join()
            self.registration_job = None
        self.line_edit_token.setText("")
        self.line_edit_url.setText("")
        self.line_edit_user.setText("")
        self.widget_registration.hide()
        self.button_cancel.hide()
        self.button_register.show()
        self.line_edit_username.show()
        self.checkbox_is_admin.show()
        self.closing_allowed = True

    def closeEvent(self, event):
        if not self.closing_allowed:
            show_warning(
                self,
                _(
                    "Can not close this window while waiting for the new user to register. "
                    "Please cancel first."
                ),
            )
            event.ignore()
        else:
            event.accept()

    def register_user(self):
        if not self.line_edit_username.text():
            show_warning(self, _("Please enter a username."))
            return

        try:
            users = self.portal.run(
                self.core.fs.backend_cmds.user_find, self.line_edit_username.text()
            )
        except BackendNotAvailable as exc:
            show_error(
                self,
                QCoreApplication.translate(
                    "RegisterUserDialog", f"Can not invite this user ({str(exc)})."
                ),
            )
            self.registration_error.emit()
            return

        if len(users):
            show_warning(self, _("A user with the same name already exists."))
            return
        try:
            token = core_generate_invitation_token()
            self.line_edit_user.setText(self.line_edit_username.text())
            self.line_edit_user.setCursorPosition(0)
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
                new_user_id=self.line_edit_username.text(),
                token=token,
                is_admin=self.checkbox_is_admin.isChecked(),
            )
            self.button_cancel.show()
            self.line_edit_username.hide()
            self.checkbox_is_admin.hide()
            self.button_register.hide()
            self.closing_allowed = False
        except:
            show_warning(self, _("Could not register the user."))
