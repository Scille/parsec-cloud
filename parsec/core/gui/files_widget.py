import os
import pathlib
from enum import IntEnum
from uuid import UUID
import pendulum

from PyQt5.QtCore import Qt, QCoreApplication, pyqtSignal
from PyQt5.QtWidgets import (
    QWidget,
    QMenu,
    QStyledItemDelegate,
    QStyle,
    QFileDialog,
    QHeaderView,
    QTableWidgetItem,
    QStyleOptionViewItem,
)
from PyQt5.QtGui import QIcon

from parsec.core.gui import desktop
from parsec.core.gui.core_call import core_call
from parsec.core.gui.custom_widgets import show_error, ask_question, get_text
from parsec.core.gui.ui.files_widget import Ui_FilesWidget
from parsec.core.gui.file_size import get_filesize
from parsec.core.fs import FSEntryNotFound


class CustomTableItem(QTableWidgetItem):
    def __lt__(self, other):
        return (
            self.data(Qt.UserRole) is not None
            and other.data(Qt.UserRole) is not None
            and self.data(Qt.UserRole) < other.data(Qt.UserRole)
        )


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


class FileType(IntEnum):
    ParentWorkspace = 1
    ParentFolder = 2
    Folder = 3
    File = 4


class FilesWidget(QWidget, Ui_FilesWidget):
    fs_changed_qt = pyqtSignal(str, UUID, str)
    back_clicked = pyqtSignal()

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        h_header = self.table_files.horizontalHeader()
        h_header.setDefaultAlignment(Qt.AlignLeft | Qt.AlignVCenter)
        h_header.setSectionResizeMode(0, QHeaderView.Fixed)
        self.table_files.setColumnWidth(0, 60)
        h_header.setSectionResizeMode(1, QHeaderView.Stretch)
        h_header.setSectionResizeMode(2, QHeaderView.Fixed)
        self.table_files.setColumnWidth(2, 200)
        h_header.setSectionResizeMode(3, QHeaderView.Fixed)
        self.table_files.setColumnWidth(3, 200)
        h_header.setSectionResizeMode(4, QHeaderView.Fixed)
        self.table_files.setColumnWidth(4, 100)
        h_header.setSectionResizeMode(5, QHeaderView.Fixed)
        self.table_files.setColumnWidth(5, 60)
        v_header = self.table_files.verticalHeader()
        v_header.setSectionResizeMode(QHeaderView.Fixed)
        v_header.setDefaultSectionSize(48)
        self.table_files.setItemDelegate(ItemDelegate())
        self.button_back.clicked.connect(self.back_clicked)
        self.button_create_folder.clicked.connect(self.create_folder_clicked)
        self.button_import_files.clicked.connect(self.import_files_clicked)
        self.button_import_folder.clicked.connect(self.import_folder_clicked)
        self.table_files.setContextMenuPolicy(Qt.CustomContextMenu)
        self.table_files.customContextMenuRequested.connect(self.show_context_menu)
        self.table_files.itemSelectionChanged.connect(self.change_selection)
        self.table_files.cellClicked.connect(self.item_clicked)
        self.table_files.cellDoubleClicked.connect(self.item_double_clicked)
        self.line_edit_search.textChanged.connect(self.filter_files)
        self.current_directory = ""
        self.mountpoint = ""
        self.workspace = None
        core_call().connect_event("fs.entry.updated", self._on_fs_entry_updated_trio)
        core_call().connect_event("fs.entry.synced", self._on_fs_entry_synced_trio)
        self.fs_changed_qt.connect(self._on_fs_changed_qt)
        self.previous_selection = []

    def item_clicked(self, row, column):
        file_type = self.table_files.item(row, 0).data(Qt.UserRole)
        if (
            column == 5
            and file_type != FileType.ParentFolder
            and file_type != FileType.ParentWorkspace
        ):
            self.delete_item(row)

    def change_selection(self):
        selected = self.table_files.selectedItems()
        for item in self.previous_selection:
            if item.column() == 0:
                file_type = item.data(Qt.UserRole)
                if file_type == FileType.Folder:
                    item.setIcon(QIcon(":/icons/images/icons/folder.png"))
                elif file_type == FileType.File:
                    item.setIcon(QIcon(":/icons/images/icons/file.png"))
                else:
                    item.setIcon(QIcon(":/icons/images/icons/folder-up.png"))
            if item.column() == 5 and not item.icon().isNull():
                item.setIcon(QIcon(":/icons/images/icons/garbage.png"))
        for item in selected:
            if item.column() == 0:
                file_type = item.data(Qt.UserRole)
                if file_type == FileType.Folder:
                    item.setIcon(QIcon(":/icons/images/icons/folder_selected.png"))
                elif file_type == FileType.File:
                    item.setIcon(QIcon(":/icons/images/icons/file_selected.png"))
                else:
                    item.setIcon(QIcon(":/icons/images/icons/folder-up_selected.png"))
            if item.column() == 5 and not item.icon().isNull():
                item.setIcon(QIcon(":/icons/images/icons/garbage_selected.png"))
        self.previous_selection = selected

    def set_workspace(self, workspace):
        self.workspace = workspace
        self.label_current_workspace.setText(workspace)
        self.load("")
        self.table_files.sortItems(0)

    def load(self, directory):
        self.table_files.clearContents()
        self.table_files.setRowCount(0)
        old_sort = self.table_files.horizontalHeader().sortIndicatorSection()
        old_order = self.table_files.horizontalHeader().sortIndicatorOrder()
        self.table_files.setSortingEnabled(False)
        self.previous_selection = []
        self.current_directory = directory
        if self.current_directory:
            self._add_parent_folder()
        else:
            self._add_parent_workspace()
        if not self.current_directory:
            self.label_current_directory.hide()
            self.label_caret.hide()
        else:
            self.label_current_directory.setText(self.current_directory)
            self.label_current_directory.show()
            self.label_caret.show()
        dir_path = os.path.join("/", self.workspace, self.current_directory)
        dir_stat = core_call().stat(dir_path)
        for child in dir_stat["children"]:
            child_stat = core_call().stat(os.path.join(dir_path, child))
            if child_stat["is_folder"]:
                self._add_folder(child)
            else:
                self._add_file(
                    child, child_stat["size"], child_stat["created"], child_stat["updated"]
                )
        self.table_files.sortItems(old_sort, old_order)
        self.table_files.setSortingEnabled(True)
        if self.line_edit_search.text():
            self.filter_files(self.line_edit_search.text())

    def _import_folder(self, src, dst):
        err = False
        try:
            core_call().create_folder(dst)
            for f in src.iterdir():
                try:
                    if f.is_dir():
                        err |= self._import_folder(f, os.path.join(dst, f.name))
                    elif f.is_file():
                        err |= self._import_file(f, os.path.join(dst, f.name))
                except:
                    err = True
        except:
            err = True
        return err

    def _import_file(self, src, dst):
        fd_out = None
        try:
            core_call().file_create(dst)
            fd_out = core_call().file_open(dst)
            with open(src, "rb") as fd_in:
                while True:
                    chunk = fd_in.read(8192)
                    if not chunk:
                        break
                    core_call().file_write(fd_out, chunk)
            return False
        except:
            return True
        finally:
            if fd_out:
                core_call().file_close(fd_out)

    def import_files_clicked(self):
        paths, _ = QFileDialog.getOpenFileNames(
            self,
            QCoreApplication.translate("FilesWidget", "Select files to import"),
            str(pathlib.Path.home()),
        )
        if not paths:
            return
        for path in paths:
            p = pathlib.Path(path)
            err = self._import_file(
                str(p), os.path.join("/", self.workspace, self.current_directory, p.name)
            )
        if err:
            show_error(
                self, QCoreApplication.translate("FilesWidget", "Some files could not be imported.")
            )
        self.load(self.current_directory)

    def import_folder_clicked(self):
        path = QFileDialog.getExistingDirectory(
            self,
            QCoreApplication.translate("FilesWidget", "Select a directory to import"),
            str(pathlib.Path.home()),
        )
        if not path:
            return
        p = pathlib.Path(path)
        err = self._import_folder(
            p, os.path.join("/", self.workspace, self.current_directory, p.name)
        )
        if err:
            show_error(
                self, QCoreApplication.translate("FilesWidget", "The folder could not be imported.")
            )
        self.load(self.current_directory)

    def filter_files(self, pattern):
        pattern = pattern.lower()
        for i in range(self.table_files.rowCount()):
            file_type = self.table_files.item(i, 0).data(Qt.UserRole)
            name_item = self.table_files.item(i, 1)
            if file_type != FileType.ParentFolder and file_type != FileType.ParentWorkspace:
                if pattern not in name_item.text().lower():
                    self.table_files.setRowHidden(i, True)
                else:
                    self.table_files.setRowHidden(i, False)

    def _create_folder(self, folder_name):
        try:
            core_call().create_folder(
                os.path.join("/", self.workspace, self.current_directory, folder_name)
            )
            return True
        except FileExistsError:
            show_error(
                self,
                QCoreApplication.translate(
                    "FilesWidget", "A folder with the same name already exists."
                ),
            )
            return False

    def _add_parent_folder(self):
        row_idx = self.table_files.rowCount()
        self.table_files.insertRow(row_idx)
        item = CustomTableItem(QIcon(":/icons/images/icons/folder-up.png"), "")
        item.setData(Qt.UserRole, FileType.ParentFolder)
        self.table_files.setItem(row_idx, 0, item)
        item = QTableWidgetItem(QCoreApplication.translate("FilesWidget", "Parent Folder"))
        self.table_files.setItem(row_idx, 1, item)
        item = CustomTableItem()
        item.setData(Qt.UserRole, pendulum.datetime(1970, 1, 1))
        self.table_files.setItem(row_idx, 2, item)
        item = CustomTableItem()
        item.setData(Qt.UserRole, pendulum.datetime(1970, 1, 1))
        self.table_files.setItem(row_idx, 3, item)
        item = CustomTableItem()
        item.setData(Qt.UserRole, -2)
        self.table_files.setItem(row_idx, 4, item)
        item = QTableWidgetItem()
        self.table_files.setItem(row_idx, 5, item)

    def _add_parent_workspace(self):
        row_idx = self.table_files.rowCount()
        self.table_files.insertRow(row_idx)
        item = CustomTableItem(QIcon(":/icons/images/icons/folder-up.png"), "")
        item.setData(Qt.UserRole, FileType.ParentWorkspace)
        self.table_files.setItem(row_idx, 0, item)
        item = QTableWidgetItem(QCoreApplication.translate("FilesWidget", "Parent Workspace"))
        self.table_files.setItem(row_idx, 1, item)
        item = CustomTableItem()
        item.setData(Qt.UserRole, pendulum.datetime(1970, 1, 1))
        self.table_files.setItem(row_idx, 2, item)
        item = CustomTableItem()
        item.setData(Qt.UserRole, pendulum.datetime(1970, 1, 1))
        self.table_files.setItem(row_idx, 3, item)
        item = CustomTableItem()
        item.setData(Qt.UserRole, -2)
        self.table_files.setItem(row_idx, 4, item)
        item = QTableWidgetItem()
        self.table_files.setItem(row_idx, 5, item)

    def _add_folder(self, folder_name):
        row_idx = self.table_files.rowCount()
        self.table_files.insertRow(row_idx)
        item = CustomTableItem(QIcon(":/icons/images/icons/folder.png"), "")
        item.setData(Qt.UserRole, FileType.Folder)
        self.table_files.setItem(row_idx, 0, item)
        item = QTableWidgetItem(folder_name)
        self.table_files.setItem(row_idx, 1, item)
        item = CustomTableItem()
        item.setData(Qt.UserRole, pendulum.datetime(1971, 1, 1))
        self.table_files.setItem(row_idx, 2, item)
        item = CustomTableItem()
        item.setData(Qt.UserRole, pendulum.datetime(1971, 1, 1))
        self.table_files.setItem(row_idx, 3, item)
        item = CustomTableItem()
        item.setData(Qt.UserRole, -1)
        self.table_files.setItem(row_idx, 4, item)
        item = QTableWidgetItem(QIcon(":/icons/images/icons/garbage.png"), "")
        self.table_files.setItem(row_idx, 5, item)

    def _add_file(self, file_name, file_size, created_on, updated_on):
        row_idx = self.table_files.rowCount()
        self.table_files.insertRow(row_idx)
        item = CustomTableItem(QIcon(":/icons/images/icons/file.png"), "")
        item.setData(Qt.UserRole, FileType.File)
        self.table_files.setItem(row_idx, 0, item)
        item = QTableWidgetItem(file_name)
        self.table_files.setItem(row_idx, 1, item)
        item = CustomTableItem(created_on.format("%x %X"))
        item.setData(Qt.UserRole, created_on)
        self.table_files.setItem(row_idx, 2, item)
        item = CustomTableItem(updated_on.format("%x %X"))
        item.setData(Qt.UserRole, updated_on)
        self.table_files.setItem(row_idx, 3, item)
        item = CustomTableItem(get_filesize(file_size))
        item.setData(Qt.UserRole, file_size)
        self.table_files.setItem(row_idx, 4, item)
        item = QTableWidgetItem(QIcon(":/icons/images/icons/garbage.png"), "")
        self.table_files.setItem(row_idx, 5, item)

    def delete_item(self, row):
        name_item = self.table_files.item(row, 1)
        type_item = self.table_files.item(row, 0)
        QCoreApplication.processEvents()
        result = ask_question(
            self,
            QCoreApplication.translate("FilesWidget", "Confirmation"),
            QCoreApplication.translate(
                "FilesWidget", 'Are you sure you want to delete "{}" ?'
            ).format(name_item.text()),
        )
        if not result:
            return
        path = os.path.join("/", self.workspace, self.current_directory, name_item.text())
        try:
            if type_item.data(Qt.UserRole) == FileType.Folder:
                self._delete_folder(path)
            else:
                core_call().delete_file(path)
            self.table_files.removeRow(row)
        except:
            show_error(
                self, QCoreApplication.translate('Can not delete "{}"').format(name_item.text())
            )

    def _delete_folder(self, path):
        result = core_call().stat(path)
        for child in result.get("children", []):
            child_path = os.path.join(path, child)
            file_infos = core_call().stat(os.path.join(path, child))
            if file_infos["is_folder"]:
                self._delete_folder(child_path)
            else:
                core_call().delete_file(child_path)
        core_call().delete_folder(path)

    def create_folder_clicked(self):
        folder_name = get_text(
            self,
            QCoreApplication.translate("FilesWidget", "Folder name"),
            QCoreApplication.translate("FilesWidget", "Enter new folder name"),
            placeholder="Name",
        )
        if not folder_name:
            return
        self._create_folder(folder_name)

    def show_context_menu(self, pos):
        global_pos = self.table_files.mapToGlobal(pos)
        row = self.table_files.currentRow()
        name_item = self.table_files.item(row, 1)
        type_item = self.table_files.item(row, 0)
        file_type = type_item.data(Qt.UserRole)
        if file_type == FileType.ParentFolder or file_type == FileType.ParentWorkspace:
            return
        menu = QMenu(self.table_files)
        if file_type == FileType.File:
            action = menu.addAction(QCoreApplication.translate("FilesWidget", "Open"))
        else:
            action = menu.addAction(QCoreApplication.translate("FilesWidget", "Open in explorer"))
        action.triggered.connect(self.open_file(name_item.text()))
        action = menu.addAction(QCoreApplication.translate("FilesWidget", "Rename"))
        action.triggered.connect(self.rename(name_item, type_item))
        menu.exec_(global_pos)

    def rename(self, name_item, type_item):
        def _inner_rename():
            new_name = get_text(
                self,
                QCoreApplication.translate("FilesWidget", "New name"),
                QCoreApplication.translate("FilesWidget", "Enter file new name"),
                placeholder="File name",
                default_text=name_item.text(),
            )
            if not new_name:
                return
            try:
                if type_item.data(Qt.UserRole) == FileType.Folder:
                    core_call().move_folder(
                        os.path.join("/", self.workspace, self.current_directory, name_item.text()),
                        os.path.join("/", self.workspace, self.current_directory, new_name),
                    )
                else:
                    core_call().move_file(
                        os.path.join("/", self.workspace, self.current_directory, name_item.text()),
                        os.path.join("/", self.workspace, self.current_directory, new_name),
                    )
                self.load(self.current_directory)
            except:
                show_error(self, QCoreApplication.translate("FilesWidget", "Can not rename."))

        return _inner_rename

    def open_file(self, file_name):
        def _inner_open_file():
            desktop.open_file(
                os.path.join(self.mountpoint, self.workspace, self.current_directory, file_name)
            )

        return _inner_open_file

    def item_double_clicked(self, row, column):
        name_item = self.table_files.item(row, 1)
        type_item = self.table_files.item(row, 0)
        file_type = type_item.data(Qt.UserRole)
        if file_type == FileType.ParentFolder:
            self.load(os.path.dirname(self.current_directory))
        elif file_type == FileType.ParentWorkspace:
            self.back_clicked.emit()
        elif file_type == FileType.File:
            desktop.open_file(
                os.path.join(
                    self.mountpoint, self.workspace, self.current_directory, name_item.text()
                )
            )
        elif file_type == FileType.Folder:
            self.load(os.path.join(os.path.join(self.current_directory, name_item.text())))

    def reset(self):
        self.workspace = ""
        self.current_directory = ""
        self.previous_selection = []
        self.table_files.clearContents()
        self.table_files.setRowCount(0)

    def _on_fs_entry_synced_trio(self, event, path, id):
        self.fs_changed_qt.emit(event, id, path)

    def _on_fs_entry_updated_trio(self, event, id):
        self.fs_changed_qt.emit(event, id, None)

    def _on_fs_changed_qt(self, event, id, path):
        if not path:
            try:
                path = core_call().get_entry_path(id)
            except FSEntryNotFound:
                # Entry not locally present, nothing to do
                return

        # Modifications on root is handled by workspace_widget
        if path == "/":
            return

        # TODO: too cumbersome...
        if isinstance(path, pathlib.PurePosixPath):
            modified_hops = [x for x in path.parts if x != "/"]
        else:
            modified_hops = [x for x in path.split("/") if x and x != "/"]

        if self.workspace and self.current_directory:
            current_dir_hops = [self.workspace] + [
                x for x in self.current_directory.split("/") if x
            ]
        else:
            current_dir_hops = []
        # Only direct children to current directory require reloading
        if modified_hops == current_dir_hops or modified_hops[:-1] == current_dir_hops:
            self.load(self.current_directory)
