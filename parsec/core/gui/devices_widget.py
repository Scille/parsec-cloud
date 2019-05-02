# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pendulum
from PyQt5.QtCore import pyqtSignal, Qt
from PyQt5.QtWidgets import QWidget, QMenu
from PyQt5.QtGui import QPixmap

from parsec.types import DeviceID
from parsec.crypto import build_device_certificate
from parsec.core.backend_connection import BackendNotAvailable, BackendCmdsBadResponse

from parsec.core.gui.lang import translate as _
from parsec.core.gui.custom_widgets import TaskbarButton, show_info, show_error, ask_question
from parsec.core.gui.ui.devices_widget import Ui_DevicesWidget
from parsec.core.gui.register_device_dialog import RegisterDeviceDialog
from parsec.core.gui.ui.device_button import Ui_DeviceButton


class DeviceButton(QWidget, Ui_DeviceButton):
    revoke_clicked = pyqtSignal(QWidget)

    def __init__(self, device_name, is_current_device, is_revoked, revoked_on, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.revoked_on = revoked_on
        self.label.is_revoked = is_revoked
        if is_current_device:
            self.label.setPixmap(QPixmap(":/icons/images/icons/personal-computer.png"))
        else:
            self.label.setPixmap(QPixmap(":/icons/images/icons/personal-computer.png"))
        self.name = device_name
        self.setContextMenuPolicy(Qt.CustomContextMenu)
        self.customContextMenuRequested.connect(self.show_context_menu)

    @property
    def name(self):
        return self._name

    @name.setter
    def name(self, value):
        self._name = value
        if len(value) > 16:
            value = value[:16] + "-\n" + value[16:]
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
        action = menu.addAction(_("Show info"))
        action.triggered.connect(self.show_info)
        if not self.label.is_revoked:
            action = menu.addAction(_("Revoke"))
            action.triggered.connect(self.revoke)
        menu.exec_(global_pos)

    def show_info(self):
        text = "{}".format(self.name)
        if self.label.is_revoked:
            text += "\n\n"
            text += _("This device has been revoked.")
        show_info(self, text)

    def revoke(self):
        self.revoke_clicked.emit(self)


class DevicesWidget(QWidget, Ui_DevicesWidget):
    def __init__(self, core, jobs_ctx, event_bus, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.setupUi(self)
        self.jobs_ctx = jobs_ctx
        self.core = core
        self.event_bus = event_bus
        self.devices = []
        self.taskbar_buttons = []
        button_add_device = TaskbarButton(icon_path=":/icons/images/icons/plus_off.png")
        button_add_device.clicked.connect(self.register_new_device)
        self.taskbar_buttons.append(button_add_device)
        self.line_edit_search.textChanged.connect(self.filter_devices)
        self.reset()

    def disconnect_all(self):
        pass

    def get_taskbar_buttons(self):
        return self.taskbar_buttons.copy()

    def filter_devices(self, pattern):
        pattern = pattern.lower()
        for i in range(self.layout_devices.count()):
            item = self.layout_devices.itemAt(i)
            if item:
                w = item.widget()
                if pattern and pattern not in w.name.lower():
                    w.hide()
                else:
                    w.show()

    def revoke_device(self, device_button):
        device_name = device_button.name
        result = ask_question(
            self,
            _("Confirmation"),
            _('Are you sure you want to revoke device "{}" ?').format(device_name),
        )
        if not result:
            return

        try:
            revoked_device_certificate = build_device_certificate(
                self.core.device.device_id,
                self.core.device.signing_key,
                DeviceID(f"{self.core.device.device_id.user_id}@{device_name}"),
                pendulum.now(),
            )
            self.jobs_ctx.run(self.core.fs.backend_cmds.device_revoke, revoked_device_certificate)
            device_button.is_revoked = True
            show_info(self, _('Device "{}" has been revoked.').format(device_name))
        except BackendCmdsBadResponse as exc:
            if exc.status == "already_revoked":
                show_error(self, _('Device "{}" has already been revoked.').format(device_name))
            elif exc.status == "not_found":
                show_error(self, _('Device "{}" not found.').format(device_name))
            elif exc.status == "invalid_role" or exc.status == "invalid_certification":
                show_error(self, _("You don't have the permission to revoke this device."))
        except:
            show_error(self, _("Can not revoke this device."))

    def register_new_device(self):
        self.register_device_dialog = RegisterDeviceDialog(
            core=self.core, jobs_ctx=self.jobs_ctx, parent=self
        )
        self.register_device_dialog.exec_()
        self.reset()

    def add_device(self, device_name, is_current_device, is_revoked, revoked_on):
        if device_name in self.devices:
            return
        button = DeviceButton(device_name, is_current_device, is_revoked, revoked_on)
        self.layout_devices.addWidget(
            button, int(len(self.devices) / 4), int(len(self.devices) % 4)
        )
        button.revoke_clicked.connect(self.revoke_device)
        self.devices.append(device_name)

    def reset(self):
        self.devices = []
        while self.layout_devices.count() != 0:
            item = self.layout_devices.takeAt(0)
            if item:
                w = item.widget()
                self.layout_devices.removeWidget(w)
                w.setParent(None)
        try:
            current_device = self.core.device
            _, devices = self.jobs_ctx.run(
                self.core.remote_devices_manager.get_user_and_devices, self.core.device.user_id
            )
            for device in devices:
                self.add_device(
                    device.device_name,
                    is_current_device=device.device_name == current_device.device_name,
                    is_revoked=bool(device.revoked_on),
                    revoked_on=device.revoked_on,
                )
        except BackendNotAvailable:
            pass
        except:
            pass
