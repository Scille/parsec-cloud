from PyQt5.QtWidgets import QWidget

from parsec.core.gui.ui.devices_widget import Ui_DevicesWidget
from parsec.core.gui.register_device_dialog import RegisterDeviceDialog


class DevicesWidget(QWidget, Ui_DevicesWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.setupUi(self)
        self.button_add_device.clicked.connect(self.emit_register_device)

    def emit_register_device(self):
        self.register_device_dialog = RegisterDeviceDialog(parent=self)
        self.register_device_dialog.show()

    def reset(self):
        pass
