# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import Qt, pyqtSignal, QSize, QPoint
from PyQt5.QtGui import QIcon, QPainter, QColor, QPen, QCursor
from PyQt5.QtWidgets import QLineEdit, QPushButton, QLabel, QGraphicsDropShadowEffect


class FileLineEdit(QLineEdit):
    clicked = pyqtSignal(str)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.full_text = ""

    def setText(self, text):
        self.full_text = text
        if len(text) > 30:
            text = text[:30] + "..."
        super().setText(text)

    def mousePressEvent(self, event):
        if event.button() & Qt.LeftButton:
            self.clicked.emit(self.full_text)


class TaskbarButton(QPushButton):
    def __init__(self, icon_path, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setMinimumSize(64, 64)
        self.setMaximumSize(64, 64)
        self.setFixedSize(64, 64)
        self.setIcon(QIcon(icon_path))
        self.setIconSize(QSize(50, 50))
        self.setStyleSheet(
            "QPushButton{background-color: rgba(0, 0, 0, 0); border: 0;}\n"
            "QToolTip{background-color: rgb(46, 146, 208);"
            "border: 1px solid rgb(12, 65, 157);color: rgb(255, 255, 255);}"
        )


class NotificationTaskbarButton(TaskbarButton):
    def __init__(self, *args, **kwargs):
        super().__init__(
            icon_path=":/icons/images/icons/tray_icons/bell-solid.svg", *args, **kwargs
        )
        self.notif_count = 0

    def inc_notif_count(self):
        self.notif_count += 1

    def reset_notif_count(self):
        self.notif_count = 0

    def setChecked(self, val):
        super().setChecked(val)
        if val:
            self.setIcon(QIcon(":/icons/images/icons/tray_icons/bell-regular.svg"))
            self.setIconSize(QSize(50, 50))
        else:
            self.setIcon(QIcon(":/icons/images/icons/tray_icons/bell-solid.svg"))
            self.setIconSize(QSize(50, 50))

    def paintEvent(self, event):
        super().paintEvent(event)
        if self.notif_count == 0:
            return
        text = str(self.notif_count)
        if self.notif_count >= 100:
            text = "99+"
        rect = event.rect()
        painter = QPainter(self)
        painter.setPen(QColor(220, 54, 66))
        painter.setBrush(QColor(220, 54, 66))
        painter.drawEllipse(rect.right() - 35, 0, 35, 35)
        painter.setPen(QColor(255, 255, 255))
        if len(text) == 1:
            painter.drawText(QPoint(rect.right() - 22, 23), text)
        elif len(text) == 2:
            painter.drawText(QPoint(rect.right() - 27, 23), text)
        elif len(text) == 3:
            painter.drawText(QPoint(rect.right() - 30, 23), text)
        painter.end()


class PageLabel(QLabel):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.page = 0
        self.max_page = 0

    def paintEvent(self, event):
        super().paintEvent(event)
        painter = QPainter(self)
        grey = QColor(215, 215, 215)
        blue = QColor(38, 142, 212)

        x = 0
        for p in range(self.max_page):
            if p == self.page:
                painter.setPen(blue)
                painter.setBrush(blue)
            else:
                painter.setPen(grey)
                painter.setBrush(grey)
            x = p * 16 + 22 * p
            painter.drawEllipse(x, 20, 16, 16)
        painter.end()


class DeviceLabel(QLabel):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.is_revoked = False

    def paintEvent(self, event):
        super().paintEvent(event)
        if not self.is_revoked:
            return
        rect = event.rect()
        painter = QPainter(self)
        painter.setRenderHints(QPainter.HighQualityAntialiasing)
        pen = QPen(QColor(218, 53, 69))
        pen.setWidth(5)
        pen.setJoinStyle(Qt.RoundJoin)
        pen.setCapStyle(Qt.RoundCap)
        painter.setPen(pen)
        painter.drawEllipse(rect.right() - 53, 3, 50, 53)
        painter.drawLine(rect.right() - 44, 44, rect.right() - 12, 12)
        painter.end()


class PointingHandButton(QPushButton):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setCursor(QCursor(Qt.PointingHandCursor))


class ShadowedButton(PointingHandButton):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        effect = QGraphicsDropShadowEffect(self)
        effect.setColor(QColor(164, 164, 164))
        effect.setBlurRadius(4)
        effect.setXOffset(2)
        effect.setYOffset(2)
        self.setGraphicsEffect(effect)


class ClickableLabel(QLabel):
    clicked = pyqtSignal(str)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setCursor(QCursor(Qt.PointingHandCursor))

    def mousePressEvent(self, event):
        if event.button() & Qt.LeftButton:
            self.clicked.emit(self.text())
