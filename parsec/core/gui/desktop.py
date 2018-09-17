from PyQt5.QtCore import QUrl, QFileInfo, QSysInfo
from PyQt5.QtGui import QDesktopServices


def open_file(path):
    QDesktopServices.openUrl(QUrl(QFileInfo(path).absoluteFilePath()))


def get_default_device():
    return QSysInfo.productType()
