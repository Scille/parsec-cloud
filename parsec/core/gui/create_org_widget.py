# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
from structlog import get_logger
import trio
import json
from urllib.request import Request, urlopen
from urllib.error import URLError
from http.client import HTTPException

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget, QApplication, QDialog

from parsec.api.protocol import OrganizationID
from parsec.core.types import BackendOrganizationBootstrapAddr, BackendAddr

from parsec.core.backend_connection import (
    backend_administration_cmds_factory,
    BackendConnectionRefused,
)

from parsec.core.gui.custom_dialogs import GreyedDialog, show_error
from parsec.core.gui.trio_thread import JobResultError, ThreadSafeQtSignal
from parsec.core.gui.lang import translate as _

from parsec.core.gui.ui.create_org_widget import Ui_CreateOrgWidget
from parsec.core.gui.ui.create_org_first_page_widget import Ui_CreateOrgFirstPageWidget
from parsec.core.gui.ui.create_org_saas_widget import Ui_CreateOrgSaasWidget
from parsec.core.gui.ui.create_org_custom_widget import Ui_CreateOrgCustomWidget


logger = get_logger()


async def _do_create_organization(backend_addr, org_name, admin_token):
    try:
        async with backend_administration_cmds_factory(backend_addr, admin_token) as cmds:
            rep = await cmds.organization_create(org_name, None)
            if rep["status"] != "ok":
                raise JobResultError(rep["status"])
            return BackendOrganizationBootstrapAddr.build(
                backend_addr, org_name, rep["bootstrap_token"]
            )
    except BackendConnectionRefused as exc:
        if str(exc) == "Invalid administration token":
            raise JobResultError("invalid-admin-token")
        raise JobResultError("offline")


async def _do_api_request(email, organization_id):
    data = json.dumps({"email": email, "organization_id": organization_id}).encode("utf-8")
    url = os.environ.get("BOOTSTRAP_API_URL", "https://bms.parsec.cloud/api/quickjoin")
    req = Request(url, method="POST", data=data, headers={"Content-Type": "application/json"})
    try:
        response = await trio.to_thread.run_sync(lambda: urlopen(req))
        if response.status != 200:
            raise JobResultError("invalid_response")
        try:
            content = await trio.to_thread.run_sync(lambda: response.read())
            content = json.loads(content)
            if content.get("error", None):
                raise JobResultError(content["error"])
            return (
                content["parsec_id"],
                BackendOrganizationBootstrapAddr.from_url(content["bootstrap_link"]),
            )
        except (TypeError, KeyError) as exc:
            raise JobResultError("invalid_response", exc=exc)
    except (HTTPException, URLError) as exc:
        raise JobResultError("offline", exc=exc)


class CreateOrgFirstPageWidget(QWidget, Ui_CreateOrgFirstPageWidget):
    def __init__(self):
        super().__init__()
        self.setupUi(self)
        self.radio_create_org_saas.setChecked(True)


class CreateOrgSaasWidget(QWidget, Ui_CreateOrgSaasWidget):
    def __init__(self):
        super().__init__()
        self.setupUi(self)


class CreateOrgCustomWidget(QWidget, Ui_CreateOrgCustomWidget):
    def __init__(self):
        super().__init__()
        self.setupUi(self)


