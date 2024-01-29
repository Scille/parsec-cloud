# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QLabel, QWidget

from parsec.api.protocol import UserProfile
from parsec.core.fs import WorkspaceFS
from parsec.core.gui.custom_dialogs import GreyedDialog, show_error, show_info
from parsec.core.gui.custom_widgets import SpinnerWidget
from parsec.core.gui.lang import translate as T
from parsec.core.gui.search_user_widget import SearchUserWidget
from parsec.core.gui.snackbar_widget import SnackbarManager
from parsec.core.gui.trio_jobs import QtToTrioJob, QtToTrioJobScheduler
from parsec.core.gui.ui.reassign_workspace_roles_summary_widget import (
    Ui_ReassignWorkspaceRolesSummaryWidget,
)
from parsec.core.gui.ui.reassign_workspace_roles_widget import Ui_ReassignWorkspaceRolesWidget
from parsec.core.gui.workspace_roles import get_role_translation
from parsec.core.logged_core import LoggedCore
from parsec.core.types import UserInfo, WorkspaceRole


class ReassignWorkspaceRolesSummaryWidget(QWidget, Ui_ReassignWorkspaceRolesSummaryWidget):
    previous_clicked = pyqtSignal()
    assign_clicked = pyqtSignal(UserInfo, dict)

    def __init__(self, user_info: UserInfo, roles: dict[WorkspaceFS, WorkspaceRole]) -> None:
        super().__init__()
        self.setupUi(self)
        self.label.setText(
            T("TEXT_REASSIGN_ROLE_SUMMARY_TITLE_user").format(
                user=user_info.human_handle.label if user_info.human_handle else user_info.user_id
            )
        )
        for workspace, role in roles.items():
            label = QLabel(
                T("TEXT_REASSIGN_ROLE_SUMMARY_DETAIL_role_workspace").format(
                    role=get_role_translation(role), workspace=workspace.get_workspace_name()
                )
            )
            self.widget_summary.layout().addWidget(label)
        self.button_assign.clicked.connect(lambda: self.assign_clicked.emit(user_info, roles))
        self.button_previous.clicked.connect(self.previous_clicked.emit)


