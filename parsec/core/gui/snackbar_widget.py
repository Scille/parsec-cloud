# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import platform

from PyQt5.QtCore import (
    Qt,
    QTimer,
    QPropertyAnimation,
    QEvent,
    QRect,
    pyqtSignal,
    pyqtProperty,
    QEasingCurve,
    QPoint,
    QObject,
    QSize,
)
from PyQt5.QtGui import QPainter, QBrush, QColor, QCursor
from PyQt5.QtWidgets import QWidget

from parsec.core.gui.custom_widgets import Pixmap
from parsec.core.gui.parsec_application import ParsecApp
from parsec.core.gui.ui.snackbar_widget import Ui_SnackbarWidget


class SnackbarWidget(QWidget, Ui_SnackbarWidget):
    _dismissed = pyqtSignal(QWidget)

    def _set_opacity(self, o):
        self._opacity = o
        self.setWindowOpacity(o)

    def _get_opacity(self):
        return self._opacity

    opacity = pyqtProperty(float, fset=_set_opacity, fget=_get_opacity)

    def __init__(self, msg, icon=None, timeout=3000, action_text=None, action=None, animate=True):
        super().__init__()
        self.setupUi(self)
        self.animate = animate
        self.setWindowFlags(Qt.FramelessWindowHint | Qt.Tool | Qt.WindowStaysOnTopHint)
        self.setAttribute(Qt.WA_TranslucentBackground)
        self.setAttribute(Qt.WA_ShowWithoutActivating)
        self.setObjectName("SnackbarWidget")
        self.label.setText(msg)
        self.action = action
        if action_text and action:
            self.button_action.setText(action_text)
            self.button_action.clicked.connect(self._on_action_clicked)
            self.button_action.setCursor(QCursor(Qt.PointingHandCursor))
        if icon:
            self.label_icon.setPixmap(icon)
        self.button_close.clicked.connect(self._on_timeout)

        self.timer = QTimer()
        self.timer.setInterval(timeout)
        self.timer.setSingleShot(True)
        self.timer.timeout.connect(self._on_timeout)
        self.index = 0

        if self.animate:
            self._set_opacity(0.0)
            self.show_animation = QPropertyAnimation(self, b"opacity")
            self.show_animation.finished.connect(self.timer.start)
            self.show_animation.setDuration(500)
            self.show_animation.setStartValue(0.0)
            self.show_animation.setEndValue(1.0)
            self.show_animation.setEasingCurve(QEasingCurve.Linear)

            self.hide_animation = QPropertyAnimation(self, b"opacity")
            self.hide_animation.finished.connect(self.hide)
            self.hide_animation.setDuration(500)
            self.hide_animation.setStartValue(1.0)
            self.hide_animation.setEndValue(0.0)
            self.show_animation.setEasingCurve(QEasingCurve.Linear)
        else:
            self._set_opacity(1.0)

    def _on_action_clicked(self):
        self.action()
        self.timer.stop()
        if self.animate:
            self.show_animation.stop()
            self.hide_animation.start()

    def set_index(self, index):
        self.index = index
        self.move_popup()

    def paintEvent(self, _):
        painter = QPainter(self)
        painter.setRenderHint(QPainter.Antialiasing)
        rect = QRect()
        rect.setX(self.rect().x() + 5)
        rect.setY(self.rect().y() + 5)
        rect.setWidth(self.rect().width() - 10)
        rect.setHeight(self.rect().height() - 10)
        painter.setBrush(QBrush(QColor(0x32, 0x32, 0x32, 0xDD)))
        painter.setPen(Qt.NoPen)
        painter.drawRoundedRect(rect, 10, 10)

    def move_popup(self):
        main_window = ParsecApp.get_main_window()
        if not main_window:
            return
        offset = 10
        height = 101 if platform.system() == "Windows" else 75
        width = min(500, main_window.size().width() - 40)
        self.resize(QSize(width, height))

        x = main_window.size().width() - width - 20
        y = main_window.size().height() - ((height + offset) * (self.index + 1))
        # Hide the snackbar if the main window does not have enough space to show it
        self.set_visible(y > 30)
        pos = main_window.mapToGlobal(QPoint(x, y))
        self.setGeometry(pos.x(), pos.y(), width, height)

    def _on_timeout(self):
        if self.animate:
            self.show_animation.stop()
            self.hide_animation.start()
        else:
            self.hide()

    def set_visible(self, visible):
        if not visible:
            super().hide()
        else:
            super().show()

    def hide(self):
        if self.animate:
            self.show_animation.stop()
            self.hide_animation.stop()
        self.timer.stop()
        super().hide()
        self._dismissed.emit(self)

    def show(self):
        self.move_popup()
        super().show()
        if self.animate:
            self._set_opacity(0.0)
            self.show_animation.start()
        else:
            self._set_opacity(1.0)
            self.timer.start()


