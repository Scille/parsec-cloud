# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from pathlib import Path
from collections import namedtuple
from urllib.request import urlopen, Request
import trio

from PyQt5.QtCore import pyqtSignal, Qt
from PyQt5.QtWidgets import QWidget

from parsec.api.protocol.pki import pki_enrollment_get_reply_serializer
from parsec.core.gui.snackbar_widget import SnackbarManager
from parsec.core.gui.trio_jobs import QtToTrioJob

from parsec.core.local_device import generate_new_device, save_device_with_smartcard_in_config

from parsec.core.types import BackendOrganizationAddr

from parsec.core.local_device import list_available_devices, AvailableDevice, DeviceFileType

from parsec.core.gui.lang import translate as _
from parsec.core.gui.parsec_application import ParsecApp
from parsec.core.gui.ui.login_widget import Ui_LoginWidget
from parsec.core.gui.ui.account_button import Ui_AccountButton
from parsec.core.gui.ui.login_accounts_widget import Ui_LoginAccountsWidget
from parsec.core.gui.ui.login_password_input_widget import Ui_LoginPasswordInputWidget
from parsec.core.gui.ui.login_smartcard_input_widget import Ui_LoginSmartcardInputWidget
from parsec.core.gui.ui.login_no_devices_widget import Ui_LoginNoDevicesWidget
from parsec.core.gui.ui.enrollment_pending_button import Ui_EnrollmentPendingButton


PendingEnrollment = namedtuple("PendingEnrollment", ["request", "reply_info", "status"])


class EnrollmentPendingButton(QWidget, Ui_EnrollmentPendingButton):
    finalize_clicked = pyqtSignal(QWidget)
    clear_clicked = pyqtSignal(QWidget)

    def __init__(self, enrollment_info):
        super().__init__()
        self.setupUi(self)
        self.enrollment_info = enrollment_info
        self.label_org.setText(self.enrollment_info.request.organization_id.str)
        self.label_name.setText(self.enrollment_info.request.human_handle.label)
        if self.enrollment_info.status == "rejected":
            self.label_status.setText(_("TEXT_ENROLLMENT_STATUS_REJECTED"))
            self.label_status.setToolTip(_("TEXT_ENROLLMENT_STATUS_REJECTED_TOOLTIP"))
            self.button_action.setText(_("ACTION_ENROLLMENT_CLEAR"))
            self.button_action.clicked.connect(lambda: self.clear_clicked.emit(self))
        elif self.enrollment_info.status in ("certificate not found", "request not found"):
            self.label_status.setText(_("TEXT_ENROLLMENT_STATUS_OUTDATED"))
            self.label_status.setToolTip(_("TEXT_ENROLLMENT_STATUS_OUTDATED_TOOLTIP"))
            self.button_action.setText(_("ACTION_ENROLLMENT_CLEAR"))
            self.button_action.clicked.connect(lambda: self.clear_clicked.emit(self))
        elif self.enrollment_info.status == "accepted":
            self.label_status.setText(_("TEXT_ENROLLMENT_STATUS_ACCEPTED"))
            self.label_status.setToolTip(_("TEXT_ENROLLMENT_STATUS_ACCEPTED_TOOLTIP"))
            self.button_action.setText(_("ACTION_ENROLLMENT_FINALIZE"))
            self.button_action.clicked.connect(lambda: self.finalize_clicked.emit(self))
        else:
            self.button_action.hide()
            self.label_status.setText(_("TEXT_ENROLLMENT_STATUS_PENDING"))
            self.label_status.setToolTip(_("TEXT_ENROLLMENT_STATUS_PENDING_TOOLTIP"))

    @property
    def token(self):
        return self.enrollment_info.token


class AccountButton(QWidget, Ui_AccountButton):
    clicked = pyqtSignal(AvailableDevice)

    def __init__(self, device):
        super().__init__()
        self.setupUi(self)
        self.device = device
        self.label_device.setText(self.device.device_display)
        self.label_name.setText(self.device.short_user_display)
        self.label_organization.setText(str(self.device.organization_id))

    def mousePressEvent(self, event):
        if event.button() & Qt.LeftButton:
            self.clicked.emit(self.device)


class LoginAccountsWidget(QWidget, Ui_LoginAccountsWidget):
    account_clicked = pyqtSignal(AvailableDevice)
    pending_finalize_clicked = pyqtSignal(PendingEnrollment)
    pending_clear_clicked = pyqtSignal(PendingEnrollment)

    def __init__(self, devices, pending):
        super().__init__()
        self.setupUi(self)
        for available_device in devices:
            ab = AccountButton(available_device)
            ab.clicked.connect(self.account_clicked.emit)
            self.accounts_widget.layout().insertWidget(0, ab)
        for p in pending:
            epb = EnrollmentPendingButton(p)
            epb.finalize_clicked.connect(self._on_pending_finalize_clicked)
            epb.clear_clicked.connect(self._on_pending_clear_clicked)
            self.accounts_widget.layout().insertWidget(0, epb)

    def _on_pending_clear_clicked(self, epb):
        self.pending_clear_clicked.emit(epb.enrollment_info)

    def _on_pending_finalize_clicked(self, epb):
        self.pending_finalize_clicked.emit(epb.enrollment_info)

    def reset(self):
        pass


