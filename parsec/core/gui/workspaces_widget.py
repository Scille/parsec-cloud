import os
from uuid import UUID

from PyQt5.QtCore import pyqtSignal, QCoreApplication, QPoint, Qt
from PyQt5.QtWidgets import QWidget, QMenu, QDialog

from parsec.core.gui.custom_widgets import show_error, show_warning, show_info, get_text
from parsec.core.gui.core_call import core_call
from parsec.core.gui.ui.workspaces_widget import Ui_WorkspacesWidget
from parsec.core.gui.ui.workspace_button import Ui_WorkspaceButton
from parsec.core.gui.ui.shared_dialog import Ui_SharedDialog

from parsec.core.fs import FSEntryNotFound
from parsec.core.fs.sharing import SharingRecipientError


class SharedDialog(QDialog, Ui_SharedDialog):
    def __init__(self, owner, shared_list, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.setWindowFlags(Qt.SplashScreen)
        if owner:
            self.label_title.setText(QCoreApplication.translate("WorkspacesWidget", "Shared with"))
        else:
            self.label_title.setText(QCoreApplication.translate("WorkspacesWidget", "Shared by"))
        for user in shared_list:
            self.list_shared.addItem(user)


class WorkspaceButton(QWidget, Ui_WorkspaceButton):
    context_menu_requested = pyqtSignal(QWidget, QPoint)
    clicked = pyqtSignal(str)

    def __init__(self, workspace_name, creator, shared_with=None, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.creator = creator
        self.shared_with = shared_with
        self.name = workspace_name
        self.setContextMenuPolicy(Qt.CustomContextMenu)
        self.customContextMenuRequested.connect(self.emit_context_menu_requested)

    @property
    def name(self):
        return self.workspace_name

    @name.setter
    def name(self, value):
        self.workspace_name = value
        if self.shared_with:
            if self.is_owner:
                self.label_workspace.setText(
                    QCoreApplication.translate("WorkspacesWidget", "{} (shared)".format(value))
                )
            else:
                self.label_workspace.setText(
                    QCoreApplication.translate(
                        "WorkspacesWidget", "{} (shared by {})".format(value, self.creator)
                    )
                )
        else:
            self.label_workspace.setText(value)

    @property
    def is_owner(self):
        return self.shared_with and self.creator not in self.shared_with

    def mousePressEvent(self, event):
        if event.button() & Qt.LeftButton:
            self.clicked.emit(self.workspace_name)

    def emit_context_menu_requested(self, pos):
        self.context_menu_requested.emit(self, pos)


class WorkspacesWidget(QWidget, Ui_WorkspacesWidget):
    fs_changed_qt = pyqtSignal(str, UUID, str)
    load_workspace_clicked = pyqtSignal(str)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        core_call().connect_event("fs.entry.updated", self._on_fs_entry_updated_trio)
        core_call().connect_event("fs.entry.synced", self._on_fs_entry_synced_trio)
        self.fs_changed_qt.connect(self._on_fs_changed_qt)
        self.button_add_workspace.clicked.connect(self.create_workspace_clicked)

    def load_workspace(self, workspace):
        self.load_workspace_clicked.emit(workspace)

    def add_workspace(self, workspace_name, creator, shared_with=None):
        button = WorkspaceButton(workspace_name, creator, shared_with)
        button.clicked.connect(self.load_workspace)
        self.layout_workspaces.addWidget(
            button, int(self.workspaces_count / 4), int(self.workspaces_count % 4)
        )
        self.workspaces_count += 1
        button.context_menu_requested.connect(self.workspace_context_menu_clicked)

    def workspace_context_menu_clicked(self, workspace_button, pos):
        global_pos = workspace_button.mapToGlobal(pos)
        menu = QMenu(workspace_button)
        action = menu.addAction(QCoreApplication.translate("WorkspacesWidget", "Share"))
        action.triggered.connect(self.share_workspace(workspace_button))
        action = menu.addAction(QCoreApplication.translate("WorkspacesWidget", "Rename"))
        action.triggered.connect(self.action_move_workspace(workspace_button))
        if workspace_button.shared_with:
            action = menu.addAction(
                QCoreApplication.translate("WorkspacesWidget", "See sharing infos")
            )
            action.triggered.connect(self.action_show_sharing_infos(workspace_button))
        menu.exec_(global_pos)

    def action_show_sharing_infos(self, workspace_button):
        def _inner_show_sharing_infos():
            s = SharedDialog(workspace_button.is_owner, workspace_button.shared_with)
            s.exec_()

        return _inner_show_sharing_infos

    def action_move_workspace(self, workspace_button):
        def _inner_move_workspace():
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
                core_call().move_workspace(current_file_path, os.path.join("/", new_name))
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

        return _inner_move_workspace

    def share_workspace(self, workspace_button):
        def _inner_share_workspace():
            current_device = core_call().logged_device()
            current_user = current_device.id.split("@")[0]
            user = get_text(
                self,
                QCoreApplication.translate("WorkspacesWidget", "Share a workspace"),
                QCoreApplication.translate(
                    "WorkspacesWidget", "Give a user name to share the workspace {} with."
                ).format(workspace_button.name),
                placeholder=QCoreApplication.translate("WorkspacesWidget", "User name"),
                completion=[
                    d.split("@")[0]
                    for d in core_call().get_devices()
                    if d.split("@")[0] != current_user
                ],
            )
            if not user:
                return
            try:
                core_call().share_workspace("/" + workspace_button.name, user)
                show_info(
                    self,
                    QCoreApplication.translate(
                        "WorkspacesWidget", "The workspaces has been shared."
                    ),
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

        return _inner_share_workspace

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
            self.add_workspace(workspace_name)
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
            if user_id == ws_infos["creator"]:
                ws_infos["participants"].remove(user_id)
            self.add_workspace(workspace, ws_infos["creator"], ws_infos["participants"])

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
