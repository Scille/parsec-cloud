# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import pyqtSignal, Qt
from PyQt5.QtWidgets import QWidget, QGraphicsDropShadowEffect
from PyQt5.QtGui import QColor, QCursor

from parsec.core.fs import WorkspaceFS, WorkspaceFSTimestamped
from parsec.core.types import EntryID

from parsec.core.gui.lang import translate as _, format_datetime
from parsec.core.gui.color import StringToColor

from parsec.core.gui.ui.workspace_button import Ui_WorkspaceButton


class WorkspaceButton(QWidget, Ui_WorkspaceButton):
    clicked = pyqtSignal(WorkspaceFS)
    share_clicked = pyqtSignal(WorkspaceFS)
    reencrypt_clicked = pyqtSignal(EntryID, bool, bool)
    delete_clicked = pyqtSignal(WorkspaceFS)
    rename_clicked = pyqtSignal(QWidget)
    remount_ts_clicked = pyqtSignal(WorkspaceFS)

    def __init__(
        self,
        workspace_name,
        workspace_fs,
        is_shared,
        is_creator,
        files=None,
        enable_workspace_color=False,
        reencryption_needs=None,
        timestamped=False,
        *args,
        **kwargs,
    ):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.workspace_name = workspace_name
        self.workspace_fs = workspace_fs
        self.label_empty.show()
        self.widget_files.hide()
        self.reencryption_needs = reencryption_needs
        self.timestamped = timestamped
        self.is_shared = is_shared
        self.is_creator = is_creator
        self.reencrypting = None
        self.setCursor(QCursor(Qt.PointingHandCursor))
        files = files or []

        if not len(files):
            self.label_empty.show()
            self.widget_files.hide()
        else:
            for i, f in enumerate(files, 1):
                if i > 4:
                    break
                label = getattr(self, "line_edit_file{}".format(i))
                label.clicked.connect(self._on_file_clicked)
                label.setText(f)
                label.setCursorPosition(0)
            self.widget_files.show()
            self.label_empty.hide()

        if enable_workspace_color:
            c = StringToColor.from_string(str(self.workspace_fs.workspace_id))
            self.setStyleSheet("background-color: {};".format(c.hex()))

        if self.timestamped:
            self.button_reencrypt.hide()
            self.button_remount_ts.hide()
            self.button_share.hide()
            self.button_rename.hide()
            self.label_shared.hide()
            self.label_owner.hide()
            for i in range(5, 9):
                getattr(self, f"line_{i}").hide()
        else:
            self.button_delete.hide()
            self.line_5.hide()

        effect = QGraphicsDropShadowEffect(self)
        effect.setColor(QColor(164, 164, 164))
        effect.setBlurRadius(15)
        effect.setXOffset(4)
        effect.setYOffset(4)
        self.setGraphicsEffect(effect)
        self.button_share.clicked.connect(self.button_share_clicked)
        self.button_reencrypt.clicked.connect(self.button_reencrypt_clicked)
        self.button_delete.clicked.connect(self.button_delete_clicked)
        self.button_rename.clicked.connect(self.button_rename_clicked)
        self.button_remount_ts.clicked.connect(self.button_remount_ts_clicked)
        if not self.is_creator:
            self.label_owner.hide()
        if not self.is_shared:
            self.label_shared.hide()
        self.reload_workspace_name(self.workspace_name)

    def _on_file_clicked(self, _):
        self.clicked.emit(self.workspace_fs)

    def button_share_clicked(self):
        self.share_clicked.emit(self.workspace_fs)

    def button_reencrypt_clicked(self):
        if self.reencryption_needs:
            self.reencrypt_clicked.emit(
                self.workspace_fs.workspace_id,
                bool(self.reencryption_needs.user_revoked),
                bool(self.reencryption_needs.role_revoked),
            )

    def button_delete_clicked(self):
        self.delete_clicked.emit(self.workspace_fs)

    def button_rename_clicked(self):
        self.rename_clicked.emit(self)

    def button_remount_ts_clicked(self):
        self.remount_ts_clicked.emit(self.workspace_fs)

    @property
    def name(self):
        return self.workspace_name

    @property
    def reencryption_needs(self):
        return self._reencryption_needs

    @reencryption_needs.setter
    def reencryption_needs(self, val):
        self._reencryption_needs = val
        if self.reencryption_needs and self.reencryption_needs.need_reencryption:
            self.button_reencrypt.show()
            self.line_6.show()
            self.button_reencrypt.setToolTip(_("TOOLTIP_WORKSPACE_NEEDS_REENCRYPTION"))
        else:
            self.button_reencrypt.hide()
            self.line_6.hide()

    def reload_workspace_name(self, workspace_name):
        self.workspace_name = workspace_name
        display = workspace_name
        if len(display) > 20:
            display = display[:20] + "..."

        if self.is_shared:
            if self.is_creator:
                display += " ({})".format(_("WORKSPACE_SHARED_DISPLAY"))
            # TODO: uncomment once the workspace name does not contain "shared by XX" anymore
            # else:
            #     display += _(" (shared with you)")

        if isinstance(self.workspace_fs, WorkspaceFSTimestamped):
            display += _("WORKSPACE_NAME_TIMESTAMPED_{}").format(
                format_datetime(self.workspace_fs.timestamp)
            )

        if self.reencrypting:
            total, done = self.reencrypting
            display += _("WORKSPACE_NAME_REENCRYPTION_{}").format(int(done / total * 100))

        self.label_workspace.setText(display)
        self.label_workspace.setToolTip(workspace_name)

    def mousePressEvent(self, event):
        if event.button() & Qt.LeftButton:
            self.clicked.emit(self.workspace_fs)
