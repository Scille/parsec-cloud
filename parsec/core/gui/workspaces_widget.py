# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from parsec.core.core_events import CoreEvent

from PyQt5.QtCore import pyqtSignal, Qt, QTimer
from PyQt5.QtWidgets import QWidget, QLabel

import pendulum

from contextlib import contextmanager
from structlog import get_logger
from parsec.core.types import (
    UserInfo,
    EntryID,
    EntryName,
    BackendOrganizationFileLinkAddr,
    WorkspaceRole,
)
from parsec.core.backend_connection import BackendNotAvailable
from parsec.core.fs import (
    FsPath,
    WorkspaceFS,
    WorkspaceFSTimestamped,
    FSBackendOfflineError,
    FSError,
    FSWorkspaceNoAccess,
    FSWorkspaceNotFoundError,
    FSWorkspaceInMaintenance,
)
from parsec.core.mountpoint.exceptions import (
    MountpointAlreadyMounted,
    MountpointNotMounted,
    MountpointError,
    MountpointNoDriveAvailable,
)

from parsec.core.gui.trio_jobs import JobResultError, QtToTrioJob, JobSchedulerNotAvailable
from parsec.core.gui import desktop
from parsec.core.gui.custom_dialogs import show_error, get_text_input, ask_question
from parsec.core.gui.flow_layout import FlowLayout
from parsec.core.gui import validators
from parsec.core.gui.lang import translate as _
from parsec.core.gui.workspace_button import WorkspaceButton
from parsec.core.gui.timestamped_workspace_widget import TimestampedWorkspaceWidget
from parsec.core.gui.ui.workspaces_widget import Ui_WorkspacesWidget
from parsec.core.gui.workspace_sharing_widget import WorkspaceSharingWidget


logger = get_logger()


async def _get_reencryption_needs(workspace_fs):
    try:
        reenc_needs = await workspace_fs.get_reencryption_need()
    except FSBackendOfflineError as exc:
        raise JobResultError("offline") from exc
    return workspace_fs.workspace_id, reenc_needs


async def _do_workspace_create(core, workspace_name):
    try:
        workspace_name = EntryName(workspace_name)
    except ValueError:
        # This should never occurs given new_name is checked by a validator in the GUI
        # TODO: improve this logic ?
        raise JobResultError("invalid-name")
    workspace_id = await core.user_fs.workspace_create(workspace_name)
    return workspace_id


async def _do_workspace_rename(core, workspace_id, new_name, button):
    try:
        new_name = EntryName(new_name)
    except ValueError:
        # This should never occurs given new_name is checked by a validator in the GUI
        # TODO: improve this logic ?
        raise JobResultError("invalid-name")
    try:
        await core.user_fs.workspace_rename(workspace_id, new_name)
        return button, new_name
    except Exception as exc:
        raise JobResultError("rename-error") from exc


async def _do_workspace_list(core):
    workspaces = []

    async def _add_workspacefs(workspace_fs, timestamped):
        ws_entry = workspace_fs.get_workspace_entry()
        users_roles = {}
        try:
            roles = await workspace_fs.get_user_roles()
            for user, role in roles.items():
                user_info = None
                if role == WorkspaceRole.OWNER or len(roles) <= 2:
                    user_info = await core.get_user_info(user)
                users_roles[user] = (role, user_info)
        except (FSBackendOfflineError, BackendNotAvailable):
            # Fallback to craft a custom list with only our device since it's
            # the only one we know about
            user_info = UserInfo(
                user_id=core.device.user_id,
                human_handle=core.device.human_handle,
                profile=core.device.profile,
                revoked_on=None,
                # Unfortunately, this field is not available from LocalDevice
                # so we have to set it with a dummy value :'(
                # However it's more a hack than an issue given this field is
                # not used here.
                created_on=pendulum.from_timestamp(0),
            )
            users_roles[user_info.user_id] = (ws_entry.role, user_info)

        # List files and directories in the root directory
        # This is used for preview.
        files = []
        try:
            async for child in workspace_fs.iterdir("/"):
                child_info = await workspace_fs.path_info(child)
                # Do not include confined files and directories
                if child_info["confinement_point"] is None:
                    files.append(child.name)
                if len(files) == 4:
                    break
        except FSBackendOfflineError:
            pass
        except FSWorkspaceInMaintenance:
            # If a reencryption has already been started, workspace files can not be fetched
            # But the workspace need to be displayed to be able to trigger for example
            # reencryption operation
            pass

        workspace_name = workspace_fs.get_workspace_name()
        return workspace_fs, workspace_name, ws_entry, users_roles, files, timestamped

    user_manifest = core.user_fs.get_user_manifest()
    available_workspaces = [w for w in user_manifest.workspaces if w.role]
    for workspace in available_workspaces:
        workspace_id = workspace.id
        workspace_fs = core.user_fs.get_workspace(workspace_id)
        workspaces.append(await _add_workspacefs(workspace_fs, timestamped=False))
    worspaces_timestamped_dict = await core.mountpoint_manager.get_timestamped_mounted()
    for (workspace_id, timestamp), workspace_fs in worspaces_timestamped_dict.items():
        workspaces.append(await _add_workspacefs(workspace_fs, timestamped=True))

    return workspaces


