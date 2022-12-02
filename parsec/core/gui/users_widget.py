# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from math import ceil
from typing import Any

from PyQt5.QtCore import QPoint, Qt, QTimer, pyqtSignal
from PyQt5.QtGui import QColor
from PyQt5.QtWidgets import QGraphicsDropShadowEffect, QLabel, QMenu, QWidget

from parsec._parsec import InvitationEmailSentStatus, InvitationType, InviteListItem
from parsec.api.protocol import InvitationToken, UserProfile
from parsec.core.backend_connection import (
    BackendConnectionError,
    BackendInvitationOnExistingMember,
    BackendNotAvailable,
)
from parsec.core.gui import desktop, validators
from parsec.core.gui.custom_dialogs import (
    ask_question,
    get_text_input,
    show_error,
    show_info_copy_link,
)
from parsec.core.gui.custom_widgets import Pixmap, ensure_string_size
from parsec.core.gui.flow_layout import FlowLayout
from parsec.core.gui.greet_user_widget import GreetUserWidget
from parsec.core.gui.lang import translate as T
from parsec.core.gui.snackbar_widget import SnackbarManager
from parsec.core.gui.trio_jobs import JobResultError, QtToTrioJob, QtToTrioJobScheduler
from parsec.core.gui.ui.user_button import Ui_UserButton
from parsec.core.gui.ui.user_invitation_button import Ui_UserInvitationButton
from parsec.core.gui.ui.users_widget import Ui_UsersWidget
from parsec.core.logged_core import LoggedCore
from parsec.core.types import BackendInvitationAddr, UserInfo
from parsec.event_bus import EventBus

USERS_PER_PAGE = 100


class UserInvitationButton(QWidget, Ui_UserInvitationButton):
    greet_clicked = pyqtSignal(InvitationToken)
    cancel_clicked = pyqtSignal(InvitationToken)

    def __init__(self, email: str, addr: BackendInvitationAddr) -> None:
        super().__init__()
        self.setupUi(self)
        self.addr = addr
        self.email = email
        self.label_addr.setText(ensure_string_size(self.addr.to_url(), 160, self.label_addr.font()))
        self.label_addr.setToolTip(self.addr.to_url())
        self.label_email.setText(ensure_string_size(self.email, 160, self.label_email.font()))
        self.label_email.setToolTip(self.email)

        self.button_greet.clicked.connect(self._on_greet_clicked)
        self.button_cancel.clicked.connect(self._on_cancel_clicked)
        self.label_icon.apply_style()
        self.setContextMenuPolicy(Qt.CustomContextMenu)
        self.customContextMenuRequested.connect(self.show_context_menu)
        effect = QGraphicsDropShadowEffect(self)
        effect.setColor(QColor(0x99, 0x99, 0x99))
        effect.setBlurRadius(10)
        effect.setXOffset(2)
        effect.setYOffset(2)
        self.setGraphicsEffect(effect)

    def show_context_menu(self, pos: QPoint) -> None:
        global_pos = self.mapToGlobal(pos)
        menu = QMenu(self)
        action = menu.addAction(T("ACTION_USER_INVITE_COPY_ADDR"))
        action.triggered.connect(self.copy_addr)
        action = menu.addAction(T("ACTION_USER_INVITE_COPY_EMAIL"))
        action.triggered.connect(self.copy_email)
        menu.exec_(global_pos)

    def copy_addr(self) -> None:
        desktop.copy_to_clipboard(self.addr.to_url())
        SnackbarManager.inform(T("TEXT_GREET_USER_ADDR_COPIED_TO_CLIPBOARD"))

    def copy_email(self) -> None:
        desktop.copy_to_clipboard(self.email)
        SnackbarManager.inform(T("TEXT_GREET_USER_EMAIL_COPIED_TO_CLIPBOARD"))

    @property
    def token(self) -> InvitationToken:
        return self.addr.token

    def _on_greet_clicked(self) -> None:
        self.greet_clicked.emit(self.token)

    def _on_cancel_clicked(self) -> None:
        self.cancel_clicked.emit(self.token)


