# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import sys

from typing import Optional
from PyQt5.QtCore import pyqtSignal, Qt
from PyQt5.QtWidgets import QWidget, QGraphicsDropShadowEffect, QMenu
from PyQt5.QtGui import QColor, QCursor

from parsec.core.fs import WorkspaceFS
from parsec.core.types import EntryID, WorkspaceRole
from parsec.core.fs.workspacefs import ReencryptionNeed

from parsec.core.gui.lang import translate as _, format_datetime
from parsec.core.gui.workspace_roles import NOT_SHARED_KEY, get_role_translation
from parsec.core.gui.custom_dialogs import show_info
from parsec.core.gui.custom_widgets import ensure_string_size

from parsec.core.gui.ui.workspace_button import Ui_WorkspaceButton
from parsec.core.gui.ui.empty_workspace_widget import Ui_EmptyWorkspaceWidget
from parsec.core.gui.ui.temporary_workspace_widget import Ui_TemporaryWorkspaceWidget

from parsec.core.gui.switch_button import SwitchButton


# Only used because we can't hide widgets in QtDesigner and adding the empty workspace
# button changes the minimum size we can set for the workspace button.
class EmptyWorkspaceWidget(QWidget, Ui_EmptyWorkspaceWidget):
    def __init__(self):
        super().__init__()
        self.setupUi(self)
        self.label_icon.apply_style()


class TemporaryWorkspaceWidget(QWidget, Ui_TemporaryWorkspaceWidget):
    def __init__(self):
        super().__init__()
        self.setupUi(self)


