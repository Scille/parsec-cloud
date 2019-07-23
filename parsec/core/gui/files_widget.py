# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pathlib
from uuid import UUID
import trio

from PyQt5.QtCore import Qt, pyqtSignal, QTimer
from PyQt5.QtWidgets import QFileDialog, QWidget

from parsec.core.types import FsPath, WorkspaceEntry, WorkspaceRole
from parsec.core.fs import WorkspaceFSTimestamped

from parsec.core.gui.trio_thread import JobResultError, ThreadSafeQtSignal, QtToTrioJob
from parsec.core.gui import desktop
from parsec.core.gui.file_items import FileType, TYPE_DATA_INDEX, UUID_DATA_INDEX
from parsec.core.gui.custom_dialogs import QuestionDialog, show_error, show_warning, TextInputDialog
from parsec.core.gui.custom_widgets import TaskbarButton

from parsec.core.gui.loading_dialog import LoadingDialog
from parsec.core.gui.lang import translate as _
from parsec.core.gui.ui.files_widget import Ui_FilesWidget


class CancelException(Exception):
    pass


async def _do_reset(workspace_fs):
    return await workspace_fs.get_workspace_name()


async def _do_set_workspace_fs(workspace_fs):
    return await workspace_fs.get_workspace_entry()


async def _do_rename(workspace_fs, paths):
    new_names = {}
    for (old_path, new_path, uuid) in paths:
        try:
            await workspace_fs.rename(old_path, new_path)
            new_names[uuid] = FsPath(new_path).name
        except FileExistsError as exc:
            raise JobResultError("already-exists", multi=len(paths) > 1) from exc
        except OSError as exc:
            raise JobResultError("not-empty", multi=len(paths) > 1) from exc


async def _do_delete(workspace_fs, files, silent=False):
    for path, file_type in files:
        try:
            if file_type == FileType.Folder:
                await workspace_fs.rmtree(path)
            else:
                await workspace_fs.unlink(path)
        except Exception as exc:
            if not silent:
                raise JobResultError("error", multi=len(files) > 1) from exc


async def _do_folder_stat(workspace_fs, path):
    stats = {}
    dir_stat = await workspace_fs.path_info(path)
    for child in dir_stat["children"]:
        child_stat = await workspace_fs.path_info(path / child)
        stats[child] = child_stat
    return path, dir_stat["id"], stats


async def _do_folder_create(workspace_fs, path):
    try:
        await workspace_fs.mkdir(path)
    except FileExistsError as exc:
        raise JobResultError("already-exists") from exc


