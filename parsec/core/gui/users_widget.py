from PyQt5.QtWidgets import QWidget

from parsec.core.gui.ui.users_widget import Ui_UsersWidget


class UsersWidget(QWidget, Ui_UsersWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.setupUi(self)
