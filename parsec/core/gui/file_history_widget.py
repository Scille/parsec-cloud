# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations
from typing import Callable, Tuple

from PyQt5.QtCore import pyqtBoundSignal, pyqtSignal
from PyQt5.QtWidgets import QWidget

from parsec._parsec import DateTime
from parsec.api.protocol.types import UserID
from parsec.core.fs import FsPath, WorkspaceFS
from parsec.core.gui.file_items import Optional
from parsec.core.logged_core import LoggedCore
from parsec.core.gui.trio_jobs import QtToTrioJob, QtToTrioJobScheduler
from parsec.core.fs.workspacefs.versioning_helpers import TimestampBoundedData, VersionLister
from parsec.core.gui.lang import translate as _, format_datetime
from parsec.core.gui.custom_dialogs import show_error, GreyedDialog
from parsec.core.gui.file_size import get_filesize
from parsec.core.gui.ui.file_history_widget import Ui_FileHistoryWidget
from parsec.core.gui.ui.file_history_button import Ui_FileHistoryButton
from parsec.core.types import UserInfo


async def _do_workspace_version(
    version_lister: VersionLister, path: FsPath, core: LoggedCore
) -> Tuple[list[Tuple[UserInfo, TimestampBoundedData]], bool]:
    versions_list, download_limit_reached = await version_lister.list(
        path, max_manifest_queries=100
    )

    _cache: dict[UserID, UserInfo] = {}

    async def get_user_info(user_id: UserID) -> UserInfo:
        return _cache.setdefault(user_id, await core.get_user_info(user_id))

    return (
        [(await get_user_info(v.creator.user_id), v) for v in versions_list],
        download_limit_reached,
    )
    # TODO : check no exception raised, create tests...


class FileHistoryButton(QWidget, Ui_FileHistoryButton):
    def __init__(
        self,
        version: str,
        creator: str,
        name: FsPath,
        size: int,
        src: FsPath,
        dst: FsPath,
        timestamp: DateTime,
    ) -> None:
        super().__init__()
        self.setupUi(self)
        self.label_version.setText(str(version))
        self.label_user.setText(creator)
        self.label_size.setText(get_filesize(size) if size is not None else "")
        self.label_date.setText(format_datetime(timestamp))
        if not src:
            self.label_src.hide()
        else:
            self.label_src.setText(str(src))
        if not dst:
            self.label_dst.hide()
        else:
            self.label_dst.setText(str(dst))


class FileHistoryWidget(QWidget, Ui_FileHistoryWidget):
    get_versions_success = pyqtSignal(QtToTrioJob)
    get_versions_error = pyqtSignal(QtToTrioJob)

    def __init__(
        self,
        jobs_ctx: QtToTrioJobScheduler,
        workspace_fs: WorkspaceFS,
        path: FsPath,
        reload_timestamped_signal: pyqtBoundSignal,
        update_version_list: pyqtBoundSignal,
        close_version_list: pyqtBoundSignal,
        core: LoggedCore,
    ) -> None:
        super().__init__()
        self.setupUi(self)
        self.jobs_ctx = jobs_ctx
        self.dialog: Optional[GreyedDialog] = None
        self.core = core
        update_version_list.connect(self.reset_dialog)
        self.get_versions_success.connect(self.on_get_version_success)
        self.get_versions_error.connect(self.on_get_version_error)
        self.button_load_more_entries.clicked.connect(self.load_more)
        self.workspace_fs = workspace_fs
        self.version_lister = workspace_fs.get_version_lister()
        self.set_loading_in_progress(False)
        self.reset_dialog(workspace_fs, self.version_lister, path)

    def set_loading_in_progress(self, in_progress: bool) -> None:
        self.loading_in_progress = in_progress
        self.area_list.setVisible(not in_progress)
        self.spinner.setVisible(in_progress)

    def reset_dialog(
        self, workspace_fs: WorkspaceFS, version_lister: VersionLister, path: FsPath
    ) -> None:
        if self.loading_in_progress:
            return
        self.set_loading_in_progress(True)
        self.workspace_fs = workspace_fs
        self.path = path
        self.reset_list()

    def load_more(self) -> None:
        if self.loading_in_progress:
            return
        self.set_loading_in_progress(True)
        self.reset_list()

    def reset_list(self) -> None:
        while self.layout_history.count() != 0:
            item = self.layout_history.takeAt(0)
            if item:
                w = item.widget()
                self.layout_history.removeWidget(w)
                w.hide()
                w.setParent(0)
        self.versions_job = self.jobs_ctx.submit_job(
            (self, "get_versions_success"),
            (self, "get_versions_error"),
            _do_workspace_version,
            version_lister=self.version_lister,
            path=self.path,
            core=self.core,
        )

    def add_history_item(
        self,
        version: str,
        path: FsPath,
        creator: str,
        size: int,
        timestamp: DateTime,
        src_path: FsPath,
        dst_path: FsPath,
    ) -> None:
        button = FileHistoryButton(
            version=version,
            creator=creator,
            name=path,
            size=size,
            src=src_path,
            dst=dst_path,
            timestamp=timestamp,
        )
        self.layout_history.addWidget(button)
        button.show()

    def on_get_version_success(self, job: QtToTrioJob) -> None:
        assert self.versions_job is not None
        assert self.versions_job.ret is not None
        versions_list, download_limit_reached = self.versions_job.ret
        if download_limit_reached:
            self.button_load_more_entries.setVisible(False)
        self.versions_job = None
        for author, version in versions_list:
            self.add_history_item(
                version=version.version,
                path=self.path,
                creator=author.short_user_display,
                size=version.size,
                timestamp=version.early,
                src_path=version.source,
                dst_path=version.destination,
            )
        self.set_loading_in_progress(False)

    def on_get_version_error(self, job: QtToTrioJob) -> None:
        if self.versions_job and self.versions_job.status != "cancelled":
            show_error(self, _("TEXT_FILE_HISTORY_LIST_FAILURE"), exception=self.versions_job.exc)
        self.versions_job = None
        assert self.dialog is not None
        self.dialog.reject()

    def on_close(self) -> None:
        if self.versions_job:
            self.versions_job.cancel()

    @classmethod
    def show_modal(  # type: ignore[misc]
        cls,
        jobs_ctx: QtToTrioJobScheduler,
        workspace_fs: WorkspaceFS,
        path: FsPath,
        reload_timestamped_signal: pyqtBoundSignal,
        update_version_list: pyqtBoundSignal,
        close_version_list: pyqtBoundSignal,
        core: LoggedCore,
        parent: QWidget,
        on_finished: Optional[Callable[..., None]],
    ) -> FileHistoryWidget:
        w = cls(
            jobs_ctx=jobs_ctx,
            workspace_fs=workspace_fs,
            path=path,
            reload_timestamped_signal=reload_timestamped_signal,
            update_version_list=update_version_list,
            close_version_list=close_version_list,
            core=core,
        )
        d = GreyedDialog(
            w,
            title=_("TEXT_FILE_HISTORY_TITLE_name").format(
                name=path.name if not path.is_root() else workspace_fs.get_workspace_name().str
            ),
            parent=parent,
        )
        w.dialog = d
        if on_finished:
            d.finished.connect(on_finished)
        # Unlike exec_, show is asynchronous and works within the main Qt loop
        d.show()
        return w
