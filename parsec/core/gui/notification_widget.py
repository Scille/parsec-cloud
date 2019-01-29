from PyQt5.QtWidgets import QWidget, QStyle, QStyleOption
from PyQt5.QtGui import QPixmap, QPainter

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

    def paintEvent(self, _):
        opt = QStyleOption()
        opt.initFrom(self)
        p = QPainter(self)
        self.style().drawPrimitive(QStyle.PE_Widget, opt, p, self)


class ErrorNotificationWidget(NotificationWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.label_message.setStyleSheet("color: rgb(226, 25, 11);")
        self.set_icon(":/icons/images/icons/error.png")


class WarningNotificationWidget(NotificationWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.label_message.setStyleSheet("color: rgb(244, 203, 60);")
        self.set_icon(":/icons/images/icons/warning.png")


class InfoNotificationWidget(NotificationWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.label_message.setStyleSheet("color: rgb(66, 152, 244);")
        self.set_icon(":/icons/images/icons/info.png")
