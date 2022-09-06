# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from pathlib import Path

import trio

from PyQt5.QtCore import pyqtSignal, Qt
from PyQt5.QtWidgets import QWidget

from parsec.core.pki import (
    PkiEnrollmentSubmitterSubmittedCtx,
    PkiEnrollmentSubmitterSubmittedStatusCtx,
    PkiEnrollmentSubmitterCancelledStatusCtx,
    PkiEnrollmentSubmitterRejectedStatusCtx,
    PkiEnrollmentSubmitterAcceptedStatusButBadSignatureCtx,
    PkiEnrollmentSubmitterAcceptedStatusCtx,
)
from parsec.core.pki.exceptions import (
    PkiEnrollmentCertificateValidationError,
    PkiEnrollmentCertificateNotFoundError,
    PkiEnrollmentCertificatePinCodeUnavailableError,
)
from parsec.core.pki.submitter import BasePkiEnrollmentSubmitterStatusCtx

from parsec.core.local_device import save_device_with_smartcard_in_config


from parsec.core.local_device import list_available_devices, AvailableDevice, DeviceFileType

from parsec.core.gui.lang import translate as _
from parsec.core.gui.custom_dialogs import show_error
from parsec.core.gui.parsec_application import ParsecApp
from parsec.core.gui.ui.login_widget import Ui_LoginWidget
from parsec.core.gui.snackbar_widget import SnackbarManager
from parsec.core.gui.ui.account_button import Ui_AccountButton
from parsec.core.gui.ui.login_accounts_widget import Ui_LoginAccountsWidget
from parsec.core.gui.ui.login_password_input_widget import Ui_LoginPasswordInputWidget
from parsec.core.gui.ui.login_smartcard_input_widget import Ui_LoginSmartcardInputWidget
from parsec.core.gui.ui.login_no_devices_widget import Ui_LoginNoDevicesWidget
from parsec.core.gui.ui.enrollment_pending_button import Ui_EnrollmentPendingButton


class EnrollmentPendingButton(QWidget, Ui_EnrollmentPendingButton):
    finalize_clicked = pyqtSignal(PkiEnrollmentSubmitterAcceptedStatusCtx)
    clear_clicked = pyqtSignal(BasePkiEnrollmentSubmitterStatusCtx)

    def __init__(self, config, jobs_ctx, context):
        super().__init__()
        self.setupUi(self)
        self.config = config
        self.jobs_ctx = jobs_ctx
        self.context = context
        self.label_org.setText(self.context.addr.organization_id.str)
        self.label_name.setText(self.context.x509_certificate.subject_common_name)
        self.button_action.hide()
        self.label_status.setText(_("TEXT_ENROLLMENT_RETRIEVING_STATUS"))
        self.label_status.setToolTip("")
        self.jobs_ctx.submit_job(None, None, self._get_pending_enrollment_infos)

    async def _get_pending_enrollment_infos(self):
        try:
            while True:
                new_context = None
                try:
                    new_context = await self.context.poll(
                        extra_trust_roots=self.config.pki_extra_trust_roots
                    )
                except Exception:
                    self.label_status.setText(_("TEXT_ENROLLMENT_STATUS_CANNOT_RETRIEVE"))
                    self.label_status.setToolTip(
                        _("TEXT_ENROLLMENT_STATUS_CANNOT_RETRIEVE_TOOLTIP")
                    )
                    self.button_action.hide()
                else:
                    if isinstance(new_context, PkiEnrollmentSubmitterSubmittedStatusCtx):
                        self.button_action.hide()
                        self.label_status.setText(_("TEXT_ENROLLMENT_STATUS_PENDING"))
                        self.label_status.setToolTip(_("TEXT_ENROLLMENT_STATUS_PENDING_TOOLTIP"))
                    elif isinstance(new_context, PkiEnrollmentSubmitterCancelledStatusCtx):
                        self.context = new_context
                        self.label_status.setText(_("TEXT_ENROLLMENT_STATUS_OUTDATED"))
                        self.label_status.setToolTip(_("TEXT_ENROLLMENT_STATUS_OUTDATED_TOOLTIP"))
                        self.button_action.setText(_("ACTION_ENROLLMENT_CLEAR"))
                        self.button_action.clicked.connect(
                            lambda: self.clear_clicked.emit(self.context)
                        )
                        self.button_action.show()
                        return
                    elif isinstance(new_context, PkiEnrollmentSubmitterRejectedStatusCtx):
                        self.context = new_context
                        self.label_status.setText(_("TEXT_ENROLLMENT_STATUS_REJECTED"))
                        self.label_status.setToolTip(_("TEXT_ENROLLMENT_STATUS_REJECTED_TOOLTIP"))
                        self.button_action.setText(_("ACTION_ENROLLMENT_CLEAR"))
                        self.button_action.clicked.connect(
                            lambda: self.clear_clicked.emit(self.context)
                        )
                        self.button_action.show()
                        return
                    elif isinstance(
                        new_context, PkiEnrollmentSubmitterAcceptedStatusButBadSignatureCtx
                    ):
                        if isinstance(new_context.error, PkiEnrollmentCertificateValidationError):
                            self.label_status.setText(_("TEXT_ENROLLMENT_STATUS_VALIDATION_FAILED"))
                            self.label_status.setToolTip(
                                _("TEXT_ENROLLMENT_STATUS_VALIDATION_FAILED_TOOLTIP")
                            )
                            self.button_action.hide()
                            return
                        else:
                            self.label_status.setText(_("TEXT_ENROLLMENT_STATUS_ERROR_WITH_ACCEPT"))
                            self.label_status.setToolTip(
                                _("TEXT_ENROLLMENT_STATUS_ERROR_WITH_ACCEPT_TOOLTIP")
                            )
                            self.button_action.setText(_("ACTION_ENROLLMENT_CLEAR"))
                            self.button_action.clicked.connect(
                                lambda: self.clear_clicked.emit(self.context)
                            )
                            self.button_action.show()
                            return
                    else:
                        assert isinstance(new_context, PkiEnrollmentSubmitterAcceptedStatusCtx)
                        self.context = new_context
                        self.label_status.setText(_("TEXT_ENROLLMENT_STATUS_ACCEPTED"))
                        self.label_status.setToolTip(_("TEXT_ENROLLMENT_STATUS_ACCEPTED_TOOLTIP"))
                        self.button_action.setText(_("ACTION_ENROLLMENT_FINALIZE"))
                        self.button_action.show()
                        self.button_action.clicked.connect(
                            lambda: self.finalize_clicked.emit(self.context)
                        )
                        return
                await trio.sleep(10.0)
        except RuntimeError:
            # In some rare cases when closing the app, we may try to access a QWidget when the underlying C++
            # object has already been deleted. Catching the exception just avoid printing an ugly message
            # and reporting it to sentry.
            pass


