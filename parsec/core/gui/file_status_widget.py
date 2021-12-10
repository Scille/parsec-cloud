# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS
from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget

from parsec.core.fs.workspacefs.entry_transactions import BlockInfo
from parsec.core.gui.custom_dialogs import GreyedDialog
from parsec.core.gui.trio_jobs import QtToTrioJob
from parsec.core.gui.ui.file_status_widget import Ui_FileInfoWidget
from parsec.core.gui.lang import translate as _


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
        self.get_status_success.connect(self.on_get_status_success)
        self.get_status_error.connect(self.on_get_status_error)

        self.jobs_ctx.submit_job(self.get_status_success, self.get_status_error, self.get_status)

    def on_get_status_success(self):
        pass

    def on_get_status_error(self):
        pass

    async def get_status(self):
        block_info: BlockInfo = await self.workspace_fs.get_blocks_by_type(self.path)
        path_info = await self.workspace_fs.path_info(self.path)
        version_lister = self.workspace_fs.get_version_lister()
        version1 = await version_lister.list(path=self.path)
        user_id = version1[0][0].creator.user_id
        user_id_last = version1[0][-1].creator.user_id
        creator = await self.core.get_user_info(user_id)
        creator_last = await self.core.get_user_info(user_id_last)
        from structlog import get_logger

        logger = get_logger()
        self.label_location.setText(str(self.path))
        self.label_filetype.setText(str(path_info["type"]))
        self.label_size.setText(str(path_info["size"]))
        self.label_workspace.setText(self.workspace_fs.get_workspace_name())
        self.label_created_on.setText(str(path_info["created"]))
        self.label_last_updated_on.setText(str(path_info["updated"]))
        self.label_created_by.setText(creator.short_user_display)
        self.label_last_updated_by.setText(creator_last.short_user_display)

        local_blocks = str(len(block_info.local_only_blocks))
        remote_blocks = str(len(block_info.remote_only_blocks))
        local_and_remote_blocks = str(len(block_info.local_and_remote_blocks))

        self.label_blocks.setText(f"{local_blocks} |  {remote_blocks} | {local_and_remote_blocks}")
        if block_info.pending_chunks_size == 0:
            self.label_availability.setText("Yes")
        else:
            self.label_availability.setText("No")

        # other info if needed
        logger.warning("Block size:" + str(block_info.proper_blocks_size))
        logger.warning("Pending Chunk size:" + str(block_info.pending_chunks_size))

    @classmethod
    def show_modal(cls, jobs_ctx, workspace_fs, path, core, parent, on_finished):
        w = cls(jobs_ctx=jobs_ctx, workspace_fs=workspace_fs, path=path, core=core)
        d = GreyedDialog(
            w, title=_("TEXT_FILE_STATUS_TITLE_name").format(name=path.name), parent=parent
        )
        w.dialog = d
        if on_finished:
            d.finished.connect(on_finished)
        # Unlike exec_, show is asynchronous and works within the main Qt loop
        d.show()
        return w
