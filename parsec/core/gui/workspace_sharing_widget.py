# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations
from typing import Callable, Union, cast, Optional

from PyQt5.QtCore import QCoreApplication, QObject, pyqtSignal, QEvent, QTimer
from PyQt5.QtGui import QPixmap, QColor
from PyQt5.QtWidgets import QWidget, QComboBox
from parsec.api.protocol import RealmRole

from parsec.api.protocol.types import UserProfile
from parsec.core.logged_core import LoggedCore, UserID
from parsec.core.types import EntryName, UserInfo
from parsec.core.fs import FSError, FSBackendOfflineError, UserFS, WorkspaceFS
from parsec.core.types import WorkspaceRole
from parsec.core.backend_connection import BackendNotAvailable

from parsec.core.gui.trio_jobs import JobResultError, QtToTrioJob, QtToTrioJobScheduler

from parsec.core.gui.custom_dialogs import show_error, GreyedDialog
from parsec.core.gui.custom_widgets import Pixmap
from parsec.core.gui.lang import translate as _
from parsec.core.gui.workspace_roles import get_role_translation, NOT_SHARED_KEY
from parsec.core.gui.snackbar_widget import SnackbarManager
from parsec.core.gui.ui.workspace_sharing_widget import Ui_WorkspaceSharingWidget
from parsec.core.gui.ui.sharing_widget import Ui_SharingWidget


_ROLES_TO_INDEX: dict[Union[WorkspaceRole, str], int] = {
    NOT_SHARED_KEY: 0,
    WorkspaceRole.READER: 1,
    WorkspaceRole.CONTRIBUTOR: 2,
    WorkspaceRole.MANAGER: 3,
    WorkspaceRole.OWNER: 4,
}


def _index_to_role(index: int) -> Optional[RealmRole]:
    for role, idx in _ROLES_TO_INDEX.items():
        if index == idx:
            return cast(RealmRole, role)
    return None


async def _do_get_users(
    core: LoggedCore, workspace_fs: WorkspaceFS
) -> dict[UserInfo, WorkspaceRole | str]:
    ret: dict[UserInfo, WorkspaceRole | str] = {}
    try:
        participants = await workspace_fs.get_user_roles()
        updated_participants = {}
        for user, role in participants.items():
            user_info = await core.get_user_info(user)
            updated_participants[user_info] = role
        # TODO: handle pagination
        users, _ = await core.find_humans()

        for user_info, role in updated_participants.items():
            ret[user_info] = role
        for user_info in users:
            if user_info not in ret:
                ret[user_info] = NOT_SHARED_KEY
        return ret
    except (FSBackendOfflineError, BackendNotAvailable) as exc:
        raise JobResultError("offline") from exc


async def _do_share_workspace(
    user_fs: UserFS, workspace_fs: WorkspaceFS, user_info: UserInfo, role: WorkspaceRole
) -> tuple[UserInfo, WorkspaceRole, EntryName]:
    workspace_name = workspace_fs.get_workspace_name()

    try:
        await user_fs.workspace_share(workspace_fs.workspace_id, user_info.user_id, role)
        return user_info, role, workspace_name
    except ValueError as exc:
        raise JobResultError(
            "invalid-user", user_info=user_info, role=role, workspace_name=workspace_name
        ) from exc
    except FSBackendOfflineError as exc:
        raise JobResultError(
            "offline", user_info=user_info, role=role, workspace_name=workspace_name
        ) from exc
    except FSError as exc:
        raise JobResultError(
            "fs-error", user_info=user_info, role=role, workspace_name=workspace_name
        ) from exc
    except Exception as exc:
        raise JobResultError(
            "error", user_info=user_info, role=role, workspace_name=workspace_name
        ) from exc


