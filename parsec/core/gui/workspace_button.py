# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import pyqtSignal, Qt
from PyQt5.QtWidgets import QWidget, QGraphicsDropShadowEffect, QMenu
from PyQt5.QtGui import QColor, QCursor

from parsec.core.fs import WorkspaceFS, WorkspaceFSTimestamped
from parsec.core.types import EntryID, WorkspaceRole

from parsec.core.gui.lang import translate as _, format_datetime
from parsec.core.gui.custom_dialogs import show_info

from parsec.core.gui.ui.workspace_button import Ui_WorkspaceButton
from parsec.core.gui.ui.empty_workspace_widget import Ui_EmptyWorkspaceWidget

from parsec.core.gui.switch_button import SwitchButton


# Only used because we can't hide widgets in QtDesigner and adding the empty workspace
# button changes the minimum size we can set for the workspace button.
class EmptyWorkspaceWidget(QWidget, Ui_EmptyWorkspaceWidget):
    def __init__(self):
        super().__init__()
        self.setupUi(self)
        self.label_icon.apply_style()


class WorkspaceButton(QWidget, Ui_WorkspaceButton):
    clicked = pyqtSignal(WorkspaceFS)
    share_clicked = pyqtSignal(WorkspaceFS)
    reencrypt_clicked = pyqtSignal(EntryID, bool, bool)
    delete_clicked = pyqtSignal(WorkspaceFS)
    rename_clicked = pyqtSignal(QWidget)
    remount_ts_clicked = pyqtSignal(WorkspaceFS)
    open_clicked = pyqtSignal(WorkspaceFS)

    def __init__(
        self,
        workspace_name,
        workspace_fs,
        users_roles,
        files=None,
        reencryption_needs=None,
        timestamped=False,
        parent=None,
    ):
        self._parent = parent
        # Only useful for testing,
        # Qt doesn't like getting a mock object as a parent
        if not isinstance(parent, QWidget):
            parent = None
        super().__init__(parent=parent)

        self.setupUi(self)
        self.users_roles = users_roles
        self.workspace_name = workspace_name
        self.workspace_fs = workspace_fs
        self.reencryption_needs = reencryption_needs
        self.timestamped = timestamped
        self.reencrypting = None
        self.setCursor(QCursor(Qt.PointingHandCursor))
        self.widget_empty.layout().addWidget(EmptyWorkspaceWidget())
        self.setContextMenuPolicy(Qt.CustomContextMenu)
        self.customContextMenuRequested.connect(self.show_context_menu)
        files = files or []

        # Add switch button
        # TODO: integrated properly in the .ui files
        self.switch_button = SwitchButton(parent=self)
        bottom_layout = self.button_open.parent().layout()
        bottom_layout.insertWidget(0, self.switch_button)
        bottom_layout.insertSpacing(0, 20)
        self.switch_button.clicked.connect(self.on_switch_clicked)
        self.switch_button.toggled.connect(self.on_switch_toggled)

        # Connect to workspaces_widget signals
        self.workspaces_widget.mountpoint_started.connect(self.on_mountpoint_started)
        self.workspaces_widget.mountpoint_stopped.connect(self.on_mountpoint_stopped)

        if not len(files):
            self.widget_empty.show()
            self.widget_files.hide()
        else:
            for i, f in enumerate(files, 1):
                if i > 4:
                    break
                label = getattr(self, "file{}_name".format(i))
                label.setText(f)
            self.widget_files.show()
            self.widget_empty.hide()

        if self.timestamped:
            self.widget_title.setStyleSheet(
                "background-color: #E3E3E3; border-top-left-radius: 8px; border-top-right-radius: 8px;"
            )
            self.widget_actions.setStyleSheet(
                "background-color: #E3E3E3; border-bottom-left-radius: 8px; border-bottom-right-radius: 8px;"
            )
            self.setStyleSheet("background-color: #E3E3E3; border-radius: 8px;")
            self.button_reencrypt.hide()
            self.button_remount_ts.hide()
            self.button_share.hide()
            self.button_rename.hide()
            self.label_shared.hide()
            self.label_owner.hide()
        else:
            self.widget_title.setStyleSheet(
                "background-color: #FFFFFF; border-top-left-radius: 8px; border-top-right-radius: 8px;"
            )
            self.widget_actions.setStyleSheet(
                "background-color: #FFFFFF; border-bottom-left-radius: 8px; border-bottom-right-radius: 8px;"
            )
            self.setStyleSheet("background-color: #FFFFFF; border-radius: 8px;")
            self.button_delete.hide()

        effect = QGraphicsDropShadowEffect(self)
        effect.setColor(QColor(0x99, 0x99, 0x99))
        effect.setBlurRadius(10)
        effect.setXOffset(2)
        effect.setYOffset(2)
        self.setGraphicsEffect(effect)
        if not self.is_owner:
            self.button_reencrypt.hide()
        self.label_reencrypting.hide()
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
        self.label_reencrypting.apply_style()
        if not self.is_owner:
            self.label_owner.hide()
        if not self.is_shared:
            self.label_shared.hide()
        self.reload_workspace_name(self.workspace_name)
        self.reload_workspace_mounted_status()

    @property
    def is_shared(self):
        return len(self.users_roles) > 1

    @property
    def owner(self):
        for user_id, role in self.users_roles.items():
            if role == WorkspaceRole.OWNER:
                return user_id
        raise ValueError

    @property
    def others(self):
        return [
            user_id for user_id in self.users_roles if user_id != self.workspace_fs.device.user_id
        ]

    @property
    def is_owner(self):
        user_id = self.workspace_fs.device.user_id
        return self.users_roles[user_id] == WorkspaceRole.OWNER

    def show_context_menu(self, pos):
        global_pos = self.mapToGlobal(pos)
        menu = QMenu(self)

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
                show_info(self.parent(), message=_("TEXT_WORKSPACE_ONLY_OWNER_CAN_REENCRYPT"))
                return
            self.reencrypt_clicked.emit(
                self.workspace_id,
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
    def workspaces_widget(self):
        return self._parent

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
    def reencryption_needs(self):
        return self._reencryption_needs

    @reencryption_needs.setter
    def reencryption_needs(self, val):
        self._reencryption_needs = val
        if not self.is_owner:
            return
        if self.reencryption_needs and self.reencryption_needs.need_reencryption:
            self.button_reencrypt.show()
        else:
            self.button_reencrypt.hide()

    @property
    def reencrypting(self):
        return self._reencrypting

    @reencrypting.setter
    def reencrypting(self, val):
        def _start_reencrypting():
            self.button_reencrypt.hide()
            self.label_reencrypting.show()

        def _stop_reencrypting():
            self.button_reencrypt.hide()
            self.label_reencrypting.hide()

        self._reencrypting = val
        if not self.is_owner:
            return
        if self._reencrypting:
            _start_reencrypting()
            total, done = self._reencrypting
            self.label_reencrypting.setToolTip(
                "{} {}%".format(
                    _("TEXT_WORKSPACE_CURRENTLY_REENCRYPTING_TOOLTIP"), int(done / total * 100)
                )
            )
        else:
            _stop_reencrypting()

    def reload_workspace_name(self, workspace_name):
        self.workspace_name = workspace_name
        display = workspace_name

        if not self.is_shared:
            shared_message = _("TEXT_WORKSPACE_IS_PRIVATE")
        elif not self.is_owner:
            shared_message = _("TEXT_WORKSPACE_IS_OWNED_BY_user").format(user=self.owner)
        elif len(self.others) == 1:
            user, = self.others
            shared_message = _("TEXT_WORKSPACE_IS_SHARED_WITH_user").format(user=user)
        else:
            n = len(self.others)
            assert n > 1
            shared_message = _("TEXT_WORKSPACE_IS_SHARED_WITH_n_USERS").format(n=n)
        display += " ({})".format(shared_message)

        if isinstance(self.workspace_fs, WorkspaceFSTimestamped):
            display += "-" + _("TEXT_WORKSPACE_IS_TIMESTAMPED_date").format(
                date=format_datetime(self.workspace_fs.timestamp)
            )
        self.label_title.setToolTip(display)
        if len(display) > 20:
            display = display[:20] + "..."
        self.label_title.setText(display)

    def mousePressEvent(self, event):
        if event.button() & Qt.LeftButton and self.switch_button.isChecked():
            self.clicked.emit(self.workspace_fs)

    def on_switch_clicked(self, state):
        if state:
            self.workspaces_widget.mount_workspace(self.workspace_id, self.timestamp)
        else:
            self.workspaces_widget.unmount_workspace(self.workspace_id, self.timestamp)
        if not self.timestamp:
            self.workspaces_widget.update_workspace_config(self.workspace_id, state)

    def on_switch_toggled(self, state):
        self.button_open.setEnabled(state)
        if not self.timestamped:
            if state:
                self.setStyleSheet("background-color: #FFFFFF; border-radius: 8px;")
            else:
                self.setStyleSheet("background-color: #E3E3E3; border-radius: 8px;")

    def reload_workspace_mounted_status(self):
        is_mounted = self.workspaces_widget.is_workspace_mounted(
            self.workspace_fs.workspace_id, self.timestamp
        )
        self.switch_button.setChecked(is_mounted)
        self.on_switch_toggled(is_mounted)

    def on_mountpoint_started(self, workspace_id, timestamp):
        if workspace_id != self.workspace_id or timestamp != self.timestamp:
            return
        self.switch_button.setChecked(True)

    def on_mountpoint_stopped(self, workspace_id, timestamp):
        if workspace_id != self.workspace_id or timestamp != self.timestamp:
            return
        self.switch_button.setChecked(False)
