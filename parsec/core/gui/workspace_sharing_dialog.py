# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import QTimer, Qt, QCoreApplication, pyqtSignal
from PyQt5.QtWidgets import QDialog, QCompleter, QWidget

from parsec.core.fs import FSError
from parsec.types import UserID
from parsec.core.types import WorkspaceRole

from parsec.core.gui.custom_dialogs import show_info, show_warning, show_error, QuestionDialog
from parsec.core.gui.lang import translate as _
from parsec.core.gui.trio_thread import ThreadSafeQtSignal, QtToTrioJob
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


async def _do_init(instance):
    return await instance.workspace_fs.get_workspace_entry()


async def _do_popup(instance, user, exc, fs_error):
    workspace_name = instance.workspace_fs.workspace_name
    if fs_error:
        show_error(
            instance,
            _("ERR_WORKSPACE_CAN_NOT_SHARE_{}").format(workspace_name, user),
            exception=exc,
        )
    else:
        show_error(
            instance,
            _("ERR_WORKSPACE_CAN_NOT_SHARE_{}").format(workspace_name, user),
            execption=exc,
        )


class SharingWidget(QWidget, Ui_SharingWidget):
    delete_clicked = pyqtSignal(UserID)
    role_changed = pyqtSignal()

    def __init__(self, user, is_current_user, current_user_role, role, *args, **kwargs):
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
        self.user = user
        if self.role == WorkspaceRole.OWNER:
            self.label_name.setText(f"<b>{self.user}</b>")
        else:
            self.label_name.setText(self.user)

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
            self.setDisabled(True)

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
        self.init_success = pyqtSignal(QtToTrioJob)
        self.init_error = pyqtSignal(QtToTrioJob)
        self.popup_success = pyqtSignal(QtToTrioJob)
        self.popup_error = pyqtSignal(QtToTrioJob)
        self.timer.timeout.connect(self.show_auto_complete)
        self.button_close.clicked.connect(self.on_close_requested)
        self.button_share.clicked.connect(self.on_share_clicked)
        self.button_apply.clicked.connect(self.on_update_permissions_clicked)
        ws_entry = self.workspace_fs.get_workspace_entry()
        self.current_user_role = ws_entry.role
        current_index = _ROLES_TO_INDEX[self.current_user_role]
        for role, index in _ROLES_TO_INDEX.items():
            if index <= current_index:
                if role == WorkspaceRole.READER:
                    self.combo_role.insertItem(index, _("WORKSPACE_ROLE_READER"))
                elif role == WorkspaceRole.CONTRIBUTOR:
                    self.combo_role.insertItem(index, _("WORKSPACE_ROLE_CONTRIBUTOR"))
                elif role == WorkspaceRole.MANAGER:
                    self.combo_role.insertItem(index, _("WORKSPACE_ROLE_MANAGER"))
                elif role == WorkspaceRole.OWNER:
                    self.combo_role.insertItem(index, _("WORKSPACE_ROLE_OWNER"))
        if (
            self.current_user_role == WorkspaceRole.READER
            or self.current_user_role == WorkspaceRole.CONTRIBUTOR
        ):
            self.widget_add.hide()
            self.button_apply.hide()
        self.init_success.connect(self.on_init_success)
        self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "init_success", QtToTrioJob),
            ThreadSafeQtSignal(self, "init_error", QtToTrioJob),
            _do_init,
            self,
        )
        self.reset()

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
            users = self.jobs_ctx.run(
                self.core.backend_cmds.user_find, self.line_edit_share.text(), 1, 100, True
            )
            users = [u for u in users if u != self.core.device.user_id]
            completer = QCompleter(users)
            completer.setCaseSensitivity(Qt.CaseInsensitive)
            completer.popup().setStyleSheet("border: 1px solid rgb(30, 78, 162);")
            self.line_edit_share.setCompleter(completer)
            self.line_edit_share.completer().complete()

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
        try:
            user = UserID(user_name)
            self.jobs_ctx.run(
                self.user_fs.workspace_share,
                self.workspace_fs.workspace_id,
                user,
                _index_to_role(self.combo_role.currentIndex()),
            )
            self.add_participant(user, False, _index_to_role(self.combo_role.currentIndex()))
        except FSError as exc:
            self.jobs_ctx.submit_job(
                ThreadSafeQtSignal(self, "reload_workspace_name_success", QtToTrioJob),
                ThreadSafeQtSignal(self, "reload_workspace_name_error", QtToTrioJob),
                _do_popup,
                self,
                user,
                exc,
                True,
            )
        except Exception as exc:
            self.jobs_ctx.submit_job(
                ThreadSafeQtSignal(self, "reload_workspace_name_success", QtToTrioJob),
                ThreadSafeQtSignal(self, "reload_workspace_name_error", QtToTrioJob),
                _do_popup,
                self,
                user,
                exc,
                False,
            )

    def add_participant(self, user, is_current_user, role):
        w = SharingWidget(
            user=user,
            is_current_user=is_current_user,
            current_user_role=self.current_user_role,
            role=role,
            parent=self,
        )
        w.role_changed.connect(self.on_role_changed)
        self.scroll_content.layout().insertWidget(0, w)
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

        self.jobs_ctx.run(self.user_fs.workspace_share, self.workspace_fs.workspace_id, user, None)
        self.reset()

    def on_update_permissions_clicked(self):
        errors = []
        updated = False
        for i in range(self.scroll_content.layout().count()):
            item = self.scroll_content.layout().itemAt(i)
            w = item.widget()
            if not w or not isinstance(w, SharingWidget):
                continue
            if w.should_update():
                try:
                    self.jobs_ctx.run(
                        self.user_fs.workspace_share,
                        self.workspace_fs.workspace_id,
                        w.user,
                        w.new_role,
                    )
                    updated = True
                except:
                    errors.append(w.user)
        if errors:
            show_error(self, _("ERR_WORKSPACE_ROLE_UPDATE_ERROR_{}".format("\n".join(errors))))
        elif updated:
            show_info(self, _("INFO_WORKSPACE_ROLE_UPDATE_SUCCESS"))
        self.reset()

    def on_init_success(self, job):
        ws_entry = job.ret
        self.current_user_role = ws_entry.role
        current_index = _ROLES_TO_INDEX[self.current_user_role]
        for role, index in _ROLES_TO_INDEX.items():
            if index <= current_index:
                if role == WorkspaceRole.READER:
                    self.combo_role.insertItem(index, _("Reader"))
                elif role == WorkspaceRole.CONTRIBUTOR:
                    self.combo_role.insertItem(index, _("Contributor"))
                elif role == WorkspaceRole.MANAGER:
                    self.combo_role.insertItem(index, _("Manager"))
                elif role == WorkspaceRole.OWNER:
                    self.combo_role.insertItem(index, _("Owner"))
        if (
            self.current_user_role == WorkspaceRole.READER
            or self.current_user_role == WorkspaceRole.CONTRIBUTOR
        ):
            self.widget_add.hide()
            self.button_apply.hide()

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

    def reset(self):
        while self.scroll_content.layout().count() > 1:
            item = self.scroll_content.layout().takeAt(0)
            w = item.widget()
            self.scroll_content.layout().removeItem(item)
            w.setParent(None)
        QCoreApplication.processEvents()
        participants = self.jobs_ctx.run(self.workspace_fs.get_user_roles)
        for user, role in participants.items():
            self.add_participant(user, user == self.core.device.user_id, role)
        self.line_edit_share.setText("")
        self.button_share.setDisabled(True)
        self.button_apply.setDisabled(True)
