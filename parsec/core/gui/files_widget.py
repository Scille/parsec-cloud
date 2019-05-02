# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import time
import pathlib
from uuid import UUID

from PyQt5.QtCore import Qt, pyqtSignal, QTimer
from PyQt5.QtWidgets import QFileDialog, QApplication, QDialog, QWidget

from parsec.core.types import FsPath, AccessID
from parsec.core.gui import desktop
from parsec.core.gui.file_items import FileType, TYPE_DATA_INDEX
from parsec.core.gui.custom_widgets import show_error, ask_question, get_text, TaskbarButton
from parsec.core.gui.loading_dialog import LoadingDialog
from parsec.core.gui.lang import translate as _
from parsec.core.gui.replace_dialog import ReplaceDialog
from parsec.core.gui.ui.files_widget import Ui_FilesWidget
from parsec.core.fs import FSEntryNotFound


class CancelException(Exception):
    pass


class FilesWidget(QWidget, Ui_FilesWidget):
    fs_updated_qt = pyqtSignal(str, UUID)
    fs_synced_qt = pyqtSignal(str, UUID, str)
    back_clicked = pyqtSignal()

    def __init__(self, core, jobs_ctx, event_bus, workspace_fs, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.core = core
        self.jobs_ctx = jobs_ctx
        self.event_bus = event_bus
        self.workspace_fs = workspace_fs

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
        self.line_edit_search.textChanged.connect(self.filter_files)
        self.current_directory = FsPath("/")
        self.fs_updated_qt.connect(self._on_fs_updated_qt)
        self.fs_synced_qt.connect(self._on_fs_synced_qt)
        self.update_timer = QTimer()
        self.update_timer.timeout.connect(self.reload)
        self.default_import_path = str(pathlib.Path.home())
        self.table_files.file_moved.connect(self.on_file_moved)
        self.table_files.item_activated.connect(self.item_activated)
        self.table_files.rename_clicked.connect(self.rename_files)
        self.table_files.delete_clicked.connect(self.delete_files)
        self.table_files.open_clicked.connect(self.open_files)

        self.core.fs.event_bus.connect("fs.entry.updated", self._on_fs_entry_updated_trio)
        self.core.fs.event_bus.connect("fs.entry.synced", self._on_fs_entry_synced_trio)

        self.label_current_workspace.setText(workspace_fs.workspace_name)
        self.load(self.current_directory)
        self.table_files.sortItems(0)

    def disconnect_all(self):
        self.core.fs.event_bus.disconnect("fs.entry.updated", self._on_fs_entry_updated_trio)
        self.core.fs.event_bus.disconnect("fs.entry.synced", self._on_fs_entry_synced_trio)

    def rename_files(self):
        files = self.table_files.selected_files()
        if len(files) == 1:
            new_name = get_text(
                self,
                _("Rename a file"),
                _("Enter file new name"),
                placeholder="File name",
                default_text=files[0][2],
            )
            if not new_name:
                return
            if not self.rename(files[0][2], new_name):
                show_error(self, _('File "{}" could not be renamed.').format(files[0][2]))

        else:
            new_name = get_text(
                self,
                _("Rename {} files").format(len(files)),
                _("Enter files new name (without extension)"),
                placeholder="Files name",
            )
            if not new_name:
                return
            r = True
            for i, f in enumerate(files, 1):
                old_file = pathlib.Path(f[2])
                r &= self.rename(f[2], "{}_{}{}".format(new_name, i, ".".join(old_file.suffixes)))
            if not r:
                show_error(self, _("Some files could not be renamed."))

    def rename(self, file_name, new_name):
        try:
            self.jobs_ctx.run(
                self.workspace_fs.rename,
                self.current_directory / file_name,
                self.current_directory / new_name,
            )
            return True
        except:
            return False

    def delete_files(self):
        files = self.table_files.selected_files()
        if len(files) == 1:
            result = ask_question(
                self,
                _("Confirmation"),
                _('Are you sure you want to delete "{}"?').format(files[0][2]),
            )
        else:
            result = ask_question(
                self,
                _("Confirmation"),
                _("Are you sure you want to delete {} files?").format(len(files)),
            )
        if not result:
            return
        r = True
        for f in files:
            r &= self.delete_item(*f)
            if r:
                self.table_files.removeRow(f[0])
        if not r:
            if len(files) == 1:
                show_error(self, _('Can not delete "{}".').format(f[2]))
            else:
                show_error(self, _("Some files could not be deleted."))
        self.table_files.clearSelection()

    def delete_item(self, row, file_type, file_name):
        path = self.current_directory / file_name
        try:
            if file_type == FileType.Folder:
                self._delete_folder(path)
            else:
                self.jobs_ctx.run(self.workspace_fs.unlink, path)
            return True
        except:
            return False

    def _delete_folder(self, path):
        self.jobs_ctx.run(self.workspace_fs.rmtree, path)

    def open_files(self):
        files = self.table_files.selected_files()
        if len(files) == 1:
            self.open_file(files[0][2])
        else:
            result = ask_question(
                self,
                _("Confirmation"),
                _("Are you sure you want to open {} files?").format(len(files)),
            )
            if not result:
                return
            for f in files:
                self.open_file(f[2])

    def open_file(self, file_name):
        path = self.core.mountpoint_manager.get_path_in_mountpoint(
            self.workspace_fs.workspace_id, self.current_directory / file_name
        )
        desktop.open_file(str(path))

    def item_activated(self, file_type, file_name):
        if file_type == FileType.ParentFolder:
            self.load(self.current_directory.parent)
        elif file_type == FileType.ParentWorkspace:
            self.back_clicked.emit()
        elif file_type == FileType.File:
            self.open_file(file_name)
        elif file_type == FileType.Folder:
            self.load(self.current_directory / file_name)

    def get_taskbar_buttons(self):
        return self.taskbar_buttons

    def reload(self):
        self.load(self.current_directory)

    def load(self, directory):
        self.update_timer.stop()

        self.table_files.clear()
        old_sort = self.table_files.horizontalHeader().sortIndicatorSection()
        old_order = self.table_files.horizontalHeader().sortIndicatorOrder()
        self.table_files.setSortingEnabled(False)
        self.current_directory = directory
        if self.current_directory == FsPath("/"):
            self.table_files.add_parent_workspace()
        else:
            self.table_files.add_parent_folder()
        self.label_current_directory.setText(str(self.current_directory))
        self.label_current_directory.show()
        self.label_caret.show()
        dir_path = self.current_directory
        dir_stat = self.jobs_ctx.run(self.workspace_fs.path_info, dir_path)
        for i, child in enumerate(dir_stat["children"]):
            child_stat = self.jobs_ctx.run(self.workspace_fs.path_info, dir_path / child)
            if child_stat["type"] == "folder":
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
                self.jobs_ctx.run(self.workspace_fs.path_info, dst)
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
            dst = self.current_directory / p.name
            files.append((p, dst))
            total_size += p.stat().st_size
        return files, total_size

    def get_folder(self, src, dst):
        files = []
        total_size = 0
        try:
            self.jobs_ctx.run(self.workspace_fs.mkdir, dst)
        except FileExistsError:
            pass
        for f in src.iterdir():
            if f.is_dir():
                new_files, new_size = self.get_folder(f, FsPath("/") / dst / f.name)
                files.extend(new_files)
                total_size += new_size
            elif f.is_file():
                new_dst = FsPath("/") / dst / f.name
                files.append((f, new_dst))
                total_size += f.stat().st_size
        return files, total_size

    def import_file(self, src, dst, current_size, loading_dialog):
        loading_dialog.set_current_file(src.name)
        try:
            try:
                # TODO: use `self.workspace_fs.open(dst, "w")` when implemented
                self.jobs_ctx.run(self.workspace_fs.touch, dst)
            except FileExistsError:
                self.jobs_ctx.run(self.workspace_fs.truncate, 0)
            with open(src, "rb") as fd_in:
                i = 0
                read_size = 0
                while True:
                    chunk = fd_in.read(65536)
                    if not chunk:
                        break
                    self.jobs_ctx.run(self.workspace_fs.write_bytes, dst, chunk, read_size)
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
            if loading_dialog.is_cancelled:
                self.jobs_ctx.run(self.workspace_fs.unlink, dst)

    # slot
    def import_files_clicked(self):
        paths, x = QFileDialog.getOpenFileNames(
            self, _("Select files to import"), self.default_import_path
        )
        if not paths:
            return
        files, total_size = self.get_files(paths)
        f = files[0][0]
        self.default_import_path = str(f.parent)
        self.import_all(files, total_size)

    # slot
    def import_folder_clicked(self):
        path = QFileDialog.getExistingDirectory(
            self, _("Select a directory to import"), self.default_import_path
        )
        if not path:
            return
        p = pathlib.Path(path)
        files, total_size = self.get_folder(p, self.current_directory / p.name)
        self.default_import_path = str(p)
        self.import_all(files, total_size)

    # slot
    def filter_files(self, pattern):
        pattern = pattern.lower()
        for i in range(self.table_files.rowCount()):
            file_type = self.table_files.item(i, 0).data(TYPE_DATA_INDEX)
            name_item = self.table_files.item(i, 1)
            if file_type != FileType.ParentFolder and file_type != FileType.ParentWorkspace:
                if pattern not in name_item.text().lower():
                    self.table_files.setRowHidden(i, True)
                else:
                    self.table_files.setRowHidden(i, False)

    def _create_folder(self, folder_name):
        try:
            dir_path = self.current_directory / folder_name
            self.jobs_ctx.run(self.workspace_fs.mkdir, dir_path)
            return True
        except FileExistsError:
            show_error(self, _("A folder with the same name already exists."))
            return False

    # slot
    def create_folder_clicked(self):
        folder_name = get_text(
            self, _("Folder name"), _("Enter new folder name"), placeholder="Name"
        )
        if not folder_name:
            return
        self._create_folder(folder_name)

    # slot
    def on_file_moved(self, src, dst):
        src_path = self.current_directory / src
        dst_path = ""
        if dst == "..":
            target_dir = self.current_directory.parent
            dst_path = FsPath("/") / target_dir / src
        else:
            dst_path = FsPath("/") / dst / src
        self.jobs_ctx.run(self.workspace_fs.move, src_path, dst_path)

    def keyPressEvent(self, event):
        if event.key() == Qt.Key_Delete:
            row = self.table_files.currentRow()
            self.delete_item(row)()

    # slot
    def _on_fs_entry_synced_trio(self, event, path, id):
        self.fs_synced_qt.emit(event, id, path)

    # slot
    def _on_fs_entry_updated_trio(self, event, id):
        self.fs_updated_qt.emit(event, id)

    # slot
    def _on_fs_synced_qt(self, event, id, path):
        if path is None:
            try:
                path = self.jobs_ctx.run(self.workspace_fs.get_entry_path, id)
            except FSEntryNotFound:
                return

        if path is None:
            return

        path = FsPath(path)
        modified_hops = list(path.parts)
        current_dir_hops = ["/", self.workspace_fs.workspace_name]
        current_dir_hops.extend((x for x in self.current_directory.parts if x != "/"))

        # Only visible files require updated
        if modified_hops[:-1] != current_dir_hops:
            return

        for i in range(self.table_files.rowCount()):
            name_item = self.table_files.item(i, 1)
            if name_item.text() == path.name:
                icon_item = self.table_files.item(i, 0)
                file_type = icon_item.data(TYPE_DATA_INDEX)
                if file_type == FileType.File or file_type == FileType.Folder:
                    icon_item.is_synced = True
                    return

    # slot
    def _on_fs_updated_qt(self, event, id):
        id = AccessID(id)
        path = None
        try:
            path = self.jobs_ctx.run(self.workspace_fs.get_entry_path, id)
        except FSEntryNotFound:
            # Entry not locally present, nothing to do
            return

        # Modifications on root is handled by workspace_widget
        if path is None or path == pathlib.Path("/"):
            return

        modified_hops = list(path.parts)
        current_dir_hops = ["/", self.workspace_fs.workspace_name]
        current_dir_hops.extend((x for x in self.current_directory.parts if x != "/"))
        # Only direct children to current directory require reloading
        if modified_hops == current_dir_hops or modified_hops[:-1] == current_dir_hops:
            self.update_timer.start(1000)
