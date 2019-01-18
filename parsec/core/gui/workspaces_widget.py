import os
from uuid import UUID
import pathlib

from PyQt5.QtCore import QCoreApplication, pyqtSignal
from PyQt5.QtWidgets import QWidget

from parsec.core.gui.custom_widgets import (
    show_error,
    show_warning,
    show_info,
    get_text,
    get_user_name,
)
from parsec.core.gui.workspace_button import WorkspaceButton
from parsec.core.gui.ui.workspaces_widget import Ui_WorkspacesWidget
from parsec.core.fs import FSEntryNotFound
from parsec.core.fs.sharing import SharingRecipientError


class WorkspacesWidget(QWidget, Ui_WorkspacesWidget):
    fs_changed_qt = pyqtSignal(str, UUID, str)
    load_workspace_clicked = pyqtSignal(str)

    COLUMNS_NUMBER = 3

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self._core = None
        self._portal = None
        self.fs_changed_qt.connect(self._on_fs_changed_qt)
        self.button_add_workspace.clicked.connect(self.create_workspace_clicked)

    @property
    def core(self):
        return self._core

    @core.setter
    def core(self, c):
        if self._core:
            self.core.fs.event_bus.disconnect("fs.entry.updated", self._on_fs_entry_updated_trio)
            self.core.fs.event_bus.disconnect("fs.entry.synced", self._on_fs_entry_synced_trio)
        self._core = c
        if self._core:
            self.core.fs.event_bus.connect("fs.entry.updated", self._on_fs_entry_updated_trio)
            self.core.fs.event_bus.connect("fs.entry.synced", self._on_fs_entry_synced_trio)

    @property
    def portal(self):
        return self._portal

    @portal.setter
    def portal(self, p):
        self._portal = p

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
        button.details_clicked.connect(self.show_workspace_details)
        button.delete_clicked.connect(self.delete_workspace)
        button.rename_clicked.connect(self.rename_workspace)

    def show_workspace_details(self, workspace_button):
        text = QCoreApplication.translate("WorkspacesWidget", "{}\n\nCreated by {}.\n").format(
            workspace_button.name, workspace_button.creator
        )
        if len(workspace_button.participants):
            text += QCoreApplication.translate("WorkspacesWidget", "Shared with {} people.").format(
                len(workspace_button.participants)
            )
        else:
            text += QCoreApplication.translate("WorkspacesWidget", "Not shared.")
        show_info(self, text)

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
        current_user = self.core.device.user_id
        user = get_user_name(
            portal=self.portal,
            core=self.core,
            parent=self,
            title=QCoreApplication.translate("WorkspacesWidget", "Share a workspace"),
            message=QCoreApplication.translate(
                "WorkspacesWidget", "Give a user name to share the workspace {} with."
            ).format(workspace_button.name),
            exclude=[current_user],
        )
        if not user:
            return
        try:
            self.portal.run(self.core.fs.share, "/" + workspace_button.name, user)
            show_info(
                self,
                QCoreApplication.translate("WorkspacesWidget", "The workspaces has been shared."),
            )
            workspace_button.participants = workspace_button.participants + [user]
        except SharingRecipientError:
            show_warning(
                self,
                QCoreApplication.translate(
                    "WorkspacesWidget", 'Can not share the workspace "{}" with this user.'
                ).format(workspace_button.name),
            )
        except:
            show_error(
                self,
                QCoreApplication.translate(
                    "WorkspacesWidget", 'Can not share the workspace "{}" with "{}".'
                ).format(workspace_button.name, user),
            )

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
