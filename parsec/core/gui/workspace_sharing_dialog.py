# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import QTimer, Qt, QCoreApplication, pyqtSignal
from PyQt5.QtWidgets import QDialog, QCompleter, QWidget

from parsec.core.fs import FSError
from parsec.types import UserID
from parsec.core.types import WorkspaceRole

from parsec.core.gui.custom_widgets import show_error, show_warning, ask_question, show_info
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


class SharingWidget(QWidget, Ui_SharingWidget):
    delete_clicked = pyqtSignal(UserID)

    def __init__(self, user, is_current_user, current_user_role, role, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.ROLES_TRANSLATIONS = {
            WorkspaceRole.READER: _("Reader"),
            WorkspaceRole.CONTRIBUTOR: _("Contributor"),
            WorkspaceRole.MANAGER: _("Manager"),
            WorkspaceRole.OWNER: _("Owner"),
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
                if current_index <= index:
                    break
                self.combo_role.insertItem(index, self.ROLES_TRANSLATIONS[role])

        self.combo_role.setCurrentIndex(_ROLES_TO_INDEX[self.role])
        self.button_delete.clicked.connect(self.on_delete_clicked)

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
        self.timer.timeout.connect(self.show_auto_complete)
        self.button_close.clicked.connect(self.on_close_requested)
        self.button_share.clicked.connect(self.on_share_clicked)
        self.button_apply.clicked.connect(self.on_update_permissions_clicked)
        for role, index in _ROLES_TO_INDEX.items():
            if role == WorkspaceRole.READER:
                self.combo_role.insertItem(index, _("Reader"))
            elif role == WorkspaceRole.CONTRIBUTOR:
                self.combo_role.insertItem(index, _("Contributor"))
            elif role == WorkspaceRole.MANAGER:
                self.combo_role.insertItem(index, _("Manager"))
        ws_entry = self.workspace_fs.get_workspace_entry()
        self.current_user_role = ws_entry.role
        if (
            self.current_user_role == WorkspaceRole.READER
            or self.current_user_role == WorkspaceRole.CONTRIBUTOR
        ):
            self.widget_add.hide()
            self.button_apply.hide()
        self.reset()

    def text_changed(self, text):
        # In order to avoid a segfault by making to many requests,
        # we wait a little bit after the user has stopped pressing keys
        # to make the query.
        if len(text):
            self.timer.start(500)

    def show_auto_complete(self):
        self.timer.stop()
        if len(self.line_edit_share.text()):
            users = self.jobs_ctx.run(
                self.core.fs.backend_cmds.user_find, self.line_edit_share.text(), 1, 100, True
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
            show_warning(self, _("You can not share a workspace with yourself.").format(user_name))
            return
        for i in range(self.scroll_content.layout().count()):
            item = self.scroll_content.layout().itemAt(i)
            if item and item.widget() and item.widget().user == user_name:
                show_warning(
                    self, _('This workspace is already shared with "{}".').format(user_name)
                )
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
        except FSError:
            show_warning(
                self,
                _('Can not share the workspace "{}" with this user.').format(
                    self.workspace_fs.workspace_name
                ),
            )
        except:
            import traceback

            traceback.print_exc()
            show_error(
                self,
                _('Can not share the workspace "{}" with "{}".').format(
                    self.workspace_fs.workspace_name, user
                ),
            )

    def add_participant(self, user, is_current_user, role):
        w = SharingWidget(
            user=user,
            is_current_user=is_current_user,
            current_user_role=self.current_user_role,
            role=role,
            parent=self,
        )
        self.scroll_content.layout().insertWidget(0, w)
        w.delete_clicked.connect(self.on_remove_user_clicked)

    def on_remove_user_clicked(self, user):
        r = ask_question(
            parent=self,
            title=_("Remove this user"),
            message=_("Are you sure you want to stop sharing this workspace with {}?").format(user),
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
                    import traceback

                    traceback.print_exc()
        if errors:
            show_error(
                self,
                _(
                    "Permissions could not be updated for the following users: {}".format(
                        "\n".join(errors)
                    )
                ),
            )
        elif updated:
            show_info(self, _("Permissions have been updated."))
        self.reset()

    def has_changes(self):
        for i in range(self.scroll_content.layout().count() - 1):
            item = self.scroll_content.layout().itemAt(i)
            w = item.widget()
            if w.should_update():
                return True
        return False

    def on_close_requested(self):
        if self.has_changes():
            r = ask_question(
                parent=self,
                title=_("Are you sure?"),
                message=_(
                    "You have made some modifications, but have not applied them by clicking "
                    '"Apply". Are you sure you want to close this window and discard these '
                    "modifications?"
                ),
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
