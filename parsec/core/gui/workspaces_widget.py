# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from contextlib import contextmanager
from enum import Enum
from pathlib import PurePath
from typing import Any, Iterator, Sequence, cast

from PyQt5.QtCore import QDate, QEvent, QObject, Qt, QTime, QTimer, pyqtBoundSignal, pyqtSignal
from PyQt5.QtWidgets import QAbstractButton, QLabel, QWidget
from structlog import get_logger

from parsec._parsec import CoreEvent, DateTime, LocalDateTime
from parsec.core.fs import (
    FSBackendOfflineError,
    FSError,
    FsPath,
    FSWorkspaceNoAccess,
    FSWorkspaceNotFoundError,
    WorkspaceFS,
    WorkspaceFSTimestamped,
)
from parsec.core.gui import desktop, validators
from parsec.core.gui.custom_dialogs import ask_question, get_text_input, show_error
from parsec.core.gui.flow_layout import FlowLayout
from parsec.core.gui.lang import translate as T
from parsec.core.gui.timestamped_workspace_widget import TimestampedWorkspaceWidget
from parsec.core.gui.trio_jobs import JobResultError, QtToTrioJob, QtToTrioJobScheduler
from parsec.core.gui.ui.workspaces_widget import Ui_WorkspacesWidget
from parsec.core.gui.workspace_button import WorkspaceButton
from parsec.core.gui.workspace_sharing_widget import WorkspaceSharingWidget
from parsec.core.logged_core import EventBus, LoggedCore
from parsec.core.mountpoint.exceptions import (
    MountpointAlreadyMounted,
    MountpointError,
    MountpointNoDriveAvailable,
    MountpointNotMounted,
)
from parsec.core.types import BackendOrganizationFileLinkAddr, EntryID, EntryName, UserInfo
from parsec.event_bus import EventCallback

logger = get_logger()


async def _do_workspace_create(core: LoggedCore, workspace_name: str) -> EntryID:
    try:
        workspace_name_entry = EntryName(workspace_name)
    except ValueError:
        # This should never occurs given new_name is checked by a validator in the GUI
        # TODO: improve this logic ?
        raise JobResultError("invalid-name")
    workspace_id = await core.user_fs.workspace_create(workspace_name_entry)
    return workspace_id


async def _do_workspace_rename(
    core: LoggedCore,
    workspace_id: EntryID,
    new_name: str,
    button: QAbstractButton | WorkspaceButton | None,
) -> tuple[QAbstractButton | WorkspaceButton | None, str]:
    try:
        new_name_entry = EntryName(new_name)
    except ValueError:
        # This should never occurs given new_name is checked by a validator in the GUI
        # TODO: improve this logic ?
        raise JobResultError("invalid-name")
    try:
        await core.user_fs.workspace_rename(workspace_id, new_name_entry)
        return button, new_name
    except Exception as exc:
        raise JobResultError("rename-error") from exc


async def _do_workspace_list(core: LoggedCore) -> list[WorkspaceFS]:
    workspaces = []
    user_manifest = core.user_fs.get_user_manifest()
    available_workspaces = [w for w in user_manifest.workspaces if w.role]
    for workspace in available_workspaces:
        workspace_id = workspace.id
        workspace_fs = core.user_fs.get_workspace(workspace_id)
        workspaces.append(workspace_fs)
    workspaces_timestamped_dict = await core.mountpoint_manager.get_timestamped_mounted()
    for (workspace_id, timestamp), workspace_fs in workspaces_timestamped_dict.items():
        workspaces.append(workspace_fs)

    return workspaces


async def _do_workspace_mount(
    core: LoggedCore, workspace_id: EntryID, timestamp: DateTime | None = None
) -> None:
    try:
        await core.mountpoint_manager.mount_workspace(workspace_id, timestamp)
    except MountpointAlreadyMounted:
        pass


async def _do_workspace_unmount(
    core: LoggedCore, workspace_id: EntryID, timestamp: DateTime | None = None
) -> None:
    try:
        await core.mountpoint_manager.unmount_workspace(workspace_id, timestamp)
    except MountpointNotMounted:
        pass