class SnackbarManager(QObject):
    _manager = None

    def __init__(self):
        super().__init__()
        self.snackbars = []
        ParsecApp.get_main_window().installEventFilter(self)
        self.setParent(ParsecApp.get_main_window())
        self.destroyed.connect(self._on_destroyed)

    def eventFilter(self, obj, event):
        if event.type() == QEvent.Move or event.type() == QEvent.Resize:
            for sb in self.snackbars:
                sb.move_popup()
        elif event.type() == QEvent.Hide:
            for sb in self.snackbars:
                sb.set_visible(False)
        elif event.type() == QEvent.Show:
            for sb in self.snackbars:
                sb.set_visible(True)
        elif event.type() == QEvent.Close:
            # Making sure we don't interact with snackbars after the main window is closed
            self.snackbars = []
        return False

    def add_snackbar(self, snackbar):
        snackbar._dismissed.connect(self._on_dismissed)
        self.snackbars.insert(0, snackbar)
        for i, sb in enumerate(self.snackbars):
            sb.set_index(i)
        snackbar.show()

    def _on_destroyed(self, _):
        self.snackbars = []

    def _remove_snackbar(self, snackbar):
        try:
            self.snackbars.remove(snackbar)
        except ValueError:
            pass

    def _on_dismissed(self, snackbar):
        self._remove_snackbar(snackbar)
        snackbar.setParent(None)
        for i, sb in enumerate(self.snackbars):
            sb.set_index(i)

    @classmethod
    def get_manager(cls):
        # Create the manager the first time it's used.
        # Else it will try to plug itself onto the main window
        # that does not exist yet.
        if not cls._manager:
            cls._manager = cls()
        return cls._manager

    @classmethod
    def inform(cls, msg, timeout=3000, action_text=None, action=None, animate=True):
        pix = Pixmap(":/icons/images/material/info.svg")
        pix.replace_color(QColor(0, 0, 0), QColor(73, 153, 208))
        snackbar = SnackbarWidget(
            msg, icon=pix, timeout=timeout, action_text=action_text, action=action, animate=animate
        )
        cls.get_manager().add_snackbar(snackbar)

    @classmethod
    def congratulate(cls, msg, timeout=3000, action_text=None, action=None, animate=True):
        pix = Pixmap(":/icons/images/material/check_circle.svg")
        pix.replace_color(QColor(0, 0, 0), QColor(73, 208, 86))
        snackbar = SnackbarWidget(
            msg, icon=pix, timeout=timeout, action_text=action_text, action=action, animate=animate
        )
        cls.get_manager().add_snackbar(snackbar)

    @classmethod
    def warn(cls, msg, timeout=3000, action_text=None, action=None, animate=True):
        pix = Pixmap(":/icons/images/material/report_problem.svg")
        pix.replace_color(QColor(0, 0, 0), QColor(208, 102, 73))
        snackbar = SnackbarWidget(
            msg, icon=pix, timeout=timeout, action_text=action_text, action=action, animate=animate
        )
        cls.get_manager().add_snackbar(snackbar)
