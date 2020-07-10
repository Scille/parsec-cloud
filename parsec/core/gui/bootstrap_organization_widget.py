# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget, QApplication, QDialog

from structlog import get_logger

from parsec.api.protocol import HumanHandle
from parsec.core.types import BackendOrganizationBootstrapAddr
from parsec.core.backend_connection import (
    apiv1_backend_anonymous_cmds_factory,
    BackendNotAvailable,
    BackendConnectionRefused,
    BackendConnectionError,
)
from parsec.core.invite import bootstrap_organization
from parsec.core.invite.exceptions import InviteNotFoundError, InviteAlreadyUsedError
from parsec.core.local_device import save_device_with_password
from parsec.core.gui.trio_thread import JobResultError, ThreadSafeQtSignal
from parsec.core.gui.custom_dialogs import show_error, GreyedDialog, show_info
from parsec.core.gui.desktop import get_default_device
from parsec.core.gui.lang import translate as _
from parsec.core.gui import validators
from parsec.core.gui.password_validation import PasswordStrengthWidget, get_password_strength
from parsec.core.gui.ui.bootstrap_organization_widget import Ui_BootstrapOrganizationWidget

logger = get_logger()


async def _do_bootstrap_organization(
    config_dir,
    password: str,
    password_check: str,
    human_handle: HumanHandle,
    device_label: str,
    bootstrap_addr: BackendOrganizationBootstrapAddr,
):
    if password != password_check:
        raise JobResultError("password-mismatch")
    if len(password) < 8:
        raise JobResultError("password-size")

    try:
        async with apiv1_backend_anonymous_cmds_factory(addr=bootstrap_addr) as cmds:
            new_device = await bootstrap_organization(
                cmds=cmds, human_handle=human_handle, device_label=device_label
            )
            save_device_with_password(config_dir, new_device, password)
            return new_device, password
    except InviteNotFoundError as exc:
        raise JobResultError("not-found", info=str(exc)) from exc
    except InviteAlreadyUsedError as exc:
        raise JobResultError("already-bootstrapped", info=str(exc)) from exc
    except BackendConnectionRefused as exc:
        raise JobResultError("invalid-url", info=str(exc)) from exc
    except BackendNotAvailable as exc:
        raise JobResultError("backend-offline", info=str(exc)) from exc
    except BackendConnectionError as exc:
        raise JobResultError("refused-by-backend", info=str(exc)) from exc


