# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import pyqtSignal, Qt, QTimer
from PyQt5.QtWidgets import QWidget, QMenu
from PyQt5.QtGui import QPixmap

from parsec.core.gui.trio_thread import JobResultError, ThreadSafeQtSignal, QtToTrioJob
from parsec.core.gui.lang import translate as _, format_datetime
from parsec.core.gui.password_change_dialog import PasswordChangeDialog
from parsec.core.gui.custom_widgets import TaskbarButton, FlowLayout
from parsec.core.gui.custom_dialogs import show_info
from parsec.core.gui.ui.devices_widget import Ui_DevicesWidget
from parsec.core.gui.register_device_dialog import RegisterDeviceDialog
from parsec.core.gui.ui.device_button import Ui_DeviceButton
from parsec.core.remote_devices_manager import RemoteDevicesManagerBackendOfflineError


class DeviceButton(QWidget, Ui_DeviceButton):
    change_password_clicked = pyqtSignal(str)

    def __init__(self, device_name, is_current_device, certified_on, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.is_current_device = is_current_device
        self.label.setPixmap(QPixmap(":/icons/images/icons/personal-computer.png"))
        self.device_name = device_name
        self.certified_on = certified_on
        self.set_display(device_name)
        self.setContextMenuPolicy(Qt.CustomContextMenu)
        self.customContextMenuRequested.connect(self.show_context_menu)

    def set_display(self, value):
        if len(value) > 16:
            value = value[:16] + "-\n" + value[16:]
        if self.is_current_device:
            value += "\n({})".format(_("DEVICE_CURRENT_TEXT"))
        self.label_device.setText(value)

    @property
    def is_revoked(self):
        return self.label.is_revoked

    @is_revoked.setter
    def is_revoked(self, value):
        self.label.is_revoked = value
        self.label.repaint()

    def show_context_menu(self, pos):
        global_pos = self.mapToGlobal(pos)
        menu = QMenu(self)
        action = menu.addAction(_("DEVICE_MENU_SHOW_INFO"))
        action.triggered.connect(self.show_device_info)
        if self.is_current_device:
            action = menu.addAction(_("DEVICE_MENU_CHANGE_PASSWORD"))
            action.triggered.connect(self.change_password)
        menu.exec_(global_pos)

    def show_device_info(self):
        text = f"{self.device_name}\n\n"
        text += _("DEVICE_CREATED_ON_{}").format(format_datetime(self.certified_on, full=True))
        if self.label.is_revoked:
            text += "\n\n"
            text += _("DEVICE_IS_REVOKED")
        show_info(self, text)

    def change_password(self):
        self.change_password_clicked.emit(self.device_name)


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

    def __init__(self, core, jobs_ctx, event_bus, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.setupUi(self)
        self.jobs_ctx = jobs_ctx
        self.core = core
        self.event_bus = event_bus
        self.layout_devices = FlowLayout(spacing=20)
        self.layout_content.addLayout(self.layout_devices)
        self.devices = []
        self.taskbar_buttons = []
        button_add_device = TaskbarButton(
            icon_path=":/icons/images/icons/tray_icons/plus-$STATE.svg"
        )
        button_add_device.clicked.connect(self.register_new_device)
        button_add_device.setToolTip(_("BUTTON_TASKBAR_ADD_DEVICE"))
        self.taskbar_buttons.append(button_add_device)
        self.filter_timer = QTimer()
        self.filter_timer.setInterval(300)
        self.list_success.connect(self.on_list_success)
        self.list_error.connect(self.on_list_error)
        self.line_edit_search.textChanged.connect(self.filter_timer.start)
        self.filter_timer.timeout.connect(self.on_filter_timer_timeout)
        self.reset()

    def get_taskbar_buttons(self):
        return self.taskbar_buttons.copy()

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
        dlg = PasswordChangeDialog(core=self.core, parent=self)
        dlg.exec_()

    def register_new_device(self):
        self.register_device_dialog = RegisterDeviceDialog(
            core=self.core, jobs_ctx=self.jobs_ctx, parent=self
        )
        self.register_device_dialog.exec_()
        self.reset()

    def add_device(self, device_name, is_current_device, certified_on):
        if device_name in self.devices:
            return
        button = DeviceButton(device_name, is_current_device, certified_on)
        self.layout_devices.addWidget(button)
        button.change_password_clicked.connect(self.change_password)
        button.show()
        self.devices.append(device_name)

    def on_list_success(self, job):
        devices = job.ret
        current_device = self.core.device
        self.devices = []
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
