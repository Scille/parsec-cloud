# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from uuid import UUID

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget

from parsec.core.types import FsPath, WorkspaceEntry
from parsec.core.fs import WorkspaceFS
from parsec.core.mountpoint.exceptions import MountpointAlreadyMounted, MountpointDisabled
from parsec.core.gui import desktop
from parsec.core.gui.custom_widgets import show_error, show_warning, get_text, TaskbarButton
from parsec.core.gui.lang import translate as _
from parsec.core.gui.workspace_button import WorkspaceButton
from parsec.core.gui.ui.workspaces_widget import Ui_WorkspacesWidget
from parsec.core.gui.workspace_sharing_dialog import WorkspaceSharingDialog


class WorkspacesWidget(QWidget, Ui_WorkspacesWidget):
    _fs_changed_qt = pyqtSignal(str, UUID, str)
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
        self._workspace_created_qt.connect(self._on_workspace_created_qt)

        self.taskbar_buttons.append(button_add_workspace)

        user_manifest = self.core.fs._user_fs.get_user_manifest()
        for count, workspace in enumerate(user_manifest.workspaces):
            workspace_id = workspace.access.id
            workspace_fs = self.core.fs._user_fs.get_workspace(workspace_id)
            self.add_workspace(workspace_fs, count)

    def load_workspace(self, workspace):
        self.load_workspace_clicked.emit(workspace)

    def add_workspace(self, workspace_fs, count=None):
        workspace_info = self.jobs_ctx.run(workspace_fs.workspace_info)
        workspace_files = self.jobs_ctx.run(workspace_fs.entry_info, FsPath("/"))
        button = WorkspaceButton(
            workspace_fs,
            participants=workspace_info["participants"],
            is_creator=workspace_info["creator"] == self.core.device.user_id,
            files=workspace_files["children"][:4],
        )
        if count is None:
            count = len(self.core.fs._user_fs.get_user_manifest().workspaces) - 1

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
                self.core.mountpoint_manager.mount_workspace, workspace_fs.workspace_entry.access.id
            )
        except (MountpointAlreadyMounted, MountpointDisabled):
            pass

    def open_workspace_file(self, workspace_fs, file_name):
        try:
            self.jobs_ctx.run(
                self.core.mountpoint_manager.mount_workspace, workspace_fs.workspace_entry.access.id
            )
        except MountpointAlreadyMounted:
            pass
        except Exception:
            show_error(self, _("Can not acces this file."))
            return
        path = self.core.mountpoint_manager.get_path_in_mountpoint(
            workspace_fs.workspace_entry.access.id, file_name
        )
        desktop.open_file(str(path))

    def get_taskbar_buttons(self):
        return self.taskbar_buttons

    def delete_workspace(self, workspace_button):
        show_warning(self, _("Not yet implemented."))

    def rename_workspace(self, workspace_button):
        new_name = get_text(
            self, _("New name"), _("Enter workspace new name"), placeholder=_("Workspace name")
        )
        if not new_name:
            return
        try:
            workspace_id = workspace_button.workspace_fs.workspace_entry.access.id
            self.jobs_ctx.run(self.core.fs._user_fs.workspace_rename, workspace_id, new_name)
            workspace_button.workspace_fs = self.core.fs._user_fs.get_workspace(workspace_id)
            workspace_button.name = new_name
        except FileExistsError:
            show_warning(self, _("A workspace with the same name already exists."))
        except:
            show_error(self, _("Can not rename the workspace."))
        else:
            self.jobs_ctx.run(self.core.mountpoint_manager.unmount_workspace, workspace_id)
            self.jobs_ctx.run(self.core.mountpoint_manager.mount_workspace, workspace_id)

    def share_workspace(self, workspace_button):
        d = WorkspaceSharingDialog(workspace_button.name, self.core, self.jobs_ctx)
        d.exec_()

    def create_workspace_clicked(self):
        workspace_name = get_text(
            self, _("New workspace"), _("Enter new workspace name"), _("Workspace name")
        )
        if not workspace_name:
            return
        try:
            self.jobs_ctx.run(self.core.fs._user_fs.workspace_create, workspace_name)
        except FileExistsError:
            show_error(self, _("A workspace with the same name already exists."))

    def _on_workspace_created_trio(self, event, new_entry):
        self._workspace_created_qt.emit(new_entry)

    def _on_workspace_created_qt(self, workspace_entry):
        workspace_fs = self.core.fs._user_fs.get_workspace(workspace_entry.access.id)
        self.add_workspace(workspace_fs)
