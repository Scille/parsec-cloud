# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import math

from enum import Enum

from PyQt5.QtCore import Qt, pyqtSignal, QSize
from PyQt5.QtGui import QIcon, QPainter, QColor, QPen, QCursor, QPixmap, QFont, QFontMetrics
from PyQt5.QtWidgets import (
    QPushButton,
    QLabel,
    QGraphicsDropShadowEffect,
    QWidget,
    QListView,
    QComboBox,
    QLineEdit,
    QToolButton,
)
from PyQt5.QtSvg import QSvgWidget

from parsec.api.data import SASCode
from parsec.core.gui.ui.code_input_widget import Ui_CodeInputWidget
from parsec.core.gui.ui.spinner_widget import Ui_SpinnerWidget


class SpinnerWidget(QWidget, Ui_SpinnerWidget):
    def __init__(self, parent=None):
        super().__init__(parent=parent)
        self.setupUi(self)
        self.spinner = QSvgWidget(":/icons/images/icons/spinner.svg")
        self.widget.layout().insertWidget(1, self.spinner)


class CenteredSpinnerWidget(SpinnerWidget):
    def __init__(self, parent=None):
        super().__init__(parent=parent)
        self.setMaximumSize(QSize(16777215, 16777215))


def ensure_string_size(s, size, font):
    metrics = QFontMetrics(font)
    if metrics.horizontalAdvance(s) > size:
        while metrics.horizontalAdvance(s + "...") > size:
            s = s[: len(s) - 1]
        s += "..."
    return s


class CodeInputWidget(QWidget, Ui_CodeInputWidget):
    good_code_clicked = pyqtSignal()
    wrong_code_clicked = pyqtSignal()
    none_clicked = pyqtSignal()

    class CodeButton(QPushButton):
        clicked_with_code = pyqtSignal(SASCode)

        def __init__(self, code):
            super().__init__(code.str)
            self.code = code
            font = self.font()
            font.setBold(True)
            font.setPointSize(20)
            font.setLetterSpacing(QFont.PercentageSpacing, 180)
            self.setStyleSheet("padding-left: 5px; padding-right: 5px; text-align: center;")
            self.setFont(font)
            self.setFixedSize(QSize(120, 120))
            self.clicked.connect(self._on_clicked)

        def _on_clicked(self):
            self.clicked_with_code.emit(self.code)

    def __init__(self, parent=None):
        super().__init__(parent)
        self.setupUi(self)
        self.right_choice = None
        self.button_none.clicked.connect(self.none_clicked.emit)

    def set_choices(self, choices, right_choice):
        while self.code_layout.count() != 0:
            item = self.code_layout.takeAt(0)
            if item:
                w = item.widget()
                if w:
                    self.code_layout.removeWidget(w)
                    w.hide()
                    w.setParent(None)

        if not choices or not right_choice:
            return
        self.right_choice = right_choice
        height = round(math.sqrt(len(choices))) or 1
        for idx, choice in enumerate(choices):
            b = self.CodeButton(choice)
            self.code_layout.addWidget(
                b, int(idx / height), idx % height, Qt.AlignHCenter | Qt.AlignVCenter
            )
            b.clicked_with_code.connect(self._on_button_clicked)

    def _on_button_clicked(self, code):
        if code != self.right_choice:
            self.wrong_code_clicked.emit()
        else:
            self.good_code_clicked.emit()


class Pixmap(QPixmap):
    def replace_color(self, old_color, new_color):
        mask = self.createMaskFromColor(old_color, Qt.MaskOutColor)
        self.fill(new_color)
        self.setMask(mask)


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


