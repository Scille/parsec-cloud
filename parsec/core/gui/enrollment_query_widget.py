# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations
from typing import Callable

from PyQt5.QtWidgets import QWidget
from parsec._parsec import BackendPkiEnrollmentAddr, DeviceLabel
from parsec.core.gui.trio_jobs import QtToTrioJobScheduler

from parsec.core.pki import PkiEnrollmentSubmitterInitialCtx
from parsec.core.config import CoreConfig


from parsec.core.gui.custom_dialogs import GreyedDialog, show_error, ask_question
from parsec.core.gui.lang import translate
from parsec.core.gui import desktop, validators

from parsec.core.gui.ui.enrollment_query_widget import Ui_EnrollmentQueryWidget
from parsec.core.pki.exceptions import (
    PkiEnrollmentCertificatePinCodeUnavailableError,
    PkiEnrollmentCertificateNotFoundError,
    PkiEnrollmentSubmitCertificateAlreadySubmittedError,
    PkiEnrollmentSubmitCertificateEmailAlreadyUsedError,
)
from parsec.core.pki.submitter import PkiEnrollmentSubmitterSubmittedStatusCtx


class EnrollmentQueryWidget(QWidget, Ui_EnrollmentQueryWidget):
    def __init__(
        self, jobs_ctx: QtToTrioJobScheduler, config: CoreConfig, addr: BackendPkiEnrollmentAddr
    ):
        super().__init__()
        self.dialog: None | GreyedDialog = None
        self.setupUi(self)
        self.status = False
        self.jobs_ctx = jobs_ctx
        self.config = config
        self.dialog = None
        self.addr = addr
        self.label_instructions.setText(
            translate("TEXT_ENROLLMENT_INSTRUCTIONS_organization").format(
                organization=self.addr.organization_id.str
            )
        )
        self.context: PkiEnrollmentSubmitterInitialCtx | PkiEnrollmentSubmitterSubmittedStatusCtx | None = (
            None
        )
        self.label_cert_error.setVisible(False)
        self.widget_user_info.setVisible(False)
        self.button_ask_to_join.setEnabled(False)
        self.button_select_cert.clicked.connect(self._on_select_cert_clicked)
        self.button_ask_to_join.clicked.connect(self._on_ask_to_join_clicked)
        self.line_edit_device.set_validator(validators.DeviceLabelValidator())
        self.line_edit_device.textChanged.connect(self._check_infos)

    def _check_infos(self, _: object) -> None:
        self.button_ask_to_join.setEnabled(self.line_edit_device.is_input_valid())

    def _on_ask_to_join_clicked(self) -> None:
        self.jobs_ctx.submit_job(None, None, self.make_enrollment_request)

    async def make_enrollment_request(self) -> None:
        # Catch all errors
        try:
            self.button_ask_to_join.setEnabled(False)

            # Try the enrollment submission without the force flag
            try:
                assert isinstance(
                    self.context, PkiEnrollmentSubmitterInitialCtx
                ), "Context must be in its initial state (use `prepare_enrollment_request` for that)"
                self.context = await self.context.submit(
                    config_dir=self.config.config_dir,
                    requested_device_label=DeviceLabel(self.line_edit_device.text()),
                    force=False,
                )

            # This certificate has already been submitted
            except PkiEnrollmentSubmitCertificateAlreadySubmittedError:

                # Prompt for permission to force
                answer = ask_question(
                    self,
                    translate("TEXT_ENROLLMENT_SUBMIT_ALREADY_EXISTS_TITLE"),
                    translate("TEXT_ENROLLMENT_SUBMIT_ALREADY_EXISTS_QUESTION"),
                    [translate("ACTION_ENROLLMENT_FORCE"), translate("ACTION_NO")],
                    oriented_question=True,
                )

                # No permission, we're done
                if answer != translate("ACTION_ENROLLMENT_FORCE"):
                    return

                # Submit the enrollment with the force flag
                assert isinstance(
                    self.context, PkiEnrollmentSubmitterInitialCtx
                ), "Context must be in its initial state (use `prepare_enrollment_request` for that)"
                self.context = await self.context.submit(
                    config_dir=self.config.config_dir,
                    requested_device_label=DeviceLabel(self.line_edit_device.text()),
                    force=True,
                )
        except PkiEnrollmentCertificatePinCodeUnavailableError:
            # User declined to provide a PIN code (cancelled the prompt). We do nothing.
            pass
        # Email already attributed to an active user
        except PkiEnrollmentSubmitCertificateEmailAlreadyUsedError as exc:
            show_error(self, translate("TEXT_ENROLLMENT_SUBMIT_FAILED_EMAIL_USED"), exc)
        # Enrollment submission failed
        except Exception as exc:
            show_error(self, translate("TEXT_ENROLLMENT_SUBMIT_FAILED"), exc)

        # Enrollment submission is a success
        else:
            self.status = True
            if self.dialog:
                self.dialog.accept()

        # In all cases, restore the button status
        finally:
            self.button_ask_to_join.setEnabled(True)

    async def prepare_enrollment_request(self) -> None:
        try:
            self.context = await PkiEnrollmentSubmitterInitialCtx.new(self.addr)
            self.widget_user_info.setVisible(True)
            self.label_cert_error.setVisible(False)
            self.line_edit_user_name.setText(self.context.x509_certificate.subject_common_name)
            self.line_edit_user_email.setText(self.context.x509_certificate.subject_email_address)
            self.line_edit_device.setText(desktop.get_default_device())
            self.button_select_cert.setText(str(self.context.x509_certificate.certificate_id))
        except PkiEnrollmentCertificateNotFoundError:
            # User did not provide a certificate (cancelled the prompt). We do nothing.
            pass
        except PkiEnrollmentCertificatePinCodeUnavailableError:
            # User did not provide a pin code (cancelled the prompt). We do nothing.
            pass
        except Exception as exc:
            show_error(self, translate("TEXT_ENROLLMENT_ERROR_LOADING_CERTIFICATE"), exception=exc)
            self.widget_user_info.setVisible(False)
            self.label_cert_error.setText(translate("TEXT_ENROLLMENT_ERROR_LOADING_CERTIFICATE"))
            self.label_cert_error.setVisible(True)
            self.button_ask_to_join.setEnabled(False)

    def _on_select_cert_clicked(self) -> None:
        self.jobs_ctx.submit_job(None, None, self.prepare_enrollment_request)

    @classmethod
    def show_modal(
        cls,
        jobs_ctx: QtToTrioJobScheduler,
        config: CoreConfig,
        addr: BackendPkiEnrollmentAddr,
        parent: QWidget | None,
        on_finished: Callable[[], None],
    ) -> EnrollmentQueryWidget:
        widget = cls(jobs_ctx=jobs_ctx, config=config, addr=addr)
        dialog = GreyedDialog(
            widget, translate("TEXT_ENROLLMENT_QUERY_TITLE"), parent=parent, width=800
        )
        widget.dialog = dialog

        def _on_finished(result: bool) -> None:
            return on_finished()

        dialog.finished.connect(_on_finished)

        # Unlike exec_, show is asynchronous and works within the main Qt loop
        dialog.show()
        return widget
