# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from PyQt5.QtGui import QIcon
from PyQt5.QtWidgets import QMenu, QSystemTrayIcon

from parsec.core.gui.lang import translate as _


def systray_available() -> bool:
    return QSystemTrayIcon.isSystemTrayAvailable()


class Systray(QSystemTrayIcon):
    def __init__(self, **kwargs):
        super().__init__(**kwargs)

        self.menu = QMenu()
        self.show_action = self.menu.addAction(_("ACTION_SYSTRAY_SHOW_WINDOW"))
        self.close_action = self.menu.addAction(_("ACTION_SYSTRAY_QUIT_PARSEC"))

        self.on_show = self.show_action.triggered
        self.on_close = self.close_action.triggered

        self.setContextMenu(self.menu)
        self.setIcon(QIcon(":/logos/images/icons/parsec.png"))
        self.activated.connect(self.on_activated)
        self.show()

    def on_systray_notification(self, title, msg, msec=2000):
        self.showMessage(title, msg, msecs=msec)

    def on_activated(self, reason):
        if reason == QSystemTrayIcon.DoubleClick:
            self.on_show.emit()