class BootstrapOrganizationWidget(QWidget, Ui_BootstrapOrganizationWidget):
    bootstrap_success = pyqtSignal()
    bootstrap_error = pyqtSignal()

    def __init__(self, jobs_ctx, config, addr: BackendOrganizationBootstrapAddr):
        super().__init__()
        self.setupUi(self)
        self.dialog = None
        self.jobs_ctx = jobs_ctx
        self.config = config
        self.addr = addr
        self.label_instructions.setText(
            _("TEXT_BOOTSTRAP_ORG_INSTRUCTIONS_url-organization").format(
                url=self.addr.to_url(), organization=self.addr.organization_id
            )
        )
        self.bootstrap_job = None
        self.button_bootstrap.clicked.connect(self.bootstrap_clicked)
        pwd_str_widget = PasswordStrengthWidget()
        self.layout_password_strength.addWidget(pwd_str_widget)
        self.line_edit_password.textChanged.connect(pwd_str_widget.on_password_change)
        self.line_edit_login.textChanged.connect(self.check_infos)
        self.line_edit_device.textChanged.connect(self.check_infos)
        self.line_edit_password.textChanged.connect(self.check_infos)
        self.line_edit_password_check.textChanged.connect(self.check_infos)
        self.line_edit_device.setValidator(validators.DeviceNameValidator())
        self.bootstrap_success.connect(self.on_bootstrap_success)
        self.bootstrap_error.connect(self.on_bootstrap_error)

        self.line_edit_device.setText(get_default_device())

        self.status = None

        self.check_infos()

    def on_bootstrap_error(self):
        assert self.bootstrap_job
        assert self.bootstrap_job.is_finished()
        assert self.bootstrap_job.status != "ok"

        status = self.bootstrap_job.status

        if status == "cancelled":
            self.bootstrap_job = None
            return

        if status == "invalid-url" or status == "bad-url":
            errmsg = _("TEXT_BOOTSTRAP_ORG_INVALID_URL")
        elif status == "not-found":
            errmsg = _("TEXT_BOOTSTRAP_ORG_INVITE_NOT_FOUND")
        elif status == "already-bootstrapped":
            errmsg = _("TEXT_BOOTSTRAP_ORG_ALREADY_BOOTSTRAPPED")
        elif status == "user-exists":
            errmsg = _("TEXT_BOOTSTRAP_ORG_USER_EXISTS")
        elif status == "password-mismatch":
            errmsg = _("TEXT_BOOTSTRAP_ORG_PASSWORD_MISMATCH")
        elif status == "password-size":
            errmsg = _("TEXT_BOOTSTRAP_ORG_PASSWORD_COMPLEXITY_TOO_LOW")
        elif status == "bad-device_name":
            errmsg = _("TEXT_BOOTSTRAP_ORG_BAD_DEVICE_NAME")
        elif status == "bad-api-version":
            errmsg = _("TEXT_BOOTSTRAP_ORG_BAD_API_VERSION")
        elif status == "refused-by-backend":
            errmsg = _("TEXT_BOOTSTRAP_ORG_BACKEND_REFUSAL")
        elif status == "backend-offline":
            errmsg = _("TEXT_BOOTSTRAP_ORG_BACKEND_OFFLINE")
        else:
            errmsg = _("TEXT_BOOTSTRAP_ORG_UNKNOWN_FAILURE")
        show_error(self, errmsg, exception=self.bootstrap_job.exc)
        self.bootstrap_job = None
        self.dialog.reject()

    def on_bootstrap_success(self):
        assert self.bootstrap_job
        assert self.bootstrap_job.is_finished()
        assert self.bootstrap_job.status == "ok"

        self.button_bootstrap.setDisabled(False)
        self.status = self.bootstrap_job.ret
        self.bootstrap_job = None
        self.check_infos()
        show_info(
            parent=self,
            message=_("TEXT_BOOTSTRAP_ORG_SUCCESS_organization").format(
                organization=self.addr.organization_id
            ),
            button_text=_("ACTION_CONTINUE"),
        )
        if self.dialog:
            self.dialog.accept()
        elif QApplication.activeModalWidget():
            QApplication.activeModalWidget().accept()
        else:
            logger.warning("Cannot close dialog when bootstraping")

    def bootstrap_clicked(self):
        assert not self.bootstrap_job

        try:
            human_handle = HumanHandle(
                email=self.line_edit_email.text(), label=self.line_edit_login.text()
            )
        except ValueError as exc:
            show_error(_("TEXT_BOOTSTRAP_ORG_INVALID_EMAIL"), exception=exc)
            return

        self.bootstrap_job = self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "bootstrap_success"),
            ThreadSafeQtSignal(self, "bootstrap_error"),
            _do_bootstrap_organization,
            config_dir=self.config.config_dir,
            password=self.line_edit_password.text(),
            password_check=self.line_edit_password_check.text(),
            human_handle=human_handle,
            device_label=self.line_edit_device.text(),
            bootstrap_addr=self.addr,
        )
        self.check_infos()

    def on_close(self):
        self.cancel_bootstrap()

    def cancel_bootstrap(self):
        if self.bootstrap_job:
            self.bootstrap_job.cancel_and_join()

    def check_infos(self, _=""):
        if (
            len(self.line_edit_login.text())
            and len(self.line_edit_device.text())
            and not self.bootstrap_job
            and len(self.line_edit_password.text())
            and get_password_strength(self.line_edit_password.text()) > 0
            and len(self.line_edit_password_check.text())
            and len(self.line_edit_email.text())
        ):
            self.button_bootstrap.setDisabled(False)
        else:
            self.button_bootstrap.setDisabled(True)

    @classmethod
    def exec_modal(cls, jobs_ctx, config, addr, parent):
        w = cls(jobs_ctx=jobs_ctx, config=config, addr=addr)
        d = GreyedDialog(w, _("TEXT_BOOTSTRAP_ORG_TITLE"), parent=parent, width=1000)
        w.dialog = d
        w.line_edit_login.setFocus()
        if d.exec_() == QDialog.Accepted and w.status:
            return w.status
        return None