class LoginSmartcardInputWidget(QWidget, Ui_LoginSmartcardInputWidget):
    back_clicked = pyqtSignal()
    log_in_clicked = pyqtSignal(AvailableDevice)

    def __init__(self, device, hide_back=False):
        super().__init__()
        self.setupUi(self)
        self.device = device
        if hide_back:
            self.button_back.hide()
        self.button_back.clicked.connect(self.back_clicked.emit)
        self.button_login.clicked.connect(self._on_log_in_clicked)
        self.label_instructions.setText(
            _("TEXT_LOGIN_SELECT_SMARTCARD_INSTRUCTIONS_organization-device-user").format(
                organization=self.device.organization_id,
                user=self.device.short_user_display,
                device=self.device.device_display,
            )
        )

    def _on_log_in_clicked(self):
        self.button_login.setDisabled(True)
        self.button_login.setText(_("ACTION_LOGGING_IN"))
        self.log_in_clicked.emit(self.device)

    def keyPressEvent(self, event):
        if event.key() in (Qt.Key_Return, Qt.Key_Enter) and self.button_login.isEnabled():
            self._on_log_in_clicked()
        event.accept()

    def reset(self):
        self.button_login.setDisabled(True)
        self.button_login.setText(_("ACTION_LOG_IN"))


class LoginPasswordInputWidget(QWidget, Ui_LoginPasswordInputWidget):
    back_clicked = pyqtSignal()
    log_in_clicked = pyqtSignal(AvailableDevice, str)

    def __init__(self, device, hide_back=False):
        super().__init__()
        self.setupUi(self)
        self.device = device
        if hide_back:
            self.button_back.hide()
        self.button_back.clicked.connect(self.back_clicked.emit)
        self.button_login.clicked.connect(self._on_log_in_clicked)
        self.label_instructions.setText(
            _("TEXT_LOGIN_ENTER_PASSWORD_INSTRUCTIONS_organization-device-user").format(
                organization=self.device.organization_id,
                user=self.device.short_user_display,
                device=self.device.device_display,
            )
        )

    def _on_log_in_clicked(self):
        self.button_login.setDisabled(True)
        self.button_login.setText(_("ACTION_LOGGING_IN"))
        self.log_in_clicked.emit(self.device, self.line_edit_password.text())

    def keyPressEvent(self, event):
        if event.key() in (Qt.Key_Return, Qt.Key_Enter) and self.button_login.isEnabled():
            self._on_log_in_clicked()
        event.accept()

    def reset(self):
        self.button_login.setDisabled(False)
        self.line_edit_password.setText("")
        self.button_login.setText(_("ACTION_LOG_IN"))


class LoginNoDevicesWidget(QWidget, Ui_LoginNoDevicesWidget):
    join_organization_clicked = pyqtSignal()
    create_organization_clicked = pyqtSignal()

    def __init__(self):
        super().__init__()
        self.setupUi(self)
        if ParsecApp.connected_devices:
            self.label_no_device.setText(_("TEXT_LOGIN_NO_AVAILABLE_DEVICE"))
        else:
            self.label_no_device.setText(_("TEXT_LOGIN_NO_DEVICE_ON_MACHINE"))
        self.button_create_org.clicked.connect(self.create_organization_clicked.emit)
        self.button_join_org.clicked.connect(self.join_organization_clicked.emit)

    def reset(self):
        pass


