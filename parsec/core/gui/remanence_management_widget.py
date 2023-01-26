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

        # Connect signals
        self.button_close_dialog.clicked.connect(self._on_close_clicked)
        self.enable_block_remanence_button.clicked.connect(self._on_enable_block_remanence_clicked)
        self.disable_block_remanence_button.clicked.connect(
            self._on_disable_block_remanence_clicked
        )

        # Hide widgets
        self.label_blocks.hide()
        self.progress_bar.hide()
        self.enable_block_remanence_button.hide()
        self.disable_block_remanence_button.hide()
        self.label_warning.hide()

        # Set waiting state
        self._show_waiting_widgets()
        self.wait_prepared_job: QtToTrioJob[None] = self.jobs_ctx.submit_job(
            None, None, self._wait_prepared
        )

        # Update style for close button
        self.setStyleSheet(
            "#button_close_dialog {background-color: darkgrey;} #button_close_dialog:hover {background-color: grey;}"
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

    def _show_waiting_widgets(self) -> None:
        self.waiting_widget.show()
        self.waiting_label.show()
        self.waiting_label.setText(T("TEXT_REMANENCE_WAITING"))

    def _hide_waiting_widget(self) -> None:
        self.waiting_widget.hide()
        self.waiting_label.hide()

    def _update_buttons(self) -> None:
        self.enable_block_remanence_button.setEnabled(True)
        self.disable_block_remanence_button.setEnabled(True)
        if self.workspace_fs.is_block_remanent():
            self.enable_block_remanence_button.hide()
            self.disable_block_remanence_button.show()
        else:
            self.enable_block_remanence_button.show()
            self.disable_block_remanence_button.hide()

    def _update_text(self) -> None:
        # Do not update while we're performing an operation
        if self.waiting_widget.isVisible():
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
        # Lable size label
        self.label_size.setText(
            T("TEXT_WORKSPACE_SIZE_local_size_total_size").format(
                local_size=get_filesize(info.local_and_remote_size),
                total_size=get_filesize(info.total_size),
            )
        )
        # Warning label
        if not info.is_block_remanent and info.remote_only_size > 0:
            self.label_warning.setText(
                T("TEXT_BLOCK_REMANENCE_WARNING_remote_size").format(
                    remote_size=get_filesize(info.remote_only_size)
                )
            )
            self.label_warning.show()
        else:
            self.label_warning.hide()

    def _update_progress_bar(self) -> None:
        # Progress bar
        info = self.workspace_fs.get_remanence_manager_info()
        self.progress_bar.setMinimum(0)
        self.progress_bar.setMaximum(info.total_size)
        self.progress_bar.setValue(info.local_and_remote_size)

    async def _wait_prepared(self) -> None:
        with self._handle_error(T("TEXT_GET_REMANENCE_STATUS_FAILED")):
            await self.workspace_fs.remanence_manager.wait_prepared()
            # Done waiting
            self._hide_waiting_widget()
            # Update content
            self._update_text()
            self._update_buttons()
            # Update text periodically
            while True:
                # The dialog has been closed
                if not self.dialog.isVisible():
                    return
                # Update 1 time per seconds
                self._update_text()
                await trio.sleep(1)

    async def _on_enable_block_remanence_clicked(self, *_: object) -> None:
        # Close the dialog when an error occurs
        with self._handle_error(T("TEXT_ENABLE_BLOCK_REMANENCE_FAILED")):
            # Disable the corresponding button
            self.enable_block_remanence_button.setDisabled(True)
            self.label_warning.hide()
            # Show waiting state
            self._show_waiting_widgets()
            # Actually enable block remanence
            await self.workspace_fs.enable_block_remanence()
            # Hide waiting state
            self._hide_waiting_widget()
            # Update content
            self._update_buttons()
            self._update_text()
            # No data to download
            if not self.workspace_fs.get_remanence_manager_info().remote_only_size:
                SnackbarManager.congratulate(T("TEXT_ENABLE_BLOCK_REMANENCE_SUCCESS"))
                return
            # Some data needs to be downloaded
            SnackbarManager.inform(T("TEXT_BLOCK_DOWNLOADING_STARTED"))
            # Show progress bar
            self.progress_bar.show()
            # Loop while there's more data to download
            while self.workspace_fs.get_remanence_manager_info().remote_only_size:
                # Update content
                self._update_progress_bar()
                # Workspace is no longer remanent
                if not self.workspace_fs.is_block_remanent():
                    self.progress_bar.hide()
                    return
                # The dialog has been closed
                if not self.dialog.isVisible():
                    return
                # Update 20 times per seconds
                await trio.sleep(0.05)
            # Hide the progress bar
            self.progress_bar.hide()
            self._update_text()
            # Send notification
            SnackbarManager.congratulate(T("TEXT_BLOCK_DOWNLOADING_FINISHED"))

    async def _on_disable_block_remanence_clicked(self, *_: object) -> None:
        # Close the dialog when a error occurs
        with self._handle_error(T("TEXT_DISABLE_BLOCK_REMANENCE_FAILED")):
            # Disable the corresponding button
            self.disable_block_remanence_button.setDisabled(True)
            self.progress_bar.hide()
            # Show waiting widgets while disabling block remanance
            self._show_waiting_widgets()
            await self.workspace_fs.disable_block_remanence()
            self._hide_waiting_widget()
            # Update content
            self._update_text()
            self._update_buttons()

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
        )
        widget.dialog = dialog

        # Unlike exec_, show is asynchronous and works within the main Qt loop
        dialog.show()
