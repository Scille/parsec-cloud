# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import trio
from enum import IntEnum
from structlog import get_logger
from PyQt5.QtCore import pyqtSignal
from PyQt5.QtGui import QFont
from PyQt5.QtWidgets import QWidget

from parsec.api.protocol import DeviceLabel, HumanHandle
from parsec.core.types import LocalDevice
from parsec.core.local_device import (
    save_device_with_password_in_config,
    save_device_with_smartcard_in_config,
    DeviceFileType,
    LocalDeviceCryptoError,
    LocalDeviceError,
    LocalDeviceNotFoundError,
)
from parsec.core.fs.storage.user_storage import user_storage_non_speculative_init
from parsec.core.invite import claimer_retrieve_info, InvitePeerResetError
from parsec.core.backend_connection import (
    backend_invited_cmds_factory,
    BackendConnectionRefused,
    BackendInvitationAlreadyUsed,
    BackendNotAvailable,
    BackendOutOfBallparkError,
)
from parsec.core.gui import validators
from parsec.core.gui.trio_jobs import JobResultError, QtToTrioJob
from parsec.core.gui.desktop import get_default_device
from parsec.core.gui.custom_dialogs import show_error, GreyedDialog, show_info
from parsec.core.gui.lang import translate as _
from parsec.core.gui.ui.claim_user_widget import Ui_ClaimUserWidget
from parsec.core.gui.ui.claim_user_code_exchange_widget import Ui_ClaimUserCodeExchangeWidget
from parsec.core.gui.ui.claim_user_provide_info_widget import Ui_ClaimUserProvideInfoWidget
from parsec.core.gui.ui.claim_user_instructions_widget import Ui_ClaimUserInstructionsWidget
from parsec.core.gui.ui.claim_user_finalize_widget import Ui_ClaimUserFinalizeWidget


logger = get_logger()


