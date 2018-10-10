from PyQt5.QtCore import pyqtSignal, QCoreApplication, Qt
from PyQt5.QtWidgets import QWidget

from parsec.core.gui.desktop import get_default_device
from parsec.core.gui.custom_widgets import show_error
from parsec.core.gui.ui.login_widget import Ui_LoginWidget
from parsec.core.gui.ui.login_login_widget import Ui_LoginLoginWidget
from parsec.core.gui.ui.login_register_user_widget import Ui_LoginRegisterUserWidget
from parsec.core.gui.ui.login_register_device_widget import Ui_LoginRegisterDeviceWidget


class LoginLoginWidget(QWidget, Ui_LoginLoginWidget):
    login_with_password_clicked = pyqtSignal(str, str)
    login_with_nitrokey_clicked = pyqtSignal(str, str, int, int)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.button_login.clicked.connect(self.emit_login)
        self.reset()

    def emit_login(self):
        if self.check_box_use_nitrokey.checkState() == Qt.Unchecked:
            self.login_with_password_clicked.emit(
                self.combo_devices.currentText(), self.line_edit_password.text()
            )
        else:
            self.login_with_nitrokey_clicked.emit(
                self.combo_devices.currentText(),
                self.line_edit_nitrokey_pin.text(),
                int(self.combo_nitrokey_key.currentText()),
                int(self.combo_nitrokey_token.currentText()),
            )

    def reset(self):
        self.line_edit_password.setText("")
        self.check_box_use_nitrokey.setCheckState(Qt.Unchecked)
        self.line_edit_password.setDisabled(False)
        self.line_edit_nitrokey_pin.setText("")
        self.combo_nitrokey_key.clear()
        self.combo_nitrokey_key.addItem("0")
        self.combo_nitrokey_token.clear()
        self.combo_nitrokey_token.addItem("0")
        self.widget_nitrokey.hide()

    def add_device(self, device_name):
        self.combo_devices.addItem(device_name)


class LoginRegisterUserWidget(QWidget, Ui_LoginRegisterUserWidget):
    register_with_password_clicked = pyqtSignal(str, str, str, str)
    register_with_nitrokey_clicked = pyqtSignal(str, str, str, int, int)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.button_register.clicked.connect(self.emit_register)
        self.line_edit_login.textChanged.connect(self.check_infos)
        self.line_edit_device.textChanged.connect(self.check_infos)
        self.line_edit_token.textChanged.connect(self.check_infos)

    def check_infos(self, _):
        if (
            len(self.line_edit_login.text())
            and len(self.line_edit_token.text())
            and len(self.line_edit_device.text())
        ):
            self.button_register.setDisabled(False)
        else:
            self.button_register.setDisabled(True)

    def reset(self):
        self.line_edit_login.setText("")
        self.line_edit_password.setText("")
        self.line_edit_password_check.setText("")
        self.line_edit_device.setText(get_default_device())
        self.line_edit_token.setText("")
        self.check_box_use_nitrokey.setCheckState(Qt.Unchecked)
        self.line_edit_password.setDisabled(False)
        self.line_edit_password_check.setDisabled(False)
        self.combo_nitrokey_key.clear()
        self.combo_nitrokey_key.addItem("0")
        self.combo_nitrokey_token.clear()
        self.combo_nitrokey_token.addItem("0")
        self.widget_nitrokey.hide()

    def emit_register(self):
        if self.check_box_use_nitrokey.checkState() == Qt.Unchecked:
            if (
                len(self.line_edit_password.text()) > 0
                or len(self.line_edit_password_check.text()) > 0
            ):
                if self.line_edit_password.text() != self.line_edit_password_check.text():
                    show_error(
                        self,
                        QCoreApplication.translate(
                            "LoginRegisterUserWidget", "Passwords don't match"
                        ),
                    )
                    return
            self.register_with_password_clicked.emit(
                self.line_edit_login.text(),
                self.line_edit_password.text(),
                self.line_edit_device.text(),
                self.line_edit_token.text(),
            )
        else:
            self.register_with_nitrokey_clicked.emit(
                self.line_edit_login.text(),
                self.line_edit_device.text(),
                self.line_edit_token.text(),
                int(self.combo_nitrokey_key.currentText()),
                int(self.combo_nitrokey_token.currentText()),
            )


