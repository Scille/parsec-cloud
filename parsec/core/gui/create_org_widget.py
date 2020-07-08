# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
from structlog import get_logger
import trio
import json
from urllib.request import Request, urlopen
from urllib.error import URLError
from http.client import HTTPException

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget, QDialog

from parsec.api.protocol import OrganizationID
from parsec.core.backend_connection import (
    apiv1_backend_administration_cmds_factory,
    BackendConnectionError,
)
from parsec.core.types import BackendOrganizationBootstrapAddr, BackendAddr

from parsec.core.gui.custom_dialogs import GreyedDialog, show_error
from parsec.core.gui.trio_thread import JobResultError, ThreadSafeQtSignal
from parsec.core.gui.lang import translate as _

from parsec.core.gui.ui.create_org_widget import Ui_CreateOrgWidget
from parsec.core.gui.ui.create_org_chose_action_widget import Ui_CreateOrgChoseActionWidget
from parsec.core.gui.ui.create_org_website_widget import Ui_CreateOrgWebsiteWidget
from parsec.core.gui.ui.create_org_custom_widget import Ui_CreateOrgCustomWidget


logger = get_logger()


async def _do_create_website(email, organization_id):
    data = json.dumps({"email": email, "organization_id": str(organization_id)}).encode("utf-8")
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
            return (email, BackendOrganizationBootstrapAddr.from_url(content["bootstrap_link"]))
        except (TypeError, KeyError) as exc:
            raise JobResultError("invalid_response", exc=exc)
    except (HTTPException, URLError) as exc:
        raise JobResultError("offline", exc=exc)


async def _do_create_custom(organization_id, server_url, admin_token):
    try:
        async with apiv1_backend_administration_cmds_factory(server_url, admin_token) as cmds:
            rep = await cmds.organization_create(organization_id, None)
            if rep["status"] != "ok":
                raise JobResultError(rep["status"])
            bootstrap_token = rep["bootstrap_token"]
            return BackendOrganizationBootstrapAddr.build(
                server_url, organization_id, bootstrap_token
            )
    except BackendConnectionError as exc:
        if "Invalid url format" in str(exc):
            raise JobResultError("invalid-url")
        else:
            raise JobResultError("offline")


class CreateOrgChoseActionWidget(QWidget, Ui_CreateOrgChoseActionWidget):
    def __init__(self):
        super().__init__()
        self.setupUi(self)
        self.radio_create_website.setChecked(True)


class CreateOrgWebsiteWidget(QWidget, Ui_CreateOrgWebsiteWidget):
    def __init__(self):
        super().__init__()
        self.setupUi(self)


class CreateOrgCustomWidget(QWidget, Ui_CreateOrgCustomWidget):
    def __init__(self):
        super().__init__()
        self.setupUi(self)