class AccountButton(QWidget, Ui_AccountButton):
    clicked = pyqtSignal(AvailableDevice)

    def __init__(self, device):
        super().__init__()
        self.setupUi(self)
        self.device = device
        self.label_device.setText(self.device.device_display)
        self.label_name.setText(self.device.short_user_display)
        self.label_organization.setText(self.device.organization_id.str)

    def mousePressEvent(self, event):
        if event.button() & Qt.LeftButton:
            self.clicked.emit(self.device)


class LoginAccountsWidget(QWidget, Ui_LoginAccountsWidget):
    account_clicked = pyqtSignal(AvailableDevice)
    pending_finalize_clicked = pyqtSignal(PkiEnrollmentSubmitterAcceptedStatusCtx)
    pending_clear_clicked = pyqtSignal(BasePkiEnrollmentSubmitterStatusCtx)

    def __init__(self, config, jobs_ctx, devices, pendings):
        super().__init__()
        self.setupUi(self)
        for available_device in devices:
            ab = AccountButton(available_device)
            ab.clicked.connect(self.account_clicked.emit)
            self.accounts_widget.layout().insertWidget(
                self.accounts_widget.layout().count() - 1, ab
            )
        for ctx in pendings:
            epb = EnrollmentPendingButton(config, jobs_ctx, ctx)
            epb.finalize_clicked.connect(self._on_pending_finalize_clicked)
            epb.clear_clicked.connect(self._on_pending_clear_clicked)
            self.accounts_widget.layout().insertWidget(0, epb)

    def _on_pending_clear_clicked(self, pending):
        self.pending_clear_clicked.emit(pending)

    def _on_pending_finalize_clicked(self, pending):
        self.pending_finalize_clicked.emit(pending)

    def clear(self):
        while self.accounts_widget.layout().count() != 1:
            item = self.accounts_widget.layout().takeAt(0)
            if item and item.widget():
                item.widget().hide()
                item.widget().setParent(None)


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
                organization=self.device.organization_id.str,
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
        self.button_login.setDisabled(False)
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
                organization=self.device.organization_id.str,
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
    recover_device_clicked = pyqtSignal()

    def __init__(self):
        super().__init__()
        self.setupUi(self)
        if ParsecApp.connected_devices:
            self.label_no_device.setText(_("TEXT_LOGIN_NO_AVAILABLE_DEVICE"))
        else:
            self.label_no_device.setText(_("TEXT_LOGIN_NO_DEVICE_ON_MACHINE"))
        self.button_create_org.clicked.connect(self.create_organization_clicked.emit)
        self.button_join_org.clicked.connect(self.join_organization_clicked.emit)
        self.button_recover_device.clicked.connect(self.recover_device_clicked.emit)

    def reset(self):
        pass


