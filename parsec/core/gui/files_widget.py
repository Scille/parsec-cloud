# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import time
import pathlib
from uuid import UUID

from PyQt5.QtCore import Qt, pyqtSignal, QTimer
from PyQt5.QtWidgets import QFileDialog, QApplication, QDialog, QWidget

from parsec.core.types import FsPath, EntryID, WorkspaceEntry, WorkspaceRole
from parsec.core.fs import FSEntryNotFound

from parsec.core.gui.trio_thread import JobResultError, ThreadSafeQtSignal, QtToTrioJob
from parsec.core.gui import desktop
from parsec.core.gui.file_items import FileType, TYPE_DATA_INDEX
from parsec.core.gui.custom_widgets import (
    QuestionDialog,
    MessageDialog,
    TextInputDialog,
    TaskbarButton,
)
from parsec.core.gui.loading_dialog import LoadingDialog
from parsec.core.gui.lang import translate as _
from parsec.core.gui.replace_dialog import ReplaceDialog
from parsec.core.gui.ui.files_widget import Ui_FilesWidget


class CancelException(Exception):
    pass


async def _do_rename(workspace_fs, old_path, new_path):
    try:
        await workspace_fs.rename(old_path, new_path)
    except:
        raise JobResultError("error")


async def _do_delete(workspace_fs, files):
    try:
        rows = []
        for path, file_type, row in files:
            try:
                if file_type == FileType.Folder:
                    await workspace_fs.rmtree(path)
                else:
                    await workspace_fs.unlink(path)
                rows.append(row)
            except:
                pass
        return rows
    except:
        raise JobResultError("error")


async def _do_folder_stat(workspace_fs, path):
    try:
        stats = {}
        dir_stat = await workspace_fs.path_info(path)
        for child in dir_stat["children"]:
            child_stat = await workspace_fs.path_info(path / child)
            stats[child] = child_stat
        return path, stats
    except:
        raise JobResultError("error")


async def _do_folder_create(workspace_fs, path):
    try:
        await workspace_fs.mkdir(path)
    except FileExistsError:
        raise JobResultError("already-exists")
    except:
        raise JobResultError("error")


async def _do_import(workspace_fs, files, destination, progress_signal):
    for f in files:
        try:
            progress_signal.emit(100)
        except:
            pass


