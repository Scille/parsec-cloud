# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from enum import IntEnum

from PyQt5.QtCore import Qt
from PyQt5.QtWidgets import QTableWidgetItem
from PyQt5.QtGui import QIcon, QPainter, QColor

from parsec.core.gui.custom_widgets import Pixmap

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
    SYNCED_ICON = None
    UNSYNCED_ICON = None

    def __init__(self, is_synced, pixmap, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.source = pixmap
        if not IconTableItem.SYNCED_ICON:
            IconTableItem.SYNCED_ICON = Pixmap(":/icons/images/material/check.svg")
            IconTableItem.SYNCED_ICON.replace_color(QColor(0, 0, 0), QColor(50, 168, 82))
        if not IconTableItem.UNSYNCED_ICON:
            IconTableItem.UNSYNCED_ICON = Pixmap(":/icons/images/material/cached.svg")
            IconTableItem.UNSYNCED_ICON.replace_color(QColor(0, 0, 0), QColor(219, 20, 20))
        self._is_synced = is_synced
        self.switch_icon()

    def switch_icon(self):
        icon = self._draw_pixmap(self.source, self.isSelected(), self.is_synced)
        self.setIcon(QIcon(icon))

    def _draw_pixmap(self, source, selected, synced):
        color = QColor(0x33, 0x33, 0x33) if selected else QColor(0x99, 0x99, 0x99)
        p = Pixmap(source)
        p.replace_color(QColor(0, 0, 0), color)
        painter = QPainter(p)
        if synced:
            painter.drawPixmap(p.width() - 90, 0, 100, 100, IconTableItem.SYNCED_ICON)
        else:
            painter.drawPixmap(p.width() - 90, 0, 100, 100, IconTableItem.UNSYNCED_ICON)
        painter.end()
        return p

    @property
    def is_synced(self):
        return self._is_synced

    @is_synced.setter
    def is_synced(self, value):
        self._is_synced = value
        self.switch_icon()


class FileTableItem(IconTableItem):
    def __init__(self, is_synced, file_name, *args, **kwargs):
        super().__init__(is_synced, ":/icons/images/material/description.svg", "", *args, **kwargs)
        self.setData(TYPE_DATA_INDEX, FileType.File)


class FolderTableItem(IconTableItem):
    def __init__(self, is_synced, *args, **kwargs):
        super().__init__(is_synced, ":/icons/images/material/folder_open.svg", "", *args, **kwargs)
        self.setData(NAME_DATA_INDEX, 0)
        self.setData(TYPE_DATA_INDEX, FileType.Folder)


class InconsistencyTableItem(IconTableItem):
    def __init__(self, is_synced, *args, **kwargs):
        super().__init__(is_synced, ":/icons/images/material/warning.svg", "", *args, **kwargs)
        self.setData(NAME_DATA_INDEX, 0)
        self.setData(TYPE_DATA_INDEX, FileType.Inconsistency)
