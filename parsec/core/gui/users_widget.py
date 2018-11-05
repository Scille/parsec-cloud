from PyQt5.QtCore import pyqtSignal, QCoreApplication
from PyQt5.QtWidgets import QWidget, QInputDialog

from parsec.core.gui.core_call import core_call
from parsec.core.gui.custom_widgets import get_text
from parsec.core.gui.ui.user_button import Ui_UserButton
from parsec.core.gui.ui.users_widget import Ui_UsersWidget


class UserButton(QWidget, Ui_UserButton):
    def __init__(self, user_name, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.label_user.setText(user_name)

    @property
    def name(self):
        return self.label_user.text()

    @name.setter
    def name(self, value):
        self.label_user.setText(value)


class UsersWidget(QWidget, Ui_UsersWidget):
    register_user_clicked = pyqtSignal(str)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.setupUi(self)
        self.reset()
        self.button_add_user.clicked.connect(self.emit_register_user)
        self.line_edit_search.textChanged.connect(self.filter_users)

    def filter_users(self, pattern):
        for i in range(self.layout_users.count()):
            item = self.layout_users.itemAt(i)
            if item:
                w = item.widget()
                if pattern and pattern not in w.name:
                    w.hide()
                else:
                    w.show()

    def emit_register_user(self):
        user_name = get_text(
            self,
            QCoreApplication.translate("UsersWidget", "New user"),
            QCoreApplication.translate("UsersWidget", "Enter new user name"),
            QCoreApplication.translate("UsersWidget", "User name"),
        )
        if not user_name:
            return
        self.register_user_clicked.emit(user_name)

    def set_claim_infos(self, login, token):
        self.widget_info.show()
        self.line_edit_user_id.setText(login)
        self.line_edit_token.setText(token)

    def add_user(self, user_name):
        if user_name in self.users:
            return
        button = UserButton(user_name)
        self.layout_users.addWidget(button, int(self.users_count / 4), int(self.users_count % 4))
        self.users_count += 1
        self.users.append(user_name)

    def reset(self):
        self.line_edit_user_id.setText("")
        self.line_edit_token.setText("")
        self.widget_info.hide()
        self.users_count = 0
        self.users = []
        while self.layout_users.count() != 0:
            item = self.layout_users.takeAt(0)
            if item:
                w = item.widget()
                self.layout_users.removeWidget(w)
                w.setParent(None)
        for user in core_call().get_devices():
            self.add_user(user.split("@")[0])
