from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget

from parsec.core.gui.ui.login_widget import Ui_LoginWidget


class LoginWidget(QWidget, Ui_LoginWidget):
    loginClicked = pyqtSignal(str, str)
    registerClicked = pyqtSignal(str, str, str)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.setupUi(self)
        self.connect_all()

    def connect_all(self):
        self.button_login.clicked.connect(self.emit_login)
        self.line_edit_password.textChanged.connect(self.check_infos)
        self.button_register.clicked.connect(self.emit_register)
        self.line_edit_register_login.textChanged.connect(self.check_register_infos)
        self.line_edit_register_password.textChanged.connect(self.check_register_infos)
        self.line_edit_register_password_check.textChanged.connect(self.check_register_infos)
        self.line_edit_register_device.textChanged.connect(self.check_register_infos)

    def add_device(self, device_name):
        self.combo_devices.addItem(device_name)

    def check_infos(self, _):
        if len(self.combo_devices.currentText()) and len(self.line_edit_password.text()):
            self.button_login.setDisabled(False)
        else:
            self.button_login.setDisabled(True)
        self.label_error.setText('')

    def check_register_infos(self, _):
        if (len(self.line_edit_register_login.text()) and
             len(self.line_edit_register_password.text()) and
             len(self.line_edit_register_password_check.text()) and
             len(self.line_edit_register_device.text())):
            self.button_register.setDisabled(False)
        else:
            self.button_register.setDisabled(True)
        self.label_register_error.setText('')

    def reset(self):
        self.line_edit_register_login.setText('')
        self.line_edit_register_password.setText('')
        self.line_edit_register_password_check.setText('')
        self.line_edit_register_device.setText('')
        self.line_edit_password.setText('')
        self.button_register.setDisabled(True)
        self.button_login.setDisabled(True)
        self.label_error.setText('')
        self.label_register_error.setText('')

    def emit_login(self):
        self.loginClicked.emit(self.combo_devices.currentText(), self.line_edit_password.text())

    def emit_register(self):
        if (self.line_edit_register_password.text() !=
                self.line_edit_register_password_check.text()):
            self.set_register_error("Passwords don't match.")
            return
        self.registerClicked.emit(self.line_edit_register_login.text(),
                                  self.line_edit_register_password.text(),
                                  self.line_edit_register_device.text())

    def set_login_error(self, error):
        self.label_error.setText(error)

    def set_register_error(self, error):
        self.label_register_error.setText(error)
