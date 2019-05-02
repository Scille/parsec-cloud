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
        self.show_workspaces_widget()

    def disconnect_all(self):
        pass

    def load_workspace(self, workspace_fs):
        self.show_files_widget(workspace_fs)

    def get_taskbar_buttons(self):
        item = self.layout_content.itemAt(0)
        if item:
            return item.widget().get_taskbar_buttons().copy()
        return []

    def show_files_widget(self, workspace_fs):
        self.clear_widgets()
        files_widget = FilesWidget(
            self.core, self.jobs_ctx, self.event_bus, workspace_fs, parent=self
        )
        self.layout_content.insertWidget(0, files_widget)
        files_widget.back_clicked.connect(self.show_workspaces_widget)
        self.widget_switched.emit(self.get_taskbar_buttons())

    def show_workspaces_widget(self):
        self.clear_widgets()
        workspaces_widget = WorkspacesWidget(self.core, self.jobs_ctx, self.event_bus, parent=self)
        self.layout_content.insertWidget(0, workspaces_widget)
        workspaces_widget.load_workspace_clicked.connect(self.load_workspace)
        self.widget_switched.emit(self.get_taskbar_buttons())

    def clear_widgets(self):
        item = self.layout_content.takeAt(0)
        if item:
            item.widget().disconnect_all()
            item.widget().hide()
            item.widget().setParent(None)
