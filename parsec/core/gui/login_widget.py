from zxcvbn import zxcvbn

from PyQt5.QtCore import pyqtSignal, QCoreApplication, Qt
from PyQt5.QtWidgets import QWidget, QCompleter

import trio

from parsec.types import BackendOrganizationAddr, DeviceID
from parsec.core import devices_manager
from parsec.core.backend_connection import backend_anonymous_cmds_factory, BackendCmdsBadResponse
from parsec.core.invite_claim import claim_user as core_claim_user
from parsec.core.invite_claim import claim_device as core_claim_device
from parsec.core.gui.desktop import get_default_device
from parsec.core.gui.custom_widgets import show_error, show_info
from parsec.core.gui import settings
from parsec.core.gui.ui.login_widget import Ui_LoginWidget
from parsec.core.gui.ui.login_login_widget import Ui_LoginLoginWidget
from parsec.core.gui.ui.login_register_user_widget import Ui_LoginRegisterUserWidget
from parsec.core.gui.ui.login_register_device_widget import Ui_LoginRegisterDeviceWidget


PASSWORD_STRENGTH_TEXTS = [
    QCoreApplication.translate("LoginWidget", "Very weak"),
    QCoreApplication.translate("LoginWidget", "Weak"),
    QCoreApplication.translate("LoginWidget", "Good"),
    QCoreApplication.translate("LoginWidget", "Strong"),
    QCoreApplication.translate("LoginWidget", "Very strong"),
]


