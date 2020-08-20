# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from structlog import get_logger

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget, QApplication, QDialog

from parsec.api.protocol import OrganizationID, DeviceName, HumanHandle
from parsec.core.backend_connection import (
    apiv1_backend_anonymous_cmds_factory,
    BackendConnectionRefused,
    BackendNotAvailable,
)
from parsec.core.types import BackendOrganizationBootstrapAddr
from parsec.core.invite import bootstrap_organization, InviteAlreadyUsedError
from parsec.core.local_device import save_device_with_password

from parsec.core.gui.custom_dialogs import GreyedDialog, show_error, show_info
from parsec.core.gui.trio_thread import JobResultError, ThreadSafeQtSignal
from parsec.core.gui.desktop import get_default_device
from parsec.core.gui.lang import translate as _
from parsec.core.gui import validators
from parsec.core.gui.password_validation import PasswordStrengthWidget, get_password_strength

from parsec.core.gui.ui.create_org_widget import Ui_CreateOrgWidget
from parsec.core.gui.ui.create_org_user_info_widget import Ui_CreateOrgUserInfoWidget
from parsec.core.gui.ui.create_org_device_info_widget import Ui_CreateOrgDeviceInfoWidget


logger = get_logger()


async def _do_create_org(config, human_handle, device_name, password, backend_addr):
    try:
        async with apiv1_backend_anonymous_cmds_factory(addr=backend_addr) as cmds:
            new_device = await bootstrap_organization(
                cmds=cmds, human_handle=human_handle, device_label=device_name
            )
            save_device_with_password(
                config_dir=config.config_dir, device=new_device, password=password
            )
            return new_device, password
    except InviteAlreadyUsedError as exc:
        raise JobResultError("invite-already-used", exc=exc)
    except BackendConnectionRefused as exc:
        raise JobResultError("connection-refused", exc=exc)
    except BackendNotAvailable as exc:
        raise JobResultError("connection-error", exc=exc)


class CreateOrgUserInfoWidget(QWidget, Ui_CreateOrgUserInfoWidget):
    valid_info_entered = pyqtSignal()
    invalid_info_entered = pyqtSignal()

    def __init__(self):
        super().__init__()
        self.setupUi(self)
        self.line_edit_user_email.textChanged.connect(self.check_infos)
        self.line_edit_user_full_name.textChanged.connect(self.check_infos)
        self.line_edit_org_name.textChanged.connect(self.check_infos)
        self.line_edit_org_name.setValidator(validators.OrganizationIDValidator())
        self.check_accept_contract.clicked.connect(self.check_infos)

    def check_infos(self):
        if (
            self.line_edit_user_email.text()
            and self.line_edit_user_full_name.text()
            and self.line_edit_org_name.text()
            and self.check_accept_contract.isChecked()
        ):
            self.valid_info_entered.emit()
        else:
            self.invalid_info_entered.emit()


class CreateOrgDeviceInfoWidget(QWidget, Ui_CreateOrgDeviceInfoWidget):
    valid_info_entered = pyqtSignal()
    invalid_info_entered = pyqtSignal()

    def __init__(self):
        super().__init__()
        self.setupUi(self)
        pwd_str_widget = PasswordStrengthWidget()
        self.layout_password_strength.addWidget(pwd_str_widget)
        self.line_edit_password.textChanged.connect(pwd_str_widget.on_password_change)
        self.line_edit_password.textChanged.connect(self.check_infos)
        self.line_edit_password_check.textChanged.connect(self.check_infos)
        self.line_edit_device.setText(get_default_device())
        self.line_edit_device.textChanged.connect(self.check_infos)
        self.line_edit_device.setValidator(validators.DeviceNameValidator())

    def check_infos(self, _=""):
        if (
            len(self.line_edit_device.text())
            and len(self.line_edit_password.text())
            and get_password_strength(self.line_edit_password.text()) > 0
            and len(self.line_edit_password_check.text())
            and self.line_edit_password.text() == self.line_edit_password_check.text()
        ):
            self.valid_info_entered.emit()
        else:
            self.invalid_info_entered.emit()