class LoginRegisterDeviceWidget(QWidget, Ui_LoginRegisterDeviceWidget):
    register_with_password_clicked = pyqtSignal(str, str, str, str)
    register_with_nitrokey_clicked = pyqtSignal(str, str, str, str)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.button_register.clicked.connect(self.emit_register)
        self.line_edit_login.textChanged.connect(self.check_infos)
        self.line_edit_device.textChanged.connect(self.check_infos)
        self.line_edit_token.textChanged.connect(self.check_infos)

    def reset(self):
        self.line_edit_login.setText("")
        self.line_edit_password.setText("")
        self.line_edit_password_check.setText("")
        self.line_edit_device.setText(get_default_device())
        self.line_edit_token.setText("")
        self.line_edit_login.setEnabled(True)
        self.line_edit_password.setEnabled(True)
        self.line_edit_password_check.setEnabled(True)
        self.line_edit_device.setEnabled(True)
        self.line_edit_token.setEnabled(True)
        self.check_box_use_nitrokey.setCheckState(Qt.Unchecked)
        self.line_edit_password.setDisabled(False)
        self.line_edit_password_check.setDisabled(False)
        self.set_error(None)
        self.combo_nitrokey_key.clear()
        self.combo_nitrokey_key.addItem("0")
        self.combo_nitrokey_token.clear()
        self.combo_nitrokey_token.addItem("0")
        self.widget_nitrokey.hide()

    def check_infos(self, _):
        if (
            len(self.line_edit_login.text())
            and len(self.line_edit_token.text())
            and len(self.line_edit_device.text())
        ):
            self.button_register.setDisabled(False)
        else:
            self.button_register.setDisabled(True)

    def set_error(self, error):
        if not error:
            self.label_error.setText("")
            self.label_error.hide()
        else:
            self.label_error.setText(error)
            self.label_error.show()

    def emit_register(self):
        if self.check_box_use_nitrokey.checkState() == Qt.Unchecked:
            if (
                len(self.line_edit_password.text()) > 0
                or len(self.line_edit_password_check.text()) > 0
            ):
                if self.line_edit_password.text() != self.line_edit_password_check.text():
                    show_error(
                        self,
                        QCoreApplication.translate(
                            "LoginRegisterDeviceWidget", "Passwords don't match."
                        ),
                    )
                    return
            self.button_register.hide()
            self.register_with_password_clicked.emit(
                self.line_edit_login.text(),
                self.line_edit_password.text(),
                self.line_edit_device.text(),
                self.line_edit_token.text(),
            )
        else:
            self.register_with_nitrokey_clicked.emit(
                self.line_edit_login.text(),
                self.line_edit_device.text(),
                self.line_edit_token.text(),
                int(self.combo_nitrokey_key.currentText()),
                int(self.combo_nitrokey_token.currentText()),
            )
        self.set_error(
            QCoreApplication.translate(
                "LoginRegisterDeviceWidget", "Waiting for existing device to register us..."
            )
        )
        self.line_edit_login.setEnabled(False)
        self.line_edit_password.setEnabled(False)
        self.line_edit_password_check.setEnabled(False)
        self.line_edit_device.setEnabled(False)
        self.line_edit_token.setEnabled(False)


class LoginWidget(QWidget, Ui_LoginWidget):
    login_with_password_clicked = pyqtSignal(str, str)
    login_with_nitrokey_clicked = pyqtSignal(str, str, int, int)
    register_user_with_password_clicked = pyqtSignal(str, str, str, str)
    register_user_with_nitrokey_clicked = pyqtSignal(str, str, str, int, int)
    register_device_with_password_clicked = pyqtSignal(str, str, str, str)
    register_device_with_nitrokey_clicked = pyqtSignal(str, str, str, int, int)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)

        self.login_widget = LoginLoginWidget()
        self.layout().insertWidget(0, self.login_widget)
        self.register_user_widget = LoginRegisterUserWidget()
        self.layout().insertWidget(0, self.register_user_widget)
        self.register_user_widget.hide()
        self.register_device_widget = LoginRegisterDeviceWidget()
        self.layout().insertWidget(0, self.register_device_widget)
        self.register_device_widget.hide()
        self.button_login_instead.clicked.connect(self.show_login_widget)
        self.button_register_user_instead.clicked.connect(self.show_register_user_widget)
        self.button_register_device_instead.clicked.connect(self.show_register_device_widget)
        self.login_widget.login_with_password_clicked.connect(self.emit_login_with_password)
        self.login_widget.login_with_nitrokey_clicked.connect(self.emit_login_with_nitrokey)
        self.register_user_widget.register_with_password_clicked.connect(
            self.emit_register_user_with_password
        )
        self.register_user_widget.register_with_nitrokey_clicked.connect(
            self.emit_register_user_with_nitrokey
        )
        self.register_device_widget.register_with_password_clicked.connect(
            self.emit_register_device_with_password
        )
        self.register_device_widget.register_with_nitrokey_clicked.connect(
            self.emit_register_device_with_nitrokey
        )
        self.reset()

    def emit_register_user_with_password(self, login, password, device, token):
        self.register_user_with_password_clicked.emit(login, password, device, token)

    def emit_register_user_with_nitrokey(self, login, device, token, nitrokey_key, nitrokey_token):
        self.register_user_with_nitrokey_clicked.emit(
            login, device, token, nitrokey_key, nitrokey_token
        )

    def emit_register_device_with_password(self, login, password, device, token):
        self.register_device_with_password_clicked.emit(login, password, device, token)

    def emit_register_device_with_nitrokey(
        self, login, device, token, nitrokey_key, nitrokey_token
    ):
        self.register_device_with_nitrokey_clicked.emit(
            login, device, token, nitrokey_key, nitrokey_token
        )

    def emit_login_with_password(self, login, password):
        self.login_with_password_clicked.emit(login, password)

    def emit_login_with_nitrokey(self, login, nitrokey_pin, nitrokey_key, nitrokey_token):
        self.login_with_nitrokey_clicked.emit(login, nitrokey_pin, nitrokey_key, nitrokey_token)

    def show_login_widget(self):
        self.login_widget.show()
        self.register_user_widget.hide()
        self.register_device_widget.hide()

    def show_register_user_widget(self):
        self.login_widget.hide()
        self.register_user_widget.show()
        self.register_device_widget.hide()

    def show_register_device_widget(self):
        self.login_widget.hide()
        self.register_user_widget.hide()
        self.register_device_widget.show()

    def add_device(self, device_name):
        self.login_widget.add_device(device_name)

    def reset(self):
        self.login_widget.show()
        self.register_device_widget.hide()
        self.register_user_widget.hide()
        self.button_login_instead.hide()
        self.button_register_device_instead.show()
        self.button_register_user_instead.show()
        self.login_widget.reset()
        self.register_user_widget.reset()
        self.register_device_widget.reset()
