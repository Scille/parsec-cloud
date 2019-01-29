from PyQt5.QtCore import QUrl, QFileInfo, QSysInfo, QLocale
from PyQt5.QtGui import QDesktopServices, QGuiApplication, QClipboard

from parsec.types import DeviceName


def open_file(path):
    QDesktopServices.openUrl(QUrl.fromLocalFile(QFileInfo(path).absoluteFilePath()))


def get_default_device():
    return "".join([c for c in QSysInfo.productType() if DeviceName.regex.match(c)])


def get_locale_language():
    return QLocale.system().name()[:2].lower()


def copy_to_clipboard(text):
    QGuiApplication.clipboard().setText(text, QClipboard.Clipboard)
    QGuiApplication.clipboard().setText(text, QClipboard.Selection)
