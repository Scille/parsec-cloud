# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import time
import os
import pathlib
from uuid import UUID

from PyQt5.QtCore import Qt, QCoreApplication, pyqtSignal, QTimer
from PyQt5.QtWidgets import QMenu, QFileDialog, QApplication, QDialog

from parsec.core.types import FsPath
from parsec.core.gui import desktop
from parsec.core.gui.file_items import FileType
from parsec.core.gui.custom_widgets import show_error, ask_question, get_text, TaskbarButton
from parsec.core.gui.core_widget import CoreWidget
from parsec.core.gui.loading_dialog import LoadingDialog
from parsec.core.gui.replace_dialog import ReplaceDialog
from parsec.core.gui.ui.files_widget import Ui_FilesWidget
from parsec.core.fs import FSEntryNotFound


class CancelException(Exception):
    pass


class FilesWidget(CoreWidget, Ui_FilesWidget):
    fs_updated_qt = pyqtSignal(str, UUID)
    fs_synced_qt = pyqtSignal(str, UUID, str)
    back_clicked = pyqtSignal()

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.taskbar_buttons = []
        button_back = TaskbarButton(icon_path=":/icons/images/icons/return_off.png")
        button_back.clicked.connect(self.back_clicked)
        self.taskbar_buttons.append(button_back)
        button_import_folder = TaskbarButton(icon_path=":/icons/images/icons/upload_folder_off.png")
        button_import_folder.clicked.connect(self.import_folder_clicked)
        self.taskbar_buttons.append(button_import_folder)
        button_import_files = TaskbarButton(icon_path=":/icons/images/icons/upload_file_off.png")
        button_import_files.clicked.connect(self.import_files_clicked)
        self.taskbar_buttons.append(button_import_files)
        button_create_folder = TaskbarButton(icon_path=":/icons/images/icons/plus_off.png")
        button_create_folder.clicked.connect(self.create_folder_clicked)
        self.taskbar_buttons.append(button_create_folder)
        self.table_files.customContextMenuRequested.connect(self.show_context_menu)
        self.table_files.cellDoubleClicked.connect(self.item_double_clicked)
        self.line_edit_search.textChanged.connect(self.filter_files)
        self.current_directory = ""
        self.workspace = None
        self.fs_updated_qt.connect(self._on_fs_updated_qt)
        self.fs_synced_qt.connect(self._on_fs_synced_qt)
        self.update_timer = QTimer()
        self.update_timer.timeout.connect(self.reload)
        self.table_files.file_moved.connect(self.on_file_moved)
        self.table_files.init()

    def get_taskbar_buttons(self):
        return self.taskbar_buttons

    @CoreWidget.core.setter
    def core(self, c):
        if self._core:
            self._core.fs.event_bus.disconnect("fs.entry.updated", self._on_fs_entry_updated_trio)
            self._core.fs.event_bus.disconnect("fs.entry.synced", self._on_fs_entry_synced_trio)
        self._core = c
        if self._core:
            self._core.fs.event_bus.connect("fs.entry.updated", self._on_fs_entry_updated_trio)
            self._core.fs.event_bus.connect("fs.entry.synced", self._on_fs_entry_synced_trio)

    def set_workspace(self, workspace):
        self.workspace = workspace
        self.label_current_workspace.setText(workspace)
        self.load("")
        self.table_files.sortItems(0)

    def reload(self):
        self.load(self.current_directory)

    def load(self, directory):
        if not self.workspace and not directory:
            return

        self.update_timer.stop()

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
        for i, child in enumerate(dir_stat["children"]):
            child_stat = self.portal.run(self.core.fs.stat, os.path.join(dir_path, child))
            if child_stat["is_folder"]:
                self.table_files.add_folder(child, not child_stat["need_sync"])
            else:
                self.table_files.add_file(
                    child,
                    child_stat["size"],
                    child_stat["created"],
                    child_stat["updated"],
                    not child_stat["need_sync"],
                )
            if i % 5 == 0:
                QApplication.processEvents()
        self.table_files.sortItems(old_sort, old_order)
        self.table_files.setSortingEnabled(True)
        if self.line_edit_search.text():
            self.filter_files(self.line_edit_search.text())

    def import_all(self, files, total_size):
        loading_dialog = LoadingDialog(total_size=total_size + len(files), parent=self)
        loading_dialog.show()
        QApplication.processEvents()
        current_size = 0
        start_time = time.time()
        skip_all = False
        replace_all = False
        for src, dst in files:
            file_exists = False
            try:
                self.portal.run(self.core.fs.stat, dst)
                file_exists = True
            except FileNotFoundError:
                file_exists = False
            if file_exists:
                replace = False or replace_all
                skip = False or skip_all
                if not replace and not skip:
                    replace_dialog = ReplaceDialog(dst, parent=self)
                    if replace_dialog.exec_() == QDialog.Rejected:
                        return
                    else:
                        if replace_dialog.skip:
                            skip = True
                            if replace_dialog.all_files:
                                skip_all = True
                        elif replace_dialog.replace:
                            replace = True
                            if replace_dialog.all_files:
                                replace_all = True
                if replace:
                    self.import_file(src, dst, current_size, loading_dialog)
                elif skip:
                    continue
            else:
                self.import_file(src, dst, current_size, loading_dialog)
            if not loading_dialog.is_cancelled:
                current_size += src.stat().st_size + 1
                loading_dialog.set_progress(current_size)
            else:
                break
        elapsed = time.time() - start_time
        # Done for ergonomy. We don't want a window just flashing before the user, so we
        # add this little trick. The window will be opened at least 0.5s, which is more than
        # enough for the user to realize that it is a progress bar and not just a bug.
        if elapsed < 0.5:
            time.sleep(1.0 - elapsed)
        loading_dialog.hide()
        loading_dialog.setParent(None)
        QApplication.processEvents()

    def get_files(self, paths):
        files = []
        total_size = 0
        for path in paths:
            p = pathlib.Path(path)
            dst = os.path.join("/", self.workspace, self.current_directory, p.name)
            files.append((p, dst))
            total_size += p.stat().st_size
        return files, total_size

    def get_folder(self, src, dst):
        files = []
        total_size = 0
        try:
            self.portal.run(self.core.fs.folder_create, dst)
        except FileExistsError:
            pass
        for f in src.iterdir():
            if f.is_dir():
                new_files, new_size = self.get_folder(f, os.path.join(dst, f.name))
                files.extend(new_files)
                total_size += new_size
            elif f.is_file():
                new_dst = os.path.join(dst, f.name)
                files.append((f, new_dst))
                total_size += f.stat().st_size
        return files, total_size

    def import_file(self, src, dst, current_size, loading_dialog):
        loading_dialog.set_current_file(src.name)
        fd_out = None
        try:
            try:
                self.portal.run(self.core.fs.file_create, dst)
            except FileExistsError:
                pass
            fd_out = self.portal.run(self.core.fs.file_fd_open, dst)
            with open(src, "rb") as fd_in:
                i = 0
                read_size = 0
                while True:
                    chunk = fd_in.read(65536)
                    if not chunk:
                        break
                    self.portal.run(self.core.fs.file_fd_write, fd_out, chunk)
                    read_size += len(chunk)
                    i += 1
                    if i % 5 == 0:
                        if loading_dialog.is_cancelled:
                            raise CancelException()
                        loading_dialog.set_progress(current_size + read_size)
                        QApplication.processEvents()
        except CancelException:
            loading_dialog.set_cancel_state()
            QApplication.processEvents()
        except:
            import traceback

            traceback.print_exc()
        finally:
            if fd_out:
                self.portal.run(self.core.fs.file_fd_close, fd_out)
            if loading_dialog.is_cancelled:
                self.portal.run(self.core.fs.delete, dst)

    # slot
    def import_files_clicked(self):
        paths, _ = QFileDialog.getOpenFileNames(
            self,
            QCoreApplication.translate("FilesWidget", "Select files to import"),
            str(pathlib.Path.home()),
        )
        if not paths:
            return
        files, total_size = self.get_files(paths)
        self.import_all(files, total_size)

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
        files, total_size = self.get_folder(
            p, os.path.join("/", self.workspace, self.current_directory, p.name)
        )
        self.import_all(files, total_size)

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
            if not name_item or not type_item:
                return
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
    def on_file_moved(self, src, dst):
        src_path = os.path.join("/", self.workspace, self.current_directory, src)
        dst_path = ""
        if dst == "..":
            target_dir = pathlib.Path(
                os.path.join("/", self.workspace, self.current_directory)
            ).parent
            dst_path = os.path.join("/", self.workspace, target_dir, src)
        else:
            dst_path = os.path.join("/", self.workspace, dst, src)
        self.portal.run(self.core.fs.move, src_path, dst_path)

    # slot
    def show_context_menu(self, pos):
        global_pos = self.table_files.mapToGlobal(pos)
        row = self.table_files.currentRow()
        name_item = self.table_files.item(row, 1)
        type_item = self.table_files.item(row, 0)
        if not name_item or not type_item:
            return
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
            path = FsPath("/") / self.workspace / self.current_directory / file_name
            desktop.open_file(str(self.core.mountpoint_manager.get_path_in_mountpoint(path)))

        return _inner_open_file

    # slot
    def item_double_clicked(self, row, column):
        name_item = self.table_files.item(row, 1)
        type_item = self.table_files.item(row, 0)
        file_type = type_item.data(Qt.UserRole)
        try:
            if file_type == FileType.ParentFolder:
                self.load(os.path.dirname(self.current_directory))
            elif file_type == FileType.ParentWorkspace:
                self.back_clicked.emit()
            elif file_type == FileType.File:
                self.open_file(name_item.text())()
            elif file_type == FileType.Folder:
                self.load(os.path.join(os.path.join(self.current_directory, name_item.text())))
        except AttributeError:
            # This can happen when updating the list: double click event gets processed after
            # the item has been removed.
            pass

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
        self.fs_synced_qt.emit(event, id, path)

    # slot
    def _on_fs_entry_updated_trio(self, event, id):
        self.fs_updated_qt.emit(event, id)

    # slot
    def _on_fs_synced_qt(self, event, id, path):
        # if path == "/":
        #     return
        # hops = [x for x in path.split("/")[2:] if x]
        # if self.current_directory:
        #     current_hops = [
        #         x for x in self.current_directory.split("/") if x
        #     ]
        # else:
        #     current_hops = []
        pass

    # slot
    def _on_fs_updated_qt(self, event, id):
        path = None
        try:
            path = self.portal.run(self.core.fs.get_entry_path, id)
        except FSEntryNotFound:
            # Entry not locally present, nothing to do
            return

        # Modifications on root is handled by workspace_widget
        if path == "/":
            return

        modified_hops = [x for x in path.parts if x != "/"]

        if self.workspace and self.current_directory:
            current_dir_hops = [self.workspace] + [
                x for x in self.current_directory.split("/") if x
            ]
        else:
            current_dir_hops = []
        # Only direct children to current directory require reloading
        if modified_hops == current_dir_hops or modified_hops[:-1] == current_dir_hops:
            self.update_timer.start(1000)