class CreateOrgWidget(QWidget, Ui_CreateOrgWidget):
    req_saas_success = pyqtSignal()
    req_saas_error = pyqtSignal()
    req_custom_success = pyqtSignal()
    req_custom_error = pyqtSignal()

    def __init__(self, jobs_ctx):
        super().__init__()
        self.setupUi(self)
        self.jobs_ctx = jobs_ctx
        self.req_job = None
        self.dialog = None
        self.status = None
        self.button_validate.clicked.connect(self._validate_clicked)
        self.button_previous.clicked.connect(self._previous_clicked)
        self.button_previous.hide()
        self.current_widget = CreateOrgFirstPageWidget()
        self.main_layout.addWidget(self.current_widget)
        self.req_saas_success.connect(self._on_req_saas_success)
        self.req_saas_error.connect(self._on_req_saas_error)
        self.req_custom_success.connect(self._on_req_custom_success)
        self.req_custom_error.connect(self._on_req_custom_error)

    def _clear_page(self):
        item = self.main_layout.takeAt(0)
        if item:
            w = item.widget()
            self.main_layout.removeWidget(w)
            w.hide()
            w.setParent(None)

    def on_close(self):
        self.status = None
        if self.req_job:
            self.req_job.cancel_and_join()

    def _on_req_saas_success(self):
        assert self.req_job
        assert self.req_job.is_finished()
        assert self.req_job.status == "ok"

        _, self.status = self.req_job.ret
        self.req_job = None
        if self.dialog:
            self.dialog.accept()
        elif QApplication.activeModalWidget():
            QApplication.activeModalWidget().accept()
        else:
            logger.warning("Cannot close dialog when org wizard")

    def _on_req_saas_error(self):
        assert self.req_job
        assert self.req_job.is_finished()
        assert self.req_job.status != "ok"

        status = self.req_job.status

        if status == "cancelled":
            return

        errmsg = None
        if status == "email_already_exists":
            errmsg = _("TEXT_ORG_WIZARD_EMAIL_ALREADY_EXISTS")
        elif status == "organization_already_exists":
            errmsg = _("TEXT_ORG_WIZARD_ORGANIZATION_ALREADY_EXISTS")
        elif status == "invalid_email":
            errmsg = _("TEXT_ORG_WIZARD_INVALID_EMAIL")
        elif status == "invalid_organization_id":
            errmsg = _("TEXT_ORG_WIZARD_INVALID_ORGANIZATION_ID")
        elif status == "invalid_response":
            errmsg = _("TEXT_ORG_WIZARD_INVALID_RESPONSE")
        elif status == "offline":
            errmsg = _("TEXT_ORG_WIZARD_OFFLINE")
        else:
            errmsg = _("TEXT_ORG_WIZARD_UNKNOWN_FAILURE")
        exc = self.req_job.exc
        if exc.params.get("exc"):
            exc = exc.params.get("exc")
        show_error(self, errmsg, exception=exc)
        self.req_job = None
        self.button_validate.setEnabled(True)
        self.button_previous.show()

    def _on_req_custom_success(self):
        assert self.req_job
        assert self.req_job.is_finished()
        assert self.req_job.status == "ok"

        self.status = self.req_job.ret
        self.req_job = None
        if self.dialog:
            self.dialog.accept()
        elif QApplication.activeModalWidget():
            QApplication.activeModalWidget().accept()
        else:
            logger.warning("Cannot close dialog when org wizard")

    def _on_req_custom_error(self):
        assert self.req_job
        assert self.req_job.is_finished()
        assert self.req_job.status != "ok"

        status = self.req_job.status

        if status == "cancelled":
            return

        errmsg = None
        if status == "organization_already_exists":
            errmsg = _("TEXT_ORG_WIZARD_ORGANIZATION_ALREADY_EXISTS")
        elif status == "invalid_organization_id":
            errmsg = _("TEXT_ORG_WIZARD_INVALID_ORGANIZATION_ID")
        elif status == "offline":
            errmsg = _("TEXT_ORG_WIZARD_OFFLINE")
        elif status == "invalid-admin-token":
            errmsg = _("TEXT_ORG_WIZARD_INVALID_ADMIN_TOKEN")
        else:
            errmsg = _("TEXT_ORG_WIZARD_UNKNOWN_FAILURE")
        exc = self.req_job.exc
        if exc.params.get("exc"):
            exc = exc.params.get("exc")
        show_error(self, errmsg, exception=exc)
        self.req_job = None
        self.button_validate.setEnabled(True)
        self.button_previous.show()

    def _validate_clicked(self):
        if isinstance(self.current_widget, CreateOrgFirstPageWidget):
            if self.current_widget.radio_bootstrap_org.isChecked():
                self.status = ""
                if self.dialog:
                    self.dialog.accept()
                elif QApplication.activeModalWidget():
                    QApplication.activeModalWidget().accept()
                else:
                    logger.warning("Cannot close dialog when org wizard")
            elif self.current_widget.radio_create_org_saas.isChecked():
                self.button_validate.setText(_("ACTION_CREATE_ORGANIZATION"))
                self.button_previous.show()
                self._clear_page()
                self.current_widget = CreateOrgSaasWidget()
                self.main_layout.addWidget(self.current_widget)
                self.current_widget.line_edit_user_email.textChanged.connect(self._check_saas_infos)
                self.current_widget.line_edit_org_name.textChanged.connect(self._check_saas_infos)
                self.button_validate.setEnabled(False)
            elif self.current_widget.radio_create_org_custom.isChecked():
                self.button_validate.setText(_("ACTION_CREATE_ORGANIZATION"))
                self.button_previous.show()
                self._clear_page()
                self.current_widget = CreateOrgCustomWidget()
                self.main_layout.addWidget(self.current_widget)
                self.current_widget.line_edit_server_addr.textChanged.connect(
                    self._check_custom_infos
                )
                self.current_widget.line_edit_secret_key.textChanged.connect(
                    self._check_custom_infos
                )
                self.current_widget.line_edit_org_name.textChanged.connect(self._check_custom_infos)
                self.button_validate.setEnabled(False)
        elif isinstance(self.current_widget, CreateOrgSaasWidget):
            self.req_job = self.jobs_ctx.submit_job(
                ThreadSafeQtSignal(self, "req_saas_success"),
                ThreadSafeQtSignal(self, "req_saas_error"),
                _do_api_request,
                email=self.current_widget.line_edit_user_email.text(),
                organization_id=self.current_widget.line_edit_org_name.text(),
            )
            self.button_validate.setEnabled(False)
            self.button_previous.hide()
        elif isinstance(self.current_widget, CreateOrgCustomWidget):
            backend_addr = None
            org_name = None
            try:
                backend_addr = BackendAddr.from_url(
                    self.current_widget.line_edit_server_addr.text()
                )
            except:
                show_error(self, _("TEXT_INVALID_URL"))
                return
            try:
                org_name = OrganizationID(self.current_widget.line_edit_org_name.text())
            except:
                show_error(self, _("TEXT_ORG_WIZARD_INVALID_ORGANIZATION_ID"))
                return
            self.req_job = self.jobs_ctx.submit_job(
                ThreadSafeQtSignal(self, "req_custom_success"),
                ThreadSafeQtSignal(self, "req_custom_error"),
                _do_create_organization,
                backend_addr=backend_addr,
                org_name=org_name,
                admin_token=self.current_widget.line_edit_secret_key.text(),
            )
            self.button_validate.setEnabled(False)
            self.button_previous.hide()

    def _previous_clicked(self):
        self._clear_page()
        self.current_widget = CreateOrgFirstPageWidget()
        self.main_layout.addWidget(self.current_widget)
        self.button_previous.hide()
        self.button_validate.setText(_("ACTION_NEXT"))
        self.button_validate.setEnabled(True)

    def _check_saas_infos(self):
        if (
            self.current_widget.line_edit_user_email.text()
            and self.current_widget.line_edit_org_name.text()
        ):
            self.button_validate.setEnabled(True)
        else:
            self.button_validate.setEnabled(False)

    def _check_custom_infos(self):
        if (
            self.current_widget.line_edit_server_addr.text()
            and self.current_widget.line_edit_secret_key.text()
            and self.current_widget.line_edit_org_name.text()
        ):
            self.button_validate.setEnabled(True)
        else:
            self.button_validate.setEnabled(False)

    @classmethod
    def exec_modal(cls, jobs_ctx, parent):
        w = cls(jobs_ctx)
        d = GreyedDialog(w, _("TEXT_ORG_WIZARD_TITLE"), parent=parent)
        w.dialog = d
        if d.exec_() == QDialog.Accepted:
            return w.status
        return None
