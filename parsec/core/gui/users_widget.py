from PyQt5.QtWidgets import QWidget
from PyQt5.QtGui import QPixmap

from parsec.core.backend_connection import BackendNotAvailable

from parsec.core.gui.register_user_dialog import RegisterUserDialog
from parsec.core.gui.custom_widgets import TaskbarButton
from parsec.core.gui.core_widget import CoreWidget
from parsec.core.gui.ui.user_button import Ui_UserButton
from parsec.core.gui.ui.users_widget import Ui_UsersWidget


class UserButton(QWidget, Ui_UserButton):
    def __init__(self, user_name, is_current_user, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        if is_current_user:
            self.label.setPixmap(QPixmap(":/icons/images/icons/user_owner.png"))
        else:
            self.label.setPixmap(QPixmap(":/icons/images/icons/user.png"))
        self.label_user.setText(user_name)

    @property
    def name(self):
        return self.label_user.text()

    @name.setter
    def name(self, value):
        self.label_user.setText(value)


class UsersWidget(CoreWidget, Ui_UsersWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.setupUi(self)
        self.users = []
        self.taskbar_buttons = []
        button_add_user = TaskbarButton(icon_path=":/icons/images/icons/add-plus-button.png")
        button_add_user.clicked.connect(self.register_user)
        self.taskbar_buttons.append(button_add_user)
        self.line_edit_search.textChanged.connect(self.filter_users)

    def get_taskbar_buttons(self):
        return self.taskbar_buttons

    def filter_users(self, pattern):
        pattern = pattern.lower()
        for i in range(self.layout_users.count()):
            item = self.layout_users.itemAt(i)
            if item:
                w = item.widget()
                if pattern and pattern not in w.name.lower():
                    w.hide()
                else:
                    w.show()

    def register_user(self):
        d = RegisterUserDialog(parent=self, portal=self.portal, core=self.core)
        d.exec_()
        self.reset()

    def add_user(self, user_name, is_current_user):
        if user_name in self.users:
            return
        button = UserButton(user_name, is_current_user)
        self.layout_users.addWidget(button, int(len(self.users) / 4), int(len(self.users) % 4))
        self.users.append(user_name)

    def reset(self):
        self.line_edit_search.setText("")
        self.users = []
        while self.layout_users.count() != 0:
            item = self.layout_users.takeAt(0)
            if item:
                w = item.widget()
                self.layout_users.removeWidget(w)
                w.setParent(None)
        if self.portal and self.core:
            try:
                user_id = self.core.device.user_id
                users = self.portal.run(self.core.fs.backend_cmds.user_find)
                for user in users:
                    self.add_user(str(user), is_current_user=user_id == user)
            except BackendNotAvailable:
                pass
