# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import pyqtSignal, Qt
from PyQt5.QtWidgets import QWidget

from parsec.api.protocol import DeviceID
from parsec.core.local_device import LocalDeviceAlreadyExistsError, save_device_with_password
from parsec.core.types import BackendOrganizationAddr
from parsec.core.invite_claim import (
    claim_device as core_claim_device,
    InviteClaimError,
    InviteClaimBackendOfflineError,
)
from parsec.core.gui import validators
from parsec.core.gui.trio_thread import JobResultError, ThreadSafeQtSignal
from parsec.core.gui.desktop import get_default_device
from parsec.core.gui.custom_dialogs import show_error, show_info
from parsec.core.gui.lang import translate as _
from parsec.core.gui.claim_dialog import ClaimDialog
from parsec.core.gui.password_validation import (
    get_password_strength,
    get_password_strength_text,
    PASSWORD_CSS,
)
from parsec.core.gui.ui.claim_device_widget import Ui_ClaimDeviceWidget


async def _do_claim_device(
    config_dir,
    password: str,
    password_check: str,
    token: str,
    user_id: str,
    device_name: str,
    organization_addr: str,
):
    if password != password_check:
        raise JobResultError("password-mismatch")
    if len(password) < 8:
        raise JobResultError("password-size")

    try:
        organization_addr = BackendOrganizationAddr(organization_addr)
    except ValueError as exc:
        raise JobResultError("bad-url") from exc

    try:
        device_id = DeviceID(f"{user_id}@{device_name}")
    except ValueError as exc:
        raise JobResultError("bad-device_name") from exc

    try:
        device = await core_claim_device(organization_addr, device_id, token)

    except InviteClaimBackendOfflineError as exc:
        raise JobResultError("backend-offline", info=str(exc)) from exc

    except InviteClaimError as exc:
        print(exc)
        if "Cannot retrieve invitation creator" in str(exc):
            raise JobResultError("not_found", info=str(exc)) from exc
        else:
            raise JobResultError("refused-by-backend", info=str(exc)) from exc

    try:
        save_device_with_password(config_dir, device, password)

    except LocalDeviceAlreadyExistsError as exc:
        raise JobResultError("device-exists") from exc

    return device, password


class ClaimDeviceWidget(QWidget, Ui_ClaimDeviceWidget):
    device_claimed = pyqtSignal()
    claim_success = pyqtSignal()
    claim_error = pyqtSignal()

    def __init__(self, jobs_ctx, config, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.jobs_ctx = jobs_ctx
        self.config = config
        self.claim_device_job = None
        self.button_claim.clicked.connect(self.claim_clicked)
        self.line_edit_login.textChanged.connect(self.check_infos)
        self.line_edit_device.textChanged.connect(self.check_infos)
        self.line_edit_token.textChanged.connect(self.check_infos)
        self.line_edit_url.textChanged.connect(self.check_infos)
        self.line_edit_password.textChanged.connect(self.check_infos)
        self.line_edit_password_check.textChanged.connect(self.check_infos)
        self.line_edit_password.textChanged.connect(self.password_changed)
        self.claim_success.connect(self.on_claim_success)
        self.claim_error.connect(self.on_claim_error)
        self.line_edit_login.setValidator(validators.UserIDValidator())
        self.line_edit_device.setValidator(validators.DeviceNameValidator())
        self.line_edit_url.setValidator(validators.BackendOrganizationAddrValidator())
        self.claim_dialog = ClaimDialog(parent=self)
        self.claim_dialog.setText(_("LABEL_DEVICE_REGISTRATION"))
        self.claim_dialog.cancel_clicked.connect(self.cancel_claim)
        self.claim_dialog.hide()
        self.line_edit_device.setText(get_default_device())
        self.label_password_strength.hide()
        self.check_infos()

    def on_claim_error(self):
        assert self.claim_device_job
        assert self.claim_device_job.is_finished()
        assert self.claim_device_job.status != "ok"

        status = self.claim_device_job.status
        if status != "cancelled":
            if status == "not_found":
                errmsg = _("ERR_CLAIM_DEVICE_NOT_FOUND")
            elif status == "password-mismatch":
                errmsg = _("ERR_PASSWORD_MISMATCH")
            elif status == "password-size":
                errmsg = _("ERR_PASSWORD_COMPLEXITY")
            elif status == "bad-url":
                errmsg = _("ERR_BAD_URL")
            elif status == "bad-device_name":
                errmsg = _("ERR_BAD_DEVICE_NAME")
            elif status == "bad-user_id":
                errmsg = _("ERR_BAD_USER_NAME")
            elif status == "refused-by-backend":
                errmsg = _("ERR_BACKEND_REFUSED")
            elif status == "backend-offline":
                errmsg = _("ERR_BACKEND_OFFLINE")
            else:
                errmsg = _("ERR_CLAIM_DEVICE_UNKNOWN")
            show_error(self, errmsg, exception=self.claim_device_job.exc)
        self.claim_device_job = None
        self.check_infos()

    def on_claim_success(self):
        assert self.claim_device_job
        assert self.claim_device_job.is_finished()
        assert self.claim_device_job.status == "ok"

        self.claim_dialog.hide()
        self.button_claim.setDisabled(False)
        show_info(self, _("INFO_DEVICE_REGISTERED"))
        self.claim_device_job = None
        self.device_claimed.emit()
        self.check_infos()

    def cancel_claim(self):
        if self.claim_device_job:
            self.claim_device_job.cancel_and_join()

    def password_changed(self, text):
        if len(text):
            self.label_password_strength.show()
            score = get_password_strength(text)
            self.label_password_strength.setText(
                _("LABEL_PASSWORD_STRENGTH_{}").format(get_password_strength_text(score))
            )
            self.label_password_strength.setStyleSheet(PASSWORD_CSS[score])
        else:
            self.label_password_strength.hide()

    def check_infos(self, _=""):
        if self.claim_device_job:
            self.claim_dialog.show()
        else:
            self.claim_dialog.hide()

        if (
            len(self.line_edit_login.text())
            and len(self.line_edit_token.text())
            and len(self.line_edit_device.text())
            and len(self.line_edit_url.text())
            and not self.claim_device_job
            and len(self.line_edit_password.text())
            and get_password_strength(self.line_edit_password.text()) > 0
            and len(self.line_edit_password_check.text())
        ):
            self.button_claim.setDisabled(False)
        else:
            self.button_claim.setDisabled(True)

    def keyPressEvent(self, event):
        if event.key() == Qt.Key_Return and not self.claim_device_job:
            self.claim_clicked()
        event.accept()

    def claim_clicked(self):
        assert not self.claim_device_job

        self.claim_device_job = self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "claim_success"),
            ThreadSafeQtSignal(self, "claim_error"),
            _do_claim_device,
            config_dir=self.config.config_dir,
            password=self.line_edit_password.text(),
            password_check=self.line_edit_password_check.text(),
            token=self.line_edit_token.text(),
            user_id=self.line_edit_login.text(),
            device_name=self.line_edit_device.text(),
            organization_addr=self.line_edit_url.text(),
        )
        self.check_infos()
