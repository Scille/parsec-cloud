# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import sys
from enum import Enum
from typing import Tuple

from PyQt5.QtCore import QPoint, Qt, pyqtSignal
from PyQt5.QtGui import QColor, QCursor, QMouseEvent
from PyQt5.QtWidgets import QGraphicsDropShadowEffect, QMenu, QWidget

from parsec._parsec import DateTime
from parsec.api.protocol import RealmRole
from parsec.core.backend_connection import BackendNotAvailable
from parsec.core.fs import FSBackendOfflineError, WorkspaceFS, WorkspaceFSTimestamped
from parsec.core.fs.workspacefs import ReencryptionNeed
from parsec.core.gui.custom_dialogs import show_info
from parsec.core.gui.custom_widgets import ensure_string_size
from parsec.core.gui.lang import format_datetime
from parsec.core.gui.lang import translate as _
from parsec.core.gui.switch_button import SwitchButton
from parsec.core.gui.trio_jobs import QtToTrioJobScheduler
from parsec.core.gui.ui.temporary_workspace_widget import Ui_TemporaryWorkspaceWidget
from parsec.core.gui.ui.workspace_button import Ui_WorkspaceButton
from parsec.core.gui.workspace_roles import get_role_translation
from parsec.core.logged_core import LoggedCore, UserID
from parsec.core.types import EntryID, EntryName, UserInfo, WorkspaceRole


class TemporaryWorkspaceWidget(QWidget, Ui_TemporaryWorkspaceWidget):
    def __init__(self) -> None:
        super().__init__()
        self.setupUi(self)


class SharingStatus(Enum):
    Unknown = 0
    Shared = 1
    NotShared = 2


