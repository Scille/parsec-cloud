from PyQt5.QtWidgets import QWidget
from PyQt5.QtGui import QPixmap

from parsec.core.gui.core_widget import CoreWidget
from parsec.core.gui.custom_widgets import TaskbarButton
from parsec.core.gui.ui.devices_widget import Ui_DevicesWidget
from parsec.core.gui.register_device_dialog import RegisterDeviceDialog
from parsec.core.gui.ui.device_button import Ui_DeviceButton


class DeviceButton(QWidget, Ui_DeviceButton):
    def __init__(self, device_name, is_current_device, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        if is_current_device:
            self.label.setPixmap(QPixmap(":/icons/images/icons/personal-computer.png"))
        else:
            self.label.setPixmap(QPixmap(":/icons/images/icons/personal-computer.png"))
        self.label_device.setText(device_name)

    @property
    def name(self):
        return self.label_device.text()

    @name.setter
    def name(self, value):
        self.label_device.setText(value)


class DevicesWidget(CoreWidget, Ui_DevicesWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.setupUi(self)
        self.devices = []
        self.taskbar_buttons = []
        button_add_device = TaskbarButton(icon_path=":/icons/images/icons/plus_off.png")
        button_add_device.clicked.connect(self.register_new_device)
        self.taskbar_buttons.append(button_add_device)
        self.line_edit_search.textChanged.connect(self.filter_devices)

    def get_taskbar_buttons(self):
        return self.taskbar_buttons

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

    def register_new_device(self):
        self.register_device_dialog = RegisterDeviceDialog(
            parent=self, portal=self.portal, core=self.core
        )
        self.register_device_dialog.exec_()
        self.reset()

    def add_device(self, device_name, is_current_device):
        if device_name in self.devices:
            return
        button = DeviceButton(device_name, is_current_device)
        self.layout_devices.addWidget(
            button, int(len(self.devices) / 4), int(len(self.devices) % 4)
        )
        self.devices.append(device_name)

    def reset(self):
        self.line_edit_search.setText("")
        self.devices = []
        while self.layout_devices.count() != 0:
            item = self.layout_devices.takeAt(0)
            if item:
                w = item.widget()
                self.layout_devices.removeWidget(w)
                w.setParent(None)
        if self.portal and self.core:
            try:
                current_device = self.core.device
                user = self.portal.run(self.core.backend_cmds.user_get, self.core.device.user_id)
                for device in user[0].devices:
                    self.add_device(device, is_current_device=device == current_device.device_name)
            except:
                import traceback

                traceback.print_exc()
                pass