class WorkspacesWidget(QWidget, Ui_WorkspacesWidget):
    REFRESH_WORKSPACES_LIST_DELAY = 1  # 1s

    load_workspace_clicked = pyqtSignal(WorkspaceFS, FsPath, bool)
    workspace_reencryption_success = pyqtSignal(QtToTrioJob)
    workspace_reencryption_error = pyqtSignal(QtToTrioJob)
    workspace_reencryption_progress = pyqtSignal(EntryID, int, int)
    mountpoint_state_updated = pyqtSignal(object, object)

    rename_success = pyqtSignal(QtToTrioJob)
    rename_error = pyqtSignal(QtToTrioJob)
    create_success = pyqtSignal(QtToTrioJob)
    create_error = pyqtSignal(QtToTrioJob)
    list_success = pyqtSignal(QtToTrioJob)
    list_error = pyqtSignal(QtToTrioJob)
    mount_success = pyqtSignal(QtToTrioJob)
    mount_error = pyqtSignal(QtToTrioJob)
    unmount_success = pyqtSignal(QtToTrioJob)
    unmount_error = pyqtSignal(QtToTrioJob)
    ignore_success = pyqtSignal(QtToTrioJob)
    ignore_error = pyqtSignal(QtToTrioJob)
    file_open_success = pyqtSignal(QtToTrioJob)
    file_open_error = pyqtSignal(QtToTrioJob)

    def __init__(
        self, core: LoggedCore, jobs_ctx: QtToTrioJobScheduler, event_bus: EventBus, **kwargs: Any
    ) -> None:
        super().__init__(**kwargs)
        self.setupUi(self)

        self.core = core
        self.jobs_ctx = jobs_ctx
        self.event_bus = event_bus
        self.reencrypting: set[EntryID] = set()
        self.disabled_workspaces = self.core.config.disabled_workspaces
        self.workspace_button_mapping: dict[tuple[EntryID, DateTime | None], WorkspaceButton] = {}

        self.layout_workspaces = FlowLayout(spacing=40)
        self.layout_content.addLayout(self.layout_workspaces)

        if self.core.device.is_outsider:
            self.button_add_workspace.hide()
        else:
            self.button_add_workspace.clicked.connect(self.create_workspace_clicked)
        self.button_goto_file.clicked.connect(self.goto_file_clicked)

        self.button_add_workspace.apply_style()
        self.button_goto_file.apply_style()

        self.search_timer = QTimer()
        self.search_timer.setInterval(300)
        self.search_timer.setSingleShot(True)
        self.search_timer.timeout.connect(self.on_workspace_filter)

        self.line_edit_search.textChanged.connect(self.search_timer.start)
        self.line_edit_search.clear_clicked.connect(self.search_timer.start)

        self.rename_success.connect(self.on_rename_success)
        self.rename_error.connect(self.on_rename_error)
        self.create_success.connect(self.on_create_success)
        self.create_error.connect(self.on_create_error)
        self.list_success.connect(self.on_list_success)
        self.list_error.connect(self.on_list_error)
        self.workspace_reencryption_progress.connect(self._on_workspace_reencryption_progress)
        self.mount_success.connect(self.on_mount_success)
        self.mount_error.connect(self.on_mount_error)
        self.unmount_success.connect(self.on_unmount_success)
        self.unmount_error.connect(self.on_unmount_error)
        self.file_open_success.connect(self._on_file_open_success)
        self.file_open_error.connect(self._on_file_open_error)
        self.check_hide_unmounted.stateChanged.connect(self._on_hide_unmounted_changed)

        self.workspace_reencryption_success.connect(self._on_workspace_reencryption_success)
        self.workspace_reencryption_error.connect(self._on_workspace_reencryption_error)

        self.filter_remove_button.clicked.connect(self.remove_user_filter)
        self.filter_remove_button.apply_style()

        self.filter_user_info: UserInfo | None = None
        self.filter_layout_widget.hide()

    def remove_user_filter(self) -> None:
        self.filter_user_info = None
        self.filter_layout_widget.hide()
        self.reset()

    def set_user_info(self, user_info: UserInfo) -> None:
        self.filter_user_info = user_info
        self.filter_layout_widget.show()
        self.filter_label.setText(
            T("TEXT_WORKSPACE_FILTERED_user").format(user=user_info.short_user_display)
        )

    def disconnect_all(self) -> None:
        pass

    def showEvent(self, event: QEvent) -> None:
        self.event_bus.connect(
            CoreEvent.FS_WORKSPACE_CREATED, cast(EventCallback, self._on_workspace_created)
        )
        self.event_bus.connect(
            CoreEvent.FS_ENTRY_UPDATED, cast(EventCallback, self._on_fs_entry_updated)
        )
        self.event_bus.connect(
            CoreEvent.FS_ENTRY_SYNCED, cast(EventCallback, self._on_fs_entry_synced)
        )
        self.event_bus.connect(
            CoreEvent.SHARING_UPDATED, cast(EventCallback, self._on_sharing_updated)
        )
        self.event_bus.connect(
            CoreEvent.FS_ENTRY_DOWNSYNCED, cast(EventCallback, self._on_entry_downsynced)
        )
        self.event_bus.connect(
            CoreEvent.MOUNTPOINT_STARTED, cast(EventCallback, self._on_mountpoint_started)
        )
        self.event_bus.connect(
            CoreEvent.MOUNTPOINT_STOPPED, cast(EventCallback, self._on_mountpoint_stopped)
        )
        self.reset()

    def hideEvent(self, event: QEvent) -> None:
        try:
            self.event_bus.disconnect(
                CoreEvent.FS_WORKSPACE_CREATED, cast(EventCallback, self._on_workspace_created)
            )
            self.event_bus.disconnect(
                CoreEvent.FS_ENTRY_UPDATED, cast(EventCallback, self._on_fs_entry_updated)
            )
            self.event_bus.disconnect(
                CoreEvent.FS_ENTRY_SYNCED, cast(EventCallback, self._on_fs_entry_synced)
            )
            self.event_bus.disconnect(
                CoreEvent.SHARING_UPDATED, cast(EventCallback, self._on_sharing_updated)
            )
            self.event_bus.disconnect(
                CoreEvent.FS_ENTRY_DOWNSYNCED, cast(EventCallback, self._on_entry_downsynced)
            )
            self.event_bus.disconnect(
                CoreEvent.MOUNTPOINT_STARTED, cast(EventCallback, self._on_mountpoint_started)
            )
            self.event_bus.disconnect(
                CoreEvent.MOUNTPOINT_STOPPED, cast(EventCallback, self._on_mountpoint_stopped)
            )
        except ValueError:
            pass

    def has_workspaces_displayed(self) -> bool:
        return self.layout_workspaces.count() >= 1 and isinstance(
            self.layout_workspaces.itemAt(0).widget(), WorkspaceButton
        )

    def _on_hide_unmounted_changed(self, state: object) -> None:
        self.refresh_workspace_layout()

    def goto_file_clicked(self) -> None:
        file_link = get_text_input(
            self,
            T("TEXT_WORKSPACE_GOTO_FILE_LINK_TITLE"),
            T("TEXT_WORKSPACE_GOTO_FILE_LINK_INSTRUCTIONS"),
            placeholder=T("TEXT_WORKSPACE_GOTO_FILE_LINK_PLACEHOLDER"),
            default_text="",
            button_text=T("ACTION_GOTO_FILE_LINK"),
        )
        if not file_link:
            return

        try:
            addr = BackendOrganizationFileLinkAddr.from_url(file_link, allow_http_redirection=True)
        except ValueError as exc:
            show_error(self, T("TEXT_WORKSPACE_GOTO_FILE_LINK_INVALID_LINK"), exception=exc)
            return

        button = self.get_workspace_button(addr.workspace_id)
        if button is not None:
            try:
                path = button.workspace_fs.decrypt_file_link_path(addr)
                ts = button.workspace_fs.decrypt_timestamp(addr)

                # Mount workspace at the link's creation time
                if not self.core.mountpoint_manager.is_workspace_mounted(
                    button.workspace_fs.workspace_id, ts
                ):
                    self.mount_workspace(addr.workspace_id, ts)
            except ValueError as exc:
                show_error(self, T("TEXT_WORKSPACE_GOTO_FILE_LINK_INVALID_LINK"), exception=exc)
                return
            self.load_workspace(
                WorkspaceFSTimestamped(button.workspace_fs, ts)
                if ts is not None
                else button.workspace_fs,
                path=path,
                selected=True,
            )
            return

        show_error(self, T("TEXT_WORKSPACE_GOTO_FILE_LINK_WORKSPACE_NOT_FOUND"))

    def on_workspace_filter(self) -> None:
        self.refresh_workspace_layout()

    def load_workspace(
        self, workspace_fs: WorkspaceFS, path: FsPath = FsPath("/"), selected: bool = False
    ) -> None:
        self.load_workspace_clicked.emit(workspace_fs, path, selected)

    def on_create_success(self, job: QtToTrioJob[EntryID]) -> None:
        self.remove_user_filter()

    def on_create_error(self, job: QtToTrioJob[EntryID]) -> None:
        if job.status == "invalid-name":
            show_error(self, T("TEXT_WORKSPACE_CREATE_NEW_INVALID_NAME"), exception=job.exc)
        else:
            show_error(self, T("TEXT_WORKSPACE_CREATE_NEW_UNKNOWN_ERROR"), exception=job.exc)

    def on_rename_success(self, job: QtToTrioJob[tuple[QAbstractButton, str]]) -> None:
        assert job.ret is not None
        workspace_button, workspace_name = job.ret
        workspace_button: WorkspaceButton
        workspace_name: str

        if workspace_button:
            workspace_button.reload_workspace_name(EntryName(workspace_name))

    def on_rename_error(self, job: QtToTrioJob[tuple[QAbstractButton, str]]) -> None:
        if job.status == "invalid-name":
            show_error(self, T("TEXT_WORKSPACE_RENAME_INVALID_NAME"), exception=job.exc)
        else:
            show_error(self, T("TEXT_WORKSPACE_RENAME_UNKNOWN_ERROR"), exception=job.exc)

    def on_list_success(self, job: QtToTrioJob[list[WorkspaceFS]]) -> None:
        # Hide the spinner in case it was visible
        self.spinner.hide()
        assert job.ret is not None
        workspaces: list[WorkspaceFS] = job.ret

        # Use temporary dict to update the workspace mapping
        new_mapping: dict[tuple[EntryID, DateTime | None], WorkspaceButton] = {}
        old_mapping = dict(self.workspace_button_mapping)

        # Loop over the resulting workspaces
        assert workspaces is not None
        for workspace_fs in workspaces:

            # Pop button from existing mapping
            key = (workspace_fs.workspace_id, getattr(workspace_fs, "timestamp", None))
            button = old_mapping.pop(key, None)

            # Create and bind button if it doesn't exist
            if button is None:
                button = WorkspaceButton(self.core, self.jobs_ctx, workspace_fs, parent=self)
                button.clicked.connect(self.load_workspace)
                button.share_clicked.connect(self.share_workspace)
                button.reencrypt_clicked.connect(self.reencrypt_workspace)
                button.delete_clicked.connect(self.delete_workspace)
                button.rename_clicked.connect(self.rename_workspace)
                button.remount_ts_clicked.connect(self.remount_workspace_ts)
                button.open_clicked.connect(self.open_workspace)
                button.switch_clicked.connect(self._on_switch_clicked)
            button.refresh_status()

            # Add the button to the new mapping
            # Note that the order of insertion matters as it corresponds to the order in which
            # the workspaces are displayed.
            new_mapping[key] = button

        # Set the new mapping
        self.workspace_button_mapping = new_mapping

        # Refresh the layout, taking the filtering into account
        self.refresh_workspace_layout()

        # Dereference the old buttons
        for button in old_mapping.values():
            # Mypy: this is the correct way to remove a parent from a widget.
            button.setParent(None)  # type: ignore[call-overload]

    def on_list_error(self, job: QtToTrioJob[list[WorkspaceFS]]) -> None:
        self.spinner.hide()
        self.layout_workspaces.clear()
        label = QLabel(T("TEXT_WORKSPACE_NO_WORKSPACES"))
        label.setAlignment(Qt.AlignHCenter | Qt.AlignVCenter)
        self.layout_workspaces.addWidget(label)

    def refresh_workspace_layout(self) -> None:
        # This user has no workspaces yet
        if not self.workspace_button_mapping:
            self.layout_workspaces.clear()
            self.line_edit_search.hide()
            label = QLabel(T("TEXT_WORKSPACE_NO_WORKSPACES"))
            label.setAlignment(Qt.AlignHCenter | Qt.AlignVCenter)
            self.layout_workspaces.addWidget(label)
            return

        position = 0
        if self.scrollArea.verticalScrollBar():
            position = self.scrollArea.verticalScrollBar().sliderPosition()

        # Make sure the search bar is visible
        self.line_edit_search.show()

        # Get info for both filters
        name_filter = self.line_edit_search.text().lower() or None
        user_filter = self.filter_user_info and self.filter_user_info.user_id
        hide_unmounted_filter = self.check_hide_unmounted.checkState() == Qt.Checked

        # Remove all widgets and add them back in order to make sure the order is always correct
        self.layout_workspaces.pop_all()

        # Loop over buttons
        for button in self.workspace_button_mapping.values():
            # Filter by name
            assert button.name is not None
            if name_filter is not None and name_filter not in button.name.str.lower():
                continue
            # Filter by user
            if user_filter is not None and (
                button.users_roles is None or user_filter not in button.users_roles
            ):
                continue
            # Filter unmounted workspaces
            if hide_unmounted_filter and not button.is_mounted():
                continue
            # Show and add widget to the layout
            button.show()
            self.layout_workspaces.addWidget(button)

        # Force the layout to update
        self.layout_workspaces.update()
        # Restore the scroll bar position
        if self.scrollArea.verticalScrollBar():
            self.scrollArea.verticalScrollBar().setSliderPosition(position)

    def on_mount_success(self, job: QtToTrioJob[None]) -> None:
        self.reset()

    def on_mount_error(self, job: QtToTrioJob[None]) -> None:
        if isinstance(job.exc, MountpointError):
            workspace_id = job.arguments.get("workspace_id")
            timestamp = job.arguments.get("timestamp")
            assert workspace_id is not None
            assert timestamp is not None
            wb = self.get_workspace_button(cast(EntryID, workspace_id), cast(DateTime, timestamp))
            if wb:
                wb.set_mountpoint_state(False)
            if isinstance(job.exc, MountpointNoDriveAvailable):
                show_error(self, T("TEXT_WORKSPACE_CANNOT_MOUNT_NO_DRIVE"), exception=job.exc)
            else:
                show_error(self, T("TEXT_WORKSPACE_CANNOT_MOUNT"), exception=job.exc)

    def on_unmount_success(self, job: QtToTrioJob[None]) -> None:
        self.reset()

    def on_unmount_error(self, job: QtToTrioJob[None]) -> None:
        if isinstance(job.exc, MountpointError):
            show_error(self, T("TEXT_WORKSPACE_CANNOT_UNMOUNT"), exception=job.exc)

    def fix_legacy_workspace_names(self, workspace_fs: WorkspaceFS, workspace_name: str) -> None:
        # Temporary code to fix the workspace names edited by
        # the previous naming policy (the userfs used to add
        # `(shared by <device>)` at the end of the workspace name)
        token = " (shared by "
        if token in workspace_name:
            workspace_name, *_ = workspace_name.split(token)
            _ = self.jobs_ctx.submit_job(
                (self, "ignore_success"),
                (self, "ignore_error"),
                _do_workspace_rename,
                core=self.core,
                workspace_id=workspace_fs.workspace_id,
                new_name=workspace_name,
                button=None,
            )

    def _on_switch_clicked(
        self, state: bool, workspace_fs: WorkspaceFS, timestamp: DateTime
    ) -> None:
        if state:
            self.mount_workspace(workspace_fs.workspace_id, timestamp)
        else:
            self.unmount_workspace(workspace_fs.workspace_id, timestamp)
        if not timestamp:
            self.update_workspace_config(workspace_fs.workspace_id, state)

    def open_workspace(self, workspace_fs: WorkspaceFS) -> None:
        self.open_workspace_file(workspace_fs, None)

    def open_workspace_file(self, workspace_fs: WorkspaceFS, file_name: str | None) -> None:
        file_name = FsPath("/", file_name) if file_name else FsPath("/")  # type: ignore[call-arg]

        try:
            path = self.core.mountpoint_manager.get_path_in_mountpoint(
                workspace_fs.workspace_id,
                file_name,
                workspace_fs.timestamp
                if isinstance(workspace_fs, WorkspaceFSTimestamped)
                else None,
            )
            _ = self.jobs_ctx.submit_job(
                (self, "file_open_success"),
                (self, "file_open_error"),
                desktop.open_files_job,
                [path],
            )
        except MountpointNotMounted:
            # The mountpoint has been unmounted in our back, nothing left to do
            show_error(self, T("TEXT_FILE_OPEN_ERROR_file").format(file=str(file_name)))

    def _on_file_open_success(self, job: QtToTrioJob[tuple[bool, Sequence[PurePath]]]) -> None:
        assert job.ret is not None
        status, paths = job.ret
        status: bool
        paths: list[FsPath]
        if not status:
            show_error(self, T("TEXT_FILE_OPEN_ERROR_file").format(file=paths[0].name))

    def _on_file_open_error(self, job: QtToTrioJob[tuple[bool, Sequence[PurePath]]]) -> None:
        logger.error("Failed to open the workspace in the explorer")

    def remount_workspace_ts(self, workspace_fs: WorkspaceFS) -> None:
        def _on_finished(date: QDate, time: QTime) -> None:
            if not date or not time:
                return

            datetime = LocalDateTime(
                date.year(), date.month(), date.day(), time.hour(), time.minute(), time.second()
            ).to_utc()
            self.mount_workspace(workspace_fs.workspace_id, datetime)

        TimestampedWorkspaceWidget.show_modal(
            workspace_fs=workspace_fs, jobs_ctx=self.jobs_ctx, parent=self, on_finished=_on_finished
        )

    def mount_workspace(self, workspace_id: EntryID, timestamp: DateTime | None = None) -> None:
        # In successful cases, the events MOUNTPOINT_STARTED
        # will take care of refreshing the state of the button,
        # the mount_success signal is not connected to anything but
        # is being kept for potential testing purposes
        _ = self.jobs_ctx.submit_job(
            (self, "mount_success"),
            (self, "mount_error"),
            _do_workspace_mount,
            core=self.core,
            workspace_id=workspace_id,
            timestamp=timestamp,
        )

    def unmount_workspace(self, workspace_id: EntryID, timestamp: DateTime | None = None) -> None:
        # In successful cases, the event MOUNTPOINT_STOPPED
        # will take care of refreshing the state of the button,
        # the unmount_success signal is not connected to anything but
        # is being kept for potential testing purposes
        _ = self.jobs_ctx.submit_job(
            (self, "unmount_success"),
            (self, "unmount_error"),
            _do_workspace_unmount,
            core=self.core,
            workspace_id=workspace_id,
            timestamp=timestamp,
        )

    def update_workspace_config(self, workspace_id: EntryID, state: bool) -> None:
        if state:
            self.disabled_workspaces -= {workspace_id}
        else:
            self.disabled_workspaces |= {workspace_id}
        self.event_bus.send(
            CoreEvent.GUI_CONFIG_CHANGED, disabled_workspaces=self.disabled_workspaces
        )

    def is_workspace_mounted(
        self, workspace_id: EntryID, timestamp: DateTime | None = None
    ) -> bool:
        return self.core.mountpoint_manager.is_workspace_mounted(workspace_id, timestamp)

    def delete_workspace(self, workspace_fs: WorkspaceFS) -> None:
        if isinstance(workspace_fs, WorkspaceFSTimestamped):
            self.unmount_workspace(workspace_fs.workspace_id, workspace_fs.timestamp)
            return
        else:
            workspace_name = workspace_fs.get_workspace_name()
            result = ask_question(
                self,
                T("TEXT_WORKSPACE_DELETE_TITLE"),
                T("TEXT_WORKSPACE_DELETE_INSTRUCTIONS_workspace").format(workspace=workspace_name),
                [T("ACTION_DELETE_WORKSPACE_CONFIRM"), T("ACTION_CANCEL")],
            )
            if result != T("ACTION_DELETE_WORKSPACE_CONFIRM"):
                return
            # Workspace deletion is not available yet (button should be hidden anyway)

    def rename_workspace(self, workspace_button: WorkspaceButton) -> None:
        assert workspace_button.name is not None
        new_name = get_text_input(
            self,
            T("TEXT_WORKSPACE_RENAME_TITLE"),
            T("TEXT_WORKSPACE_RENAME_INSTRUCTIONS"),
            placeholder=T("TEXT_WORKSPACE_RENAME_PLACEHOLDER"),
            default_text=workspace_button.name.str,
            button_text=T("ACTION_WORKSPACE_RENAME_CONFIRM"),
            validator=validators.WorkspaceNameValidator(),
        )
        if not new_name:
            return
        _ = self.jobs_ctx.submit_job(
            (self, "rename_success"),
            (self, "rename_error"),
            _do_workspace_rename,
            core=self.core,
            workspace_id=workspace_button.workspace_fs.workspace_id,
            new_name=new_name,
            button=workspace_button,
        )

    def on_sharing_closing(self, has_changes: bool) -> None:
        if has_changes:
            self.reset()

    def share_workspace(self, workspace_fs: WorkspaceFS) -> None:
        WorkspaceSharingWidget.show_modal(
            user_fs=self.core.user_fs,
            workspace_fs=workspace_fs,
            core=self.core,
            jobs_ctx=self.jobs_ctx,
            parent=self,
            on_finished=self.on_sharing_closing,
        )

    def reencrypt_workspace(
        self,
        workspace_id: EntryID,
        user_revoked: bool,
        role_revoked: bool,
        reencryption_already_in_progress: bool,
    ) -> None:
        if workspace_id in self.reencrypting or (
            not user_revoked and not role_revoked and not reencryption_already_in_progress
        ):
            return

        question = ""
        if user_revoked:
            question += "{}\n".format(T("TEXT_WORKSPACE_NEED_REENCRYPTION_BECAUSE_USER_REVOKED"))
        if role_revoked:
            question += "{}\n".format(T("TEXT_WORKSPACE_NEED_REENCRYPTION_BECAUSE_USER_REMOVED"))
        question += T("TEXT_WORKSPACE_NEED_REENCRYPTION_INSTRUCTIONS")

        r = ask_question(
            self,
            T("TEXT_WORKSPACE_NEED_REENCRYPTION_TITLE"),
            question,
            [T("ACTION_WORKSPACE_REENCRYPTION_CONFIRM"), T("ACTION_CANCEL")],
        )
        if r != T("ACTION_WORKSPACE_REENCRYPTION_CONFIRM"):
            return

        @contextmanager
        def _handle_fs_errors() -> Iterator[None]:
            try:
                yield
            except FSBackendOfflineError as exc:
                raise JobResultError(ret=workspace_id, status="offline-backend", origin=exc)
            except FSWorkspaceNoAccess as exc:
                raise JobResultError(ret=workspace_id, status="access-error", origin=exc)
            except FSWorkspaceNotFoundError as exc:
                raise JobResultError(ret=workspace_id, status="not-found", origin=exc)
            except FSError as exc:
                raise JobResultError(ret=workspace_id, status="fs-error", origin=exc)

        async def _reencrypt(on_progress: tuple[QObject, str], workspace_id: EntryID) -> EntryID:
            on_progress: pyqtBoundSignal = getattr(
                on_progress[0], on_progress[1]
            )  # Retrieve the signal
            with _handle_fs_errors():
                if reencryption_already_in_progress:
                    job = await self.core.user_fs.workspace_continue_reencryption(workspace_id)
                else:
                    job = await self.core.user_fs.workspace_start_reencryption(workspace_id)
            while True:
                with _handle_fs_errors():
                    total, done = await job.do_one_batch()
                on_progress.emit(workspace_id, total, done)
                if total == done:
                    break
            return workspace_id

        self.reencrypting.add(workspace_id)

        # Initialize progress to 0 percent
        workspace_button = self.get_workspace_button(workspace_id, None)
        assert workspace_button is not None
        workspace_button.reencryption = 1, 0

        _ = self.jobs_ctx.submit_job(
            (self, "workspace_reencryption_success"),
            (self, "workspace_reencryption_error"),
            _reencrypt,
            on_progress=(self, "workspace_reencryption_progress"),
            workspace_id=workspace_id,
        )

    def _on_workspace_reencryption_success(self, job: QtToTrioJob[EntryID]) -> None:
        workspace_id = cast(EntryID, job.arguments["workspace_id"])
        workspace_button = self.get_workspace_button(workspace_id, None)
        assert workspace_button is not None
        workspace_button.reencryption_needs = None
        workspace_button.reencryption = None
        self.reencrypting.remove(workspace_id)

    def _on_workspace_reencryption_error(self, job: QtToTrioJob[EntryID]) -> None:
        workspace_id = cast(EntryID, job.arguments["workspace_id"])
        workspace_button = self.get_workspace_button(workspace_id, None)
        assert workspace_button is not None
        workspace_button.reencryption = None
        self.reencrypting.remove(workspace_id)
        if job.is_cancelled():
            return
        if job.status == "offline-backend":
            err_msg = T("TEXT_WORKSPACE_REENCRYPT_OFFLINE_ERROR")
        elif job.status == "access-error":
            err_msg = T("TEXT_WORKSPACE_REENCRYPT_ACCESS_ERROR")
        elif job.status == "not-found":
            err_msg = T("TEXT_WORKSPACE_REENCRYPT_NOT_FOUND_ERROR")
        elif job.status == "fs-error":
            err_msg = T("TEXT_WORKSPACE_REENCRYPT_FS_ERROR")
        else:
            err_msg = T("TEXT_WORKSPACE_REENCRYPT_UNKOWN_ERROR")
        show_error(self, err_msg, exception=job.exc)

    def get_workspace_button(
        self, workspace_id: EntryID, timestamp: DateTime | None = None
    ) -> WorkspaceButton | None:
        key = (workspace_id, timestamp)
        return self.workspace_button_mapping.get(key)

    def _on_workspace_reencryption_progress(
        self, workspace_id: EntryID, total: int, done: int
    ) -> None:
        wb = self.get_workspace_button(workspace_id, None)
        assert wb is not None
        if done == total:
            wb.reencryption = None
        else:
            wb.reencryption = (total, done)

    def create_workspace_clicked(self) -> None:
        workspace_name = get_text_input(
            parent=self,
            title=T("TEXT_WORKSPACE_NEW_TITLE"),
            message=T("TEXT_WORKSPACE_NEW_INSTRUCTIONS"),
            placeholder=T("TEXT_WORKSPACE_NEW_PLACEHOLDER"),
            button_text=T("ACTION_WORKSPACE_NEW_CREATE"),
            validator=validators.WorkspaceNameValidator(),
        )
        if not workspace_name:
            return
        _ = self.jobs_ctx.submit_job(
            (self, "create_success"),
            (self, "create_error"),
            _do_workspace_create,
            core=self.core,
            workspace_name=workspace_name,
        )

    def reset(self) -> None:
        self.list_workspaces()

    def list_workspaces(self) -> None:
        if not self.has_workspaces_displayed():
            self.layout_workspaces.clear()
            self.spinner.show()
        _ = self.jobs_ctx.submit_throttled_job(
            "workspace_widget.list_workspaces",
            self.REFRESH_WORKSPACES_LIST_DELAY,
            (self, "list_success"),
            (self, "list_error"),
            _do_workspace_list,
            core=self.core,
        )

    def _on_sharing_updated(self, event: QEvent, new_entry: str, previous_entry: str) -> None:
        self.reset()

    def _on_workspace_created(self, event: Enum, new_entry: str) -> None:
        self.reset()

    def _on_fs_entry_synced(
        self, event: QEvent, id: str, workspace_id: EntryID | None = None
    ) -> None:
        self.reset()

    def _on_fs_entry_updated(
        self, event: QEvent, workspace_id: EntryID | None = None, id: EntryID | None = None
    ) -> None:
        assert id is not None
        if workspace_id and id == workspace_id:
            self.reset()

    def _on_entry_downsynced(
        self, event: QEvent, workspace_id: EntryID | None = None, id: str | None = None
    ) -> None:
        self.reset()

    def _on_mountpoint_state_updated(
        self, workspace_id: EntryID, timestamp: DateTime | None
    ) -> None:
        wb = self.get_workspace_button(workspace_id, timestamp)
        if wb:
            mounted = self.is_workspace_mounted(workspace_id, timestamp)
            wb.set_mountpoint_state(mounted)
            self.refresh_workspace_layout()

    def _on_mountpoint_started(
        self,
        event: QEvent,
        mountpoint: object,
        workspace_id: EntryID,
        timestamp: DateTime | None,
    ) -> None:
        self._on_mountpoint_state_updated(workspace_id, timestamp)

    def _on_mountpoint_stopped(
        self,
        event: QEvent,
        mountpoint: object,
        workspace_id: EntryID,
        timestamp: DateTime | None,
    ) -> None:
        self._on_mountpoint_state_updated(workspace_id, timestamp)
