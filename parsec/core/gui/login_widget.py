from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget

from parsec.core.gui.ui.login_widget import Ui_LoginWidget


class LoginWidget(QWidget, Ui_LoginWidget):
    loginClicked = pyqtSignal(str, str)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.setupUi(self)
        self.button_log_in.clicked.connect(self.emit_login)
        self.line_input_login.textChanged.connect(self.check_infos)
        self.line_input_password.textChanged.connect(self.check_infos)

    def check_infos(self, _):
        if len(self.line_input_login.text()) and len(self.line_input_password.text()):
            self.button_log_in.setDisabled(False)
        else:
            self.button_log_in.setDisabled(True)

    def emit_login(self):
        self.loginClicked.emit(self.line_input_login.text(), self.line_input_password.text())