class WorkspaceButton(QWidget, Ui_WorkspaceButton):
    clicked = pyqtSignal(WorkspaceFS)
    share_clicked = pyqtSignal(WorkspaceFS)
    reencrypt_clicked = pyqtSignal(EntryID, bool, bool, bool)
    delete_clicked = pyqtSignal(WorkspaceFS)
    rename_clicked = pyqtSignal(QWidget)
    remount_ts_clicked = pyqtSignal(WorkspaceFS)
    open_clicked = pyqtSignal(WorkspaceFS)
    switch_clicked = pyqtSignal(bool, WorkspaceFS, object)

    def __init__(self, workspace_fs, parent=None):
        # Initialize UI
        super().__init__(parent=parent)
        self.setupUi(self)

        # Read-only attributes
        self.workspace_fs = workspace_fs

        # Property inner state
        self._reencrypting = None
        self._reencryption_needs = None

        # Static initialization
        self.switch_button = SwitchButton()
        self.widget_actions.layout().insertWidget(0, self.switch_button)
        self.switch_button.clicked.connect(self._on_switch_clicked)
        self.setCursor(QCursor(Qt.PointingHandCursor))
        self.setContextMenuPolicy(Qt.CustomContextMenu)
        self.customContextMenuRequested.connect(self.show_context_menu)

        if not self.timestamped:
            self.button_delete.hide()
            self.button_reencrypt.hide()
            self.widget_empty.layout().addWidget(EmptyWorkspaceWidget())
        else:
            self.switch_button.setChecked(True)
            self.button_reencrypt.hide()
            self.button_remount_ts.hide()
            self.button_share.hide()
            self.button_rename.hide()
            self.label_shared.hide()
            self.label_owner.hide()
            self.switch_button.hide()
            widget_tmp = TemporaryWorkspaceWidget()
            self.widget_empty.layout().addWidget(widget_tmp)

        effect = QGraphicsDropShadowEffect(self)
        effect.setColor(QColor(0x99, 0x99, 0x99))
        effect.setBlurRadius(10)
        effect.setXOffset(2)
        effect.setYOffset(2)
        self.setGraphicsEffect(effect)
        self.widget_reencryption.hide()
        self.button_share.clicked.connect(self.button_share_clicked)
        self.button_share.apply_style()
        self.button_reencrypt.clicked.connect(self.button_reencrypt_clicked)
        self.button_reencrypt.apply_style()
        self.button_delete.clicked.connect(self.button_delete_clicked)
        self.button_delete.apply_style()
        self.button_rename.clicked.connect(self.button_rename_clicked)
        self.button_rename.apply_style()
        self.button_remount_ts.clicked.connect(self.button_remount_ts_clicked)
        self.button_remount_ts.apply_style()
        self.button_open.clicked.connect(self.button_open_workspace_clicked)
        self.button_open.apply_style()
        self.label_owner.apply_style()
        self.label_shared.apply_style()

    def apply_state(
        self, workspace_name, workspace_fs, users_roles, is_mounted, files=None, timestamped=False
    ):
        # Not meant to change
        assert timestamped == self.timestamped
        assert workspace_fs == self.workspace_fs

        # Update attributes
        self.workspace_name = workspace_name
        self.workspace_fs = workspace_fs
        self.users_roles = users_roles
        self.files = files or []

        # Update dependent widgets
        if not self.timestamped:
            if not len(self.files):
                self.widget_empty.show()
                self.widget_files.hide()
            else:
                self.widget_files.show()
                self.widget_empty.hide()
                for i in range(1, 5):
                    label = getattr(self, "file{}_name".format(i))
                    if len(files) >= i:
                        label.setText(files[i - 1].str)
                    else:
                        label.setText("")
        else:
            widget_temp = self.widget_empty.layout().itemAt(0).widget()
            widget_temp.label_timestamp.setText(format_datetime(self.timestamp))

        # Retrieve current role for ourself
        user_id = self.workspace_fs.device.user_id
        try:
            current_role, _ = self.users_roles[user_id]
        except KeyError:
            current_role = NOT_SHARED_KEY

        self.label_role.setText(get_role_translation(current_role))
        self.label_owner.setVisible(self.is_owner)
        self.label_shared.setVisible(self.is_shared)
        self.reload_workspace_name(self.workspace_name)
        self.set_mountpoint_state(is_mounted)

    @classmethod
    def create(
        cls, workspace_name, workspace_fs, users_roles, is_mounted, files=None, timestamped=False
    ):
        instance = cls(workspace_fs)
        instance.apply_state(
            workspace_name=workspace_name,
            workspace_fs=workspace_fs,
            users_roles=users_roles,
            is_mounted=is_mounted,
            files=files,
            timestamped=timestamped,
        )
        return instance

    @property
    def is_shared(self):
        return len(self.users_roles) > 1

    @property
    def owner(self):
        for user_id, (role, user_info) in self.users_roles.items():
            if role == WorkspaceRole.OWNER and user_info:
                return user_info
        raise ValueError

    @property
    def others(self):
        return [
            user_info
            for user_id, (role, user_info) in self.users_roles.items()
            if user_id != self.workspace_fs.device.user_id
        ]

    @property
    def is_owner(self):
        user_id = self.workspace_fs.device.user_id
        return user_id in self.users_roles and self.users_roles[user_id][0] == WorkspaceRole.OWNER

    def show_context_menu(self, pos):
        global_pos = self.mapToGlobal(pos)
        menu = QMenu(self)

        if sys.platform == "darwin":
            action = menu.addAction(_("ACTION_WORKSPACE_OPEN_IN_FINDER"))
        else:
            action = menu.addAction(_("ACTION_WORKSPACE_OPEN_IN_FILE_EXPLORER"))
        action.triggered.connect(self.button_open_workspace_clicked)
        if not self.timestamped:
            action = menu.addAction(_("ACTION_WORKSPACE_RENAME"))
            action.triggered.connect(self.button_rename_clicked)
            action = menu.addAction(_("ACTION_WORKSPACE_SHARE"))
            action.triggered.connect(self.button_share_clicked)
            action = menu.addAction(_("ACTION_WORKSPACE_SEE_IN_THE_PAST"))
            action.triggered.connect(self.button_remount_ts_clicked)
            if self.reencryption_needs and self.reencryption_needs.need_reencryption:
                action = menu.addAction(_("ACTION_WORKSPACE_REENCRYPT"))
                action.triggered.connect(self.button_reencrypt_clicked)
        else:
            action = menu.addAction(_("ACTION_WORKSPACE_DELETE"))
            action.triggered.connect(self.button_delete_clicked)

        menu.exec_(global_pos)

    def button_open_workspace_clicked(self):
        self.open_clicked.emit(self.workspace_fs)

    def button_share_clicked(self):
        self.share_clicked.emit(self.workspace_fs)

    def button_reencrypt_clicked(self):
        if self.reencryption_needs:
            if not self.is_owner:
                show_info(self, message=_("TEXT_WORKSPACE_ONLY_OWNER_CAN_REENCRYPT"))
                return
            self.reencrypt_clicked.emit(
                self.workspace_id,
                bool(self.reencryption_needs.user_revoked),
                bool(self.reencryption_needs.role_revoked),
                bool(self.reencryption_needs.reencryption_already_in_progress),
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
    def workspace_id(self):
        return self.workspace_fs.workspace_id

    @property
    def timestamp(self):
        return getattr(self.workspace_fs, "timestamp", None)

    @property
    def timestamped(self):
        return self.timestamp is not None

    @property
    def reencryption_needs(self) -> Optional[ReencryptionNeed]:
        return self._reencryption_needs

    @reencryption_needs.setter
    def reencryption_needs(self, val: Optional[ReencryptionNeed]):
        self._reencryption_needs = val
        if val and val.need_reencryption and self.is_owner:
            self.button_reencrypt.show()
        else:
            self.button_reencrypt.hide()

    @property
    def reencrypting(self):
        return self._reencrypting

    @reencrypting.setter
    def reencrypting(self, val):
        def _start_reencrypting():
            self.widget_reencryption.show()
            self.widget_actions.hide()
            self.button_reencrypt.hide()
            self.setContextMenuPolicy(Qt.NoContextMenu)

        def _stop_reencrypting():
            self.button_reencrypt.hide()
            self.widget_actions.show()
            self.widget_reencryption.hide()
            self.button_reencrypt.setVisible(bool(self.reencryption_needs))
            self.setContextMenuPolicy(Qt.CustomContextMenu)

        self._reencrypting = val
        if not self.is_owner:
            return
        if self._reencrypting:
            _start_reencrypting()
            total, done = self._reencrypting
            self.progress_reencryption.setValue(int(done / total * 100))
        else:
            _stop_reencrypting()

    def reload_workspace_name(self, workspace_name):
        self.workspace_name = workspace_name
        display = workspace_name.str
        extra_space = 40

        if not self.timestamped:
            if not self.is_shared:
                shared_message = _("TEXT_WORKSPACE_IS_PRIVATE")
            elif not self.is_owner:
                shared_message = _("TEXT_WORKSPACE_IS_OWNED_BY_user").format(
                    user=self.owner.short_user_display
                )
            elif len(self.others) == 1 and self.others[0]:
                (user,) = self.others
                shared_message = _("TEXT_WORKSPACE_IS_SHARED_WITH_user").format(
                    user=user.short_user_display
                )
            else:
                n = len(self.others)
                assert n > 1
                shared_message = _("TEXT_WORKSPACE_IS_SHARED_WITH_n_USERS").format(n=n)
            display += " ({})".format(shared_message)
            if self.is_shared:
                extra_space += 40
            if self.is_owner:
                extra_space += 40
        else:
            display += "-" + _("TEXT_WORKSPACE_IS_TIMESTAMPED_date").format(
                date=format_datetime(self.workspace_fs.timestamp)
            )
            if self.is_shared:
                extra_space += 40
            if self.is_owner:
                extra_space += 40
        self.label_title.setToolTip(display)
        size = self.size().width() - extra_space
        self.label_title.setText(ensure_string_size(display, size, self.label_title.font()))

    def mousePressEvent(self, event):
        if event.button() & Qt.LeftButton and self.switch_button.isChecked():
            self.clicked.emit(self.workspace_fs)

    def _on_switch_clicked(self, state):
        self.switch_clicked.emit(state, self.workspace_fs, self.timestamp)

    def set_mountpoint_state(self, state):
        if self.timestamped:
            return
        self.switch_button.setChecked(state)
        if state:
            self.widget.setStyleSheet("background-color: #FFFFFF;")
            self.widget_title.setStyleSheet("background-color: #FFFFFF;")
            self.widget_actions.setStyleSheet("background-color: #FFFFFF;")
            self.button_open.setDisabled(False)
        else:
            self.widget.setStyleSheet("background-color: #DDDDDD;")
            self.widget_title.setStyleSheet("background-color: #DDDDDD;")
            self.widget_actions.setStyleSheet("background-color: #DDDDDD;")
            self.button_open.setDisabled(True)
