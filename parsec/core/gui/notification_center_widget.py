from PyQt5.QtCore import Qt
from PyQt5.QtGui import QPainter
from PyQt5.QtWidgets import (
    QWidget,
    QStyle,
    QStyleOption,
    QListWidgetItem,
    QStyledItemDelegate,
    QStyleOptionViewItem,
)

from parsec.core.gui.notification_widget import (
    ErrorNotificationWidget,
    WarningNotificationWidget,
    InfoNotificationWidget,
)
from parsec.core.gui.ui.notification_center_widget import Ui_NotificationCenterWidget


class ItemDelegate(QStyledItemDelegate):
    def paint(self, painter, option, index):
        view_option = QStyleOptionViewItem(option)
        view_option.decorationAlignment |= Qt.AlignHCenter
        # Even though we told Qt that we don't want any selection on our
        # list, it still adds a blue rectangle on focused items.
        # So we just get rid of focus.
        if option.state & QStyle.State_HasFocus:
            view_option.state &= ~QStyle.State_HasFocus
        super().paint(painter, view_option, index)


class NotificationCenterWidget(QWidget, Ui_NotificationCenterWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.list_notifications.setItemDelegate(ItemDelegate())

    def paintEvent(self, _):
        opt = QStyleOption()
        opt.initFrom(self)
        p = QPainter(self)
        self.style().drawPrimitive(QStyle.PE_Widget, opt, p, self)

    def add_notification(self, notif_type, msg):
        widget = None
        if notif_type == "ERROR":
            widget = ErrorNotificationWidget()
        elif notif_type == "WARNING":
            widget = WarningNotificationWidget()
        elif notif_type == "INFO":
            widget = InfoNotificationWidget()
        if not widget:
            return
        widget.message = msg
        item = QListWidgetItem()
        item.setSizeHint(widget.size())
        self.list_notifications.addItem(item)
        self.list_notifications.setItemWidget(item, widget)
