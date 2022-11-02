# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations
from typing import Any

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtGui import QPaintEvent, QPainter, QPixmap
from PyQt5.QtWidgets import QWidget, QStyle, QStyleOption

from parsec.core.gui.ui.menu_widget import Ui_MenuWidget


class MenuWidget(QWidget, Ui_MenuWidget):
    files_clicked = pyqtSignal()
    users_clicked = pyqtSignal()
    devices_clicked = pyqtSignal()
    enrollment_clicked = pyqtSignal()

    def __init__(self, *args: Any, **kwargs: Any) -> None:
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.button_files.clicked.connect(self.files_clicked.emit)
        self.button_users.clicked.connect(self.users_clicked.emit)
        self.button_devices.clicked.connect(self.devices_clicked.emit)
        self.button_enrollment.clicked.connect(self.enrollment_clicked.emit)
        self.button_files.apply_style()
        self.button_users.apply_style()
        self.button_devices.apply_style()
        self.button_enrollment.apply_style()
        self.icon_connection.apply_style()
        self.button_enrollment.hide()

    def paintEvent(self, _: QPaintEvent) -> None:
        opt = QStyleOption()
        opt.initFrom(self)
        p = QPainter(self)
        self.style().drawPrimitive(QStyle.PE_Widget, opt, p, self)

    def activate_files(self) -> None:
        self._deactivate_all()
        self.button_files.setChecked(True)

    def activate_devices(self) -> None:
        self._deactivate_all()
        self.button_devices.setChecked(True)

    def activate_users(self) -> None:
        self._deactivate_all()
        self.button_users.setChecked(True)

    def activate_enrollment(self) -> None:
        self._deactivate_all()
        self.button_enrollment.setChecked(True)

    def _deactivate_all(self) -> None:
        self.button_files.setChecked(False)
        self.button_users.setChecked(False)
        self.button_devices.setChecked(False)
        self.button_enrollment.setChecked(False)

    def set_connection_state(self, text: str, tooltip: str, icon: QPixmap) -> None:
        self.label_connection_state.setText(text)
        self.label_connection_state.setToolTip(tooltip)
        self.icon_connection.setPixmap(icon)
        self.icon_connection.apply_style()