class SharingWidget(QWidget, Ui_SharingWidget):
    role_changed = pyqtSignal(UserInfo, object)

    def __init__(
        self,
        user_info: UserInfo,
        is_current_user: bool,
        current_user_role: WorkspaceRole,
        role: WorkspaceRole | str,
    ) -> None:
        super().__init__()
        self.setupUi(self)

        self.combo_role.installEventFilter(self)

        enabled = True
        # Current user cannot change their own permission on the workspace
        if is_current_user:
            enabled = False
        # Current user is READER of CONTRIBUTOR, they cannot change the permission
        elif (
            current_user_role == WorkspaceRole.READER
            or current_user_role == WorkspaceRole.CONTRIBUTOR
        ):
            enabled = False
        # User has permission equivalent or higher than current user
        elif current_user_role == WorkspaceRole.MANAGER and (
            role == WorkspaceRole.OWNER or role == WorkspaceRole.MANAGER
        ):
            enabled = False

        self._role: WorkspaceRole | str = role
        self.current_user_role = current_user_role
        self.is_current_user = is_current_user
        self.user_info = user_info
        if self.is_current_user:
            self.label_name.setText(f"<b>{self.user_info.short_user_display}</b>")
        else:
            self.label_name.setText(self.user_info.short_user_display)
        if self.user_info.human_handle:
            self.label_email.setText(self.user_info.human_handle.email)

        if self.user_info.is_revoked:
            self.setDisabled(True)
            font = self.label_name.font()
            font.setStrikeOut(True)
            self.label_name.setFont(font)
            self.setToolTip(_("TEXT_WORKSPACE_SHARING_USER_HAS_BEEN_REVOKED"))

        self.setEnabled(enabled)

        if not enabled:
            # Not enabled, no point in filling all the roles since they cannot be changed anyway
            self.combo_role.addItem(get_role_translation(role))
        else:
            for _role, index in _ROLES_TO_INDEX.items():
                self.combo_role.insertItem(index, get_role_translation(_role))
                # Outsider cannot be Manager or Owner, Manager cannot set Manager or Owner
                if (
                    self.user_info.profile == UserProfile.OUTSIDER
                    or current_user_role == WorkspaceRole.MANAGER
                ) and _role in (
                    WorkspaceRole.MANAGER,
                    WorkspaceRole.OWNER,
                ):
                    item = self.combo_role.model().item(index)
                    item.setEnabled(False)
                    font = item.font()
                    font.setStrikeOut(True)
                    item.setFont(font)
                    if current_user_role == WorkspaceRole.MANAGER:
                        item.setToolTip(_("NOT_ALLOWED_FOR_MANAGER_ROLE_TOOLTIP"))
                    else:
                        item.setToolTip(_("NOT_ALLOWED_FOR_OUTSIDER_PROFILE_TOOLTIP"))

            self.combo_role.setCurrentIndex(_ROLES_TO_INDEX[self._role])
            self.combo_role.currentIndexChanged.connect(self.on_role_changed)
            self.status_timer = QTimer()
            self.status_timer.setInterval(3000)
            self.status_timer.setSingleShot(True)
            self.status_timer.timeout.connect(self._refresh_status)

    @property
    def role(self) -> Optional[WorkspaceRole]:
        return cast(Optional[WorkspaceRole], self._role if self._role != NOT_SHARED_KEY else None)

    @role.setter
    def role(self, val: str | WorkspaceRole) -> None:
        self._role = val
        self.combo_role.setCurrentIndex(_ROLES_TO_INDEX[self._role or NOT_SHARED_KEY])

    def _refresh_status(self) -> None:
        self.label_status.setPixmap(QPixmap())

    def set_status_updating(self) -> None:
        p = Pixmap(":/icons/images/material/update.svg")
        p.replace_color(QColor(0, 0, 0), QColor(0x99, 0x99, 0x99))
        self.label_status.setPixmap(p)

    def set_status_updated(self) -> None:
        p = Pixmap(":/icons/images/material/done.svg")
        p.replace_color(QColor(0, 0, 0), QColor(0x8B, 0xC3, 0x4A))
        self.label_status.setPixmap(p)
        self.status_timer.start()

    def set_status_update_failed(self) -> None:
        p = Pixmap(":/icons/images/material/sync_problem.svg")
        p.replace_color(QColor(0, 0, 0), QColor(0xF1, 0x96, 0x2B))
        self.label_status.setPixmap(p)
        self.status_timer.start()

    def on_role_changed(self, index: int) -> None:
        self.role_changed.emit(self.user_info, _index_to_role(index))

    def eventFilter(self, obj_src: QObject, event: QEvent) -> bool:
        if event.type() == QEvent.Wheel and isinstance(obj_src, QComboBox):
            event.ignore()
            return True
        return super().eventFilter(obj_src, event)

    @property
    def user_id(self) -> UserID:
        return self.user_info.user_id