class LoginWidget(QWidget, Ui_LoginWidget):
    login_with_password_clicked = pyqtSignal(Path, str)
    login_with_smartcard_clicked = pyqtSignal(Path)
    create_organization_clicked = pyqtSignal()
    join_organization_clicked = pyqtSignal()
    login_canceled = pyqtSignal()
    request_file_cleared = pyqtSignal(QtToTrioJob)

    def __init__(self, jobs_ctx, event_bus, config, login_failed_sig, parent):
        super().__init__(parent=parent)
        self.setupUi(self)
        self.jobs_ctx = jobs_ctx
        self.event_bus = event_bus
        self.config = config
        self.login_failed_sig = login_failed_sig

        login_failed_sig.connect(self.on_login_failed)
        self.request_file_cleared.connect(self._on_request_file_cleared)
        self.reload_devices()

    def on_login_failed(self):
        item = self.widget.layout().itemAt(0)
        if item:
            lw = item.widget()
            lw.reset()

    def keyPressEvent(self, event):
        if event.key() in (Qt.Key_Return, Qt.Key_Enter) and self.button_login.isEnabled():
            self.try_login()
        event.accept()

    async def list_pending_enrollments(self):
        from parsec_ext import smartcard

        async def _http_request(url, method="GET", data=None):
            def _do_req():
                req = Request(url=url, method=method, data=data)
                with urlopen(req) as rep:
                    return rep.read()

            return await trio.to_thread.run_sync(_do_req)

        enrollments = []
        async for path in smartcard.LocalEnrollmentRequest.iter_path(self.config.config_dir):
            if path.name.startswith(""):
                request = await smartcard.LocalEnrollmentRequest.load_from_path(path)
                url = request.backend_addr.to_http_domain_url(
                    f"/anonymous/pki/{request.organization_id}/enrollment_get_reply"
                )
                await _http_request(url)
                req_data = pki_enrollment_get_reply_serializer.req_dumps(
                    {"certificate_id": request.certificate_sha1, "request_id": request.request_id}
                )

                rep_data = await _http_request(url, "POST", req_data)
                rep = pki_enrollment_get_reply_serializer.rep_loads(rep_data)

                if rep["status"] != "ok":
                    enrollments.append(PendingEnrollment(request, None, rep["status"]))
                else:
                    reply = rep["reply"]
                    subject, reply_info = smartcard.verify_enrollment_reply(self.config, reply)
                    enrollments.append(PendingEnrollment(request, reply_info, "accepted"))

        return enrollments

    async def list_devices_and_enrollments(self):
        pending = await self.list_pending_enrollments()
        devices = [
            device
            for device in list_available_devices(self.config.config_dir)
            if not ParsecApp.is_device_connected(device.organization_id, device.device_id)
        ]
        if not len(devices) and not len(pending):
            no_device_widget = LoginNoDevicesWidget()
            no_device_widget.create_organization_clicked.connect(
                self.create_organization_clicked.emit
            )
            no_device_widget.join_organization_clicked.connect(self.join_organization_clicked.emit)
            self.widget.layout().addWidget(no_device_widget)
            no_device_widget.setFocus()
        elif len(devices) == 1 and not len(pending):
            self._on_account_clicked(devices[0], hide_back=True)
        else:
            accounts_widget = LoginAccountsWidget(devices, pending)
            accounts_widget.account_clicked.connect(self._on_account_clicked)
            accounts_widget.pending_finalize_clicked.connect(self._on_pending_finalize_clicked)
            accounts_widget.pending_clear_clicked.connect(self._on_pending_clear_clicked)
            self.widget.layout().addWidget(accounts_widget)
            accounts_widget.setFocus()

    def _on_pending_finalize_clicked(self, enrollment_info):
        from parsec_ext import smartcard

        request = enrollment_info.request
        reply_info = enrollment_info.reply_info

        private_key, signing_key = smartcard.recover_private_keys(request)

        organization_address = BackendOrganizationAddr.build(
            request.backend_addr, request.organization_id, reply_info.root_verify_key
        )
        local_device = generate_new_device(
            organization_addr=organization_address,
            device_id=reply_info.device_id,
            profile=reply_info.profile,
            human_handle=reply_info.human_handle,
            device_label=reply_info.device_label,
            signing_key=signing_key,
            private_key=private_key,
        )

        save_device_with_smartcard_in_config(
            self.config.config_dir,
            local_device,
            certificate_id=request.certificate_id,
            certificate_sha1=request.certificate_sha1,
        )
        SnackbarManager.inform(_("TEXT_CLAIM_DEVICE_SUCCESSFUL"))
        self.jobs_ctx.submit_job(self.request_file_cleared, None, self._clear_request_file, request)

    async def _clear_request_file(self, request):
        await request.get_path(self.config.config_dir).unlink()

    def _on_request_file_cleared(self):
        self.reload_devices()

    def _on_pending_clear_clicked(self, enrollment_info):
        self.jobs_ctx.submit_job(
            self.request_file_cleared, None, self._clear_request_file, enrollment_info.request
        )

    def reload_devices(self):
        self._clear_widget()
        self.jobs_ctx.submit_job(None, None, self.list_devices_and_enrollments)

    def _clear_widget(self):
        while self.widget.layout().count() != 0:
            item = self.widget.layout().takeAt(0)
            if item:
                w = item.widget()
                self.widget.layout().removeWidget(w)
                w.hide()
                w.setParent(None)

    def _on_account_clicked(self, device, hide_back=False):
        self._clear_widget()
        if device.type == DeviceFileType.PASSWORD:
            lw = LoginPasswordInputWidget(device, hide_back=hide_back)
            lw.back_clicked.connect(self._on_back_clicked)
            lw.log_in_clicked.connect(self.try_login_with_password)
            self.widget.layout().addWidget(lw)
            lw.line_edit_password.setFocus()
        elif device.type == DeviceFileType.SMARTCARD:
            lw = LoginSmartcardInputWidget(device, hide_back=hide_back)
            lw.back_clicked.connect(self._on_back_clicked)
            lw.log_in_clicked.connect(self.try_login_with_smartcard)
            self.widget.layout().addWidget(lw)

    def _on_back_clicked(self):
        self.login_canceled.emit()
        self.reload_devices()

    def try_login_with_password(self, device, password):
        self.login_with_password_clicked.emit(device.key_file_path, password)

    def try_login_with_smartcard(self, device):
        self.login_with_smartcard_clicked.emit(device.key_file_path)

    def disconnect_all(self):
        pass
