# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pendulum
from PyQt5.QtCore import Qt, pyqtSignal, QTimer
from PyQt5.QtWidgets import QWidget, QMenu
from PyQt5.QtGui import QPixmap

from parsec.api.protocol import UserID
from parsec.api.data import RevokedUserCertificateContent
from parsec.core.remote_devices_manager import (
    RemoteDevicesManagerError,
    RemoteDevicesManagerBackendOfflineError,
)

from parsec.core.backend_connection import BackendConnectionError, BackendNotAvailable

from parsec.core.gui.trio_thread import JobResultError, ThreadSafeQtSignal, QtToTrioJob
from parsec.core.gui.register_user_dialog import RegisterUserDialog
from parsec.core.gui.custom_dialogs import show_error, show_info, QuestionDialog
from parsec.core.gui.custom_widgets import TaskbarButton, FlowLayout
from parsec.core.gui.lang import translate as _, format_datetime
from parsec.core.gui.ui.user_button import Ui_UserButton
from parsec.core.gui.ui.users_widget import Ui_UsersWidget


class UserButton(QWidget, Ui_UserButton):
    revoke_clicked = pyqtSignal(QWidget)

    def __init__(
        self,
        user_name,
        is_current_user,
        is_admin,
        certified_on,
        is_revoked,
        current_user_is_admin,
        *args,
        **kwargs
    ):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.current_user_is_admin = current_user_is_admin
        self.is_admin = is_admin
        if is_admin:
            self.label.setPixmap(QPixmap(":/icons/images/icons/owner2.png"))
        else:
            self.label.setPixmap(QPixmap(":/icons/images/icons/user2.png"))
        self.label.is_revoked = is_revoked
        self.certified_on = certified_on
        self.is_current_user = is_current_user
        self.user_name = user_name
        self.set_display(user_name)
        self.setContextMenuPolicy(Qt.CustomContextMenu)
        self.customContextMenuRequested.connect(self.show_context_menu)

    def set_display(self, value):
        if len(value) > 16:
            value = value[:16] + "-\n" + value[16:]
        if self.is_current_user:
            value += "\n({})".format(_("USER_CURRENT_TEXT"))
        self.label_user.setText(value)

    @property
    def is_revoked(self):
        return self.label.is_revoked

    @is_revoked.setter
    def is_revoked(self, value):
        self.label.is_revoked = value
        self.label.repaint()

    def show_context_menu(self, pos):
        global_pos = self.mapToGlobal(pos)
        menu = QMenu(self)
        action = menu.addAction(_("USER_MENU_SHOW_INFO"))
        action.triggered.connect(self.show_user_info)
        if not self.label.is_revoked and not self.is_current_user and self.current_user_is_admin:
            action = menu.addAction(_("USER_MENU_REVOKE"))
            action.triggered.connect(self.revoke)
        menu.exec_(global_pos)

    def show_user_info(self):
        text = "{}\n\n".format(self.user_name)
        text += _("USER_CREATED_ON_{}").format(format_datetime(self.certified_on, full=True))
        if self.is_admin:
            text += "\n\n"
            text += _("USER_IS_ADMIN")
        if self.label.is_revoked:
            text += "\n\n"
            text += _("USER_IS_REVOKED")
        show_info(self, text)

    def revoke(self):
        self.revoke_clicked.emit(self)


