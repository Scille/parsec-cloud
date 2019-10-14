# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from collections import namedtuple

from PyQt5.QtCore import Qt, pyqtSignal
from PyQt5.QtGui import QIcon
from PyQt5.QtWidgets import QTableWidget, QHeaderView

from parsec.core.types import WorkspaceRole

from parsec.core.gui.lang import format_datetime
from parsec.core.gui.file_items import (
    CustomTableItem,
    FileType,
    NAME_DATA_INDEX,
    TYPE_DATA_INDEX,
    UUID_DATA_INDEX,
)
from parsec.core.gui.file_size import get_filesize

from parsec.core.gui.file_table import ItemDelegate


class VersionsTable(QTableWidget):
    file_moved = pyqtSignal(str, str)
    item_activated = pyqtSignal(FileType, str)
    files_dropped = pyqtSignal(list, str)
    delete_clicked = pyqtSignal()
    rename_clicked = pyqtSignal()
    open_clicked = pyqtSignal()
    show_history_clicked = pyqtSignal()

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.previous_selection = []
        self.setColumnCount(5)
        h_header = self.horizontalHeader()
        h_header.setDefaultAlignment(Qt.AlignLeft | Qt.AlignVCenter)

        h_header.setSectionResizeMode(0, QHeaderView.Fixed)
        h_header.setSectionResizeMode(1, QHeaderView.Stretch)
        h_header.setSectionResizeMode(2, QHeaderView.Stretch)
        h_header.setSectionResizeMode(3, QHeaderView.Stretch)
        h_header.setSectionResizeMode(4, QHeaderView.Stretch)

        self.setColumnWidth(0, 300)
        self.setColumnWidth(1, 200)
        self.setColumnWidth(2, 200)
        self.setColumnWidth(3, 100)
        self.setColumnWidth(4, 100)

        v_header = self.verticalHeader()
        v_header.setSectionResizeMode(QHeaderView.Fixed)
        v_header.setDefaultSectionSize(48)
        self.setItemDelegate(ItemDelegate())
        self.setContextMenuPolicy(Qt.CustomContextMenu)
        self.itemSelectionChanged.connect(self.change_selection)
        self.cellDoubleClicked.connect(self.item_double_clicked)
        self.current_user_role = WorkspaceRole.OWNER

    def selected_files(self):
        SelectedFile = namedtuple("SelectedFile", ["row", "type", "name", "uuid"])

        files = []
        for r in self.selectedRanges():
            for row in range(r.topRow(), r.bottomRow() + 1):
                item = self.item(row, 1)
                files.append(
                    SelectedFile(
                        row,
                        item.data(TYPE_DATA_INDEX),
                        item.data(NAME_DATA_INDEX),
                        item.data(UUID_DATA_INDEX),
                    )
                )
        return files

    def has_file(self, uuid):
        return any(
            uuid == self.item(row, 1).data(UUID_DATA_INDEX) for row in range(self.rowCount())
        )

    def item_double_clicked(self, row, column):
        name_item = self.item(row, 1)
        type_item = self.item(row, 0)
        file_type = type_item.data(TYPE_DATA_INDEX)
        try:
            self.item_activated.emit(file_type, name_item.data(NAME_DATA_INDEX))
        except AttributeError:
            # This can happen when updating the list: double click event gets processed after
            # the item has been removed.
            pass

    def clear(self):
        self.clearContents()
        self.setRowCount(0)
        self.previous_selection = []

    def change_selection(self):
        selected = self.selectedItems()
        for item in self.previous_selection:
            if item.column() == 0:
                file_type = item.data(TYPE_DATA_INDEX)
                if file_type == FileType.ParentWorkspace or file_type == FileType.ParentFolder:
                    item.setIcon(QIcon(":/icons/images/icons/folder-up.png"))
        for item in selected:
            if item.column() == 0:
                file_type = item.data(TYPE_DATA_INDEX)
                if file_type == FileType.ParentWorkspace or file_type == FileType.ParentFolder:
                    item.setIcon(QIcon(":/icons/images/icons/folder-up_selected.png"))
        self.previous_selection = selected

    def _config_item_data(
        self, name, name_data_index, type_data_index, uuid_data_index, row_id, col_id
    ):
        name = "" if name is None else str(name)
        item = CustomTableItem(name)
        item.setToolTip("\n".join(name[i : i + 64] for i in range(0, len(name), 64)))
        item.setData(NAME_DATA_INDEX, name_data_index)
        item.setData(TYPE_DATA_INDEX, type_data_index)
        item.setData(UUID_DATA_INDEX, uuid_data_index)
        self.setItem(row_id, col_id, item)

    def addItem(
        self,
        entry_id,
        version,
        item_name,
        is_folder,
        creator,
        size,
        early_timestamp,
        late_timestamp,
        source_path,
        destination_path,
    ):
        def _config_data(col_id, name):
            self._config_item_data(
                name,
                name,
                FileType.Folder if is_folder else FileType.File,
                entry_id,
                row_id,
                col_id,
            )

        row_id = self.rowCount()
        self.insertRow(row_id)

        _config_data(0, format_datetime(early_timestamp))
        _config_data(1, creator)
        _config_data(2, get_filesize(size) if size is not None else "")
        _config_data(3, source_path)
        _config_data(4, destination_path)
