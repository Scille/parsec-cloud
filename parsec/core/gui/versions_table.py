# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import Qt
from PyQt5.QtWidgets import QTableWidget, QHeaderView

from parsec.core.gui.lang import format_datetime
from parsec.core.gui.file_items import CustomTableItem, FileType
from parsec.core.gui.file_size import get_filesize
from parsec.core.gui.file_table import ItemDelegate


NAME_DATA_INDEX = Qt.UserRole
TYPE_DATA_INDEX = Qt.UserRole + 1
EARLY_TIMESTAMP_DATA_INDEX = Qt.UserRole + 2
LATE_TIMESTAMP_DATA_INDEX = Qt.UserRole + 3
SOURCE_PATH_DATA_INDEX = Qt.UserRole + 4
DESTINATION_PATH_DATA_INDEX = Qt.UserRole + 5
ACTUAL_PATH_DATA_INDEX = Qt.UserRole + 6


class VersionsTable(QTableWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
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
        self.cellDoubleClicked.connect(self.item_double_clicked)

    def set_reload_timestamped_signal(self, signal):
        self.reload_timestamped_signal = signal

    def item_double_clicked(self, row, column):
        target_path = self.item(row, 0).data(ACTUAL_PATH_DATA_INDEX)
        target_timestamp = self.item(row, 0).data(EARLY_TIMESTAMP_DATA_INDEX)
        if column == 3:  # source clicked
            source_path = self.item(row, 0).data(SOURCE_PATH_DATA_INDEX)
            if source_path is not None:
                target_path = source_path
                target_timestamp = (
                    self.item(row, 0).data(EARLY_TIMESTAMP_DATA_INDEX).add(microseconds=-1)
                )
        elif column == 4:  # destination clicked
            destination_path = self.item(row, 0).data(DESTINATION_PATH_DATA_INDEX)
            if destination_path is not None:
                target_path = destination_path
                target_timestamp = self.item(row, 0).data(LATE_TIMESTAMP_DATA_INDEX)
        file_type = self.item(row, 0).data(TYPE_DATA_INDEX)
        self.reload_timestamped_signal.emit(
            target_timestamp,
            target_path,
            file_type,
            column < 3 and file_type is not FileType.Folder,
            False,
            column >= 3,
        )

    def clear(self):
        self.clearContents()
        self.setRowCount(0)

    def _config_item_data(
        self, name, actual_path, type_data, early, late, source, destination, row_id, col_id
    ):
        name = "" if name is None else str(name)
        item = CustomTableItem(name)
        item.setToolTip("\n".join(name[i : i + 64] for i in range(0, len(name), 64)))
        item.setData(NAME_DATA_INDEX, name)
        item.setData(TYPE_DATA_INDEX, type_data)
        item.setData(EARLY_TIMESTAMP_DATA_INDEX, early)
        item.setData(LATE_TIMESTAMP_DATA_INDEX, late)
        item.setData(SOURCE_PATH_DATA_INDEX, source)
        item.setData(DESTINATION_PATH_DATA_INDEX, destination)
        item.setData(ACTUAL_PATH_DATA_INDEX, actual_path)
        self.setItem(row_id, col_id, item)

    def add_item(
        self,
        entry_id,
        version,
        actual_path,
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
                actual_path,
                FileType.Folder if is_folder else FileType.File,
                early_timestamp,
                late_timestamp,
                source_path,
                destination_path,
                row_id,
                col_id,
            )

        row_id = self.rowCount()
        self.insertRow(row_id)
        _config_data(0, format_datetime(early_timestamp, seconds=True))
        _config_data(1, creator)
        _config_data(2, get_filesize(size) if size is not None else "")
        _config_data(3, source_path)
        _config_data(4, destination_path)
