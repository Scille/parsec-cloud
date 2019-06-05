# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from uuid import UUID

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget

from parsec.core.types import WorkspaceEntry, FsPath, WorkspaceRole
from parsec.core.fs import WorkspaceFS
from parsec.core.mountpoint.exceptions import MountpointAlreadyMounted, MountpointDisabled
from parsec.core.gui import desktop
from parsec.core.gui.custom_widgets import show_error, show_warning, get_text, TaskbarButton
from parsec.core.gui.lang import translate as _
from parsec.core.gui.workspace_button import WorkspaceButton
from parsec.core.gui.ui.workspaces_widget import Ui_WorkspacesWidget
from parsec.core.gui.workspace_sharing_dialog import WorkspaceSharingDialog


class WorkspacesWidget(QWidget, Ui_WorkspacesWidget):
    fs_updated_qt = pyqtSignal(str, UUID)
    fs_synced_qt = pyqtSignal(str, UUID)
    _workspace_created_qt = pyqtSignal(WorkspaceEntry)
    load_workspace_clicked = pyqtSignal(WorkspaceFS)

    COLUMNS_NUMBER = 3

    def __init__(self, core, jobs_ctx, event_bus, **kwargs):
        super().__init__(**kwargs)
        self.setupUi(self)

        self.core = core
        self.jobs_ctx = jobs_ctx
        self.event_bus = event_bus

        self.taskbar_buttons = []

        button_add_workspace = TaskbarButton(icon_path=":/icons/images/icons/plus_off.png")
        button_add_workspace.clicked.connect(self.create_workspace_clicked)

        self.event_bus.connect("fs.workspace.created", self._on_workspace_created_trio)
        self.event_bus.connect("fs.entry.updated", self._on_fs_entry_updated_trio)
        self.event_bus.connect("fs.entry.synced", self._on_fs_entry_synced_trio)

        self.fs_updated_qt.connect(self._on_fs_updated_qt)
        self.fs_synced_qt.connect(self._on_fs_synced_qt)

        self._workspace_created_qt.connect(self._on_workspace_created_qt)

        self.taskbar_buttons.append(button_add_workspace)
        self.reset()

    def disconnect_all(self):
        self.event_bus.disconnect("fs.workspace.created", self._on_workspace_created_trio)
        self.event_bus.disconnect("fs.entry.updated", self._on_fs_entry_updated_trio)
        self.event_bus.disconnect("fs.entry.synced", self._on_fs_entry_synced_trio)

    def load_workspace(self, workspace_fs):
        self.load_workspace_clicked.emit(workspace_fs)

    def add_workspace(self, workspace_fs, count=None):
        # TODO: workspace's participants must be fetched from the backend
        ws_entry = workspace_fs.get_workspace_entry()
        users_roles = self.jobs_ctx.run(workspace_fs.get_user_roles)
        root_info = self.jobs_ctx.run(workspace_fs.path_info, "/")
        button = WorkspaceButton(
            workspace_fs,
            is_shared=len(users_roles) > 1,
            is_creator=ws_entry.role == WorkspaceRole.OWNER,
            files=root_info["children"][:4],
            enable_workspace_color=self.core.config.gui_workspace_color,
        )
        if count is None:
            count = len(self.core.user_fs.get_user_manifest().workspaces) - 1

        self.layout_workspaces.addWidget(
            button, int(count / self.COLUMNS_NUMBER), int(count % self.COLUMNS_NUMBER)
        )
        button.clicked.connect(self.load_workspace)
        button.share_clicked.connect(self.share_workspace)
        button.delete_clicked.connect(self.delete_workspace)
        button.rename_clicked.connect(self.rename_workspace)
        button.file_clicked.connect(self.open_workspace_file)
        try:
            self.jobs_ctx.run(
                self.core.mountpoint_manager.mount_workspace, workspace_fs.workspace_id
            )
        except (MountpointAlreadyMounted, MountpointDisabled):
            pass

    def open_workspace_file(self, workspace_fs, file_name):
        try:
            self.jobs_ctx.run(
                self.core.mountpoint_manager.mount_workspace, workspace_fs.workspace_id
            )
        except MountpointAlreadyMounted:
            pass
        except Exception:
            show_error(self, _("Can not acces this file."))
            return
        file_name = FsPath("/", file_name)
        path = self.core.mountpoint_manager.get_path_in_mountpoint(
            workspace_fs.workspace_id, file_name
        )
        desktop.open_file(str(path))

    def get_taskbar_buttons(self):
        return self.taskbar_buttons

    def delete_workspace(self, workspace_entry):
        show_warning(self, _("Not yet implemented."))

    def rename_workspace(self, workspace_button):
        new_name = get_text(
            self, _("New name"), _("Enter workspace new name"), placeholder=_("Workspace name")
        )
        if not new_name:
            return
        try:
            workspace_id = workspace_button.workspace_fs.workspace_id
            self.jobs_ctx.run(self.core.user_fs.workspace_rename, workspace_id, new_name)
            workspace_button.reload_workspace_name()
        except:
            show_error(self, _("Can not rename the workspace."))
        else:
            self.jobs_ctx.run(self.core.mountpoint_manager.unmount_workspace, workspace_id)
            self.jobs_ctx.run(self.core.mountpoint_manager.mount_workspace, workspace_id)

    def share_workspace(self, workspace_fs):
        d = WorkspaceSharingDialog(
            self.core.user_fs.user_fs, workspace_fs, self.core, self.jobs_ctx
        )
        d.exec_()

    def create_workspace_clicked(self):
        workspace_name = get_text(
            self, _("New workspace"), _("Enter new workspace name"), _("Workspace name")
        )
        if not workspace_name:
            return
        self.jobs_ctx.run(self.core.user_fs.workspace_create, workspace_name)

    def reset(self):
        while self.layout_workspaces.count() != 0:
            item = self.layout_workspaces.takeAt(0)
            if item:
                w = item.widget()
                self.layout_workspaces.removeWidget(w)
                w.setParent(None)
        user_manifest = self.core.user_fs.get_user_manifest()
        for count, workspace in enumerate(user_manifest.workspaces):
            workspace_id = workspace.id
            workspace_fs = self.core.user_fs.get_workspace(workspace_id)
            self.add_workspace(workspace_fs, count)

    def _on_workspace_created_trio(self, event, new_entry):
        self._workspace_created_qt.emit(new_entry)

    def _on_workspace_created_qt(self, workspace_entry):
        workspace_fs = self.core.user_fs.get_workspace(workspace_entry.id)
        self.add_workspace(workspace_fs)

    def _on_fs_entry_synced_trio(self, event, id, path=None, workspace_id=None):
        self.fs_synced_qt.emit(event, id)

    def _on_fs_entry_updated_trio(self, event, workspace_id=None, id=None):
        if workspace_id and not id:
            self.fs_updated_qt.emit(event, workspace_id)

    def _on_fs_synced_qt(self, event, id):
        self.reset()

    def _on_fs_updated_qt(self, event, workspace_id):
        self.reset()
