# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import textwrap

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtGui import QIcon, QColor
from PyQt5.QtWidgets import QWidget

from parsec.api.protocol import UserProfile, HumanHandle, DeviceLabel
from parsec.core.backend_connection.exceptions import BackendConnectionError
from parsec.core.gui.trio_jobs import QtToTrioJob

from parsec.core.types import BackendPkiEnrollmentAddr

from parsec.core.core_events import CoreEvent

from parsec.core.pki import (
    PkiEnrollementAccepterValidSubmittedCtx,
    PkiEnrollementAccepterInvalidSubmittedCtx,
)
from parsec.core.pki.exceptions import PkiEnrollmentListError

from parsec.core.gui.custom_dialogs import GreyedDialog
from parsec.core.gui import validators, desktop
from parsec.core.gui.lang import translate, format_datetime
from parsec.core.gui.custom_widgets import Pixmap
from parsec.core.gui.snackbar_widget import SnackbarManager

from parsec.core.gui.ui.enrollment_widget import Ui_EnrollmentWidget
from parsec.core.gui.ui.enrollment_button import Ui_EnrollmentButton
from parsec.core.gui.ui.greet_user_check_info_widget import Ui_GreetUserCheckInfoWidget


class AcceptCheckInfoWidget(QWidget, Ui_GreetUserCheckInfoWidget):
    succeeded = pyqtSignal()
    failed = pyqtSignal(object)

    def __init__(self, pending, user_profile_outsider_allowed=False):
        super().__init__()
        self.setupUi(self)

        self.label_waiting.hide()

        self.line_edit_user_full_name.validity_changed.connect(self.check_infos)
        self.line_edit_user_full_name.set_validator(validators.UserNameValidator())
        self.line_edit_user_email.validity_changed.connect(self.check_infos)
        self.line_edit_user_email.set_validator(validators.EmailValidator())
        self.line_edit_device.validity_changed.connect(self.check_infos)
        self.line_edit_device.set_validator(validators.DeviceLabelValidator())
        self.combo_profile.currentIndexChanged.connect(self.check_infos)

        self.line_edit_user_full_name.setText(
            pending.submitter_x509_certificate.subject_common_name
        )
        self.line_edit_user_email.setText(pending.submitter_x509_certificate.subject_email_address)
        self.line_edit_device.setText(pending.submit_payload.requested_device_label.str)

        self.combo_profile.addItem(translate("TEXT_SELECT_USER_PROFILE"), None)
        self.combo_profile.addItem(translate("TEXT_USER_PROFILE_OUTSIDER"), UserProfile.OUTSIDER)
        self.combo_profile.addItem(translate("TEXT_USER_PROFILE_STANDARD"), UserProfile.STANDARD)
        self.combo_profile.addItem(translate("TEXT_USER_PROFILE_ADMIN"), UserProfile.ADMIN)

        item = self.combo_profile.model().item(2)
        item.setToolTip(translate("TEXT_USER_PROFILE_STANDARD_TOOLTIP"))
        item = self.combo_profile.model().item(3)
        item.setToolTip(translate("TEXT_USER_PROFILE_ADMIN_TOOLTIP"))

        if not user_profile_outsider_allowed:
            item = self.combo_profile.model().item(1)
            item.setEnabled(False)
            item.setToolTip(translate("NOT_ALLOWED_OUTSIDER_PROFILE_TOOLTIP"))
        else:
            item = self.combo_profile.model().item(1)
            item.setToolTip(translate("TEXT_USER_PROFILE_OUTSIDER_TOOLTIP"))

        # No profile by default, forcing the greeter to choose one
        self.combo_profile.setCurrentIndex(0)

        self.button_create_user.clicked.connect(self._on_create_user_clicked)

    def check_infos(self, _=None):
        if (
            self.line_edit_user_full_name.is_input_valid()
            and self.line_edit_device.is_input_valid()
            and self.line_edit_user_email.is_input_valid()
            and self.combo_profile.currentIndex() != 0
        ):
            self.button_create_user.setDisabled(False)
        else:
            self.button_create_user.setDisabled(True)

    @property
    def device_label(self):
        return DeviceLabel(self.line_edit_device.text())

    @property
    def profile(self):
        return self.combo_profile.currentData()

    @property
    def human_handle(self):
        return HumanHandle(
            label=self.line_edit_user_full_name.clean_text(), email=self.line_edit_user_email.text()
        )

    def _on_create_user_clicked(self):
        self.dialog.accept()

    @classmethod
    def show_modal(cls, enrollment_info, parent, on_finished, user_profile_outsider_allowed):
        w = cls(enrollment_info, user_profile_outsider_allowed)
        d = GreyedDialog(
            w, translate("TEXT_ENROLLMENT_ACCEPT_CHECK_INFO_TITLE"), parent=parent, width=800
        )
        w.dialog = d

        def _on_finished(result):
            return on_finished(result, w)

        d.finished.connect(_on_finished)

        # Unlike exec_, show is asynchronous and works within the main Qt loop
        d.show()
        return w


