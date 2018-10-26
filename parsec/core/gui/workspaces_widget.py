import os

from PyQt5.QtCore import pyqtSignal, QCoreApplication, QPoint, QSize, Qt
from PyQt5.QtGui import QIcon, QPixmap
from PyQt5.QtWidgets import QWidget, QInputDialog

from parsec.core.gui.core_call import core_call
from parsec.core.gui.ui.workspaces_widget import Ui_WorkspacesWidget
from parsec.core.gui.ui.workspace_button import Ui_WorkspaceButton


class WorkspaceButton(QWidget, Ui_WorkspaceButton):
    context_menu_requested = pyqtSignal(str, QPoint)
    clicked = pyqtSignal(str)

    def __init__(self, workspace_name, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.workspace_name = workspace_name
        self.label_workspace.setText(self.workspace_name)
        self.setContextMenuPolicy(Qt.CustomContextMenu)
        self.customContextMenuRequested.connect(self.emit_context_menu_requested)

    def mousePressEvent(self, event):
        if event.button() & Qt.LeftButton:
            self.clicked.emit(self.workspace_name)

    def emit_context_menu_requested(self, pos):
        self.context_menu_requested.emit(self.workspace_name, pos)


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
        action.triggered.connect(self.share_workspace(workspace_button.text()))
        action = menu.addAction(QCoreApplication.translate("WorkspacesWidget", "Rename"))
        action.triggered.connect(self.action_move_workspace(workspace_button.text()))
        menu.exec_(global_pos)

    def action_move_workspace(self, workspace_name):
        def _inner_move_workspace():
            current_file_path = os.path.join("/", workspace_name)
            new_name, ok = QInputDialog.getText(
                self,
                QCoreApplication.translate("WorkspacesWidget", "New name"),
                QCoreApplication.translate("WorkspacesWidget", "Enter workspace new name"),
            )
            if not ok:
                return
            if not new_name:
                show_warning(
                    self, QCoreApplication.translate("WorkspacesWidget", "This name is not valid.")
                )
                return
            if new_name in self._workspace_names():
                show_error(
                    self,
                    QCoreApplication.translate(
                        "WorkspacesWidget", "A workspace of the same name already exists."
                    ),
                )
                return
            core_call().move_workspace(current_file_path, os.path.join("/", new_name))

        return _inner_move_workspace

    def share_workspace(self, workspace_name):
        def _inner_share_workspace():
            user, ok = QInputDialog.getText(
                self,
                QCoreApplication.translate("WorkspacesWidget", "Share a workspace"),
                QCoreApplication.translate(
                    "WorkspacesWidget", "Give a user name to share the workspace {} with."
                ).format(workspace_name),
            )
            if not ok or not user:
                return
            try:
                core_call().share_workspace("/" + workspace_name, user)
            except SharingRecipientError:
                show_warning(
                    self,
                    QCoreApplication.translate(
                        "WorkspacesWidget", 'Can not share the workspace "{}" with this user.'
                    ).format(workspace_name),
                )
            except:
                show_error(
                    self,
                    QCoreApplication.translate(
                        "WorkspacesWidget", 'Can not share the workspace "{}" with "{}".'
                    ).format(workspace_name, user),
                )

        return _inner_share_workspace

    def create_workspace_clicked(self):
        workspace_name, ok = QInputDialog.getText(
            self,
            QCoreApplication.translate("WorkspacesWidget", "New workspace"),
            QCoreApplication.translate("WorkspacesWidget", "Enter new workspace name"),
        )
        if not ok or not workspace_name:
            return
        try:
            core_call().create_workspace(os.path.join("/", workspace_name))
            self.add_workspace(workspace_name)
        except FileExistsError:
            show_warning(
                self,
                QCoreApplication.translate(
                    "WorkspacesWidget", "A workspace with the same name already exists."
                ),
            )
            return

    def reset(self):
        self.workspaces_count = 0
        for i in range(self.layout_workspaces.count()):
            item = self.layout_workspaces.takeAt(i)
            if item:
                w = item.widget()
                self.layout_workspaces.removeWidget(w)
                w.setParent(None)
        result = core_call().stat("/")
        for workspace in result.get("children", []):
            self.add_workspace(workspace)
