# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import QCoreApplication, pyqtSignal, Qt, QPoint
from PyQt5.QtWidgets import QDialog, QToolTip

from parsec.core.invite_claim import (
    InviteClaimBackendOfflineError,
    InviteClaimError,
    generate_invitation_token as core_generate_invitation_token,
    invite_and_create_user as core_invite_and_create_user,
)
from parsec.types import BackendOrganizationAddr, UserID
from parsec.core.backend_connection import BackendNotAvailable, BackendConnectionError
from parsec.core.gui import desktop
from parsec.core.gui import validators
from parsec.core.gui.custom_dialogs import show_info, show_warning, show_error
from parsec.core.gui.lang import translate as _
from parsec.core.gui.ui.register_user_dialog import Ui_RegisterUserDialog
from parsec.core.gui.trio_thread import JobResultError, ThreadSafeQtSignal


STATUS_TO_ERRMSG = {
    "registration-invite-bad-value": _("Bad user id."),
    "registration-invite-already-exists": _("A user with the same name already exists."),
    "registration-invite-error": _("Only admins can invite a new user."),
    "registration-invite-offline": _("Cannot invite a user without being online."),
    "timeout": _("User took too much time to register."),
}

DEFAULT_ERRMSG = _("Cannot register this user ({info}).")


async def _do_registration(core, device, new_user_id, token, is_admin):
    try:
        new_user_id = UserID(new_user_id)
    except ValueError as exc:
        raise JobResultError("registration-invite-bad-value") from exc

    try:
        users = await core.user_fs.backend_cmds.user_find(new_user_id)
    except BackendNotAvailable as exc:
        raise JobResultError("registration-invite-offline") from exc
    except BackendConnectionError as exc:
        raise JobResultError("registration-invite-error", info=str(exc)) from exc
    for u in users:
        if u == new_user_id:
            raise JobResultError("registration-invite-already-exists")

    try:
        await core_invite_and_create_user(device, new_user_id, token, is_admin)
    except InviteClaimBackendOfflineError as exc:
        raise JobResultError("registration-invite-offline") from exc
    except InviteClaimError as exc:
        raise JobResultError("registration-invite-error", info=str(exc)) from exc

    return new_user_id, token


class RegisterUserDialog(QDialog, Ui_RegisterUserDialog):
    user_registered = pyqtSignal(BackendOrganizationAddr, UserID, str)
    registration_success = pyqtSignal()
    registration_error = pyqtSignal()

    def __init__(self, core, jobs_ctx, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.core = core
        self.jobs_ctx = jobs_ctx
        self.registration_job = None
        self.widget_registration.hide()
        self.button_cancel.hide()
        self.button_register.clicked.connect(self.register_user)
        self.button_cancel.clicked.connect(self.cancel_registration)
        self.button_copy_username.clicked.connect(
            self.copy_field(self.button_copy_username, self.line_edit_user)
        )
        self.button_copy_token.clicked.connect(
            self.copy_field(self.button_copy_token, self.line_edit_token)
        )
        self.button_copy_url.clicked.connect(
            self.copy_field(self.button_copy_url, self.line_edit_url)
        )
        self.registration_success.connect(self.on_registration_success)
        self.registration_error.connect(self.on_registration_error)
        self.line_edit_username.setValidator(validators.UserIDValidator())
        self.setWindowFlags(Qt.SplashScreen)
        self.adjust_size()

    def adjust_size(self):
        size = self.sizeHint()
        size.setWidth(400)
        self.resize(size)

    def copy_field(self, button, widget):
        def _inner_copy_field():
            desktop.copy_to_clipboard(widget.text())
            QToolTip.showText(button.mapToGlobal(QPoint(0, 0)), _("Copied to clipboard"))

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
        self.button_close.show()
        self.adjust_size()

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
        new_user_id, token = self.registration_job.ret
        self.user_registered.emit(self.core.device.organization_addr, new_user_id, token)
        self.registration_job = None
        self.line_edit_token.setText("")
        self.line_edit_url.setText("")
        self.line_edit_user.setText("")
        self.widget_registration.hide()
        self.button_cancel.hide()
        self.button_register.show()
        self.line_edit_username.show()
        self.checkbox_is_admin.show()
        self.button_close.show()
        self.adjust_size()

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
        self.button_close.show()
        self.adjust_size()

    def register_user(self):
        if not self.line_edit_username.text():
            show_warning(self, _("Please enter a username."))
            return

        token = core_generate_invitation_token()
        self.line_edit_user.setText(self.line_edit_username.text())
        self.line_edit_user.setCursorPosition(0)
        self.line_edit_token.setText(token)
        self.line_edit_token.setCursorPosition(0)
        self.line_edit_url.setText(self.core.device.organization_addr)
        self.line_edit_url.setCursorPosition(0)
        self.button_cancel.setFocus()
        self.widget_registration.show()
        self.registration_job = self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "registration_success"),
            ThreadSafeQtSignal(self, "registration_error"),
            _do_registration,
            core=self.core,
            device=self.core.device,
            new_user_id=self.line_edit_username.text(),
            token=token,
            is_admin=self.checkbox_is_admin.isChecked(),
        )
        self.button_cancel.show()
        self.line_edit_username.hide()
        self.checkbox_is_admin.hide()
        self.button_register.hide()
        self.button_close.hide()
