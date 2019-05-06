# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pendulum

from PyQt5.QtCore import pyqtSignal, Qt
from PyQt5.QtWidgets import QWidget

from parsec.crypto import SigningKey, build_user_certificate, build_device_certificate
from parsec.types import BackendOrganizationBootstrapAddr, DeviceID, OrganizationID
from parsec.core.backend_connection import (
    BackendCmdsBadResponse,
    backend_anonymous_cmds_factory,
    BackendNotAvailable,
    BackendIncompatibleVersion,
    BackendHandshakeError,
)
from parsec.core.local_device import (
    generate_new_device,
    save_device_with_password,
    LocalDeviceAlreadyExistsError,
)
from parsec.core.gui.trio_thread import JobResultError, ThreadSafeQtSignal
from parsec.core.gui.custom_widgets import show_error
from parsec.core.gui.desktop import get_default_device
from parsec.core.gui.lang import translate as _
from parsec.core.gui import validators
from parsec.core.gui.password_validation import (
    get_password_strength,
    get_password_strength_text,
    PASSWORD_CSS,
)
from parsec.core.gui.ui.bootstrap_organization_widget import Ui_BootstrapOrganizationWidget


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

    except BackendIncompatibleVersion as exc:
        raise JobResultError("bad-api-version", info=str(exc))

    except BackendHandshakeError:
        raise JobResultError("invalid-url")

    except BackendNotAvailable as exc:
        raise JobResultError("backend-offline", info=str(exc))

    except BackendCmdsBadResponse as exc:
        raise JobResultError("refused-by-backend", info=str(exc))

    return device, password


class BootstrapOrganizationWidget(QWidget, Ui_BootstrapOrganizationWidget):
    bootstrap_success = pyqtSignal()
    bootstrap_error = pyqtSignal()
    organization_bootstrapped = pyqtSignal(OrganizationID, DeviceID, str)

    def __init__(self, jobs_ctx, config, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.jobs_ctx = jobs_ctx
        self.config = config
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

        self.line_edit_device.setText(get_default_device())
        self.combo_pkcs11_key.addItem("0")
        self.combo_pkcs11_token.addItem("0")
        self.button_cancel.hide()
        self.widget_pkcs11.hide()
        self.label_password_strength.hide()
        self.check_infos()

    def on_bootstrap_error(self):
        assert self.bootstrap_job
        assert self.bootstrap_job.is_finished()
        assert self.bootstrap_job.status != "ok"

        status = self.bootstrap_job.status
        if status == "invalid-url":
            errmsg = _("This organization does not exist (is the URL correct ?).")
        elif status == "user-exists":
            errmsg = _("This user already exists.")
        elif status == "password-mismatch":
            errmsg = _("Passwords don't match.")
        elif status == "password-size":
            errmsg = _("Password must be at least 8 caracters long.")
        elif status in ("bad-url", "bad-device_name", "bad-user_id"):
            errmsg = _("URL or device is invalid.")
        else:
            errmsg = _("Can not bootstrap this organization ({info}).")

        show_error(self, errmsg.format(**self.bootstrap_job.exc.params))
        self.bootstrap_job = None
        self.check_infos()

    def on_bootstrap_success(self):
        assert self.bootstrap_job
        assert self.bootstrap_job.is_finished()
        assert self.bootstrap_job.status == "ok"

        self.button_bootstrap.setDisabled(False)
        self.button_cancel.hide()
        device, password = self.bootstrap_job.ret
        self.bootstrap_job = None
        self.check_infos()

        self.organization_bootstrapped.emit(device.organization_id, device.device_id, password)

    def bootstrap_clicked(self):
        assert not self.bootstrap_job

        self.bootstrap_job = self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "bootstrap_success"),
            ThreadSafeQtSignal(self, "bootstrap_error"),
            _do_bootstrap_organization,
            config_dir=self.config.config_dir,
            use_pkcs11=(self.check_box_use_pkcs11.checkState() == Qt.Checked),
            password=self.line_edit_password.text(),
            password_check=self.line_edit_password_check.text(),
            user_id=self.line_edit_login.text(),
            device_name=self.line_edit_device.text(),
            bootstrap_addr=self.line_edit_url.text(),
            pkcs11_token=int(self.combo_pkcs11_token.currentText()),
            pkcs11_key=int(self.combo_pkcs11_key.currentText()),
        )
        self.check_infos()

    def cancel_bootstrap(self):
        if self.bootstrap_job:
            self.bootstrap_job.cancel_and_join()
            self.bootstrap_job = None
        self.check_infos()

    def check_infos(self, _=""):
        if self.bootstrap_job:
            self.button_cancel.show()
        else:
            self.button_cancel.hide()

        if (
            len(self.line_edit_login.text())
            and len(self.line_edit_device.text())
            and len(self.line_edit_url.text())
            and not self.bootstrap_job
        ):
            self.button_bootstrap.setDisabled(False)
        else:
            self.button_bootstrap.setDisabled(True)

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
