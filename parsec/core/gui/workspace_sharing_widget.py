# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import QTimer, Qt, QCoreApplication, pyqtSignal
from PyQt5.QtWidgets import QCompleter, QWidget

from parsec.core.types import UserInfo
from parsec.core.fs import FSError, FSBackendOfflineError
from parsec.core.types import WorkspaceRole
from parsec.core.backend_connection import BackendNotAvailable

from parsec.core.gui.trio_thread import JobResultError, ThreadSafeQtSignal, QtToTrioJob

from parsec.core.gui.custom_dialogs import show_info, show_error, ask_question, GreyedDialog
from parsec.core.gui.lang import translate as _
from parsec.core.gui.ui.workspace_sharing_widget import Ui_WorkspaceSharingWidget
from parsec.core.gui.ui.sharing_widget import Ui_SharingWidget


_ROLES_TO_INDEX = {
    WorkspaceRole.READER: 0,
    WorkspaceRole.CONTRIBUTOR: 1,
    WorkspaceRole.MANAGER: 2,
    WorkspaceRole.OWNER: 3,
}


def _index_to_role(index):
    for role, idx in _ROLES_TO_INDEX.items():
        if index == idx:
            return role
    return None


async def _do_get_participants(core, workspace_fs):
    ret = {}
    try:
        participants = await workspace_fs.get_user_roles()
    except BackendNotAvailable as exc:
        raise JobResultError("offline") from exc
    for user, role in participants.items():
        user_info = await core.get_user_info(user)
        ret[user_info.user_id] = (role, user_info)
    return ret


async def _do_user_find(core, text):
    try:
        users, total = await core.find_humans(text, omit_revoked=True)
    except BackendNotAvailable as exc:
        raise JobResultError("offline") from exc
    users = [u for u in users if u.user_id != core.device.user_id]
    return users


async def _do_share_workspace(user_fs, workspace_fs, user_info, role):
    try:
        workspace_name = workspace_fs.get_workspace_name()
        await user_fs.workspace_share(workspace_fs.workspace_id, user_info.user_id, role)
        return workspace_name, user_info, role
    except ValueError as exc:
        raise JobResultError("invalid-user", workspace_name=workspace_name, user=user_info) from exc
    except FSBackendOfflineError as exc:
        raise JobResultError("offline", workspace_name=workspace_name, user=user_info) from exc
    except FSError as exc:
        raise JobResultError("fs-error", workspace_name=workspace_name, user=user_info) from exc
    except Exception as exc:
        raise JobResultError("error", workspace_name=workspace_name, user=user_info) from exc


async def _do_share_workspace_multiple(user_fs, workspace_fs, user_roles):
    errors = {}
    successes = {}
    for user_info, role in user_roles.items():
        try:
            await user_fs.workspace_share(workspace_fs.workspace_id, user_info.user_id, role)
            successes[user_info] = role
        except Exception:
            errors[user_info] = role
    return workspace_fs.get_workspace_name(), successes, errors


