# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from enum import IntEnum
import pathlib

from PyQt5.QtCore import Qt
from PyQt5.QtWidgets import QTableWidgetItem
from PyQt5.QtGui import QIcon


class FileType(IntEnum):
    ParentWorkspace = 1
    ParentFolder = 2
    Folder = 3
    File = 4


class CustomTableItem(QTableWidgetItem):
    def __lt__(self, other):
        return (
            self.data(Qt.UserRole) is not None
            and other.data(Qt.UserRole) is not None
            and self.data(Qt.UserRole) < other.data(Qt.UserRole)
        )


class FileTableItem(CustomTableItem):
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

    def __init__(self, file_name, *args, **kwargs):
        ext = pathlib.Path(file_name).suffix
        icon = self.EXTENSIONS.get(ext, "file_unknown")
        super().__init__(QIcon(":/icons/images/icons/{}.png".format(icon)), "", *args, **kwargs)
        self.setData(Qt.UserRole, FileType.File)


class ParentFolderTableItem(CustomTableItem):
    def __init__(self, *args, **kwargs):
        super().__init__(QIcon(":/icons/images/icons/folder-up.png"), "", *args, **kwargs)
        self.setData(Qt.UserRole, FileType.ParentFolder)


class ParentWorkspaceTableItem(CustomTableItem):
    def __init__(self, *args, **kwargs):
        super().__init__(QIcon(":/icons/images/icons/folder-up.png"), "", *args, **kwargs)
        self.setData(Qt.UserRole, FileType.ParentWorkspace)


class FolderTableItem(CustomTableItem):
    def __init__(self, *args, **kwargs):
        super().__init__(QIcon(":/icons/images/icons/folder.png"), "", *args, **kwargs)
        self.setData(Qt.UserRole, FileType.Folder)
