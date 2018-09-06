from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget

from parsec.core.gui.ui.users_widget import Ui_UsersWidget


class UsersWidget(QWidget, Ui_UsersWidget):
    registerClicked = pyqtSignal(str)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.setupUi(self)
        self.label_help.hide()
        self.line_edit_new_login.textChanged.connect(self.check_register_infos)
        self.button_register.clicked.connect(self.emit_register)

    def check_register_infos(self):
        if len(self.line_edit_new_login.text()):
            self.button_register.setEnabled(True)
        else:
            self.button_register.setEnabled(False)

    def emit_register(self):
        self.registerClicked.emit(self.line_edit_new_login.text())

    def set_claim_infos(self, login, token):
        self.label_help.show()
        self.label_new_user_login.setText(login)
        self.label_new_user_token.setText(token)
