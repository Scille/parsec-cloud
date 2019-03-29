# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pendulum

from PyQt5.QtCore import QCoreApplication, pyqtSignal, Qt
from PyQt5.QtWidgets import QWidget

from parsec.crypto import SigningKey, build_user_certificate, build_device_certificate
from parsec.types import BackendOrganizationBootstrapAddr, DeviceID
from parsec.core.backend_connection import (
    BackendCmdsBadResponse,
    backend_anonymous_cmds_factory,
    BackendNotAvailable,
    BackendHandshakeError,
)
from parsec.core.local_device import (
    generate_new_device,
    save_device_with_password,
    LocalDeviceAlreadyExistsError,
)
from parsec.core.gui.trio_thread import JobResultError, ThreadSafeQtSignal
from parsec.core.gui.custom_widgets import show_error, show_info
from parsec.core.gui.desktop import get_default_device
from parsec.core.gui import validators
from parsec.core.gui.password_validation import (
    get_password_strength,
    PASSWORD_STRENGTH_TEXTS,
    PASSWORD_CSS,
)
from parsec.core.gui.ui.bootstrap_organization_widget import Ui_BootstrapOrganizationWidget


_translate = QCoreApplication.translate


STATUS_TO_ERRMSG = {
    "invalid-url": _translate(
        "BootstrapOrganizationWidget", "This organization does not exist (is the URL correct ?)."
    ),
    "user-exists": _translate("BootstrapOrganizationWidget", "This user already exists."),
    "password-mismatch": _translate("BootstrapOrganizationWidget", "Passwords don't match."),
    "password-size": _translate(
        "BootstrapOrganizationWidget", "Password must be at least 8 caracters long."
    ),
    "bad-url": _translate("BootstrapOrganizationWidget", "URL or device is invalid."),
    "bad-device_name": _translate("BootstrapOrganizationWidget", "URL or device is invalid."),
    "bad-user_id": _translate("BootstrapOrganizationWidget", "URL or device is invalid."),
}
DEFAULT_ERRMSG = _translate(
    "BootstrapOrganizationWidget", "Can not bootstrap this organization ({info})."
)


async def _do_bootstrap_organization(
    config_dir,
    use_pkcs11: bool,
    password: str,
    password_check: str,
    user_id: str,
    device_name: str,
    bootstrap_addr: str,
    pkcs11_token: int,
    pkcs11_key: int,
):
    if not use_pkcs11:
        if password != password_check:
            raise JobResultError("password-mismatch")
        if len(password) < 8:
            raise JobResultError("password-size")

    try:
        bootstrap_addr = BackendOrganizationBootstrapAddr(bootstrap_addr)
    except ValueError:
        raise JobResultError("bad-url")

    try:
        device_id = DeviceID(f"{user_id}@{device_name}")
    except ValueError:
        raise JobResultError("bad-device_name")

    root_signing_key = SigningKey.generate()
    root_verify_key = root_signing_key.verify_key
    organization_addr = bootstrap_addr.generate_organization_addr(root_verify_key)

    try:
        device = generate_new_device(device_id, organization_addr)
        save_device_with_password(config_dir, device, password)

    except LocalDeviceAlreadyExistsError:
        raise JobResultError("user-exists")

    now = pendulum.now()
    user_certificate = build_user_certificate(
        None, root_signing_key, device.user_id, device.public_key, now
    )
    device_certificate = build_device_certificate(
        None, root_signing_key, device_id, device.verify_key, now
    )

    try:
        async with backend_anonymous_cmds_factory(bootstrap_addr) as cmds:
            await cmds.organization_bootstrap(
                bootstrap_addr.organization_id,
                bootstrap_addr.bootstrap_token,
                root_verify_key,
                user_certificate,
                device_certificate,
            )

    except BackendHandshakeError:
        raise JobResultError("invalid-url")

    except BackendNotAvailable as exc:
        raise JobResultError("backend-offline", info=str(exc))

    except BackendCmdsBadResponse as exc:
        raise JobResultError("refused-by-backend", info=str(exc))