class SharingWidget(QWidget, Ui_SharingWidget):
    delete_clicked = pyqtSignal(UserInfo)
    role_changed = pyqtSignal()

    def __init__(self, user_info, is_current_user, current_user_role, role, enabled):
        super().__init__()
        self.setupUi(self)
        self.ROLES_TRANSLATIONS = {
            WorkspaceRole.READER: _("TEXT_WORKSPACE_ROLE_READER"),
            WorkspaceRole.CONTRIBUTOR: _("TEXT_WORKSPACE_ROLE_CONTRIBUTOR"),
            WorkspaceRole.MANAGER: _("TEXT_WORKSPACE_ROLE_MANAGER"),
            WorkspaceRole.OWNER: _("TEXT_WORKSPACE_ROLE_OWNER"),
        }
        self.role = role
        self.current_user_role = current_user_role
        self.is_current_user = is_current_user
        self.user_info = user_info
        self.button_delete.apply_style()
        if self.role == WorkspaceRole.OWNER:
            self.label_name.setText(f"<b>{self.user_info.short_user_display}</b>")
        else:
            self.label_name.setText(self.user_info.short_user_display)

        if self.user_info.is_revoked:
            self.setDisabled(True)
            font = self.label_name.font()
            font.setStrikeOut(True)
            self.label_name.setFont(font)
            self.setToolTip(_("TEXT_WORKSPACE_SHARING_USER_IS_REVOKED"))

        if not enabled:
            for role, index in _ROLES_TO_INDEX.items():
                self.combo_role.insertItem(index, self.ROLES_TRANSLATIONS[role])
        else:
            current_index = _ROLES_TO_INDEX[self.current_user_role]
            for role, index in _ROLES_TO_INDEX.items():
                if current_index < index:
                    break
                self.combo_role.insertItem(index, self.ROLES_TRANSLATIONS[role])

        self.combo_role.setCurrentIndex(_ROLES_TO_INDEX[self.role])
        self.button_delete.clicked.connect(self.on_delete_clicked)
        self.combo_role.currentIndexChanged.connect(self.on_role_changed)

    def on_role_changed(self):
        self.role_changed.emit()

    @property
    def is_revoked(self):
        return self.user_info.is_revoked

    @property
    def new_role(self):
        return _index_to_role(self.combo_role.currentIndex())

    def on_delete_clicked(self):
        self.delete_clicked.emit(self.user_info)

    def should_update(self):
        return self.role != self.new_role


