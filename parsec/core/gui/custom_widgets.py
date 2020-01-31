# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import Qt, pyqtSignal, QSize, QPoint, QRect
from PyQt5.QtGui import QIcon, QPainter, QColor, QPen, QCursor
from PyQt5.QtWidgets import (
    QPushButton,
    QLabel,
    QGraphicsDropShadowEffect,
    QLayout,
    QSizePolicy,
    QStyle,
)


class FlowLayout(QLayout):
    def __init__(self, spacing=10, parent=None):
        super().__init__(parent)
        self.spacing = spacing
        self.items = []

    def addItem(self, item):
        self.items.append(item)

    def horizontalSpacing(self):
        if self.spacing >= 0:
            return self.spacing
        return self._smart_spacing(QStyle.PM_LayoutHorizontalSpacing)

    def verticalSpacing(self):
        if self.spacing >= 0:
            return self.spacing
        return self._smart_spacing(QStyle.PM_LayoutVerticalSpacing)

    def count(self):
        return len(self.items)

    def itemAt(self, index):
        if index >= 0 and index < len(self.items):
            return self.items[index]
        return None

    def takeAt(self, index):
        if index >= 0 and index < len(self.items):
            return self.items.pop(index)
        return None

    def clear(self):
        while self.count() != 0:
            item = self.takeAt(0)
            if item:
                w = item.widget()
                self.removeWidget(w)
                w.hide()
                w.setParent(None)

    def expandingDirections(self):
        return Qt.Horizontal

    def hasHeightForWidth(self):
        return True

    def heightForWidth(self, width):
        height = self._do_layout(QRect(0, 0, width, 0), True)
        return height

    def setGeometry(self, rect):
        super().setGeometry(rect)
        self._do_layout(rect, False)

    def sizeHint(self):
        return self.minimumSize()

    def minimumSize(self):
        size = QSize()
        for item in self.items:
            size = size.expandedTo(item.minimumSize())
        return size

    def _do_layout(self, rect, test_only):
        x = rect.x()
        y = rect.y()
        line_height = 0

        for item in self.items:
            w = item.widget()
            space_x = self.horizontalSpacing()
            if space_x == -1:
                space_x = w.style().layoutSpacing(
                    QSizePolicy.PushButton, QSizePolicy.PushButton, Qt.Horizontal
                )
            space_y = self.verticalSpacing()
            if space_y == -1:
                space_y = w.style().layoutSpacing(
                    QSizePolicy.QPushButton, QSizePolicy.QPushButton, Qt.Vertical
                )
            next_x = x + item.sizeHint().width() + space_x
            if next_x - space_x > rect.right() and line_height > 0:
                x = rect.x()
                y = y + line_height + space_y
                next_x = x + item.sizeHint().width() + space_x
                line_height = 0
            if not test_only:
                item.setGeometry(QRect(QPoint(x, y), item.sizeHint()))
            x = next_x
            line_height = max(line_height, item.sizeHint().height())
        return y + line_height - rect.x()

    def _smart_spacing(self, pm):
        parent = self.parent()
        if not parent:
            return -1
        elif parent.isWidgetType():
            parent.style().pixelMetric(pm, 0, parent)
        else:
            parent.spacing()


class FileLabel(QLabel):
    clicked = pyqtSignal(str)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setCursor(QCursor(Qt.PointingHandCursor))
        self.full_text = ""

    def setText(self, text):
        self.full_text = text
        if len(text) > 30:
            text = text[:30] + "..."
        super().setText(text)


class MenuButton(QPushButton):
    def __init__(self):
        super().__init__(QIcon(":/icons/images/icons/settings_bars.png"), "")
        self.setMinimumSize(32, 32)
        self.setMaximumSize(32, 32)
        self.setFixedSize(32, 32)
        self.setStyleSheet(
            "QPushButton{"
            "background-color: rgb(255, 255, 255);"
            "border: none;"
            "padding: 10px;"
            "}"
        )


class TaskbarButton(QPushButton):
    def __init__(self, icon_path, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.state_string = "default"
        self.setMinimumSize(64, 64)
        self.setMaximumSize(64, 64)
        self.setFixedSize(64, 64)
        self.setIconPath(icon_path)
        self.setIconSize(QSize(50, 50))
        self.setStyleSheet(
            "QPushButton{background-color: rgba(0, 0, 0, 0); border: 0;}\n"
            "QToolTip{background-color: rgb(46, 146, 208);"
            "border: 1px solid rgb(12, 65, 157);color: rgb(255, 255, 255);}"
        )

    def enterEvent(self, event):
        self.state_string = "hover"
        self.reloadIcon()

    def leaveEvent(self, event):
        self.state_string = "default"
        self.reloadIcon()

    def setIconPath(self, path):
        self.icon_path = path
        self.reloadIcon()

    def reloadIcon(self):
        self.setIcon(QIcon(self.icon_path.replace("$STATE", self.state_string)))


class NotificationTaskbarButton(TaskbarButton):
    def __init__(self, *args, **kwargs):
        super().__init__(
            icon_path=":/icons/images/icons/tray_icons/bell-solid-$STATE.svg", *args, **kwargs
        )
        self.notif_count = 0

    def inc_notif_count(self):
        self.notif_count += 1

    def reset_notif_count(self):
        self.notif_count = 0

    def setChecked(self, val):
        super().setChecked(val)
        if val:
            self.setIconPath(":/icons/images/icons/tray_icons/bell-regular-$STATE.svg")
            self.setIconSize(QSize(50, 50))
        else:
            self.setIconPath(":/icons/images/icons/tray_icons/bell-solid-$STATE.svg")
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