class UserButton(QWidget, Ui_UserButton):
    revoke_clicked = pyqtSignal(UserInfo)
    filter_user_workspaces_clicked = pyqtSignal(UserInfo)

    def __init__(
        self, user_info: UserInfo, is_current_user: bool, current_user_is_admin: bool
    ) -> None:
        super().__init__()
        self.setupUi(self)

        self.user_info = user_info
        self.is_current_user = is_current_user
        self.current_user_is_admin = current_user_is_admin

        if self.is_current_user:
            self.label_is_current.setText("({})".format(T("TEXT_USER_IS_CURRENT")))
        self.setContextMenuPolicy(Qt.CustomContextMenu)
        self.customContextMenuRequested.connect(self.show_context_menu)
        effect = QGraphicsDropShadowEffect(self)
        effect.setColor(QColor(0x99, 0x99, 0x99))
        effect.setBlurRadius(10)
        effect.setXOffset(2)
        effect.setYOffset(2)
        self.setGraphicsEffect(effect)

    @property
    def user_info(self) -> UserInfo:
        return self._user_info

    @user_info.setter
    def user_info(self, val: UserInfo) -> None:
        profiles_txt = {
            UserProfile.OUTSIDER: T("TEXT_USER_PROFILE_OUTSIDER"),
            UserProfile.STANDARD: T("TEXT_USER_PROFILE_STANDARD"),
            UserProfile.ADMIN: T("TEXT_USER_PROFILE_ADMIN"),
        }
        profiles_icons = {
            UserProfile.OUTSIDER: ":/icons/images/material/person_outline.svg",
            UserProfile.STANDARD: ":/icons/images/material/person.svg",
            UserProfile.ADMIN: ":/icons/images/material/manage_accounts.svg",
        }
        self._user_info = val
        if self.user_info.is_revoked:
            self.setToolTip(T("TEXT_USER_IS_REVOKED"))
            self.widget.setStyleSheet("background-color: #DDDDDD;")
            self.label_revoked.setText(T("TEXT_USER_IS_REVOKED"))
        else:
            self.label_revoked.setText("")
            self.setToolTip("")
            self.widget.setStyleSheet("background-color: #FFFFFF;")
        if self.user_info.human_handle:
            self.label_email.setText(
                ensure_string_size(self.user_info.human_handle.email, 160, self.label_email.font())
            )
            self.label_email.setToolTip(self.user_info.human_handle.email)

        self.label_username.setText(
            ensure_string_size(self.user_info.short_user_display, 160, self.label_username.font())
        )
        self.label_username.setToolTip(self.user_info.short_user_display)
        self.label_role.setText(profiles_txt[self.user_info.profile])
        pix = Pixmap(profiles_icons[self.user_info.profile])
        pix.replace_color(QColor(0, 0, 0), QColor(153, 153, 153))
        self.label_icon.setPixmap(pix)

    @property
    def user_email(self) -> str:
        if self.user_info.human_handle:
            return self.user_info.human_handle.email
        return ""

    def show_context_menu(self, pos: QPoint) -> None:
        global_pos = self.mapToGlobal(pos)
        menu = QMenu(self)

        if self.user_email:
            action = menu.addAction(T("ACTION_USER_INVITE_COPY_EMAIL"))
            action.triggered.connect(self.copy_email)

        if not self.is_current_user:
            action = menu.addAction(T("ACTION_USER_MENU_FILTER"))
            action.triggered.connect(self.filter_user_workspaces)
            if not self.user_info.is_revoked and self.current_user_is_admin:
                action = menu.addAction(T("ACTION_USER_MENU_REVOKE"))
                action.triggered.connect(self.revoke)

        if not menu.isEmpty():
            menu.exec_(global_pos)

    def copy_email(self) -> None:
        desktop.copy_to_clipboard(self.user_email)
        SnackbarManager.inform(T("TEXT_GREET_USER_EMAIL_COPIED_TO_CLIPBOARD"))

    def revoke(self) -> None:
        self.revoke_clicked.emit(self.user_info)

    def filter_user_workspaces(self) -> None:
        self.filter_user_workspaces_clicked.emit(self.user_info)


