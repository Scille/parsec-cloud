# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations
from typing import Any, Callable

import trio
from enum import IntEnum
from PyQt5.QtCore import pyqtSignal
from PyQt5.QtGui import QFont
from PyQt5.QtWidgets import QDialog, QWidget

from parsec._parsec import SASCode
from parsec.api.protocol import DeviceLabel, HumanHandle, InvitationToken, UserProfile
from parsec.core.backend_connection import BackendNotAvailable
from parsec.core.invite import (
    InviteError,
    InvitePeerResetError,
    InviteActiveUsersLimitReachedError,
    InviteAlreadyUsedError,
    UserGreetInProgress2Ctx,
)
from parsec.core.logged_core import LoggedCore
from parsec.core.gui.trio_jobs import JobResultError, QtToTrioJob, QtToTrioJobScheduler
from parsec.core.gui.custom_dialogs import show_error, GreyedDialog, show_info
from parsec.core.gui import validators
from parsec.core.gui.lang import translate as _
from parsec.core.gui.ui.greet_user_widget import Ui_GreetUserWidget
from parsec.core.gui.ui.greet_user_code_exchange_widget import Ui_GreetUserCodeExchangeWidget
from parsec.core.gui.ui.greet_user_check_info_widget import Ui_GreetUserCheckInfoWidget
from parsec.core.gui.ui.greet_user_instructions_widget import Ui_GreetUserInstructionsWidget


class Greeter:
    class Step(IntEnum):
        WaitPeer = 1
        GetGreeterSas = 2
        WaitPeerTrust = 3
        GetClaimerSas = 4
        SignifyTrust = 5
        GetClaimRequests = 6
        CreateNewUser = 7

    def __init__(self) -> None:
        self.main_mc_send: trio.MemorySendChannel[Any]
        self.main_mc_recv: trio.MemoryReceiveChannel[Any]
        self.main_mc_send, self.main_mc_recv = trio.open_memory_channel(0)

        self.job_mc_send: trio.MemorySendChannel[Any]
        self.job_mc_recv: trio.MemoryReceiveChannel[Any]
        self.job_mc_send, self.job_mc_recv = trio.open_memory_channel(0)

    async def run(self, core: LoggedCore, token: InvitationToken) -> None:
        try:
            r = await self.main_mc_recv.receive()

            assert r == self.Step.WaitPeer
            try:
                in_progress_ctx = await core.start_greeting_user(token=token)
                await self.job_mc_send.send((True, None))
            except Exception as exc:
                await self.job_mc_send.send((False, exc))

            r = await self.main_mc_recv.receive()

            assert r == self.Step.GetGreeterSas
            await self.job_mc_send.send(in_progress_ctx.greeter_sas)

            r = await self.main_mc_recv.receive()

            assert r == self.Step.WaitPeerTrust
            try:
                in_progress_ctx_greet: UserGreetInProgress2Ctx = (
                    await in_progress_ctx.do_wait_peer_trust()
                )
                await self.job_mc_send.send((True, None))
            except Exception as exc:
                await self.job_mc_send.send((False, exc))

            r = await self.main_mc_recv.receive()

            assert r == self.Step.GetClaimerSas
            try:
                choices = in_progress_ctx_greet.generate_claimer_sas_choices(size=4)
                await self.job_mc_send.send(
                    (True, None, in_progress_ctx_greet.claimer_sas, choices)
                )
            except Exception as exc:
                await self.job_mc_send.send((False, exc, None, None))

            r = await self.main_mc_recv.receive()

            assert r == self.Step.SignifyTrust
            try:
                in_progress_ctx_trust = await in_progress_ctx_greet.do_signify_trust()
                await self.job_mc_send.send((True, None))
            except Exception as exc:
                await self.job_mc_send.send((False, exc))

            r = await self.main_mc_recv.receive()

            assert r == self.Step.GetClaimRequests
            try:
                in_progress_ctx_request = await in_progress_ctx_trust.do_get_claim_requests()
                await self.job_mc_send.send(
                    (
                        True,
                        None,
                        in_progress_ctx_request.requested_human_handle,
                        in_progress_ctx_request.requested_device_label,
                    )
                )
            except Exception as exc:
                await self.job_mc_send.send((False, exc, None, None))

            r = await self.main_mc_recv.receive()

            assert r == self.Step.CreateNewUser
            try:
                human_handle, device_label, profile = await self.main_mc_recv.receive()
                await in_progress_ctx_request.do_create_new_user(
                    author=core.device,
                    device_label=device_label,
                    human_handle=human_handle,
                    profile=profile,
                )
                await self.job_mc_send.send((True, None))
            except InviteError as exc:
                await self.job_mc_send.send((False, exc))
            except Exception as exc:
                await self.job_mc_send.send((False, exc))

        except BackendNotAvailable as exc:
            raise JobResultError(status="backend-not-available", origin=exc)

    async def wait_peer(self) -> None:
        await self.main_mc_send.send(self.Step.WaitPeer)
        r, exc = await self.job_mc_recv.receive()
        if not r:
            raise JobResultError(status="wait-peer-failed", origin=exc)

    async def get_greeter_sas(self) -> SASCode:
        await self.main_mc_send.send(self.Step.GetGreeterSas)
        greeter_sas = await self.job_mc_recv.receive()
        return greeter_sas

    async def wait_peer_trust(self) -> None:
        await self.main_mc_send.send(self.Step.WaitPeerTrust)
        r, exc = await self.job_mc_recv.receive()
        if not r:
            raise JobResultError(status="wait-peer-trust-failed", origin=exc)

    async def get_claimer_sas(self) -> tuple[SASCode, list[SASCode]]:
        await self.main_mc_send.send(self.Step.GetClaimerSas)
        r, exc, claimer_sas, choices = await self.job_mc_recv.receive()
        if not r:
            raise JobResultError(status="get-claimer-sas-failed", origin=exc)
        return claimer_sas, choices

    async def signify_trust(self) -> None:
        await self.main_mc_send.send(self.Step.SignifyTrust)
        r, exc = await self.job_mc_recv.receive()
        if not r:
            raise JobResultError(status="signify-trust-failed", origin=exc)

    async def get_claim_requests(self) -> tuple[HumanHandle, DeviceLabel]:
        await self.main_mc_send.send(self.Step.GetClaimRequests)
        r, exc, human_handle, device_label = await self.job_mc_recv.receive()
        if not r:
            raise JobResultError(status="get-claim-request-failed", origin=exc)
        return human_handle, device_label

    async def create_new_user(
        self, human_handle: HumanHandle, device_label: DeviceLabel, profile: UserProfile
    ) -> None:
        await self.main_mc_send.send(self.Step.CreateNewUser)
        await self.main_mc_send.send((human_handle, device_label, profile))
        r, exc = await self.job_mc_recv.receive()
        if not r:
            raise JobResultError(status="create-new-user-failed", origin=exc)


