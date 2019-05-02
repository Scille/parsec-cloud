# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import pyqtSignal, Qt
from PyQt5.QtWidgets import QWidget, QGraphicsDropShadowEffect
from PyQt5.QtGui import QColor

from parsec.core.fs import WorkspaceFS

from parsec.core.gui.lang import translate as _
from parsec.core.gui.color import StringToColor

from parsec.core.gui.ui.workspace_button import Ui_WorkspaceButton


class WorkspaceButton(QWidget, Ui_WorkspaceButton):
    clicked = pyqtSignal(WorkspaceFS)
    share_clicked = pyqtSignal(QWidget)
    delete_clicked = pyqtSignal(QWidget)
    rename_clicked = pyqtSignal(QWidget)
    file_clicked = pyqtSignal(WorkspaceFS, str)

    def __init__(
        self,
        workspace_fs,
        participants,
        is_creator,
        files=None,
        enable_workspace_color=False,
        *args,
        **kwargs
    ):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.workspace_fs = workspace_fs
        self.label_empty.show()
        self.widget_files.hide()
        self.participants = participants
        self.is_creator = is_creator
        files = files or []

        if not len(files):
            self.label_empty.show()
            self.widget_files.hide()
        else:
            for i, f in enumerate(files, 1):
                if i > 4:
                    break
                label = getattr(self, "line_edit_file{}".format(i))
                label.clicked.connect(self.open_clicked_file)
                label.setText(f)
                label.setCursorPosition(0)
            self.widget_files.show()
            self.label_empty.hide()

        if enable_workspace_color:
            c = StringToColor.from_string(str(self.workspace_fs.workspace_id))
            self.setStyleSheet("background-color: {};".format(c.hex()))

        effect = QGraphicsDropShadowEffect(self)
        effect.setColor(QColor(164, 164, 164))
        effect.setBlurRadius(15)
        effect.setXOffset(4)
        effect.setYOffset(4)
        self.setGraphicsEffect(effect)
        self.button_share.clicked.connect(self.button_share_clicked)
        self.button_delete.clicked.connect(self.button_delete_clicked)
        self.button_rename.clicked.connect(self.button_rename_clicked)
        self.button_open_workspace.clicked.connect(self.button_open_workspace_clicked)
        if not self.is_creator:
            self.label_owner.hide()
        if len(participants) == 1:
            self.label_shared.hide()
        self.reload_workspace_name()

    def button_open_workspace_clicked(self):
        self.open_clicked_file("/")

    def open_clicked_file(self, file_name):
        self.file_clicked.emit(self.workspace_fs, file_name)

    def button_share_clicked(self):
        self.share_clicked.emit(self)

    def button_delete_clicked(self):
        self.delete_clicked.emit(self)

    def button_rename_clicked(self):
        self.rename_clicked.emit(self)

    @property
    def name(self):
        return self.workspace_fs.get_workspace_entry().name

    def reload_workspace_name(self):
        workspace_name = self.workspace_fs.get_workspace_entry().name
        display = workspace_name
        if len(display) > 20:
            display = display[:20] + "..."

        if len(self.participants) > 1:
            if self.is_creator:
                display += _(" (shared)")
            else:
                display += _(" (shared by {})").format(self.workspace_fs.device.user_id)
        self.label_workspace.setText(display)
        self.label_workspace.setToolTip(workspace_name)

    def mousePressEvent(self, event):
        if event.button() & Qt.LeftButton:
            self.clicked.emit(self.workspace_fs)
