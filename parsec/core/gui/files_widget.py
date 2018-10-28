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
from parsec.core.gui.ui.files_widget import Ui_FilesWidget
from parsec.core.gui.item_widget import FileItemWidget, FolderItemWidget, ParentItemWidget
from parsec.core.fs import FSManifestLocalMiss
from parsec.core.fs.sharing import SharingRecipientError


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
        self.list_files.itemSelectionChanged.connect(self.set_item_selected)
        self.list_files.itemDoubleClicked.connect(self.item_double_clicked)
        self.line_edit_search.textChanged.connect(self.filter_files)
        self.previously_selected = None
        self.current_directory = ""
        self.mountpoint = ""
        self.workspace = None
        # core_call().connect_event("fs.entry.updated", self._on_fs_entry_updated_trio)
        # core_call().connect_event("fs.entry.synced", self._on_fs_entry_synced_trio)
        # self.current_directory_changed_qt.connect(self._on_current_directory_changed_qt)

    def set_item_selected(self):
        if self.previously_selected:
            self.list_files.itemWidget(self.previously_selected).set_selected(False)
        self.previously_selected = self.list_files.currentItem()
        self.list_files.itemWidget(self.previously_selected).set_selected(True)

    def set_workspace(self, workspace):
        self.workspace = workspace
        self.load("")

    def load(self, directory):
        self.previously_selected = None
        self.list_files.clear()
        self.current_directory = directory
        if self.current_directory:
            self._add_parent_folder()
        dir_path = os.path.join("/", self.workspace, self.current_directory)
        self.label_current_directory.setText(dir_path)
        result = core_call().stat(dir_path)
        for child in result["children"]:
            attrs = core_call().stat(os.path.join(dir_path, child))
            if attrs["type"] == "folder":
                self._add_folder(child)
            elif attrs["type"] == "file":
                self._add_file(child, attrs["size"], attrs["created"], attrs["updated"])

    def _import_folder(self, path):
        pass

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
            return True
        except OSError as exc:
            return False
        finally:
            if fd_out:
                core_call().file_close(fd_out)

    def import_clicked(self):
        result, files = get_open_files(self)
        if not result:
            return
        errs = []
        for f in files:
            p = pathlib.Path(f)
            if p.is_dir():
                self._import_folder(p)
            elif p.is_file():
                if not self._import_file(
                    str(p), os.path.join("/", self.workspace, self.current_directory, p.name)
                ):
                    errs.append(str(p))
            else:
                print("Neither file nor dir, ignoring", f)
        self.load(self.current_directory)

    def filter_files(self, pattern):
        for i in range(self.list_files.count()):
            item = self.list_files.item(i)
            widget = self.list_files.itemWidget(item)
            if pattern not in widget.name:
                item.setHidden(True)
            else:
                item.setHidden(False)

    def _create_folder(self, folder_name):
        try:
            core_call().create_folder(
                os.path.join("/", self.workspace, self.current_directory, folder_name)
            )
            return True
        except FileExistError:
            show_warning(
                self,
                QCoreApplication.translate(
                    "FilesWidget", "A folder with the same name already exists."
                ),
            )
            return False

    def _add_parent_folder(self):
        item = QListWidgetItem()
        widget = ParentItemWidget()
        item.setSizeHint(widget.sizeHint())
        self.list_files.addItem(item)
        self.list_files.setItemWidget(item, widget)

    def _add_folder(self, folder_name):
        item = QListWidgetItem()
        widget = FolderItemWidget(folder_name, parent=self.list_files)
        item.setSizeHint(widget.sizeHint())
        self.list_files.addItem(item)
        self.list_files.setItemWidget(item, widget)

    def _add_file(self, file_name, file_size, created_on, updated_on):
        item = QListWidgetItem()
        widget = FileItemWidget(
            file_name, file_size, created_on, updated_on, parent=self.list_files
        )
        item.setSizeHint(widget.sizeHint())
        self.list_files.addItem(item)
        self.list_files.setItemWidget(item, widget)

    def create_folder_clicked(self):
        folder_name, ok = QInputDialog.getText(
            self,
            QCoreApplication.translate("FilesWidget", "Folder name"),
            QCoreApplication.translate("FilesWidget", "Enter new folder name"),
        )
        if not ok:
            return
        self._create_folder(folder_name)

    def show_context_menu(self, pos):
        pass

    def item_double_clicked(self, item):
        widget = self.list_files.itemWidget(item)
        if isinstance(widget, ParentItemWidget):
            self.load(os.path.dirname(self.current_directory))
        elif isinstance(widget, FileItemWidget):
            desktop.open_file(
                os.path.join(self.mountpoint, self.workspace, self.current_directory, widget.name)
            )
        elif isinstance(widget, FolderItemWidget):
            self.load(os.path.join(os.path.join(self.current_directory, widget.name)))

    def reset(self):
        self.workspace = ""
        self.current_directory = ""
        self.list_files.clear()

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
