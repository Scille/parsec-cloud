import os

from PyQt5.QtCore import pyqtSignal, QCoreApplication, QPoint, QSize, Qt
from PyQt5.QtGui import QIcon, QPixmap
from PyQt5.QtWidgets import QWidget, QMenu

from parsec.core.gui.custom_widgets import show_error, show_warning, show_info, get_text
from parsec.core.gui.core_call import core_call
from parsec.core.gui.ui.workspaces_widget import Ui_WorkspacesWidget
from parsec.core.gui.ui.workspace_button import Ui_WorkspaceButton

from parsec.core.fs.sharing import SharingRecipientError


class WorkspaceButton(QWidget, Ui_WorkspaceButton):
    context_menu_requested = pyqtSignal(QWidget, QPoint)
    clicked = pyqtSignal(str)

    def __init__(self, workspace_name, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.workspace_name = workspace_name
        self.label_workspace.setText(self.workspace_name)
        self.setContextMenuPolicy(Qt.CustomContextMenu)
        self.customContextMenuRequested.connect(self.emit_context_menu_requested)

    @property
    def name(self):
        return self.label_workspace.text()

    @name.setter
    def name(self, value):
        self.label_workspace.setText(value)

    def mousePressEvent(self, event):
        if event.button() & Qt.LeftButton:
            self.clicked.emit(self.workspace_name)

    def emit_context_menu_requested(self, pos):
        self.context_menu_requested.emit(self, pos)


class WorkspacesWidget(QWidget, Ui_WorkspacesWidget):
    load_workspace_clicked = pyqtSignal(str)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.reset()
        self.button_add_workspace.clicked.connect(self.create_workspace_clicked)

    def load_workspace(self, workspace):
        self.load_workspace_clicked.emit(workspace)

    def add_workspace(self, workspace_name):
        button = WorkspaceButton(workspace_name)
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
        menu.exec_(global_pos)

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
            user = get_text(
                self,
                QCoreApplication.translate("WorkspacesWidget", "Share a workspace"),
                QCoreApplication.translate(
                    "WorkspacesWidget", "Give a user name to share the workspace {} with."
                ).format(workspace_button.name),
                placeholder=QCoreApplication.translate("WorkspacesWidget", "User name"),
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
        for workspace in result.get("children", []):
            self.add_workspace(workspace)
