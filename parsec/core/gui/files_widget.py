import os
import pathlib
from uuid import UUID
from pathlib import PurePosixPath

from PyQt5.QtCore import Qt, QSize, QCoreApplication, QDir, pyqtSignal, QPoint
from PyQt5.QtGui import QIcon, QPixmap
from PyQt5.QtWidgets import (
    QWidget,
    QListWidgetItem,
    QMenu,
    QMessageBox,
    QInputDialog,
    QFileDialog,
    QHeaderView,
    QTableWidgetItem,
)

from parsec.core.gui import desktop
from parsec.core.gui.core_call import core_call
from parsec.core.gui.file_size import get_filesize
from parsec.core.gui.custom_widgets import show_error, show_warning, get_open_files
from parsec.core.gui.ui.parent_folder_widget import Ui_ParentFolderWidget
from parsec.core.gui.ui.files_widget import Ui_FilesWidget
from parsec.core.gui.ui.file_item_widget import Ui_FileItemWidget
from parsec.core.fs import FSManifestLocalMiss
from parsec.core.fs.sharing import SharingRecipientError


class ParentFolderWidget(QWidget, Ui_ParentFolderWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.label_icon.setPixmap(QPixmap(":/icons/images/icons/folder_parent.png"))


class FilesWidget(QWidget, Ui_FilesWidget):
    current_directory_changed_qt = pyqtSignal(str, UUID, str)
    back_clicked = pyqtSignal()

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.button_back.clicked.connect(self.back_clicked)
        self.button_create_folder.clicked.connect(self.create_folder_clicked)
        self.button_import.clicked.connect(self.import_clicked)
        self.list_files.setContextMenuPolicy(Qt.CustomContextMenu)
        self.list_files.customContextMenuRequested.connect(self.show_context_menu)
        self.list_files.itemDoubleClicked.connect(self.item_double_clicked)
        self.line_edit_search.textChanged.connect(self.filter_files)
        self.current_directory = None
        self.workspace = None
        # core_call().connect_event("fs.entry.updated", self._on_fs_entry_updated_trio)
        # core_call().connect_event("fs.entry.synced", self._on_fs_entry_synced_trio)
        # self.current_directory_changed_qt.connect(self._on_current_directory_changed_qt)

    def set_workspace(self, workspace):
        self.workspace = workspace
        self.label_current_directory.setText(os.path.join("/", self.workspace))

    def load(self):
        dir_path = os.path.join("/", self.workspace, self.current_directory)
        result = core_call.stat(dir_path)
        for child in result["children"]:
            print(child)

    def import_clicked(self):
        result, files = get_open_files(self)
        if not result:
            return
        for f in files:
            p = pathlib.Path(f)
            if p.is_dir():
                print("Importing directory", f)
            elif p.is_file():
                print("Importing file", f)
            else:
                print("Neither file nor dir, ignoring", f)

    def filter_files(self, pattern):
        pass

    def _create_folder(self, folder_name):
        try:
            core_call().create_folder(
                os.path.join("/", self.workspace, self.current_directory or "", folder_name)
            )
            return True
        except:
            import traceback

            traceback.print_exc()
            return False

    def _add_folder(self, folder_name):
        self.table_files.insertRow(self.table_files.rowCount())
        item = QTableWidgetItem(folder_name)
        item.setData(Qt.UserRole, "folder")
        row_idx = self.table_files.rowCount() - 1
        self.table_files.setItem(row_idx, 1, item)
        self.table_files.setItem(
            row_idx, 0, QTableWidgetItem(QIcon(":/icons/images/icons/folder.png"), "")
        )
        self.table_files.setRowHeight(row_idx, 32)

    def create_folder_clicked(self):
        folder_name, ok = QInputDialog.getText(
            self,
            QCoreApplication.translate("FilesWidget", "Folder name"),
            QCoreApplication.translate("FilesWidget", "Enter new folder name"),
        )
        if not ok:
            return
        if not self._create_folder(folder_name):
            show_error(
                self, QCoreApplication.translate("FilesWidget", "Can not create the new folder.")
            )
        else:
            self._add_folder(folder_name)

    def show_context_menu(self, pos):
        pass

    def item_double_clicked(self, item):
        item = self.table_files.item(item.row(), 1)
        if item.data(Qt.UserRole) == "file":
            desktop.open_file(
                os.path.join("/", self.workspace, self.current_directory, item.text())
            )

    def reset(self):
        self.current_directory = None

    # def _on_fs_entry_synced_trio(self, event, path, id):
    #     self.current_directory_changed_qt.emit(event, id, path)

    # def _on_fs_entry_updated_trio(self, event, id):
    #     self.current_directory_changed_qt.emit(event, id, None)

    # def _on_current_directory_changed_qt(self, event, id, path):
    #     if not path:
    #         path = core_call().get_entry_path(id)
    #     # TODO: too cumbersome...
    #     if isinstance(path, PurePosixPath):
    #         modified_hops = list(path.parts)
    #     else:
    #         modified_hops = [x for x in path.split("/") if x]

    #     if self.current_workspace is not None or self.current_directory is not None:
    #         current_dir_hops = ["/", self.current_workspace] + [
    #             x for x in self.current_directory.split("/") if x
    #         ]
    #     else:
    #         current_dir_hops = []
    #     # Only direct children to current directory require reloading
    #     if modified_hops == current_dir_hops or modified_hops[:-1] == current_dir_hops:
    #         self.reload_current_directory()

    # def delete_all_subs(self, dir_path):
    #     result = core_call().stat(dir_path)
    #     for child in result.get("children", []):
    #         file_infos = core_call().stat(os.path.join(dir_path, child))
    #         if file_infos["type"] == "folder":
    #             self.delete_all_subs(os.path.join(dir_path, child))
    #         else:
    #             core_call().delete_file(os.path.join(dir_path, child))
    #     core_call().delete_folder(dir_path)
