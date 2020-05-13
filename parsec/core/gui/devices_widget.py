# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from uuid import UUID

from PyQt5.QtCore import pyqtSignal, Qt, QTimer
from PyQt5.QtWidgets import QWidget, QMenu, QGraphicsDropShadowEffect, QApplication
from PyQt5.QtGui import QColor

from parsec.api.protocol import DeviceID, InvitationType, InvitationStatus, InvitationDeletedReason
from parsec.core.backend_connection import BackendConnectionError, BackendNotAvailable, backend_authenticated_cmds_factory
from parsec.core.remote_devices_manager import RemoteDevicesManagerBackendOfflineError
from parsec.core.types import BackendInvitationAddr

from parsec.core.gui.trio_thread import JobResultError, ThreadSafeQtSignal, QtToTrioJob
from parsec.core.gui.greet_device_widget import GreetDeviceWidget
from parsec.core.gui.lang import translate as _
from parsec.core.gui.custom_widgets import ensure_string_size
from parsec.core.gui.password_change_widget import PasswordChangeWidget
from parsec.core.gui.flow_layout import FlowLayout
from parsec.core.gui.ui.devices_widget import Ui_DevicesWidget
from parsec.core.gui.ui.device_button import Ui_DeviceButton


class DeviceButton(QWidget, Ui_DeviceButton):
    change_password_clicked = pyqtSignal(str)

    def __init__(self, device_name, is_current_device, certified_on):
        super().__init__()
        self.setupUi(self)
        self.is_current_device = is_current_device
        self.label_icon.apply_style()
        self.device_name = device_name
        self.certified_on = certified_on
        self.label_device_name.setText(ensure_string_size(self.device_name, 260, self.label_device_name.font()))
        self.label_device_name.setToolTip(self.device_name)
        if self.is_current_device:
            self.label_is_current.setText("({})".format(_("TEXT_DEVICE_IS_CURRENT")))
        self.setContextMenuPolicy(Qt.CustomContextMenu)
        self.customContextMenuRequested.connect(self.show_context_menu)
        effect = QGraphicsDropShadowEffect(self)
        effect.setColor(QColor(0x99, 0x99, 0x99))
        effect.setBlurRadius(10)
        effect.setXOffset(2)
        effect.setYOffset(2)
        self.setGraphicsEffect(effect)

    def show_context_menu(self, pos):
        if not self.is_current_device:
            return
        global_pos = self.mapToGlobal(pos)
        menu = QMenu(self)
        action = menu.addAction(_("ACTION_DEVICE_MENU_CHANGE_PASSWORD"))
        action.triggered.connect(self.change_password)
        menu.exec_(global_pos)

    def change_password(self):
        self.change_password_clicked.emit(self.device_name)


async def _do_invite_device(device, config):
    async with backend_authenticated_cmds_factory(
        addr=device.organization_addr,
        device_id=device.device_id,
        signing_key=device.signing_key,
        keepalive=config.backend_connection_keepalive,
    ) as cmds:
        rep = await cmds.invite_new(type=InvitationType.DEVICE, send_email=False)
        if rep["status"] != "ok":
            raise JobResultError(rep["status"])
        action_addr = BackendInvitationAddr.build(
            backend_addr=device.organization_addr,
            organization_id=device.organization_id,
            invitation_type=InvitationType.DEVICE,
            token=rep["token"],
        )
        return action_addr


async def _do_list_devices(core):
    try:
        current_device = core.device
        _, _, devices = await core.remote_devices_manager.get_user_and_devices(
            current_device.user_id
        )
        return devices
    except RemoteDevicesManagerBackendOfflineError as exc:
        raise JobResultError("offline") from exc
    # TODO : handle all errors from the remote_devices_manager and notify GUI
    # Raises:
    #     RemoteDevicesManagerError
    #     RemoteDevicesManagerBackendOfflineError
    #     RemoteDevicesManagerNotFoundError
    #     RemoteDevicesManagerInvalidTrustchainError


class DevicesWidget(QWidget, Ui_DevicesWidget):
    list_success = pyqtSignal(QtToTrioJob)
    list_error = pyqtSignal(QtToTrioJob)

    invite_success = pyqtSignal(QtToTrioJob)
    invite_error = pyqtSignal(QtToTrioJob)

    def __init__(self, core, jobs_ctx, event_bus, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.setupUi(self)
        self.jobs_ctx = jobs_ctx
        self.core = core
        self.event_bus = event_bus
        self.layout_devices = FlowLayout(spacing=40)
        self.layout_content.addLayout(self.layout_devices)
        self.button_add_device.clicked.connect(self.invite_device)
        self.button_add_device.apply_style()
        self.filter_timer = QTimer()
        self.filter_timer.setInterval(300)
        self.list_success.connect(self.on_list_success)
        self.list_error.connect(self.on_list_error)
        self.invite_success.connect(self._on_invite_success)
        self.invite_error.connect(self._on_invite_error)
        self.line_edit_search.textChanged.connect(self.filter_timer.start)
        self.filter_timer.timeout.connect(self.on_filter_timer_timeout)
        self.reset()

    def on_filter_timer_timeout(self):
        self.filter_devices(self.line_edit_search.text())

    def filter_devices(self, pattern):
        pattern = pattern.lower()
        for i in range(self.layout_devices.count()):
            item = self.layout_devices.itemAt(i)
            if item:
                w = item.widget()
                if pattern and pattern not in w.device_name.lower():
                    w.hide()
                else:
                    w.show()

    def change_password(self, device_name):
        PasswordChangeWidget.exec_modal(core=self.core, parent=self)

    def invite_device(self):
        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "invite_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "invite_error", QtToTrioJob),
            _do_invite_device,
            device=self.core.device,
            config=self.core.config,
        )

    def _on_invite_success(self, job):
        GreetDeviceWidget.exec_modal(core=self.core, jobs_ctx=self.jobs_ctx, invite_addr=job.ret, parent=self)
        self.reset()

    def _on_invite_error(self, job):
        show_error(_("TEXT_DEVICES_CANNOT_INVITE_DEVICE"))

    def add_device(self, device_name, is_current_device, certified_on):
        button = DeviceButton(device_name, is_current_device, certified_on)
        self.layout_devices.addWidget(button)
        button.change_password_clicked.connect(self.change_password)
        button.show()

    def on_list_success(self, job):
        devices = job.ret
        current_device = self.core.device
        self.layout_devices.clear()
        for device in devices:
            device_name = device.device_id.device_name
            self.add_device(
                device_name,
                is_current_device=device_name == current_device.device_id.device_name,
                certified_on=device.timestamp,
            )

    def on_list_error(self, job):
        pass

    def reset(self):
        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "list_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "list_error", QtToTrioJob),
            _do_list_devices,
            core=self.core,
        )