class LoginLoginWidget(QWidget, Ui_LoginLoginWidget):
    login_with_password_clicked = pyqtSignal(str, str)
    login_with_pkcs11_clicked = pyqtSignal(str, str, int, int)

    def __init__(self, core_config, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.core_config = core_config
        self.button_login.clicked.connect(self.emit_login)
        self.devices = []
        self.reset()

    def emit_login(self):
        if self.check_box_use_pkcs11.checkState() == Qt.Unchecked:
            self.login_with_password_clicked.emit(
                self.line_edit_device.text(), self.line_edit_password.text()
            )
        else:
            self.login_with_pkcs11_clicked.emit(
                self.line_edit_device.text(),
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
        self.devices = devices_manager.list_available_devices(self.core_config.config_dir)
        if len(self.devices) == 1:
            self.line_edit_device.setText(self.devices[0][0])
            self.line_edit_password.setFocus()
        elif len(self.devices) > 1:
            last_device = settings.get_value("last_device")
            if last_device and last_device in [d[0] for d in self.devices]:
                self.line_edit_device.setText(last_device)
                self.line_edit_password.setFocus()
            else:
                self.line_edit_device.setFocus()
        else:
            self.line_edit_device.setText("")
            self.line_edit_device.setFocus()
        completer = QCompleter([d[0] for d in self.devices])
        completer.setCaseSensitivity(Qt.CaseInsensitive)
        completer.popup().setStyleSheet("border: 2px solid rgb(46, 145, 208); border-top: 0;")
        self.line_edit_device.setCompleter(completer)


class LoginRegisterUserWidget(QWidget, Ui_LoginRegisterUserWidget):
    user_registered = pyqtSignal()

    def __init__(self, core_config, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.core_config = core_config
        self.button_claim.clicked.connect(self.claim_user)
        self.line_edit_login.textChanged.connect(self.check_infos)
        self.line_edit_device.textChanged.connect(self.check_infos)
        self.line_edit_token.textChanged.connect(self.check_infos)
        self.line_edit_url.textChanged.connect(self.check_infos)
        self.line_edit_password.textChanged.connect(self.password_changed)

    def password_changed(self, text):
        if len(text):
            self.label_password_strength.show()
            result = zxcvbn(text)
            score = result["score"]
            self.label_password_strength.setText(
                QCoreApplication.translate("LoginWidget", "Password strength: {}").format(
                    PASSWORD_STRENGTH_TEXTS[score]
                )
            )
            if score == 0 or score == 1:
                self.label_password_strength.setStyleSheet(
                    "color: white; background-color: rgb(194, 51, 51);"
                )
            elif score == 2:
                self.label_password_strength.setStyleSheet(
                    "color: white; background-color: rgb(219, 125, 58);"
                )
            else:
                self.label_password_strength.setStyleSheet(
                    "color: white; background-color: rgb(55, 130, 65);"
                )
        else:
            self.label_password_strength.hide()

    def check_infos(self, _):
        if (
            len(self.line_edit_login.text())
            and len(self.line_edit_token.text())
            and len(self.line_edit_device.text())
            and len(self.line_edit_url.text())
        ):
            self.button_claim.setDisabled(False)
        else:
            self.button_claim.setDisabled(True)

    def reset(self):
        self.line_edit_login.setText("")
        self.line_edit_password.setText("")
        self.line_edit_password_check.setText("")
        self.line_edit_url.setText("")
        self.line_edit_device.setText(get_default_device())
        self.line_edit_token.setText("")
        self.check_box_use_pkcs11.setCheckState(Qt.Unchecked)
        self.line_edit_password.setDisabled(False)
        self.line_edit_password_check.setDisabled(False)
        self.combo_pkcs11_key.clear()
        self.combo_pkcs11_key.addItem("0")
        self.combo_pkcs11_token.clear()
        self.combo_pkcs11_token.addItem("0")
        self.widget_pkcs11.hide()
        self.label_password_strength.hide()

    def claim_user(self):
        async def _claim_user(
            addr, device_id, token, use_pkcs11, password=None, pkcs11_token=None, pkcs11_key=None
        ):
            async with backend_anonymous_cmds_factory(addr) as cmds:
                device = await core_claim_user(cmds, device_id, token)
            if use_pkcs11:
                devices_manager.save_device_with_pkcs11(
                    self.core_config.config_dir, device, pkcs11_token, pkcs11_key
                )
            else:
                devices_manager.save_device_with_password(
                    self.core_config.config_dir, device, password
                )

        use_pkcs11 = True
        if self.check_box_use_pkcs11.checkState() == Qt.Unchecked:
            use_pkcs11 = False
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
        backend_addr = None
        device_id = None
        try:
            backend_addr = BackendOrganizationAddr(self.line_edit_url.text())
        except:
            show_error(
                self, QCoreApplication.translate("LoginRegisterUserWidget", "URL is invalid.")
            )
            return
        try:
            device_id = DeviceID(
                "{}@{}".format(self.line_edit_login.text(), self.line_edit_device.text())
            )
        except:
            show_error(
                self,
                QCoreApplication.translate(
                    "LoginRegisterUserWidget", "Login or device is invalid."
                ),
            )
            return
        try:
            if use_pkcs11:
                trio.run(
                    _claim_user,
                    backend_addr,
                    device_id,
                    self.line_edit_token.text(),
                    True,
                    None,
                    int(self.line_edit_pkcs11_token.text()),
                    int(self.line_edit_pkcs11.key.text()),
                )
            else:
                trio.run(
                    _claim_user,
                    backend_addr,
                    device_id,
                    self.line_edit_token.text(),
                    False,
                    self.line_edit_password.text(),
                )
            show_info(
                self,
                QCoreApplication.translate(
                    "LoginRegisterUserWidget", "The user has been registered. You can now login."
                ),
            )
            self.user_registered.emit()
        except BackendCmdsBadResponse as exc:
            print(exc)
            show_error(
                self,
                QCoreApplication.translate(
                    "LoginRegisterUserWidget", "Can not register the new user."
                ),
            )
        except:
            import traceback

            traceback.print_exc()
            show_error(
                self,
                QCoreApplication.translate(
                    "LoginRegisterUserWidget", "Can not register the new user."
                ),
            )


class LoginRegisterDeviceWidget(QWidget, Ui_LoginRegisterDeviceWidget):
    device_registered = pyqtSignal()

    def __init__(self, core_config, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.core_config = core_config
        self.button_claim.clicked.connect(self.claim_device)
        self.line_edit_login.textChanged.connect(self.check_infos)
        self.line_edit_device.textChanged.connect(self.check_infos)
        self.line_edit_url.textChanged.connect(self.check_infos)
        self.line_edit_token.textChanged.connect(self.check_infos)
        self.line_edit_password.textChanged.connect(self.password_changed)

    def password_changed(self, text):
        if len(text):
            self.label_password_strength.show()
            result = zxcvbn(text)
            score = result["score"]
            self.label_password_strength.setText(
                QCoreApplication.translate("LoginWidget", "Password strength: {}").format(
                    PASSWORD_STRENGTH_TEXTS[score]
                )
            )
            if score == 0 or score == 1:
                self.label_password_strength.setStyleSheet(
                    "color: white; background-color: rgb(194, 51, 51);"
                )
            elif score == 2:
                self.label_password_strength.setStyleSheet(
                    "color: white; background-color: rgb(219, 125, 58);"
                )
            else:
                self.label_password_strength.setStyleSheet(
                    "color: white; background-color: rgb(55, 130, 65);"
                )
        else:
            self.label_password_strength.hide()

    def reset(self):
        self.line_edit_login.setText("")
        self.line_edit_password.setText("")
        self.line_edit_password_check.setText("")
        self.line_edit_url.setText("")
        self.line_edit_device.setText(get_default_device())
        self.line_edit_token.setText("")
        self.line_edit_password.setEnabled(True)
        self.line_edit_password_check.setEnabled(True)
        self.label_password_strength.hide()
        self.check_box_use_pkcs11.setCheckState(Qt.Unchecked)
        self.combo_pkcs11_key.clear()
        self.combo_pkcs11_key.addItem("0")
        self.combo_pkcs11_token.clear()
        self.combo_pkcs11_token.addItem("0")
        self.widget_pkcs11.hide()

    def check_infos(self, _):
        if (
            len(self.line_edit_login.text())
            and len(self.line_edit_token.text())
            and len(self.line_edit_device.text())
            and len(self.line_edit_url.text())
        ):
            self.button_claim.setDisabled(False)
        else:
            self.button_claim.setDisabled(True)

    def claim_device(self):
        async def _claim_device(
            addr, device_id, token, use_pkcs11, password=None, pkcs11_token=None, pkcs11_key=None
        ):
            async with backend_anonymous_cmds_factory(addr) as cmds:
                device = await core_claim_device(cmds, device_id, token)
            if use_pkcs11:
                devices_manager.save_device_with_pkcs11(
                    self.core_config.config_dir, device, pkcs11_token, pkcs11_key
                )
            else:
                devices_manager.save_device_with_password(
                    self.core_config.config_dir, device, password
                )

        use_pkcs11 = True
        if self.check_box_use_pkcs11.checkState() == Qt.Unchecked:
            use_pkcs11 = False
            if (
                len(self.line_edit_password.text()) > 0
                or len(self.line_edit_password_check.text()) > 0
            ):
                if self.line_edit_password.text() != self.line_edit_password_check.text():
                    show_error(
                        self,
                        QCoreApplication.translate(
                            "LoginRegisterDeviceWidget", "Passwords don't match"
                        ),
                    )
                    return
        backend_addr = None
        device_id = None
        try:
            backend_addr = BackendOrganizationAddr(self.line_edit_url.text())
        except:
            show_error(
                self, QCoreApplication.translate("LoginRegisterDeviceWidget", "URL is invalid.")
            )
            return
        try:
            device_id = DeviceID(
                "{}@{}".format(self.line_edit_login.text(), self.line_edit_device.text())
            )
        except:
            show_error(
                self,
                QCoreApplication.translate(
                    "LoginRegisterDeviceWidget", "Login or device is invalid."
                ),
            )
            return
        try:
            if use_pkcs11:
                trio.run(
                    _claim_device,
                    backend_addr,
                    device_id,
                    self.line_edit_token.text(),
                    True,
                    None,
                    int(self.line_edit_pkcs11_token.text()),
                    int(self.line_edit_pkcs11.key.text()),
                )
            else:
                trio.run(
                    _claim_device,
                    backend_addr,
                    device_id,
                    self.line_edit_token.text(),
                    False,
                    self.line_edit_password.text(),
                )
            show_info(
                self,
                QCoreApplication.translate(
                    "LoginRegisterDeviceWidget",
                    "The device has been registered. You can now login.",
                ),
            )
            self.device_registered.emit()
        except:
            import traceback

            traceback.print_exc()
            show_error(
                self,
                QCoreApplication.translate(
                    "LoginRegisterDeviceWidget", "Can not register the new device."
                ),
            )


class LoginWidget(QWidget, Ui_LoginWidget):
    login_with_password_clicked = pyqtSignal(str, str)
    login_with_pkcs11_clicked = pyqtSignal(str, str, int, int)

    def __init__(self, core_config, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)

        self.login_widget = LoginLoginWidget(core_config)
        self.layout.insertWidget(0, self.login_widget)
        self.register_user_widget = LoginRegisterUserWidget(core_config)
        self.layout.insertWidget(0, self.register_user_widget)
        self.register_device_widget = LoginRegisterDeviceWidget(core_config)
        self.layout.insertWidget(0, self.register_device_widget)
        self.button_login_instead.clicked.connect(self.show_login_widget)
        self.button_register_user_instead.clicked.connect(self.show_register_user_widget)
        self.button_register_device_instead.clicked.connect(self.show_register_device_widget)
        self.login_widget.login_with_password_clicked.connect(self.emit_login_with_password)
        self.login_widget.login_with_pkcs11_clicked.connect(self.emit_login_with_pkcs11)
        self.register_user_widget.user_registered.connect(self.show_login_widget)
        self.register_device_widget.device_registered.connect(self.show_login_widget)
        self.reset()

    def emit_login_with_password(self, login, password):
        self.login_with_password_clicked.emit(login, password)

    def emit_login_with_pkcs11(self, login, pkcs11_pin, pkcs11_key, pkcs11_token):
        self.login_with_pkcs11_clicked.emit(login, pkcs11_pin, pkcs11_key, pkcs11_token)

    def show_login_widget(self):
        self.register_user_widget.hide()
        self.register_device_widget.hide()
        self.login_widget.show()

    def show_register_user_widget(self):
        self.login_widget.hide()
        self.register_device_widget.hide()
        self.register_user_widget.show()

    def show_register_device_widget(self):
        self.login_widget.hide()
        self.register_user_widget.hide()
        self.register_device_widget.show()

    def reset(self):
        self.show_login_widget()
        self.register_user_widget.reset()
        self.register_device_widget.reset()
        self.login_widget.reset()