async def _do_revoke_user(core: LoggedCore, user_info: UserInfo) -> UserInfo:
    try:
        await core.revoke_user(user_info.user_id)
        user_info = await core.get_user_info(user_info.user_id)
        return user_info
    except BackendNotAvailable as exc:
        raise JobResultError("offline") from exc
    except BackendConnectionError as exc:
        raise JobResultError("error") from exc


async def _do_list_users_and_invitations(
    core: LoggedCore,
    page: int,
    pattern: str | None = None,
    omit_revoked: bool = False,
    omit_invitation: bool = False,
) -> tuple[int, list[UserInfo], list[InviteListItem]]:
    try:
        if not pattern:
            users, total = await core.find_humans(
                page=page, per_page=USERS_PER_PAGE, omit_revoked=omit_revoked
            )
            invitations = [] if omit_invitation else await core.list_invitations()
            return total, users, [inv for inv in invitations if inv.type == InvitationType.USER]
        else:
            users, total = await core.find_humans(
                page=page, per_page=USERS_PER_PAGE, query=pattern, omit_revoked=omit_revoked
            )
            return total, users, []
    except BackendNotAvailable as exc:
        raise JobResultError("offline") from exc

    except BackendConnectionError as exc:
        raise JobResultError("error") from exc


async def _do_cancel_invitation(core: LoggedCore, token: InvitationToken) -> None:
    try:
        await core.delete_invitation(token=token)
    except BackendNotAvailable as exc:
        raise JobResultError("offline") from exc
    except BackendConnectionError as exc:
        raise JobResultError("error") from exc


async def _do_invite_user(
    core: LoggedCore, email: str
) -> tuple[str, BackendInvitationAddr, InvitationEmailSentStatus]:
    try:
        invitation_addr, email_sent_status = await core.new_user_invitation(
            email=email, send_email=True
        )
        return email, invitation_addr, email_sent_status
    except BackendNotAvailable as exc:
        raise JobResultError("offline") from exc
    except BackendInvitationOnExistingMember as exc:
        raise JobResultError("already_member") from exc
    except BackendConnectionError as exc:
        raise JobResultError("error") from exc


