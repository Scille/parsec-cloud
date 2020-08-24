# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from collections import namedtuple

from PyQt5.QtCore import QCoreApplication, pyqtSignal, QEvent
from PyQt5.QtWidgets import QWidget, QComboBox

from parsec.core.types import UserInfo
from parsec.core.fs import FSError, FSBackendOfflineError
from parsec.core.types import WorkspaceRole
from parsec.core.backend_connection import BackendNotAvailable

from parsec.core.gui.trio_thread import JobResultError, ThreadSafeQtSignal, QtToTrioJob

from parsec.core.gui.custom_dialogs import show_error, GreyedDialog
from parsec.core.gui.lang import translate as _
from parsec.core.gui.ui.workspace_sharing_widget import Ui_WorkspaceSharingWidget
from parsec.core.gui.ui.sharing_widget import Ui_SharingWidget


NOT_SHARED_KEY = "NOT_SHARED"

_ROLES_TO_INDEX = {
    NOT_SHARED_KEY: 0,
    WorkspaceRole.READER: 1,
    WorkspaceRole.CONTRIBUTOR: 2,
    WorkspaceRole.MANAGER: 3,
    WorkspaceRole.OWNER: 4,
}


def _index_to_role(index):
    for role, idx in _ROLES_TO_INDEX.items():
        if index == idx:
            return role
    return None


async def _do_get_users(core, workspace_fs):
    ret = {}
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
    except BackendNotAvailable as exc:
        raise JobResultError("offline") from exc


async def _do_share_workspace(user_fs, workspace_fs, user_roles):
    errors = []
    successes = []
    workspace_name = workspace_fs.get_workspace_name()

    SharingResult = namedtuple("SharingResult", ("user_info", "status", "role", "error"))

    for user_info, role in user_roles.items():
        try:
            await user_fs.workspace_share(workspace_fs.workspace_id, user_info.user_id, role)
            successes.append(SharingResult(user_info, True, role, None))
        except ValueError:
            errors.append(SharingResult(user_info, False, role, "invalid-user"))
        except FSBackendOfflineError:
            errors.append(SharingResult(user_info, False, role, "offline"))
        except FSError:
            errors.append(SharingResult(user_info, False, role, "fs-error"))
        except Exception:
            errors.append(SharingResult(user_info, False, role, "error"))
    return workspace_name, successes, errors


class SharingWidget(QWidget, Ui_SharingWidget):
    role_changed = pyqtSignal(UserInfo, object)

    def __init__(self, user_info, is_current_user, current_user_role, role, enabled):
        super().__init__()
        self.setupUi(self)

        self.combo_role.installEventFilter(self)

        self.ROLES_TRANSLATIONS = {
            WorkspaceRole.READER: _("TEXT_WORKSPACE_ROLE_READER"),
            WorkspaceRole.CONTRIBUTOR: _("TEXT_WORKSPACE_ROLE_CONTRIBUTOR"),
            WorkspaceRole.MANAGER: _("TEXT_WORKSPACE_ROLE_MANAGER"),
            WorkspaceRole.OWNER: _("TEXT_WORKSPACE_ROLE_OWNER"),
            NOT_SHARED_KEY: _("TEXT_WORKSPACE_ROLE_NOT_SHARED"),
        }
        self.role = role
        self.current_user_role = current_user_role
        self.is_current_user = is_current_user
        self.user_info = user_info
        if self.is_current_user:
            self.label_name.setText(f"<b>{self.user_info.short_user_display}</b>")
        else:
            self.label_name.setText(self.user_info.short_user_display)

        if self.user_info.is_revoked:
            self.setDisabled(True)
            font = self.label_name.font()
            font.setStrikeOut(True)
            self.label_name.setFont(font)
            self.setToolTip(_("TEXT_WORKSPACE_SHARING_USER_HAS_BEEN_REVOKED"))

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
        self.combo_role.currentIndexChanged.connect(self.on_role_changed)

    def on_role_changed(self, index):
        self.role_changed.emit(self.user_info, _index_to_role(index))

    def eventFilter(self, obj_src, event):
        if event.type() == QEvent.Wheel and isinstance(obj_src, QComboBox):
            event.ignore()
            return True
        return super().eventFilter(obj_src, event)


