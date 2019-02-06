import os
import queue
import threading
import pathlib
from uuid import UUID

from PyQt5.QtCore import Qt, QCoreApplication, pyqtSignal
from PyQt5.QtWidgets import QMenu, QFileDialog

from parsec.core.gui import desktop
from parsec.core.gui.file_items import FileType
from parsec.core.gui.custom_widgets import show_error, ask_question, get_text, TaskbarButton
from parsec.core.gui.core_widget import CoreWidget
from parsec.core.gui.ui.files_widget import Ui_FilesWidget
from parsec.core.fs import FSEntryNotFound


class FilesWidget(CoreWidget, Ui_FilesWidget):
    fs_changed_qt = pyqtSignal(str, UUID, str)
    back_clicked = pyqtSignal()

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.taskbar_buttons = []
        button_back = TaskbarButton(icon_path=":/icons/images/icons/go-back_button.png")
        button_back.clicked.connect(self.back_clicked)
        self.taskbar_buttons.append(button_back)
        button_import_folder = TaskbarButton(icon_path=":/icons/images/icons/import_folder.png")
        button_import_folder.clicked.connect(self.import_folder_clicked)
        self.taskbar_buttons.append(button_import_folder)
        button_import_files = TaskbarButton(icon_path=":/icons/images/icons/import_file.png")
        button_import_files.clicked.connect(self.import_files_clicked)
        self.taskbar_buttons.append(button_import_files)
        button_create_folder = TaskbarButton(icon_path=":/icons/images/icons/add-plus-button.png")
        button_create_folder.clicked.connect(self.create_folder_clicked)
        self.taskbar_buttons.append(button_create_folder)
        self.table_files.customContextMenuRequested.connect(self.show_context_menu)
        self.table_files.cellDoubleClicked.connect(self.item_double_clicked)
        self.line_edit_search.textChanged.connect(self.filter_files)
        self.current_directory = ""
        self.workspace = None
        self.fs_changed_qt.connect(self._on_fs_changed_qt)
        self.file_queue = queue.Queue(1024)
        self.table_files.init()

    def get_taskbar_buttons(self):
        return self.taskbar_buttons

    @CoreWidget.core.setter
    def core(self, c):
        if self._core:
            self._core.fs.event_bus.disconnect("fs.entry.updated", self._on_fs_entry_updated_trio)
            self._core.fs.event_bus.disconnect("fs.entry.synced", self._on_fs_entry_synced_trio)
            self.stop()
        self._core = c
        if self._core:
            self._core.fs.event_bus.connect("fs.entry.updated", self._on_fs_entry_updated_trio)
            self._core.fs.event_bus.connect("fs.entry.synced", self._on_fs_entry_synced_trio)
            self.start()

    def start(self):
        self.import_thread = threading.Thread(target=self._import_files)
        self.import_thread.start()

    def stop(self):
        self.file_queue.put_nowait((None, None))
        self.import_thread.join()

    def _import_files(self):
        while True:
            src, dst = self.file_queue.get()
            if src is None or dst is None:
                return
            self._import_file(src, dst)

    def set_workspace(self, workspace):
        self.workspace = workspace
        self.label_current_workspace.setText(workspace)
        self.load("")
        self.table_files.sortItems(0)

    def load(self, directory):
        if not self.workspace and not directory:
            return

        self.table_files.clear()
        old_sort = self.table_files.horizontalHeader().sortIndicatorSection()
        old_order = self.table_files.horizontalHeader().sortIndicatorOrder()
        self.table_files.setSortingEnabled(False)
        self.current_directory = directory
        if self.current_directory:
            self.table_files.add_parent_folder()
        else:
            self.table_files.add_parent_workspace()
        if not self.current_directory:
            self.label_current_directory.hide()
            self.label_caret.hide()
        else:
            self.label_current_directory.setText(self.current_directory)
            self.label_current_directory.show()
            self.label_caret.show()
        dir_path = os.path.join("/", self.workspace, self.current_directory)
        dir_stat = self.portal.run(self.core.fs.stat, dir_path)
        for child in dir_stat["children"]:
            child_stat = self.portal.run(self.core.fs.stat, os.path.join(dir_path, child))
            if child_stat["is_folder"]:
                self.table_files.add_folder(child)
            else:
                self.table_files.add_file(
                    child, child_stat["size"], child_stat["created"], child_stat["updated"]
                )
        self.table_files.sortItems(old_sort, old_order)
        self.table_files.setSortingEnabled(True)
        if self.line_edit_search.text():
            self.filter_files(self.line_edit_search.text())

    def _import_folder(self, src, dst):
        err = False
        try:
            self.portal.run(self.core.fs.folder_create, dst)
            for f in src.iterdir():
                try:
                    if f.is_dir():
                        err |= self._import_folder(f, os.path.join(dst, f.name))
                    elif f.is_file():
                        try:
                            self.file_queue.put_nowait((f, os.path.join(dst, f.name)))
                        except queue.Full:
                            err = True
                except:
                    err = True
        except:
            err = True
        return err

    def _import_file(self, src, dst):
        fd_out = None
        try:
            self.portal.run(self.core.fs.file_create, dst)
            fd_out = self.portal.run(self.core.fs.file_fd_open, dst)
            with open(src, "rb") as fd_in:
                while True:
                    chunk = fd_in.read(8192)
                    if not chunk:
                        break
                    self.portal.run(self.core.fs.file_fd_write, fd_out, chunk)
            return False
        except:
            return True
        finally:
            if fd_out:
                self.portal.run(self.core.fs.file_fd_close, fd_out)

    # slot
    def import_files_clicked(self):
        paths, _ = QFileDialog.getOpenFileNames(
            self,
            QCoreApplication.translate("FilesWidget", "Select files to import"),
            str(pathlib.Path.home()),
        )
        if not paths:
            return
        err = False
        for path in paths:
            p = pathlib.Path(path)
            try:
                self.file_queue.put_nowait(
                    (str(p), os.path.join("/", self.workspace, self.current_directory, p.name))
                )
            except queue.Full:
                err = True
        if err:
            show_error(
                self, QCoreApplication.translate("FilesWidget", "Some files could not be imported.")
            )

    # slot
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

    # slot
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
            self.portal.run(
                self.core.fs.folder_create,
                os.path.join("/", self.workspace, self.current_directory, folder_name),
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

    def delete_item(self, row):
        def _inner_delete_item():
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
                    self.portal.run(self.core.fs.delete, path)
                self.table_files.removeRow(row)
            except:
                show_error(
                    self, QCoreApplication.translate('Can not delete "{}"').format(name_item.text())
                )

        return _inner_delete_item

    def _delete_folder(self, path):
        result = self.portal.run(self.core.fs.stat, path)
        for child in result.get("children", []):
            child_path = os.path.join(path, child)
            file_infos = self.portal.run(self.core.fs.stat, os.path.join(path, child))
            if file_infos["is_folder"]:
                self._delete_folder(child_path)
            else:
                self.portal.run(self.core.fs.delete, child_path)
        self.portal.run(self.core.fs.delete, path)

    # slot
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

    # slot
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
        action = menu.addAction(QCoreApplication.translate("FilesWidget", "Delete"))
        action.triggered.connect(self.delete_item(row))
        menu.exec_(global_pos)

    # slot
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
                    self.portal.run(
                        self.core.fs.move,
                        os.path.join("/", self.workspace, self.current_directory, name_item.text()),
                        os.path.join("/", self.workspace, self.current_directory, new_name),
                    )
                else:
                    self.portal.run(
                        self.core.fs.move,
                        os.path.join("/", self.workspace, self.current_directory, name_item.text()),
                        os.path.join("/", self.workspace, self.current_directory, new_name),
                    )
            except:
                show_error(self, QCoreApplication.translate("FilesWidget", "Can not rename."))

        return _inner_rename

    # slot
    def open_file(self, file_name):
        def _inner_open_file():
            desktop.open_file(
                os.path.join(
                    self.core.mountpoint, self.workspace, self.current_directory, file_name
                )
            )

        return _inner_open_file

    # slot
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
                    self.core.mountpoint, self.workspace, self.current_directory, name_item.text()
                )
            )
        elif file_type == FileType.Folder:
            self.load(os.path.join(os.path.join(self.current_directory, name_item.text())))

    def keyPressEvent(self, event):
        if event.key() == Qt.Key_Delete:
            row = self.table_files.currentRow()
            self.delete_item(row)()

    def reset(self):
        self.workspace = ""
        self.current_directory = ""
        self.table_files.clear()

    # slot
    def _on_fs_entry_synced_trio(self, event, path, id):
        self.fs_changed_qt.emit(event, id, path)

    # slot
    def _on_fs_entry_updated_trio(self, event, id):
        self.fs_changed_qt.emit(event, id, None)

    # slot
    def _on_fs_changed_qt(self, event, id, path):
        if not path:
            try:
                path = self.portal.run(self.core.fs.get_entry_path, id)
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