class EnrollmentButton(QWidget, Ui_EnrollmentButton):
    accept_clicked = pyqtSignal(QWidget)
    reject_clicked = pyqtSignal(QWidget)

    def __init__(self, pending):
        super().__init__()
        self.setupUi(self)
        self.pending = pending
        accept_pix = Pixmap(":/icons/images/material/done.svg")
        accept_pix.replace_color(QColor(0x00, 0x00, 0x00), QColor(0xFF, 0xFF, 0xFF))
        reject_pix = Pixmap(":/icons/images/material/clear.svg")
        reject_pix.replace_color(QColor(0x00, 0x00, 0x00), QColor(0xFF, 0xFF, 0xFF))
        self.button_accept.setIcon(QIcon(accept_pix))
        self.button_reject.setIcon(QIcon(reject_pix))
        self.label_date.setText(format_datetime(pending.submitted_on))

        if isinstance(self.pending, PkiEnrollementAccepterInvalidSubmittedCtx):
            if self.pending.submitter_x509_certificate:
                self.widget_cert_infos.setVisible(True)
                self.widget_cert_error.setVisible(False)
                self.label_name.setText(self.pending.submitter_x509_certificate.subject_common_name)
                self.label_email.setText(
                    self.pending.submitter_x509_certificate.subject_email_address
                )
                self.label_issuer.setText(
                    self.pending.submitter_x509_certificate.issuer_common_name
                )
            else:
                self.widget_cert_infos.setVisible(False)
                self.widget_cert_error.setVisible(True)
                self.label_error.setText(translate("TEXT_ENROLLMENT_ERROR_WITH_CERTIFICATE"))
            self.button_accept.setVisible(False)
            self.label_cert_validity.setStyleSheet("#label_cert_validity { color: #F44336; }")
            self.label_cert_validity.setText(
                "✘ " + translate("TEXT_ENROLLMENT_CERTIFICATE_IS_INVALID")
            )
            self.label_cert_validity.setToolTip(textwrap.fill(str(self.pending.error), 80))
        else:
            assert isinstance(self.pending, PkiEnrollementAccepterValidSubmittedCtx)
            self.widget_cert_infos.setVisible(True)
            self.widget_cert_error.setVisible(False)
            self.button_accept.setVisible(True)
            self.label_name.setText(self.pending.submitter_x509_certificate.subject_common_name)
            self.label_email.setText(self.pending.submitter_x509_certificate.subject_email_address)
            self.label_issuer.setText(self.pending.submitter_x509_certificate.issuer_common_name)
            self.label_cert_validity.setStyleSheet("#label_cert_validity { color: #8BC34A; }")
            self.label_cert_validity.setText(
                "✔ " + translate("TEXT_ENROLLMENT_CERTIFICATE_IS_VALID")
            )

        self.button_accept.clicked.connect(lambda: self.accept_clicked.emit(self))
        self.button_reject.clicked.connect(lambda: self.reject_clicked.emit(self))

    def set_buttons_enabled(self, enabled):
        self.button_accept.setEnabled(enabled)
        self.button_reject.setEnabled(enabled)


