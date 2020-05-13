# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pendulum
from uuid import UUID
from PyQt5.QtCore import Qt, pyqtSignal, QTimer
from PyQt5.QtWidgets import QWidget, QMenu, QGraphicsDropShadowEffect
from PyQt5.QtGui import QColor

from parsec.api.protocol import UserID, InvitationType, InvitationDeletedReason
from parsec.api.data import RevokedUserCertificateContent, UserProfile
from parsec.core.types import BackendInvitationAddr
from parsec.core.remote_devices_manager import (
    RemoteDevicesManagerError,
    RemoteDevicesManagerBackendOfflineError,
)

from parsec.core.backend_connection import (
    BackendConnectionError,
    BackendNotAvailable,
    backend_authenticated_cmds_factory,
)

from parsec.core.gui.trio_thread import JobResultError, ThreadSafeQtSignal, QtToTrioJob
from parsec.core.gui.custom_dialogs import show_error, show_info, ask_question, get_text_input
from parsec.core.gui.custom_widgets import ensure_string_size
from parsec.core.gui.flow_layout import FlowLayout
from parsec.core.gui import desktop
from parsec.core.gui.lang import translate as _
from parsec.core.gui.greet_user_widget import GreetUserWidget
from parsec.core.gui.ui.user_button import Ui_UserButton
from parsec.core.gui.ui.user_invitation_button import Ui_UserInvitationButton
from parsec.core.gui.ui.users_widget import Ui_UsersWidget


class UserInvitationButton(QWidget, Ui_UserInvitationButton):
    greet_clicked = pyqtSignal(UUID)
    cancel_clicked = pyqtSignal(UUID)

    def __init__(self, email, addr):
        super().__init__()
        self.setupUi(self)
        self.addr = addr
        self.email = email
        self.label_addr.setText(ensure_string_size(str(self.addr), 260, self.label_addr.font()))
        self.label_addr.setToolTip(str(self.addr))
        self.label_email.setText(ensure_string_size(self.email, 260, self.label_email.font()))
        self.label_email.setToolTip(self.email)

        self.button_greet.clicked.connect(self._on_greet_clicked)
        self.button_cancel.clicked.connect(self._on_cancel_clicked)
        self.button_cancel.apply_style()
        self.label_icon.apply_style()
        self.setContextMenuPolicy(Qt.CustomContextMenu)
        self.customContextMenuRequested.connect(self.show_context_menu)
        effect = QGraphicsDropShadowEffect(self)
        effect.setColor(QColor(0x99, 0x99, 0x99))
        effect.setBlurRadius(10)
        effect.setXOffset(2)
        effect.setYOffset(2)
        self.setGraphicsEffect(effect)

    def show_context_menu(self, pos):
        global_pos = self.mapToGlobal(pos)
        menu = QMenu(self)
        action = menu.addAction(_("ACTION_USER_INVITE_COPY_ADDR"))
        action.triggered.connect(self.copy_addr)
        action = menu.addAction(_("ACTION_USER_INVITE_COPY_EMAIL"))
        action.triggered.connect(self.copy_email)
        menu.exec_(global_pos)

    def copy_addr(self):
        desktop.copy_to_clipboard(str(self.addr))

    def copy_email(self):
        desktop.copy_to_clipboard(self.email)

    @property
    def token(self):
        return self.addr.token

    def _on_greet_clicked(self):
        self.greet_clicked.emit(self.token)

    def _on_cancel_clicked(self):
        self.cancel_clicked.emit(self.token)


