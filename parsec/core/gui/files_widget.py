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
        self.list_files.currentItemChanged.connect(self.set_item_selected)
        self.list_files.itemDoubleClicked.connect(self.item_double_clicked)
        self.line_edit_search.textChanged.connect(self.filter_files)
        self.current_directory = ""
        self.mountpoint = ""
        self.workspace = None
        # core_call().connect_event("fs.entry.updated", self._on_fs_entry_updated_trio)
        # core_call().connect_event("fs.entry.synced", self._on_fs_entry_synced_trio)
        # self.current_directory_changed_qt.connect(self._on_current_directory_changed_qt)

    def set_item_selected(self, current, previous):
        if previous:
            self.list_files.itemWidget(previous).set_selected(False)
        if current:
            self.list_files.itemWidget(current).set_selected(True)

    def set_workspace(self, workspace):
        self.workspace = workspace
        self.load("")

    def load(self, directory):
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

    def _import_folder(self, src, dst):
        errs = []
        for f in src.iterdir():
            try:
                core_call().create_folder(dst)
                if f.is_dir():
                    errs.extend(self._import_folder(f, os.path.join(dst, f.name)))
                elif f.is_file():
                    errs.extend(self._import_file(f, os.path.join(dst, f.name)))
            except:
                errs.append(f)
        return errs

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
            return []
        except:
            return [src]
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
                errs.extend(
                    self._import_folder(
                        p, os.path.join("/", self.workspace, self.current_directory, p.name)
                    )
                )
            elif p.is_file():
                errs.extend(
                    self._import_file(
                        str(p), os.path.join("/", self.workspace, self.current_directory, p.name)
                    )
                )
            else:
                print("Neither file nor dir, ignoring", f)
        if errs:
            show_error(
                self,
                QCoreApplication.translate(
                    "FilesWidget", "Some files or folders could not be imported:\n{}"
                ).format("\n".join([str(e) for e in errs])),
            )
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
        except FileExistsError:
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
        widget = FolderItemWidget(item, folder_name, parent=self.list_files)
        item.setSizeHint(widget.sizeHint())
        self.list_files.addItem(item)
        self.list_files.setItemWidget(item, widget)
        widget.delete_clicked.connect(self.delete)

    def _add_file(self, file_name, file_size, created_on, updated_on):
        item = QListWidgetItem()
        widget = FileItemWidget(
            item, file_name, file_size, created_on, updated_on, parent=self.list_files
        )
        item.setSizeHint(widget.sizeHint())
        self.list_files.addItem(item)
        self.list_files.setItemWidget(item, widget)
        widget.delete_clicked.connect(self.delete)

    def delete(self, item):
        widget = self.list_files.itemWidget(item)
        result = QMessageBox.question(
            self,
            QCoreApplication.translate("FilesWidget", "Confirmation"),
            QCoreApplication.translate(
                "FilesWidget", 'Are you sure you want to delete "{}" ?'
            ).format(widget.name),
        )
        if result != QMessageBox.Yes:
            return
        path = os.path.join("/", self.workspace, self.current_directory, widget.name)
        try:
            if isinstance(widget, FolderItemWidget):
                self._delete_folder(path)
            else:
                core_call().delete_file(path)
            self.list_files.takeItem(self.list_files.row(item))
        except:
            show_error(self, QCoreApplication.translate('Can not delete "{}"').format(widget.name))

    def _delete_folder(self, path):
        result = core_call().stat(path)
        for child in result.get("children", []):
            child_path = os.path.join(path, child)
            file_infos = core_call().stat(os.path.join(path, child))
            if file_infos["type"] == "folder":
                self._delete_folder(child_path)
            else:
                core_call().delete_file(child_path)
        core_call().delete_folder(path)

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
        global_pos = self.list_files.mapToGlobal(pos)
        current_widget = self.list_files.itemWidget(self.list_files.currentItem())
        if isinstance(current_widget, ParentItemWidget):
            return
        menu = QMenu(self.list_files)
        action = menu.addAction(QCoreApplication.translate("FilesWidget", "Rename"))
        action.triggered.connect(self.rename(current_widget))
        if isinstance(current_widget, FileItemWidget):
            action = menu.addAction(QCoreApplication.translate("FilesWidget", "Open"))
        else:
            action = menu.addAction(QCoreApplication.translate("FilesWidget", "Open in explorer"))
        action.triggered.connect(self.open_file(current_widget))
        menu.exec_(global_pos)

    def rename(self, widget):
        def _inner_rename():
            new_name, ok = QInputDialog.getText(
                self,
                QCoreApplication.translate("FilesWidget", "New name"),
                QCoreApplication.translate("FilesWidget", "Enter file new name"),
            )
            if not ok:
                return
            if not new_name:
                show_warning(self, "This name is not valid.")
            try:
                if isinstance(widget, FolderItemWidget):
                    core_call().move_folder(
                        os.path.join("/", self.workspace, self.current_directory, widget.name),
                        os.path.join("/", self.workspace, self.current_directory, new_name),
                    )
                else:
                    core_call().move_file(
                        os.path.join("/", self.workspace, self.current_directory, widget.name),
                        os.path.join("/", self.workspace, self.current_directory, new_name),
                    )
                self.load(self.current_directory)
            except:
                show_error(self, QCoreApplication.translate("FilesWidget", "Can not rename."))

        return _inner_rename

    def open_file(self, widget):
        def _inner_open_file():
            desktop.open_file(
                os.path.join(self.mountpoint, self.workspace, self.current_directory, widget.name)
            )

        return _inner_open_file

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
