# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pendulum
from PyQt5.QtCore import Qt, pyqtSignal, QTimer
from PyQt5.QtWidgets import QWidget, QMenu
from PyQt5.QtGui import QPixmap

from parsec.crypto import build_revoked_device_certificate
from parsec.core.backend_connection import BackendNotAvailable, BackendCmdsBadResponse

from parsec.core.gui.trio_thread import JobResultError, ThreadSafeQtSignal, QtToTrioJob
from parsec.core.gui.register_user_dialog import RegisterUserDialog
from parsec.core.gui.custom_widgets import TaskbarButton, show_error, show_info, QuestionDialog
from parsec.core.gui.lang import translate as _, format_datetime
from parsec.core.gui.ui.user_button import Ui_UserButton
from parsec.core.gui.ui.users_widget import Ui_UsersWidget


class UserButton(QWidget, Ui_UserButton):
    revoke_clicked = pyqtSignal(QWidget)

    def __init__(
        self, user_name, is_current_user, is_admin, certified_on, is_revoked, *args, **kwargs
    ):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
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
            value += _("\n(you)")
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
        action = menu.addAction(_("Show info"))
        action.triggered.connect(self.show_user_info)
        if not self.label.is_revoked and not self.is_current_user:
            action = menu.addAction(_("Revoke"))
            action.triggered.connect(self.revoke)
        menu.exec_(global_pos)

    def show_user_info(self):
        text = "{}\n\n".format(self.user_name)
        text += _("Created on {}").format(format_datetime(self.certified_on))
        if self.label.is_revoked:
            text += "\n\n"
            text += _("This user has been revoked.")
        show_info(self, text)

    def revoke(self):
        self.revoke_clicked.emit(self)


async def _do_revoke_user(core, user_name, button):
    try:
        user_info, user_devices = await core.remote_devices_manager.get_user_and_devices(user_name)
        for device in user_devices:
            revoked_device_certificate = build_revoked_device_certificate(
                core.device.device_id, core.device.signing_key, device.device_id, pendulum.now()
            )
            await core.user_fs.backend_cmds.device_revoke(revoked_device_certificate)
        return button
    except BackendCmdsBadResponse as exc:
        raise JobResultError(exc.status) from exc


async def _do_list_users(core):
    try:
        ret = {}
        users = await core.user_fs.backend_cmds.user_find()
        for user in users:
            user_info, user_devices = await core.remote_devices_manager.get_user_and_devices(user)
            ret[user] = (user_info, user_devices)
        return ret
    except BackendNotAvailable as exc:
        raise JobResultError("offline") from exc


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
        self.users = []
        self.taskbar_buttons = []
        if core.device.is_admin:
            button_add_user = TaskbarButton(icon_path=":/icons/images/icons/plus_off.png")
            button_add_user.clicked.connect(self.register_user)
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

    def disconnect_all(self):
        pass

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
        button = UserButton(user_name, is_current_user, is_admin, certified_on, is_revoked)
        self.layout_users.addWidget(button, int(len(self.users) / 4), int(len(self.users) % 4))
        button.revoke_clicked.connect(self.revoke_user)
        button.show()
        self.users.append(user_name)

    def on_revoke_success(self, job):
        button = job.ret
        show_info(self, _('User "{}" has been revoked.').format(button.user_name))
        button.is_revoked = True

    def on_revoke_error(self, job):
        status = job.status
        if status == "already_revoked":
            show_error(self, _("User has already been revoked."))
        elif status == "not_found":
            show_error(self, _("User not found."))
        elif status == "invalid_role" or status == "invalid_certification":
            show_error(self, _("You don't have the permission to revoke this user."))
        elif status == "error":
            show_error(self, _("Can not revoke this user."))

    def revoke_user(self, user_button):
        result = QuestionDialog.ask(
            self,
            _("Confirmation"),
            _('Are you sure you want to revoke user "{}" ?').format(user_button.user_name),
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
        users = job.ret
        self.users = []
        while self.layout_users.count() != 0:
            item = self.layout_users.takeAt(0)
            if item:
                w = item.widget()
                self.layout_users.removeWidget(w)
                w.setParent(None)
        current_user = self.core.device.user_id
        for user, (user_info, user_devices) in users.items():
            self.add_user(
                str(user_info.user_id),
                is_current_user=current_user == user,
                is_admin=False,
                certified_on=user_info.certified_on,
                is_revoked=all([device.revoked_on for device in user_devices]),
            )

    def on_list_error(self, job):
        pass

    def reset(self):
        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "list_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "list_error", QtToTrioJob),
            _do_list_users,
            core=self.core,
        )