class BootstrapOrganizationWidget(QWidget, Ui_BootstrapOrganizationWidget):
    bootstrap_success = pyqtSignal()
    bootstrap_error = pyqtSignal()
    organization_bootstrapped = pyqtSignal()

    def __init__(self, portal, core_config, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.portal = portal
        self.core_config = core_config
        self.bootstrap_job = None
        self.button_cancel.hide()
        self.button_bootstrap.clicked.connect(self.bootstrap_clicked)
        self.button_cancel.clicked.connect(self.cancel_bootstrap)
        self.line_edit_password.textChanged.connect(self.password_changed)
        self.line_edit_login.textChanged.connect(self.check_infos)
        self.line_edit_device.textChanged.connect(self.check_infos)
        self.line_edit_url.textChanged.connect(self.check_infos)
        self.line_edit_login.setValidator(validators.UserIDValidator())
        self.line_edit_device.setValidator(validators.DeviceNameValidator())
        self.line_edit_url.setValidator(validators.BackendOrganizationAddrValidator())
        self.bootstrap_success.connect(self.on_bootstrap_success)
        self.bootstrap_error.connect(self.on_bootstrap_error)

    def on_bootstrap_error(self):
        assert self.bootstrap_job.is_finished()
        assert self.bootstrap_job.status != "ok"
        self.button_cancel.hide()
        self.check_infos()
        errmsg = STATUS_TO_ERRMSG.get(self.bootstrap_job.status, DEFAULT_ERRMSG)
        show_error(self, errmsg.format(**self.bootstrap_job.exc.params))
        self.bootstrap_job = None

    def on_bootstrap_success(self):
        assert self.bootstrap_job.is_finished()
        assert self.bootstrap_job.status == "ok"
        self.button_bootstrap.setDisabled(False)
        self.button_cancel.hide()
        show_info(
            self,
            QCoreApplication.translate(
                "BootstrapOrganizationWidget",
                "The organization and the user have been created. You can now login.",
            ),
        )
        self.bootstrap_job = None
        self.organization_bootstrapped.emit()

    def bootstrap_clicked(self):
        assert not self.bootstrap_job
        self.bootstrap_job = self.portal.submit_job(
            ThreadSafeQtSignal(self, "bootstrap_success"),
            ThreadSafeQtSignal(self, "bootstrap_error"),
            _do_bootstrap_organization,
            config_dir=self.core_config.config_dir,
            use_pkcs11=(self.check_box_use_pkcs11.checkState() == Qt.Checked),
            password=self.line_edit_password.text(),
            password_check=self.line_edit_password_check.text(),
            user_id=self.line_edit_login.text(),
            device_name=self.line_edit_device.text(),
            bootstrap_addr=self.line_edit_url.text(),
            pkcs11_token=int(self.combo_pkcs11_token.currentText()),
            pkcs11_key=int(self.combo_pkcs11_key.currentText()),
        )
        self.button_cancel.show()
        self.button_bootstrap.setDisabled(True)

    def cancel_bootstrap(self):
        assert self.bootstrap_job
        self.bootstrap_job.cancel_and_join()
        self.bootstrap_job = None
        self.button_cancel.hide()
        self.button_bootstrap.setDisabled(False)
        self.check_infos()

    def check_infos(self, _=""):
        if (
            len(self.line_edit_login.text())
            and len(self.line_edit_device.text())
            and len(self.line_edit_url.text())
        ):
            self.button_bootstrap.setDisabled(False)
        else:
            self.button_bootstrap.setDisabled(True)

    def password_changed(self, text):
        if len(text):
            self.label_password_strength.show()
            score = get_password_strength(text)
            self.label_password_strength.setText(
                QCoreApplication.translate(
                    "BootstrapOrganizationWidget", "Password strength: {}"
                ).format(PASSWORD_STRENGTH_TEXTS[score])
            )
            self.label_password_strength.setStyleSheet(PASSWORD_CSS[score])
        else:
            self.label_password_strength.hide()

    def reset(self):
        self.line_edit_login.setText("")
        self.line_edit_password.setText("")
        self.line_edit_password_check.setText("")
        self.line_edit_url.setText("")
        self.line_edit_device.setText(get_default_device())
        self.check_box_use_pkcs11.setCheckState(Qt.Unchecked)
        self.combo_pkcs11_key.clear()
        self.combo_pkcs11_key.addItem("0")
        self.combo_pkcs11_token.clear()
        self.combo_pkcs11_token.addItem("0")
        self.button_cancel.hide()
        self.widget_pkcs11.hide()
        self.label_password_strength.hide()
