from PyQt5.QtCore import pyqtSignal, QSysInfo, QCoreApplication
from PyQt5.QtWidgets import QWidget

from parsec.core.gui.ui.login_widget import Ui_LoginWidget


class LoginWidget(QWidget, Ui_LoginWidget):
    loginClicked = pyqtSignal(str, str)
    claimClicked = pyqtSignal(str, str, str, str)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.setupUi(self)
        self.group_claim.hide()
        self.connect_all()
        self.label_error.hide()
        self.label_claim_error.hide()
        self.line_edit_claim_device.setText(QSysInfo.productType())

    def connect_all(self):
        self.button_login.clicked.connect(self.emit_login)
        self.button_claim.clicked.connect(self.emit_claim)
        self.line_edit_claim_login.textChanged.connect(self.check_claim_infos)
        self.line_edit_claim_device.textChanged.connect(self.check_claim_infos)
        self.line_edit_claim_token.textChanged.connect(self.check_claim_infos)

    def add_device(self, device_name):
        self.combo_devices.addItem(device_name)

    def check_claim_infos(self, _):
        if (len(self.line_edit_claim_login.text()) and
             len(self.line_edit_claim_token.text()) and
             len(self.line_edit_claim_device.text())):
            self.button_claim.setDisabled(False)
        else:
            self.button_claim.setDisabled(True)
        self.label_claim_error.setText('')

    def reset(self):
        self.line_edit_claim_login.setText('')
        self.line_edit_claim_password.setText('')
        self.line_edit_claim_password_check.setText('')
        self.line_edit_claim_device.setText('')
        self.line_edit_password.setText('')
        self.button_claim.setDisabled(True)
        self.label_error.setText('')
        self.label_claim_error.setText('')
        self.group_login.show()
        self.group_claim.hide()
        self.label_error.hide()
        self.label_claim_error.hide()

    def emit_login(self):
        self.loginClicked.emit(self.combo_devices.currentText(), self.line_edit_password.text())

    def emit_claim(self):
        if (len(self.line_edit_claim_password.text()) > 0 or
             len(self.line_edit_claim_password_check.text()) > 0):
            if (self.line_edit_claim_password.text() !=
                 self.line_edit_claim_password_check.text()):
                self.set_claim_error(
                    QCoreApplication.translate(self.__class__.__name__,
                                               "Passwords don't match."))
                self.label_claim_error.show()
                return
        self.claimClicked.emit(self.line_edit_claim_login.text(),
                               self.line_edit_claim_password.text(),
                               self.line_edit_claim_device.text(),
                               self.line_edit_claim_token.text())

    def set_login_error(self, error):
        self.label_error.setText(error)
        self.label_error.show()

    def set_claim_error(self, error):
        self.label_claim_error.setText(error)
        self.label_claim_error.show()