class ReassignWorkspaceRolesWidget(QWidget, Ui_ReassignWorkspaceRolesWidget):
    def __init__(
        self,
        core: LoggedCore,
        jobs_ctx: QtToTrioJobScheduler,
        user_info: UserInfo,
    ) -> None:
        super().__init__()
        self.setupUi(self)
        self.core = core
        self.jobs_ctx = jobs_ctx
        self.roles: dict[WorkspaceFS, tuple[WorkspaceRole, WorkspaceRole]] = {}
        self.user_info = user_info
        self.dialog: GreyedDialog[ReassignWorkspaceRolesWidget] | None = None
        self.last_assign_roles_job: QtToTrioJob[None] | None = None
        self.last_filter_roles_job: QtToTrioJob[None] | None = None
        spinner = SpinnerWidget()
        self.main_layout.insertWidget(0, spinner)
        self.get_common_workspaces_job: QtToTrioJob[None] = self.jobs_ctx.submit_job(
            None, None, self._get_common_reassignable_workspaces
        )

    async def _get_common_reassignable_workspaces(self) -> None:
        def _can_reassign_workspace(client_role: WorkspaceRole) -> bool:
            return client_role in [WorkspaceRole.MANAGER, WorkspaceRole.OWNER]

        try:
            workspaces = self.core.user_fs.get_available_workspaces()
            common_workspaces = {}
            for workspace in workspaces:
                roles = await workspace.get_user_roles()
                if self.user_info.user_id in roles and _can_reassign_workspace(
                    roles[self.core.user_fs.device.user_id]
                ):
                    common_workspaces[workspace] = (
                        roles[self.core.user_fs.device.user_id],
                        roles[self.user_info.user_id],
                    )
            self.roles = common_workspaces
        except Exception:
            show_error(self, T("TEXT_GET_WORKSPACES_TO_REASSIGN_ERROR"))
            if self.dialog:
                self.dialog.accept()
        else:
            if not self.roles:
                show_info(
                    self,
                    T("TEXT_NO_WORKSPACES_TO_REASSIGN_user").format(
                        user=self.user_info.short_user_display
                    ),
                )
                if self.dialog:
                    self.dialog.accept()
            else:
                self._switch_to_search()

    def _clear_layout(self) -> None:
        if self.main_layout.count() > 0:
            item = self.main_layout.takeAt(0)
            if item and item.widget():
                item.widget().hide()
                item.widget().setParent(None)

    def _switch_to_summary(
        self, selected_user_info: UserInfo, roles: dict[WorkspaceFS, WorkspaceRole]
    ) -> None:
        self._clear_layout()
        summary_widget = ReassignWorkspaceRolesSummaryWidget(selected_user_info, roles)
        summary_widget.previous_clicked.connect(self._switch_to_search)
        summary_widget.assign_clicked.connect(self._assign_roles)
        self.main_layout.insertWidget(0, summary_widget)
        if self.dialog:
            self.dialog.update_title(
                T("TEXT_REASSIGN_WORKSPACE_ROLES_userFrom-userTo").format(
                    userFrom=self.user_info.short_user_display,
                    userTo=selected_user_info.short_user_display,
                )
            )

    def _switch_to_search(self) -> None:
        self._clear_layout()
        search_user = SearchUserWidget(
            self.core, self.jobs_ctx, [self.user_info.user_id, self.core.device.user_id]
        )
        search_user.user_selected.connect(self._on_user_selected)
        self.main_layout.insertWidget(0, search_user)
        if self.dialog:
            self.dialog.update_title(
                T("TEXT_REASSIGN_WORKSPACE_ROLES_user").format(
                    user=self.user_info.short_user_display
                )
            )

    def _on_user_selected(self, selected_user_info: UserInfo) -> None:
        self._clear_layout()
        sw = SpinnerWidget()
        self.main_layout.insertWidget(0, sw)
        self.last_filter_roles_job = self.jobs_ctx.submit_job(
            None, None, self._filter_roles, selected_user_info, self.roles
        )

    async def _filter_roles(
        self, user_info: UserInfo, roles: dict[WorkspaceFS, tuple[WorkspaceRole, WorkspaceRole]]
    ) -> None:
        WORKSPACE_ROLES_WEIGHTS = {
            WorkspaceRole.READER: 1,
            WorkspaceRole.CONTRIBUTOR: 2,
            WorkspaceRole.MANAGER: 3,
            WorkspaceRole.OWNER: 4,
        }
        new_roles: dict[WorkspaceFS, WorkspaceRole] = {}
        for workspace, (client_role, user_role) in roles.items():
            # Cannot set an Outsider to Manager or Owner, we downgrade to the next role
            if user_info.profile == UserProfile.OUTSIDER and user_role in [
                WorkspaceRole.MANAGER,
                WorkspaceRole.OWNER,
            ]:
                user_role = WorkspaceRole.CONTRIBUTOR
            # A manager cannot set an Owner or Manager, we downgrade to the next role
            if client_role == WorkspaceRole.MANAGER and user_role in [
                WorkspaceRole.OWNER,
                WorkspaceRole.MANAGER,
            ]:
                user_role = WorkspaceRole.CONTRIBUTOR

            # Make sure the user does not currently have a higher role on the workspace
            participants = await workspace.get_user_roles()
            existing_role = participants.get(user_info.user_id)
            if (
                existing_role
                and WORKSPACE_ROLES_WEIGHTS[existing_role] >= WORKSPACE_ROLES_WEIGHTS[user_role]
            ):
                continue
            new_roles[workspace] = user_role

        if not new_roles:
            show_info(
                self,
                T("TEXT_REASSIGN_ROLES_NO_ROLE_TO_ASSIGN_user").format(
                    user=user_info.short_user_display
                ),
            )
            self._switch_to_search()
        else:
            self._switch_to_summary(user_info, new_roles)

    async def _do_assign_roles(
        self, user_info: UserInfo, roles: dict[WorkspaceFS, WorkspaceRole]
    ) -> None:
        if not self.dialog:
            return

        failure_count = 0
        for workspace, role in roles.items():
            try:
                await self.core.user_fs.workspace_share(
                    workspace.workspace_id, user_info.user_id, role
                )
            except Exception:
                failure_count += 1
        if failure_count == 0:
            SnackbarManager.inform(
                T("TEXT_REASSIGN_ROLE_SUCCESS_user").format(user=user_info.short_user_display)
            )
            self.dialog.accept()
            return

        if failure_count == len(self.roles):
            show_error(self, T("TEXT_REASSIGN_ROLE_ALL_FAILED"))
        else:
            show_error(self, T("TEXT_REASSIGN_ROLE_SOME_FAILED"))
        self.dialog.accept()

    def _assign_roles(self, user_info: UserInfo, roles: dict[WorkspaceFS, WorkspaceRole]) -> None:
        self.last_assign_roles_job = self.jobs_ctx.submit_job(
            None, None, self._do_assign_roles, user_info, roles
        )

    @classmethod
    def show_modal(
        cls,
        core: LoggedCore,
        jobs_ctx: QtToTrioJobScheduler,
        user_info: UserInfo,
        parent: QWidget,
    ) -> ReassignWorkspaceRolesWidget:
        w = cls(core=core, jobs_ctx=jobs_ctx, user_info=user_info)
        d = GreyedDialog(
            w,
            title=T("TEXT_REASSIGN_WORKSPACE_ROLES_user").format(user=user_info.short_user_display),
            parent=parent,
            width=1000,
        )
        w.dialog = d
        # d.closing.connect(w.on_close)
        # Unlike exec_, show is asynchronous and works within the main Qt loop
        d.show()
        return w
