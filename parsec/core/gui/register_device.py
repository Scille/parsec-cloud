from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QDialog

from parsec.core.gui.ui.register_device import Ui_RegisterDevice
from parsec.core.gui.core_call import core_call
from parsec.core.backend_connection import BackendNotAvailable


# TODO: rename to DeclareDevice
class RegisterDevice(QDialog, Ui_RegisterDevice):
    device_try_claim_submitted_qt = pyqtSignal(str, str, str)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.reset()

        self.button_register_device.clicked.connect(self.emit_register_device)
        core_call().connect_signal(
            "backend.device.try_claim_submitted", self.on_device_try_claim_submitted_trio
        )
        self.device_try_claim_submitted_qt.connect(self.on_device_try_claim_submitted_qt)

    def on_device_try_claim_submitted_trio(self, sender, device_name, config_try_id):
        self.device_try_claim_submitted_qt.emit(sender, device_name, config_try_id)

    def on_device_try_claim_submitted_qt(self, sender, device_name, config_try_id):
        password = self.password.text()
        # TODO: better errors handling
        try:
            core_call().accept_device_configuration_try(config_try_id, password)
        except BackendNotAvailable as exc:
            self.device_config_done("Error: cannot declare device while offline")
        except Exception as exc:
            self.device_config_done(f"An error occured: {str(exc)}")
            raise
        self.device_config_done("Device successfully added, you can close this window")

    def reset(self):
        self.device_name.setText("")
        self.password.setText("")
        self.device_token.setText("")
        self.config_waiter_panel.hide()
        self.button_register_device.show()
        self.outcome_panel.hide()

    def device_config_done(self, status):
        self.config_waiter_panel.hide()
        self.outcome_status.setText(status)
        self.outcome_panel.show()

    def emit_register_device(self):
        self.button_register_device.hide()
        self.config_waiter_panel.show()

        # TODO: better errors handling
        # TODO: API should be higher level
        # TODO: check password validity before declaring device
        try:
            configure_device_token = core_call().declare_device(self.device_name.text())
            self.device_token.setText(configure_device_token)
        except BackendNotAvailable as exc:
            self.device_config_done("Error: cannot declare device while offline")
        except Exception as exc:
            self.device_config_done(f"An error occured: {str(exc)}")
            raise
