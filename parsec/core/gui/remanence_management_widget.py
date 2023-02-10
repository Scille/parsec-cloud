# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

from contextlib import contextmanager
from typing import Iterator

import trio
from PyQt5.QtWidgets import QWidget

from parsec.core.fs import WorkspaceFS
from parsec.core.gui.custom_dialogs import GreyedDialog, show_error
from parsec.core.gui.file_size import get_filesize
from parsec.core.gui.lang import translate as T
from parsec.core.gui.snackbar_widget import SnackbarManager
from parsec.core.gui.switch_button import SwitchButton
from parsec.core.gui.trio_jobs import QtToTrioJob, QtToTrioJobScheduler
from parsec.core.gui.ui.remanence_management_widget import Ui_RemanenceManagementWidget
from parsec.core.logged_core import LoggedCore


class RemanenceManagementWidget(QWidget, Ui_RemanenceManagementWidget):
    def __init__(self, jobs_ctx: QtToTrioJobScheduler, core: LoggedCore, workspace_fs: WorkspaceFS):
        super().__init__()
        self.setupUi(self)

        self.core = core
        self.jobs_ctx = jobs_ctx
        self.workspace_fs = workspace_fs
        self.dialog: GreyedDialog[RemanenceManagementWidget]
        self._download_started: bool = False
        self.switch_button = SwitchButton()
        self.switch_layout.insertWidget(1, self.switch_button)

        # Connect signals
        self.switch_button.clicked.connect(self._on_switch_clicked)
        self.button_close_dialog.clicked.connect(self._on_close_clicked)

        # Set waiting state
        self._update_text()
        self._update_switch_button(disabled=True)
        self.wait_prepared_job: QtToTrioJob[None] = self.jobs_ctx.submit_job(
            None, None, self._wait_prepared
        )

    @contextmanager
    def _handle_error(self, message: str) -> Iterator[None]:
        try:
            yield
        except Exception as exc:
            if self.dialog.isVisible():
                show_error(self, message, exc)
            self.dialog.accept()
            return

    def _update_switch_button(self, disabled: bool = False) -> None:
        self.switch_button.setDisabled(disabled)
        self.switch_button.setChecked(self.workspace_fs.is_block_remanent())

    def _update_text(self) -> None:
        # Do not update while we're performing an operation
        if not self.switch_button.isEnabled():
            return
        info = self.workspace_fs.get_remanence_manager_info()
        # Label info label
        if info.is_block_remanent:
            self.label_info.setText(T("TEXT_BLOCK_REMANENCE_ENABLED"))
        else:
            cache_size = get_filesize(self.core.config.workspace_storage_cache_size)
            self.label_info.setText(
                T("TEXT_BLOCK_REMANENCE_DISABLED_cache_size").format(cache_size=cache_size)
            )
        # Warning label
        if not info.is_block_remanent and info.is_prepared and info.remote_only_size > 0:
            self.label_warning.setText(
                T("TEXT_BLOCK_REMANENCE_WARNING_remote_size").format(
                    remote_size=get_filesize(info.remote_only_size)
                )
            )
            self.label_warning.show()
        else:
            self.label_warning.hide()
        # Close button
        if info.is_block_remanent and info.is_prepared and info.remote_only_size > 0:
            self.button_close_dialog.setText(T("ACTION_CONTINUE_IN_BACKGROUND"))
        else:
            self.button_close_dialog.setText(T("ACTION_CLOSE"))
        # Progress bar
        self.progress_bar.show()
        self.progress_bar.setEnabled(info.is_block_remanent)
        if not info.is_prepared:
            self.progress_bar.setMinimum(0)
            self.progress_bar.setMaximum(0)
            self.progress_bar.setValue(0)
        elif info.total_size == 0:
            self.progress_bar.setFormat(T("TEXT_WORKSPACE_IS_EMPTY"))
            self.progress_bar.setMinimum(0)
            self.progress_bar.setMaximum(1)
            self.progress_bar.setValue(1)
        else:
            title = (
                T("TEXT_REMANENCE_PROGRESS_BAR_TITLE_DOWNLOADING")
                if info.is_block_remanent and info.remote_only_size
                else T("TEXT_REMANENCE_PROGRESS_BAR_TITLE_IDLE")
            )
            format_value = "{} - {} / {} (%p%)".format(
                title, get_filesize(info.local_and_remote_size), get_filesize(info.total_size)
            )
            self.progress_bar.setFormat(format_value)
            self.progress_bar.setMinimum(0)
            self.progress_bar.setMaximum(info.total_size)
            self.progress_bar.setValue(info.local_and_remote_size)
        # Congratulate when a download is finished
        if (
            self._download_started
            and info.is_prepared
            and info.total_size > 0
            and info.remote_only_size == 0
        ):
            self._download_started = False
            SnackbarManager.congratulate(T("TEXT_BLOCK_DOWNLOADING_FINISHED"))

    async def _wait_prepared(self) -> None:
        with self._handle_error(T("TEXT_GET_REMANENCE_STATUS_FAILED")):
            await self.workspace_fs.remanence_manager.wait_prepared()
            # The dialog has been closed
            if not self.dialog.isVisible():
                return
            # Update content
            self._update_text()
            self._update_switch_button()
            # Update text periodically
            while True:
                # The dialog has been closed
                if not self.dialog.isVisible():
                    return
                # Refresh content
                self._update_text()
                # Update 2 times per seconds
                await trio.sleep(0.5)

    async def _on_switch_clicked(self, state: bool) -> None:
        if state:
            await self._enable_block_remanence()
        else:
            await self._disable_block_remanence()

    async def _enable_block_remanence(self) -> None:
        # Close the dialog when an error occurs
        with self._handle_error(T("TEXT_ENABLE_BLOCK_REMANENCE_FAILED")):
            # Update widgets
            self.switch_button.setDisabled(True)
            self.label_warning.hide()
            # Enable block remanence
            await self.workspace_fs.enable_block_remanence()
            # Update content
            self._update_switch_button()
            self._update_text()
            info = self.workspace_fs.get_remanence_manager_info()
            if not info.is_prepared or info.remote_only_size == 0:
                # No data to download
                SnackbarManager.congratulate(T("TEXT_ENABLE_BLOCK_REMANENCE_SUCCESS"))
            else:
                # Some data needs to be downloaded
                SnackbarManager.inform(T("TEXT_BLOCK_DOWNLOADING_STARTED"))
                self._download_started = True

    async def _disable_block_remanence(self, *_: object) -> None:
        # Close the dialog when a error occurs
        with self._handle_error(T("TEXT_DISABLE_BLOCK_REMANENCE_FAILED")):
            # Update widgets
            self.switch_button.setDisabled(True)
            # Disable block remanence
            await self.workspace_fs.disable_block_remanence()
            # Update content
            self._update_switch_button()
            self._update_text()

    def _on_close_clicked(self) -> None:
        self.dialog.accept()

    @classmethod
    def show_modal(
        cls,
        jobs_ctx: QtToTrioJobScheduler,
        core: LoggedCore,
        workspace_fs: WorkspaceFS,
        parent: QWidget,
    ) -> None:
        widget = cls(jobs_ctx, core, workspace_fs)
        workspace_name = workspace_fs.get_workspace_name()
        dialog = GreyedDialog(
            widget,
            T("TEXT_REMANENCE_DIALOG_TITLE_workspace_name").format(workspace_name=workspace_name),
            parent=parent,
            width=1000,
            close_on_click=False,
            hide_close=True,
        )
        widget.dialog = dialog

        # Unlike exec_, show is asynchronous and works within the main Qt loop
        dialog.show()
