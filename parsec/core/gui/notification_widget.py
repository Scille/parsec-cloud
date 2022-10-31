# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations
from typing import Any

from parsec._parsec import DateTime

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget, QStyle, QStyleOption
from PyQt5.QtGui import QPixmap, QPainter, QPaintEvent

from parsec.core.gui.ui.notification_widget import Ui_NotificationWidget
from parsec.core.gui.lang import format_datetime


class NotificationWidget(QWidget, Ui_NotificationWidget):
    close_clicked = pyqtSignal(QWidget)

    def __init__(self, *args: Any, **kwargs: Any) -> None:
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        now = DateTime.now()
        self.label_date.setText(format_datetime(now))
        self.button_close.clicked.connect(self.emit_close_clicked)

    @property
    def message(self) -> str:
        return self.label_message.text()

    @message.setter
    def message(self, val: str) -> None:
        self.label_message.setText(val)

    def emit_close_clicked(self) -> None:
        self.close_clicked.emit(self)

    def set_icon(self, icon_path: str) -> None:
        self.label_icon.setPixmap(QPixmap(icon_path))

    def paintEvent(self, _: QPaintEvent) -> None:
        opt = QStyleOption()
        opt.initFrom(self)
        p = QPainter(self)
        self.style().drawPrimitive(QStyle.PE_Widget, opt, p, self)


class ErrorNotificationWidget(NotificationWidget):
    def __init__(self, *args: Any, **kwargs: Any) -> None:
        super().__init__(*args, **kwargs)
        self.label_message.setStyleSheet("color: rgb(218, 53, 69);")
        self.label_date.setStyleSheet("color: rgb(218, 53, 69);")
        self.set_icon(":/icons/images/icons/error.png")


class WarningNotificationWidget(NotificationWidget):
    def __init__(self, *args: Any, **kwargs: Any) -> None:
        super().__init__(*args, **kwargs)
        self.label_message.setStyleSheet("color: rgb(254, 195, 7);")
        self.label_date.setStyleSheet("color: rgb(254, 195, 7);")
        self.set_icon(":/icons/images/icons/warning.png")


class InfoNotificationWidget(NotificationWidget):
    def __init__(self, *args: Any, **kwargs: Any) -> None:
        super().__init__(*args, **kwargs)
        self.label_message.setStyleSheet("color: rgb(20, 160, 183);")
        self.label_date.setStyleSheet("color: rgb(20, 160, 183);")
        self.set_icon(":/icons/images/icons/info.png")
