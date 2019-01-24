from PyQt5.QtGui import QPainter
from PyQt5.QtWidgets import QWidget, QStyle, QStyleOption

from parsec.core.gui.ui.notification_center_widget import Ui_NotificationCenterWidget


class NotificationCenterWidget(QWidget, Ui_NotificationCenterWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)

    def paintEvent(self, _):
        opt = QStyleOption()
        opt.initFrom(self)
        p = QPainter(self)
        self.style().drawPrimitive(QStyle.PE_Widget, opt, p, self)