class WorkspaceButton(QWidget, Ui_WorkspaceButton):
    clicked = pyqtSignal(WorkspaceFS)
    share_clicked = pyqtSignal(WorkspaceFS)
    reencrypt_clicked = pyqtSignal(EntryID, bool, bool, bool)
    delete_clicked = pyqtSignal(WorkspaceFS)
    rename_clicked = pyqtSignal(QWidget)
    remount_ts_clicked = pyqtSignal(WorkspaceFS)
    open_clicked = pyqtSignal(WorkspaceFS)
    switch_clicked = pyqtSignal(bool, WorkspaceFS, object)

    def __init__(
        self,
        core: LoggedCore,
        jobs_ctx: QtToTrioJobScheduler,
        workspace_fs: WorkspaceFS,
        parent: QWidget | None = None,
    ) -> None:
        # Initialize UI
        super().__init__(parent=parent)
        self.setupUi(self)

        # Read-only attributes
        self.workspace_fs = workspace_fs
        self.core = core
        self.jobs_ctx = jobs_ctx

        # Property inner state
        self._reencryption: Tuple[int, int] | None = None
        self._reencryption_needs: ReencryptionNeed | None = None
        self.users_roles: dict[UserID, Tuple[RealmRole, UserInfo]] | None = None
        self.workspace_name = workspace_fs.get_workspace_name()

        # Static initialization
        self.switch_button = SwitchButton()
        self.switch_button.setChecked(
            self.core.mountpoint_manager.is_workspace_mounted(self.workspace_fs.workspace_id, None)
        )
        self.widget_actions.layout().insertWidget(0, self.switch_button)
        self.switch_button.clicked.connect(self._on_switch_clicked)
        self.setCursor(QCursor(Qt.PointingHandCursor))
        self.setContextMenuPolicy(Qt.CustomContextMenu)
        self.customContextMenuRequested.connect(self.show_context_menu)

        if self.is_timestamped:
            widget_tmp = TemporaryWorkspaceWidget()
            self.widget.layout().insertWidget(1, widget_tmp)
            assert self.timestamp is not None
            widget_tmp.label_timestamp.setText(format_datetime(self.timestamp))

        effect = QGraphicsDropShadowEffect(self)
        effect.setColor(QColor(0x99, 0x99, 0x99))
        effect.setBlurRadius(10)
        effect.setXOffset(2)
        effect.setYOffset(2)
        self.setGraphicsEffect(effect)

        if not self.is_timestamped:
            self.button_delete.hide()
        else:
            self.switch_button.hide()
            self.label_shared_info.hide()

        self.button_reencrypt.hide()
        self.button_remount_ts.hide()
        self.button_share.hide()
        self.button_rename.hide()
        self.label_shared.hide()
        self.label_owner.hide()

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

        self.reload_workspace_name(self.workspace_name)

    def refresh_status(self) -> None:
        # Start the asynchronous job to get the workspace state
        if not self.is_timestamped:
            _ = self.jobs_ctx.submit_job(
                None,
                None,
                self._get_workspace_info,
            )

    async def _get_workspace_info(self) -> None:
        users_roles = {}

        try:
            roles = await self.workspace_fs.get_user_roles()
            for user_id, role in roles.items():
                user_info = await self.core.get_user_info(user_id)
                users_roles[user_id] = (role, user_info)
        except (FSBackendOfflineError, BackendNotAvailable):
            self.users_roles = None
        else:
            self.users_roles = users_roles

        current_role = self.role

        if not self.core.device.is_outsider and current_role not in (
            WorkspaceRole.READER,
            WorkspaceRole.CONTRIBUTOR,
        ):
            self.button_share.show()

        self.label_role.setText(get_role_translation(current_role))
        self.label_owner.setVisible(self.is_owner)
        self.label_shared.setVisible(self.sharing_status == SharingStatus.Shared)
        self.reload_workspace_name(self.workspace_name)

        self.button_remount_ts.show()
        self.button_rename.show()

        if self.is_owner:
            try:
                reenc_needs = await self.workspace_fs.get_reencryption_need()
                self.reencryption_needs = reenc_needs
            except FSBackendOfflineError:
                pass
        else:
            self.reencryption_needs = None

        self.set_mountpoint_state(self.is_mounted())

    def get_owner(self) -> UserInfo | None:
        if self.users_roles:
            for user_id, (role, user_info) in self.users_roles.items():
                if role == WorkspaceRole.OWNER and user_info:
                    return user_info
        elif self.role == WorkspaceRole.OWNER:
            # Fallback to craft a custom list with only our device since it's
            # the only one we know about
            return UserInfo(
                user_id=self.core.device.user_id,
                human_handle=self.core.device.human_handle,
                profile=self.core.device.profile,
                revoked_on=None,
                # Unfortunately, this field is not available from LocalDevice
                # so we have to set it with a dummy value :'(
                # However it's more a hack than an issue given this field is
                # not used here.
                created_on=DateTime.from_timestamp(0),
            )
        return None

    @property
    def sharing_status(self) -> SharingStatus:
        if self.users_roles is None:
            return SharingStatus.Unknown
        return SharingStatus.Shared if self.get_others() else SharingStatus.NotShared

    @property
    def role(self) -> RealmRole | None:
        return self.workspace_fs.get_workspace_entry().role

    def get_others(self) -> list[UserInfo]:
        if self.users_roles:
            return [
                user_info
                for user_id, (role, user_info) in self.users_roles.items()
                if user_id != self.workspace_fs.device.user_id and not user_info.is_revoked
            ]
        return []

    @property
    def is_owner(self) -> bool:
        return self.role == WorkspaceRole.OWNER

    def show_context_menu(self, pos: QPoint) -> None:
        global_pos = self.mapToGlobal(pos)
        menu = QMenu(self)

        if sys.platform == "darwin":
            action = menu.addAction(_("ACTION_WORKSPACE_OPEN_IN_FINDER"))
        else:
            action = menu.addAction(_("ACTION_WORKSPACE_OPEN_IN_FILE_EXPLORER"))
        action.triggered.connect(self.button_open_workspace_clicked)
        if not self.is_timestamped:
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

    def button_open_workspace_clicked(self) -> None:
        self.open_clicked.emit(self.workspace_fs)

    def button_share_clicked(self) -> None:
        self.share_clicked.emit(self.workspace_fs)

    def button_reencrypt_clicked(self) -> None:
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

    def button_delete_clicked(self) -> None:
        self.delete_clicked.emit(self.workspace_fs)

    def button_rename_clicked(self) -> None:
        self.rename_clicked.emit(self)

    def button_remount_ts_clicked(self) -> None:
        self.remount_ts_clicked.emit(self.workspace_fs)

    @property
    def name(self) -> EntryName | None:
        return self.workspace_name

    @property
    def workspace_id(self) -> EntryID:
        return self.workspace_fs.workspace_id

    def is_mounted(self) -> bool:
        return self.switch_button.isChecked()

    @property
    def timestamp(self) -> DateTime | None:
        return getattr(self.workspace_fs, "timestamp", None)

    @property
    def is_timestamped(self) -> bool:
        return self.timestamp is not None

    @property
    def reencryption_needs(self) -> ReencryptionNeed | None:
        return self._reencryption_needs

    @reencryption_needs.setter
    def reencryption_needs(self, val: ReencryptionNeed | None) -> None:
        self._reencryption_needs = val
        if val and val.need_reencryption and self.is_owner:
            self.button_reencrypt.show()
        else:
            self.button_reencrypt.hide()

    @property
    def reencryption(self) -> object | None:
        return self._reencryption

    @reencryption.setter
    def reencryption(self, val: Tuple[int, int]) -> None:
        def _start_reencrypting() -> None:
            self.widget_reencryption.show()
            self.widget_actions.hide()
            self.button_reencrypt.hide()
            self.setContextMenuPolicy(Qt.NoContextMenu)

        def _stop_reencrypting() -> None:
            self.button_reencrypt.hide()
            self.widget_actions.show()
            self.widget_reencryption.hide()
            self.button_reencrypt.setVisible(bool(self.reencryption_needs))
            self.setContextMenuPolicy(Qt.CustomContextMenu)

        self._reencryption = val
        if not self.is_owner:
            return
        if self._reencryption:
            _start_reencrypting()
            total, done = self._reencryption
            self.progress_reencryption.setValue(int(done / total * 100))
        else:
            _stop_reencrypting()

    def reload_workspace_name(self, workspace_name: EntryName) -> None:
        self.workspace_name = workspace_name
        display = workspace_name.str
        shared_message = ""

        others = self.get_others()

        if not self.is_timestamped:
            if self.sharing_status == SharingStatus.Unknown:
                shared_message = ""
            # Workspace is not shared
            elif self.sharing_status == SharingStatus.NotShared:
                shared_message = _("TEXT_WORKSPACE_IS_PRIVATE")
            # Workspace is shared, we are not the owner
            elif not self.is_owner:
                # We know the owner, display their name
                if self.get_owner():
                    owner = self.get_owner()
                    if owner:
                        shared_message = _("TEXT_WORKSPACE_IS_OWNED_BY_user").format(
                            user=owner.short_user_display
                        )
                # We don't know the owner's name, just display that the workspace is shared
                else:
                    shared_message = _("TEXT_WORKSPACE_IS_SHARED")
            # We are the owner, workspace is shared with only one user
            elif len(others) == 1:
                # We have their info
                if others[0]:
                    (user,) = others
                    shared_message = _("TEXT_WORKSPACE_IS_SHARED_WITH_user").format(
                        user=user.short_user_display
                    )
                # We don't, just display that the workspace is shared
                else:
                    shared_message = _("TEXT_WORKSPACE_IS_SHARED")
            # Only case left should be that we are the owner and there are more than one user, display the number
            else:
                n = len(others)
                assert n > 1
                shared_message = _("TEXT_WORKSPACE_IS_SHARED_WITH_n_USERS").format(n=n)
        else:
            assert isinstance(
                self.workspace_fs, WorkspaceFSTimestamped
            ), "WorkspaceFS is not timestamped"
            display += "-" + _("TEXT_WORKSPACE_IS_TIMESTAMPED_date").format(
                date=format_datetime(self.workspace_fs.timestamp)
            )
        # 20 for left and right paddings
        size = self.size().width() - 20
        self.label_title.setText(ensure_string_size(display, size, self.label_title.font()))
        self.label_title.setToolTip(display)
        self.label_shared_info.setText(
            ensure_string_size(shared_message, size, self.label_shared_info.font())
        )
        self.label_shared_info.setToolTip(shared_message)

    def mousePressEvent(self, event: QMouseEvent) -> None:
        if event.button() & Qt.LeftButton and self.switch_button.isChecked():
            self.clicked.emit(self.workspace_fs)

    def _on_switch_clicked(self, state: bool) -> None:
        self.switch_clicked.emit(state, self.workspace_fs, self.timestamp)

    def set_mountpoint_state(self, state: bool) -> None:
        if self.is_timestamped:
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
