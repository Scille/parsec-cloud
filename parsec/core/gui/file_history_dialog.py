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

    def __init__(
        self,
        jobs_ctx,
        workspace_fs,
        path,
        reload_timestamped_signal,
        update_version_list,
        close_version_list,
        *args,
        **kwargs,
    ):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.jobs_ctx = jobs_ctx
        self.versions_table.set_reload_timestamped_signal(reload_timestamped_signal)
        update_version_list.connect(self.reset_dialog)
        close_version_list.connect(self.close_dialog)
        self.setWindowFlags(Qt.SplashScreen)
        self.get_versions_success.connect(self.add_history)
        self.get_versions_error.connect(self.show_error)
        self.button_close.clicked.connect(self.close_dialog)
        self.workspace_fs = workspace_fs
        self.reset_dialog(workspace_fs, path)

    def reset_dialog(self, workspace_fs, path):
        self.label_file_name.setText(f'"{path.name}"')
        self.workspace_fs = workspace_fs
        self.path = path
        self.versions_table.clear()
        self.versions_job = self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "get_versions_success"),
            ThreadSafeQtSignal(self, "get_versions_error"),
            _do_workspace_version,
            workspace_fs=self.workspace_fs,
            path=path,
        )

    def add_history(self):
        versions_dict = self.versions_job.ret
        self.versions_job = None
        if not versions_dict:
            return  # TODO : error before?
        for k, v in versions_dict.items():
            self.versions_table.add_item(
                entry_id=k[0],
                version=k[1],
                actual_path=self.path,
                is_folder=v[0][2],
                creator=v[0][0],
                size=v[0][3],
                early_timestamp=k[2],
                late_timestamp=k[3],
                source_path=v[1],
                destination_path=v[2],
            )

    def show_error(self):
        if self.versions_job and self.versions_job.status != "cancelled":
            show_error(self, _("ERR_LIST_VERSIONS_ACCESS"), exception=self.versions_job.exc)
        self.versions_job = None
        self.reject()

    def close_dialog(self):
        if self.versions_job:
            self.versions_job.cancel_and_join()
        else:
            self.reject()