class WorkspaceSharingWidget(QWidget, Ui_WorkspaceSharingWidget):
    get_users_success = pyqtSignal(QtToTrioJob)
    get_users_error = pyqtSignal(QtToTrioJob)
    share_success = pyqtSignal(QtToTrioJob)
    share_error = pyqtSignal(QtToTrioJob)

    def __init__(self, user_fs, workspace_fs, core, jobs_ctx):
        super().__init__()
        self.setupUi(self)
        self.user_fs = user_fs
        self.core = core
        self.jobs_ctx = jobs_ctx
        self.workspace_fs = workspace_fs

        self.share_success.connect(self._on_share_success)
        self.share_error.connect(self._on_share_error)
        self.get_users_success.connect(self._on_get_users_success)
        self.get_users_error.connect(self._on_get_users_error)
        self.line_edit_filter.textChanged.connect(self._on_filter_changed)

        ws_entry = self.jobs_ctx.run_sync(self.workspace_fs.get_workspace_entry)
        self.current_user_role = ws_entry.role
        self.reset()

    def _on_filter_changed(self, text):
        text = text.lower()
        for i in range(self.scroll_content.layout().count()):
            w = self.scroll_content.layout().itemAt(i).widget()
            if w:
                if text in w.user_info.short_user_display.lower():
                    w.setVisible(True)
                else:
                    w.setVisible(False)

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
        self.scroll_content.layout().insertWidget(self.scroll_content.layout().count() - 1, w)

    def on_role_changed(self, user_info, role):
        if role == NOT_SHARED_KEY:
            role = None
        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "share_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "share_error", QtToTrioJob),
            _do_share_workspace,
            user_fs=self.user_fs,
            workspace_fs=self.workspace_fs,
            user_roles={user_info: role},
        )

    def _on_share_success(self, job):
        workspace_name, successes, errors = job.ret
        if errors:
            self._process_sharing_errors(workspace_name, errors)

    def _process_sharing_errors(self, workspace_name, errors):
        reset = True
        if errors:
            result = errors[0]
            if result.error == "offline":
                show_error(self, _("TEXT_WORKSPACE_SHARING_OFFLINE"))
                reset = False
            elif result.role == NOT_SHARED_KEY:
                show_error(
                    self,
                    _("TEXT_WORKSPACE_SHARING_UNSHARE_ERROR_workspace-user").format(
                        workspace=workspace_name, user=result.user_info.short_user_display
                    ),
                )
            else:
                show_error(
                    self,
                    _("TEXT_WORKSPACE_SHARING_SHARE_ERROR_workspace-user").format(
                        workspace=workspace_name, user=result.user_info.short_user_display
                    ),
                )
        if reset:
            self.reset()

    def _on_share_error(self, job):
        self.reset()

    def _on_get_users_success(self, job):
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

    def _on_get_users_error(self, job):
        assert job.is_finished()
        assert job.status != "ok"

        if job.status == "offline":
            show_error(self, _("TEXT_WORKSPACE_SHARING_OFFLINE"))
        self.spinner.hide()
        self.widget_users.show()

    def reset(self):
        self.spinner.show()
        self.widget_users.hide()
        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "get_users_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "get_users_error", QtToTrioJob),
            _do_get_users,
            core=self.core,
            workspace_fs=self.workspace_fs,
        )

    @classmethod
    def show_modal(cls, user_fs, workspace_fs, core, jobs_ctx, parent, on_finished):
        w = cls(user_fs=user_fs, workspace_fs=workspace_fs, core=core, jobs_ctx=jobs_ctx)
        d = GreyedDialog(w, title=_("TEXT_WORKSPACE_SHARING_TITLE"), parent=parent, width=1000)

        d.finished.connect(on_finished)
        # Unlike exec_, show is asynchronous and works within the main Qt loop
        d.show()
        return w
