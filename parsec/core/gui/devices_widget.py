# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import pyqtSignal, Qt, QTimer
from PyQt5.QtWidgets import QWidget, QMenu, QGraphicsDropShadowEffect, QLabel
from PyQt5.QtGui import QColor

from parsec.api.protocol import InvitationType
from parsec.core.backend_connection import (
    BackendNotAvailable,
    backend_authenticated_cmds_factory,
    BackendConnectionError,
)
from parsec.core.types import BackendInvitationAddr

from parsec.core.gui.trio_thread import JobResultError, ThreadSafeQtSignal, QtToTrioJob
from parsec.core.gui.greet_device_widget import GreetDeviceWidget
from parsec.core.gui.lang import translate as _
from parsec.core.gui.custom_widgets import ensure_string_size
from parsec.core.gui.custom_dialogs import show_error
from parsec.core.gui.password_change_widget import PasswordChangeWidget
from parsec.core.gui.flow_layout import FlowLayout
from parsec.core.gui.ui.devices_widget import Ui_DevicesWidget
from parsec.core.gui.ui.device_button import Ui_DeviceButton


class DeviceButton(QWidget, Ui_DeviceButton):
    change_password_clicked = pyqtSignal(str)

    def __init__(self, device_info, is_current_device):
        super().__init__()
        self.setupUi(self)
        self.is_current_device = is_current_device
        self.device_info = device_info
        self.label_icon.apply_style()

        self.label_device_name.setText(
            ensure_string_size(self.device_info.device_display, 260, self.label_device_name.font())
        )
        self.label_device_name.setToolTip(self.device_info.device_display)
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

    def show(self):
        self.reset()
        super().show()

    def on_filter_timer_timeout(self):
        self.filter_devices(self.line_edit_search.text())

    def filter_devices(self, pattern):
        pattern = pattern.lower()
        for i in range(self.layout_devices.count()):
            item = self.layout_devices.itemAt(i)
            if item:
                w = item.widget()
                if pattern and pattern not in w.device_info.device_display.lower():
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
        GreetDeviceWidget.exec_modal(
            core=self.core, jobs_ctx=self.jobs_ctx, invite_addr=job.ret, parent=self
        )
        self.reset()

    def _on_invite_error(self, job):
        show_error(_("TEXT_DEVICES_CANNOT_INVITE_DEVICE"))

    def add_device(self, device_info, is_current_device):
        button = DeviceButton(device_info, is_current_device)
        self.layout_devices.addWidget(button)
        button.change_password_clicked.connect(self.change_password)
        button.show()

    def _flush_devices_list(self):
        self.layout_devices.clear()

    def on_list_success(self, job):
        devices = job.ret
        current_device = self.core.device
        self._flush_devices_list()
        for device in devices:
            self.add_device(device, is_current_device=current_device.device_id == device.device_id)

    def on_list_error(self, job):
        status = job.status
        if status == "error":
            self._flush_devices_list()
            label = QLabel(_("TEXT_DEVICE_LIST_RETRIEVABLE_FAILURE"))
            label.setAlignment(Qt.AlignHCenter | Qt.AlignVCenter)
            self.layout_devices.addWidget(label)

    def reset(self):
        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "list_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "list_error", QtToTrioJob),
            _do_list_devices,
            core=self.core,
        )
