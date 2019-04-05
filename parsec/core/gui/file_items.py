# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from enum import IntEnum
import pathlib

from PyQt5.QtCore import Qt
from PyQt5.QtWidgets import QTableWidgetItem
from PyQt5.QtGui import QIcon, QPixmap, QPainter


class FileType(IntEnum):
    ParentWorkspace = 1
    ParentFolder = 2
    Folder = 3
    File = 4


class CustomTableItem(QTableWidgetItem):
    def __lt__(self, other):
        self_type = self.data(Qt.UserRole + 1)
        other_type = other.data(Qt.UserRole + 1)
        self_data = self.data(Qt.UserRole)
        other_data = other.data(Qt.UserRole)

        if self_type == FileType.ParentWorkspace or self_type == FileType.ParentFolder:
            return False
        elif other_type == FileType.ParentWorkspace or other_type == FileType.ParentFolder:
            return False
        return self_data < other_data


class IconTableItem(CustomTableItem):
    def __init__(self, is_synced, pixmap, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.base_pixmap = pixmap
        self.is_synced = is_synced

    @property
    def is_synced(self):
        return self._is_synced

    @is_synced.setter
    def is_synced(self, value):
        self._is_synced = value
        p = self.base_pixmap
        if self._is_synced:
            m = QPixmap(":/icons/images/icons/checked.png")
        else:
            m = QPixmap(":/icons/images/icons/menu_cancel.png")
        painter = QPainter(p)
        painter.drawPixmap(p.width() - 50, 0, 50, 50, m)
        self.setIcon(QIcon(p))


class FileTableItem(IconTableItem):
    EXTENSIONS = {
        ".jpg": "file_jpg",
        ".jpeg": "file_jpg",
        ".png": "file_png",
        ".avi": "file_avi",
        ".mp4": "file_mp4",
        ".xls": "file_xls",
        ".xlsx": "file_xls",
        ".doc": "file_doc",
        ".docx": "file_doc",
        ".rtf": "file_rtf",
        ".mp3": "file_mp3",
        ".zip": "file_zip",
        ".rar": "file_archive",
        ".gz": "file_archive",
        ".tar": "file_archive",
        ".bz": "file_archive",
        ".bz2": "file_archive",
        ".7z": "file_archive",
        ".ppt": "file_ppt",
        ".pptx": "file_ppt",
        ".psd": "file_psd",
        ".json": "file_json",
        ".js": "file_js",
        ".css": "file_css",
        ".exe": "file_exe",
        ".html": "file_html",
        ".htm": "file_html",
        ".dbf": "file_dbf",
        ".dwg": "file_dwg",
        ".fla": "file_fla",
        ".iso": "file_iso",
        ".pdf": "file_pdf",
        ".txt": "file_txt",
        ".xml": "file_xml",
    }

    def __init__(self, is_synced, file_name, *args, **kwargs):
        ext = pathlib.Path(file_name).suffix
        icon = self.EXTENSIONS.get(ext, "file_unknown")
        super().__init__(
            is_synced, QPixmap(":/icons/images/icons/{}.png".format(icon)), "", *args, **kwargs
        )
        self.setData(Qt.UserRole + 1, FileType.File)


class FolderTableItem(IconTableItem):
    def __init__(self, is_synced, *args, **kwargs):
        super().__init__(is_synced, QPixmap(":/icons/images/icons/folder.png"), "", *args, **kwargs)
        self.setData(Qt.UserRole, 0)
        self.setData(Qt.UserRole + 1, FileType.Folder)
