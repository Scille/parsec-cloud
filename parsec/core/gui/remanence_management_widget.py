# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

from PyQt5.QtCore import QSignalBlocker
from PyQt5.QtWidgets import QWidget

from parsec.core.fs import WorkspaceFS
from parsec.core.gui.custom_dialogs import GreyedDialog, ask_question, show_info
from parsec.core.gui.lang import translate
from parsec.core.gui.trio_jobs import QtToTrioJobScheduler
from parsec.core.gui.ui.remanence_management_widget import Ui_RemanenceManagementWidget


class RemanenceManagementWidget(QWidget, Ui_RemanenceManagementWidget):
    def __init__(self, jobs_ctx: QtToTrioJobScheduler, workspace_fs: WorkspaceFS):
        super().__init__()
        self.setupUi(self)
        self.jobs_ctx = jobs_ctx
        self.workspace_fs = workspace_fs
        self.dialog: GreyedDialog | None = None
        self.check_remanence_state.toggled.connect(self._on_remanence_state_changed)
        self.progress_bar.setVisible(False)
        self.waiting_widget.setVisible(False)
        self.button_close_dialog.clicked.connect(self._on_close_clicked)

    def _on_remanence_state_changed(self, state: bool) -> None:
        if state:
            self.jobs_ctx.submit_job(None, None, self._enable_data_remanence)
        else:
            answer = ask_question(
                self,
                "Test",
                "This will delete the local cache blabla are you sure",
                [translate("ACTION_YES"), translate("ACTION_NO")],
            )
            if answer == translate("ACTION_NO"):
                with QSignalBlocker(self.check_remanence_state):
                    self.check_remanence_state.setChecked(not state)
                return
            self.jobs_ctx.submit_job(None, None, self._disable_data_remanence)

    def _toggle_processing(self, is_processing: bool) -> None:
        self.button_close_dialog.setEnabled(not is_processing)
        self.check_remanence_state.setEnabled(not is_processing)
        self.waiting_widget.setVisible(is_processing)
        if self.dialog:
            self.dialog.button_close.setVisible(not is_processing)

    def _on_close_clicked(self) -> None:
        if self.dialog:
            self.dialog.accept()

    async def _disable_data_remanence(self) -> None:
        self.waiting_label.setText("Please wait while we delete the cache.")
        self._toggle_processing(True)
        # do things
        self._toggle_processing(False)
        show_info(self, "Cache deleted")

    async def _enable_data_remanence(self) -> None:
        self._toggle_processing(True)
        # get infos
        self._toggle_processing(False)
        answer = ask_question(
            self,
            "Test",
            "This will take a lot of Gb, you sure?",
            [translate("ACTION_YES"), translate("ACTION_NO")],
        )
        if answer == translate("ACTION_NO"):
            with QSignalBlocker(self.check_remanence_state):
                self.check_remanence_state.setChecked(False)
            return
        self.waiting_label.setText("Please wait while we download missing blocks")
        self._toggle_processing(True)
        self.progress_bar.setValue(0)
        self.progress_bar.setVisible(True)
        # download
        self.progress_bar.setValue(100)
        self.progress_bar.setVisible(False)
        self._toggle_processing(False)
        show_info(self, "Data downloaded")

    @classmethod
    def show_modal(
        cls, jobs_ctx: QtToTrioJobScheduler, workspace_fs: WorkspaceFS, parent: QWidget
    ) -> None:
        widget = cls(jobs_ctx, workspace_fs)
        dialog = GreyedDialog(
            widget,
            "Data remanence management for workspace {WorkspaceName}",
            parent=parent,
            width=1000,
            close_on_click=False,
        )
        widget.dialog = dialog

        # Unlike exec_, show is asynchronous and works within the main Qt loop
        dialog.show()
