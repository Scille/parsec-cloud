# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations
from typing import Any, cast

from PyQt5.QtCore import Qt, pyqtSignal, QModelIndex
from PyQt5.QtGui import QPainter, QPaintEvent
from PyQt5.QtWidgets import QWidget, QStyle, QStyleOption, QStyledItemDelegate, QStyleOptionViewItem

from parsec.core.gui.notification_widget import (
    ErrorNotificationWidget,
    NotificationWidget,
    WarningNotificationWidget,
    InfoNotificationWidget,
)
from parsec.core.gui.ui.notification_center_widget import Ui_NotificationCenterWidget


class ItemDelegate(QStyledItemDelegate):
    def paint(self, painter: QPainter, option: QStyleOptionViewItem, index: QModelIndex) -> None:
        view_option = QStyleOptionViewItem(option)
        view_option.decorationAlignment |= Qt.AlignHCenter
        # Even though we told Qt that we don't want any selection on our
        # list, it still adds a blue rectangle on focused items.
        # So we just get rid of focus.
        if option.state & QStyle.State_HasFocus:
            view_option.state &= cast(QStyle.State, ~QStyle.State_HasFocus)
        super().paint(painter, view_option, index)


class NotificationCenterWidget(QWidget, Ui_NotificationCenterWidget):
    close_requested = pyqtSignal()

    def __init__(self, *args: Any, **kwargs: Any) -> None:
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.button_close.clicked.connect(self.close_requested.emit)

    def paintEvent(self, _: QPaintEvent) -> None:
        opt = QStyleOption()
        opt.initFrom(self)
        p = QPainter(self)
        self.style().drawPrimitive(QStyle.PE_Widget, opt, p, self)

    def close_notification(self, notif: QWidget) -> None:
        self.widget_layout.layout().removeWidget(notif)
        notif.hide()
        notif.setParent(None)  # type: ignore[call-overload]

    def clear(self) -> None:
        while self.widget_layout.layout().count() > 1:
            item = self.widget_layout.layout().takeAt(0)
            if item.widget():
                item.widget().hide()
                item.widget().setParent(None)

    def add_notification(self, notif_type: str, msg: str) -> None:
        widget: NotificationWidget | None = None
        if notif_type == "ERROR":
            widget = ErrorNotificationWidget()
        elif notif_type == "WARNING":
            widget = WarningNotificationWidget()
        elif notif_type == "INFO":
            widget = InfoNotificationWidget()
        if not widget:
            return
        widget.message = msg
        self.widget_layout.layout().insertWidget(0, widget)
        widget.close_clicked.connect(self.close_notification)