class Claimer:
    class Step(IntEnum):
        RetrieveInfo = 1
        WaitPeer = 2
        GetGreeterSas = 3
        SignifyTrust = 4
        GetClaimerSas = 5
        WaitPeerTrust = 6
        ClaimUser = 7

    def __init__(self):
        self.main_oob_send, self.main_oob_recv = trio.open_memory_channel(0)
        self.job_oob_send, self.job_oob_recv = trio.open_memory_channel(0)

    async def run(self, addr, config):
        try:
            async with backend_invited_cmds_factory(
                addr=addr, keepalive=config.backend_connection_keepalive
            ) as cmds:
                # Trigger handshake
                await cmds.ping()
                r = await self.main_oob_recv.receive()

                assert r == self.Step.RetrieveInfo
                try:
                    initial_ctx = await claimer_retrieve_info(cmds)
                    await self.job_oob_send.send((True, None, initial_ctx.claimer_email))
                except Exception as exc:
                    await self.job_oob_send.send((False, exc, None))

                r = await self.main_oob_recv.receive()

                assert r == self.Step.WaitPeer
                try:
                    in_progress_ctx = await initial_ctx.do_wait_peer()
                    await self.job_oob_send.send((True, None))
                except Exception as exc:
                    await self.job_oob_send.send((False, exc))

                r = await self.main_oob_recv.receive()

                assert r == self.Step.GetGreeterSas
                try:
                    choices = in_progress_ctx.generate_greeter_sas_choices(size=4)
                    await self.job_oob_send.send((True, None, in_progress_ctx.greeter_sas, choices))
                except Exception as exc:
                    await self.job_oob_send.send((False, exc, None, None))

                r = await self.main_oob_recv.receive()

                assert r == self.Step.SignifyTrust
                try:
                    in_progress_ctx = await in_progress_ctx.do_signify_trust()
                    await self.job_oob_send.send((True, None))
                except Exception as exc:
                    await self.job_oob_send.send((False, exc))

                r = await self.main_oob_recv.receive()

                assert r == self.Step.GetClaimerSas
                await self.job_oob_send.send(in_progress_ctx.claimer_sas)

                r = await self.main_oob_recv.receive()

                assert r == self.Step.WaitPeerTrust
                try:
                    in_progress_ctx = await in_progress_ctx.do_wait_peer_trust()
                    await self.job_oob_send.send((True, None))
                except Exception as exc:
                    await self.job_oob_send.send((False, exc))

                r = await self.main_oob_recv.receive()
                assert r == self.Step.ClaimUser

                try:
                    device_label, human_handle = await self.main_oob_recv.receive()

                    new_device = await in_progress_ctx.do_claim_user(
                        requested_device_label=device_label, requested_human_handle=human_handle
                    )
                    # Claiming a user means we are it first device, hence we know there
                    # is no existing user manifest (hence our placeholder is non-speculative)
                    await user_storage_non_speculative_init(
                        data_base_dir=config.data_base_dir, device=new_device
                    )
                    await self.job_oob_send.send((True, None, new_device))
                except Exception as exc:
                    await self.job_oob_send.send((False, exc, None))
        except BackendNotAvailable as exc:
            raise JobResultError(status="backend-not-available", origin=exc)
        except BackendInvitationAlreadyUsed as exc:
            raise JobResultError(status="invitation-already-used", origin=exc)
        except BackendConnectionRefused as exc:
            raise JobResultError(status="invitation-not-found", origin=exc)
        except BackendOutOfBallparkError as exc:
            raise JobResultError(status="out-of-ballpark", origin=exc)

    async def retrieve_info(self):
        await self.main_oob_send.send(self.Step.RetrieveInfo)
        r, exc, user_email = await self.job_oob_recv.receive()
        if not r:
            raise JobResultError(status="retrieve-info-failed", origin=exc)
        return user_email

    async def wait_peer(self):
        await self.main_oob_send.send(self.Step.WaitPeer)
        r, exc = await self.job_oob_recv.receive()
        if not r:
            raise JobResultError(status="wait-peer-failed", origin=exc)

    async def get_greeter_sas(self):
        await self.main_oob_send.send(self.Step.GetGreeterSas)
        r, exc, greeter_sas, choices = await self.job_oob_recv.receive()
        if not r:
            raise JobResultError(status="get-greeter-sas-failed", origin=exc)
        return greeter_sas, choices

    async def signify_trust(self):
        await self.main_oob_send.send(self.Step.SignifyTrust)
        r, exc = await self.job_oob_recv.receive()
        if not r:
            raise JobResultError(status="signify-trust-failed", origin=exc)

    async def get_claimer_sas(self):
        await self.main_oob_send.send(self.Step.GetClaimerSas)
        claimer_sas = await self.job_oob_recv.receive()
        return claimer_sas

    async def wait_peer_trust(self):
        await self.main_oob_send.send(self.Step.WaitPeerTrust)
        r, exc = await self.job_oob_recv.receive()
        if not r:
            raise JobResultError(status="wait-trust-failed", origin=exc)

    async def claim_user(self, device_label: DeviceLabel, human_handle: HumanHandle):
        await self.main_oob_send.send(self.Step.ClaimUser)
        await self.main_oob_send.send((device_label, human_handle))
        r, exc, new_device = await self.job_oob_recv.receive()
        if not r:
            raise JobResultError(status="claim-user-failed", origin=exc)
        return new_device


class ClaimUserFinalizeWidget(QWidget, Ui_ClaimUserFinalizeWidget):
    succeeded = pyqtSignal(LocalDevice, DeviceFileType, str)
    failed = pyqtSignal(object)  # QtToTrioJob or None

    def __init__(self, config, jobs_ctx, new_device):
        super().__init__()
        self.setupUi(self)
        self.config = config
        self.jobs_ctx = jobs_ctx
        self.new_device = new_device

        self.widget_auth.exclude_strings(
            [
                new_device.organization_addr.organization_id.str,
                new_device.device_label.str,
                new_device.human_handle.email,
                new_device.human_handle.label,
            ]
        )
        self.widget_auth.authentication_state_changed.connect(self.check_infos)
        self.button_finalize.setDisabled(True)
        self.button_finalize.clicked.connect(self._on_finalize_clicked)

    def check_infos(self, _=""):
        if self.widget_auth.is_auth_valid():
            self.button_finalize.setDisabled(False)
        else:
            self.button_finalize.setDisabled(True)

    async def _on_finalize_clicked(self):
        try:
            if self.widget_auth.get_auth_method() == DeviceFileType.PASSWORD:
                save_device_with_password_in_config(
                    config_dir=self.config.config_dir,
                    device=self.new_device,
                    password=self.widget_auth.get_auth(),
                )
            elif self.widget_auth.get_auth_method() == DeviceFileType.SMARTCARD:
                await save_device_with_smartcard_in_config(
                    config_dir=self.config.config_dir, device=self.new_device
                )
            self.succeeded.emit(
                self.new_device, self.widget_auth.get_auth_method(), self.widget_auth.get_auth()
            )
        except LocalDeviceCryptoError as exc:
            if self.widget_auth.get_auth_method() == DeviceFileType.SMARTCARD:
                show_error(self, _("TEXT_INVALID_SMARTCARD"), exception=exc)
        except LocalDeviceNotFoundError as exc:
            if self.widget_auth.get_auth_method() == DeviceFileType.PASSWORD:
                show_error(self, _("TEXT_CANNOT_SAVE_DEVICE"), exception=exc)
        except LocalDeviceError as exc:
            show_error(self, _("TEXT_CANNOT_SAVE_DEVICE"), exception=exc)


