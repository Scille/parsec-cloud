# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import pyqtSignal, Qt
from PyQt5.QtWidgets import QWidget

from parsec.types import BackendOrganizationAddr, DeviceID
from parsec.core.local_device import (
    LocalDeviceAlreadyExistsError,
    save_device_with_password,
    save_device_with_pkcs11,
)
from parsec.core.invite_claim import (
    claim_device as core_claim_device,
    InviteClaimError,
    InviteClaimBackendOfflineError,
)
from parsec.core.gui import validators
from parsec.core.gui.trio_thread import JobResultError, ThreadSafeQtSignal
from parsec.core.gui.desktop import get_default_device
from parsec.core.gui.custom_widgets import show_error, show_info
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
    use_pkcs11: bool,
    password: str,
    password_check: str,
    token: str,
    user_id: str,
    device_name: str,
    organization_addr: str,
    pkcs11_token: int,
    pkcs11_key: int,
):
    if not use_pkcs11:
        if password != password_check:
            raise JobResultError("password-mismatch")
        if len(password) < 8:
            raise JobResultError("password-size")

    try:
        organization_addr = BackendOrganizationAddr(organization_addr)
    except ValueError:
        raise JobResultError("bad-url")

    try:
        device_id = DeviceID(f"{user_id}@{device_name}")
    except ValueError:
        raise JobResultError("bad-device_name")

    try:
        device = await core_claim_device(organization_addr, device_id, token)

    except InviteClaimBackendOfflineError as exc:
        raise JobResultError("backend-offline", info=str(exc))

    except InviteClaimError as exc:
        raise JobResultError("refused-by-backend", info=str(exc))

    try:
        if use_pkcs11:
            save_device_with_pkcs11(config_dir, device, pkcs11_token, pkcs11_key)
        else:
            save_device_with_password(config_dir, device, password)

    except LocalDeviceAlreadyExistsError:
        raise JobResultError("device-exists")

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
        self.line_edit_password.textChanged.connect(self.password_changed)
        self.claim_success.connect(self.on_claim_success)
        self.claim_error.connect(self.on_claim_error)
        self.line_edit_login.setValidator(validators.UserIDValidator())
        self.line_edit_device.setValidator(validators.DeviceNameValidator())
        self.line_edit_url.setValidator(validators.BackendOrganizationAddrValidator())
        self.claim_dialog = ClaimDialog(parent=self)
        self.claim_dialog.setText(_("Please wait while the device is registered."))
        self.claim_dialog.cancel_clicked.connect(self.cancel_claim)
        self.claim_dialog.hide()
        self.check_box_use_pkcs11.hide()
        self.line_edit_device.setText(get_default_device())
        self.combo_pkcs11_key.addItem("0")
        self.combo_pkcs11_token.addItem("0")
        self.widget_pkcs11.hide()
        self.label_password_strength.hide()
        self.check_infos()

    def on_claim_error(self):
        assert self.claim_device_job
        assert self.claim_device_job.is_finished()
        assert self.claim_device_job.status != "ok"

        status = self.claim_device_job.status
        if status == "not_found":
            errmsg = _("No invitation found for this device.")
        elif status == "password-mismatch":
            errmsg = _("Passwords don't match.")
        elif status == "password-size":
            errmsg = _("Password must be at least 8 caracters long.")
        elif status in ("bad-url", "bad-device_name", "bad-user_id"):
            errmsg = (_("URL or device is invalid."),)
        else:
            errmsg = _("Can not claim this device ({info}).")
        show_error(self, errmsg.format(**self.claim_device_job.exc.params))
        self.claim_device_job = None
        self.check_infos()

    def on_claim_success(self):
        assert self.claim_device_job
        assert self.claim_device_job.is_finished()
        assert self.claim_device_job.status == "ok"

        self.claim_dialog.hide()
        self.button_claim.setDisabled(False)
        show_info(self, _("The device has been registered. You can now login."))
        self.claim_device_job = None
        self.device_claimed.emit()
        self.check_infos()

    def cancel_claim(self):
        if self.claim_device_job:
            self.claim_device_job.cancel_and_join()
            self.claim_device_job = None
        self.check_infos()

    def password_changed(self, text):
        if len(text):
            self.label_password_strength.show()
            score = get_password_strength(text)
            self.label_password_strength.setText(
                _("Password strength: {}").format(get_password_strength_text(score))
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
            use_pkcs11=(self.check_box_use_pkcs11.checkState() == Qt.Checked),
            password=self.line_edit_password.text(),
            password_check=self.line_edit_password_check.text(),
            token=self.line_edit_token.text(),
            user_id=self.line_edit_login.text(),
            device_name=self.line_edit_device.text(),
            organization_addr=self.line_edit_url.text(),
            pkcs11_token=int(self.combo_pkcs11_token.currentText()),
            pkcs11_key=int(self.combo_pkcs11_key.currentText()),
        )
        self.check_infos()