class CreateOrgWidget(QWidget, Ui_CreateOrgWidget):
    req_success = pyqtSignal()
    req_error = pyqtSignal()

    def __init__(self, jobs_ctx, config, start_addr):
        super().__init__()
        self.setupUi(self)
        self.jobs_ctx = jobs_ctx
        self.config = config
        self.create_job = None
        self.dialog = None
        self.status = None

        self.device_widget = CreateOrgDeviceInfoWidget()
        self.device_widget.valid_info_entered.connect(self._on_info_valid)
        self.device_widget.invalid_info_entered.connect(self._on_info_invalid)

        self.user_widget = CreateOrgUserInfoWidget()
        self.user_widget.valid_info_entered.connect(self._on_info_valid)
        self.user_widget.invalid_info_entered.connect(self._on_info_invalid)

        self.main_layout.addWidget(self.user_widget)
        self.main_layout.addWidget(self.device_widget)

        self._on_previous_clicked()

        self.button_validate.setEnabled(False)
        self.button_previous.clicked.connect(self._on_previous_clicked)
        self.button_previous.hide()

        self.req_success.connect(self._on_req_success)
        self.req_error.connect(self._on_req_error)

        self.start_addr = start_addr

        if self.start_addr:
            self.user_widget.line_edit_org_name.setText(self.start_addr.organization_id)
            self.user_widget.line_edit_org_name.setReadOnly(True)
            self.label_instructions.setText(
                _("TEXT_BOOTSTRAP_ORGANIZATION_INSTRUCTIONS_organization").format(
                    organization=self.start_addr.organization_id
                )
            )

    def _on_info_valid(self):
        self.button_validate.setEnabled(True)

    def _on_info_invalid(self):
        self.button_validate.setEnabled(False)

    def on_close(self):
        self.status = None
        if self.create_job:
            self.create_job.cancel_and_join()

    def _on_previous_clicked(self):
        self.user_widget.show()
        self.device_widget.hide()

        try:
            self.button_validate.clicked.disconnect(self._on_validate_clicked)
        except TypeError:
            pass
        self.button_validate.setText(_("ACTION_NEXT"))
        self.button_validate.clicked.connect(self._on_next_clicked)
        self.button_previous.hide()
        self.user_widget.check_infos()

    def _on_next_clicked(self):
        self.user_widget.hide()
        self.device_widget.show()

        try:
            self.button_validate.clicked.disconnect(self._on_next_clicked)
        except TypeError:
            pass
        self.button_validate.setText(_("ACTION_CREATE_ORG"))
        self.button_validate.clicked.connect(self._on_validate_clicked)
        self.button_previous.show()
        self.device_widget.check_infos()

    def _on_validate_clicked(self):
        backend_addr = None
        org_id = None
        device_name = None
        human_handle = None

        if self.start_addr:
            backend_addr = self.start_addr
        else:
            try:
                org_id = OrganizationID(self.user_widget.line_edit_org_name.text())
            except ValueError as exc:
                show_error(self, _("TEXT_ORG_WIZARD_INVALID_ORGANIZATION_ID"), exception=exc)
                return
            try:
                backend_addr = BackendOrganizationBootstrapAddr.build(
                    backend_addr=self.config.default_backend_addr, organization_id=org_id
                )
            except ValueError as exc:
                show_error(self, _("TEXT_ORG_WIZARD_INVALID_BACKEND_ADDR"), exception=exc)
                return
        try:
            device_name = DeviceName(self.device_widget.line_edit_device.text())
        except ValueError as exc:
            show_error(self, _("TEXT_ORG_WIZARD_INVALID_DEVICE_NAME"), exception=exc)
            return
        try:
            human_handle = HumanHandle(
                self.user_widget.line_edit_user_email.text(),
                self.user_widget.line_edit_user_full_name.text(),
            )
        except ValueError as exc:
            show_error(self, _("TEXT_ORG_WIZARD_INVALID_HUMAN_HANDLE"), exception=exc)
            return

        self.create_job = self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "req_success"),
            ThreadSafeQtSignal(self, "req_error"),
            _do_create_org,
            config=self.config,
            human_handle=human_handle,
            device_name=device_name,
            password=self.device_widget.line_edit_password.text(),
            backend_addr=backend_addr,
        )
        self.button_validate.setEnabled(False)

    def _on_req_success(self):
        assert self.create_job
        assert self.create_job.is_finished()
        assert self.create_job.status == "ok"

        self.status = self.create_job.ret
        self.create_job = None
        show_info(
            parent=self,
            message=_("TEXT_BOOTSTRAP_ORG_SUCCESS_organization").format(
                organization=self.status[0].organization_id
            ),
            button_text=_("ACTION_CONTINUE"),
        )
        if self.dialog:
            self.dialog.accept()
        elif QApplication.activeModalWidget():
            QApplication.activeModalWidget().accept()
        else:
            logger.warning("Cannot close dialog when org wizard")

    def _on_req_error(self):
        assert self.create_job
        assert self.create_job.is_finished()
        assert self.create_job.status != "ok"

        status = self.create_job.status

        if status == "cancelled":
            return

        errmsg = None
        if status == "organization_already_exists":
            errmsg = _("TEXT_ORG_WIZARD_ORGANIZATION_ALREADY_EXISTS")
        elif status == "invalid_email":
            errmsg = _("TEXT_ORG_WIZARD_INVALID_EMAIL")
        elif status == "invalid_organization_id":
            errmsg = _("TEXT_ORG_WIZARD_INVALID_ORGANIZATION_ID")
        elif status == "offline":
            errmsg = _("TEXT_ORG_WIZARD_OFFLINE")
        elif status == "invite-already-used":
            errmsg = _("TEXT_ORG_WIZARD_INVITE_ALREADY_USED")
        elif status == "connection-refused":
            errmsg = _("TEXT_ORG_WIZARD_CONNECTION_REFUSED")
        elif status == "connection-error":
            errmsg = _("TEXT_ORG_WIZARD_CONNECTION_ERROR")
        else:
            errmsg = _("TEXT_ORG_WIZARD_UNKNOWN_FAILURE")
        exc = self.create_job.exc
        if exc.params.get("exc"):
            exc = exc.params.get("exc")
        show_error(self, errmsg, exception=exc)
        self.status = None
        self.create_job = None
        self.button_validate.setEnabled(True)

    @classmethod
    def show_modal(cls, jobs_ctx, config, parent, on_finished, start_addr=None):
        w = cls(jobs_ctx, config, start_addr)
        d = GreyedDialog(w, _("TEXT_ORG_WIZARD_TITLE"), parent=parent, width=1000)
        w.dialog = d

        def _on_finished(result):
            if result == QDialog.Accepted:
                return on_finished(w.status)
            return on_finished(None)

        d.finished.connect(_on_finished)
        # Unlike exec_, show is asynchronous and works within the main Qt loop
        d.show()
        return w