class ClaimUserCodeExchangeWidget(QWidget, Ui_ClaimUserCodeExchangeWidget):
    succeeded = pyqtSignal()
    failed = pyqtSignal(object)  # QtToTrioJob or None

    signify_trust_success = pyqtSignal(QtToTrioJob)
    signify_trust_error = pyqtSignal(QtToTrioJob)

    wait_peer_trust_success = pyqtSignal(QtToTrioJob)
    wait_peer_trust_error = pyqtSignal(QtToTrioJob)

    get_greeter_sas_success = pyqtSignal(QtToTrioJob)
    get_greeter_sas_error = pyqtSignal(QtToTrioJob)

    get_claimer_sas_success = pyqtSignal(QtToTrioJob)
    get_claimer_sas_error = pyqtSignal(QtToTrioJob)

    def __init__(self, jobs_ctx, claimer):
        super().__init__()
        self.setupUi(self)
        self.claimer = claimer
        self.jobs_ctx = jobs_ctx

        self.signify_trust_job = None
        self.wait_peer_trust_job = None
        self.get_greeter_sas_job = None
        self.get_claimer_sas_job = None

        self.widget_claimer_code.hide()

        font = self.line_edit_claimer_code.font()
        font.setBold(True)
        font.setLetterSpacing(QFont.PercentageSpacing, 180)
        self.line_edit_claimer_code.setFont(font)

        self.code_input_widget.good_code_clicked.connect(self._on_good_greeter_code_clicked)
        self.code_input_widget.wrong_code_clicked.connect(self._on_wrong_greeter_code_clicked)
        self.code_input_widget.none_clicked.connect(self._on_none_clicked)

        self.get_greeter_sas_success.connect(self._on_get_greeter_sas_success)
        self.get_greeter_sas_error.connect(self._on_get_greeter_sas_error)
        self.get_claimer_sas_success.connect(self._on_get_claimer_sas_success)
        self.get_claimer_sas_error.connect(self._on_get_claimer_sas_error)
        self.signify_trust_success.connect(self._on_signify_trust_success)
        self.signify_trust_error.connect(self._on_signify_trust_error)
        self.wait_peer_trust_success.connect(self._on_wait_peer_trust_success)
        self.wait_peer_trust_error.connect(self._on_wait_peer_trust_error)

        self.get_greeter_sas_job = self.jobs_ctx.submit_job(
            self.get_greeter_sas_success, self.get_greeter_sas_error, self.claimer.get_greeter_sas
        )

    def _on_good_greeter_code_clicked(self):
        self.widget_greeter_code.setDisabled(True)
        self.signify_trust_job = self.jobs_ctx.submit_job(
            self.signify_trust_success, self.signify_trust_error, self.claimer.signify_trust
        )

    def _on_wrong_greeter_code_clicked(self):
        show_error(self, _("TEXT_CLAIM_USER_INVALID_CODE_CLICKED"))
        self.failed.emit(None)

    def _on_none_clicked(self):
        show_info(self, _("TEXT_CLAIM_USER_NONE_CODE_CLICKED"))
        self.failed.emit(None)

    def _on_get_greeter_sas_success(self, job):
        if job != self.get_greeter_sas_job:
            return
        self.get_greeter_sas_job = None
        assert job
        assert job.is_finished()
        assert job.status == "ok"
        greeter_sas, choices = job.ret
        self.code_input_widget.set_choices(choices, greeter_sas)

    def _on_get_greeter_sas_error(self, job):
        if job != self.get_greeter_sas_job:
            return
        self.get_greeter_sas_job = None
        assert job
        assert job.is_finished()
        assert job.status != "ok"
        if job.status != "cancelled":
            exc = None
            msg = _("TEXT_CLAIM_USER_GET_GREETER_SAS_ERROR")
            if job.exc:
                exc = job.exc.params.get("origin", None)
                if isinstance(exc, InvitePeerResetError):
                    msg = _("TEXT_CLAIM_USER_PEER_RESET")
                elif isinstance(exc, BackendInvitationAlreadyUsed):
                    msg = _("TEXT_INVITATION_ALREADY_USED")
            show_error(self, msg, exception=exc)
        self.failed.emit(job)

    def _on_get_claimer_sas_success(self, job):
        assert self.get_claimer_sas_job
        assert self.get_claimer_sas_job.is_finished()
        assert self.get_claimer_sas_job.status == "ok"
        claimer_sas = self.get_claimer_sas_job.ret
        self.get_claimer_sas_job = None
        self.widget_greeter_code.setVisible(False)
        self.widget_claimer_code.setVisible(True)
        self.line_edit_claimer_code.setText(claimer_sas.str)
        self.wait_peer_trust_job = self.jobs_ctx.submit_job(
            self.wait_peer_trust_success, self.wait_peer_trust_error, self.claimer.wait_peer_trust
        )

    def _on_get_claimer_sas_error(self, job):
        if job != self.get_claimer_sas_job:
            return
        self.get_claimer_sas_job = None
        assert job
        assert job.is_finished()
        assert job.status != "ok"
        self.failed.emit(job)

    def _on_signify_trust_success(self, job):
        assert self.signify_trust_job
        assert self.signify_trust_job.is_finished()
        assert self.signify_trust_job.status == "ok"
        self.signify_trust_job = None
        self.get_claimer_sas_job = self.jobs_ctx.submit_job(
            self.get_claimer_sas_success, self.get_claimer_sas_error, self.claimer.get_claimer_sas
        )

    def _on_signify_trust_error(self, job):
        if job != self.signify_trust_job:
            return
        self.signify_trust_job = None
        assert job
        assert job.is_finished()
        assert job.status != "ok"
        if job.status != "cancelled":
            exc = None
            msg = _("TEXT_CLAIM_USER_SIGNIFY_TRUST_ERROR")
            if job.exc:
                exc = job.exc.params.get("origin", None)
                if isinstance(exc, InvitePeerResetError):
                    msg = _("TEXT_CLAIM_USER_PEER_RESET")
                elif isinstance(exc, BackendInvitationAlreadyUsed):
                    msg = _("TEXT_INVITATION_ALREADY_USED")
            show_error(self, msg, exception=exc)
        self.failed.emit(job)

    def _on_wait_peer_trust_success(self, job):
        if job != self.wait_peer_trust_job:
            return
        self.wait_peer_trust_job = None
        assert job
        assert job.is_finished()
        assert job.status == "ok"
        self.succeeded.emit()

    def _on_wait_peer_trust_error(self, job):
        if job != self.wait_peer_trust_job:
            return
        self.wait_peer_trust_job = None
        assert job
        assert job.is_finished()
        assert job.status != "ok"
        if job.status != "cancelled":
            exc = None
            msg = _("TEXT_CLAIM_USER_WAIT_PEER_TRUST_ERROR")
            if job.exc:
                exc = job.exc.params.get("origin", None)
                if isinstance(exc, InvitePeerResetError):
                    msg = _("TEXT_CLAIM_USER_PEER_RESET")
                elif isinstance(exc, BackendInvitationAlreadyUsed):
                    msg = _("TEXT_INVITATION_ALREADY_USED")
            show_error(self, msg, exception=exc)
        self.failed.emit(job)

    def cancel(self):
        if self.signify_trust_job:
            self.signify_trust_job.cancel()
        if self.wait_peer_trust_job:
            self.wait_peer_trust_job.cancel()
        if self.get_claimer_sas_job:
            self.get_claimer_sas_job.cancel()
        if self.get_greeter_sas_job:
            self.get_greeter_sas_job.cancel()


