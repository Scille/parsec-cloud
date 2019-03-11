# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
from uuid import UUID
import pathlib

from PyQt5.QtCore import QCoreApplication, pyqtSignal

from parsec.core.types import FsPath
from parsec.core.fs import FSEntryNotFound
from parsec.core.mountpoint.exceptions import MountpointAlreadyMounted
from parsec.core.gui import desktop
from parsec.core.gui.custom_widgets import (
    show_error,
    show_warning,
    get_text,
    TaskbarButton,
)
from parsec.core.gui.core_widget import CoreWidget
from parsec.core.gui.workspace_button import WorkspaceButton
from parsec.core.gui.ui.workspaces_widget import Ui_WorkspacesWidget
from parsec.core.gui.workspace_sharing_dialog import WorkspaceSharingDialog


class WorkspacesWidget(CoreWidget, Ui_WorkspacesWidget):
    fs_changed_qt = pyqtSignal(str, UUID, str)
    load_workspace_clicked = pyqtSignal(str)

    COLUMNS_NUMBER = 3

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.fs_changed_qt.connect(self._on_fs_changed_qt)
        self.taskbar_buttons = []
        button_add_workspace = TaskbarButton(icon_path=":/icons/images/icons/plus_off.png")
        button_add_workspace.clicked.connect(self.create_workspace_clicked)
        self.taskbar_buttons.append(button_add_workspace)

    @CoreWidget.core.setter
    def core(self, c):
        if self._core:
            self._core.fs.event_bus.disconnect("fs.entry.updated", self._on_fs_entry_updated_trio)
            self._core.fs.event_bus.disconnect("fs.entry.synced", self._on_fs_entry_synced_trio)
        self._core = c
        if self._core:
            self._core.fs.event_bus.connect("fs.entry.updated", self._on_fs_entry_updated_trio)
            self._core.fs.event_bus.connect("fs.entry.synced", self._on_fs_entry_synced_trio)

    def load_workspace(self, workspace):
        self.load_workspace_clicked.emit(workspace)

    def add_workspace(self, workspace_name, is_owner, creator, files, shared_with=None):
        button = WorkspaceButton(
            workspace_name, is_owner, creator, shared_with=shared_with, files=files
        )
        button.clicked.connect(self.load_workspace)
        self.layout_workspaces.addWidget(
            button,
            int(self.workspaces_count / self.COLUMNS_NUMBER),
            int(self.workspaces_count % self.COLUMNS_NUMBER),
        )
        self.workspaces_count += 1
        button.share_clicked.connect(self.share_workspace)
        button.delete_clicked.connect(self.delete_workspace)
        button.rename_clicked.connect(self.rename_workspace)
        button.file_clicked.connect(self.open_workspace_file)
        try:
            self.portal.run(self.core.mountpoint_manager.mount_workspace, workspace_name)
        except MountpointAlreadyMounted:
            pass

    def open_workspace_file(self, workspace_name, file_name, is_dir):
        try:
            self.portal.run(self.core.mountpoint_manager.mount_workspace, workspace_name)
        except MountpointAlreadyMounted:
            pass
        except:
            show_error(self, QCoreApplication.translate("MountWidget", "Can not acces this file."))
            return
        path = FsPath("/") / workspace_name / file_name
        desktop.open_file(str(self.core.mountpoint_manager.get_path_in_mountpoint(path)))

    def get_taskbar_buttons(self):
        return self.taskbar_buttons

    def delete_workspace(self, workspace_button):
        show_warning(self, QCoreApplication.translate("WorkspacesWidget", "Not yet implemented."))

    def rename_workspace(self, workspace_button):
        current_file_path = os.path.join("/", workspace_button.name)
        new_name = get_text(
            self,
            QCoreApplication.translate("WorkspacesWidget", "New name"),
            QCoreApplication.translate("WorkspacesWidget", "Enter workspace new name"),
            placeholder=QCoreApplication.translate("WorkspacesWidget", "Workspace name"),
        )
        if not new_name:
            return
        try:
            self.portal.run(
                self.core.fs.workspace_rename, current_file_path, os.path.join("/", new_name)
            )
            workspace_button.name = new_name
        except FileExistsError:
            show_warning(
                self,
                QCoreApplication.translate(
                    "WorkspacesWidget", "A workspace with the same name already exists."
                ),
            )
        except:
            show_error(
                self,
                QCoreApplication.translate("WorkspacesWidget", "Can not rename the workspace."),
            )

    def share_workspace(self, workspace_button):
        d = WorkspaceSharingDialog(workspace_button.name, self.core, self.portal)
        d.exec_()

    def create_workspace_clicked(self):
        workspace_name = get_text(
            self,
            QCoreApplication.translate("WorkspacesWidget", "New workspace"),
            QCoreApplication.translate("WorkspacesWidget", "Enter new workspace name"),
            QCoreApplication.translate("WorkspacesWidget", "Workspace name"),
        )
        if not workspace_name:
            return
        try:
            self.portal.run(self.core.fs.workspace_create, os.path.join("/", workspace_name))
        except FileExistsError:
            show_error(
                self,
                QCoreApplication.translate(
                    "WorkspacesWidget", "A workspace with the same name already exists."
                ),
            )
            return

    def reset(self):
        self.workspaces_count = 0
        while self.layout_workspaces.count() != 0:
            item = self.layout_workspaces.takeAt(0)
            if item:
                w = item.widget()
                self.layout_workspaces.removeWidget(w)
                w.setParent(None)
        if self.portal:
            result = self.portal.run(self.core.fs.stat, "/")
            user_id = self.core.device.user_id
            for workspace in result.get("children", []):
                ws_infos = self.portal.run(self.core.fs.stat, os.path.join("/", workspace))
                ws_infos["participants"].remove(user_id)
                files = ws_infos["children"][:4]
                display_files = {}
                for f in files:
                    f_infos = self.portal.run(self.core.fs.stat, os.path.join("/", workspace, f))
                    display_files[f] = f_infos["is_folder"]
                self.add_workspace(
                    workspace,
                    user_id == ws_infos["creator"],
                    ws_infos["creator"],
                    files=display_files,
                    shared_with=ws_infos["participants"],
                )

    def _on_fs_entry_synced_trio(self, event, path, id):
        self.fs_changed_qt.emit(event, id, path)

    def _on_fs_entry_updated_trio(self, event, id):
        self.fs_changed_qt.emit(event, id, None)

    def _on_fs_changed_qt(self, event, id, path):
        if not path:
            try:
                path = self.portal.run(self.core.fs.get_entry_path, id)
            except FSEntryNotFound:
                # Entry not locally present, nothing to do
                return

        # We are only interested into modification of root manifest given we
        # don't care about the date of modification of the workspaces)
        path = pathlib.PurePath(path)
        if len(path.parts) <= 2:
            self.reset()
