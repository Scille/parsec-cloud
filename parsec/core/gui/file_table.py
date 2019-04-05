# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pendulum

from PyQt5.QtCore import Qt, pyqtSignal
from PyQt5.QtGui import QIcon
from PyQt5.QtWidgets import (
    QTableWidget,
    QHeaderView,
    QStyledItemDelegate,
    QStyleOptionViewItem,
    QStyle,
)

from parsec.core.gui.lang import translate as _
from parsec.core.gui.file_items import FileTableItem, CustomTableItem, FolderTableItem, FileType
from parsec.core.gui.file_size import get_filesize


class ItemDelegate(QStyledItemDelegate):
    def paint(self, painter, option, index):
        view_option = QStyleOptionViewItem(option)
        view_option.decorationAlignment |= Qt.AlignHCenter
        # Qt tries to be nice and adds a lovely background color
        # on the focused item. Since we select items by rows and not
        # individually, we don't want that, so we remove the focus
        if option.state & QStyle.State_HasFocus:
            view_option.state &= ~QStyle.State_HasFocus
        super().paint(painter, view_option, index)


class FileTable(QTableWidget):
    file_moved = pyqtSignal(str, str)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.previous_selection = []

    def init(self):
        h_header = self.horizontalHeader()
        h_header.setDefaultAlignment(Qt.AlignLeft | Qt.AlignVCenter)
        h_header.setSectionResizeMode(0, QHeaderView.Fixed)
        self.setColumnWidth(0, 60)
        h_header.setSectionResizeMode(1, QHeaderView.Stretch)
        h_header.setSectionResizeMode(2, QHeaderView.Fixed)
        self.setColumnWidth(2, 200)
        h_header.setSectionResizeMode(3, QHeaderView.Fixed)
        self.setColumnWidth(3, 200)
        h_header.setSectionResizeMode(4, QHeaderView.Fixed)
        self.setColumnWidth(4, 100)
        v_header = self.verticalHeader()
        v_header.setSectionResizeMode(QHeaderView.Fixed)
        v_header.setDefaultSectionSize(48)
        self.setItemDelegate(ItemDelegate())
        self.setContextMenuPolicy(Qt.CustomContextMenu)
        self.itemSelectionChanged.connect(self.change_selection)

    def clear(self):
        self.clearContents()
        self.setRowCount(0)
        self.previous_selection = []

    def change_selection(self):
        selected = self.selectedItems()
        for item in self.previous_selection:
            if item.column() == 0:
                file_type = item.data(Qt.UserRole + 1)
                if file_type == FileType.ParentWorkspace or file_type == FileType.ParentFolder:
                    item.setIcon(QIcon(":/icons/images/icons/folder-up.png"))
        for item in selected:
            if item.column() == 0:
                file_type = item.data(Qt.UserRole + 1)
                if file_type == FileType.ParentWorkspace or file_type == FileType.ParentFolder:
                    item.setIcon(QIcon(":/icons/images/icons/folder-up_selected.png"))
        self.previous_selection = selected

    def add_parent_folder(self):
        row_idx = self.rowCount()
        self.insertRow(row_idx)
        item = CustomTableItem(QIcon(":/icons/images/icons/folder-up.png"), "")
        item.setData(Qt.UserRole + 1, FileType.ParentFolder)
        self.setItem(row_idx, 0, item)
        item = CustomTableItem(_("Parent Folder"))
        item.setData(Qt.UserRole + 1, FileType.ParentFolder)
        self.setItem(row_idx, 1, item)
        item = CustomTableItem()
        item.setData(Qt.UserRole + 1, FileType.ParentFolder)
        self.setItem(row_idx, 2, item)
        item = CustomTableItem()
        item.setData(Qt.UserRole + 1, FileType.ParentFolder)
        self.setItem(row_idx, 3, item)
        item = CustomTableItem()
        item.setData(Qt.UserRole + 1, FileType.ParentFolder)
        self.setItem(row_idx, 4, item)

    def add_parent_workspace(self):
        row_idx = self.rowCount()
        self.insertRow(row_idx)
        item = CustomTableItem(QIcon(":/icons/images/icons/folder-up.png"), "")
        item.setData(Qt.UserRole + 1, FileType.ParentWorkspace)
        self.setItem(row_idx, 0, item)
        item = CustomTableItem(_("Parent Workspace"))
        item.setData(Qt.UserRole + 1, FileType.ParentWorkspace)
        self.setItem(row_idx, 1, item)
        item = CustomTableItem()
        item.setData(Qt.UserRole + 1, FileType.ParentWorkspace)
        self.setItem(row_idx, 2, item)
        item = CustomTableItem()
        item.setData(Qt.UserRole + 1, FileType.ParentWorkspace)
        self.setItem(row_idx, 3, item)
        item = CustomTableItem()
        item.setData(Qt.UserRole + 1, FileType.ParentWorkspace)
        self.setItem(row_idx, 4, item)

    def add_folder(self, folder_name, is_synced):
        row_idx = self.rowCount()
        self.insertRow(row_idx)
        item = FolderTableItem(is_synced)
        self.setItem(row_idx, 0, item)
        item = CustomTableItem(folder_name)
        item.setData(Qt.UserRole, folder_name)
        item.setData(Qt.UserRole + 1, FileType.Folder)
        self.setItem(row_idx, 1, item)
        item = CustomTableItem()
        item.setData(Qt.UserRole, pendulum.datetime(1970, 1, 1))
        item.setData(Qt.UserRole + 1, FileType.Folder)
        self.setItem(row_idx, 2, item)
        item = CustomTableItem()
        item.setData(Qt.UserRole, pendulum.datetime(1970, 1, 1))
        item.setData(Qt.UserRole + 1, FileType.Folder)
        self.setItem(row_idx, 3, item)
        item = CustomTableItem()
        item.setData(Qt.UserRole, -1)
        item.setData(Qt.UserRole + 1, FileType.Folder)
        self.setItem(row_idx, 4, item)

    def add_file(self, file_name, file_size, created_on, updated_on, is_synced):
        row_idx = self.rowCount()
        self.insertRow(row_idx)
        item = FileTableItem(is_synced, file_name)
        item.setData(Qt.UserRole, 1)
        self.setItem(row_idx, 0, item)
        item = CustomTableItem(file_name)
        item.setData(Qt.UserRole, file_name)
        item.setData(Qt.UserRole + 1, FileType.File)
        self.setItem(row_idx, 1, item)
        item = CustomTableItem(created_on.format("%x %X"))
        item.setData(Qt.UserRole, created_on)
        item.setData(Qt.UserRole + 1, FileType.File)
        self.setItem(row_idx, 2, item)
        item = CustomTableItem(updated_on.format("%x %X"))
        item.setData(Qt.UserRole, updated_on)
        item.setData(Qt.UserRole + 1, FileType.File)
        self.setItem(row_idx, 3, item)
        item = CustomTableItem(get_filesize(file_size))
        item.setData(Qt.UserRole, file_size)
        item.setData(Qt.UserRole + 1, FileType.File)
        self.setItem(row_idx, 4, item)

    def dropEvent(self, event):
        if event.source() != self:
            return
        target_row = self.indexAt(event.pos()).row()
        rows = set([i.row() for i in self.selectedIndexes() if i != target_row])
        if not rows:
            return
        file_type = self.item(target_row, 0).data(Qt.UserRole + 1)
        target_name = self.item(target_row, 1).text()

        if file_type != FileType.ParentFolder and file_type != FileType.Folder:
            return
        for row in rows:
            file_name = self.item(row, 1).text()
            if file_type == FileType.ParentFolder:
                self.file_moved.emit(file_name, "..")
            else:
                self.file_moved.emit(file_name, target_name)
            self.removeRow(row)
        event.accept()
