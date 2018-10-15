from PyQt5.QtCore import pyqtSignal, QCoreApplication
from PyQt5.QtWidgets import QWidget, QInputDialog

from parsec.core.gui.ui.users_widget import Ui_UsersWidget


class UsersWidget(QWidget, Ui_UsersWidget):
    register_user_clicked = pyqtSignal(str)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.setupUi(self)
        self.reset()
        self.button_add_user.clicked.connect(self.emit_register_user)

    def emit_register_user(self):
        user_name, ok = QInputDialog.getText(
            self,
            QCoreApplication.translate("UsersWidget", "New user"),
            QCoreApplication.translate("UsersWidget", "Enter new user name"),
        )
        if not ok or not user_name:
            return
        self.register_user_clicked.emit(user_name)

    def set_claim_infos(self, login, token):
        self.widget_info.show()
        self.line_edit_user_id.setText(login)
        self.line_edit_token.setText(token)

    def reset(self):
        self.line_edit_user_id.setText("")
        self.line_edit_token.setText("")
        self.widget_info.hide()
