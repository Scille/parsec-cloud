# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import sys

from PyQt5.QtWidgets import QApplication
from PyQt5.QtCore import QFile, QEvent
from PyQt5.QtGui import QFont, QFontDatabase, QPalette, QColor
from structlog import get_logger


logger = get_logger()


class ParsecApp(QApplication):
    connected_devices = set()

    def __init__(self):
        super().__init__(["-stylesheet"])
        self.setOrganizationName("Scille")
        self.setOrganizationDomain("parsec.cloud")
        self.setApplicationName("Parsec")
        pal = self.palette()
        pal.setColor(QPalette.Link, QColor(0x00, 0x92, 0xFF))
        pal.setColor(QPalette.LinkVisited, QColor(0x00, 0x70, 0xDD))
        self.setPalette(pal)

    def event(self, e):
        """Handle macOS FileOpen events."""
        if sys.platform == "darwin" and e.type() == QEvent.ApplicationActivate:
            # Necessary to reopen window with dock icon after being closed with
            # red X on MacOS.
            # There are three events related to the dock icon click:
            # ApplicationActivate, ApplicationDeactivate and ApplicationStateChange.
            # The events ApplicationDeactivate and ApplicationStateChange are
            # tied to whenever the window state changes from foreground to
            # background. Inversely, the events ApplicationActivate and
            # ApplicationStateChange can be caught when switching from
            # background to foreground, or clicking the dock icon.
            # Even though ApplicationActivate is said to be deprecated, it's
            # seemingly the only event we can use for this particular case,
            # ApplicationStateChange not being specific enough.
            self.get_main_window().show_top()
        if e.type() != QEvent.FileOpen or e.url().scheme() != "parsec":
            return super().event(e)
        try:
            url = e.url().url()
            mw = self.get_main_window()
            if not mw:
                pass
            else:
                mw.new_instance_needed.emit(url)
        except Exception:
            logger.exception("Url handling failed")

        return True

    def load_stylesheet(self, res=":/styles/styles/main.css"):
        rc = QFile(res)
        rc.open(QFile.ReadOnly)
        content = rc.readAll().data()
        self.setStyleSheet(str(content, "utf-8"))

    def load_font(self, font="Montserrat"):
        QFontDatabase.addApplicationFont(":/fonts/fonts/Montserrat.ttf")
        QFontDatabase.addApplicationFont(":/fonts/fonts/Roboto-Regular.ttf")
        f = QFont(font)
        self.setFont(f)

    @classmethod
    def add_connected_device(cls, org_id, device_id):
        cls.connected_devices.add((org_id, device_id))

    @classmethod
    def remove_connected_device(cls, org_id, device_id):
        cls.connected_devices.discard((org_id, device_id))

    @classmethod
    def is_device_connected(cls, org_id, device_id):
        return (org_id, device_id) in cls.connected_devices

    @classmethod
    def has_active_modal(cls):
        if cls.activeModalWidget():
            return True
        mw = cls.get_main_window()
        if not mw:
            return False
        for win in mw.children():
            if win.objectName() == "GreyedDialog":
                return True
        return False

    @classmethod
    def get_main_window(cls):
        for win in cls.topLevelWidgets():
            if win.objectName() == "MainWindow":
                return win
        return None
