# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import QCoreApplication
from PyQt5.QtGui import QIcon
from PyQt5.QtWidgets import QMenu, QSystemTrayIcon


def systray_available() -> bool:
    return QSystemTrayIcon.isSystemTrayAvailable()


class Systray(QSystemTrayIcon):
    def __init__(self, **kwargs):
        super().__init__(**kwargs)

        self.menu = QMenu()
        self.show_action = self.menu.addAction(
            QCoreApplication.translate("MainWindow", "Show window")
        )
        self.close_action = self.menu.addAction(QCoreApplication.translate("MainWindow", "Exit"))

        self.on_show = self.show_action.triggered
        self.on_close = self.close_action.triggered

        self.setContextMenu(self.menu)
        self.setIcon(QIcon(":/icons/images/icons/parsec.png"))
        self.activated.connect(self.on_activated)
        self.show()

    def on_activated(self, reason):
        if reason == QSystemTrayIcon.DoubleClick:
            self.on_show.emit()
