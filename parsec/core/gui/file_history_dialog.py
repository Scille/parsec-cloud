# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import Qt, pyqtSignal
from PyQt5.QtWidgets import QDialog

from parsec.core.gui.lang import translate as _
from parsec.core.gui.custom_dialogs import show_error
from parsec.core.gui.trio_thread import ThreadSafeQtSignal
from parsec.core.gui.ui.file_history_dialog import Ui_FileHistoryDialog


async def _do_workspace_version(workspace_fs, path):
    return await workspace_fs.versions(path)


class FileHistoryDialog(QDialog, Ui_FileHistoryDialog):
    get_versions_success = pyqtSignal()
    get_versions_error = pyqtSignal()

    def __init__(self, jobs_ctx, workspace_fs, path, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.jobs_ctx = jobs_ctx
        self.path = path
        self.setWindowFlags(Qt.SplashScreen)
        self.label_file_name.setText(path.name)
        self.get_versions_success.connect(self.add_history)
        self.get_versions_error.connect(self.show_error)
        self.versions_job = self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "get_versions_success"),
            ThreadSafeQtSignal(self, "get_versions_error"),
            _do_workspace_version,
            workspace_fs=workspace_fs,
            path=path,
        )
        # TODO : cancellable

    def add_history(self):
        versions_dict = self.versions_job.ret
        if not versions_dict:
            return  # TODO : something something before
        for k, v in versions_dict.items():
            self.versions_table.addItem(
                k[0], k[1], self.path, v[0][2], v[0][0], v[0][3], k[2], k[3], v[1], v[2]
            )

    def show_error(self):
        if self.versions_job and self.versions_job.status != "cancelled":
            show_error(self, _("ERR_LIST_VERSIONS_ACCESS"), exception=self.versions_job.exc)
        self.versions_job = None
        self.reject()
