# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pendulum

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget

from parsec.crypto import SigningKey
from parsec.api.data import UserCertificateContent, DeviceCertificateContent
from parsec.api.protocol import OrganizationID, DeviceID
from parsec.core.types import BackendOrganizationBootstrapAddr
from parsec.core.backend_connection import (
    backend_anonymous_cmds_factory,
    BackendNotAvailable,
    BackendConnectionRefused,
    BackendConnectionError,
)
from parsec.core.local_device import (
    generate_new_device,
    save_device_with_password,
    LocalDeviceAlreadyExistsError,
)
from parsec.core.gui.trio_thread import JobResultError, ThreadSafeQtSignal
from parsec.core.gui.custom_dialogs import show_error
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
    password: str,
    password_check: str,
    user_id: str,
    device_name: str,
    bootstrap_addr: BackendOrganizationBootstrapAddr,
):
    if password != password_check:
        raise JobResultError("password-mismatch")
    if len(password) < 8:
        raise JobResultError("password-size")

    try:
        device_id = DeviceID(f"{user_id}@{device_name}")
    except ValueError as exc:
        raise JobResultError("bad-device_name") from exc

    root_signing_key = SigningKey.generate()
    root_verify_key = root_signing_key.verify_key
    organization_addr = bootstrap_addr.generate_organization_addr(root_verify_key)

    try:
        device = generate_new_device(device_id, organization_addr, is_admin=True)
        save_device_with_password(config_dir, device, password)

    except LocalDeviceAlreadyExistsError as exc:
        raise JobResultError("user-exists") from exc

    now = pendulum.now()
    user_certificate = UserCertificateContent(
        author=None,
        timestamp=now,
        user_id=device.user_id,
        public_key=device.public_key,
        is_admin=device.is_admin,
    ).dump_and_sign(root_signing_key)
    device_certificate = DeviceCertificateContent(
        author=None, timestamp=now, device_id=device_id, verify_key=device.verify_key
    ).dump_and_sign(root_signing_key)

    try:
        async with backend_anonymous_cmds_factory(bootstrap_addr) as cmds:
            rep = await cmds.organization_bootstrap(
                bootstrap_addr.organization_id,
                bootstrap_addr.token,
                root_verify_key,
                user_certificate,
                device_certificate,
            )

            if rep["status"] in ("already_bootstrapped", "not_found"):
                raise JobResultError("invalid-url", info=str(rep))
            elif rep["status"] != "ok":
                raise JobResultError("refused-by-backend", info=str(rep))
            # TODO: handle `JobResultError("bad-api-version")` ?

    except BackendConnectionRefused as exc:
        raise JobResultError("invalid-url", info=str(exc)) from exc

    except BackendNotAvailable as exc:
        raise JobResultError("backend-offline", info=str(exc)) from exc

    except BackendConnectionError as exc:
        raise JobResultError("refused-by-backend", info=str(exc)) from exc

    return device, password


class BootstrapOrganizationWidget(QWidget, Ui_BootstrapOrganizationWidget):
    bootstrap_success = pyqtSignal()
    bootstrap_error = pyqtSignal()
    organization_bootstrapped = pyqtSignal(OrganizationID, DeviceID, str)

    def __init__(self, jobs_ctx, config, addr: BackendOrganizationBootstrapAddr, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.jobs_ctx = jobs_ctx
        self.config = config
        self.addr = addr
        self.label_instructions.setText(
            _("LABEL_BOOTSTRAP_INSTRUCTIONS").format(
                url=self.addr.to_url(), organization=self.addr.organization_id
            )
        )
        self.bootstrap_job = None
        self.button_cancel.hide()
        self.button_bootstrap.clicked.connect(self.bootstrap_clicked)
        self.button_cancel.clicked.connect(self.cancel_bootstrap)
        self.line_edit_password.textChanged.connect(self.password_changed)
        self.line_edit_login.textChanged.connect(self.check_infos)
        self.line_edit_device.textChanged.connect(self.check_infos)
        self.line_edit_password.textChanged.connect(self.check_infos)
        self.line_edit_password_check.textChanged.connect(self.check_infos)
        self.line_edit_login.setValidator(validators.UserIDValidator())
        self.line_edit_device.setValidator(validators.DeviceNameValidator())
        self.bootstrap_success.connect(self.on_bootstrap_success)
        self.bootstrap_error.connect(self.on_bootstrap_error)

        self.line_edit_device.setText(get_default_device())
        self.button_cancel.hide()
        self.label_password_strength.hide()

        self.check_infos()

    def on_bootstrap_error(self):
        assert self.bootstrap_job
        assert self.bootstrap_job.is_finished()
        assert self.bootstrap_job.status != "ok"

        status = self.bootstrap_job.status
        if status == "invalid-url":
            errmsg = _("ERR_BAD_URL")
        elif status == "user-exists":
            errmsg = _("ERR_REGISTER_USER_EXISTS")
        elif status == "password-mismatch":
            errmsg = _("ERR_PASSWORD_MISMATCH")
        elif status == "password-size":
            errmsg = _("ERR_PASSWORD_COMPLEXITY")
        elif status == "bad-url" or status == "invalid-url":
            errmsg = _("ERR_BAD_URL")
        elif status == "bad-device_name":
            errmsg = _("ERR_BAD_DEVICE_NAME")
        elif status == "bad-user_id":
            errmsg = _("ERR_BAD_USER_NAME")
        elif status == "bad-api-version":
            errmsg = _("ERR_BAD_API_VERSION")
        elif status == "refused-by-backend":
            errmsg = _("ERR_BACKEND_REFUSED")
        elif status == "backend-offline":
            errmsg = _("ERR_BACKEND_OFFLINE")
        else:
            errmsg = _("ERR_BOOTSTRAP_ORG_UNKNOWN")
        show_error(self, errmsg, exception=self.bootstrap_job.exc)
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
            password=self.line_edit_password.text(),
            password_check=self.line_edit_password_check.text(),
            user_id=self.line_edit_login.text(),
            device_name=self.line_edit_device.text(),
            bootstrap_addr=self.addr,
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
            and not self.bootstrap_job
            and len(self.line_edit_password.text())
            and get_password_strength(self.line_edit_password.text()) > 0
            and len(self.line_edit_password_check.text())
        ):
            self.button_bootstrap.setDisabled(False)
        else:
            self.button_bootstrap.setDisabled(True)

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