class FilesWidget(QWidget, Ui_FilesWidget):
    fs_updated_qt = pyqtSignal(str, UUID)
    fs_synced_qt = pyqtSignal(str, UUID)
    sharing_updated_qt = pyqtSignal(WorkspaceEntry, WorkspaceEntry)
    sharing_revoked_qt = pyqtSignal(WorkspaceEntry, WorkspaceEntry)
    taskbar_updated = pyqtSignal()
    back_clicked = pyqtSignal()

    rename_success = pyqtSignal(QtToTrioJob)
    rename_error = pyqtSignal(QtToTrioJob)
    delete_success = pyqtSignal(QtToTrioJob)
    delete_error = pyqtSignal(QtToTrioJob)
    folder_stat_success = pyqtSignal(QtToTrioJob)
    folder_stat_error = pyqtSignal(QtToTrioJob)
    folder_create_success = pyqtSignal(QtToTrioJob)
    folder_create_error = pyqtSignal(QtToTrioJob)
    import_success = pyqtSignal(QtToTrioJob)
    import_error = pyqtSignal(QtToTrioJob)

    import_progress = pyqtSignal(int)

    def __init__(self, core, jobs_ctx, event_bus, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.core = core
        self.jobs_ctx = jobs_ctx
        self.event_bus = event_bus
        self._workspace_fs = None
        self.import_job = None

        self.ROLES_TEXTS = {
            WorkspaceRole.READER: _("Reader"),
            WorkspaceRole.CONTRIBUTOR: _("Contributor"),
            WorkspaceRole.MANAGER: _("Manager"),
            WorkspaceRole.OWNER: _("Owner"),
        }

        self.button_back = TaskbarButton(icon_path=":/icons/images/icons/return_off.png")
        self.button_back.clicked.connect(self.back_clicked)
        self.button_import_folder = TaskbarButton(
            icon_path=":/icons/images/icons/upload_folder_off.png"
        )
        self.button_import_folder.clicked.connect(self.import_folder_clicked)
        self.button_import_files = TaskbarButton(
            icon_path=":/icons/images/icons/upload_file_off.png"
        )
        self.button_import_files.clicked.connect(self.import_files_clicked)
        self.button_create_folder = TaskbarButton(icon_path=":/icons/images/icons/plus_off.png")
        self.button_create_folder.clicked.connect(self.create_folder_clicked)
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

        self.event_bus.connect("fs.entry.updated", self._on_fs_entry_updated_trio)
        self.event_bus.connect("fs.entry.synced", self._on_fs_entry_synced_trio)
        self.event_bus.connect("sharing.updated", self._on_sharing_updated_trio)
        self.event_bus.connect("sharing.revoked", self._on_sharing_revoked_trio)

        self.sharing_updated_qt.connect(self._on_sharing_updated_qt)
        self.sharing_revoked_qt.connect(self._on_sharing_revoked_qt)
        self.rename_success.connect(self._on_rename_success)
        self.rename_error.connect(self._on_rename_error)
        self.delete_success.connect(self._on_delete_success)
        self.delete_error.connect(self._on_delete_error)
        self.folder_stat_success.connect(self._on_folder_stat_success)
        self.folder_stat_error.connect(self._on_folder_stat_error)
        self.folder_create_success.connect(self._on_folder_create_success)
        self.folder_create_error.connect(self._on_folder_create_error)
        self.import_success.connect(self._on_import_success)
        self.import_error.connect(self._on_import_error)

    def disconnect_all(self):
        self.event_bus.disconnect("fs.entry.updated", self._on_fs_entry_updated_trio)
        self.event_bus.disconnect("fs.entry.synced", self._on_fs_entry_synced_trio)
        self.event_bus.disconnect("sharing.updated", self._on_sharing_updated_trio)
        self.event_bus.disconnect("sharing.revoked", self._on_sharing_revoked_trio)

    @property
    def workspace_fs(self):
        return self._workspace_fs

    @workspace_fs.setter
    def workspace_fs(self, val):
        self.current_directory = FsPath("/")
        self._workspace_fs = val
        ws_entry = self.workspace_fs.get_workspace_entry()
        self.current_user_role = ws_entry.role
        self.label_role.setText(self.ROLES_TEXTS[self.current_user_role])
        self.table_files.current_user_role = self.current_user_role
        self.reset()

    def reset(self):
        self.label_current_workspace.setText(self.workspace_fs.workspace_name)
        self.load(self.current_directory)
        self.table_files.sortItems(0)

    def rename_files(self):
        files = self.table_files.selected_files()
        if len(files) == 1:
            new_name = TextInputDialog.get_text(
                self,
                _("Rename a file"),
                _("Enter file new name"),
                placeholder="File name",
                default_text=files[0][2],
            )
            if not new_name:
                return
            if not self.rename(files[0][2], new_name):
                MessageDialog.show_error(
                    self, _('File "{}" could not be renamed.').format(files[0][2])
                )

        else:
            new_name = TextInputDialog.get_text(
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
                MessageDialog.show_error(self, _("Some files could not be renamed."))

    def rename(self, file_name, new_name):
        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "rename_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "rename_error", QtToTrioJob),
            _do_rename,
            workspace_fs=self.workspace_fs,
            old_path=self.current_directory / file_name,
            new_path=self.current_directory / new_name,
        )

    def delete_files(self):
        files = self.table_files.selected_files()
        if len(files) == 1:
            result = QuestionDialog.ask(
                self,
                _("Confirmation"),
                _('Are you sure you want to delete "{}"?').format(files[0][2]),
            )
        else:
            result = QuestionDialog.ask(
                self,
                _("Confirmation"),
                _("Are you sure you want to delete {} files?").format(len(files)),
            )
        if not result:
            return
        r = True

        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "delete_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "delete_error", QtToTrioJob),
            _do_delete,
            workspace_fs=self.workspace_fs,
            files=[(self.current_directory / f[2], f[1], f[0]) for f in files],
        )

    def open_files(self):
        files = self.table_files.selected_files()
        if len(files) == 1:
            self.open_file(files[0][2])
        else:
            result = QuestionDialog.ask(
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
        if self.current_user_role == WorkspaceRole.READER:
            return [self.button_back]
        else:
            return [
                self.button_back,
                self.button_import_folder,
                self.button_import_files,
                self.button_create_folder,
            ]

    def reload(self):
        self.load(self.current_directory)

    def load(self, directory):
        self.update_timer.stop()

        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "folder_stat_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "folder_stat_error", QtToTrioJob),
            _do_folder_stat,
            workspace_fs=self.workspace_fs,
            path=directory,
        )

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

    def create_folder_clicked(self):
        folder_name = TextInputDialog.get_text(
            self, _("Folder name"), _("Enter new folder name"), placeholder="Name"
        )
        if not folder_name:
            return

        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "create_folder_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "create_folder_error", QtToTrioJob),
            _do_folder_create,
            workspace_fs=self.workspace_fs,
            path=self.current_directory / folder_name,
        )

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

    def _on_rename_success(self, job):
        pass

    def _on_rename_error(self, job):
        MessageDialog.show_error(self, _("Can not rename the file."))

    def _on_delete_success(self, job):
        rows = job.ret
        for row in rows:
            self.table_files.removeRow(f[0])
        self.table_files.clearSelection()

    def _on_delete_error(self, job):
        MessageDialog.show_error(self, _("Can not delete the file."))

    def _on_folder_stat_success(self, job):
        self.current_directory, files_stats = job.ret
        str_dir = str(self.current_directory)

        self.table_files.clear()
        old_sort = self.table_files.horizontalHeader().sortIndicatorSection()
        old_order = self.table_files.horizontalHeader().sortIndicatorOrder()
        self.table_files.setSortingEnabled(False)
        if self.current_directory == FsPath("/"):
            self.table_files.add_parent_workspace()
        else:
            self.table_files.add_parent_folder()
        self.line_edit_current_directory.setText(str_dir)
        self.line_edit_current_directory.setCursorPosition(0)
        for path, stats in files_stats.items():
            if stats["type"] == "folder":
                self.table_files.add_folder(str(path), not stats["need_sync"])
            else:
                self.table_files.add_file(
                    str(path), stats["size"], stats["created"], stats["updated"], not stats["need_sync"]
                )
        self.table_files.sortItems(old_sort, old_order)
        self.table_files.setSortingEnabled(True)
        if self.line_edit_search.text():
            self.filter_files(self.line_edit_search.text())

    def _on_folder_stat_error(self, job):
        print("STAT ERROR")

    def _on_folder_create_success(self, job):
        pass

    def _on_folder_create_error(self, job):
        if job.status == "already-exists":
            MessageDialog.show_error(self, _("A folder with the same name already exists."))
        else:
            MessageDialog.show_error(self, _("Can not create the folder."))

    def _on_import_success(self):
        pass

    def _on_import_error(self):
        pass

    def _on_fs_entry_synced_trio(self, event, path, id):
        self.fs_synced_qt.emit(event, id, path)

    def _on_fs_entry_updated_trio(self, event, workspace_id=None, id=None):
        assert id is not None
        if workspace_id is None or workspace_id == self.workspace_fs.workspace_id:
            self.fs_updated_qt.emit(event, id)

    def _on_fs_synced_qt(self, event, id, path):
        if not self.workspace_fs:
            return
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

    def _on_fs_updated_qt(self, event, id):
        if not self.workspace_fs:
            return

        id = EntryID(id)
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

    def _on_sharing_revoked_trio(self, event, new_entry, previous_entry):
        self.sharing_revoked_qt.emit(new_entry, previous_entry)

    def _on_sharing_revoked_qt(self, new_entry, previous_entry):
        MessageDialog.show_error(
            self, _("You no longer have the permission to access this workspace.")
        )
        self.back_clicked.emit()

    def _on_sharing_updated_trio(self, event, new_entry, previous_entry):
        self.sharing_updated_qt.emit(new_entry, previous_entry)

    def _on_sharing_updated_qt(self, new_entry, previous_entry):
        self.current_user_role = new_entry.role
        self.label_role.setText(self.ROLES_TEXTS[self.current_user_role])
        if previous_entry.role != WorkspaceRole.READER and new_entry.role == WorkspaceRole.READER:
            MessageDialog.show_warning(self, _("You are now a reader on this workspace."))
            self.taskbar_updated.emit()
        else:
            self.taskbar_updated.emit()