class GreetUserInstructionsWidget(QWidget, Ui_GreetUserInstructionsWidget):
    succeeded = pyqtSignal()
    failed = pyqtSignal(object)  # QtToTrioJob or None

    wait_peer_success = pyqtSignal(QtToTrioJob)
    wait_peer_error = pyqtSignal(QtToTrioJob)

    def __init__(self, jobs_ctx: QtToTrioJobScheduler, greeter: Greeter) -> None:
        super().__init__()
        self.setupUi(self)
        self.jobs_ctx = jobs_ctx
        self.greeter = greeter
        self.wait_peer_job: QtToTrioJob[None] | None = None
        self.wait_peer_success.connect(self._on_wait_peer_success)
        self.wait_peer_error.connect(self._on_wait_peer_error)
        self.button_start.clicked.connect(self._on_button_start_clicked)

    def _on_button_start_clicked(self, checked: bool) -> None:
        self.button_start.setDisabled(True)
        self.button_start.setText(_("TEXT_GREET_USER_WAITING"))
        self.wait_peer_job = self.jobs_ctx.submit_job(
            (self, "wait_peer_success"), (self, "wait_peer_error"), self.greeter.wait_peer
        )

    def _on_wait_peer_success(self, job: QtToTrioJob[None]) -> None:
        if self.wait_peer_job != job:
            return
        self.wait_peer_job = None
        assert job
        assert job.is_finished()
        assert job.status == "ok"
        self.greeter_sas = job.ret
        self.succeeded.emit()

    def _on_wait_peer_error(self, job: QtToTrioJob[None]) -> None:
        if self.wait_peer_job != job:
            return
        self.wait_peer_job = None
        assert job
        assert job.is_finished()
        assert job.status != "ok"
        if job.status != "cancelled":
            msg = _("TEXT_GREET_USER_WAIT_PEER_ERROR")
            exc = None
            if job.exc:
                assert isinstance(job.exc, JobResultError)
                exc = job.exc.origin
                if isinstance(exc, InvitePeerResetError):
                    msg = _("TEXT_GREET_USER_PEER_RESET")
                elif isinstance(exc, InviteAlreadyUsedError):
                    msg = _("TEXT_INVITATION_ALREADY_USED")
            show_error(self, msg, exception=exc)
        self.failed.emit(job)


