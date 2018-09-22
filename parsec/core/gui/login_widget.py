from PyQt5.QtCore import pyqtSignal, QCoreApplication
from PyQt5.QtWidgets import QWidget

from parsec.core.gui.desktop import get_default_device
from parsec.core.gui.ui.login_widget import Ui_LoginWidget


class LoginWidget(QWidget, Ui_LoginWidget):
    loginClicked = pyqtSignal(str, str)
    claimClicked = pyqtSignal(str, str, str, str)
    configureDeviceClicked = pyqtSignal(str, str, str, str)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.setupUi(self)
        self.button_login_instead.hide()
        self.group_claim.hide()
        self.group_config_device.hide()
        self.connect_all()
        self.label_error.hide()
        self.label_claim_error.hide()
        self.line_edit_claim_device.setText(get_default_device())

        self.device_progress_bar.hide()
        self.device_line_edit_device.setText(get_default_device())

    def connect_all(self):
        self.button_login.clicked.connect(self.emit_login)

        self.button_claim.clicked.connect(self.emit_claim)
        self.line_edit_claim_login.textChanged.connect(self.user_check_claim_infos)
        self.line_edit_claim_device.textChanged.connect(self.user_check_claim_infos)
        self.line_edit_claim_token.textChanged.connect(self.user_check_claim_infos)

        self.device_button_config.clicked.connect(self.emit_config_device)
        self.device_line_edit_login.textChanged.connect(self.device_check_config_infos)
        self.device_line_edit_device.textChanged.connect(self.device_check_config_infos)
        self.device_line_edit_token.textChanged.connect(self.device_check_config_infos)

    def add_device(self, device_name):
        self.combo_devices.addItem(device_name)

    def user_check_claim_infos(self, _):
        if (
            len(self.line_edit_claim_login.text())
            and len(self.line_edit_claim_token.text())
            and len(self.line_edit_claim_device.text())
        ):
            self.button_claim.setDisabled(False)
        else:
            self.button_claim.setDisabled(True)
        self.label_claim_error.setText("")

    def device_check_config_infos(self, _):
        if (
            len(self.device_line_edit_login.text())
            and len(self.device_line_edit_token.text())
            and len(self.device_line_edit_device.text())
        ):
            self.device_button_config.setDisabled(False)
        else:
            self.device_button_config.setDisabled(True)
        self.device_label_error.setText("")

    def reset(self):
        self.line_edit_claim_login.setText("")
        self.line_edit_claim_password.setText("")
        self.line_edit_claim_password_check.setText("")
        self.line_edit_claim_device.setText(get_default_device())
        self.line_edit_claim_token.setText("")
        self.line_edit_password.setText("")
        self.button_claim.setDisabled(True)
        self.label_error.setText("")
        self.label_claim_error.setText("")
        self.group_login.show()
        self.group_claim.hide()
        self.label_error.hide()
        self.label_claim_error.hide()
        self.button_login_instead.hide()

        self.device_progress_bar.hide()
        self.device_line_edit_login.setEnabled(True)
        self.device_line_edit_password.setEnabled(True)
        self.device_line_edit_device.setEnabled(True)
        self.device_line_edit_token.setEnabled(True)
        self.device_line_edit_login.setText("")
        self.device_line_edit_password.setText("")
        self.device_line_edit_device.setText("")
        self.device_line_edit_token.setText("")
        self.device_label_error.hide()
        self.device_button_config.show()

    def emit_login(self):
        self.loginClicked.emit(self.combo_devices.currentText(), self.line_edit_password.text())

    def emit_claim(self):
        if (
            len(self.line_edit_claim_password.text()) > 0
            or len(self.line_edit_claim_password_check.text()) > 0
        ):
            if self.line_edit_claim_password.text() != self.line_edit_claim_password_check.text():
                self.set_claim_error(
                    QCoreApplication.translate("LoginWidget", "Passwords don't match.")
                )
                self.label_claim_error.show()
                return
        self.claimClicked.emit(
            self.line_edit_claim_login.text(),
            self.line_edit_claim_password.text(),
            self.line_edit_claim_device.text(),
            self.line_edit_claim_token.text(),
        )

    def emit_config_device(self):
        if (
            len(self.device_line_edit_password.text()) > 0
            or len(self.device_line_edit_password_check.text()) > 0
        ):
            if self.device_line_edit_password.text() != self.device_line_edit_password_check.text():
                self.set_device_config_error(
                    QCoreApplication.translate("LoginWidget", "Passwords don't match.")
                )
                self.label_claim_error.show()
                return

        self.device_button_config.hide()
        self.set_device_config_error("Waiting for existing device to register us...")
        self.device_progress_bar.show()
        self.configureDeviceClicked.emit(
            self.device_line_edit_login.text(),
            self.device_line_edit_password.text(),
            self.device_line_edit_device.text(),
            self.device_line_edit_token.text(),
        )

    def set_device_working(self):
        self.device_line_edit_login.setEnabled(False)
        self.device_line_edit_password.setEnabled(False)
        self.device_line_edit_device.setEnabled(False)
        self.device_line_edit_token.setEnabled(False)
        self.device_progress_bar.show()

    def set_login_error(self, error):
        self.label_error.setText(error)
        self.label_error.show()

    def set_claim_error(self, error):
        self.label_claim_error.setText(error)
        self.label_claim_error.show()

    def set_device_config_error(self, error):
        self.device_label_error.setText(error)
        self.device_label_error.show()
        self.device_progress_bar.hide()
        self.device_button_config.show()
