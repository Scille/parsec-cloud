# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import pyqtSignal, Qt, QPoint
from PyQt5.QtWidgets import QDialog, QToolTip

from parsec.api.protocol import UserID
from parsec.core.invite_claim import (
    InviteClaimTimeoutError,
    InviteClaimBackendOfflineError,
    InviteClaimError,
    generate_invitation_token as core_generate_invitation_token,
    invite_and_create_user as core_invite_and_create_user,
)
from parsec.core.types import BackendOrganizationAddr, BackendOrganizationClaimUserAddr

from parsec.core.backend_connection import BackendNotAvailable, BackendConnectionError
from parsec.core.gui import desktop
from parsec.core.gui import validators
from parsec.core.gui.custom_dialogs import show_info, show_warning, show_error
from parsec.core.gui.lang import translate as _
from parsec.core.gui.ui.register_user_dialog import Ui_RegisterUserDialog
from parsec.core.gui.trio_thread import JobResultError, ThreadSafeQtSignal


async def _do_registration(core, device, new_user_id, token, is_admin):
    try:
        new_user_id = UserID(new_user_id)
    except ValueError as exc:
        raise JobResultError("registration-invite-bad-value") from exc

    try:
        rep = await core.user_fs.backend_cmds.user_find(new_user_id)
    except BackendNotAvailable as exc:
        raise JobResultError("registration-invite-offline") from exc
    except BackendConnectionError as exc:
        raise JobResultError("registration-invite-error", info=str(exc)) from exc

    if rep["status"] != "ok":
        raise JobResultError("registration-invite-error", info=str(rep))

    for u in rep["results"]:
        if u == new_user_id:
            raise JobResultError("registration-invite-already-exists")

    try:
        await core_invite_and_create_user(
            device=device,
            user_id=new_user_id,
            token=token,
            is_admin=is_admin,
            keepalive=core.config.backend_connection_keepalive,
        )
    except InviteClaimTimeoutError:
        raise JobResultError("registration-invite-timeout")
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
            QToolTip.showText(button.mapToGlobal(QPoint(0, 0)), _("TOOLTIP_COPIED_TO_CLIPBOARD"))

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

        if not self.registration_job:
            return
        assert self.registration_job.is_finished()
        if self.registration_job.status == "cancelled":
            self.registration_job = None
            return
        assert self.registration_job.status != "ok"

        status = self.registration_job.status
        if status == "registration-invite-bad-value":
            errmsg = _("ERR_BAD_USER_NAME")
        elif status == "registration-invite-already-exists":
            errmsg = _("ERR_REGISTER_USER_EXISTS")
        elif status == "registration-invite-error":
            errmsg = _("ERR_REGISTER_WRONG_PARAMETERS")
        elif status == "registration-invite-offline":
            errmsg = _("ERR_REGISTER_USER_OFFLINE")
        elif status == "registration-invite-timeout":
            errmsg = _("ERR_REGISTER_USER_TIMEOUT")
        else:
            errmsg = _("ERR_REGISTER_USER_UNKNOWN")
        show_error(self, errmsg, exception=self.registration_job.exc)
        self.registration_job = None

    def on_registration_success(self):
        assert self.registration_job.is_finished()
        assert self.registration_job.status == "ok"
        show_info(self, _("INFO_REGISTER_USER_SUCCESS"))
        new_user_id, token = self.registration_job.ret
        self.registration_job = None
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

    def register_user(self):
        if not self.line_edit_username.text():
            show_warning(self, _("WARN_REGISTER_USER_EMPTY"))
            return

        try:
            new_user_id = UserID(self.line_edit_username.text())
        except ValueError as exc:
            show_error(self, _("ERR_BAD_USER_NAME"), exception=exc)
            return

        token = core_generate_invitation_token()
        try:
            addr = BackendOrganizationClaimUserAddr.build(
                self.core.device.organization_addr, user_id=new_user_id
            )
        except ValueError as exc:
            show_error(self, _("ERR_REGISTER_WRONG_PARAMETERS"), exception=exc)
            return

        token = core_generate_invitation_token()
        self.line_edit_user.setText(new_user_id)
        self.line_edit_user.setCursorPosition(0)
        self.line_edit_token.setText(token)
        self.line_edit_token.setCursorPosition(0)
        self.line_edit_url.setText(addr.to_url())
        self.line_edit_url.setCursorPosition(0)
        self.button_cancel.setFocus()
        self.widget_registration.show()
        self.registration_job = self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "registration_success"),
            ThreadSafeQtSignal(self, "registration_error"),
            _do_registration,
            core=self.core,
            device=self.core.device,
            new_user_id=new_user_id,
            token=token,
            is_admin=self.checkbox_is_admin.isChecked(),
        )
        self.button_cancel.show()
        self.line_edit_username.hide()
        self.checkbox_is_admin.hide()
        self.button_register.hide()
        self.button_close.hide()
