from hurry.filesize import size

from PyQt5.QtCore import QCoreApplication


def get_filesize(bytesize):
    SYSTEM = [
        (1024 ** 4, QCoreApplication.translate("FileSize", " TB")),
        (1024 ** 3, QCoreApplication.translate("FileSize", " GB")),
        (1024 ** 2, QCoreApplication.translate("FileSize", " MB")),
        (1024 ** 1, QCoreApplication.translate("FileSize", " KB")),
        (1024 ** 0, QCoreApplication.translate("FileSize", " B")),
    ]

    return size(bytesize, system=SYSTEM)
