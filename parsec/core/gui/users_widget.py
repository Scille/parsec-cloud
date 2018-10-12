from PyQt5.QtCore import pyqtSignal, QCoreApplication
from PyQt5.QtWidgets import QWidget, QInputDialog

from parsec.core.gui.ui.users_widget import Ui_UsersWidget


class UsersWidget(QWidget, Ui_UsersWidget):
    registerUserClicked = pyqtSignal(str)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.setupUi(self)
        self.label_help.hide()
        self.label_register_error.hide()
        self.label_new_user_login.hide()
        self.label_new_user_token.hide()
        self.button_add_user.clicked.connect(self.emit_register_user)

    def emit_register_user(self):
        user_name, ok = QInputDialog.getText(
            self,
            QCoreApplication.translate("UsersWidget", "New user"),
            QCoreApplication.translate("UsersWidget", "Enter new user name"),
        )
        if not ok or not user_name:
            return
        self.registerUserClicked.emit(user_name)

    def set_claim_infos(self, login, token):
        self.label_help.show()
        self.label_new_user_login.setText(login)
        self.label_new_user_token.setText(token)
        self.label_register_error.hide()
        self.label_new_user_login.show()
        self.label_new_user_token.show()

    def set_error(self, error):
        self.label_new_user_login.hide()
        self.label_new_user_token.hide()
        self.label_register_error.show()
        self.label_register_error.setText(error)

    def reset(self):
        self.label_new_user_login.setText("")
        self.label_new_user_token.setText("")
        self.label_new_user_login.hide()
        self.label_new_user_token.hide()
        self.label_help.hide()
        self.label_register_error.hide()
        self.label_register_error.setText("")
