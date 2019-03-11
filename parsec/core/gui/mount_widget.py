# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import pyqtSignal

from parsec.core.gui.files_widget import FilesWidget
from parsec.core.gui.workspaces_widget import WorkspacesWidget
from parsec.core.gui.core_widget import CoreWidget

from parsec.core.gui.ui.mount_widget import Ui_MountWidget


class MountWidget(CoreWidget, Ui_MountWidget):
    reset_taskbar = pyqtSignal()

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.files_widget = FilesWidget(parent=self)
        self.workspaces_widget = WorkspacesWidget(parent=self)
        self.layout_content.addWidget(self.files_widget)
        self.layout_content.addWidget(self.workspaces_widget)
        self.files_widget.hide()
        self.workspaces_widget.load_workspace_clicked.connect(self.load_workspace)
        self.files_widget.back_clicked.connect(self.reset)

    def set_core_attributes(self, core, portal):
        super().set_core_attributes(core, portal)
        self.workspaces_widget.set_core_attributes(core, portal)
        self.files_widget.set_core_attributes(core, portal)
        self.workspaces_widget.reset()

    def stop(self):
        self.files_widget.stop()

    def set_mountpoint(self, mountpoint):
        self.files_widget.mountpoint = mountpoint

    def load_workspace(self, workspace_name):
        self.workspaces_widget.hide()
        self.files_widget.set_workspace(workspace_name)
        self.files_widget.show()
        self.reset_taskbar.emit()

    def get_taskbar_buttons(self):
        if self.files_widget.isHidden():
            return self.workspaces_widget.get_taskbar_buttons()
        return self.files_widget.get_taskbar_buttons()

    def reset(self):
        self.files_widget.reset()
        self.workspaces_widget.reset()
        self.files_widget.hide()
        self.workspaces_widget.show()
        self.reset_taskbar.emit()