class Button(QPushButton):
    clicked_self = pyqtSignal(QWidget)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.clicked.connect(self._on_clicked)
        self.current_color = QColor(0, 0, 0)

    def _on_clicked(self):
        self.clicked_self.emit(self)

    def enterEvent(self, _):
        color = self.property("hover_color")
        if not color:
            return
        sizes = self.icon().availableSizes()
        pixmap = Pixmap(self.icon().pixmap(sizes[-1] if len(sizes) else QSize(144, 144)))
        pixmap.replace_color(self.current_color, color)
        self.setIcon(QIcon(pixmap))
        self.current_color = color

    def leaveEvent(self, _):
        self.apply_style()

    def apply_style(self):
        color = self.property("color")
        if not color:
            return
        sizes = self.icon().availableSizes()
        pixmap = Pixmap(self.icon().pixmap(sizes[-1] if len(sizes) else QSize(144, 144)))
        pixmap.replace_color(self.current_color, color)
        self.setIcon(QIcon(pixmap))
        self.current_color = color


class MenuButton(Button):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.current_color = QColor(0, 0, 0)
        self.toggled.connect(self.on_toggle)

    def apply_style(self):
        color = self.property("unchecked_color")
        if not color:
            return
        sizes = self.icon().availableSizes()
        pixmap = Pixmap(self.icon().pixmap(sizes[-1] if len(sizes) else QSize(144, 144)))
        pixmap.replace_color(self.current_color, color)
        self.current_color = color
        self.setIcon(QIcon(pixmap))

    def enterEvent(self, _):
        pass

    def leaveEvent(self, _):
        pass

    def on_toggle(self, checked):
        prop = "checked_color" if checked else "unchecked_color"
        new_color = self.property(prop)
        if not new_color:
            return
        sizes = self.icon().availableSizes()
        pixmap = Pixmap(self.icon().pixmap(sizes[-1] if len(sizes) else QSize(144, 144)))
        pixmap.replace_color(self.current_color, new_color)
        self.current_color = new_color
        self.setIcon(QIcon(pixmap))


class ShadowedButton(Button):
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


class OverlayLabel(ClickableLabel):
    OPEN_FULLSCREEN_ICON = None
    CLOSE_FULLSCREEN_ICON = None

    class ClickMode(Enum):
        OpenFullScreen = 1
        CloseFullScreen = 2

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self._pix = None
        self.click_mode = OverlayLabel.ClickMode.OpenFullScreen
        self.show_icon = True
        if not OverlayLabel.OPEN_FULLSCREEN_ICON:
            OverlayLabel.OPEN_FULLSCREEN_ICON = Pixmap(":/icons/images/material/fullscreen.svg")
            OverlayLabel.OPEN_FULLSCREEN_ICON.replace_color(QColor(0, 0, 0), QColor(164, 164, 164))
        if not OverlayLabel.CLOSE_FULLSCREEN_ICON:
            OverlayLabel.CLOSE_FULLSCREEN_ICON = Pixmap(
                ":/icons/images/material/fullscreen_exit.svg"
            )
            OverlayLabel.CLOSE_FULLSCREEN_ICON.replace_color(QColor(0, 0, 0), QColor(164, 164, 164))

    def set_mode(self, click_mode, show_icon):
        self.click_mode = click_mode
        self.show_icon = show_icon

    def enterEvent(self, event):
        self._draw_overlay()

    def leaveEvent(self, event):
        self._draw_overlay()

    def setPixmap(self, pix):
        self._pix = pix.copy()
        self._draw_overlay()

    def _draw_overlay(self):
        if not self._pix:
            return

        if not self.underMouse():
            super().setPixmap(self._pix)
            return

        icon = None

        if self.click_mode == OverlayLabel.ClickMode.OpenFullScreen and self.show_icon:
            icon = OverlayLabel.OPEN_FULLSCREEN_ICON
            icon = icon.scaled(self._pix.width() - 10, self._pix.height() - 10, Qt.KeepAspectRatio)
        elif self.click_mode == OverlayLabel.ClickMode.CloseFullScreen and self.show_icon:
            icon = OverlayLabel.CLOSE_FULLSCREEN_ICON
            icon = icon.scaled(
                int(self._pix.width() / 5), int(self._pix.height() / 5), Qt.KeepAspectRatio
            )

        p = self._pix.copy()
        painter = QPainter(p)
        if icon:
            pos_x = int(p.width() / 2) - int(icon.width() / 2)
            pos_y = int(p.height() / 2) - int(icon.height() / 2)
            painter.drawPixmap(pos_x, pos_y, icon)
        painter.end()
        super().setPixmap(p)


