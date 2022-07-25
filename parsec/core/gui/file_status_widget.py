# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget

from parsec.core.fs import WorkspaceFSTimestamped
from parsec.core.types import DEFAULT_BLOCK_SIZE
from parsec.core.fs.workspacefs.entry_transactions import BlockInfo
from parsec.core.gui.custom_dialogs import GreyedDialog
from parsec.core.gui.file_size import get_filesize
from parsec.core.gui.trio_jobs import QtToTrioJob
from parsec.core.gui.ui.file_status_widget import Ui_FileInfoWidget
from parsec.core.gui.lang import translate as _, format_datetime


class FileStatusWidget(QWidget, Ui_FileInfoWidget):

    get_status_success = pyqtSignal(QtToTrioJob)
    get_status_error = pyqtSignal(QtToTrioJob)

    def __init__(self, jobs_ctx, workspace_fs, path, core):
        super().__init__()
        self.setupUi(self)
        self.jobs_ctx = jobs_ctx
        self.workspace_fs = workspace_fs
        self.path = path
        self.core = core

        self.jobs_ctx.submit_job(self.get_status_success, self.get_status_error, self.get_status)

    async def get_status(self):
        path_info = await self.workspace_fs.path_info(self.path)
        block_info = None
        if path_info["type"] == "file":
            block_info: BlockInfo = await self.workspace_fs.get_blocks_by_type(self.path)
            self.label_size.setText(get_filesize(path_info["size"]))

        version_lister = self.workspace_fs.get_version_lister()

        version_list = await version_lister.list(path=self.path)
        user_id = version_list[0][0].creator.user_id

        created_time = version_list[0][0].updated
        updated_time = path_info["updated"]

        version_list = await version_lister.list(path=self.path, starting_timestamp=updated_time)
        user_id_last = version_list[0][-1].creator.user_id

        creator = await self.core.get_user_info(user_id)
        last_author = await self.core.get_user_info(user_id_last)

        full_path = self.core.mountpoint_manager.get_path_in_mountpoint(
            self.workspace_fs.workspace_id,
            self.path,
            self.workspace_fs.timestamp
            if isinstance(self.workspace_fs, WorkspaceFSTimestamped)
            else None,
        )

        self.label_location.setText(str(full_path))
        self.label_filetype.setText(str(path_info["type"]))

        self.label_workspace.setText(self.workspace_fs.get_workspace_name().str)
        self.label_created_on.setText(format_datetime(created_time))
        self.label_last_updated_on.setText(format_datetime(updated_time))
        self.label_created_by.setText(creator.short_user_display)
        self.label_last_updated_by.setText(last_author.short_user_display)

        if block_info:
            local_blocks = len(block_info.local_only_blocks)
            remote_blocks = len(block_info.remote_only_blocks)
            local_and_remote_blocks = len(block_info.local_and_remote_blocks)
            total_blocks = local_blocks + remote_blocks + local_and_remote_blocks

            local_block_count = local_blocks + local_and_remote_blocks
            local_block_percentage = int(
                (local_block_count / total_blocks if total_blocks else 1) * 100.0
            )
            remote_block_count = remote_blocks + local_and_remote_blocks
            remote_block_percentage = int(
                (remote_block_count / total_blocks if total_blocks else 1) * 100.0
            )

            if local_block_count == total_blocks:
                self.label_availability.setText(_("TEXT_YES"))
            else:
                self.label_availability.setText(_("TEXT_NO"))

            if remote_block_count == total_blocks:
                self.label_uploaded.setText(_("TEXT_YES"))
            else:
                self.label_uploaded.setText(_("TEXT_NO"))

            self.label_local.setText(
                f"{local_block_count}/{total_blocks} ({local_block_percentage}%)"
            )
            self.label_remote.setText(
                f"{remote_block_count}/{total_blocks} ({remote_block_percentage}%)"
            )
            self.label_default_block_size.setText(get_filesize(DEFAULT_BLOCK_SIZE))

    @classmethod
    def show_modal(cls, jobs_ctx, workspace_fs, path, core, parent, on_finished):
        w = cls(jobs_ctx=jobs_ctx, workspace_fs=workspace_fs, path=path, core=core)
        d = GreyedDialog(
            w,
            title=_("TEXT_FILE_STATUS_TITLE_name").format(
                name=path.name if not path.is_root() else workspace_fs.get_workspace_name().str
            ),
            parent=parent,
            width=1000,
        )
        w.dialog = d
        if on_finished:
            d.finished.connect(on_finished)
        # Unlike exec_, show is asynchronous and works within the main Qt loop
        d.show()
        return w