class UsersWidget(QWidget, Ui_UsersWidget):
    revoke_success = pyqtSignal(QtToTrioJob)
    revoke_error = pyqtSignal(QtToTrioJob)
    list_success = pyqtSignal(QtToTrioJob)
    list_error = pyqtSignal(QtToTrioJob)
    invite_user_success = pyqtSignal(QtToTrioJob)
    invite_user_error = pyqtSignal(QtToTrioJob)
    cancel_invitation_success = pyqtSignal(QtToTrioJob)
    cancel_invitation_error = pyqtSignal(QtToTrioJob)
    filter_shared_workspaces_request = pyqtSignal(UserInfo)

    def __init__(
        self,
        core: LoggedCore,
        jobs_ctx: QtToTrioJobScheduler,
        event_bus: EventBus,
        *args: Any,
        **kwargs: Any,
    ) -> None:
        super().__init__(*args, **kwargs)

        self.setupUi(self)
        self.core = core
        self.jobs_ctx = jobs_ctx
        self.event_bus = event_bus
        self.layout_users = FlowLayout(spacing=40)
        self.layout_content.addLayout(self.layout_users)
        self.button_add_user.apply_style()
        if core.device.is_admin:
            self.button_add_user.clicked.connect(self.invite_user)
        else:
            self.button_add_user.hide()
        self.search_timer = QTimer()
        self.search_timer.setInterval(300)
        self.search_timer.setSingleShot(True)
        self.search_timer.timeout.connect(self.on_filter)
        self.button_previous_page.clicked.connect(self.show_previous_page)
        self.button_next_page.clicked.connect(self.show_next_page)
        self.line_edit_search.textChanged.connect(self._search_changed)
        self.line_edit_search.clear_clicked.connect(self.on_filter)
        self.revoke_success.connect(self._on_revoke_success)
        self.revoke_error.connect(self._on_revoke_error)
        self.list_success.connect(self._on_list_success)
        self.list_error.connect(self._on_list_error)
        self.invite_user_success.connect(self._on_invite_user_success)
        self.invite_user_error.connect(self._on_invite_user_error)
        self.cancel_invitation_success.connect(self._on_cancel_invitation_success)
        self.cancel_invitation_error.connect(self._on_cancel_invitation_error)
        self.checkbox_filter_revoked.clicked.connect(self.reset)
        self.checkbox_filter_invitation.clicked.connect(self.reset)

    def show(self) -> None:
        self._page = 1
        self.reset()
        super().show()

    def show_next_page(self) -> None:
        self._page += 1
        self.on_filter(change_page=True)

    def show_previous_page(self) -> None:
        if self._page > 1:
            self._page -= 1
        self.on_filter(change_page=True)

    def _search_changed(self) -> None:
        self.search_timer.start()

    def on_filter(self, change_page: bool = False) -> None:
        self.search_timer.stop()
        if change_page is False:
            self._page = 1
        self.reset()

    def invite_user(self) -> None:
        user_email = get_text_input(
            self,
            T("TEXT_USER_INVITE_EMAIL"),
            T("TEXT_USER_INVITE_EMAIL_INSTRUCTIONS"),
            placeholder=T("TEXT_USER_INVITE_EMAIL_PLACEHOLDER"),
            button_text=T("ACTION_USER_INVITE_DO_INVITE"),
            validator=validators.EmailValidator(),
        )
        if not user_email:
            return

        _ = self.jobs_ctx.submit_job(
            (self, "invite_user_success"),
            (self, "invite_user_error"),
            _do_invite_user,
            core=self.core,
            email=user_email,
        )

    def add_user(self, user_info: UserInfo, is_current_user: bool) -> None:
        button = UserButton(
            user_info=user_info,
            is_current_user=is_current_user,
            current_user_is_admin=self.core.device.is_admin,
        )
        self.layout_users.addWidget(button)
        button.filter_user_workspaces_clicked.connect(self.filter_shared_workspaces_request.emit)
        button.revoke_clicked.connect(self.revoke_user)
        button.show()

    def add_user_invitation(self, email: str, invite_addr: BackendInvitationAddr) -> None:
        button = UserInvitationButton(email, invite_addr)
        self.layout_users.addWidget(button)
        button.greet_clicked.connect(self.greet_user)
        button.cancel_clicked.connect(self.cancel_invitation)
        button.show()

    def greet_user(self, token: InvitationToken) -> None:
        GreetUserWidget.show_modal(
            core=self.core, jobs_ctx=self.jobs_ctx, token=token, parent=self, on_finished=self.reset
        )

    def cancel_invitation(self, token: InvitationToken) -> None:
        r = ask_question(
            self,
            T("TEXT_USER_INVITE_CANCEL_INVITE_QUESTION_TITLE"),
            T("TEXT_USER_INVITE_CANCEL_INVITE_QUESTION_CONTENT"),
            [T("TEXT_USER_INVITE_CANCEL_INVITE_ACCEPT"), T("ACTION_ENABLE_TELEMETRY_REFUSE")],
        )
        if r != T("TEXT_USER_INVITE_CANCEL_INVITE_ACCEPT"):
            return
        _ = self.jobs_ctx.submit_job(
            (self, "cancel_invitation_success"),
            (self, "cancel_invitation_error"),
            _do_cancel_invitation,
            core=self.core,
            token=token,
        )

    def _on_revoke_success(self, job: QtToTrioJob[UserInfo]) -> None:
        assert job.is_finished()
        assert job.status == "ok"

        user_info = job.ret
        assert user_info is not None

        SnackbarManager.inform(
            T("TEXT_USER_REVOKE_SUCCESS_user").format(user=user_info.short_user_display),
            timeout=5000,
        )
        for i in range(self.layout_users.count()):
            item = self.layout_users.itemAt(i)
            if item:
                button = item.widget()
                if (
                    button
                    and isinstance(button, UserButton)
                    and button.user_info.user_id == user_info.user_id
                ):
                    button.user_info = user_info

    def _on_revoke_error(self, job: QtToTrioJob[UserInfo]) -> None:
        assert job.is_finished()
        assert job.status != "ok"

        status = job.status
        if status == "already_revoked":
            err_msg = T("TEXT_USER_REVOCATION_USER_ALREADY_REVOKED")
        elif status == "not_found":
            err_msg = T("TEXT_USER_REVOCATION_USER_NOT_FOUND")
        elif status == "not_allowed":
            err_msg = T("TEXT_USER_REVOCATION_NOT_ENOUGH_PERMISSIONS")
        elif status == "offline":
            err_msg = T("TEXT_USER_REVOCATION_BACKEND_OFFLINE")
        else:
            err_msg = T("TEXT_USER_REVOCATION_UNKNOWN_FAILURE")
        show_error(self, err_msg, exception=job.exc)

    def revoke_user(self, user_info: UserInfo) -> None:
        result = ask_question(
            self,
            T("TEXT_USER_REVOCATION_TITLE"),
            T("TEXT_USER_REVOCATION_INSTRUCTIONS_user").format(user=user_info.short_user_display),
            [T("ACTION_USER_REVOCATION_CONFIRM"), T("ACTION_CANCEL")],
            oriented_question=True,
            dangerous_yes=True,
        )
        if result != T("ACTION_USER_REVOCATION_CONFIRM"):
            return
        _ = self.jobs_ctx.submit_job(
            (self, "revoke_success"),
            (self, "revoke_error"),
            _do_revoke_user,
            core=self.core,
            user_info=user_info,
        )

    def pagination(self, total: int, users_on_page: int) -> None:
        """Show/activate or hide/deactivate previous and next page button"""
        self.label_page_info.show()
        # Set plage of users displayed
        user_from = (self._page - 1) * USERS_PER_PAGE + 1
        user_to = user_from - 1 + users_on_page
        self.label_page_info.setText(
            T("TEXT_USERS_PAGE_INFO_page-pagetotal-userfrom-userto-usertotal").format(
                page=self._page,
                pagetotal=ceil(total / USERS_PER_PAGE),
                userfrom=user_from,
                userto=user_to,
                usertotal=total,
            )
        )
        if total > USERS_PER_PAGE:
            self.button_previous_page.show()
            self.button_next_page.show()
            self.button_previous_page.setEnabled(True)
            self.button_next_page.setEnabled(True)
            if self._page * USERS_PER_PAGE >= total:
                self.button_next_page.setEnabled(False)
            else:
                self.button_next_page.setEnabled(True)
            if self._page <= 1:
                self.button_previous_page.setEnabled(False)
            else:
                self.button_previous_page.setEnabled(True)
        else:
            self.button_previous_page.hide()
            self.button_next_page.hide()

    def _on_list_success(
        self, job: QtToTrioJob[tuple[int, list[UserInfo], list[InviteListItem]]]
    ) -> None:
        assert job.is_finished()
        assert job.status == "ok"

        self.layout_users.clear()

        assert job.ret is not None
        total, users, invitations = job.ret
        # Securing if page go to far
        if total == 0 and self._page > 1:
            self._page -= 1
            self.reset()

        current_user = self.core.device.user_id

        for invitation in reversed(invitations):
            addr = BackendInvitationAddr.build(
                backend_addr=self.core.device.organization_addr.get_backend_addr(),
                organization_id=self.core.device.organization_id,
                invitation_type=InvitationType.USER,
                token=invitation.token,
            )
            self.add_user_invitation(invitation.claimer_email, addr)
        for user_info in users:
            self.add_user(user_info=user_info, is_current_user=current_user == user_info.user_id)
        self.spinner.hide()

        self.pagination(total=total, users_on_page=len(users))
        self.line_edit_search.setFocus()

    def _on_list_error(
        self, job: QtToTrioJob[tuple[int, list[UserInfo], list[InviteListItem]]]
    ) -> None:
        assert job.is_finished()
        assert job.status != "ok"

        self.layout_users.clear()

        status = job.status
        if status in ["error", "offline"]:
            label = QLabel(T("TEXT_USER_LIST_RETRIEVABLE_FAILURE"))
            label.setAlignment(Qt.AlignHCenter | Qt.AlignVCenter)
            self.layout_users.addWidget(label)
            return
        else:
            errmsg = T("TEXT_USER_LIST_RETRIEVABLE_FAILURE")
        self.spinner.hide()

        show_error(self, errmsg, exception=job.exc)

    def _on_cancel_invitation_success(self, job: QtToTrioJob[None]) -> None:
        assert job.is_finished()
        assert job.status == "ok"
        SnackbarManager.inform(T("TEXT_USER_INVITATION_CANCELLED"))
        self.reset()

    def _on_cancel_invitation_error(self, job: QtToTrioJob[None]) -> None:
        assert job.is_finished()
        assert job.status != "ok"

        show_error(self, T("TEXT_INVITE_USER_CANCEL_ERROR"), exception=job.exc)

    def _on_invite_user_success(
        self, job: QtToTrioJob[tuple[str, BackendInvitationAddr, InvitationEmailSentStatus]]
    ) -> None:
        assert job.is_finished()
        assert job.status == "ok"

        assert job.ret is not None
        email, invitation_addr, email_sent_status = job.ret
        if email_sent_status == InvitationEmailSentStatus.SUCCESS:
            SnackbarManager.inform(T("TEXT_USER_INVITE_SUCCESS_email").format(email=email))
        elif email_sent_status == InvitationEmailSentStatus.BAD_RECIPIENT:
            show_info_copy_link(
                self,
                T("TEXT_EMAIL_FAILED_TO_SEND_TITLE"),
                T("TEXT_INVITE_USER_EMAIL_BAD_RECIPIENT_directlink").format(
                    directlink=invitation_addr.to_url()
                ),
                T("ACTION_COPY_ADDR"),
                invitation_addr.to_url(),
            )
        else:
            show_info_copy_link(
                self,
                T("TEXT_EMAIL_FAILED_TO_SEND_TITLE"),
                T("TEXT_INVITE_USER_EMAIL_NOT_AVAILABLE_directlink").format(
                    directlink=invitation_addr.to_url()
                ),
                T("ACTION_COPY_ADDR"),
                invitation_addr.to_url(),
            )

        self.reset()

    def _on_invite_user_error(
        self, job: QtToTrioJob[tuple[str, BackendInvitationAddr, InvitationEmailSentStatus]]
    ) -> None:
        assert job.is_finished()
        assert job.status != "ok"

        status = job.status
        if status == "offline":
            errmsg = T("TEXT_INVITE_USER_INVITE_OFFLINE")
        elif status == "already_member":
            errmsg = T("TEXT_INVITE_USER_ALREADY_MEMBER_ERROR")
        else:
            errmsg = T("TEXT_INVITE_USER_INVITE_ERROR")

        show_error(self, errmsg, exception=job.exc)

    def reset(self) -> None:
        self.label_page_info.hide()
        self.button_previous_page.hide()
        self.button_next_page.hide()
        self.spinner.show()
        pattern = self.line_edit_search.text()
        _ = self.jobs_ctx.submit_job(
            (self, "list_success"),
            (self, "list_error"),
            _do_list_users_and_invitations,
            core=self.core,
            page=self._page,
            omit_revoked=self.checkbox_filter_revoked.isChecked(),
            omit_invitation=self.checkbox_filter_invitation.isChecked(),
            pattern=pattern,
        )
