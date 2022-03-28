# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from collections import namedtuple

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtGui import QIcon, QColor
from PyQt5.QtWidgets import QWidget

from parsec.api.protocol import UserProfile

from parsec.core.backend_connection.authenticated import backend_authenticated_cmds_factory

from parsec.core.core_events import CoreEvent

from parsec.core.invite.greeter import _create_new_user_certificates

from parsec.core.gui.lang import translate
from parsec.core.gui.custom_widgets import Pixmap
from parsec.core.gui.snackbar_widget import SnackbarManager

from parsec.core.gui.ui.enrollment_widget import Ui_EnrollmentWidget
from parsec.core.gui.ui.enrollment_button import Ui_EnrollmentButton


EnrollmentInfo = namedtuple(
    "EnrollmentInfo", ["request", "request_id", "request_info", "certificate_id", "certif_is_valid"]
)


class EnrollmentButton(QWidget, Ui_EnrollmentButton):
    accept_clicked = pyqtSignal(QWidget)
    reject_clicked = pyqtSignal(QWidget)

    def __init__(self, enrollment_info):
        super().__init__()
        self.setupUi(self)
        self.enrollment_info = enrollment_info
        self.label_name.setText(self.enrollment_info.request.requested_human_handle.label)
        self.label_email.setText(self.enrollment_info.request.requested_human_handle.email)
        # self.label_date.setText(self.enrollment_info.date)
        accept_pix = Pixmap(":/icons/images/material/done.svg")
        accept_pix.replace_color(QColor(0x00, 0x00, 0x00), QColor(0xFF, 0xFF, 0xFF))
        reject_pix = Pixmap(":/icons/images/material/clear.svg")
        reject_pix.replace_color(QColor(0x00, 0x00, 0x00), QColor(0xFF, 0xFF, 0xFF))
        self.button_accept.setIcon(QIcon(accept_pix))
        self.button_reject.setIcon(QIcon(reject_pix))

        if self.enrollment_info.certif_is_valid:
            self.button_accept.setVisible(True)
            self.label_certif.setStyleSheet("color: #8BC34A;")
            self.label_certif.setText("✔ " + translate("TEXT_ENROLLMENT_CERTIFICATE_IS_VALID"))
        else:
            self.button_accept.setVisible(False)
            self.label_certif.setStyleSheet("color: #F44336;")
            self.label_certif.setText("✘ " + translate("TEXT_ENROLLMENT_CERTIFICATE_IS_INVALID"))
        self.button_accept.clicked.connect(lambda: self.accept_clicked.emit(self))
        self.button_reject.clicked.connect(lambda: self.reject_clicked.emit(self))

    @property
    def token(self):
        return self.enrollment_info.token

    def set_buttons_enabled(self, enabled):
        self.button_accept.setEnabled(enabled)
        self.button_reject.setEnabled(enabled)