class GreetUserCheckInfoWidget(QWidget, Ui_GreetUserCheckInfoWidget):
    succeeded = pyqtSignal()
    failed = pyqtSignal(object)

    get_requests_success = pyqtSignal(QtToTrioJob)
    get_requests_error = pyqtSignal(QtToTrioJob)

    create_user_success = pyqtSignal(QtToTrioJob)
    create_user_error = pyqtSignal(QtToTrioJob)

    def __init__(
        self,
        jobs_ctx: QtToTrioJobScheduler,
        greeter: Greeter,
        user_profile_outsider_allowed: bool = False,
    ) -> None:
        super().__init__()
        self.setupUi(self)
        self.jobs_ctx = jobs_ctx
        self.greeter = greeter
        self.get_requests_job: QtToTrioJob[tuple[HumanHandle, DeviceLabel]] | None = None
        self.create_user_job: QtToTrioJob[None] | None = None

        self.widget_info.hide()
        self.label_waiting.show()

        self.line_edit_user_full_name.validity_changed.connect(self.check_infos)
        self.line_edit_user_full_name.set_validator(validators.UserNameValidator())
        self.line_edit_user_email.validity_changed.connect(self.check_infos)
        self.line_edit_user_email.set_validator(validators.EmailValidator())
        self.line_edit_device.validity_changed.connect(self.check_infos)
        self.line_edit_device.set_validator(validators.DeviceLabelValidator())
        self.combo_profile.currentIndexChanged.connect(self.check_infos)

        self.combo_profile.addItem(_("TEXT_SELECT_USER_PROFILE"), None)
        self.combo_profile.addItem(_("TEXT_USER_PROFILE_OUTSIDER"), UserProfile.OUTSIDER)
        self.combo_profile.addItem(_("TEXT_USER_PROFILE_STANDARD"), UserProfile.STANDARD)
        self.combo_profile.addItem(_("TEXT_USER_PROFILE_ADMIN"), UserProfile.ADMIN)

        item = self.combo_profile.model().item(2)
        item.setToolTip(_("TEXT_USER_PROFILE_STANDARD_TOOLTIP"))
        item = self.combo_profile.model().item(3)
        item.setToolTip(_("TEXT_USER_PROFILE_ADMIN_TOOLTIP"))

        if not user_profile_outsider_allowed:
            item = self.combo_profile.model().item(1)
            item.setEnabled(False)
            item.setToolTip(_("NOT_ALLOWED_OUTSIDER_PROFILE_TOOLTIP"))
        else:
            item = self.combo_profile.model().item(1)
            item.setToolTip(_("TEXT_USER_PROFILE_OUTSIDER_TOOLTIP"))

        # No profile by default, forcing the greeter to choose one
        self.combo_profile.setCurrentIndex(0)

        self.get_requests_success.connect(self._on_get_requests_success)
        self.get_requests_error.connect(self._on_get_requests_error)
        self.create_user_success.connect(self._on_create_user_success)
        self.create_user_error.connect(self._on_create_user_error)
        self.button_create_user.clicked.connect(self._on_create_user_clicked)

        self.get_requests_job = self.jobs_ctx.submit_job(
            (self, "get_requests_success"),
            (self, "get_requests_error"),
            self.greeter.get_claim_requests,
        )

    def check_infos(self, _: Any | None = None) -> None:
        if (
            self.line_edit_user_full_name.is_input_valid()
            and self.line_edit_device.is_input_valid()
            and self.line_edit_user_email.is_input_valid()
            and self.combo_profile.currentIndex() != 0
        ):
            self.button_create_user.setDisabled(False)
        else:
            self.button_create_user.setDisabled(True)

    def _on_create_user_clicked(self) -> None:
        assert not self.create_user_job
        # No try/except given inputs are validated with validators
        device_label = DeviceLabel(self.line_edit_device.clean_text())
        handle = HumanHandle(
            label=self.line_edit_user_full_name.clean_text(), email=self.line_edit_user_email.text()
        )

        self.button_create_user.setDisabled(True)
        self.button_create_user.setText(_("TEXT_GREET_USER_WAITING"))
        self.create_user_job = self.jobs_ctx.submit_job(
            (self, "create_user_success"),
            (self, "create_user_error"),
            self.greeter.create_new_user,
            human_handle=handle,
            device_label=device_label,
            profile=self.combo_profile.currentData(),
        )

    def _on_create_user_success(self, job: QtToTrioJob[None]) -> None:
        if self.create_user_job != job:
            return
        self.create_user_job = None
        assert job
        assert job.is_finished()
        assert job.status == "ok"
        self.succeeded.emit()

    def _on_create_user_error(self, job: QtToTrioJob[None]) -> None:
        if self.create_user_job != job:
            return
        self.create_user_job = None
        assert job
        assert job.is_finished()
        assert job.status != "ok"
        if job.status != "cancelled":
            msg = _("TEXT_GREET_USER_CREATE_USER_ERROR")
            exc = None
            if job.exc:
                assert isinstance(job.exc, JobResultError)
                exc = job.exc.origin
                if isinstance(exc, InvitePeerResetError):
                    msg = _("TEXT_GREET_USER_PEER_RESET")
                elif isinstance(exc, InviteAlreadyUsedError):
                    msg = _("TEXT_INVITATION_ALREADY_USED")
                elif isinstance(exc, InviteActiveUsersLimitReachedError):
                    msg = _("TEXT_GREET_USER_ACTIVE_USERS_LIMIT_REACHED")
            show_error(self, msg, exception=exc)
        self.failed.emit(job)

    def _on_get_requests_success(self, job: QtToTrioJob[tuple[HumanHandle, DeviceLabel]]) -> None:
        if self.get_requests_job != job:
            return
        self.get_requests_job = None
        assert job
        assert job.is_finished()
        assert job.status == "ok"
        assert job.ret is not None
        human_handle, device_label = job.ret
        self.label_waiting.hide()
        self.widget_info.show()
        self.line_edit_user_full_name.setText(human_handle.label)
        self.line_edit_user_email.setText(human_handle.email)
        self.line_edit_device.setText(device_label.str)
        self.check_infos()

    def _on_get_requests_error(self, job: QtToTrioJob[tuple[HumanHandle, DeviceLabel]]) -> None:
        if self.get_requests_job != job:
            return
        self.get_requests_job = None
        assert job
        assert job.is_finished()
        assert job.status != "ok"
        if job.status != "cancelled":
            msg = _("TEXT_GREET_USER_GET_REQUESTS_ERROR")
            exc = None
            if job.exc:
                assert isinstance(job.exc, JobResultError)
                exc = job.exc.origin
                if isinstance(exc, InvitePeerResetError):
                    msg = _("TEXT_GREET_USER_PEER_RESET")
                elif isinstance(exc, InviteAlreadyUsedError):
                    msg = _("TEXT_INVITATION_ALREADY_USED")
            show_error(self, msg, exception=exc)
        self.failed.emit(job)