class ClaimUserProvideInfoWidget(QWidget, Ui_ClaimUserProvideInfoWidget):
    succeeded = pyqtSignal()
    failed = pyqtSignal(object)  # QtToTrioJob or None

    claim_success = pyqtSignal(QtToTrioJob)
    claim_error = pyqtSignal(QtToTrioJob)

    def __init__(self, jobs_ctx, claimer, user_email):
        super().__init__()
        self.setupUi(self)
        self.jobs_ctx = jobs_ctx
        self.claimer = claimer
        self.claim_job = None
        self.new_device = None
        self.line_edit_user_full_name.setFocus()
        self.line_edit_user_full_name.set_validator(validators.NotEmptyValidator())
        self.line_edit_user_full_name.validity_changed.connect(self.check_infos)

        self.line_edit_user_email.setText(user_email)
        self.line_edit_user_email.set_validator(validators.EmailValidator())
        self.line_edit_user_email.validity_changed.connect(self.check_infos)

        self.line_edit_device.setText(get_default_device())
        self.line_edit_device.set_validator(validators.DeviceLabelValidator())
        self.line_edit_device.validity_changed.connect(self.check_infos)

        self.claim_success.connect(self._on_claim_success)
        self.claim_error.connect(self._on_claim_error)
        self.label_wait.hide()
        self.button_ok.clicked.connect(self._on_claim_clicked)
        self.check_infos()

    def check_infos(self, _=None):
        if (
            self.line_edit_user_full_name.is_input_valid()
            and self.line_edit_user_email.is_input_valid()
            and self.line_edit_device.is_input_valid()
        ):
            self.button_ok.setDisabled(False)
        else:
            self.button_ok.setDisabled(True)

    def _on_claim_clicked(self):
        # No try/except given `self.line_edit_device` has already been validated against `DeviceLabel`
        device_label = DeviceLabel(validators.trim_user_name(self.line_edit_device.text()))
        try:
            user_name = validators.trim_user_name(self.line_edit_user_full_name.text())
            human_handle = HumanHandle(email=self.line_edit_user_email.text(), label=user_name)
            device_label = DeviceLabel(self.line_edit_device.text())
        except ValueError as exc:
            show_error(self, _("TEXT_CLAIM_USER_INVALID_HUMAN_HANDLE"), exception=exc)
            return
        self.button_ok.setDisabled(True)
        self.widget_info.setDisabled(True)
        self.label_wait.show()
        self.claim_job = self.jobs_ctx.submit_job(
            self.claim_success,
            self.claim_error,
            self.claimer.claim_user,
            device_label=device_label,
            human_handle=human_handle,
        )

    def _on_claim_success(self, job):
        if job != self.claim_job:
            return
        self.claim_job = None
        assert job
        assert job.is_finished()
        assert job.status == "ok"
        self.new_device = job.ret

        self.succeeded.emit()

    def _on_claim_error(self, job):
        if job != self.claim_job:
            return
        self.claim_job = None
        assert job
        assert job.is_finished()
        assert job.status != "ok"
        if job.status != "cancelled":
            exc = None
            msg = _("TEXT_CLAIM_USER_CLAIM_ERROR")
            if job.exc:
                exc = job.exc.params.get("origin", None)
                if isinstance(exc, InvitePeerResetError):
                    msg = _("TEXT_CLAIM_USER_PEER_RESET")
                elif isinstance(exc, BackendInvitationAlreadyUsed):
                    msg = _("TEXT_INVITATION_ALREADY_USED")
            show_error(self, msg, exception=exc)
        self.check_infos()
        self.widget_info.setDisabled(False)
        self.label_wait.hide()
        self.failed.emit(job)

    def cancel(self):
        if self.claim_job:
            self.claim_job.cancel()