class EnrollmentWidget(QWidget, Ui_EnrollmentWidget):
    def __init__(self, core, jobs_ctx, event_bus, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.core = core
        self.jobs_ctx = jobs_ctx
        self.label_empty_list.hide()
        self.event_bus = event_bus

    def showEvent(self, event):
        self.reset()
        return
        self.event_bus.connect(CoreEvent.NEW_RECRUIT, self._on_new_recruit)
        self.event_bus.connect(CoreEvent.RECRUIT_UPDATED, self._on_recruit_updated)
        self.event_bus.connect(CoreEvent.RECRUIT_ACCEPTED, self._on_recruit_accepted)
        self.event_bus.connect(CoreEvent.RECRUIT_REJECTED, self._on_recruit_rejected)

    def hideEvent(self, event):
        return
        try:
            self.event_bus.disconnect(CoreEvent.NEW_RECRUIT, self._on_new_recruit)
            self.event_bus.disconnect(CoreEvent.RECRUIT_UPDATED, self._on_recruit_updated)
            self.event_bus.disconnect(CoreEvent.RECRUIT_ACCEPTED, self._on_recruit_accepted)
            self.event_bus.disconnect(CoreEvent.RECRUIT_REJECTED, self._on_recruit_rejected)
        except ValueError:
            pass

    def reset(self):
        self.jobs_ctx.submit_job(None, None, self.list_pending_enrollments)

    def clear_layout(self):
        while self.main_layout.count() != 0:
            item = self.main_layout.takeAt(0)
            if item and item.widget():
                w = item.widget()
                self.main_layout.removeWidget(w)
                w.hide()
                w.setParent(None)

    async def list_pending_enrollments(self):
        from parsec_ext import smartcard

        self.label_empty_list.hide()
        self.clear_layout()

        async with backend_authenticated_cmds_factory(
            addr=self.core.device.organization_addr,
            device_id=self.core.device.device_id,
            signing_key=self.core.device.signing_key,
            keepalive=self.core.config.backend_connection_keepalive,
        ) as cmds:
            rep = await cmds.pki_enrollment_get_requests()
            if rep["status"] != "ok":
                self.label_empty_list.setText(translate("TEXT_ENROLLMENT_FAILED_TO_RETRIEVE_LIST"))
                self.label_empty_list.show()
                return

            requests = []
            for certificate_id, request_id, request in rep["requests"]:
                try:
                    subject, request_info = smartcard.verify_enrollment_request(
                        self.core.config, request
                    )
                    requests.append(
                        EnrollmentInfo(request, request_id, request_info, certificate_id, True)
                    )
                except smartcard.LocalDeviceError:
                    requests.append(
                        EnrollmentInfo(request, request_id, None, certificate_id, False)
                    )

            if len(requests) == 0:
                self.label_empty_list.setText(translate("TEXT_ENROLLMENT_NO_PENDING_enrollment"))
                self.label_empty_list.show()
                return
            for recruit in requests:
                eb = EnrollmentButton(recruit)
                eb.accept_clicked.connect(self._on_accept_clicked)
                eb.reject_clicked.connect(self._on_reject_clicked)
                self.main_layout.addWidget(eb)

    def _on_accept_clicked(self, rw):
        rw.set_buttons_enabled(False)
        self.jobs_ctx.submit_job(None, None, self.accept_recruit, enrollment_button=rw)

    def _on_reject_clicked(self, rw):
        rw.set_buttons_enabled(False)
        self.jobs_ctx.submit_job(None, None, self.reject_recruit, enrollment_button=rw)

    async def accept_recruit(self, enrollment_button):
        from parsec_ext import smartcard

        try:
            async with backend_authenticated_cmds_factory(
                addr=self.core.device.organization_addr,
                device_id=self.core.device.device_id,
                signing_key=self.core.device.signing_key,
                keepalive=self.core.config.backend_connection_keepalive,
            ) as cmds:

                enrollment_info = enrollment_button.enrollment_info
                author = self.core.device
                user_certificate, redacted_user_certificate, device_certificate, redacted_device_certificate, user_confirmation = _create_new_user_certificates(
                    author,
                    enrollment_info.request_info.requested_device_label,
                    enrollment_info.request_info.requested_human_handle,
                    UserProfile.STANDARD,
                    enrollment_info.request_info.public_key,
                    enrollment_info.request_info.verify_key,
                )

                reply = smartcard.prepare_enrollment_reply(author, user_confirmation)

                # Perform the pki_enrollment_reply command
                rep = await cmds.pki_enrollment_reply(
                    enrollment_info.certificate_id,
                    enrollment_info.request_id,
                    reply=reply,
                    user_id=user_confirmation.device_id.user_id,
                    user_certificate=user_certificate,
                    device_certificate=device_certificate,
                    redacted_user_certificate=redacted_user_certificate,
                    redacted_device_certificate=redacted_device_certificate,
                )
                if rep["status"] != "ok":
                    SnackbarManager.warn(translate("TEXT_ENROLLMENT_ACCEPT_FAILURE"))
                    enrollment_button.set_buttons_enabled(True)
                else:
                    SnackbarManager.inform(translate("TEXT_ENROLLMENT_ACCEPT_SUCCESS"))
                    self.main_layout.removeWidget(enrollment_button)
        except:
            import traceback

            traceback.print_exc()
            SnackbarManager.warn(translate("TEXT_ENROLLMENT_ACCEPT_FAILURE"))
            enrollment_button.set_buttons_enabled(True)

    async def reject_recruit(self, enrollment_button):
        try:
            async with backend_authenticated_cmds_factory(
                addr=self.core.device.organization_addr,
                device_id=self.core.device.device_id,
                signing_key=self.core.device.signing_key,
                keepalive=self.core.config.backend_connection_keepalive,
            ) as cmds:
                rep = await cmds.pki_enrollment_reply(
                    enrollment_button.enrollment_info.certificate_id,
                    enrollment_button.enrollment_info.request_id,
                    reply=None,
                    user_id=None,
                )
                if rep["status"] != "ok":
                    SnackbarManager.warn(translate("TEXT_ENROLLMENT_REJECT_FAILURE"))
                    enrollment_button.set_buttons_enabled(True)
                else:
                    SnackbarManager.inform(translate("TEXT_ENROLLMENT_REJECT_SUCCESS"))
                    self.main_layout.removeWidget(enrollment_button)
        except:
            SnackbarManager.warn(translate("TEXT_ENROLLMENT_REJECT_FAILURE"))
            enrollment_button.set_buttons_enabled(True)