class GreetUserCodeExchangeWidget(QWidget, Ui_GreetUserCodeExchangeWidget):
    succeeded = pyqtSignal()
    failed = pyqtSignal(object)

    signify_trust_success = pyqtSignal(QtToTrioJob)
    signify_trust_error = pyqtSignal(QtToTrioJob)

    wait_peer_trust_success = pyqtSignal(QtToTrioJob)
    wait_peer_trust_error = pyqtSignal(QtToTrioJob)

    get_claimer_sas_success = pyqtSignal(QtToTrioJob)
    get_claimer_sas_error = pyqtSignal(QtToTrioJob)

    get_greeter_sas_success = pyqtSignal(QtToTrioJob)
    get_greeter_sas_error = pyqtSignal(QtToTrioJob)

    def __init__(self, jobs_ctx: QtToTrioJobScheduler, greeter: Greeter) -> None:
        super().__init__()
        self.setupUi(self)
        self.jobs_ctx = jobs_ctx
        self.greeter = greeter

        self.wait_peer_trust_job: QtToTrioJob[None] | None = None
        self.signify_trust_job: QtToTrioJob[None] | None = None
        self.get_claimer_sas_job: QtToTrioJob[tuple[SASCode, list[SASCode]]] | None = None
        self.get_greeter_sas_job: QtToTrioJob[SASCode] | None = None

        self.widget_claimer_code.hide()

        font = self.line_edit_greeter_code.font()
        font.setBold(True)
        font.setLetterSpacing(QFont.PercentageSpacing, 180)
        self.line_edit_greeter_code.setFont(font)

        self.code_input_widget.good_code_clicked.connect(self._on_good_claimer_code_clicked)
        self.code_input_widget.wrong_code_clicked.connect(self._on_wrong_claimer_code_clicked)
        self.code_input_widget.none_clicked.connect(self._on_none_clicked)

        self.signify_trust_success.connect(self._on_signify_trust_success)
        self.signify_trust_error.connect(self._on_signify_trust_error)
        self.wait_peer_trust_success.connect(self._on_wait_peer_trust_success)
        self.wait_peer_trust_error.connect(self._on_wait_peer_trust_error)
        self.get_greeter_sas_success.connect(self._on_get_greeter_sas_success)
        self.get_greeter_sas_error.connect(self._on_get_greeter_sas_error)
        self.get_claimer_sas_success.connect(self._on_get_claimer_sas_success)
        self.get_claimer_sas_error.connect(self._on_get_claimer_sas_error)

        self.get_greeter_sas_job = self.jobs_ctx.submit_job(
            (self, "get_greeter_sas_success"),
            (self, "get_greeter_sas_error"),
            self.greeter.get_greeter_sas,
        )

    def _on_good_claimer_code_clicked(self) -> None:
        self.widget_claimer_code.setDisabled(True)
        self.signify_trust_job = self.jobs_ctx.submit_job(
            (self, "signify_trust_success"),
            (self, "signify_trust_error"),
            self.greeter.signify_trust,
        )

    def _on_wrong_claimer_code_clicked(self) -> None:
        show_error(self, _("TEXT_GREET_USER_INVALID_CODE_CLICKED"))
        self.failed.emit(None)

    def _on_none_clicked(self) -> None:
        show_info(self, _("TEXT_GREET_USER_NONE_CODE_CLICKED"))
        self.failed.emit(None)

    def _on_get_greeter_sas_success(self, job: QtToTrioJob[SASCode]) -> None:
        if self.get_greeter_sas_job != job:
            return
        self.get_greeter_sas_job = None
        assert job
        assert job.is_finished()
        assert job.status == "ok"
        assert job.ret is not None
        greeter_sas = job.ret
        self.line_edit_greeter_code.setText(greeter_sas.str)
        self.wait_peer_trust_job = self.jobs_ctx.submit_job(
            (self, "wait_peer_trust_success"),
            (self, "wait_peer_trust_error"),
            self.greeter.wait_peer_trust,
        )

    def _on_get_greeter_sas_error(self, job: QtToTrioJob[SASCode]) -> None:
        if self.get_greeter_sas_job != job:
            return
        self.get_greeter_sas_job = None
        assert job
        assert job.is_finished()
        assert job.status != "ok"
        if job.status != "cancelled":
            msg = _("TEXT_GREET_USER_GET_GREETER_SAS_ERROR")
            exc = None
            if job.exc:
                assert isinstance(job.exc, JobResultError)
                exc = job.exc.origin
                if isinstance(exc, InvitePeerResetError):
                    msg = _("TEXT_GREET_USER_PEER_RESET")
                elif isinstance(exc, InviteAlreadyUsedError):
                    msg = _("TEXT_INVITATION_ALREADY_USED")
            show_error(self, msg, exception=exc)
        self.failed.emit(job)

    def _on_get_claimer_sas_success(self, job: QtToTrioJob[tuple[SASCode, list[SASCode]]]) -> None:
        if self.get_claimer_sas_job != job:
            return
        self.get_claimer_sas_job = None
        assert job
        assert job.is_finished()
        assert job.status == "ok"
        assert job.ret is not None
        claimer_sas, choices = job.ret
        self.widget_greeter_code.hide()
        self.widget_claimer_code.show()
        self.code_input_widget.set_choices(choices, claimer_sas)

    def _on_get_claimer_sas_error(self, job: QtToTrioJob[tuple[SASCode, list[SASCode]]]) -> None:
        if self.get_claimer_sas_job != job:
            return
        self.get_claimer_sas_job = None
        assert job
        assert job.is_finished()
        assert job.status != "ok"
        if job.status != "cancelled":
            msg = _("TEXT_GREET_USER_GET_CLAIMER_SAS_ERROR")
            exc = None
            if job.exc:
                assert isinstance(job.exc, JobResultError)
                exc = job.exc.origin
                if isinstance(exc, InvitePeerResetError):
                    msg = _("TEXT_GREET_USER_PEER_RESET")
                elif isinstance(exc, InviteAlreadyUsedError):
                    msg = _("TEXT_INVITATION_ALREADY_USED")
            show_error(self, msg, exception=exc)
        self.failed.emit(job)

    def _on_signify_trust_success(self, job: QtToTrioJob[None]) -> None:
        if self.signify_trust_job != job:
            return
        self.signify_trust_job = None
        assert job
        assert job.is_finished()
        assert job.status == "ok"
        self.succeeded.emit()

    def _on_signify_trust_error(self, job: QtToTrioJob[None]) -> None:
        if self.signify_trust_job != job:
            return
        self.signify_trust_job = None
        assert job
        assert job.is_finished()
        assert job.status != "ok"
        if job.status != "cancelled":
            msg = _("TEXT_GREET_USER_SIGNIFY_TRUST_ERROR")
            exc = None
            if job.exc:
                assert isinstance(job.exc, JobResultError)
                exc = job.exc.origin
                if isinstance(exc, InvitePeerResetError):
                    msg = _("TEXT_GREET_USER_PEER_RESET")
                elif isinstance(exc, InviteAlreadyUsedError):
                    msg = _("TEXT_INVITATION_ALREADY_USED")
            show_error(self, msg, exception=exc)
        self.failed.emit(job)

    def _on_wait_peer_trust_success(self, job: QtToTrioJob[None]) -> None:
        if self.wait_peer_trust_job != job:
            return
        self.wait_peer_trust_job = None
        assert job
        assert job.is_finished()
        assert job.status == "ok"
        self.get_claimer_sas_job = self.jobs_ctx.submit_job(
            (self, "get_claimer_sas_success"),
            (self, "get_claimer_sas_error"),
            self.greeter.get_claimer_sas,
        )

    def _on_wait_peer_trust_error(self, job: QtToTrioJob[None]) -> None:
        if self.wait_peer_trust_job != job:
            return
        self.wait_peer_trust_job = None
        assert job
        assert job.is_finished()
        assert job.status != "ok"
        if job.status != "cancelled":
            msg = _("TEXT_GREET_USER_WAIT_PEER_TRUST_ERROR")
            exc = None
            if job.exc:
                assert isinstance(job.exc, JobResultError)
                exc = job.exc.origin
                if isinstance(exc, InvitePeerResetError):
                    msg = _("TEXT_GREET_USER_PEER_RESET")
                elif isinstance(exc, InviteAlreadyUsedError):
                    msg = _("TEXT_INVITATION_ALREADY_USED")
            show_error(self, msg, exception=exc)
        self.failed.emit(job)