class UserButton(QWidget, Ui_UserButton):
    revoke_clicked = pyqtSignal(str)

    def __init__(
        self,
        user_name,
        human_handle,
        is_current_user,
        profile,
        certified_on,
        is_revoked,
        current_user_is_admin,
    ):
        super().__init__()
        self.setupUi(self)

        profiles_txt = {
            UserProfile.OUTSIDER: _("TEXT_USER_PROFILE_OUTSIDER"),
            UserProfile.STANDARD: _("TEXT_USER_PROFILE_STANDARD"),
            UserProfile.ADMIN: _("TEXT_USER_PROFILE_ADMIN"),
        }

        self.user_name = user_name
        self.current_user_is_admin = current_user_is_admin
        self.profile = profile
        self.is_revoked = is_revoked
        self.certified_on = certified_on
        self.is_current_user = is_current_user
        if human_handle:
            self.label_username.setText(
                ensure_string_size(human_handle.label, 260, self.label_username.font())
            )
            self.label_username.setToolTip(human_handle.label)
            if self.is_current_user:
                self.label_email.setText(
                    ensure_string_size(human_handle.email, 260, self.label_email.font())
                )
                self.label_email.setToolTip(human_handle.email)
        else:
            self.label_username.setText(
                ensure_string_size(user_name, 260, self.label_username.font())
            )
            self.label_username.setToolTip(user_name)
        if self.is_current_user:
            self.label_is_current.setText("({})".format(_("TEXT_USER_IS_CURRENT")))
        self.label_icon.apply_style()
        self.setContextMenuPolicy(Qt.CustomContextMenu)
        self.customContextMenuRequested.connect(self.show_context_menu)
        # This button is useless, its only purpose is to get the same alignement as a
        # UserInvitationButton
        self.button_cancel.hide()
        self.label_role.setText(profiles_txt[profile])
        effect = QGraphicsDropShadowEffect(self)
        effect.setColor(QColor(0x99, 0x99, 0x99))
        effect.setBlurRadius(10)
        effect.setXOffset(2)
        effect.setYOffset(2)
        self.setGraphicsEffect(effect)

    @property
    def is_revoked(self):
        return self._is_revoked

    @property
    def displayed_name(self):
        return self.label_username.text()

    @is_revoked.setter
    def is_revoked(self, value):
        self._is_revoked = value
        if value:
            self.setToolTip(_("TEXT_USER_IS_REVOKED"))
            self.widget.setStyleSheet("background-color: #DDDDDD;")
        else:
            self.setToolTip("")
            self.widget.setStyleSheet("background-color: #FFFFFF;")

    def show_context_menu(self, pos):
        if self.is_revoked or self.is_current_user or not self.current_user_is_admin:
            return
        global_pos = self.mapToGlobal(pos)
        menu = QMenu(self)
        action = menu.addAction(_("ACTION_USER_MENU_REVOKE"))
        action.triggered.connect(self.revoke)
        menu.exec_(global_pos)

    def revoke(self):
        self.revoke_clicked.emit(self.user_name)


async def _do_revoke_user(core, user_name):
    try:
        now = pendulum.now()
        revoked_device_certificate = RevokedUserCertificateContent(
            author=core.device.device_id, timestamp=now, user_id=UserID(user_name)
        ).dump_and_sign(core.device.signing_key)
        rep = await core.user_fs.backend_cmds.user_revoke(revoked_device_certificate)
        if rep["status"] != "ok":
            raise JobResultError(rep["status"])
        return user_name
    except BackendNotAvailable as exc:
        raise JobResultError("offline") from exc
    except BackendConnectionError as exc:
        raise JobResultError("error") from exc


async def _do_list_invitations(core):
    async with backend_authenticated_cmds_factory(
        addr=core.device.organization_addr,
        device_id=core.device.device_id,
        signing_key=core.device.signing_key,
        keepalive=core.config.backend_connection_keepalive,
    ) as cmds:
        rep = await cmds.invite_list()
        if rep["status"] != "ok":
            raise JobResultError(rep["status"])
        return [inv for inv in rep["invitations"] if inv["type"] == InvitationType.USER]


async def _do_cancel_invitation(device, config, token):
    async with backend_authenticated_cmds_factory(
        addr=device.organization_addr,
        device_id=device.device_id,
        signing_key=device.signing_key,
        keepalive=config.backend_connection_keepalive,
    ) as cmds:
        rep = await cmds.invite_delete(token=token, reason=InvitationDeletedReason.CANCELLED)
        if rep["status"] != "ok":
            raise JobResultError(rep["status"])


async def _do_invite_user(device, config, email):
    async with backend_authenticated_cmds_factory(
        addr=device.organization_addr,
        device_id=device.device_id,
        signing_key=device.signing_key,
        keepalive=config.backend_connection_keepalive,
    ) as cmds:
        rep = await cmds.invite_new(type=InvitationType.USER, claimer_email=email, send_email=False)
        if rep["status"] != "ok":
            raise JobResultError(rep["status"])
        action_addr = BackendInvitationAddr.build(
            backend_addr=device.organization_addr,
            organization_id=device.organization_id,
            invitation_type=InvitationType.USER,
            token=rep["token"],
        )
        return action_addr


