# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from enum import IntEnum
import pathlib

from PyQt5.QtCore import Qt
from PyQt5.QtWidgets import QTableWidgetItem
from PyQt5.QtGui import QIcon, QPixmap, QPainter


NAME_DATA_INDEX = Qt.UserRole
TYPE_DATA_INDEX = Qt.UserRole + 1
UUID_DATA_INDEX = Qt.UserRole + 2
COPY_STATUS_DATA_INDEX = Qt.UserRole + 3


class FileType(IntEnum):
    ParentWorkspace = 1
    ParentFolder = 2
    Folder = 3
    File = 4
    Inconsistency = 5


class CustomTableItem(QTableWidgetItem):
    def __lt__(self, other):
        self_type = self.data(TYPE_DATA_INDEX)
        other_type = other.data(TYPE_DATA_INDEX)
        self_data = self.data(NAME_DATA_INDEX)
        other_data = other.data(NAME_DATA_INDEX)

        if self_type == FileType.ParentWorkspace or self_type == FileType.ParentFolder:
            return False
        elif other_type == FileType.ParentWorkspace or other_type == FileType.ParentFolder:
            return False
        return self_data < other_data


class IconTableItem(CustomTableItem):
    def __init__(self, is_synced, pixmap, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.synced_pixmap = self._draw_synced_pixmap(pixmap)
        self.unsynced_pixmap = self._draw_unsynced_pixmap(pixmap)
        self.is_synced = is_synced

    def _draw_synced_pixmap(self, source):
        p = QPixmap(source)
        m = QPixmap(":/icons/images/icons/checked.png")
        painter = QPainter(p)
        painter.drawPixmap(p.width() - 50, 0, 50, 50, m)
        painter.end()
        return p

    def _draw_unsynced_pixmap(self, source):
        p = QPixmap(source)
        m = QPixmap(":/icons/images/icons/menu_cancel.png")
        painter = QPainter(p)
        painter.drawPixmap(p.width() - 50, 0, 50, 50, m)
        painter.end()
        return p

    @property
    def is_synced(self):
        return self._is_synced

    @is_synced.setter
    def is_synced(self, value):
        self._is_synced = value
        if self._is_synced:
            self.setIcon(QIcon(self.synced_pixmap))
        else:
            self.setIcon(QIcon(self.unsynced_pixmap))


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
        icon = self.EXTENSIONS.get(ext.lower(), "file_unknown")
        super().__init__(
            is_synced, ":/icons/images/icons/file_icons/{}.png".format(icon), "", *args, **kwargs
        )
        self.setData(TYPE_DATA_INDEX, FileType.File)


class FolderTableItem(IconTableItem):
    def __init__(self, is_synced, *args, **kwargs):
        super().__init__(
            is_synced, ":/icons/images/icons/file_icons/folder.png", "", *args, **kwargs
        )
        self.setData(NAME_DATA_INDEX, 0)
        self.setData(TYPE_DATA_INDEX, FileType.Folder)


class InconsistencyTableItem(IconTableItem):
    def __init__(self, is_synced, *args, **kwargs):
        super().__init__(is_synced, ":/icons/images/icons/warning.png", "", *args, **kwargs)
        self.setData(NAME_DATA_INDEX, 0)
        self.setData(TYPE_DATA_INDEX, FileType.Inconsistency)