class GreetUserWidget(QWidget, Ui_GreetUserWidget):
    greeter_success = pyqtSignal(QtToTrioJob)
    greeter_error = pyqtSignal(QtToTrioJob)

    def __init__(
        self, core: LoggedCore, jobs_ctx: QtToTrioJobScheduler, token: InvitationToken
    ) -> None:
        super().__init__()
        self.setupUi(self)
        self.core = core
        self.jobs_ctx = jobs_ctx
        self.token = token
        self.dialog: QDialog | None = None
        self.greeter = Greeter()
        self.greeter_job: QtToTrioJob[None] | None = None
        self.greeter_success.connect(self._on_greeter_success)
        self.greeter_error.connect(self._on_greeter_error)
        self._run_greeter()

    def _run_greeter(self) -> None:
        self.greeter_job = self.jobs_ctx.submit_job(
            (self, "greeter_success"),
            (self, "greeter_error"),
            self.greeter.run,
            core=self.core,
            token=self.token,
        )
        self._goto_page1()

    def restart(self) -> None:
        self.cancel()
        # Replace moving parts
        self.greeter = Greeter()
        self._run_greeter()

    def _on_page_failed(self, job: QtToTrioJob[None]) -> None:
        # The dialog has already been rejected
        if not self.isVisible():
            return
        # No reason to restart the process if cancelled, simply close the dialog
        if job is not None and job.status == "cancelled":
            assert self.dialog is not None
            self.dialog.reject()
            return
        # No reason to restart the process if offline, simply close the dialog
        if isinstance(job.exc, JobResultError) and isinstance(job.exc.origin, BackendNotAvailable):
            assert self.dialog is not None
            self.dialog.reject()
            return
        # No reason to restart the process if the invitation is already used, simply close the dialog
        if isinstance(job.exc, JobResultError) and isinstance(
            job.exc.origin, InviteAlreadyUsedError
        ):
            assert self.dialog is not None
            self.dialog.reject()
            return

        # No reason to restart the process if active users limit has been reached
        if isinstance(job.exc, JobResultError) and isinstance(
            job.exc.origin, InviteActiveUsersLimitReachedError
        ):
            assert self.dialog is not None
            self.dialog.reject()
            return
        # Let's try one more time with the same dialog
        self.restart()

    def _goto_page1(self) -> None:
        item = self.main_layout.takeAt(0)
        if item:
            current_page = item.widget()
            if current_page:
                current_page.hide()
                current_page.setParent(None)
        page = GreetUserInstructionsWidget(self.jobs_ctx, self.greeter)
        page.succeeded.connect(self._goto_page2)
        page.failed.connect(self._on_page_failed)
        self.main_layout.addWidget(page)

    def _goto_page2(self) -> None:
        current_page = self.main_layout.takeAt(0).widget()
        current_page.hide()
        current_page.setParent(None)
        page = GreetUserCodeExchangeWidget(self.jobs_ctx, self.greeter)
        page.succeeded.connect(self._goto_page3)
        page.failed.connect(self._on_page_failed)
        self.main_layout.addWidget(page)

    def _goto_page3(self) -> None:
        current_page = self.main_layout.takeAt(0).widget()
        current_page.hide()
        current_page.setParent(None)
        # The organization's config value is already cached in the core's logic
        # so the GUI doesn't need to set the value in its own cache
        organization_config = self.core.get_organization_config()
        page = GreetUserCheckInfoWidget(
            self.jobs_ctx, self.greeter, organization_config.user_profile_outsider_allowed
        )
        page.succeeded.connect(self._on_finished)
        page.failed.connect(self._on_page_failed)
        self.main_layout.addWidget(page)

    def _on_finished(self) -> None:
        show_info(self, _("TEXT_USER_GREET_SUCCESSFUL"))
        assert self.dialog is not None
        self.dialog.accept()

    def _on_greeter_success(self, job: QtToTrioJob[None]) -> None:
        if self.greeter_job != job:
            return
        assert self.greeter_job
        assert self.greeter_job.is_finished()
        assert self.greeter_job.status == "ok"
        self.greeter_job = None

    def _on_greeter_error(self, job: QtToTrioJob[None]) -> None:
        assert job
        assert job.is_finished()
        assert job.status != "ok"
        # This callback can be called after the creation of a new greeter job in the case
        # of a restart, due to Qt signals being called later.
        if job.status == "cancelled":
            return
        # Safety net for concurrency issues
        if self.greeter_job != job:
            return
        self.greeter_job = None
        msg = ""
        exc = None
        if job.status == "backend-not-available":
            msg = _("TEXT_INVITATION_BACKEND_NOT_AVAILABLE")
        else:
            msg = _("TEXT_GREET_USER_UNKNOWN_ERROR")
        if job.exc:
            assert isinstance(job.exc, JobResultError)
            exc = job.exc.origin
        show_error(self, msg, exception=exc)
        # No point in retrying since the greeter job itself failed, simply close the dialog
        assert self.dialog is not None
        self.dialog.reject()

    def cancel(self) -> None:
        item = self.main_layout.itemAt(0)
        if item:
            current_page = item.widget()
            if current_page and getattr(current_page, "cancel", None):
                current_page.cancel()
        if self.greeter_job:
            self.greeter_job.cancel()

    def on_close(self) -> None:
        self.cancel()

    @classmethod
    def show_modal(  # type: ignore[misc]
        cls,
        core: LoggedCore,
        jobs_ctx: QtToTrioJobScheduler,
        token: InvitationToken,
        parent: QWidget,
        on_finished: Callable[..., None],
    ) -> GreetUserWidget:
        w = cls(core=core, jobs_ctx=jobs_ctx, token=token)
        d = GreyedDialog(w, _("TEXT_GREET_USER_TITLE"), parent=parent, width=1000)
        w.dialog = d

        d.finished.connect(on_finished)
        # Unlike exec_, show is asynchronous and works within the main Qt loop
        d.show()
        return w
