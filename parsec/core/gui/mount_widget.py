# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget

from parsec.core.gui.files_widget import FilesWidget
from parsec.core.gui.workspaces_widget import WorkspacesWidget

from parsec.core.gui.ui.mount_widget import Ui_MountWidget


class MountWidget(QWidget, Ui_MountWidget):
    widget_switched = pyqtSignal(list)

    def __init__(self, core, jobs_ctx, event_bus, **kwargs):
        super().__init__(**kwargs)
        self.setupUi(self)

        self.core = core
        self.jobs_ctx = jobs_ctx
        self.event_bus = event_bus
        self.workspaces_widget = WorkspacesWidget(
            self.core, self.jobs_ctx, self.event_bus, parent=self
        )
        self.layout_content.insertWidget(0, self.workspaces_widget)
        self.workspaces_widget.load_workspace_clicked.connect(self.load_workspace)
        self.files_widget = FilesWidget(self.core, self.jobs_ctx, self.event_bus, parent=self)
        self.layout_content.insertWidget(0, self.files_widget)
        self.files_widget.back_clicked.connect(self.show_workspaces_widget)
        self.files_widget.taskbar_updated.connect(self.on_taskbar_updated)
        self.widget_switched.emit(self.get_taskbar_buttons())
        self.show_workspaces_widget()

    def disconnect_all(self):
        self.files_widget.disconnect_all()
        self.workspaces_widget.disconnect_all()

    def load_workspace(self, workspace_fs):
        self.show_files_widget(workspace_fs)

    def get_taskbar_buttons(self):
        if not self.files_widget.isVisible() and not self.workspaces_widget.isVisible():
            return self.workspaces_widget.get_taskbar_buttons().copy()
        elif self.files_widget.isVisible():
            return self.files_widget.get_taskbar_buttons().copy()
        elif self.workspaces_widget.isVisible():
            return self.workspaces_widget.get_taskbar_buttons().copy()
        return []

    def on_taskbar_updated(self):
        self.widget_switched.emit(self.get_taskbar_buttons())

    def show_files_widget(self, workspace_fs):
        self.workspaces_widget.hide()
        self.files_widget.workspace_fs = workspace_fs
        self.files_widget.show()
        self.widget_switched.emit(self.files_widget.get_taskbar_buttons().copy())

    def show_workspaces_widget(self):
        self.files_widget.hide()
        self.workspaces_widget.show()
        self.workspaces_widget.reset()
        self.widget_switched.emit(self.workspaces_widget.get_taskbar_buttons().copy())