class CreateOrgWidget(QWidget, Ui_CreateOrgWidget):
    create_website_success = pyqtSignal()
    create_website_error = pyqtSignal()
    create_custom_success = pyqtSignal()
    create_custom_error = pyqtSignal()

    def __init__(self, jobs_ctx):
        super().__init__()
        self.setupUi(self)
        self.jobs_ctx = jobs_ctx
        self.create_website_job = None
        self.create_custom_job = None
        self.dialog = None
        self.status = (None, None)
        self.button_validate.clicked.connect(self._validate_clicked)
        self.button_previous.clicked.connect(self._previous_clicked)
        self.button_previous.hide()
        self.current_widget = CreateOrgChoseActionWidget()
        self.main_layout.addWidget(self.current_widget)
        self.button_validate.setEnabled(True)
        self.create_website_success.connect(self._on_create_website_success)
        self.create_website_error.connect(self._on_create_website_error)
        self.create_custom_success.connect(self._on_create_custom_success)
        self.create_custom_error.connect(self._on_create_custom_error)

    def _clear_page(self):
        item = self.main_layout.takeAt(0)
        if item:
            w = item.widget()
            self.main_layout.removeWidget(w)
            w.hide()
            w.setParent(None)
        self.current_widget = None

    def on_close(self):
        self.status = None
        if self.create_website_job:
            self.create_website_job.cancel_and_join()
        if self.create_custom_job:
            self.create_custom_job.cancel_and_join()

    def _on_create_custom_success(self):
        assert self.create_custom_job
        assert self.create_custom_job.is_finished()
        assert self.create_custom_job.status == "ok"

        self.status = (None, self.create_custom_job.ret)
        self.create_custom_job = None
        self.dialog.accept()

    def _on_create_custom_error(self):
        assert self.create_custom_job
        assert self.create_custom_job.is_finished()
        assert self.create_custom_job.status != "ok"

        status = self.create_custom_job.status

        if status == "cancelled":
            return

        errmsg = None
        if status == "organization_already_exists":
            errmsg = _("TEXT_ORG_WIZARD_ORGANIZATION_ALREADY_EXISTS")
        elif status == "invalid_organization_id":
            errmsg = _("TEXT_ORG_WIZARD_INVALID_ORGANIZATION_ID")
        elif status == "invalid-url":
            errmsg = _("TEXT_ORG_WIZARD_INVALID_BACKEND_ADDR")
        elif status == "offline":
            errmsg = _("TEXT_ORG_WIZARD_OFFLINE")
        else:
            errmsg = _("TEXT_ORG_WIZARD_UNKNOWN_FAILURE")
        exc = self.create_custom_job.exc
        if exc.params.get("exc"):
            exc = exc.params.get("exc")
        show_error(self, errmsg, exception=exc)
        self.create_custom_job = None
        self.button_validate.setEnabled(True)
        self.button_previous.show()

    def _on_create_website_success(self):
        assert self.create_website_job
        assert self.create_website_job.is_finished()
        assert self.create_website_job.status == "ok"

        self.status = self.create_website_job.ret
        self.create_website_job = None
        self.dialog.accept()

    def _on_create_website_error(self):
        assert self.create_website_job
        assert self.create_website_job.is_finished()
        assert self.create_website_job.status != "ok"

        status = self.create_website_job.status

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
        self.create_website_job = None
        self.button_validate.setEnabled(True)
        self.button_previous.show()

    def _validate_clicked(self):
        if isinstance(self.current_widget, CreateOrgChoseActionWidget):
            if self.current_widget.radio_create_custom.isChecked():
                self._clear_page()
                self.current_widget = CreateOrgCustomWidget()
                self.main_layout.addWidget(self.current_widget)
                self.current_widget.line_edit_org_name.textChanged.connect(self._check_custom_infos)
                self.current_widget.line_edit_server_url.textChanged.connect(
                    self._check_custom_infos
                )
                self.current_widget.line_edit_admin_token.textChanged.connect(
                    self._check_custom_infos
                )
                self.button_validate.setEnabled(False)
                self.button_previous.show()
            else:
                self._clear_page()
                self.current_widget = CreateOrgWebsiteWidget()
                self.main_layout.addWidget(self.current_widget)
                self.current_widget.line_edit_user_email.textChanged.connect(
                    self._check_website_infos
                )
                self.current_widget.line_edit_org_name.textChanged.connect(
                    self._check_website_infos
                )
                self.current_widget.check_accept_contract.clicked.connect(self._check_website_infos)
                self.button_validate.setEnabled(False)
                self.button_previous.show()
        elif isinstance(self.current_widget, CreateOrgWebsiteWidget):
            org_id = None
            try:
                org_id = OrganizationID(self.current_widget.line_edit_org_name.text())
            except ValueError as exc:
                show_error(self, _("TEXT_ORG_WIZARD_INVALID_ORGANIZATION_ID"), exception=exc)
                return
            self.create_custom_job = self.jobs_ctx.submit_job(
                ThreadSafeQtSignal(self, "create_custom_success"),
                ThreadSafeQtSignal(self, "create_custom_error"),
                _do_create_website,
                email=self.current_widget.line_edit_user_email.text(),
                organization_id=org_id,
            )
            self.button_validate.setEnabled(False)
            self.button_previous.hide()
        elif isinstance(self.current_widget, CreateOrgCustomWidget):
            org_id = None
            backend_addr = None
            try:
                org_id = OrganizationID(self.current_widget.line_edit_org_name.text())
            except ValueError as exc:
                show_error(self, _("TEXT_ORG_WIZARD_INVALID_ORGANIZATION_ID"), exception=exc)
                return
            try:
                backend_addr = BackendAddr.from_url(self.current_widget.line_edit_server_url.text())
            except ValueError as exc:
                show_error(self, _("TEXT_ORG_WIZARD_INVALID_BACKEND_ADDR"), exception=exc)
                return

            self.create_custom_job = self.jobs_ctx.submit_job(
                ThreadSafeQtSignal(self, "create_custom_success"),
                ThreadSafeQtSignal(self, "create_custom_error"),
                _do_create_custom,
                organization_id=org_id,
                server_url=backend_addr,
                admin_token=self.current_widget.line_edit_admin_token.text(),
            )
            self.button_validate.setEnabled(False)
            self.button_previous.hide()

    def _previous_clicked(self):
        self._clear_page()
        self.current_widget = CreateOrgChoseActionWidget()
        self.main_layout.addWidget(self.current_widget)
        self.button_previous.hide()
        self.button_validate.setText(_("ACTION_NEXT"))
        self.button_validate.setEnabled(True)

    def _check_website_infos(self):
        if (
            self.current_widget.line_edit_user_email.text()
            and self.current_widget.line_edit_org_name.text()
            and self.current_widget.check_accept_contract.isChecked()
        ):
            self.button_validate.setEnabled(True)
        else:
            self.button_validate.setEnabled(False)

    def _check_custom_infos(self):
        if (
            self.current_widget.line_edit_org_name.text()
            and self.current_widget.line_edit_server_url.text()
            and self.current_widget.line_edit_admin_token.text()
        ):
            self.button_validate.setEnabled(True)
        else:
            self.button_validate.setEnabled(False)

    @classmethod
    def exec_modal(cls, jobs_ctx, parent):
        w = cls(jobs_ctx)
        d = GreyedDialog(w, _("TEXT_ORG_WIZARD_TITLE"), parent=parent, width=1000)
        w.dialog = d
        if d.exec_() == QDialog.Accepted:
            return w.status
        return None, None
