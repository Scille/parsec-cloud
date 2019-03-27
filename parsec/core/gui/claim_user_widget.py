# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os

from PyQt5.QtCore import pyqtSignal, QCoreApplication, Qt
from PyQt5.QtWidgets import QWidget

from parsec.types import BackendOrganizationAddr, DeviceID
from parsec.core.local_device import (
    LocalDeviceAlreadyExistsError,
    save_device_with_password,
    save_device_with_pkcs11,
)
from parsec.core.backend_connection import (
    BackendCmdsBadResponse,
    backend_anonymous_cmds_factory,
    BackendNotAvailable,
    BackendHandshakeError,
)
from parsec.core.invite_claim import claim_user as core_claim_user
from parsec.core.gui import validators
from parsec.core.gui.trio_thread import JobResultError
from parsec.core.gui.desktop import get_default_device
from parsec.core.gui.custom_widgets import show_error, show_info
from parsec.core.gui.claim_dialog import ClaimDialog
from parsec.core.gui.password_validation import (
    get_password_strength,
    PASSWORD_STRENGTH_TEXTS,
    PASSWORD_CSS,
)
from parsec.core.gui.ui.claim_user_widget import Ui_ClaimUserWidget

_translate = QCoreApplication.translate


STATUS_TO_ERRMSG = {
    "not_found": _translate("ClaimUserWidget", "No invitation found for this user."),
    "password-mismatch": _translate("ClaimUserWidget", "Passwords don't match."),
    "password-size": _translate("ClaimUserWidget", "Password must be at least 8 caracters long."),
    "bad-url": _translate("BootstrapOrganizationWidget", "URL or device is invalid."),
    "bad-device_name": _translate("BootstrapOrganizationWidget", "URL or device is invalid."),
    "bad-user_id": _translate("BootstrapOrganizationWidget", "URL or device is invalid."),
}
DEFAULT_ERRMSG = _translate("ClaimUserWidget", "Can not claim this user ({info}).")


async def _do_claim_user(
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
        async with backend_anonymous_cmds_factory(organization_addr) as cmds:
            device = await core_claim_user(cmds, device_id, token)

    except BackendHandshakeError:
        raise JobResultError("invalid-url")

    except BackendNotAvailable as exc:
        raise JobResultError("backend-offline", info=str(exc))

    except BackendCmdsBadResponse as exc:
        raise JobResultError("refused-by-backend", info=str(exc))

    try:
        if use_pkcs11:
            save_device_with_pkcs11(config_dir, device, pkcs11_token, pkcs11_key)
        else:
            save_device_with_password(config_dir, device, password)

    except LocalDeviceAlreadyExistsError:
        raise JobResultError("user-exists")


class ClaimUserWidget(QWidget, Ui_ClaimUserWidget):
    user_claimed = pyqtSignal()
    claim_success = pyqtSignal()
    claim_error = pyqtSignal()

    def __init__(self, portal, core_config, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.portal = portal
        self.core_config = core_config
        self.claim_user_job = None
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
        self.claim_dialog.setText(
            QCoreApplication.translate(
                "ClaimUserWidget", "Please wait while the user is registered."
            )
        )
        self.claim_dialog.cancel_clicked.connect(self.cancel_claim)
        self.claim_dialog.hide()

    def on_claim_error(self):
        assert self.claim_user_job.is_finished()
        assert self.claim_user_job.status != "ok"
        self.claim_dialog.hide()
        self.button_claim.setDisabled(False)
        self.check_infos()
        errmsg = STATUS_TO_ERRMSG.get(self.claim_user_job.status, DEFAULT_ERRMSG)
        show_error(self, errmsg.format(**self.claim_user_job.exc.params))
        self.claim_user_job = None

    def on_claim_success(self):
        assert self.claim_user_job.is_finished()
        assert self.claim_user_job.status == "ok"
        self.claim_dialog.hide()
        self.button_claim.setDisabled(False)
        show_info(
            self,
            QCoreApplication.translate(
                "ClaimUserWidget", "The user has been registered. You can now login."
            ),
        )
        self.claim_user_job = None
        self.user_claimed.emit()

    def cancel_claim(self):
        if self.claim_user_job:
            self.claim_user_job.cancel_and_join()
            self.claim_user_job = None
        self.claim_dialog.hide()
        self.button_claim.setDisabled(False)
        self.check_infos()

    def password_changed(self, text):
        if len(text):
            self.label_password_strength.show()
            score = get_password_strength(text)
            self.label_password_strength.setText(
                QCoreApplication.translate("ClaimUserWidget", "Password strength: {}").format(
                    PASSWORD_STRENGTH_TEXTS[score]
                )
            )
            self.label_password_strength.setStyleSheet(PASSWORD_CSS[score])
        else:
            self.label_password_strength.hide()

    def check_infos(self, _=""):
        if (
            len(self.line_edit_login.text())
            and len(self.line_edit_token.text())
            and len(self.line_edit_device.text())
            and len(self.line_edit_url.text())
        ):
            self.button_claim.setDisabled(False)
        else:
            self.button_claim.setDisabled(True)

    def keyPressEvent(self, event):
        if event.key() == Qt.Key_Return:
            self.claim_clicked()
        event.accept()

    def claim_clicked(self):
        assert not self.claim_user_job
        self.claim_user_job = self.portal.submit_job(
            self.claim_success,
            self.claim_error,
            _do_claim_user,
            config_dir=self.core_config.config_dir,
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
        self.claim_dialog.show()
        self.button_claim.setDisabled(True)

    def reset(self):
        if os.name == "nt":
            self.check_box_use_pkcs11.hide()
        self.line_edit_login.setText("")
        self.line_edit_password.setText("")
        self.line_edit_password_check.setText("")
        self.line_edit_url.setText("")
        self.line_edit_device.setText(get_default_device())
        self.line_edit_token.setText("")
        self.check_box_use_pkcs11.setCheckState(Qt.Unchecked)
        self.combo_pkcs11_key.clear()
        self.combo_pkcs11_key.addItem("0")
        self.combo_pkcs11_token.clear()
        self.combo_pkcs11_token.addItem("0")
        self.widget_pkcs11.hide()
        self.label_password_strength.hide()