async def _do_workspace_mount(core, workspace_id, timestamp: pendulum.DateTime = None):
    try:
        await core.mountpoint_manager.mount_workspace(workspace_id, timestamp)
    except MountpointAlreadyMounted:
        pass


async def _do_workspace_unmount(core, workspace_id, timestamp: pendulum.DateTime = None):
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
    reencryption_needs_success = pyqtSignal(QtToTrioJob)
    reencryption_needs_error = pyqtSignal(QtToTrioJob)
    ignore_success = pyqtSignal(QtToTrioJob)
    ignore_error = pyqtSignal(QtToTrioJob)
    file_open_success = pyqtSignal(QtToTrioJob)
    file_open_error = pyqtSignal(QtToTrioJob)

    def __init__(self, core, jobs_ctx, event_bus, **kwargs):
        super().__init__(**kwargs)
        self.setupUi(self)

        self.core = core
        self.jobs_ctx = jobs_ctx
        self.event_bus = event_bus
        self.reencrypting = set()
        self.disabled_workspaces = self.core.config.disabled_workspaces
        self.workspace_button_mapping = {}

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
        self.reencryption_needs_success.connect(self.on_reencryption_needs_success)
        self.reencryption_needs_error.connect(self.on_reencryption_needs_error)
        self.workspace_reencryption_progress.connect(self._on_workspace_reencryption_progress)
        self.mount_success.connect(self.on_mount_success)
        self.mount_error.connect(self.on_mount_error)
        self.unmount_success.connect(self.on_unmount_success)
        self.unmount_error.connect(self.on_unmount_error)
        self.file_open_success.connect(self._on_file_open_success)
        self.file_open_error.connect(self._on_file_open_error)

        self.workspace_reencryption_success.connect(self._on_workspace_reencryption_success)
        self.workspace_reencryption_error.connect(self._on_workspace_reencryption_error)

        self.filter_remove_button.clicked.connect(self.remove_user_filter)
        self.filter_remove_button.apply_style()

        self.filter_user_info = None
        self.filter_layout_widget.hide()

    def remove_user_filter(self):
        self.filter_user_info = None
        self.filter_layout_widget.hide()
        self.reset()

    def set_user_info(self, user_info):
        self.filter_user_info = user_info
        self.filter_layout_widget.show()
        self.filter_label.setText(
            _("TEXT_WORKSPACE_FILTERED_user").format(user=user_info.short_user_display)
        )

    def disconnect_all(self):
        pass

    def showEvent(self, event):
        self.event_bus.connect(CoreEvent.FS_WORKSPACE_CREATED, self._on_workspace_created)
        self.event_bus.connect(CoreEvent.FS_ENTRY_UPDATED, self._on_fs_entry_updated)
        self.event_bus.connect(CoreEvent.FS_ENTRY_SYNCED, self._on_fs_entry_synced)
        self.event_bus.connect(CoreEvent.SHARING_UPDATED, self._on_sharing_updated)
        self.event_bus.connect(CoreEvent.FS_ENTRY_DOWNSYNCED, self._on_entry_downsynced)
        self.event_bus.connect(CoreEvent.MOUNTPOINT_STARTED, self._on_mountpoint_started)
        self.event_bus.connect(CoreEvent.MOUNTPOINT_STOPPED, self._on_mountpoint_stopped)
        self.reset()

    def hideEvent(self, event):
        try:
            self.event_bus.disconnect(CoreEvent.FS_WORKSPACE_CREATED, self._on_workspace_created)
            self.event_bus.disconnect(CoreEvent.FS_ENTRY_UPDATED, self._on_fs_entry_updated)
            self.event_bus.disconnect(CoreEvent.FS_ENTRY_SYNCED, self._on_fs_entry_synced)
            self.event_bus.disconnect(CoreEvent.SHARING_UPDATED, self._on_sharing_updated)
            self.event_bus.disconnect(CoreEvent.FS_ENTRY_DOWNSYNCED, self._on_entry_downsynced)
            self.event_bus.disconnect(CoreEvent.MOUNTPOINT_STARTED, self._on_mountpoint_started)
            self.event_bus.disconnect(CoreEvent.MOUNTPOINT_STOPPED, self._on_mountpoint_stopped)
        except ValueError:
            pass

    def has_workspaces_displayed(self):
        return self.layout_workspaces.count() >= 1 and isinstance(
            self.layout_workspaces.itemAt(0).widget(), WorkspaceButton
        )

    def goto_file_clicked(self):
        file_link = get_text_input(
            self,
            _("TEXT_WORKSPACE_GOTO_FILE_LINK_TITLE"),
            _("TEXT_WORKSPACE_GOTO_FILE_LINK_INSTRUCTIONS"),
            placeholder=_("TEXT_WORKSPACE_GOTO_FILE_LINK_PLACEHOLDER"),
            default_text="",
            button_text=_("ACTION_GOTO_FILE_LINK"),
        )
        if not file_link:
            return

        try:
            addr = BackendOrganizationFileLinkAddr.from_url(file_link, allow_http_redirection=True)
        except ValueError as exc:
            show_error(self, _("TEXT_WORKSPACE_GOTO_FILE_LINK_INVALID_LINK"), exception=exc)
            return

        button = self.get_workspace_button(addr.workspace_id)
        if button is not None:
            try:
                path = button.workspace_fs.decrypt_file_link_path(addr)
            except ValueError as exc:
                show_error(self, _("TEXT_WORKSPACE_GOTO_FILE_LINK_INVALID_LINK"), exception=exc)
                return
            self.load_workspace(button.workspace_fs, path=path, selected=True)
            return

        show_error(self, _("TEXT_WORKSPACE_GOTO_FILE_LINK_WORKSPACE_NOT_FOUND"))

    def on_workspace_filter(self):
        self.refresh_workspace_layout()

    def load_workspace(self, workspace_fs, path=FsPath("/"), selected=False):
        self.load_workspace_clicked.emit(workspace_fs, path, selected)

    def on_create_success(self, job):
        self.remove_user_filter()

    def on_create_error(self, job):
        if job.status == "invalid-name":
            show_error(self, _("TEXT_WORKSPACE_CREATE_NEW_INVALID_NAME"), exception=job.exc)
        else:
            show_error(self, _("TEXT_WORKSPACE_CREATE_NEW_UNKNOWN_ERROR"), exception=job.exc)

    def on_rename_success(self, job):
        workspace_button, workspace_name = job.ret
        if workspace_button:
            workspace_button.reload_workspace_name(workspace_name)

    def on_rename_error(self, job):
        if job.status == "invalid-name":
            show_error(self, _("TEXT_WORKSPACE_RENAME_INVALID_NAME"), exception=job.exc)
        else:
            show_error(self, _("TEXT_WORKSPACE_RENAME_UNKNOWN_ERROR"), exception=job.exc)

    def on_list_success(self, job):
        # Hide the spinner in case it was visible
        self.spinner.hide()
        workspaces = job.ret

        # Use temporary dict to update the workspace mapping
        new_mapping = {}
        old_mapping = dict(self.workspace_button_mapping)

        # Loop over the resulting workspaces
        for workspace_fs, workspace_name, ws_entry, users_roles, files, timestamped in workspaces:

            # Pop button from existing mapping
            key = (workspace_fs.workspace_id, getattr(workspace_fs, "timestamp", None))
            button = old_mapping.pop(key, None)

            # Retrieve current role for ourself
            user_id = workspace_fs.device.user_id
            current_role = users_roles.get(user_id)

            # Create and bind button if it doesn't exist
            if button is None:
                button = WorkspaceButton(workspace_fs, parent=self)
                button.clicked.connect(self.load_workspace)
                if self.core.device.is_outsider:
                    button.button_share.hide()
                elif current_role in (WorkspaceRole.READER, WorkspaceRole.CONTRIBUTOR):
                    button.button_share.hide()
                else:
                    button.share_clicked.connect(self.share_workspace)
                button.reencrypt_clicked.connect(self.reencrypt_workspace)
                button.delete_clicked.connect(self.delete_workspace)
                button.rename_clicked.connect(self.rename_workspace)
                button.remount_ts_clicked.connect(self.remount_workspace_ts)
                button.open_clicked.connect(self.open_workspace)
                button.switch_clicked.connect(self._on_switch_clicked)

            # Apply new state
            button.apply_state(
                workspace_name=workspace_name,
                workspace_fs=workspace_fs,
                users_roles=users_roles,
                is_mounted=self.is_workspace_mounted(workspace_fs.workspace_id, None),
                files=files[:4],
                timestamped=timestamped,
            )

            # Use this opportunity to trigger the `get_reencryption_needs` routine
            if button.is_owner:
                try:
                    self.jobs_ctx.submit_job(
                        self.reencryption_needs_success,
                        self.reencryption_needs_error,
                        _get_reencryption_needs,
                        workspace_fs=workspace_fs,
                    )
                except JobSchedulerNotAvailable:
                    pass

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
            button.setParent(None)

    def refresh_workspace_layout(self):
        # This user has no workspaces yet
        if not self.workspace_button_mapping:
            self.layout_workspaces.clear()
            self.line_edit_search.hide()
            label = QLabel(_("TEXT_WORKSPACE_NO_WORKSPACES"))
            label.setAlignment(Qt.AlignHCenter | Qt.AlignVCenter)
            self.layout_workspaces.addWidget(label)
            return

        # Make sure the search bar is visible
        self.line_edit_search.show()

        # Get info for both filters
        name_filter = self.line_edit_search.text().lower() or None
        user_filter = self.filter_user_info and self.filter_user_info.user_id

        # Remove all widgets and add them back in order to make sure the order is always correct
        self.layout_workspaces.pop_all()

        # Loop over buttons
        for button in self.workspace_button_mapping.values():
            # Filter by name
            if name_filter is not None and name_filter not in button.name.str.lower():
                continue
            # Filter by user
            if user_filter is not None and user_filter not in button.users_roles:
                continue
            # Show and add widget to the layout
            button.show()
            self.layout_workspaces.addWidget(button)

        # Force the layout to update
        self.layout_workspaces.update()

    def on_list_error(self, job):
        self.spinner.hide()
        self.layout_workspaces.clear()
        label = QLabel(_("TEXT_WORKSPACE_NO_WORKSPACES"))
        label.setAlignment(Qt.AlignHCenter | Qt.AlignVCenter)
        self.layout_workspaces.addWidget(label)

    def on_mount_success(self, job):
        self.reset()

    def on_mount_error(self, job):
        if isinstance(job.exc, MountpointError):
            workspace_id = job.arguments.get("workspace_id")
            timestamp = job.arguments.get("timestamp")
            wb = self.get_workspace_button(workspace_id, timestamp)
            if wb:
                wb.set_mountpoint_state(False)
            if isinstance(job.exc, MountpointNoDriveAvailable):
                show_error(self, _("TEXT_WORKSPACE_CANNOT_MOUNT_NO_DRIVE"), exception=job.exc)
            else:
                show_error(self, _("TEXT_WORKSPACE_CANNOT_MOUNT"), exception=job.exc)

    def on_unmount_success(self, job):
        self.reset()

    def on_unmount_error(self, job):
        if isinstance(job.exc, MountpointError):
            show_error(self, _("TEXT_WORKSPACE_CANNOT_UNMOUNT"), exception=job.exc)

    def on_reencryption_needs_success(self, job):
        workspace_id, reencryption_needs = job.ret
        button = self.get_workspace_button(workspace_id)
        if button is not None:
            button.reencryption_needs = reencryption_needs

    def on_reencryption_needs_error(self, job):
        pass

    def fix_legacy_workspace_names(self, workspace_fs, workspace_name):
        # Temporary code to fix the workspace names edited by
        # the previous naming policy (the userfs used to add
        # `(shared by <device>)` at the end of the workspace name)
        token = " (shared by "
        if token in workspace_name:
            workspace_name, *_ = workspace_name.split(token)
            self.jobs_ctx.submit_job(
                self.ignore_success,
                self.ignore_error,
                _do_workspace_rename,
                core=self.core,
                workspace_id=workspace_fs.workspace_id,
                new_name=workspace_name,
                button=None,
            )

    def _on_switch_clicked(self, state, workspace_fs, timestamp):
        if state:
            self.mount_workspace(workspace_fs.workspace_id, timestamp)
        else:
            self.unmount_workspace(workspace_fs.workspace_id, timestamp)
        if not timestamp:
            self.update_workspace_config(workspace_fs.workspace_id, state)

    def open_workspace(self, workspace_fs):
        self.open_workspace_file(workspace_fs, None)

    def open_workspace_file(self, workspace_fs, file_name):
        file_name = FsPath("/", file_name) if file_name else FsPath("/")

        try:
            path = self.core.mountpoint_manager.get_path_in_mountpoint(
                workspace_fs.workspace_id,
                file_name,
                workspace_fs.timestamp
                if isinstance(workspace_fs, WorkspaceFSTimestamped)
                else None,
            )
            self.jobs_ctx.submit_job(
                self.file_open_success, self.file_open_error, desktop.open_files_job, [path]
            )
        except MountpointNotMounted:
            # The mountpoint has been umounted in our back, nothing left to do
            show_error(self, _("TEXT_FILE_OPEN_ERROR_file").format(file=str(file_name)))

    def _on_file_open_success(self, job):
        status, paths = job.ret
        if not status:
            show_error(self, _("TEXT_FILE_OPEN_ERROR_file").format(file=paths[0].name))

    def _on_file_open_error(self, job):
        logger.error("Failed to open the workspace in the explorer")

    def remount_workspace_ts(self, workspace_fs):
        def _on_finished(date, time):
            if not date or not time:
                return

            datetime = pendulum.local(
                date.year(), date.month(), date.day(), time.hour(), time.minute(), time.second()
            )
            self.mount_workspace(workspace_fs.workspace_id, datetime)

        TimestampedWorkspaceWidget.show_modal(
            workspace_fs=workspace_fs, jobs_ctx=self.jobs_ctx, parent=self, on_finished=_on_finished
        )

    def mount_workspace(self, workspace_id, timestamp=None):
        # In successful cases, the events MOUNTPOINT_STARTED
        # will take care of refreshing the state of the button,
        # the mount_success signal is not connected to anything but
        # is being kept for potential testing purposes
        self.jobs_ctx.submit_job(
            self.mount_success,
            self.mount_error,
            _do_workspace_mount,
            core=self.core,
            workspace_id=workspace_id,
            timestamp=timestamp,
        )

    def unmount_workspace(self, workspace_id, timestamp=None):
        # In successful cases, the event MOUNTPOINT_STOPPED
        # will take care of refreshing the state of the button,
        # the unmount_success signal is not connected to anything but
        # is being kept for potential testing purposes
        self.jobs_ctx.submit_job(
            self.unmount_success,
            self.unmount_error,
            _do_workspace_unmount,
            core=self.core,
            workspace_id=workspace_id,
            timestamp=timestamp,
        )

    def update_workspace_config(self, workspace_id, state):
        if state:
            self.disabled_workspaces -= {workspace_id}
        else:
            self.disabled_workspaces |= {workspace_id}
        self.event_bus.send(
            CoreEvent.GUI_CONFIG_CHANGED, disabled_workspaces=self.disabled_workspaces
        )

    def is_workspace_mounted(self, workspace_id, timestamp=None):
        return self.core.mountpoint_manager.is_workspace_mounted(workspace_id, timestamp)

    def delete_workspace(self, workspace_fs):
        if isinstance(workspace_fs, WorkspaceFSTimestamped):
            self.unmount_workspace(workspace_fs.workspace_id, workspace_fs.timestamp)
            return
        else:
            workspace_name = workspace_fs.get_workspace_name()
            result = ask_question(
                self,
                _("TEXT_WORKSPACE_DELETE_TITLE"),
                _("TEXT_WORKSPACE_DELETE_INSTRUCTIONS_workspace").format(workspace=workspace_name),
                [_("ACTION_DELETE_WORKSPACE_CONFIRM"), _("ACTION_CANCEL")],
            )
            if result != _("ACTION_DELETE_WORKSPACE_CONFIRM"):
                return
            # Workspace deletion is not available yet (button should be hidden anyway)

    def rename_workspace(self, workspace_button):
        new_name = get_text_input(
            self,
            _("TEXT_WORKSPACE_RENAME_TITLE"),
            _("TEXT_WORKSPACE_RENAME_INSTRUCTIONS"),
            placeholder=_("TEXT_WORKSPACE_RENAME_PLACEHOLDER"),
            default_text=workspace_button.name.str,
            button_text=_("ACTION_WORKSPACE_RENAME_CONFIRM"),
            validator=validators.WorkspaceNameValidator(),
        )
        if not new_name:
            return
        self.jobs_ctx.submit_job(
            self.rename_success,
            self.rename_error,
            _do_workspace_rename,
            core=self.core,
            workspace_id=workspace_button.workspace_fs.workspace_id,
            new_name=new_name,
            button=workspace_button,
        )

    def on_sharing_closing(self, has_changes):
        if has_changes:
            self.reset()

    def share_workspace(self, workspace_fs):
        WorkspaceSharingWidget.show_modal(
            user_fs=self.core.user_fs,
            workspace_fs=workspace_fs,
            core=self.core,
            jobs_ctx=self.jobs_ctx,
            parent=self,
            on_finished=self.on_sharing_closing,
        )

    def reencrypt_workspace(
        self, workspace_id, user_revoked, role_revoked, reencryption_already_in_progress
    ):
        if workspace_id in self.reencrypting or (
            not user_revoked and not role_revoked and not reencryption_already_in_progress
        ):
            return

        question = ""
        if user_revoked:
            question += "{}\n".format(_("TEXT_WORKSPACE_NEED_REENCRYPTION_BECAUSE_USER_REVOKED"))
        if role_revoked:
            question += "{}\n".format(_("TEXT_WORKSPACE_NEED_REENCRYPTION_BECAUSE_USER_REMOVED"))
        question += _("TEXT_WORKSPACE_NEED_REENCRYPTION_INSTRUCTIONS")

        r = ask_question(
            self,
            _("TEXT_WORKSPACE_NEED_REENCRYPTION_TITLE"),
            question,
            [_("ACTION_WORKSPACE_REENCRYPTION_CONFIRM"), _("ACTION_CANCEL")],
        )
        if r != _("ACTION_WORKSPACE_REENCRYPTION_CONFIRM"):
            return

        @contextmanager
        def _handle_fs_errors():
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

        async def _reencrypt(on_progress, workspace_id):
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
        workspace_button.reencrypting = 1, 0

        self.jobs_ctx.submit_job(
            self.workspace_reencryption_success,
            self.workspace_reencryption_error,
            _reencrypt,
            on_progress=self.workspace_reencryption_progress,
            workspace_id=workspace_id,
        )

    def _on_workspace_reencryption_success(self, job):
        workspace_id = job.arguments["workspace_id"]
        workspace_button = self.get_workspace_button(workspace_id, None)
        workspace_button.reencryption_needs = None
        workspace_button.reencrypting = None
        self.reencrypting.remove(workspace_id)

    def _on_workspace_reencryption_error(self, job):
        workspace_id = job.arguments["workspace_id"]
        workspace_button = self.get_workspace_button(workspace_id, None)
        workspace_button.reencrypting = None
        self.reencrypting.remove(workspace_id)
        if job.is_cancelled():
            return
        if job.status == "offline-backend":
            err_msg = _("TEXT_WORKPACE_REENCRYPT_OFFLINE_ERROR")
        elif job.status == "access-error":
            err_msg = _("TEXT_WORKPACE_REENCRYPT_ACCESS_ERROR")
        elif job.status == "not-found":
            err_msg = _("TEXT_WORKPACE_REENCRYPT_NOT_FOUND_ERROR")
        elif job.status == "fs-error":
            err_msg = _("TEXT_WORKPACE_REENCRYPT_FS_ERROR")
        else:
            err_msg = _("TEXT_WORKSPACE_REENCRYPT_UNKOWN_ERROR")
        show_error(self, err_msg, exception=job.exc)

    def get_workspace_button(self, workspace_id, timestamp=None):
        key = (workspace_id, timestamp)
        return self.workspace_button_mapping.get(key)

    def _on_workspace_reencryption_progress(self, workspace_id, total, done):
        wb = self.get_workspace_button(workspace_id, None)
        if done == total:
            wb.reencrypting = None
        else:
            wb.reencrypting = (total, done)

    def create_workspace_clicked(self):
        workspace_name = get_text_input(
            parent=self,
            title=_("TEXT_WORKSPACE_NEW_TITLE"),
            message=_("TEXT_WORKSPACE_NEW_INSTRUCTIONS"),
            placeholder=_("TEXT_WORKSPACE_NEW_PLACEHOLDER"),
            button_text=_("ACTION_WORKSPACE_NEW_CREATE"),
            validator=validators.WorkspaceNameValidator(),
        )
        if not workspace_name:
            return
        self.jobs_ctx.submit_job(
            self.create_success,
            self.create_error,
            _do_workspace_create,
            core=self.core,
            workspace_name=workspace_name,
        )

    def reset(self):
        self.list_workspaces()

    def list_workspaces(self):
        if not self.has_workspaces_displayed():
            self.layout_workspaces.clear()
            self.spinner.show()
        self.jobs_ctx.submit_throttled_job(
            "workspace_widget.list_workspaces",
            self.REFRESH_WORKSPACES_LIST_DELAY,
            self.list_success,
            self.list_error,
            _do_workspace_list,
            core=self.core,
        )

    def _on_sharing_updated(self, event, new_entry, previous_entry):
        self.reset()

    def _on_workspace_created(self, event, new_entry):
        self.reset()

    def _on_fs_entry_synced(self, event, id, workspace_id=None):
        self.reset()

    def _on_fs_entry_updated(self, event, workspace_id=None, id=None):
        assert id is not None
        if workspace_id and id == workspace_id:
            self.reset()

    def _on_entry_downsynced(self, event, workspace_id=None, id=None):
        self.reset()

    def _on_mountpoint_state_updated(self, workspace_id, timestamp):
        wb = self.get_workspace_button(workspace_id, timestamp)
        if wb:
            mounted = self.is_workspace_mounted(workspace_id, timestamp)
            wb.set_mountpoint_state(mounted)

    def _on_mountpoint_started(self, event, mountpoint, workspace_id, timestamp):
        self._on_mountpoint_state_updated(workspace_id, timestamp)

    def _on_mountpoint_stopped(self, event, mountpoint, workspace_id, timestamp):
        self._on_mountpoint_state_updated(workspace_id, timestamp)
