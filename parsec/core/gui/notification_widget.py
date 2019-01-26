from PyQt5.QtWidgets import QWidget
from PyQt5.QtGui import QPixmap

from parsec.core.gui.ui.notification_widget import Ui_NotificationWidget


class NotificationWidget(QWidget, Ui_NotificationWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)

    @property
    def message(self):
        return self.label_message.text()

    @message.setter
    def message(self, val):
        self.label_message.setText(val)

    def set_icon(self, icon_path):
        self.label_icon.setPixmap(QPixmap(icon_path))


class ErrorNotificationWidget(NotificationWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.label_message.setStyleSheet("color: (226, 25, 11);")
        self.set_icon(":/icons/images/icons/error.png")


class WarningNotificationWidget(NotificationWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.label_message.setStyleSheet("color: (244, 203, 60);")
        self.set_icon(":/icons/images/icons/warning.png")


class InfoNotificationWidget(NotificationWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.label_message.setStyleSheet("color: (66, 152, 244);")
        self.set_icon(":/icons/images/icons/info.png")
