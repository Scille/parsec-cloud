import os
from uuid import UUID

from PyQt5.QtCore import pyqtSignal, QCoreApplication, QPoint, Qt
from PyQt5.QtWidgets import QWidget, QMenu, QDialog

from parsec.core.gui.custom_widgets import (
    show_error,
    show_warning,
    show_info,
    get_text,
    get_user_name,
)
from parsec.core.gui.core_call import core_call
from parsec.core.gui.workspace_button import WorkspaceButton
from parsec.core.gui.ui.workspaces_widget import Ui_WorkspacesWidget
from parsec.core.gui.ui.shared_dialog import Ui_SharedDialog
from parsec.core.fs import FSEntryNotFound
from parsec.core.fs.sharing import SharingRecipientError


class SharedDialog(QDialog, Ui_SharedDialog):
    def __init__(self, is_owner, creator, shared_list, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.setWindowFlags(Qt.SplashScreen)
        if is_owner:
            self.label_title.setText(QCoreApplication.translate("WorkspacesWidget", "Shared with"))
            for user in shared_list:
                self.list_shared.addItem(user)
        else:
            self.label_title.setText(QCoreApplication.translate("WorkspacesWidget", "Shared by"))
            self.list_shared.addItem(creator)


class WorkspacesWidget(QWidget, Ui_WorkspacesWidget):
    fs_changed_qt = pyqtSignal(str, UUID, str)
    load_workspace_clicked = pyqtSignal(str)

    COLUMNS_NUMBER = 3

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        core_call().connect_event("fs.entry.updated", self._on_fs_entry_updated_trio)
        core_call().connect_event("fs.entry.synced", self._on_fs_entry_synced_trio)
        self.fs_changed_qt.connect(self._on_fs_changed_qt)
        self.button_add_workspace.clicked.connect(self.create_workspace_clicked)

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

    def show_workspace_details(self, workspace_button):
        s = SharedDialog(
            workspace_button.is_owner, workspace_button.creator, workspace_button.shared_with
        )
        s.exec_()

    def action_rename_workspace(self, workspace_button):
        def _inner_rename_workspace():
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
                core_call().rename_workspace(current_file_path, os.path.join("/", new_name))
                workspace_button.rename(new_name)
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

        return _inner_rename_workspace

    def share_workspace(self, workspace_button):
        current_device = core_call().logged_device()
        current_user = current_device.id.split("@")[0]
        user = get_user_name(
            self,
            title=QCoreApplication.translate("WorkspacesWidget", "Share a workspace"),
            message=QCoreApplication.translate(
                "WorkspacesWidget", "Give a user name to share the workspace {} with."
            ).format(workspace_button.name),
            exclude=[current_user],
        )
        if not user:
            return
        try:
            core_call().share_workspace("/" + workspace_button.name, user)
            show_info(
                self,
                QCoreApplication.translate("WorkspacesWidget", "The workspaces has been shared."),
            )
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
            core_call().create_workspace(os.path.join("/", workspace_name))
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
        result = core_call().stat("/")
        logged_device = core_call().logged_device()
        user_id = logged_device.id.split("@")[0]
        for workspace in result.get("children", []):
            ws_infos = core_call().stat("/{}".format(workspace))
            ws_infos["participants"].remove(user_id)
            self.add_workspace(
                workspace,
                user_id == ws_infos["creator"],
                ws_infos["creator"],
                files=ws_infos["children"][:4],
                shared_with=ws_infos["participants"],
            )

    def _on_fs_entry_synced_trio(self, event, path, id):
        self.fs_changed_qt.emit(event, id, path)

    def _on_fs_entry_updated_trio(self, event, id):
        self.fs_changed_qt.emit(event, id, None)

    def _on_fs_changed_qt(self, event, id, path):
        if not path:
            try:
                path = core_call().get_entry_path(id)
            except FSEntryNotFound:
                # Entry not locally present, nothing to do
                return

        # We are only interested into modification of root manifest given we
        # don't care about the date of modification of the workspaces)
        if path == "/":
            self.reset()
