from PyQt5.QtCore import QUrl, QFileInfo, QSysInfo, QLocale
from PyQt5.QtGui import QDesktopServices


def open_file(path):
    QDesktopServices.openUrl(QUrl.fromLocalFile(QFileInfo(path).absoluteFilePath()))


def get_default_device():
    return QSysInfo.productType()


def get_locale_language():
    return QLocale.system().name()[:2].lower()
