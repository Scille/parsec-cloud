# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import threading

from PyQt5.QtCore import QUrl, QFileInfo, QSysInfo, QLocale, pyqtSignal, QObject
from PyQt5.QtGui import QDesktopServices, QGuiApplication, QClipboard

from parsec.api.protocol import DeviceName


class FileOpener(QObject):
    file_opened = pyqtSignal(object, bool, list)

    def __init__(self):
        super().__init__()
        self.thread = None

    def _thread_run(self, paths):
        status = True
        for p in paths:
            status &= QDesktopServices.openUrl(QUrl.fromLocalFile(QFileInfo(p).absoluteFilePath()))
        self.file_opened.emit(self, status, paths)

    def open_files(self, paths):
        self.thread = threading.Thread(target=self._thread_run, args=(paths,))
        self.thread.start()

    def finish(self):
        self.thread.join()


def open_url(url):
    return QDesktopServices.openUrl(QUrl(url))


def open_doc_link():
    return open_url("https://parsec-cloud.readthedocs.io")


def open_feedback_link():
    return open_url("https://my.parsec.cloud/feedback")


def open_user_guide():
    return open_url("https://parsec.cloud")


def get_default_device():
    device = QSysInfo.machineHostName()
    if device.lower() == "localhost":
        device = QSysInfo.productType()
    return "".join([c for c in device if DeviceName.regex.match(c)])


def get_locale_language():
    return QLocale.system().name()[:2].lower()


def copy_to_clipboard(text):
    QGuiApplication.clipboard().setText(text, QClipboard.Clipboard)
    QGuiApplication.clipboard().setText(text, QClipboard.Selection)