class ClaimUserInstructionsWidget(QWidget, Ui_ClaimUserInstructionsWidget):
    succeeded = pyqtSignal()
    failed = pyqtSignal(object)  # QtToTrioJob or None

    wait_peer_success = pyqtSignal(QtToTrioJob)
    wait_peer_error = pyqtSignal(QtToTrioJob)

    def __init__(self, jobs_ctx, claimer, user_email):
        super().__init__()
        self.setupUi(self)
        self.jobs_ctx = jobs_ctx
        self.claimer = claimer
        self.wait_peer_job = None
        self.button_start.clicked.connect(self._on_button_start_clicked)
        self.wait_peer_success.connect(self._on_wait_peer_success)
        self.wait_peer_error.connect(self._on_wait_peer_error)

    def switch_to_info_retrieved(self):
        self.label.setText(_("TEXT_CLAIM_USER_INSTRUCTIONS"))
        self.button_start.setEnabled(True)
        self.widget_spinner.setVisible(False)

    def _on_button_start_clicked(self):
        self.button_start.setDisabled(True)
        self.button_start.setText(_("TEXT_CLAIM_USER_WAITING"))
        self.wait_peer_job = self.jobs_ctx.submit_job(
            self.wait_peer_success, self.wait_peer_error, self.claimer.wait_peer
        )

    def _on_wait_peer_success(self, job):
        if job != self.wait_peer_job:
            return
        self.wait_peer_job = None
        assert job
        assert job.is_finished()
        assert job.status == "ok"
        self.succeeded.emit()

    def _on_wait_peer_error(self, job):
        if job != self.wait_peer_job:
            return
        self.wait_peer_job = None
        assert job
        assert job.is_finished()
        assert job.status != "ok"
        if job.status != "cancelled":
            exc = None
            msg = _("TEXT_CLAIM_USER_WAIT_PEER_ERROR")
            if job.exc:
                exc = job.exc.params.get("origin", None)
                if isinstance(exc, InvitePeerResetError):
                    msg = _("TEXT_CLAIM_USER_PEER_RESET")
                elif isinstance(exc, BackendInvitationAlreadyUsed):
                    msg = _("TEXT_INVITATION_ALREADY_USED")
            self.button_start.setDisabled(False)
            self.button_start.setText(_("ACTION_START"))
            show_error(self, msg, exception=exc)
        self.failed.emit(job)

    def cancel(self):
        if self.wait_peer_job:
            self.wait_peer_job.cancel()


