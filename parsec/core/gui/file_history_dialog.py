# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import Qt, pyqtSignal
from PyQt5.QtWidgets import QDialog
from PyQt5.QtSvg import QSvgWidget

from parsec.core.gui.lang import translate as _
from parsec.core.gui.custom_dialogs import show_error
from parsec.core.gui.trio_thread import ThreadSafeQtSignal
from parsec.core.gui.ui.file_history_dialog import Ui_FileHistoryDialog


async def _do_workspace_version(version_lister, path):
    return await version_lister.list(path, max_manifest_queries=100)
    # TODO : check no exception raised, create tests...


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
        self.get_versions_success.connect(self.on_get_version_success)
        self.get_versions_error.connect(self.on_get_version_error)
        self.button_close.clicked.connect(self.close_dialog)
        self.button_load_more_entries.clicked.connect(self.load_more)
        self.workspace_fs = workspace_fs
        self.version_lister = workspace_fs.get_version_lister()
        self.spinner = QSvgWidget(":/icons/images/icons/spinner.svg")
        self.spinner.setFixedSize(100, 100)
        self.spinner_frame.setLayout(self.spinner_layout)
        self.spinner_layout.addWidget(self.spinner, Qt.AlignCenter)
        self.spinner_layout.setAlignment(Qt.AlignCenter)
        self.set_loading_in_progress(False)
        self.reset_dialog(workspace_fs, self.version_lister, path)

    def set_loading_in_progress(self, in_progress: bool):
        self.loading_in_progress = in_progress
        self.versions_table.setVisible(not in_progress)
        self.spinner_frame.setVisible(in_progress)

    def reset_dialog(self, workspace_fs, version_lister, path):
        if self.loading_in_progress:
            return
        self.set_loading_in_progress(True)
        file_name = path.name
        if len(file_name) > 64:
            file_name = file_name[:64] + "..."
        self.label_file_name.setText(f'"{file_name}"')
        self.workspace_fs = workspace_fs
        self.path = path
        self.reset_list()

    def load_more(self):
        if self.loading_in_progress:
            return
        self.set_loading_in_progress(True)
        self.reset_list()

    def reset_list(self):
        self.versions_table.clear()
        self.versions_job = self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "get_versions_success"),
            ThreadSafeQtSignal(self, "get_versions_error"),
            _do_workspace_version,
            version_lister=self.version_lister,
            path=self.path,
        )

    def on_get_version_success(self):
        versions_list, download_limit_reached = self.versions_job.ret
        if download_limit_reached:
            self.button_load_more_entries.setVisible(False)
        self.versions_job = None
        for v in versions_list:
            self.versions_table.add_item(
                entry_id=v.id,
                version=v.version,
                actual_path=self.path,
                is_folder=v.is_folder,
                creator=v.creator,
                size=v.size,
                early_timestamp=v.early,
                late_timestamp=v.late,
                source_path=v.source,
                destination_path=v.destination,
            )
        self.set_loading_in_progress(False)

    def on_get_version_error(self):
        if self.versions_job and self.versions_job.status != "cancelled":
            show_error(self, _("ERR_LIST_VERSIONS_ACCESS"), exception=self.versions_job.exc)
        self.versions_job = None
        self.reject()

    def close_dialog(self):
        if self.versions_job:
            self.versions_job.cancel_and_join()
        else:
            self.reject()
