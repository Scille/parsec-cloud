# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import pyqtSignal, Qt, QTimer
from PyQt5.QtWidgets import QWidget, QMenu, QGraphicsDropShadowEffect, QLabel
from PyQt5.QtGui import QColor

from parsec.core.backend_connection import BackendNotAvailable, BackendConnectionError
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
    change_password_clicked = pyqtSignal()

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
        self.change_password_clicked.emit()


async def _do_invite_device(core):
    try:
        return await core.new_device_invitation(send_email=False)
    except BackendNotAvailable as exc:
        raise JobResultError("offline") from exc
    except BackendConnectionError as exc:
        raise JobResultError("error") from exc


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
        self.list_success.connect(self._on_list_success)
        self.list_error.connect(self._on_list_error)
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

    def change_password(self):
        PasswordChangeWidget.exec_modal(core=self.core, parent=self)

    def invite_device(self):
        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "invite_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "invite_error", QtToTrioJob),
            _do_invite_device,
            core=self.core,
        )

    def _on_invite_success(self, job):
        assert job.is_finished()
        assert job.status == "ok"

        GreetDeviceWidget.exec_modal(
            core=self.core,
            jobs_ctx=self.jobs_ctx,
            invite_addr=job.ret,
            parent=self,
            on_finished=self.reset,
        )

    def _on_invite_error(self, job):
        assert job.is_finished()
        assert job.status != "ok"

        status = job.status
        if status == "offline":
            errmsg = _("TEXT_INVITE_DEVICE_INVITE_OFFLINE")
        else:
            errmsg = _("TEXT_INVITE_DEVICE_INVITE_ERROR")

        show_error(self, errmsg, exception=job.exc)

    def add_device(self, device_info, is_current_device):
        button = DeviceButton(device_info, is_current_device)
        self.layout_devices.addWidget(button)
        button.change_password_clicked.connect(self.change_password)
        button.show()

    def _flush_devices_list(self):
        self.layout_devices.clear()

    def _on_list_success(self, job):
        assert job.is_finished()
        assert job.status == "ok"

        devices = job.ret
        current_device = self.core.device
        self._flush_devices_list()
        for device in devices:
            self.add_device(device, is_current_device=current_device.device_id == device.device_id)

    def _on_list_error(self, job):
        assert job.is_finished()
        assert job.status != "ok"

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