class FilterLineEdit(QLineEdit):
    clear_clicked = pyqtSignal()

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.textChanged.connect(self._on_text_changed)
        self.button_clear = QToolButton()
        self.button_clear.setVisible(False)
        self.button_clear.clicked.connect(self._on_button_clear_clicked)
        self.button_clear.setParent(self)
        self.button_clear.setCursor(Qt.PointingHandCursor)
        icon = Pixmap(":/icons/images/material/clear.svg")
        icon.replace_color(QColor(0, 0, 0), QColor(164, 164, 164))
        self.button_clear.setIcon(QIcon(icon))
        self.button_clear.setStyleSheet("border: 0;")

    def resizeEvent(self, event):
        ret = super().resizeEvent(event)
        self._move_button()
        return ret

    def _on_button_clear_clicked(self):
        self.setText("")
        self.clear_clicked.emit()

    def _on_text_changed(self, text):
        self.button_clear.setVisible(len(text))

    def _move_button(self):
        r = self.geometry()
        self.button_clear.setGeometry(
            r.x() + r.width() - self.button_clear.width(), r.y(), r.height(), r.height()
        )


class IconLabel(QLabel):
    def apply_style(self):
        color = self.property("color")
        if not color:
            return
        pixmap = Pixmap(self.pixmap())
        pixmap.replace_color(QColor(0, 0, 0), color)
        self.setPixmap(pixmap)


class DropDownButton(Button):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.toggled.connect(self.on_toggle)

    def on_toggle(self, checked):
        color = self.property("color")
        if not color:
            return
        if not checked:
            pixmap = Pixmap(":/icons/images/material/arrow_drop_down.svg")
            pixmap.replace_color(QColor(0, 0, 0), color)
            self.setIcon(QIcon(pixmap))
        else:
            pixmap = Pixmap(":/icons/images/material/arrow_drop_up.svg")
            pixmap.replace_color(QColor(0, 0, 0), color)
            self.setIcon(QIcon(pixmap))


class ComboBox(QComboBox):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setView(QListView())


# class NotificationTaskbarButton(TaskbarButton):
#     def __init__(self, *args, **kwargs):
#         super().__init__(
#             icon_path=":/icons/images/icons/tray_icons/bell-solid-$STATE.svg", *args, **kwargs
#         )
#         self.notif_count = 0

#     def inc_notif_count(self):
#         self.notif_count += 1

#     def reset_notif_count(self):
#         self.notif_count = 0

#     def setChecked(self, val):
#         super().setChecked(val)
#         if val:
#             self.setIconPath(":/icons/images/icons/tray_icons/bell-regular-$STATE.svg")
#             self.setIconSize(QSize(50, 50))
#         else:
#             self.setIconPath(":/icons/images/icons/tray_icons/bell-solid-$STATE.svg")
#             self.setIconSize(QSize(50, 50))

#     def paintEvent(self, event):
#         super().paintEvent(event)
#         if self.notif_count == 0:
#             return
#         text = str(self.notif_count)
#         if self.notif_count >= 100:
#             text = "99+"
#         rect = event.rect()
#         painter = QPainter(self)
#         painter.setPen(QColor(220, 54, 66))
#         painter.setBrush(QColor(220, 54, 66))
#         painter.drawEllipse(rect.right() - 35, 0, 35, 35)
#         painter.setPen(QColor(255, 255, 255))
#         if len(text) == 1:
#             painter.drawText(QPoint(rect.right() - 22, 23), text)
#         elif len(text) == 2:
#             painter.drawText(QPoint(rect.right() - 27, 23), text)
#         elif len(text) == 3:
#             painter.drawText(QPoint(rect.right() - 30, 23), text)
#         painter.end()