class EnrollmentWidget(QWidget, Ui_EnrollmentWidget):
    list_success = pyqtSignal(QtToTrioJob)
    list_failure = pyqtSignal(QtToTrioJob)

    def __init__(self, core, jobs_ctx, event_bus, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.core = core
        self.jobs_ctx = jobs_ctx
        self.organization_config = self.core.get_organization_config()
        self.label_empty_list.hide()
        self.event_bus = event_bus
        self.button_get_enrollment_addr.apply_style()
        self.button_get_enrollment_addr.clicked.connect(self._on_get_enrollment_addr_clicked)
        self.current_job = None

    def _on_get_enrollment_addr_clicked(self):
        ba = BackendPkiEnrollmentAddr.build(
            self.core.device.organization_addr.get_backend_addr(),
            self.core.device.organization_addr.organization_id,
        )
        desktop.copy_to_clipboard(ba.to_url())
        SnackbarManager.inform(translate("TEXT_ENROLLMENT_ADDR_COPIED_TO_CLIPBOARD"))

    def showEvent(self, _):
        self.event_bus.connect(CoreEvent.PKI_ENROLLMENTS_UPDATED, self._on_updated)
        self.reset()

    def hideEvent(self, _):
        try:
            self.event_bus.disconnect(CoreEvent.PKI_ENROLLMENTS_UPDATED, self._on_updated)
        except ValueError:
            pass

    def _on_updated(self, _):
        self.reset()

    def reset(self):
        if self.current_job is not None and not self.current_job.is_finished():
            return
        self.current_job = self.jobs_ctx.submit_job(
            self.list_success, self.list_failure, self.list_pending_enrollments
        )

    def clear_layout(self):
        while self.main_layout.count() != 0:
            item = self.main_layout.takeAt(0)
            if item and item.widget():
                w = item.widget()
                self.main_layout.removeWidget(w)
                w.hide()
                w.setParent(None)

    async def list_pending_enrollments(self):
        self.label_empty_list.hide()
        self.clear_layout()

        pendings = []
        try:
            pendings = await self.core.list_submitted_enrollment_requests()
        except (PkiEnrollmentListError, BackendConnectionError):
            self.label_empty_list.setText(translate("TEXT_ENROLLMENT_FAILED_TO_RETRIEVE_PENDING"))
            self.label_empty_list.show()
            return
        if not pendings:
            self.label_empty_list.setText(translate("TEXT_ENROLLMENT_NO_PENDING_ENROLLMENT"))
            self.label_empty_list.show()
            return
        for pending in pendings:
            eb = EnrollmentButton(pending)
            eb.accept_clicked.connect(self._on_accept_clicked)
            eb.reject_clicked.connect(self._on_reject_clicked)
            self.main_layout.addWidget(eb)

    def _on_accept_clicked(self, eb):
        def _on_finished(status, dialog):
            if not status:
                eb.set_buttons_enabled(True)
            else:
                self.jobs_ctx.submit_job(
                    None,
                    None,
                    self.accept_recruit,
                    enrollment_button=eb,
                    human_handle=dialog.human_handle,
                    device_label=dialog.device_label,
                    profile=dialog.profile,
                )

        eb.set_buttons_enabled(False)
        AcceptCheckInfoWidget.show_modal(
            eb.pending,
            self,
            on_finished=_on_finished,
            user_profile_outsider_allowed=self.organization_config.user_profile_outsider_allowed,
        )

    def _on_reject_clicked(self, rw):
        rw.set_buttons_enabled(False)
        self.jobs_ctx.submit_job(None, None, self.reject_recruit, enrollment_button=rw)

    async def accept_recruit(self, enrollment_button, human_handle, device_label, profile):
        try:
            await enrollment_button.pending.accept(
                author=self.core.device,
                device_label=device_label,
                human_handle=human_handle,
                profile=profile,
            )
        except Exception:
            SnackbarManager.warn(translate("TEXT_ENROLLMENT_ACCEPT_FAILURE"))
            enrollment_button.set_buttons_enabled(True)
        else:
            SnackbarManager.inform(translate("TEXT_ENROLLMENT_ACCEPT_SUCCESS"))
            self.main_layout.removeWidget(enrollment_button)
            enrollment_button.hide()
            enrollment_button.setParent(None)

    async def reject_recruit(self, enrollment_button):
        try:
            await enrollment_button.pending.reject()
        except Exception:
            SnackbarManager.warn(translate("TEXT_ENROLLMENT_REJECT_FAILURE"))
            enrollment_button.set_buttons_enabled(True)
        else:
            SnackbarManager.inform(translate("TEXT_ENROLLMENT_REJECT_SUCCESS"))
            self.main_layout.removeWidget(enrollment_button)
            enrollment_button.hide()
            enrollment_button.setParent(None)
