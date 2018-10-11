from PyQt5.QtCore import pyqtSignal, Qt, QCoreApplication
from PyQt5.QtWidgets import QDialog

from parsec.core.devices_manager import DeviceNitrokeyError, DeviceConfigurationPasswordError

from parsec.core.gui.ui.register_device import Ui_RegisterDevice
from parsec.core.gui.core_call import core_call
from parsec.core.gui.custom_widgets import show_error
from parsec.core.backend_connection import BackendNotAvailable


# TODO: rename to DeclareDevice
class RegisterDevice(QDialog, Ui_RegisterDevice):
    device_try_claim_submitted_qt = pyqtSignal(str, str, str)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.setWindowFlags(Qt.SplashScreen)
        self.reset()

        self.button_register_device.clicked.connect(self.emit_register_device)
        core_call().connect_event(
            "backend.device.try_claim_submitted", self.on_device_try_claim_submitted_trio
        )
        self.device_try_claim_submitted_qt.connect(self.on_device_try_claim_submitted_qt)

    def on_device_try_claim_submitted_trio(self, sender, device_name, config_try_id):
        self.device_try_claim_submitted_qt.emit(sender, device_name, config_try_id)

    def on_device_try_claim_submitted_qt(self, sender, device_name, config_try_id):
        password = self.password.text()
        try:
            if self.check_box_use_nitrokey.checkState() == Qt.Unchecked:
                core_call().accept_device_configuration_try(config_try_id, password)
            else:
                core_call().accept_device_configuration_try(
                    config_try_id,
                    nitrokey_pin=self.line_edit_nitrokey_pin.text(),
                    nitrokey_token_id=int(self.combo_nitrokey_token.currentText()),
                    nitrokey_key_id=int(self.combo_nitrokey_key.currentText()),
                )
            self.device_config_done(
                QCoreApplication.translate(
                    "RegisterDevice", "Device successfully added, you can close this window."
                )
            )
        except BackendNotAvailable:
            show_error(
                self, QCoreApplication.translate("RegisterDevice", "Can not reach the server.")
            )
        except DeviceNitrokeyError:
            show_error(
                self, QCoreApplication.translate("RegisterDevice", "Invalid NitroKey information.")
            )
        except DeviceConfigurationPasswordError:
            show_error(
                self,
                QCoreApplication.translate("RegisterDevice", "Invalid password or NitroKey PIN."),
            )
        except:
            show_error(
                self,
                QCoreApplication.translate(
                    "RegisterDevice", "An unknown error occured. Can not register the new device."
                ),
            )
        finally:
            self.button_box.setEnabled(True)

    def reset(self):
        self.device_name.setText("")
        self.password.setText("")
        self.device_token.setText("")
        self.config_waiter_panel.hide()
        self.button_register_device.show()
        self.outcome_panel.hide()
        self.outcome_status.setText("")
        self.check_box_use_nitrokey.setCheckState(Qt.Unchecked)
        self.widget_nitrokey.hide()
        self.password.setDisabled(False)
        self.line_edit_nitrokey_pin.setText("")
        self.combo_nitrokey_key.clear()
        self.combo_nitrokey_key.addItem("0")
        self.combo_nitrokey_token.clear()
        self.combo_nitrokey_token.addItem("0")
        self.button_box.setEnabled(True)

    def device_config_done(self, status):
        self.config_waiter_panel.hide()
        self.outcome_status.setText(status)
        self.outcome_panel.show()

    def emit_register_device(self):
        self.button_box.setEnabled(False)
        self.button_register_device.hide()
        self.config_waiter_panel.show()

        try:
            configure_device_token = core_call().declare_device(self.device_name.text())
            self.device_token.setText(configure_device_token)
        except BackendNotAvailable:
            show_error(
                self, QCoreApplication.translate("RegisterDevice", "Can not reach the server.")
            )
            self.button_box.setEnabled(True)
        except:
            show_error(
                self,
                QCoreApplication.translate(
                    "RegisterDevice", "An unknown error occured. Can not register the new device."
                ),
            )
            self.button_box.setEnabled(True)
