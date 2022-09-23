# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import trio
import pathlib
from typing import Optional, Iterable, Tuple, List
from parsec._parsec import DateTime
from enum import IntEnum
from structlog import get_logger
from PyQt5.QtCore import Qt, pyqtSignal
from PyQt5.QtWidgets import QWidget

from parsec.api.data import EntryName
from parsec.core.core_events import CoreEvent
from parsec.core.gui.file_status_widget import FileStatusWidget
from parsec.core.gui.snackbar_widget import SnackbarManager
from parsec.core.types import WorkspaceRole, EntryID
from parsec.core.fs import FsPath, WorkspaceFS, WorkspaceFSTimestamped
from parsec.core.fs.exceptions import (
    FSLocalStorageOperationalError,
    FSRemoteManifestNotFound,
    FSInvalidArgumentError,
    FSFileNotFoundError,
)
from parsec.core.gui.trio_jobs import JobResultError, QtToTrioJob
from parsec.core.gui import desktop, validators
from parsec.core.gui.file_items import FileType, TYPE_DATA_INDEX
from parsec.core.gui.custom_dialogs import (
    ask_question,
    show_error,
    get_text_input,
    show_info,
    GreyedDialog,
    QDialogInProcess,
)
from parsec.core.gui.file_history_widget import FileHistoryWidget
from parsec.core.gui.loading_widget import LoadingWidget
from parsec.core.gui.lang import translate as _
from parsec.core.gui.workspace_roles import get_role_translation
from parsec.core.gui.ui.files_widget import Ui_FilesWidget
from parsec.core.gui.file_table import PasteStatus
from parsec.core.types import DEFAULT_BLOCK_SIZE


logger = get_logger()

# Type alias for files to import
# Files are copied from a trio path to an FS path.
ImportFiles = List[Tuple[trio.Path, FsPath]]
ImportErrors = List[Tuple[trio.Path, OSError]]


class CancelException(Exception):
    pass


async def _do_rename(workspace_fs, paths):
    new_names = {}
    for (old_path, new_path, entry_id) in paths:
        try:
            await workspace_fs.rename(old_path, new_path)
            new_names[entry_id] = FsPath(new_path).name
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


async def _do_copy_files(workspace_fs, target_dir, source_files, source_workspace):
    last_exc = None
    error_count = 0
    for src, src_type in source_files:
        # In order to be able to rename the file if a file of the same name already exists
        # we need the name without extensions.
        name_we, *_ = src.name.str.split(".", 1)
        count = 2
        file_name = src.name
        while True:
            try:
                dst = target_dir / file_name
                if src_type == FileType.Folder:
                    await workspace_fs.copytree(src, dst, source_workspace)
                else:
                    await workspace_fs.copyfile(src, dst, source_workspace)
                break
            except FileExistsError:
                # File already exists, we append a counter at the end of its name
                file_name = EntryName(
                    "{} ({}){}".format(name_we, count, "".join(pathlib.Path(src.name.str).suffixes))
                )
                count += 1
            except FSInvalidArgumentError as exc:
                # Move a file onto itself
                # Not a big deal for files, we just do nothing and pretend we
                # actually did something
                # For folders we have to warn the user
                if src_type == FileType.Folder:
                    error_count += 1
                    last_exc = exc
                break
            except Exception as exc:
                # No idea what happened, we'll just warn the user that we encountered an
                # unexcepted error and log it
                error_count += 1
                last_exc = exc
                logger.exception("Unhandled error while cut/copy file", exc_info=exc)
                break
    if error_count:
        raise JobResultError("error", last_exc=last_exc, error_count=error_count)


async def _do_move_files(workspace_fs, target_dir, source_files, source_workspace):
    error_count = 0
    last_exc = None
    for src, src_type in source_files:
        # In order to be able to rename the file if a file of the same name already exists
        # we need the name without extensions.
        name_we, *_ = src.name.str.split(".", 1)
        file_name = src.name
        count = 2
        while True:
            try:
                dst = target_dir / file_name
                await workspace_fs.move(src, dst, source_workspace)
                break
            except FileExistsError:
                # File already exists, we append a counter at the end of its name
                file_name = EntryName(
                    "{} ({}){}".format(name_we, count, "".join(pathlib.Path(src.name.str).suffixes))
                )
                count += 1
            except FSInvalidArgumentError as exc:
                # Move a file onto itself
                # Not a big deal for files, we just do nothing and pretend we
                # actually did something
                # For folders we have to warn the user
                if src_type == FileType.Folder:
                    error_count += 1
                    last_exc = exc
                break
            except Exception as exc:
                # No idea what happened, we'll just warn the user that we encountered an
                # unexcepted error and log it
                error_count += 1
                last_exc = exc
                logger.exception("Unhandled error while cut/copy file", exc_info=exc)
                break
    if error_count:
        raise JobResultError("error", last_exc=last_exc, error_count=error_count)


async def _do_folder_stat(workspace_fs, path, default_selection):
    stats = {}
    dir_stat = await workspace_fs.path_info(path)
    # Retrieve children info, this is not an atomic operation so our view
    # on the folder might be slightly non-causal (typically if a file is
    # created during while we do the loop it won't appear in the final result,
    # but later update on existing files will appear).
    # We consider this fine enough given a change in the folder should lead
    # to an event from the corefs which in turn will re-trigger this stat code
    for child in dir_stat["children"]:
        try:
            child_stat = await workspace_fs.path_info(path / child)
        except FSFileNotFoundError:
            # The child entry as been concurrently removed, just ignore it
            continue
        except FSRemoteManifestNotFound:
            # Cannot get informations about this child entry, this can occur if
            # if the manifest is inconsistent (broken data or signature).
            child_stat = {"type": "inconsistency", "id": EntryID.new()}
        stats[child] = child_stat
    return path, dir_stat["id"], stats, default_selection


