# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import QTimer, Qt, QCoreApplication, pyqtSignal
from PyQt5.QtWidgets import QDialog, QCompleter, QWidget

from parsec.api.protocol import UserID
from parsec.core.fs import FSError
from parsec.core.types import WorkspaceRole

from parsec.core.gui.trio_thread import JobResultError, ThreadSafeQtSignal, QtToTrioJob

from parsec.core.gui.custom_dialogs import show_info, show_warning, show_error, QuestionDialog
from parsec.core.gui.lang import translate as _
from parsec.core.gui.ui.workspace_sharing_dialog import Ui_WorkspaceSharingDialog
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
    participants = await workspace_fs.get_user_roles()
    for user, role in participants.items():
        user_info, revoked_info = await core.remote_devices_manager.get_user(user)
        ret[user] = (role, revoked_info)
    return ret


async def _do_user_find(core, text):
    rep = await core.backend_cmds.user_find(text, 1, 100, True)
    if rep["status"] != "ok":
        raise JobResultError("error", rep=rep)
    users = [u for u in rep["results"] if u != core.device.user_id]
    return users


async def _do_share_workspace(user_fs, workspace_fs, user, role):
    user_id = UserID(user)
    workspace_name = workspace_fs.get_workspace_name()
    try:
        await user_fs.workspace_share(workspace_fs.workspace_id, user_id, role)
        return workspace_name, user_id, role
    except ValueError as exc:
        raise JobResultError("invalid-user", workspace_name=workspace_name, user=user_id) from exc
    except FSError as exc:
        raise JobResultError("fs-error", workspace_name=workspace_name, user=user_id) from exc
    except Exception as exc:
        raise JobResultError("error", workspace_name=workspace_name, user=user_id) from exc


async def _do_share_workspace_multiple(user_fs, workspace_fs, user_roles):
    errors = {}
    successes = {}
    for user, role in user_roles.items():
        try:
            user_id = UserID(user)
            await user_fs.workspace_share(workspace_fs.workspace_id, user_id, role)
            successes[user_id] = role
        except Exception:
            errors[user_id] = role
    return workspace_fs.get_workspace_name(), successes, errors