class LoginWidget(QWidget, Ui_LoginWidget):
    login_with_password_clicked = pyqtSignal(Path, str)
    login_with_smartcard_clicked = pyqtSignal(Path)
    create_organization_clicked = pyqtSignal()
    join_organization_clicked = pyqtSignal()
    recover_device_clicked = pyqtSignal()
    login_canceled = pyqtSignal()

    def __init__(self, jobs_ctx, event_bus, config, login_failed_sig, parent):
        super().__init__(parent=parent)
        self.setupUi(self)
        self.jobs_ctx = jobs_ctx
        self.event_bus = event_bus
        self.config = config
        self.login_failed_sig = login_failed_sig

        login_failed_sig.connect(self.on_login_failed)
        self.reload_devices()

    def on_login_failed(self):
        item = self.widget.layout().itemAt(0)
        if item:
            lw = item.widget()
            lw.reset()

    def list_devices_and_enrollments(self):
        pendings = PkiEnrollmentSubmitterSubmittedCtx.list_from_disk(
            config_dir=self.config.config_dir
        )
        devices = [
            device
            for device in list_available_devices(self.config.config_dir)
            if not ParsecApp.is_device_connected(device.organization_id, device.device_id)
        ]
        if not len(devices) and not len(pendings):
            no_device_widget = LoginNoDevicesWidget()
            no_device_widget.create_organization_clicked.connect(
                self.create_organization_clicked.emit
            )
            no_device_widget.join_organization_clicked.connect(self.join_organization_clicked.emit)
            no_device_widget.recover_device_clicked.connect(self.recover_device_clicked.emit)
            self.widget.layout().addWidget(no_device_widget)
            no_device_widget.setFocus()
        elif len(devices) == 1 and not len(pendings):
            self._on_account_clicked(devices[0], hide_back=True)
        else:
            # If the GUI has a last used device, we look for it in our devices list
            # and insert it to the front, so it will be shown first
            if self.config.gui_last_device:
                last_used = next(
                    (d for d in devices if d.device_id.str == self.config.gui_last_device), None
                )
                if last_used:
                    devices.remove(last_used)
                    devices.insert(0, last_used)
            accounts_widget = LoginAccountsWidget(self.config, self.jobs_ctx, devices, pendings)
            accounts_widget.account_clicked.connect(self._on_account_clicked)
            accounts_widget.pending_finalize_clicked.connect(self._on_pending_finalize_clicked)
            accounts_widget.pending_clear_clicked.connect(self._on_pending_clear_clicked)
            self.widget.layout().addWidget(accounts_widget)
            accounts_widget.setFocus()

    def _on_pending_finalize_clicked(self, context: PkiEnrollmentSubmitterAcceptedStatusCtx):
        self.jobs_ctx.submit_job(None, None, self._finalize_enrollment, context)

    async def _finalize_enrollment(self, context):
        try:
            context = await context.finalize()
        except PkiEnrollmentCertificateNotFoundError:
            # Nothing to do, the user cancelled the certificate selection prompt
            return
        except PkiEnrollmentCertificatePinCodeUnavailableError:
            # Nothing to do, the user cancelled the pin code prompt
            return
        except Exception as exc:
            show_error(self, _("TEXT_ENROLLMENT_CANNOT_FINALIZE"), exception=exc)
            return

        try:
            await save_device_with_smartcard_in_config(
                config_dir=self.config.config_dir,
                device=context.new_device,
                certificate_id=context.x509_certificate.certificate_id,
                certificate_sha1=context.x509_certificate.certificate_sha1,
            )
        except Exception as exc:
            show_error(self, _("TEXT_CANNOT_SAVE_DEVICE"), exception=exc)
            return

        await self._remove_enrollment(context)
        SnackbarManager.inform(_("TEXT_CLAIM_DEVICE_SUCCESSFUL"))

    async def _remove_enrollment(self, context):
        try:
            await context.remove_from_disk()
        except Exception as exc:
            show_error(self, _("TEXT_CANNOT_REMOVE_LOCAL_PENDING_ENROLLMENT"), exception=exc)
            return
        self.reload_devices()

    def _on_pending_clear_clicked(self, context: BasePkiEnrollmentSubmitterStatusCtx):
        self.jobs_ctx.submit_job(None, None, self._remove_enrollment, context)

    def reload_devices(self):
        self._clear_widget()
        self.list_devices_and_enrollments()

    def _clear_widget(self):
        while self.widget.layout().count() != 0:
            item = self.widget.layout().takeAt(0)
            if item:
                w = item.widget()
                if isinstance(w, LoginAccountsWidget):
                    w.clear()
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