async def _do_revoke_user(core, user_name, button):
    try:
        now = pendulum.now()
        revoked_device_certificate = RevokedUserCertificateContent(
            author=core.device.device_id, timestamp=now, user_id=UserID(user_name)
        ).dump_and_sign(core.device.signing_key)
        rep = await core.user_fs.backend_cmds.user_revoke(revoked_device_certificate)
        if rep["status"] != "ok":
            raise JobResultError(rep["status"])
        return button
    except BackendNotAvailable as exc:
        raise JobResultError("offline") from exc
    except BackendConnectionError as exc:
        raise JobResultError("error") from exc


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

    def __init__(self, core, jobs_ctx, event_bus, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.setupUi(self)
        self.core = core
        self.jobs_ctx = jobs_ctx
        self.event_bus = event_bus
        self.layout_users = FlowLayout(spacing=20)
        self.layout_content.addLayout(self.layout_users)
        self.users = []
        self.taskbar_buttons = []
        if core.device.is_admin:
            button_add_user = TaskbarButton(
                icon_path=":/icons/images/icons/tray_icons/plus-$STATE.svg"
            )
            button_add_user.clicked.connect(self.register_user)
            button_add_user.setToolTip(_("BUTTON_TASKBAR_ADD_USER"))
            self.taskbar_buttons.append(button_add_user)
        self.filter_timer = QTimer()
        self.filter_timer.setInterval(300)
        self.line_edit_search.textChanged.connect(self.filter_timer.start)
        self.filter_timer.timeout.connect(self.on_filter_timer_timeout)
        self.revoke_success.connect(self.on_revoke_success)
        self.revoke_error.connect(self.on_revoke_error)
        self.list_success.connect(self.on_list_success)
        self.list_error.connect(self.on_list_error)
        self.reset()

    def get_taskbar_buttons(self):
        return self.taskbar_buttons.copy()

    def on_filter_timer_timeout(self):
        self.filter_users(self.line_edit_search.text())

    def filter_users(self, pattern):
        pattern = pattern.lower()
        for i in range(self.layout_users.count()):
            item = self.layout_users.itemAt(i)
            if item:
                w = item.widget()
                if pattern and pattern not in w.user_name.lower():
                    w.hide()
                else:
                    w.show()

    def register_user(self):
        d = RegisterUserDialog(core=self.core, jobs_ctx=self.jobs_ctx, parent=self)
        d.exec_()
        self.reset()

    def add_user(self, user_name, is_current_user, is_admin, certified_on, is_revoked):
        if user_name in self.users:
            return
        button = UserButton(
            user_name,
            is_current_user,
            is_admin,
            certified_on,
            is_revoked,
            current_user_is_admin=self.core.device.is_admin,
        )
        self.layout_users.addWidget(button)
        button.revoke_clicked.connect(self.revoke_user)
        button.show()
        self.users.append(user_name)

    def on_revoke_success(self, job):
        button = job.ret
        show_info(self, _("INFO_USER_REVOKED_SUCCESS_{}").format(button.user_name))
        button.is_revoked = True

    def on_revoke_error(self, job):
        status = job.status
        if status == "already_revoked":
            errmsg = _("ERR_USER_REVOKED_ALREADY")
        elif status == "not_found":
            errmsg = _("ERR_USER_REVOKED_NOT_FOUND")
        elif status == "not_allowed":
            errmsg = _("ERR_USER_REVOKED_NOT_ENOUGH_PERMISSIONS")
        elif status == "offline":
            errmsg = _("ERR_BACKEND_OFFLINE")
        else:
            errmsg = _("ERR_USER_REVOKED_UNKNOWN_ERROR")
        show_error(self, errmsg, exception=job.exc)

    def revoke_user(self, user_button):
        result = QuestionDialog.ask(
            self,
            _("ASK_USER_REVOKE_TITLE"),
            _("ASK_USER_REVOKE_CONTENT_{}").format(user_button.user_name),
        )
        if not result:
            return
        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "revoke_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "revoke_error", QtToTrioJob),
            _do_revoke_user,
            core=self.core,
            user_name=user_button.user_name,
            button=user_button,
        )

    def on_list_success(self, job):
        self.users = []
        self.layout_users.clear()
        current_user = self.core.device.user_id
        for user_info, user_revoked_info in job.ret:
            self.add_user(
                str(user_info.user_id),
                is_current_user=current_user == user_info.user_id,
                is_admin=user_info.is_admin,
                certified_on=user_info.timestamp,
                is_revoked=user_revoked_info is not None,
            )

    def on_list_error(self, job):
        status = job.status
        if status == "offline":
            return
        else:
            errmsg = _("ERR_USER_LIST_UNKNOWN_ERROR")
        show_error(self, errmsg, exception=job.exc)

    def reset(self):
        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "list_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "list_error", QtToTrioJob),
            _do_list_users,
            core=self.core,
        )
