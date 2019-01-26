from PyQt5.QtCore import pyqtSignal, Qt
from PyQt5.QtWidgets import QWidget, QCompleter

from parsec.core import devices_manager
from parsec.core.gui import settings
from parsec.core.gui.claim_user_widget import ClaimUserWidget
from parsec.core.gui.claim_device_widget import ClaimDeviceWidget
from parsec.core.gui.settings_dialog import SettingsDialog
from parsec.core.gui.create_org_dialog import CreateOrgDialog
from parsec.core.gui.ui.login_widget import Ui_LoginWidget
from parsec.core.gui.ui.login_login_widget import Ui_LoginLoginWidget


class LoginLoginWidget(QWidget, Ui_LoginLoginWidget):
    login_with_password_clicked = pyqtSignal(str, str, str)
    login_with_pkcs11_clicked = pyqtSignal(str, str, str, int, int)

    def __init__(self, core_config, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.core_config = core_config
        self.button_login.clicked.connect(self.emit_login)
        self.devices = {}
        self.reset()

    def emit_login(self):
        organization_id, device_id, _ = self.devices[self.line_edit_device.text()]
        if self.check_box_use_pkcs11.checkState() == Qt.Unchecked:
            self.login_with_password_clicked.emit(
                organization_id, device_id, self.line_edit_password.text()
            )
        else:
            self.login_with_pkcs11_clicked.emit(
                organization_id,
                device_id,
                self.line_edit_pkcs11_pin.text(),
                int(self.combo_pkcs11_key.currentText()),
                int(self.combo_pkcs11_token.currentText()),
            )

    def reset(self):
        self.line_edit_password.setText("")
        self.check_box_use_pkcs11.setCheckState(Qt.Unchecked)
        self.line_edit_password.setDisabled(False)
        self.line_edit_pkcs11_pin.setText("")
        self.combo_pkcs11_key.clear()
        self.combo_pkcs11_key.addItem("0")
        self.combo_pkcs11_token.clear()
        self.combo_pkcs11_token.addItem("0")
        self.widget_pkcs11.hide()
        devices = devices_manager.list_available_devices(self.core_config.config_dir)
        # Display devices in `<organization>:<device_id>` format
        self.devices = {f"{o}:{d}": (o, d, t) for o, d, t in devices}
        if len(self.devices) == 1:
            self.line_edit_device.setText(next(iter(self.devices)))
            self.line_edit_password.setFocus()
        elif len(self.devices) > 1:
            last_device = settings.get_value("last_device")
            if last_device and last_device in self.devices:
                self.line_edit_device.setText(last_device)
                self.line_edit_password.setFocus()
            else:
                self.line_edit_device.setFocus()
        else:
            self.line_edit_device.setText("")
            self.line_edit_device.setFocus()
        completer = QCompleter(self.devices.keys())
        completer.setCaseSensitivity(Qt.CaseInsensitive)
        completer.popup().setStyleSheet("border: 2px solid rgb(46, 145, 208); border-top: 0;")
        self.line_edit_device.setCompleter(completer)


class LoginWidget(QWidget, Ui_LoginWidget):
    login_with_password_clicked = pyqtSignal(str, str, str)
    login_with_pkcs11_clicked = pyqtSignal(str, str, str, int, int)

    def __init__(self, core_config, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)

        self.core_config = core_config
        self.login_widget = LoginLoginWidget(core_config)
        self.layout.insertWidget(0, self.login_widget)
        self.claim_user_widget = ClaimUserWidget(core_config)
        self.layout.insertWidget(0, self.claim_user_widget)
        self.claim_device_widget = ClaimDeviceWidget(core_config)
        self.layout.insertWidget(0, self.claim_device_widget)
        self.button_login_instead.clicked.connect(self.show_login_widget)
        self.button_register_user_instead.clicked.connect(self.show_claim_user_widget)
        self.button_register_device_instead.clicked.connect(self.show_claim_device_widget)
        self.login_widget.login_with_password_clicked.connect(self.emit_login_with_password)
        self.login_widget.login_with_pkcs11_clicked.connect(self.emit_login_with_pkcs11)
        self.claim_user_widget.user_claimed.connect(self.show_login_widget)
        self.claim_device_widget.device_claimed.connect(self.show_login_widget)
        self.button_create_org.clicked.connect(self.create_organization)
        self.button_settings.clicked.connect(self.show_settings)
        self.reset()

    def create_organization(self):
        d = CreateOrgDialog(parent=self)
        d.exec_()
        self.login_widget.reset()

    def emit_login_with_password(self, organization_id, device_id, password):
        self.login_with_password_clicked.emit(organization_id, device_id, password)

    def emit_login_with_pkcs11(
        self, organization_id, device_id, pkcs11_pin, pkcs11_key, pkcs11_token
    ):
        self.login_with_pkcs11_clicked.emit(
            organization_id, device_id, pkcs11_pin, pkcs11_key, pkcs11_token
        )

    def show_settings(self):
        settings_dialog = SettingsDialog(self.core_config, parent=self)
        settings_dialog.exec_()

    def show_login_widget(self):
        self.claim_user_widget.hide()
        self.claim_device_widget.hide()
        self.button_login_instead.hide()
        self.button_register_user_instead.show()
        self.button_register_device_instead.show()
        self.login_widget.reset()
        self.login_widget.show()

    def show_claim_user_widget(self):
        self.login_widget.hide()
        self.claim_device_widget.hide()
        self.button_login_instead.show()
        self.button_register_user_instead.hide()
        self.button_register_device_instead.show()
        self.claim_user_widget.reset()
        self.claim_user_widget.show()

    def show_claim_device_widget(self):
        self.login_widget.hide()
        self.claim_user_widget.hide()
        self.button_login_instead.show()
        self.button_register_user_instead.show()
        self.button_register_device_instead.hide()
        self.claim_device_widget.reset()
        self.claim_device_widget.show()

    def reset(self):
        self.show_login_widget()
        self.button_login_instead.hide()
        self.button_register_user_instead.show()
        self.button_register_device_instead.show()
        self.claim_user_widget.reset()
        self.claim_device_widget.reset()
        self.login_widget.reset()
