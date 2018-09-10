from PyQt5.QtCore import QUrl, QFileInfo
from PyQt5.QtGui import QDesktopServices


def open_file(path):
    QDesktopServices.openUrl(QUrl(QFileInfo(path).absoluteFilePath()))