class SharingWidget(QWidget, Ui_SharingWidget):
    delete_clicked = pyqtSignal(UserID)
    role_changed = pyqtSignal()

    def __init__(self, user, is_current_user, current_user_role, role, is_revoked, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.ROLES_TRANSLATIONS = {
            WorkspaceRole.READER: _("WORKSPACE_ROLE_READER"),
            WorkspaceRole.CONTRIBUTOR: _("WORKSPACE_ROLE_CONTRIBUTOR"),
            WorkspaceRole.MANAGER: _("WORKSPACE_ROLE_MANAGER"),
            WorkspaceRole.OWNER: _("WORKSPACE_ROLE_OWNER"),
        }
        self.role = role
        self.current_user_role = current_user_role
        self.is_current_user = is_current_user
        self.is_revoked = is_revoked
        self.user = user
        if self.role == WorkspaceRole.OWNER:
            self.label_name.setText(f"<b>{self.user}</b>")
        else:
            self.label_name.setText(self.user)

        if self.is_revoked:
            self.setDisabled(True)
            font = self.label_name.font()
            font.setStrikeOut(True)
            self.label_name.setFont(font)
            self.setToolTip(_("USER_IS_REVOKED"))
        if self.is_current_user:
            self.setDisabled(True)
        if (
            self.current_user_role == WorkspaceRole.READER
            or self.current_user_role == WorkspaceRole.CONTRIBUTOR
        ):
            self.setDisabled(True)
        if self.current_user_role == WorkspaceRole.MANAGER and (
            self.role == WorkspaceRole.OWNER or self.role == WorkspaceRole.MANAGER
        ):
            self.setDisabled(True)
        if self.current_user_role == WorkspaceRole.OWNER and self.role == WorkspaceRole.OWNER:
            self.setDisabled(False)

        if not self.isEnabled():
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
    def new_role(self):
        return _index_to_role(self.combo_role.currentIndex())

    def on_delete_clicked(self):
        self.delete_clicked.emit(self.user)

    def should_update(self):
        return self.role != self.new_role


class WorkspaceSharingDialog(QDialog, Ui_WorkspaceSharingDialog):
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

    def __init__(self, user_fs, workspace_fs, core, jobs_ctx, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.user_fs = user_fs
        self.core = core
        self.jobs_ctx = jobs_ctx
        self.workspace_fs = workspace_fs
        self.setWindowFlags(Qt.SplashScreen)
        self.line_edit_share.textChanged.connect(self.text_changed)
        self.timer = QTimer()
        self.timer.timeout.connect(self.show_auto_complete)
        self.button_close.clicked.connect(self.on_close_requested)
        self.button_share.clicked.connect(self.on_share_clicked)
        self.button_apply.clicked.connect(self.on_update_permissions_clicked)

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
            self.combo_role.insertItem(self.combo_role.count(), _("WORKSPACE_ROLE_READER"))
            self.combo_role.insertItem(self.combo_role.count(), _("WORKSPACE_ROLE_CONTRIBUTOR"))
        if self.current_user_role == WorkspaceRole.OWNER:
            self.combo_role.insertItem(self.combo_role.count(), _("WORKSPACE_ROLE_MANAGER"))
            self.combo_role.insertItem(self.combo_role.count(), _("WORKSPACE_ROLE_OWNER"))

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
        if user_name == self.core.device.user_id:
            show_warning(self, _("WARN_WORKSPACE_SHARING_WITH_YOURSELF"))
            return
        for i in range(self.scroll_content.layout().count()):
            item = self.scroll_content.layout().itemAt(i)
            if item and item.widget() and item.widget().user == user_name:
                show_warning(self, _("WARN_WORKSPACE_ALREADY_SHARED_{}").format(user_name))
                return

        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "share_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "share_error", QtToTrioJob),
            _do_share_workspace,
            user_fs=self.user_fs,
            workspace_fs=self.workspace_fs,
            user=user_name,
            role=_index_to_role(self.combo_role.currentIndex()),
        )

    def add_participant(self, user, is_current_user, role, is_revoked):
        w = SharingWidget(
            user=user,
            is_current_user=is_current_user,
            current_user_role=self.current_user_role,
            role=role,
            is_revoked=is_revoked,
            parent=self,
        )
        w.role_changed.connect(self.on_role_changed)
        self.scroll_content.layout().insertWidget(0, w)
        if not self.check_show_revoked.isChecked() and w.is_revoked:
            w.hide()
        w.delete_clicked.connect(self.on_remove_user_clicked)

    def on_role_changed(self):
        if self.has_changes():
            self.button_apply.setDisabled(False)
        else:
            self.button_apply.setDisabled(True)

    def on_remove_user_clicked(self, user):
        r = QuestionDialog.ask(
            parent=self,
            title=_("ASK_WORKSPACE_UNSHARE_TITLE"),
            message=_("ASK_WORKSPACE_UNSHARE_CONTENT_{}").format(user),
        )
        if not r:
            return

        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "unshare_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "unshare_error", QtToTrioJob),
            _do_share_workspace,
            user_fs=self.user_fs,
            workspace_fs=self.workspace_fs,
            user=user,
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
                user_roles[w.user] = w.new_role

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

    def on_close_requested(self):
        if self.has_changes():
            r = QuestionDialog.ask(
                parent=self,
                title=_("ASK_WORKSPACE_SHARE_UNSAVED_MODIFICATIONS_TITLE"),
                message=_("ASK_WORKSPACE_SHARE_UNSAVED_MODIFICATIONS_CONTENT"),
            )
            if r:
                self.accept()
        else:
            self.accept()

    def _on_share_success(self, job):
        workspace_name, user, role = job.ret
        self.add_participant(user, False, role, is_revoked=False)

    def _on_share_error(self, job):
        exc = job.exc
        show_error(
            self,
            _("ERR_WORKSPACE_CANNOT_SHARE_{}").format(
                workspace=exc.params.get("workspace_name"), user=exc.params.get("user")
            ),
            exception=exc,
        )

    def _on_unshare_success(self, job):
        self.reset()

    def _on_unshare_error(self, job):
        exc = job.exc
        show_error(
            self,
            _("ERR_WORKSPACE_ROLE_UNSHARE_ERROR_{}").format(
                workspace=exc.params.get("workspace_name"), user=exc.params.get("user")
            ),
        )

    def _on_share_update_success(self, job):
        workspace_name, successes, errors = job.ret

        if errors:
            show_error(
                self, _("ERR_WORKSPACE_ROLE_UPDATE_ERROR_{}".format("\n".join(errors.keys())))
            )
        else:
            show_info(self, _("INFO_WORKSPACE_ROLE_UPDATE_SUCCESS"))
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
        for user, (role, revoked_info) in participants.items():
            self.add_participant(
                user, user == self.core.device.user_id, role, is_revoked=revoked_info is not None
            )
        self.line_edit_share.setText("")
        self.button_share.setDisabled(True)
        self.button_apply.setDisabled(True)

    def _on_get_participants_error(self, job):
        pass

    def _on_user_find_success(self, job):
        users = job.ret
        completer = QCompleter(users)
        completer.setCaseSensitivity(Qt.CaseInsensitive)
        completer.popup().setStyleSheet("border: 1px solid rgb(30, 78, 162);")
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