async def _do_folder_create(workspace_fs, path):
    try:
        await workspace_fs.mkdir(path)
    except FileExistsError as exc:
        raise JobResultError("already-exists") from exc


async def _do_remount_timestamped(
    mountpoint_manager,
    workspace_fs,
    timestamp,
    path,
    file_type,
    open_after_load,
    close_after_load,
    reload_after_remount,
):
    await mountpoint_manager.remount_workspace_new_timestamp(
        workspace_fs.workspace_id,
        workspace_fs.timestamp if isinstance(workspace_fs, WorkspaceFSTimestamped) else None,
        timestamp,
    )
    # TODO : get it directly from mountpoint_manager if API evolves
    workspace_fs = await workspace_fs.to_timestamped(timestamp)
    await workspace_fs.path_info(path)  # Checks path is valid when remounted
    return (workspace_fs, path, file_type, open_after_load, close_after_load, reload_after_remount)


class Clipboard:
    class Status(IntEnum):
        Copied = 1
        Cut = 2

    def __init__(self, files, status, source_workspace=None):
        self.files = files
        self.source_workspace = source_workspace
        self.status = status


class FilesWidget(QWidget, Ui_FilesWidget):
    RELOAD_FILES_LIST_THROTTLE_DELAY = 1  # 1s

    global_clipboard_updated = pyqtSignal(object)

    back_clicked = pyqtSignal()

    rename_success = pyqtSignal(QtToTrioJob)
    rename_error = pyqtSignal(QtToTrioJob)
    delete_success = pyqtSignal(QtToTrioJob)
    delete_error = pyqtSignal(QtToTrioJob)
    folder_stat_success = pyqtSignal(QtToTrioJob)
    folder_stat_error = pyqtSignal(QtToTrioJob)
    folder_create_success = pyqtSignal(QtToTrioJob)
    folder_create_error = pyqtSignal(QtToTrioJob)
    import_error = pyqtSignal(QtToTrioJob)
    import_success = pyqtSignal(QtToTrioJob)

    copy_success = pyqtSignal(QtToTrioJob)
    copy_error = pyqtSignal(QtToTrioJob)
    move_success = pyqtSignal(QtToTrioJob)
    move_error = pyqtSignal(QtToTrioJob)

    file_open_success = pyqtSignal(QtToTrioJob)
    file_open_error = pyqtSignal(QtToTrioJob)

    import_progress = pyqtSignal(str, int)

    reload_timestamped_requested = pyqtSignal(DateTime, FsPath, FileType, bool, bool, bool)
    reload_timestamped_success = pyqtSignal(QtToTrioJob)
    reload_timestamped_error = pyqtSignal(QtToTrioJob)
    update_version_list = pyqtSignal(WorkspaceFS, FsPath)
    close_version_list = pyqtSignal()

    folder_changed = pyqtSignal(EntryName, str)

    def __init__(self, core, jobs_ctx, event_bus, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)

        self.spinner.hide()
        self.label_elements.hide()

        self.core = core
        self.jobs_ctx = jobs_ctx
        self.event_bus = event_bus
        self.workspace_fs = None
        self.clipboard = None

        self.button_back.clicked.connect(self.back_clicked)
        self.button_back.apply_style()
        self.button_import_folder.clicked.connect(self.import_folder_clicked)
        self.button_import_folder.apply_style()
        self.button_import_files.clicked.connect(self.import_files_clicked)
        self.button_import_files.apply_style()
        self.button_create_folder.clicked.connect(self.create_folder_clicked)
        self.button_create_folder.apply_style()
        self.line_edit_search.textChanged.connect(self.filter_files)
        self.line_edit_search.hide()

        # Current directory ID can be `None`.
        # This means that the corresponding EntryID is still unkown since the
        # folder stat job has not returned the info yet. It could also be that
        # the job has failed to return properly, maybe because the directory
        # no longer exists
        self.current_directory: FsPath = FsPath("/")
        self.current_directory_id: Optional[EntryID] = None

        self.default_import_path = str(pathlib.Path.home())
        self.table_files.config = self.core.config
        self.table_files.file_moved.connect(self.on_table_files_file_moved)
        self.table_files.item_activated.connect(self.item_activated)
        self.table_files.rename_clicked.connect(self.rename_files)
        self.table_files.delete_clicked.connect(self.delete_files)
        self.table_files.open_clicked.connect(self.open_files)
        self.table_files.files_dropped.connect(self.on_files_dropped)
        self.table_files.show_history_clicked.connect(self.show_selected_file_history)
        self.table_files.show_status_clicked.connect(self.show_selected_file_status)
        self.table_files.paste_clicked.connect(self.on_paste_clicked)
        self.table_files.copy_clicked.connect(self.on_copy_clicked)
        self.table_files.cut_clicked.connect(self.on_cut_clicked)
        self.table_files.file_path_clicked.connect(self.on_get_file_path_clicked)
        self.table_files.file_path_timestamp_clicked.connect(
            self.on_get_file_path_timestamp_clicked
        )
        self.table_files.open_current_dir_clicked.connect(self.on_open_current_dir_clicked)
        self.table_files.itemSelectionChanged.connect(self._on_selection_changed)
        self.table_files.new_folder_clicked.connect(self.create_folder_clicked)
        self.table_files.sort_clicked.connect(self._on_sort_clicked)
        self.table_files.show_current_folder_history_clicked.connect(
            self.show_current_folder_history
        )
        self.table_files.show_current_folder_status_clicked.connect(self.show_current_folder_status)

        self.rename_success.connect(self._on_rename_success)
        self.rename_error.connect(self._on_rename_error)
        self.delete_success.connect(self._on_delete_success)
        self.delete_error.connect(self._on_delete_error)
        self.folder_stat_success.connect(self._on_folder_stat_success)
        self.folder_stat_error.connect(self._on_folder_stat_error)
        self.folder_create_success.connect(self._on_folder_create_success)
        self.folder_create_error.connect(self._on_folder_create_error)
        self.copy_success.connect(self._on_copy_success)
        self.copy_error.connect(self._on_copy_error)
        self.move_success.connect(self._on_move_success)
        self.move_error.connect(self._on_move_error)
        self.file_open_success.connect(self._on_file_open_success)
        self.file_open_error.connect(self._on_file_open_error)

        self.reload_timestamped_requested.connect(self._on_reload_timestamped_requested)
        self.reload_timestamped_success.connect(self._on_reload_timestamped_success)
        self.reload_timestamped_error.connect(self._on_reload_timestamped_error)

        self.event_bus.connect(CoreEvent.FS_ENTRY_SYNCED, self._on_fs_entry_synced)
        self.event_bus.connect(CoreEvent.FS_ENTRY_UPDATED, self._on_fs_entry_updated)
        self.event_bus.connect(CoreEvent.FS_ENTRY_DOWNSYNCED, self._on_fs_entry_downsynced)
        self.event_bus.connect(CoreEvent.SHARING_UPDATED, self._on_sharing_updated)

    def disconnect_all(self):
        pass

    def set_workspace_fs(
        self, wk_fs, current_directory=FsPath("/"), default_selection=None, clipboard=None
    ):
        self.table_files.clear()
        self.current_directory = current_directory
        self.workspace_fs = wk_fs
        self.load(current_directory)

        ws_entry = self.workspace_fs.get_workspace_entry()
        self.current_user_role = ws_entry.role
        self.label_role.setText(get_role_translation(self.current_user_role))
        self.table_files.current_user_role = self.current_user_role
        if self.current_user_role == WorkspaceRole.READER:
            self.button_import_folder.hide()
            self.button_import_files.hide()
            self.button_create_folder.hide()
        else:
            self.button_import_folder.show()
            self.button_import_files.show()
            self.button_create_folder.show()
        self.clipboard = clipboard
        if not self.clipboard:
            self.table_files.paste_status = PasteStatus(status=PasteStatus.Status.Disabled)
        else:
            if self.clipboard.source_workspace == self.workspace_fs:
                self.table_files.paste_status = PasteStatus(status=PasteStatus.Status.Enabled)
            else:
                # Sending the source_workspace name for paste text
                self.table_files.paste_status = PasteStatus(
                    status=PasteStatus.Status.Enabled,
                    source_workspace=self.clipboard.source_workspace.get_workspace_name().str,
                )
        self.reset(default_selection)

    def _on_sort_clicked(self, column):
        self.table_files.sortItems(column.value)

    def _on_selection_changed(self):
        selected_count = len(self.table_files.selected_files())
        file_count = max(self.table_files.rowCount() - 1, 0)
        if not file_count:
            self.label_elements.hide()
            return
        self.label_elements.show()
        if selected_count:
            self.label_elements.setText(
                _("TEXT_FILE_FOLDER_INFO_WITH_SELECTED_count-selected").format(
                    count=file_count, selected=selected_count
                )
            )
        else:
            self.label_elements.setText(
                _("TEXT_FILE_FOLDER_INFO_NO_SELECTED_count").format(count=file_count)
            )

    def reset(self, default_selection=None):
        workspace_name = self.workspace_fs.get_workspace_name()
        # Reload without any delay
        self.reload(default_selection, delay=0)
        self.table_files.sortItems(0)
        self.folder_changed.emit(workspace_name, str(self.current_directory))

    def on_get_file_path_clicked(self):
        files = self.table_files.selected_files()
        if len(files) != 1:
            return
        path = self.current_directory / files[0].name
        addr = self.workspace_fs.generate_file_link(path)
        desktop.copy_to_clipboard(addr.to_url())
        SnackbarManager.inform(_("TEXT_FILE_LINK_COPIED_TO_CLIPBOARD"))

    def on_get_file_path_timestamp_clicked(self):
        files = self.table_files.selected_files()
        if len(files) != 1:
            return
        path = self.current_directory / files[0].name
        ts_addr = self.workspace_fs.generate_file_link(path, timestamp=DateTime.now())
        desktop.copy_to_clipboard(ts_addr.to_url())
        SnackbarManager.inform(_("TEXT_FILE_LINK_COPIED_TO_CLIPBOARD"))

    def on_copy_clicked(self):
        files = self.table_files.selected_files()
        files_to_copy = []
        for f in files:
            if f.type != FileType.Folder and f.type != FileType.File:
                continue
            files_to_copy.append((self.current_directory / f.name, f.type))

        if self.clipboard and self.clipboard.status == Clipboard.Status.Cut:
            if (
                len(self.clipboard.files)
                and self.clipboard.files[0][0].parent == self.current_directory
            ):
                self.table_files.reset_cut_status([f[0].name.str for f in self.clipboard.files])

        self.clipboard = Clipboard(
            files=files_to_copy, status=Clipboard.Status.Copied, source_workspace=self.workspace_fs
        )
        self.global_clipboard_updated.emit(self.clipboard)
        self.table_files.paste_status = PasteStatus(status=PasteStatus.Status.Enabled)

    def on_cut_clicked(self):
        files = self.table_files.selected_files()
        files_to_cut = []
        rows = []

        for f in files:
            if f.type != FileType.Folder and f.type != FileType.File:
                continue
            rows.append(f.row)
            files_to_cut.append((self.current_directory / f.name, f.type))
        self.table_files.set_rows_cut(rows)
        self.clipboard = Clipboard(
            files=files_to_cut, status=Clipboard.Status.Cut, source_workspace=self.workspace_fs
        )
        self.global_clipboard_updated.emit(self.clipboard)
        self.table_files.paste_status = PasteStatus(status=PasteStatus.Status.Enabled)

    def on_paste_clicked(self):
        if not self.clipboard:
            return

        if self.clipboard.status == Clipboard.Status.Cut:
            self.jobs_ctx.submit_job(
                (self, "move_success"),
                (self, "move_error"),
                _do_move_files,
                workspace_fs=self.workspace_fs,
                target_dir=self.current_directory,
                source_files=self.clipboard.files,
                source_workspace=self.clipboard.source_workspace,
            )
            self.clipboard = None
            # Set Global clipboard to none too
            self.global_clipboard_updated.emit(None)
            self.table_files.paste_status = PasteStatus(status=PasteStatus.Status.Disabled)

        elif self.clipboard.status == Clipboard.Status.Copied:
            self.jobs_ctx.submit_job(
                (self, "copy_success"),
                (self, "copy_error"),
                _do_copy_files,
                workspace_fs=self.workspace_fs,
                target_dir=self.current_directory,
                source_files=self.clipboard.files,
                source_workspace=self.clipboard.source_workspace,
            )

    def _on_move_success(self, job):
        self.reset()

    def _on_move_error(self, job):
        if not getattr(job.exc, "params", None):
            return
        if isinstance(job.exc.params.get("last_exc", None), FSInvalidArgumentError):
            show_error(self, _("TEXT_FILE_FOLDER_MOVED_INTO_ITSELF_ERROR"))
        else:
            show_error(self, _("TEXT_FILE_PASTE_ERROR"))

        self.reset()

    def _on_copy_success(self, job):
        self.reset()

    def _on_copy_error(self, job):
        show_error(self, _("TEXT_FILE_PASTE_ERROR"))
        self.reset()

    def show_history(self, path):
        FileHistoryWidget.show_modal(
            jobs_ctx=self.jobs_ctx,
            workspace_fs=self.workspace_fs,
            path=path,
            reload_timestamped_signal=self.reload_timestamped_requested,
            update_version_list=self.update_version_list,
            close_version_list=self.close_version_list,
            core=self.core,
            parent=self,
            on_finished=None,
        )

    def show_selected_file_history(self):
        files = self.table_files.selected_files()
        if len(files) > 1:
            show_error(self, _("TEXT_FILE_HISTORY_MULTIPLE_FILES_SELECTED_ERROR"))
            return
        if len(files) == 0:
            show_error(self, _("TEXT_FILE_STATUS_NO_FILE_SELECTED_ERROR"))
            return
        selected_path = self.current_directory / files[0].name
        self.show_history(selected_path)

    def show_current_folder_history(self):
        self.show_history(self.current_directory)

    def show_status(self, path):
        FileStatusWidget.show_modal(
            jobs_ctx=self.jobs_ctx,
            workspace_fs=self.workspace_fs,
            path=path,
            core=self.core,
            parent=self,
            on_finished=None,
        )

    def show_selected_file_status(self):
        files = self.table_files.selected_files()
        if len(files) > 1:
            show_error(self, _("TEXT_FILE_STATUS_MULTIPLE_FILES_SELECTED_ERROR"))
            return
        selected_path = self.current_directory / files[0].name
        self.show_status(selected_path)

    def show_current_folder_status(self):
        self.show_status(self.current_directory)

    def rename_files(self):
        files = self.table_files.selected_files()
        if len(files) == 1:
            selection_end = files[0].name.find(".")
            # If no "." or starts with a ".", we want to select the whole file name
            if selection_end in [-1, 0]:
                selection_end = len(files[0].name)
            new_name = get_text_input(
                self,
                _("TEXT_FILE_RENAME_TITLE"),
                _("TEXT_FILE_RENAME_INSTRUCTIONS"),
                placeholder=_("TEXT_FILE_RENAME_PLACEHOLDER"),
                default_text=files[0].name,
                validator=validators.FileNameValidator(),
                button_text=_("ACTION_FILE_RENAME"),
                selection=(0, selection_end),
            )
            if not new_name:
                return
            self.jobs_ctx.submit_job(
                (self, "rename_success"),
                (self, "rename_error"),
                _do_rename,
                workspace_fs=self.workspace_fs,
                paths=[
                    (
                        self.current_directory / files[0].name,
                        self.current_directory / new_name,
                        files[0].entry_id,
                    )
                ],
            )
        else:
            new_name = get_text_input(
                self,
                _("TEXT_FILE_RENAME_MULTIPLE_TITLE_count").format(count=len(files)),
                _("TEXT_FILE_RENAME_MULTIPLE_INSTRUCTIONS_count").format(count=len(files)),
                placeholder=_("TEXT_FILE_RENAME_MULTIPLE_PLACEHOLDER"),
                validator=validators.FileNameValidator(),
                button_text=_("ACTION_FILE_RENAME_MULTIPLE"),
            )
            if not new_name:
                return

            self.jobs_ctx.submit_job(
                (self, "rename_success"),
                (self, "rename_error"),
                _do_rename,
                workspace_fs=self.workspace_fs,
                paths=[
                    (
                        self.current_directory / f.name,
                        self.current_directory
                        / "{}_{}{}".format(new_name, i, ".".join(pathlib.Path(f.name).suffixes)),
                        f.entry_id,
                    )
                    for i, f in enumerate(files, 1)
                ],
            )

    def delete_files(self):
        files = self.table_files.selected_files()
        if len(files) == 1:
            result = ask_question(
                self,
                _("TEXT_FILE_DELETE_TITLE"),
                _("TEXT_FILE_DELETE_INSTRUCTIONS_name").format(name=files[0].name),
                [_("ACTION_FILE_DELETE"), _("ACTION_CANCEL")],
            )
        else:
            result = ask_question(
                self,
                _("TEXT_FILE_DELETE_MULTIPLE_TITLE_count").format(count=len(files)),
                _("TEXT_FILE_DELETE_MULTIPLE_INSTRUCTIONS_count").format(count=len(files)),
                [_("ACTION_FILE_DELETE_MULTIPLE"), _("ACTION_CANCEL")],
            )
        if result != _("ACTION_FILE_DELETE_MULTIPLE") and result != _("ACTION_FILE_DELETE"):
            return
        self.jobs_ctx.submit_job(
            (self, "delete_success"),
            (self, "delete_error"),
            _do_delete,
            workspace_fs=self.workspace_fs,
            files=[(self.current_directory / f.name, f.type) for f in files],
        )

    def on_open_current_dir_clicked(self):
        self.desktop_open_files([None])

    def desktop_open_files(self, names: Iterable[Optional[str]]):

        paths = [
            self.core.mountpoint_manager.get_path_in_mountpoint(
                self.workspace_fs.workspace_id,
                self.current_directory / name if name else self.current_directory,
                self.workspace_fs.timestamp
                if isinstance(self.workspace_fs, WorkspaceFSTimestamped)
                else None,
            )
            for name in names
        ]
        self.jobs_ctx.submit_job(
            (self, "file_open_success"), (self, "file_open_error"), desktop.open_files_job, paths
        )

    def open_files(self):
        files = self.table_files.selected_files()
        if len(files) == 1:
            self.desktop_open_files([files[0].name])
        else:
            result = ask_question(
                self,
                _("TEXT_FILE_OPEN_MULTIPLE_TITLE_count").format(count=len(files)),
                _("TEXT_FILE_OPEN_MULTIPLE_INSTRUCTIONS_count").format(count=len(files)),
                [_("ACTION_FILE_OPEN_MULTIPLE"), _("ACTION_CANCEL")],
            )
            if result != _("ACTION_FILE_OPEN_MULTIPLE"):
                return
            self.desktop_open_files([f.name for f in files])

    def _on_file_open_success(self, job):
        status, paths = job.ret
        if not status:
            if len(paths) > 1:
                show_error(self, _("TEXT_FILE_OPEN_MULTIPLE_ERROR"))
            else:
                show_error(self, _("TEXT_FILE_OPEN_ERROR_file").format(file=paths[0].name))

    def _on_file_open_error(self, job):
        logger.error("Failed to open a file, should not happen")

    def item_activated(self, file_type, file_name):
        if file_type == FileType.ParentFolder:
            self.load(self.current_directory.parent)
        elif file_type == FileType.ParentWorkspace:
            self.back_clicked.emit()
        elif file_type == FileType.File:
            self.desktop_open_files([file_name])
        elif file_type == FileType.Folder:
            self.load(self.current_directory / file_name)

    def reload(self, default_selection=None, delay=RELOAD_FILES_LIST_THROTTLE_DELAY):
        self.jobs_ctx.submit_throttled_job(
            "files_widget.reload",
            delay,
            (self, "folder_stat_success"),
            (self, "folder_stat_error"),
            _do_folder_stat,
            workspace_fs=self.workspace_fs,
            path=self.current_directory,
            default_selection=default_selection,
        )

    def load(self, directory, default_selection=None):
        self.spinner.show()
        self.label_elements.hide()
        self.table_files.setEnabled(False)
        self.current_directory = directory
        self.current_directory_id = None
        self.jobs_ctx.submit_job(
            (self, "folder_stat_success"),
            (self, "folder_stat_error"),
            _do_folder_stat,
            workspace_fs=self.workspace_fs,
            path=directory,
            default_selection=default_selection,
        )

    # Import entry points

    def import_files_clicked(self):
        paths, filters = QDialogInProcess.getOpenFileNames(
            self, _("TEXT_FILE_IMPORT_FILES"), self.default_import_path
        )
        if not paths:
            return
        self.default_import_path = str(pathlib.Path(paths[0]).parent)
        self.jobs_ctx.submit_job(
            (self, "import_success"),
            (self, "import_error"),
            self._do_import,
            paths,
            self.current_directory,
        )

    def import_folder_clicked(self):
        path = QDialogInProcess.getExistingDirectory(
            self, _("TEXT_FILE_IMPORT_FOLDER"), self.default_import_path
        )
        if not path:
            return
        self.default_import_path = str(path)
        self.jobs_ctx.submit_job(
            (self, "import_success"),
            (self, "import_error"),
            self._do_import,
            [path],
            self.current_directory,
        )

    def on_files_dropped(self, sources, dest):
        if dest == "..":
            dest = self.current_directory.parent
        elif dest == ".":
            dest = self.current_directory
        else:
            dest = self.current_directory / dest
        self.jobs_ctx.submit_job(
            (self, "import_success"), (self, "import_error"), self._do_import, sources, dest
        )

    # Import job

    async def _do_import(self, sources: Iterable[str], dest: FsPath) -> None:
        # The file list is used in the error handler
        files = []
        try:
            # Get the list of files to import with the corresponding size
            files, total_size, errors = await self._get_files_from_sources(sources, dest)

            # Nothing to do
            if not files and not errors:
                return

            # Log errors
            for source, exc in errors:
                logger.exception(f"Could not load {source}", exc_info=exc)

            # No file to import, raise the first error
            if not files and errors:
                (_source, exc), *_more_errors = errors
                raise exc

            # Make the job cancellable
            with trio.CancelScope() as cancel_scope:

                # Create loading dialog and connect to the cancel scope
                wl = LoadingWidget(total_size=total_size + len(files))
                loading_dialog = GreyedDialog(wl, _("TEXT_FILE_IMPORT_LOADING_TITLE"), parent=self)
                wl.cancelled.connect(cancel_scope.cancel)

                # Control the visibility andl life-cyle of the loading dialog
                try:
                    loading_dialog.show()

                    # Actually perform the import
                    errors = await self._import_all(files, loading_dialog)

                finally:
                    loading_dialog.hide()
                    loading_dialog.setParent(None)

                # Process the errors
                if errors:
                    text = (
                        _("TEXT_FILE_IMPORT_ONE_PERMISSION_ERROR")
                        if len(files) == 1
                        else _("TEXT_FILE_IMPORT_MULTIPLE_PERMISSION_ERROR")
                    )
                    show_error(self, text, exception=errors[0])
                    raise JobResultError("error", exceptions=errors)

            if cancel_scope.cancel_called:
                raise JobResultError("cancelled")

        # Propagate job result errors
        except JobResultError:
            raise

        # Disk full
        except FSLocalStorageOperationalError as exc:
            text = _("TEXT_FILE_IMPORT_LOCAL_STORAGE_ERROR")
            show_error(self, text, exception=exc)
            raise JobResultError("error") from exc

        # Show a dialog when an unexpected error occurs
        except Exception as exc:
            text = (
                _("TEXT_FILE_IMPORT_ONE_ERROR")
                if len(files) == 1
                else _("TEXT_FILE_IMPORT_MULTIPLE_ERROR")
            )
            show_error(self, text, exception=exc)
            raise

    # Import async helpers

    async def _get_files(
        self, source: trio.Path, dest: FsPath
    ) -> Tuple[ImportFiles, int, ImportErrors]:
        try:
            # Source is a file
            if await source.is_file():
                stat = await source.stat()
                return [(source, dest / source.name)], stat.st_size, []

            # Neither a file or a directory, simply ignore
            if not await source.is_dir():
                return [], 0, []

            # Source is a directory
            files = []
            errors = []
            total_size = 0
            # Loop over children and aggregate
            for child in await source.iterdir():
                child_files, child_size, child_errors = await self._get_files(
                    child, dest / source.name
                )
                files.extend(child_files)
                errors.extend(child_errors)
                total_size += child_size
            return files, total_size, errors

        # Source is not available (e.g `PermissionDenied`)
        except OSError as exc:
            return [], 0, [(source, exc)]

    async def _get_files_from_sources(
        self, sources: Iterable[str], dest: FsPath
    ) -> Tuple[ImportFiles, int, ImportErrors]:
        files = []
        errors = []
        total_size = 0
        # Loop over sources and make sure to work with trio paths
        for source in sources:
            source = trio.Path(source)
            # Aggregate the results
            source_files, source_size, source_errors = await self._get_files(source, dest)
            files.extend(source_files)
            errors.extend(source_errors)
            total_size += source_size
        return files, total_size, errors

    async def _import_all(
        self, files: ImportFiles, loading_dialog: LoadingWidget
    ) -> List[PermissionError]:
        # Initialize current size
        errors = []
        current_size = 0
        loading_dialog.center_widget.set_progress(current_size)

        # Loop over files to import
        for source, dest in files:

            # Import the corresponding file and update the current size
            try:
                current_size = await self._import_one(source, dest, loading_dialog, current_size)

            # Register permission errors and keep going
            except PermissionError as exc:
                errors.append(exc)

            # The import failed for some other reason
            except (Exception, trio.Cancelled):
                # Shield against cancellation
                with trio.CancelScope(shield=True):
                    # Remove the last partially written file
                    to_delete = [(dest, FileType.File)]
                    await _do_delete(self.workspace_fs, to_delete, silent=True)
                raise

        # Return the permission errors
        return errors

    async def _import_one(
        self, source: trio.Path, dest: FsPath, loading_dialog: LoadingWidget, current_size: int
    ) -> int:
        # Update loading widget
        loading_dialog.center_widget.set_current_file(source.name)

        # Create parent directory
        if dest.parent != FsPath("/"):
            await self.workspace_fs.mkdir(dest.parent, parents=True, exist_ok=True)

        # Open the file to import
        async with await trio.open_file(source, "rb") as f:

            # Getting the file name without extension (anything after the first dot is considered to be the extension)
            name_we, *suffixes = dest.name.str.split(".")
            # Count starts at 2 (1 would be the file without a number)
            count = 2
            while await self.workspace_fs.exists(dest):
                # Create the new file name by adding the count ("myfile.txt" becomes "myfile (2).txt")
                new_file_name = EntryName(".".join([f"{name_we} ({count})", *suffixes]))
                dest = dest.parent / new_file_name
                count += 1

            # Open the file to create
            async with await self.workspace_fs.open_file(dest, "wb") as dest_file:

                # Loop over chunks of DEFAULT_BLOCK_SIZE, i.e. 512KB
                read_size = 0
                while True:

                    # Read a chunk and write it back
                    chunk = await f.read(DEFAULT_BLOCK_SIZE)
                    if not chunk:
                        break
                    await dest_file.write(chunk)

                    # Update the size
                    read_size += len(chunk)
                    loading_dialog.center_widget.set_progress(current_size + read_size)

        # Update and return the current size
        current_size += read_size + 1
        loading_dialog.center_widget.set_progress(current_size)
        return current_size

    def on_table_files_file_moved(self, file_type, file_name, target_name):
        src_path = self.current_directory / file_name
        target_dir = ""
        if target_name == "..":
            target_dir = self.current_directory.parent
        else:
            target_dir = self.current_directory / target_name
        self.jobs_ctx.submit_job(
            (self, "move_success"),
            (self, "move_error"),
            _do_move_files,
            workspace_fs=self.workspace_fs,
            target_dir=target_dir,
            source_files=[(src_path, file_type)],
            source_workspace=self.workspace_fs,
        )

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
        folder_name = get_text_input(
            self,
            _("TEXT_FILE_CREATE_FOLDER_TITLE"),
            _("TEXT_FILE_CREATE_FOLDER_INSTRUCTIONS"),
            placeholder=_("TEXT_FILE_CREATE_FOLDER_PLACEHOLDER"),
            button_text=_("ACTION_FILE_CREATE_FOLDER"),
            validator=validators.FileNameValidator(),
        )
        if not folder_name:
            return

        self.jobs_ctx.submit_job(
            (self, "folder_create_success"),
            (self, "folder_create_error"),
            _do_folder_create,
            workspace_fs=self.workspace_fs,
            path=self.current_directory / folder_name,
        )

    def keyPressEvent(self, event):
        if event.key() == Qt.Key_Delete:
            self.delete_files()

    def _on_rename_success(self, job):
        self.reset()

    def _on_rename_error(self, job):
        if not getattr(job.exc, "params", None):
            return
        if job.exc.params.get("multi"):
            show_error(self, _("TEXT_FILE_RENAME_MULTIPLE_ERROR"), exception=job.exc)
        else:
            show_error(self, _("TEXT_FILE_RENAME_ERROR"), exception=job.exc)

    def _on_delete_success(self, job):
        self.reset()

    def _on_delete_error(self, job):
        if not getattr(job.exc, "params", None):
            return
        if job.exc.params.get("multi"):
            show_error(self, _("TEXT_FILE_DELETE_MULTIPLE_ERROR"), exception=job.exc)
        else:
            show_error(self, _("TEXT_FILE_DELETE_ERROR"), exception=job.exc)

    def _on_folder_stat_success(self, job):
        # Extract job information
        directory, directory_id, files_stats, default_selection = job.ret
        # Ignore old refresh jobs
        if self.current_directory != directory:
            return
        # Set the EntryID
        first_refresh = self.current_directory_id is None
        self.current_directory_id = directory_id
        # Try to keep the current selection
        if first_refresh:
            old_selection = set()
        else:
            old_selection = [x.name for x in self.table_files.selected_files()]

        self.table_files.clear()
        old_sort = self.table_files.horizontalHeader().sortIndicatorSection()
        old_order = self.table_files.horizontalHeader().sortIndicatorOrder()
        self.table_files.setSortingEnabled(False)
        if self.current_directory == FsPath("/"):
            self.table_files.add_parent_workspace()
        else:
            self.table_files.add_parent_folder()
        file_found = False
        for entry_name, stats in files_stats.items():
            # Must check first given inconsistent stats result are missing fields
            if stats["type"] == "inconsistency":
                self.table_files.add_inconsistency(entry_name, stats["id"])
                continue
            selected = False
            confined = bool(stats["confinement_point"])
            if default_selection and entry_name == default_selection:
                selected = True
                file_found = True
            elif entry_name in old_selection:
                selected = True
            if stats["type"] == "folder":
                self.table_files.add_folder(
                    entry_name, stats["id"], not stats["need_sync"], confined, selected
                )
            else:
                self.table_files.add_file(
                    entry_name,
                    stats["id"],
                    stats["size"],
                    stats["created"],
                    stats["updated"],
                    not stats["need_sync"],
                    confined,
                    selected,
                )
        self.table_files.sortItems(old_sort, old_order)
        self.table_files.setSortingEnabled(True)
        if self.line_edit_search.text():
            self.filter_files(self.line_edit_search.text())
        if default_selection and not file_found:
            show_error(self, _("TEXT_FILE_GOTO_LINK_NOT_FOUND_file").format(file=default_selection))
        workspace_name = self.workspace_fs.get_workspace_name()
        self.spinner.hide()
        self.table_files.setEnabled(True)
        selected_count = len(self.table_files.selected_files())
        if selected_count:
            self.label_elements.setText(
                _("TEXT_FILE_FOLDER_INFO_WITH_SELECTED_count-selected").format(
                    count=len(files_stats), selected=selected_count
                )
            )
        else:
            self.label_elements.setText(
                _("TEXT_FILE_FOLDER_INFO_NO_SELECTED_count").format(count=len(files_stats))
            )
        self.label_elements.show()
        self.folder_changed.emit(workspace_name, str(self.current_directory))

    def _on_folder_stat_error(self, job):
        self.table_files.clear()
        self.spinner.hide()
        self.table_files.setEnabled(True)
        self.label_elements.show()
        self.label_elements.setText(_("TEXT_FILE_FOLDER_INFO_NO_SELECTED_count").format(count=0))
        if isinstance(job.exc, FSFileNotFoundError):
            show_error(self, _("TEXT_FILE_FOLDER_NOT_FOUND"))
            self.table_files.add_parent_workspace()
            return
        if self.current_directory == FsPath("/"):
            self.table_files.add_parent_workspace()
        else:
            self.table_files.add_parent_folder()

    def _on_folder_create_success(self, job):
        pass

    def _on_folder_create_error(self, job):
        if job.status == "already-exists":
            show_error(self, _("TEXT_FILE_FOLDER_CREATE_ERROR_ALREADY_EXISTS"))
        else:
            show_error(self, _("TEXT_FILE_FOLDER_CREATE_ERROR_UNKNOWN"))

    def _on_fs_entry_downsynced(self, event, workspace_id=None, id=None):
        # No workspace FS
        if not self.workspace_fs:
            return
        # Not the corresponding workspace
        if workspace_id != self.workspace_fs.workspace_id:
            return
        # Reload, as it might correspond to the current directory
        if self.current_directory_id is None:
            self.reload()
            return
        # Reload, as it definitely corresponds to the current directory
        if self.current_directory_id == id:
            self.reload()
            return

    def _on_fs_entry_synced(self, event, id, workspace_id=None):
        if not self.workspace_fs:
            return

        if self.current_directory_id == id:
            return

        self.table_files.set_file_status(id, synced=True, confined=False)

    def _on_fs_entry_updated(self, event, workspace_id=None, id=None):
        assert id is not None
        # No workspace FS
        if not self.workspace_fs:
            return
        # Not the corresponding workspace
        if workspace_id != self.workspace_fs.workspace_id:
            return
        # Reload, as it might correspond to the current directory
        if self.current_directory_id is None:
            self.reload()
            return
        # Reload, as it definitely corresponds to the current directory
        if self.current_directory_id == id:
            self.reload()
            return
        # Reload, as the id appears in the table files
        if self.table_files.has_file(id):
            self.reload()
            return

    def _on_sharing_updated(self, event, new_entry, previous_entry):
        if new_entry is None or new_entry.role is None:
            # Sharing revoked
            show_error(
                self, _("TEXT_FILE_SHARING_REVOKED_workspace").format(workspace=previous_entry.name)
            )
            self.back_clicked.emit()

        elif previous_entry is not None and previous_entry.role is not None:
            self.current_user_role = new_entry.role
            self.label_role.setText(get_role_translation(self.current_user_role))
            if (
                previous_entry.role != WorkspaceRole.READER
                and new_entry.role == WorkspaceRole.READER
            ):
                show_info(
                    self,
                    _("TEXT_FILE_SHARING_DEMOTED_TO_READER_workspace").format(
                        workspace=previous_entry.name
                    ),
                )

    def _on_reload_timestamped_requested(
        self, timestamp, path, file_type, open_after_load, close_after_remount, reload_after_remount
    ):
        self.jobs_ctx.submit_job(
            (self, "reload_timestamped_success"),
            (self, "reload_timestamped_error"),
            _do_remount_timestamped,
            mountpoint_manager=self.core.mountpoint_manager,
            workspace_fs=self.workspace_fs,
            timestamp=timestamp,
            path=path if path is not None else self.current_directory,
            file_type=file_type,
            open_after_load=open_after_load,
            close_after_load=close_after_remount,
            reload_after_remount=reload_after_remount,
        )

    def _on_reload_timestamped_success(self, job):
        (
            workspace_fs,
            path,
            file_type,
            open_after_load,
            close_after_load,
            reload_after_remount,
        ) = job.ret
        self.set_workspace_fs(workspace_fs, path.parent if file_type == FileType.File else path)
        # TODO : Select element if possible?
        if close_after_load:
            self.close_version_list.emit()
        if reload_after_remount:
            self.update_version_list.emit(self.workspace_fs, path)
        if open_after_load:
            self.desktop_open_files(path.name)

    def _on_reload_timestamped_error(self, job):
        raise job.exc