async def _do_list_users(core):
    try:
        rep = await core.user_fs.backend_cmds.user_find()
        if rep["status"] != "ok":
            raise JobResultError(rep["status"])
    except BackendNotAvailable as exc:
        raise JobResultError("offline") from exc
    except BackendConnectionError as exc:
        raise JobResultError("error") from exc
    try:
        ret = []
        for user in rep["results"]:
            user_info, user_revoked_info = await core.remote_devices_manager.get_user(user)
            ret.append((user_info, user_revoked_info))
        return ret
    except RemoteDevicesManagerBackendOfflineError as exc:
        raise JobResultError("offline") from exc
    except RemoteDevicesManagerError as exc:
        raise JobResultError("error") from exc


class UsersWidget(QWidget, Ui_UsersWidget):
    revoke_success = pyqtSignal(QtToTrioJob)
    revoke_error = pyqtSignal(QtToTrioJob)
    list_success = pyqtSignal(QtToTrioJob)
    list_error = pyqtSignal(QtToTrioJob)
    invite_user_success = pyqtSignal(QtToTrioJob)
    invite_user_error = pyqtSignal(QtToTrioJob)
    list_invitations_success = pyqtSignal(QtToTrioJob)
    list_invitations_error = pyqtSignal(QtToTrioJob)
    cancel_invitation_success = pyqtSignal(QtToTrioJob)
    cancel_invitation_error = pyqtSignal(QtToTrioJob)

    def __init__(self, core, jobs_ctx, event_bus, *args, **kwargs):
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
        self.filter_timer = QTimer()
        self.filter_timer.setInterval(300)
        self.line_edit_search.textChanged.connect(self.filter_timer.start)
        self.filter_timer.timeout.connect(self.on_filter_timer_timeout)
        self.revoke_success.connect(self.on_revoke_success)
        self.revoke_error.connect(self.on_revoke_error)
        self.list_success.connect(self.on_list_success)
        self.list_error.connect(self.on_list_error)
        self.list_invitations_success.connect(self._on_list_invitations_success)
        self.list_invitations_error.connect(self._on_list_invitations_error)
        self.invite_user_success.connect(self._on_invite_user_success)
        self.invite_user_error.connect(self._on_invite_user_error)
        self.cancel_invitation_success.connect(self._on_cancel_invitation_success)
        self.cancel_invitation_error.connect(self._on_cancel_invitation_error)
        self.reset()

    def on_filter_timer_timeout(self):
        self.filter_users(self.line_edit_search.text())

    def filter_users(self, pattern):
        pattern = pattern.lower()
        for i in range(self.layout_users.count()):
            item = self.layout_users.itemAt(i)
            if item:
                w = item.widget()
                if pattern and (
                    isinstance(w, UserButton)
                    and pattern not in w.displayed_name.lower()
                    or isinstance(w, UserInvitationButton)
                    and pattern not in w.email.lower()
                ):
                    w.hide()
                else:
                    w.show()

    def invite_user(self):
        user_email = get_text_input(
            self,
            _("TEXT_USER_INVITE_EMAIL"),
            _("TEXT_USER_INVITE_EMAIL_INSTRUCTIONS"),
            placeholder=_("TEXT_USER_INVITE_EMAIL_PLACEHOLDER"),
            button_text=_("ACTION_USER_INVITE_DO_INVITE"),
        )
        if not user_email:
            return

        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "invite_user_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "invite_user_error", QtToTrioJob),
            _do_invite_user,
            device=self.core.device,
            config=self.core.config,
            email=user_email,
        )

    def add_user(self, user_name, human_handle, is_current_user, profile, certified_on, is_revoked):
        button = UserButton(
            user_name=user_name,
            human_handle=human_handle,
            is_current_user=is_current_user,
            profile=profile,
            certified_on=certified_on,
            is_revoked=is_revoked,
            current_user_is_admin=self.core.device.is_admin,
        )
        self.layout_users.addWidget(button)
        button.revoke_clicked.connect(self.revoke_user)
        button.show()

    def add_user_invitation(self, email, invite_addr):
        button = UserInvitationButton(email, invite_addr)
        self.layout_users.addWidget(button)
        button.greet_clicked.connect(self.greet_user)
        button.cancel_clicked.connect(self.cancel_invitation)
        button.show()

    def greet_user(self, token):
        GreetUserWidget.exec_modal(core=self.core, jobs_ctx=self.jobs_ctx, token=token, parent=self)
        self.reset()

    def cancel_invitation(self, token):
        r = ask_question(
            self,
            _("TEXT_USER_INVITE_CANCEL_INVITE_QUESTION_TITLE"),
            _("TEXT_USER_INVITE_CANCEL_INVITE_QUESTION_CONTENT"),
            [_("TEXT_USER_INVITE_CANCEL_INVITE_ACCEPT"), _("ACTION_NO")],
        )
        if r != _("TEXT_USER_INVITE_CANCEL_INVITE_ACCEPT"):
            return
        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "cancel_invitation_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "cancel_invitation_error", QtToTrioJob),
            _do_cancel_invitation,
            device=self.core.device,
            config=self.core.config,
            token=token,
        )

    def on_revoke_success(self, job):
        show_info(self, _("TEXT_USER_REVOKE_SUCCESS_user").format(user=job.ret))
        for i in range(self.layout_users.count()):
            item = self.layout_users.itemAt(i)
            if item:
                widget = item.widget()
                if widget and isinstance(widget, UserButton):
                    if widget.user_name == job.ret:
                        widget.is_revoked = True
                        return

    def on_revoke_error(self, job):
        status = job.status
        if status == "already_revoked":
            errmsg = _("TEXT_USER_REVOCATION_USER_ALREADY_REVOKED")
        elif status == "not_found":
            errmsg = _("TEXT_USER_REVOCATION_USER_NOT_FOUND")
        elif status == "not_allowed":
            errmsg = _("TEXT_USER_REVOCATION_NOT_ENOUGH_PERMISSIONS")
        elif status == "offline":
            errmsg = _("TEXT_USER_REVOCATION_BACKEND_OFFLINE")
        else:
            errmsg = _("TEXT_USER_REVOCATION_UNKNOWN_FAILURE")
        show_error(self, errmsg, exception=job.exc)

    def revoke_user(self, user_name):
        result = ask_question(
            self,
            _("TEXT_USER_REVOCATION_TITLE"),
            _("TEXT_USER_REVOCATION_INSTRUCTIONS_user").format(user=user_name),
            [_("ACTION_USER_REVOCATION_CONFIRM"), _("ACTION_CANCEL")],
        )
        if result != _("ACTION_USER_REVOCATION_CONFIRM"):
            return
        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "revoke_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "revoke_error", QtToTrioJob),
            _do_revoke_user,
            core=self.core,
            user_name=user_name,
        )

    def on_list_success(self, job):
        current_user = self.core.device.user_id
        for user_info, user_revoked_info in job.ret:
            self.add_user(
                user_name=str(user_info.user_id),
                human_handle=user_info.human_handle,
                is_current_user=current_user == user_info.user_id,
                profile=user_info.profile,
                certified_on=user_info.timestamp,
                is_revoked=user_revoked_info is not None,
            )

    def on_list_error(self, job):
        status = job.status
        if status == "offline":
            return
        else:
            errmsg = _("TEXT_USER_LIST_RETRIEVABLE_FAILURE")
        show_error(self, errmsg, exception=job.exc)

    def _on_cancel_invitation_success(self, job):
        self.reset()

    def _on_cancel_invitation_error(self, job):
        assert job.is_finished()
        assert job.status != "ok"
        show_error(self, _("TEXT_INVITE_USER_CANCEL_ERROR"), exception=job.exc)

    def _on_invite_user_success(self, job):
        self.reset()

    def _on_invite_user_error(self, job):
        assert job.is_finished()
        assert job.status != "ok"
        show_error(self, _("TEXT_INVITE_USER_INVITE_ERROR"), exception=job.exc)

    def _on_list_invitations_success(self, job):
        if not job.ret:
            return
        for invitation in job.ret:
            addr = BackendInvitationAddr.build(
                backend_addr=self.core.device.organization_addr,
                organization_id=self.core.device.organization_id,
                invitation_type=InvitationType.USER,
                token=invitation["token"],
            )
            self.add_user_invitation(invitation["claimer_email"], addr)

    def _on_list_invitations_error(self, job):
        pass

    def reset(self):
        self.layout_users.clear()

        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "list_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "list_error", QtToTrioJob),
            _do_list_users,
            core=self.core,
        )
        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "list_invitations_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "list_invitations_error", QtToTrioJob),
            _do_list_invitations,
            core=self.core,
        )