async def _do_import(workspace_fs, files, total_size, progress_signal):
    current_size = 0
    for src, dst in files:
        try:
            if dst.parent != FsPath("/"):
                await workspace_fs.mkdir(dst.parent, parents=True, exist_ok=True)
            try:
                await workspace_fs.touch(dst)
            except FileExistsError:
                await workspace_fs.truncate(dst, 0)
            with open(src, "rb") as fd_in:
                i = 0
                read_size = 0
                while True:
                    chunk = fd_in.read(65536)
                    if not chunk:
                        break
                    await workspace_fs.write_bytes(dst, chunk, read_size)
                    read_size += len(chunk)
                    i += 1
                    progress_signal.emit(current_size + read_size)
            current_size += src.stat().st_size + 1
            progress_signal.emit(current_size)
        except trio.Cancelled as exc:
            raise JobResultError("cancelled", last_file=dst) from exc


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
    set_workspace_fs_success = pyqtSignal(QtToTrioJob)
    set_workspace_fs_error = pyqtSignal(QtToTrioJob)
    reset_success = pyqtSignal(QtToTrioJob)
    reset_error = pyqtSignal(QtToTrioJob)

    import_progress = pyqtSignal(int)

    def __init__(self, core, jobs_ctx, event_bus, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.core = core
        self.jobs_ctx = jobs_ctx
        self.event_bus = event_bus
        self.workspace_fs = None
        self.import_job = None

        self.current_user_role = None
        self.ROLES_TEXTS = {
            WorkspaceRole.READER: _("WORKSPACE_ROLE_READER"),
            WorkspaceRole.CONTRIBUTOR: _("WORKSPACE_ROLE_CONTRIBUTOR"),
            WorkspaceRole.MANAGER: _("WORKSPACE_ROLE_MANAGER"),
            WorkspaceRole.OWNER: _("WORKSPACE_ROLE_OWNER"),
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
        self.current_directory_uuid = None
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
        self.set_workspace_fs_success.connect(self._on_set_workspace_fs_success)
        self.reset_success.connect(self._on_reset_success)

        self.loading_dialog = None
        self.import_progress.connect(self._on_import_progress)

    def disconnect_all(self):
        pass

    def showEvent(self, event):
        self.event_bus.connect("fs.entry.updated", self._on_fs_entry_updated_trio)
        self.event_bus.connect("fs.entry.synced", self._on_fs_entry_synced_trio)
        self.event_bus.connect("sharing.updated", self._on_sharing_updated_trio)
        self.event_bus.connect("sharing.revoked", self._on_sharing_revoked_trio)

    def hideEvent(self, event):
        try:
            self.event_bus.disconnect("fs.entry.updated", self._on_fs_entry_updated_trio)
            self.event_bus.disconnect("fs.entry.synced", self._on_fs_entry_synced_trio)
            self.event_bus.disconnect("sharing.updated", self._on_sharing_updated_trio)
            self.event_bus.disconnect("sharing.revoked", self._on_sharing_revoked_trio)
        except ValueError:
            pass

    def _on_set_workspace_fs_success(self, job):
        ws_entry = job.ret
        self.current_user_role = ws_entry.role
        self.label_role.setText(self.ROLES_TEXTS[self.current_user_role])
        self.table_files.current_user_role = self.current_user_role
        self.reset()

    def set_workspace_fs(self, wk_fs):
        self.current_directory = FsPath("/")
        self.workspace_fs = wk_fs
        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "set_workspace_fs_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "set_workspace_fs_error", QtToTrioJob),
            _do_set_workspace_fs,
            self.workspace_fs,
        )

    def _on_reset_success(self, job):
        workspace_name = job.ret
        self.label_current_workspace.setText(workspace_name)
        self.load(self.current_directory)
        self.table_files.sortItems(0)

    def reset(self):
        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "reset_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "reset_error", QtToTrioJob),
            _do_reset,
            self.workspace_fs,
        )

    def rename_files(self):
        files = self.table_files.selected_files()
        if len(files) == 1:
            new_name = TextInputDialog.get_text(
                self,
                _("ASK_FILE_RENAME_TITLE"),
                _("ASK_FILE_RENAME_CONTENT"),
                placeholder=_("ASK_FILE_RENAME_PLACEHOLDER"),
                default_text=files[0].name,
            )
            if not new_name:
                return
            self.jobs_ctx.submit_job(
                ThreadSafeQtSignal(self, "rename_success", QtToTrioJob),
                ThreadSafeQtSignal(self, "rename_error", QtToTrioJob),
                _do_rename,
                workspace_fs=self.workspace_fs,
                paths=[
                    (
                        self.current_directory / files[0].name,
                        self.current_directory / new_name,
                        files[0].uuid,
                    )
                ],
            )
        else:
            new_name = TextInputDialog.get_text(
                self,
                _("ASK_MULTI_FILE_RENAME_TITLE_{}").format(len(files)),
                _("ASK_MULTI_FILE_RENAME_CONTENT_{}").format(len(files)),
                placeholder=_("ASK_MULTI_FILE_RENAME_PLACEHOLDER"),
            )
            if not new_name:
                return

            self.jobs_ctx.submit_job(
                ThreadSafeQtSignal(self, "rename_success", QtToTrioJob),
                ThreadSafeQtSignal(self, "rename_error", QtToTrioJob),
                _do_rename,
                workspace_fs=self.workspace_fs,
                paths=[
                    (
                        self.current_directory / f.name,
                        self.current_directory
                        / "{}_{}{}".format(new_name, i, ".".join(pathlib.Path(f.name).suffixes)),
                        f.uuid,
                    )
                    for i, f in enumerate(files, 1)
                ],
            )

    def delete_files(self):
        files = self.table_files.selected_files()
        if len(files) == 1:
            result = QuestionDialog.ask(
                self,
                _("ASK_FILE_DELETE_TITLE"),
                _("ASK_FILE_DELETE_CONTENT_{}").format(files[0].name),
            )
        else:
            result = QuestionDialog.ask(
                self,
                _("ASK_MULTI_FILE_DELETE_TITLE_{}").format(len(files)),
                _("ASK_MULTI_FILE_DELETE_CONTENT_{}").format(len(files)),
            )
        if not result:
            return
        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "delete_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "delete_error", QtToTrioJob),
            _do_delete,
            workspace_fs=self.workspace_fs,
            files=[(self.current_directory / f.name, f.type) for f in files],
        )

    def open_files(self):
        files = self.table_files.selected_files()
        if len(files) == 1:
            self.open_file(files[0][2])
        else:
            result = QuestionDialog.ask(
                self,
                _("ASK_OPEN_MULTI_FILE_TITLE_{}").format(len(files)),
                _("ASK_OPEN_MULTI_FILE_CONTENT_{}").format(len(files)),
            )
            if not result:
                return
            for f in files:
                self.open_file(f[2])

    def open_file(self, file_name):
        path = self.core.mountpoint_manager.get_path_in_mountpoint(
            self.workspace_fs.workspace_id,
            self.current_directory / file_name,
            self.workspace_fs.timestamp
            if isinstance(self.workspace_fs, WorkspaceFSTimestamped)
            else None,
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
        assert not self.import_job

        self.loading_dialog = LoadingDialog(total_size=total_size + len(files), parent=self)
        self.loading_dialog.cancel_clicked.connect(self.cancel_import)
        self.loading_dialog.show()

        self.import_job = self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "import_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "import_error", QtToTrioJob),
            _do_import,
            workspace_fs=self.workspace_fs,
            files=files,
            total_size=total_size,
            progress_signal=ThreadSafeQtSignal(self, "import_progress", int),
        )

    def cancel_import(self):
        assert self.import_job
        assert self.loading_dialog

        self.import_job.cancel_and_join()

    def _on_import_progress(self, progress):
        if not self.loading_dialog:
            return
        self.loading_dialog.set_progress(progress)

    def get_files(self, paths):
        files = []
        total_size = 0
        for path in paths:
            p = pathlib.Path(path)
            dst = self.current_directory / p.name
            files.append((p, dst))
            total_size += p.stat().st_size
        return files, total_size

    def get_folder(self, src, parent=None):
        files = []
        total_size = 0
        if parent is None:
            dst = self.current_directory / src.name
        else:
            dst = parent / src.name
        for f in src.iterdir():
            if f.is_dir():
                new_files, new_size = self.get_folder(f, parent=dst)
                files.extend(new_files)
                total_size += new_size
            elif f.is_file():
                new_dst = dst / f.name
                files.append((f, new_dst))
                total_size += f.stat().st_size
        return files, total_size

    def import_files_clicked(self):
        paths, x = QFileDialog.getOpenFileNames(
            self, _("ASK_IMPORT_FILES_TITLE"), self.default_import_path
        )
        if not paths:
            return
        files, total_size = self.get_files(paths)
        f = files[0][0]
        self.default_import_path = str(f.parent)
        self.import_all(files, total_size)

    def import_folder_clicked(self):
        path = QFileDialog.getExistingDirectory(
            self, _("ASK_IMPORT_FOLDER_TITLE"), self.default_import_path
        )
        if not path:
            return
        p = pathlib.Path(path)
        files, total_size = self.get_folder(p)
        self.default_import_path = str(p)
        self.import_all(files, total_size)

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
            self,
            _("ASK_CREATE_FOLDER_TITLE"),
            _("ASK_CREATE_FOLDER_CONTENT"),
            placeholder=_("ASK_CREATE_FOLDER_PLACEHOLDER"),
        )
        if not folder_name:
            return

        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "folder_create_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "folder_create_error", QtToTrioJob),
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
        self.reset()

    def _on_rename_error(self, job):
        if job.exc.params.get("multi"):
            show_error(self, _("ERR_RENAME_MULTI_FILE"), exception=job.exc)
        else:
            show_error(self, _("ERR_RENAME_FILE"), exception=job.exc)

    def _on_delete_success(self, job):
        self.reset()

    def _on_delete_error(self, job):
        if job.exc.params.get("multi"):
            show_error(self, _("ERR_DELETE_MULTI_FILE"), exception=job.exc)
        else:
            show_error(self, _("ERR_DELETE_FILE"), exception=job.exc)

    def _on_folder_stat_success(self, job):
        self.current_directory, self.current_directory_uuid, files_stats = job.ret
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
                self.table_files.add_folder(str(path), stats["id"], not stats["need_sync"])
            else:
                self.table_files.add_file(
                    str(path),
                    stats["id"],
                    stats["size"],
                    stats["created"],
                    stats["updated"],
                    not stats["need_sync"],
                )
        self.table_files.sortItems(old_sort, old_order)
        self.table_files.setSortingEnabled(True)
        if self.line_edit_search.text():
            self.filter_files(self.line_edit_search.text())

    def _on_folder_stat_error(self, job):
        pass

    def _on_folder_create_success(self, job):
        pass

    def _on_folder_create_error(self, job):
        if job.status == "already-exists":
            show_error(self, _("ERR_FOLDER_CREATE_ALREADY_EXISTS"))
        else:
            show_error(self, _("ERR_FOLDER_CREATE_UNKNOWN"))

    def _on_import_success(self):
        assert self.loading_dialog
        self.loading_dialog.hide()
        self.loading_dialog.setParent(None)
        self.loading_dialog = None
        self.import_job = None

    def _on_import_error(self):
        assert self.loading_dialog
        if self.import_job.exc.status == "cancelled":
            self.jobs_ctx.submit_job(
                ThreadSafeQtSignal(self, "delete_success", QtToTrioJob),
                ThreadSafeQtSignal(self, "delete_error", QtToTrioJob),
                _do_delete,
                workspace_fs=self.workspace_fs,
                files=[(self.import_job.exc.params["last_file"], FileType.File)],
                silent=True,
            )
        else:
            show_error(self, _("ERR_IMPORT_FILES"), exception=self.import_job.exc)
        self.loading_dialog.hide()
        self.loading_dialog.setParent(None)
        self.loading_dialog = None
        self.import_job = None

    def _on_fs_entry_synced_trio(self, event, id, path=None, workspace_id=None):
        self.fs_synced_qt.emit(event, id)

    def _on_fs_entry_updated_trio(self, event, workspace_id=None, id=None):
        assert id is not None
        if workspace_id is None or workspace_id == self.workspace_fs.workspace_id:
            self.fs_updated_qt.emit(event, id)

    def _on_fs_synced_qt(self, event, uuid):
        if not self.workspace_fs:
            return

        if self.current_directory_uuid == uuid:
            return

        for i in range(1, self.table_files.rowCount()):
            item = self.table_files.item(i, 0)
            if item.data(UUID_DATA_INDEX) == uuid:
                if (
                    item.data(TYPE_DATA_INDEX) == FileType.File
                    or item.data(TYPE_DATA_INDEX) == FileType.Folder
                ):
                    item.is_synced = True

    def _on_fs_updated_qt(self, event, uuid):
        if not self.workspace_fs:
            return

        if self.current_directory_uuid == uuid:
            self.update_timer.start(1000)

    def _on_sharing_revoked_trio(self, event, new_entry, previous_entry):
        self.sharing_revoked_qt.emit(new_entry, previous_entry)

    def _on_sharing_revoked_qt(self, new_entry, previous_entry):
        show_error(self, _("ERR_FILE_SHARING_REVOKED"))
        self.back_clicked.emit()

    def _on_sharing_updated_trio(self, event, new_entry, previous_entry):
        self.sharing_updated_qt.emit(new_entry, previous_entry)

    def _on_sharing_updated_qt(self, new_entry, previous_entry):
        self.current_user_role = new_entry.role
        self.label_role.setText(self.ROLES_TEXTS[self.current_user_role])
        if previous_entry.role != WorkspaceRole.READER and new_entry.role == WorkspaceRole.READER:
            show_warning(self, _("WARN_FILE_SHARING_READER"))
            self.taskbar_updated.emit()
        else:
            self.taskbar_updated.emit()