class WorkspaceSharingWidget(QWidget, Ui_WorkspaceSharingWidget):
    get_users_success = pyqtSignal(QtToTrioJob)
    get_users_error = pyqtSignal(QtToTrioJob)
    share_success = pyqtSignal(QtToTrioJob)
    share_error = pyqtSignal(QtToTrioJob)
    closing = pyqtSignal(bool)

    def __init__(
        self,
        user_fs: UserFS,
        workspace_fs: WorkspaceFS,
        core: LoggedCore,
        jobs_ctx: QtToTrioJobScheduler,
    ) -> None:
        super().__init__()
        self.setupUi(self)
        self.user_fs = user_fs
        self.core = core
        self.jobs_ctx = jobs_ctx
        self.workspace_fs = workspace_fs
        self.dialog: Optional[GreyedDialog] = None

        self.has_changes = False

        self.share_success.connect(self._on_share_success)
        self.share_error.connect(self._on_share_error)
        self.get_users_success.connect(self._on_get_users_success)
        self.get_users_error.connect(self._on_get_users_error)
        self.line_edit_filter.textChanged.connect(self._on_filter_changed)

        ws_entry = self.workspace_fs.get_workspace_entry()
        self.current_user_role = ws_entry.role
        self.reset()

    def _on_filter_changed(self, text: str) -> None:
        text = text.lower()
        for i in range(self.scroll_content.layout().count()):
            w = self.scroll_content.layout().itemAt(i).widget()
            if w:
                if text in w.user_info.short_user_display.lower():
                    w.setVisible(True)
                else:
                    w.setVisible(False)

    def add_participant(
        self, user_info: UserInfo, is_current_user: bool, role: WorkspaceRole | str
    ) -> None:
        assert self.current_user_role is not None
        w = SharingWidget(
            user_info=user_info,
            is_current_user=is_current_user,
            current_user_role=self.current_user_role,
            role=role,
        )
        w.role_changed.connect(self.on_role_changed)
        self.scroll_content.layout().insertWidget(self.scroll_content.layout().count() - 1, w)

    def _get_sharing_widget(self, user_id: UserID) -> Optional[SharingWidget]:
        for i in range(self.scroll_content.layout().count() - 1):
            item = self.scroll_content.layout().itemAt(i)
            if item and item.widget() and item.widget().user_id == user_id:
                return cast(SharingWidget, item.widget())
        return None

    def on_role_changed(self, user_info: UserInfo, role: WorkspaceRole) -> None:
        sharing_widget = self._get_sharing_widget(user_info.user_id)
        if sharing_widget:
            sharing_widget.set_status_updating()
        _ = self.jobs_ctx.submit_job(
            (self, "share_success"),
            (self, "share_error"),
            _do_share_workspace,
            user_fs=self.user_fs,
            workspace_fs=self.workspace_fs,
            user_info=user_info,
            role=role,
        )

    def _on_share_success(
        self, job: QtToTrioJob[tuple[UserInfo, WorkspaceRole | str, EntryName]]
    ) -> None:
        assert job.ret is not None
        user_info, new_role, workspace_name = job.ret
        old_role = None

        self.has_changes = True
        sharing_widget = self._get_sharing_widget(user_info.user_id)
        if sharing_widget:
            sharing_widget.set_status_updated()
            old_role = sharing_widget.role
            # Mypy: `SharingWidget::role` setter handle `WorkspaceRole` and `str`
            sharing_widget.role = new_role  # type: ignore[assignment]
        if old_role is None:
            SnackbarManager.inform(
                _("TEXT_WORKSPACE_SHARING_NEW_OK_workspace-user").format(
                    workspace=workspace_name, user=user_info.short_user_display
                )
            )
        elif new_role is None:
            SnackbarManager.inform(
                _("TEXT_WORKSPACE_SHARING_REMOVED_OK_workspace-user").format(
                    workspace=workspace_name, user=user_info.short_user_display
                )
            )
        else:
            SnackbarManager.inform(
                _("TEXT_WORKSPACE_SHARING_UPDATED_workspace-user").format(
                    workspace=workspace_name, user=user_info.short_user_display
                )
            )

    def _on_share_error(
        self, job: QtToTrioJob[tuple[UserInfo, WorkspaceRole | str, EntryName]]
    ) -> None:
        if job.status == "cancelled":
            return
        reset = True
        assert isinstance(job.exc, JobResultError)
        user_info: UserInfo | None = job.exc.params.get("user_info")
        role: WorkspaceRole | str | None = job.exc.params.get("role")
        workspace_name: EntryName | None = job.exc.params.get("workspace_name")

        assert user_info is not None

        sharing_widget = self._get_sharing_widget(user_info.user_id)
        if sharing_widget:
            sharing_widget.set_status_update_failed()
        if job.status == "offline":
            SnackbarManager.warn(_("TEXT_WORKSPACE_SHARING_OFFLINE"))
            reset = False
        elif role == NOT_SHARED_KEY:
            SnackbarManager.warn(
                _("TEXT_WORKSPACE_SHARING_UNSHARE_ERROR_workspace-user").format(
                    workspace=workspace_name, user=user_info.short_user_display
                )
            )
        else:
            SnackbarManager.warn(
                _("TEXT_WORKSPACE_SHARING_SHARE_ERROR_workspace-user").format(
                    workspace=workspace_name, user=user_info.short_user_display
                )
            )
        if reset:
            self.reset()

    def _on_get_users_success(self, job: QtToTrioJob[dict[UserInfo, WorkspaceRole | str]]) -> None:
        assert job.ret is not None
        users = job.ret
        while self.scroll_content.layout().count() > 1:
            item = self.scroll_content.layout().takeAt(0)
            w = item.widget()
            self.scroll_content.layout().removeItem(item)
            w.setParent(None)
        QCoreApplication.processEvents()
        for user_info, role in users.items():
            if not user_info.revoked_on:
                self.add_participant(
                    user_info,
                    is_current_user=user_info.user_id == self.core.device.user_id,
                    role=role or "NOT_SHARED",
                )
        self.spinner.hide()
        self.widget_users.show()

    def _on_get_users_error(self, job: QtToTrioJob[dict[UserInfo, WorkspaceRole | str]]) -> None:
        assert job.is_finished()
        assert job.status != "ok"

        if job.status == "offline":
            show_error(self, _("TEXT_WORKSPACE_SHARING_OFFLINE"))
        self.spinner.hide()
        self.widget_users.show()
        if self.dialog is not None:
            self.dialog.reject()

    def on_close(self) -> None:
        self.closing.emit(self.has_changes)

    def reset(self) -> None:
        self.spinner.show()
        self.widget_users.hide()
        _ = self.jobs_ctx.submit_job(
            (self, "get_users_success"),
            (self, "get_users_error"),
            _do_get_users,
            core=self.core,
            workspace_fs=self.workspace_fs,
        )

    @classmethod
    def show_modal(  # type: ignore[misc]
        cls,
        user_fs: UserFS,
        workspace_fs: WorkspaceFS,
        core: LoggedCore,
        jobs_ctx: QtToTrioJobScheduler,
        parent: QWidget,
        on_finished: Callable[..., None],
    ) -> WorkspaceSharingWidget:
        workspace_name = workspace_fs.get_workspace_name()

        w = cls(user_fs=user_fs, workspace_fs=workspace_fs, core=core, jobs_ctx=jobs_ctx)
        d = GreyedDialog(
            w,
            title=_("TEXT_WORKSPACE_SHARING_TITLE_workspace").format(workspace=workspace_name.str),
            parent=parent,
            width=1000,
        )
        w.dialog = d
        d.closing.connect(w.on_close)
        w.closing.connect(on_finished)
        # Unlike exec_, show is asynchronous and works within the main Qt loop
        d.show()
        return w
