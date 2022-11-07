# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations
from typing import Any

from PyQt5.QtCore import pyqtSignal, Qt
from PyQt5.QtWidgets import QWidget, QGraphicsDropShadowEffect, QLabel
from PyQt5.QtGui import QColor
from parsec.api.protocol import InvitationEmailSentStatus

from parsec.core.backend_connection import BackendNotAvailable, BackendConnectionError
from parsec.core.gui.trio_jobs import JobResultError, QtToTrioJob, QtToTrioJobScheduler
from parsec.core.gui.greet_device_widget import GreetDeviceWidget
from parsec.core.gui.lang import translate as _
from parsec.core.gui.custom_widgets import ensure_string_size
from parsec.core.gui.custom_dialogs import show_error
from parsec.core.gui.flow_layout import FlowLayout
from parsec.core.gui.ui.devices_widget import Ui_DevicesWidget
from parsec.core.gui.ui.device_button import Ui_DeviceButton
from parsec.core.logged_core import LoggedCore
from parsec.core.types import BackendInvitationAddr, DeviceInfo
from parsec.event_bus import EventBus


class DeviceButton(QWidget, Ui_DeviceButton):
    def __init__(self, device_info: DeviceInfo, is_current_device: bool) -> None:
        super().__init__()
        self.setupUi(self)
        self.is_current_device = is_current_device
        self.device_info = device_info
        self.label_icon.apply_style()
        self.label_device_name.setText(
            ensure_string_size(self.device_info.device_display, 170, self.label_device_name.font())
        )
        self.label_device_name.setToolTip(self.device_info.device_display)
        if self.is_current_device:
            self.label_is_current.setText("({})".format(_("TEXT_DEVICE_IS_CURRENT")))

        effect = QGraphicsDropShadowEffect(self)
        effect.setColor(QColor(0x99, 0x99, 0x99))
        effect.setBlurRadius(10)
        effect.setXOffset(2)
        effect.setYOffset(2)
        self.setGraphicsEffect(effect)


async def _do_invite_device(
    core: LoggedCore,
) -> tuple[BackendInvitationAddr, InvitationEmailSentStatus]:
    try:
        addr, email_sent_status = await core.new_device_invitation(send_email=False)
        return addr, email_sent_status
    except BackendNotAvailable as exc:
        raise JobResultError("offline") from exc
    except BackendConnectionError as exc:
        raise JobResultError("error") from exc


async def _do_list_devices(core: LoggedCore) -> list[DeviceInfo]:
    try:
        return await core.get_user_devices_info()
    except BackendNotAvailable as exc:
        raise JobResultError("offline") from exc
    except BackendConnectionError as exc:
        raise JobResultError("error") from exc


class DevicesWidget(QWidget, Ui_DevicesWidget):
    list_success = pyqtSignal(QtToTrioJob)
    list_error = pyqtSignal(QtToTrioJob)

    invite_success = pyqtSignal(QtToTrioJob)
    invite_error = pyqtSignal(QtToTrioJob)

    def __init__(
        self,
        core: LoggedCore,
        jobs_ctx: QtToTrioJobScheduler,
        event_bus: EventBus,
        *args: Any,
        **kwargs: Any,
    ) -> None:
        super().__init__(*args, **kwargs)

        self.setupUi(self)
        self.jobs_ctx = jobs_ctx
        self.core = core
        self.event_bus = event_bus
        self.layout_devices = FlowLayout(spacing=30)
        self.layout_content.addLayout(self.layout_devices)
        self.button_add_device.clicked.connect(self.invite_device)
        self.button_add_device.apply_style()
        self.list_success.connect(self._on_list_success)
        self.list_error.connect(self._on_list_error)
        self.invite_success.connect(self._on_invite_success)
        self.invite_error.connect(self._on_invite_error)

    def show(self) -> None:
        self.reset()
        super().show()

    def invite_device(self) -> None:
        _ = self.jobs_ctx.submit_job(
            (self, "invite_success"), (self, "invite_error"), _do_invite_device, core=self.core
        )

    def _on_invite_success(
        self, job: QtToTrioJob[tuple[BackendInvitationAddr, InvitationEmailSentStatus]]
    ) -> None:
        assert job.is_finished()
        assert job.status == "ok"
        assert job.ret is not None

        invite_addr, email_sent_status = job.ret
        GreetDeviceWidget.show_modal(
            core=self.core,
            jobs_ctx=self.jobs_ctx,
            invite_addr=invite_addr,
            parent=self,
            on_finished=self.reset,
        )

    def _on_invite_error(
        self, job: QtToTrioJob[tuple[BackendInvitationAddr, InvitationEmailSentStatus]]
    ) -> None:
        assert job.is_finished()
        assert job.status != "ok"

        status = job.status
        if status == "offline":
            errmsg = _("TEXT_INVITE_DEVICE_INVITE_OFFLINE")
        else:
            errmsg = _("TEXT_INVITE_DEVICE_INVITE_ERROR")

        show_error(self, errmsg, exception=job.exc)

    def add_device(self, device_info: DeviceInfo, is_current_device: bool) -> None:
        button = DeviceButton(device_info, is_current_device)
        self.layout_devices.addWidget(button)
        button.show()

    def _on_list_success(self, job: QtToTrioJob[list[DeviceInfo]]) -> None:
        assert job.is_finished()
        assert job.status == "ok"

        assert job.ret is not None
        devices: list[DeviceInfo] = job.ret
        current_device = self.core.device
        self.layout_devices.clear()
        for device in devices:
            self.add_device(device, is_current_device=current_device.device_id == device.device_id)
        self.spinner.hide()

    def _on_list_error(self, job: QtToTrioJob[list[DeviceInfo]]) -> None:
        assert job.is_finished()
        assert job.status != "ok"

        status = job.status
        if status in ["error", "offline"]:
            self.layout_devices.clear()
            label = QLabel(_("TEXT_DEVICE_LIST_RETRIEVABLE_FAILURE"))
            label.setAlignment(Qt.AlignHCenter | Qt.AlignVCenter)
            self.layout_devices.addWidget(label)
        self.spinner.hide()

    def reset(self) -> None:
        self.layout_devices.clear()
        self.spinner.show()
        _ = self.jobs_ctx.submit_job(
            (self, "list_success"), (self, "list_error"), _do_list_devices, core=self.core
        )
