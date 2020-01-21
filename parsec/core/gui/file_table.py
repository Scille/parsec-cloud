# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from collections import namedtuple
import pendulum
import pathlib

from PyQt5.QtCore import Qt, pyqtSignal
from PyQt5.QtGui import QIcon, QColor, QKeySequence
from PyQt5.QtWidgets import (
    QTableWidget,
    QHeaderView,
    QStyledItemDelegate,
    QStyleOptionViewItem,
    QStyle,
    QMenu,
)

from parsec.core.types import WorkspaceRole

from parsec.core.gui.lang import translate as _, format_datetime
from parsec.core.gui.file_items import (
    FileTableItem,
    CustomTableItem,
    FolderTableItem,
    InconsistencyTableItem,
    FileType,
    NAME_DATA_INDEX,
    TYPE_DATA_INDEX,
    UUID_DATA_INDEX,
    COPY_STATUS_DATA_INDEX,
)
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
        if index.data(COPY_STATUS_DATA_INDEX):
            view_option.font.setItalic(True)
        super().paint(painter, view_option, index)


class FileTable(QTableWidget):
    FIXED_COL_SIZE = 560
    NAME_COL_MIN_SIZE = 150

    file_moved = pyqtSignal(str, str)
    item_activated = pyqtSignal(FileType, str)
    files_dropped = pyqtSignal(list, str)
    delete_clicked = pyqtSignal()
    rename_clicked = pyqtSignal()
    open_clicked = pyqtSignal()
    show_history_clicked = pyqtSignal()
    paste_clicked = pyqtSignal()
    cut_clicked = pyqtSignal()
    copy_clicked = pyqtSignal()

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.previous_selection = []
        self.setColumnCount(5)

        h_header = self.horizontalHeader()
        h_header.setDefaultAlignment(Qt.AlignLeft | Qt.AlignVCenter)

        h_header.setSectionResizeMode(0, QHeaderView.Fixed)
        h_header.setSectionResizeMode(1, QHeaderView.Fixed)
        h_header.setSectionResizeMode(2, QHeaderView.Fixed)
        h_header.setSectionResizeMode(3, QHeaderView.Fixed)
        h_header.setSectionResizeMode(4, QHeaderView.Fixed)

        self.setColumnWidth(0, 60)
        self.setColumnWidth(
            1, max(self.size().width() - FileTable.FIXED_COL_SIZE, FileTable.NAME_COL_MIN_SIZE)
        )
        self.setColumnWidth(2, 200)
        self.setColumnWidth(3, 200)
        self.setColumnWidth(4, 100)

        v_header = self.verticalHeader()
        v_header.setSectionResizeMode(QHeaderView.Fixed)
        v_header.setDefaultSectionSize(48)
        self.setItemDelegate(ItemDelegate())
        self.setContextMenuPolicy(Qt.CustomContextMenu)
        self.itemSelectionChanged.connect(self.change_selection)
        self.customContextMenuRequested.connect(self.show_context_menu)
        self.cellDoubleClicked.connect(self.item_double_clicked)
        self.current_user_role = WorkspaceRole.OWNER

    def resizeEvent(self, event):
        super().resizeEvent(event)
        self.setColumnWidth(
            1, max(event.size().width() - FileTable.FIXED_COL_SIZE, FileTable.NAME_COL_MIN_SIZE)
        )

    def set_rows_cut(self, rows):
        for row in range(self.rowCount()):
            for col in range(5):
                item = self.item(row, col)
                item.setData(COPY_STATUS_DATA_INDEX, row in rows)

    def keyReleaseEvent(self, event):
        if event.matches(QKeySequence.Copy):
            self.copy_clicked.emit()
        elif event.matches(QKeySequence.Cut):
            self.cut_clicked.emit()
        elif event.matches(QKeySequence.Paste):
            self.paste_clicked.emit()

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

    def show_context_menu(self, pos):
        global_pos = self.mapToGlobal(pos)

        selected = self.selected_files()
        menu = QMenu(self)

        if len(selected):
            action = menu.addAction(_("FILE_MENU_OPEN"))
            action.triggered.connect(self.open_clicked.emit)
            action = menu.addAction(_("FILE_MENU_HISTORY"))
            action.triggered.connect(self.show_history_clicked.emit)
            if self.current_user_role != WorkspaceRole.READER:
                action = menu.addAction(_("FILE_MENU_RENAME"))
                action.triggered.connect(self.rename_clicked.emit)
                action = menu.addAction(_("FILE_MENU_DELETE"))
                action.triggered.connect(self.delete_clicked.emit)
            action = menu.addAction(_("FILE_MENU_COPY"))
            action.triggered.connect(self.copy_clicked.emit)
            action = menu.addAction(_("FILE_MENU_CUT"))
            action.triggered.connect(self.cut_clicked.emit)
        action = menu.addAction(_("FILE_MENU_PASTE"))
        action.triggered.connect(self.paste_clicked.emit)
        menu.exec_(global_pos)

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

    def add_parent_folder(self):
        row_idx = self.rowCount()
        self.insertRow(row_idx)
        item = CustomTableItem(QIcon(":/icons/images/icons/folder-up.png"), "")
        item.setData(TYPE_DATA_INDEX, FileType.ParentFolder)
        item.setFlags(Qt.ItemIsEnabled)
        self.setItem(row_idx, 0, item)
        item = CustomTableItem(_("FILE_TREE_PARENT_FOLDER"))
        item.setToolTip(_("FILE_TREE_PARENT_FOLDER_TOOLTIP"))
        item.setData(TYPE_DATA_INDEX, FileType.ParentFolder)
        item.setFlags(Qt.ItemIsEnabled)
        self.setItem(row_idx, 1, item)
        item = CustomTableItem()
        item.setData(TYPE_DATA_INDEX, FileType.ParentFolder)
        item.setFlags(Qt.ItemIsEnabled)
        self.setItem(row_idx, 2, item)
        item = CustomTableItem()
        item.setData(TYPE_DATA_INDEX, FileType.ParentFolder)
        item.setFlags(Qt.ItemIsEnabled)
        self.setItem(row_idx, 3, item)
        item = CustomTableItem()
        item.setFlags(Qt.ItemIsEnabled)
        item.setData(TYPE_DATA_INDEX, FileType.ParentFolder)
        self.setItem(row_idx, 4, item)

    def add_parent_workspace(self):
        row_idx = self.rowCount()
        self.insertRow(row_idx)
        item = CustomTableItem(QIcon(":/icons/images/icons/folder-up.png"), "")
        item.setData(TYPE_DATA_INDEX, FileType.ParentWorkspace)
        item.setFlags(Qt.ItemIsEnabled)
        self.setItem(row_idx, 0, item)
        item = CustomTableItem(_("FILE_TREE_PARENT_WORKSPACE"))
        item.setToolTip(_("FILE_TREE_PARENT_WORKSPACE_TOOLTIP"))
        item.setData(TYPE_DATA_INDEX, FileType.ParentWorkspace)
        item.setFlags(Qt.ItemIsEnabled)
        self.setItem(row_idx, 1, item)
        item = CustomTableItem()
        item.setData(TYPE_DATA_INDEX, FileType.ParentWorkspace)
        item.setFlags(Qt.ItemIsEnabled)
        self.setItem(row_idx, 2, item)
        item = CustomTableItem()
        item.setData(TYPE_DATA_INDEX, FileType.ParentWorkspace)
        item.setFlags(Qt.ItemIsEnabled)
        self.setItem(row_idx, 3, item)
        item = CustomTableItem()
        item.setData(TYPE_DATA_INDEX, FileType.ParentWorkspace)
        item.setFlags(Qt.ItemIsEnabled)
        self.setItem(row_idx, 4, item)

    def add_folder(self, folder_name, uuid, is_synced):
        row_idx = self.rowCount()
        self.insertRow(row_idx)
        item = FolderTableItem(is_synced)
        item.setData(UUID_DATA_INDEX, uuid)
        self.setItem(row_idx, 0, item)
        item = CustomTableItem(folder_name)
        item.setData(NAME_DATA_INDEX, folder_name)
        item.setToolTip("\n".join(folder_name[i : i + 64] for i in range(0, len(folder_name), 64)))
        item.setData(TYPE_DATA_INDEX, FileType.Folder)
        item.setData(UUID_DATA_INDEX, uuid)
        self.setItem(row_idx, 1, item)
        item = CustomTableItem()
        item.setData(NAME_DATA_INDEX, pendulum.datetime(1970, 1, 1))
        item.setData(TYPE_DATA_INDEX, FileType.Folder)
        item.setData(UUID_DATA_INDEX, uuid)
        self.setItem(row_idx, 2, item)
        item = CustomTableItem()
        item.setData(NAME_DATA_INDEX, pendulum.datetime(1970, 1, 1))
        item.setData(TYPE_DATA_INDEX, FileType.Folder)
        item.setData(UUID_DATA_INDEX, uuid)
        self.setItem(row_idx, 3, item)
        item = CustomTableItem()
        item.setData(NAME_DATA_INDEX, -1)
        item.setData(TYPE_DATA_INDEX, FileType.Folder)
        item.setData(UUID_DATA_INDEX, uuid)
        self.setItem(row_idx, 4, item)

    def add_file(self, file_name, uuid, file_size, created_on, updated_on, is_synced):
        row_idx = self.rowCount()
        self.insertRow(row_idx)
        item = FileTableItem(is_synced, file_name)
        item.setData(NAME_DATA_INDEX, 1)
        item.setData(TYPE_DATA_INDEX, FileType.File)
        item.setData(UUID_DATA_INDEX, uuid)
        self.setItem(row_idx, 0, item)
        item = CustomTableItem(file_name)
        item.setToolTip("\n".join(file_name[i : i + 64] for i in range(0, len(file_name), 64)))
        item.setData(NAME_DATA_INDEX, file_name)
        item.setData(TYPE_DATA_INDEX, FileType.File)
        item.setData(UUID_DATA_INDEX, uuid)
        self.setItem(row_idx, 1, item)
        item = CustomTableItem(format_datetime(created_on))
        item.setData(NAME_DATA_INDEX, created_on)
        item.setData(TYPE_DATA_INDEX, FileType.File)
        item.setData(UUID_DATA_INDEX, uuid)
        self.setItem(row_idx, 2, item)
        item = CustomTableItem(format_datetime(updated_on))
        item.setData(NAME_DATA_INDEX, updated_on)
        item.setData(TYPE_DATA_INDEX, FileType.File)
        item.setData(UUID_DATA_INDEX, uuid)
        self.setItem(row_idx, 3, item)
        item = CustomTableItem(get_filesize(file_size))
        item.setData(NAME_DATA_INDEX, file_size)
        item.setData(TYPE_DATA_INDEX, FileType.File)
        item.setData(UUID_DATA_INDEX, uuid)
        self.setItem(row_idx, 4, item)

    def add_inconsistency(self, file_name, uuid):
        inconsistency_color = QColor(255, 144, 155)
        row_idx = self.rowCount()
        self.insertRow(row_idx)
        item = InconsistencyTableItem(False)
        item.setData(NAME_DATA_INDEX, 1)
        item.setData(TYPE_DATA_INDEX, FileType.Inconsistency)
        item.setData(UUID_DATA_INDEX, uuid)
        item.setBackground(inconsistency_color)
        self.setItem(row_idx, 0, item)
        item = CustomTableItem(file_name)
        item.setToolTip("\n".join(file_name[i : i + 64] for i in range(0, len(file_name), 64)))
        item.setData(NAME_DATA_INDEX, file_name)
        item.setData(TYPE_DATA_INDEX, FileType.Inconsistency)
        item.setData(UUID_DATA_INDEX, uuid)
        item.setBackground(inconsistency_color)
        self.setItem(row_idx, 1, item)
        item = CustomTableItem()
        item.setData(NAME_DATA_INDEX, pendulum.datetime(1970, 1, 1))
        item.setData(TYPE_DATA_INDEX, FileType.Inconsistency)
        item.setBackground(inconsistency_color)
        item.setData(UUID_DATA_INDEX, uuid)
        self.setItem(row_idx, 2, item)
        item = CustomTableItem()
        item.setData(NAME_DATA_INDEX, pendulum.datetime(1970, 1, 1))
        item.setData(TYPE_DATA_INDEX, FileType.Inconsistency)
        item.setData(UUID_DATA_INDEX, uuid)
        item.setBackground(inconsistency_color)
        self.setItem(row_idx, 3, item)
        item = CustomTableItem(-1)
        item.setData(NAME_DATA_INDEX, -1)
        item.setData(TYPE_DATA_INDEX, FileType.Inconsistency)
        item.setData(UUID_DATA_INDEX, uuid)
        item.setBackground(inconsistency_color)
        self.setItem(row_idx, 4, item)

    def dragEnterEvent(self, event):
        event.accept()

    def dragMoveEvent(self, event):
        if event.mimeData().hasUrls():
            event.accept()
        else:
            if event.source() != self:
                event.ignore()
                return
            target_row = self.indexAt(event.pos()).row()
            item = self.item(target_row, 0)
            if not item:
                return
            file_type = item.data(TYPE_DATA_INDEX)
            if file_type == FileType.ParentFolder or file_type == FileType.Folder:
                event.accept()
            else:
                event.ignore()

    def dropEvent(self, event):
        if event.mimeData().hasUrls():
            event.accept()
            target_row = self.indexAt(event.pos()).row()
            target_item = self.item(target_row, 0)
            files = [pathlib.Path(url.toLocalFile()) for url in event.mimeData().urls()]
            if not target_item:
                self.files_dropped.emit(files, ".")
                return
            target_type = target_item.data(TYPE_DATA_INDEX)
            if target_type == FileType.File or target_type == FileType.ParentWorkspace:
                self.files_dropped.emit(files, ".")
            elif target_type == FileType.ParentFolder:
                self.files_dropped.emit(files, "..")
            elif target_type == FileType.Folder:
                self.files_dropped.emit(files, self.item(target_row, 1).text())
        else:
            if event.source() != self:
                return
            target_row = self.indexAt(event.pos()).row()
            rows = set([i.row for i in self.selected_files() if i.row != target_row])
            if not rows:
                return
            if not self.item(target_row, 0):
                return
            file_type = self.item(target_row, 0).data(TYPE_DATA_INDEX)
            target_name = self.item(target_row, 1).text()

            if file_type != FileType.ParentFolder and file_type != FileType.Folder:
                return
            for row in rows:
                file_name = self.item(row, 1).text()
                if file_type == FileType.ParentFolder:
                    self.file_moved.emit(file_name, "..")
                else:
                    self.file_moved.emit(file_name, target_name)
            for row in rows:
                self.removeRow(row)
            event.accept()