class ClaimUserWidget(QWidget, Ui_ClaimUserWidget):
    claimer_success = pyqtSignal(QtToTrioJob)
    claimer_error = pyqtSignal(QtToTrioJob)
    retrieve_info_success = pyqtSignal(QtToTrioJob)
    retrieve_info_error = pyqtSignal(QtToTrioJob)

    def __init__(self, jobs_ctx, config, addr):
        super().__init__()
        self.setupUi(self)
        self.jobs_ctx = jobs_ctx
        self.config = config
        self.dialog = None
        self.addr = addr
        self.status = None
        self.user_email = None
        self.claimer_job = None
        self.retrieve_info_job = None
        self.claimer_success.connect(self._on_claimer_success)
        self.claimer_error.connect(self._on_claimer_error)
        self.retrieve_info_success.connect(self._on_retrieve_info_success)
        self.retrieve_info_error.connect(self._on_retrieve_info_error)
        self.claimer = Claimer()
        self._run_claimer()
        self._goto_page1()

    def _run_claimer(self):
        self.claimer_job = self.jobs_ctx.submit_job(
            self.claimer_success,
            self.claimer_error,
            self.claimer.run,
            addr=self.addr,
            config=self.config,
        )
        self.retrieve_info_job = self.jobs_ctx.submit_job(
            self.retrieve_info_success, self.retrieve_info_error, self.claimer.retrieve_info
        )

    def _on_retrieve_info_success(self, job):
        if self.retrieve_info_job is not job:
            return
        assert self.retrieve_info_job
        assert self.retrieve_info_job.is_finished()
        assert self.retrieve_info_job.status == "ok"
        self.user_email = self.retrieve_info_job.ret
        self.retrieve_info_job = None
        current_page = self.main_layout.itemAt(0).widget()
        current_page.switch_to_info_retrieved()

    def _on_retrieve_info_error(self, job):
        if self.retrieve_info_job is not job:
            return
        self.retrieve_info_job = None
        assert job
        assert job.is_finished()
        assert job.status != "ok"
        if job.status != "cancelled":
            exc = None
            msg = _("TEXT_CLAIM_USER_FAILED_TO_RETRIEVE_INFO")
            if job.exc:
                exc = job.exc.params.get("origin", None)
                if isinstance(exc, InvitePeerResetError):
                    msg = _("TEXT_CLAIM_USER_PEER_RESET")
                elif isinstance(exc, BackendInvitationAlreadyUsed):
                    msg = _("TEXT_INVITATION_ALREADY_USED")
            show_error(self, msg, exception=exc)
        # No point in retrying the process here, simply close the dialog
        self.dialog.reject()

    def _on_page_failed_force_reject(self, job):
        # The dialog has already been rejected
        if not self.isVisible():
            return
        # Do not retry after a failure on page 4, simply close the dialog
        self.dialog.reject()

    def _on_page_failed(self, job):
        # The dialog has already been rejected
        if not self.isVisible():
            return
        # No reason to restart the process if cancelled, simply close the dialog
        if job is not None and job.status == "cancelled":
            self.dialog.reject()
            return
        # No reason to restart the process if offline or if the invitation has been deleted
        # simply close the dialog
        if job is not None and isinstance(
            job.exc.params.get("origin", None), (BackendNotAvailable, BackendConnectionRefused)
        ):
            self.dialog.reject()
            return
        # Let's try one more time with the same dialog
        self.restart()

    def restart(self):
        self.cancel()
        self.status = None
        self.claimer = Claimer()
        self._run_claimer()
        self._goto_page1()

    def _goto_page1(self):
        item = self.main_layout.takeAt(0)
        if item:
            current_page = item.widget()
            if current_page:
                current_page.hide()
                current_page.setParent(None)
        page = ClaimUserInstructionsWidget(self.jobs_ctx, self.claimer, self.user_email)
        page.succeeded.connect(self._goto_page2)
        page.failed.connect(self._on_page_failed)
        self.main_layout.insertWidget(0, page)

    def _goto_page2(self):
        current_page = self.main_layout.takeAt(0).widget()
        current_page.hide()
        current_page.setParent(None)
        page = ClaimUserCodeExchangeWidget(self.jobs_ctx, self.claimer)
        page.succeeded.connect(self._goto_page3)
        page.failed.connect(self._on_page_failed)
        self.main_layout.insertWidget(0, page)

    def _goto_page3(self):
        current_page = self.main_layout.takeAt(0).widget()
        current_page.hide()
        current_page.setParent(None)
        page = ClaimUserProvideInfoWidget(self.jobs_ctx, self.claimer, self.user_email)
        page.succeeded.connect(self._goto_page4)
        page.failed.connect(self._on_page_failed)
        self.main_layout.insertWidget(0, page)

    def _goto_page4(self):
        current_page = self.main_layout.itemAt(0).widget()
        assert current_page
        new_device = current_page.new_device
        current_page.hide()
        current_page.setParent(None)
        page = ClaimUserFinalizeWidget(self.config, self.jobs_ctx, new_device)
        page.succeeded.connect(self._on_finished)
        page.failed.connect(self._on_page_failed_force_reject)
        self.main_layout.insertWidget(0, page)

    def _on_finished(self, device, auth_method, password):
        self.status = (device, auth_method, password)
        self.dialog.accept()

    def _on_claimer_success(self, job):
        if self.claimer_job is not job:
            return
        assert self.claimer_job
        assert self.claimer_job.is_finished()
        assert self.claimer_job.status == "ok"
        self.claimer_job = None

    def _on_claimer_error(self, job):
        assert job
        assert job.is_finished()
        assert job.status != "ok"
        # This callback can be called after the creation of a new claimer job in the case
        # of a restart, due to Qt signals being called later.
        if job.status == "cancelled":
            return
        # Safety net for concurrency issues
        if self.claimer_job is not job:
            return
        self.claimer_job = None
        msg = ""
        exc = None
        if job.status == "invitation-not-found":
            msg = _("TEXT_CLAIM_USER_INVITATION_NOT_FOUND")
        elif job.status == "invitation-already-used":
            msg = _("TEXT_INVITATION_ALREADY_USED")
        elif job.status == "backend-not-available":
            msg = _("TEXT_INVITATION_BACKEND_NOT_AVAILABLE")
        elif job.status == "out-of-ballpark":
            msg = _("TEXT_BACKEND_STATE_DESYNC")
        else:
            msg = _("TEXT_CLAIM_USER_UNKNOWN_ERROR")
        if job.exc:
            exc = job.exc.params.get("origin", None)
        show_error(self, msg, exception=exc)
        # No point in retrying since the claimer job itself failed, simply close the dialog
        self.dialog.reject()

    def cancel(self):
        item = self.main_layout.itemAt(0)
        if item:
            current_page = item.widget()
            if current_page and getattr(current_page, "cancel", None):
                current_page.cancel()
        if self.retrieve_info_job:
            self.retrieve_info_job.cancel()
        if self.claimer_job:
            self.claimer_job.cancel()

    def on_close(self):
        self.cancel()

    @classmethod
    def show_modal(cls, jobs_ctx, config, addr, parent, on_finished):
        w = cls(jobs_ctx=jobs_ctx, config=config, addr=addr)
        d = GreyedDialog(w, _("TEXT_CLAIM_USER_TITLE"), parent=parent, width=800)
        w.dialog = d

        d.finished.connect(on_finished)
        # Unlike exec_, show is asynchronous and works within the main Qt loop
        d.show()
        return w
