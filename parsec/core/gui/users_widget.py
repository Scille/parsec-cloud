from PyQt5.QtCore import pyqtSignal, QCoreApplication
from PyQt5.QtWidgets import QWidget, QInputDialog

from parsec.core.gui.ui.users_widget import Ui_UsersWidget
from parsec.core.gui.register_device import RegisterDevice


class UsersWidget(QWidget, Ui_UsersWidget):
    register_user_clicked = pyqtSignal(str)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.setupUi(self)
        self.reset()
        self.button_register_user.clicked.connect(self.emit_register_user)
        self.button_register_device.clicked.connect(self.show_register_device)

    def emit_register_user(self):
        user_name, ok = QInputDialog.getText(
            self,
            QCoreApplication.translate("UsersWidget", "New user"),
            QCoreApplication.translate("UsersWidget", "Enter new user name"),
        )
        if not ok or not user_name:
            return
        self.register_user_clicked.emit(user_name)

    def show_register_device(self):
        self.register_device_dialog = RegisterDevice(parent=self)
        self.register_device_dialog.show()

    def set_claim_infos(self, login, token):
        self.widget_info.show()
        self.line_edit_user_id.setText(login)
        self.line_edit_token.setText(token)

    def reset(self):
        self.line_edit_user_id.setText("")
        self.line_edit_token.setText("")
        self.widget_info.hide()