class WorkspaceSharingWidget(QWidget, Ui_WorkspaceSharingWidget):
    get_participants_success = pyqtSignal(QtToTrioJob)
    get_participants_error = pyqtSignal(QtToTrioJob)
    share_success = pyqtSignal(QtToTrioJob)
    share_error = pyqtSignal(QtToTrioJob)
    unshare_success = pyqtSignal(QtToTrioJob)
    unshare_error = pyqtSignal(QtToTrioJob)
    share_update_success = pyqtSignal(QtToTrioJob)
    share_update_error = pyqtSignal(QtToTrioJob)
    user_find_success = pyqtSignal(QtToTrioJob)
    user_find_error = pyqtSignal(QtToTrioJob)

    def __init__(self, user_fs, workspace_fs, core, jobs_ctx):
        super().__init__()
        self.setupUi(self)
        self.user_fs = user_fs
        self.core = core
        self.jobs_ctx = jobs_ctx
        self.workspace_fs = workspace_fs
        self.line_edit_share.textChanged.connect(self.text_changed)
        self.timer = QTimer()
        self.timer.timeout.connect(self.show_auto_complete)
        self.button_share.clicked.connect(self.on_share_clicked)
        self.button_apply.clicked.connect(self.on_update_permissions_clicked)

        self.last_human_find = None

        self.share_success.connect(self._on_share_success)
        self.share_error.connect(self._on_share_error)
        self.unshare_success.connect(self._on_unshare_success)
        self.unshare_error.connect(self._on_unshare_error)
        self.share_update_success.connect(self._on_share_update_success)
        self.share_update_error.connect(self._on_share_update_error)
        self.get_participants_success.connect(self._on_get_participants_success)
        self.get_participants_error.connect(self._on_get_participants_error)
        self.user_find_success.connect(self._on_user_find_success)
        self.user_find_error.connect(self._on_user_find_error)
        self.check_show_revoked.toggled.connect(self._on_show_revoked)

        ws_entry = self.jobs_ctx.run_sync(self.workspace_fs.get_workspace_entry)
        self.current_user_role = ws_entry.role

        if (
            self.current_user_role == WorkspaceRole.MANAGER
            or self.current_user_role == WorkspaceRole.OWNER
        ):
            self.combo_role.insertItem(self.combo_role.count(), _("TEXT_WORKSPACE_ROLE_READER"))
            self.combo_role.insertItem(
                self.combo_role.count(), _("TEXT_WORKSPACE_ROLE_CONTRIBUTOR")
            )
        if self.current_user_role == WorkspaceRole.OWNER:
            self.combo_role.insertItem(self.combo_role.count(), _("TEXT_WORKSPACE_ROLE_MANAGER"))
            self.combo_role.insertItem(self.combo_role.count(), _("TEXT_WORKSPACE_ROLE_OWNER"))

        if (
            self.current_user_role == WorkspaceRole.READER
            or self.current_user_role == WorkspaceRole.CONTRIBUTOR
        ):
            self.widget_add.hide()
            self.button_apply.hide()
        self.reset()

    def _on_show_revoked(self, visible):
        for i in range(self.scroll_content.layout().count()):
            w = self.scroll_content.layout().itemAt(i).widget()
            if w and w.is_revoked:
                w.setVisible(visible)

    def text_changed(self, text):
        # In order to avoid a segfault by making to many requests,
        # we wait a little bit after the user has stopped pressing keys
        # to make the query.
        if len(text):
            self.button_share.setDisabled(False)
            self.timer.start(500)
        else:
            self.button_share.setDisabled(True)

    def show_auto_complete(self):
        self.timer.stop()
        if len(self.line_edit_share.text()):
            self.jobs_ctx.submit_job(
                ThreadSafeQtSignal(self, "user_find_success", QtToTrioJob),
                ThreadSafeQtSignal(self, "user_find_error", QtToTrioJob),
                _do_user_find,
                core=self.core,
                text=self.line_edit_share.text(),
            )

    def on_share_clicked(self):
        user_name = self.line_edit_share.text()
        if not user_name:
            return
        if not self.last_human_find or user_name not in self.last_human_find:
            show_error(self, _("TEXT_WORKSPACE_SHARING_USER_NOT_FOUND"))
            return
        user_info = self.last_human_find[user_name]
        for i in range(self.scroll_content.layout().count()):
            item = self.scroll_content.layout().itemAt(i)
            if item and item.widget() and item.widget().user_info.user_id == user_info.user_id:
                show_info(
                    self,
                    _("TEXT_WORKSPACE_SHARING_ALREADY_SHARED_user").format(
                        user=str(user_info.short_user_display)
                    ),
                )
                return

        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "share_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "share_error", QtToTrioJob),
            _do_share_workspace,
            user_fs=self.user_fs,
            workspace_fs=self.workspace_fs,
            user_info=user_info,
            role=_index_to_role(self.combo_role.currentIndex()),
        )

    def add_participant(self, user_info, is_current_user, role):
        enabled = True
        if is_current_user:
            enabled = False
        elif (
            self.current_user_role == WorkspaceRole.READER
            or self.current_user_role == WorkspaceRole.CONTRIBUTOR
        ):
            enabled = False
        elif self.current_user_role == WorkspaceRole.MANAGER and (
            role == WorkspaceRole.OWNER or role == WorkspaceRole.MANAGER
        ):
            enabled = False

        w = SharingWidget(
            user_info=user_info,
            is_current_user=is_current_user,
            current_user_role=self.current_user_role,
            role=role,
            enabled=enabled,
        )
        w.role_changed.connect(self.on_role_changed)
        w.setEnabled(enabled)
        self.scroll_content.layout().insertWidget(0, w)
        if not self.check_show_revoked.isChecked() and w.is_revoked:
            w.hide()
        w.delete_clicked.connect(self.on_remove_user_clicked)

    def on_role_changed(self):
        if self.has_changes():
            self.button_apply.setDisabled(False)
        else:
            self.button_apply.setDisabled(True)

    def on_remove_user_clicked(self, user_info):
        r = ask_question(
            parent=self,
            title=_("TEXT_WORKSPACE_SHARING_UNSHARE_TITLE"),
            message=_("TEXT_WORKSPACE_SHARING_UNSHARE_INSTRUCTIONS_user").format(
                user=user_info.short_user_display
            ),
            button_texts=[_("ACTION_WORKSPACE_UNSHARE_CONFIRM"), _("ACTION_CANCEL")],
        )
        if r != _("ACTION_WORKSPACE_UNSHARE_CONFIRM"):
            return

        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "unshare_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "unshare_error", QtToTrioJob),
            _do_share_workspace,
            user_fs=self.user_fs,
            workspace_fs=self.workspace_fs,
            user_info=user_info,
            role=None,
        )

    def on_update_permissions_clicked(self):
        user_roles = {}

        for i in range(self.scroll_content.layout().count()):
            item = self.scroll_content.layout().itemAt(i)
            w = item.widget()
            if not w or not isinstance(w, SharingWidget):
                continue
            if w.should_update():
                user_roles[w.user_info] = w.new_role

        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "share_update_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "share_update_error", QtToTrioJob),
            _do_share_workspace_multiple,
            user_fs=self.user_fs,
            workspace_fs=self.workspace_fs,
            user_roles=user_roles,
        )

    def has_changes(self):
        for i in range(self.scroll_content.layout().count() - 1):
            item = self.scroll_content.layout().itemAt(i)
            w = item.widget()
            if w.should_update():
                return True
        return False

    def _on_share_success(self, job):
        workspace_name, user_info, role = job.ret
        self.line_edit_share.setText("")
        self.add_participant(user_info, is_current_user=False, role=role)

    def _on_share_error(self, job):
        exc = job.exc
        show_error(
            self,
            _("TEXT_WORKSPACE_SHARING_SHARE_ERROR_workspace-user").format(
                workspace=exc.params.get("workspace_name"),
                user=exc.params.get("user").short_user_display,
            ),
            exception=exc,
        )

    def _on_unshare_success(self, job):
        self.reset()

    def _on_unshare_error(self, job):
        exc = job.exc
        show_error(
            self,
            _("TEXT_WORKSPACE_SHARING_UNSHARE_ERROR_workspace-user").format(
                workspace=exc.params.get("workspace_name"),
                user=exc.params.get("user").short_user_display,
            ),
        )

    def _on_share_update_success(self, job):
        workspace_name, successes, errors = job.ret

        if errors:
            show_error(
                self,
                _(
                    "TEXT_WORKSPACE_SHARING_UPDATE_ROLES_ERROR_errors".format(
                        errors="\n".join([u.short_user_display for u in errors.keys()])
                    )
                ),
            )
        else:
            show_info(self, _("TEXT_WORKSPACE_SHARING_UPDATE_ROLES_SUCCESS"))
            self.reset()

    def _on_share_update_error(self, job):
        pass

    def _on_get_participants_success(self, job):
        participants = job.ret
        while self.scroll_content.layout().count() > 1:
            item = self.scroll_content.layout().takeAt(0)
            w = item.widget()
            self.scroll_content.layout().removeItem(item)
            w.setParent(None)
        QCoreApplication.processEvents()
        for (role, user_info) in participants.values():
            self.add_participant(
                user_info, is_current_user=user_info.user_id == self.core.device.user_id, role=role
            )
        self.line_edit_share.setText("")
        self.button_share.setDisabled(True)
        self.button_apply.setDisabled(True)

    def _on_get_participants_error(self, job):
        pass

    def _on_user_find_success(self, job):
        users = job.ret
        if users:
            self.last_human_find = {u.user_display: u for u in users}
            completer = QCompleter([u.user_display for u in users])
            completer.setCaseSensitivity(Qt.CaseInsensitive)
            self.line_edit_share.setCompleter(completer)
            self.line_edit_share.completer().complete()
        else:
            completer = QCompleter()
            self.line_edit_share.setCompleter(completer)
            self.line_edit_share.completer().complete()

    def _on_user_find_error(self, job):
        pass

    def reset(self):
        self.line_edit_share.setText("")
        self.button_share.setDisabled(True)
        self.button_apply.setDisabled(True)

        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "get_participants_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "get_participants_error", QtToTrioJob),
            _do_get_participants,
            core=self.core,
            workspace_fs=self.workspace_fs,
        )

    @classmethod
    def exec_modal(cls, user_fs, workspace_fs, core, jobs_ctx, parent):
        w = cls(user_fs=user_fs, workspace_fs=workspace_fs, core=core, jobs_ctx=jobs_ctx)
        d = GreyedDialog(w, title=_("TEXT_WORKSPACE_SHARING_TITLE"), parent=parent, width=1000)
        return d.exec_()
