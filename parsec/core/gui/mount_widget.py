# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Optional

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget

from parsec._parsec import DateTime
from parsec.core.fs import FsPath

from parsec.core.gui.files_widget import FilesWidget, Clipboard
from parsec.core.gui.workspaces_widget import WorkspacesWidget
from parsec.core.gui.ui.mount_widget import Ui_MountWidget


class MountWidget(QWidget, Ui_MountWidget):
    widget_switched = pyqtSignal(list)
    # First argument is an EntryName but since it can be None,
    # we can't set the type.
    folder_changed = pyqtSignal(object, str)

    def __init__(self, core, jobs_ctx, event_bus, **kwargs):
        super().__init__(**kwargs)
        self.setupUi(self)

        self.core = core
        self.jobs_ctx = jobs_ctx
        self.event_bus = event_bus
        self.global_clipboard = None
        self.workspaces_widget = WorkspacesWidget(
            self.core, self.jobs_ctx, self.event_bus, parent=self
        )
        self.layout_content.insertWidget(0, self.workspaces_widget)
        self.workspaces_widget.load_workspace_clicked.connect(self.load_workspace)
        self.files_widget = FilesWidget(self.core, self.jobs_ctx, self.event_bus, parent=self)
        self.files_widget.folder_changed.connect(self.folder_changed.emit)
        self.files_widget.global_clipboard_updated.connect(self.clipboard_updated)
        self.layout_content.insertWidget(0, self.files_widget)
        self.files_widget.back_clicked.connect(self.show_workspaces_widget)
        self.show_workspaces_widget()

    def load_workspace(self, workspace_fs, default_path, select=False):
        self.show_files_widget(workspace_fs, default_path, select)

    def load_path(self, path):
        if not self.files_widget.isVisible():
            return
        self.files_widget.load(FsPath(str(path)))

    def show_files_widget(
        self,
        workspace_fs,
        default_path,
        selected=False,
        mount_it=False,
        timestamp: Optional[DateTime] = None,
    ):
        self.workspaces_widget.hide()

        if mount_it and not self.workspaces_widget.is_workspace_mounted(
            workspace_fs.workspace_id, timestamp
        ):
            self.workspaces_widget.mount_workspace(workspace_fs.workspace_id, timestamp)

        self.files_widget.set_workspace_fs(
            workspace_fs,
            current_directory=default_path.parent,
            default_selection=default_path.name
            if len(default_path.parts) != 0 and selected
            else None,
            clipboard=self.global_clipboard,
        )
        self.files_widget.show()

    def show_workspaces_widget(self):
        self.folder_changed.emit(None, None)
        self.files_widget.hide()
        self.workspaces_widget.show()
        self.workspaces_widget.reset()

    def clipboard_updated(self, clipboard: Optional[Clipboard] = None):
        self.global_clipboard = clipboard
