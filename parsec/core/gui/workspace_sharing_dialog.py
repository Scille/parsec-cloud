# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os

from PyQt5.QtCore import QTimer, Qt, QCoreApplication, pyqtSignal
from PyQt5.QtWidgets import QDialog, QCompleter, QWidget

from parsec.core.fs.sharing import SharingRecipientError

from parsec.core.gui.custom_widgets import show_error, show_warning, ask_question

from parsec.core.gui.ui.workspace_sharing_dialog import Ui_WorkspaceSharingDialog
from parsec.core.gui.ui.sharing_widget import Ui_SharingWidget


class SharingWidget(QWidget, Ui_SharingWidget):
    remove_clicked = pyqtSignal(QWidget)

    def __init__(self, name, is_current_user, admin, read, write, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.label_name.setText(name)
        self.checkbox_read.setChecked(read)
        self.checkbox_write.setChecked(write)
        self.checkbox_admin.setChecked(admin)
        self.is_current_user = is_current_user
        if is_current_user:
            self.checkbox_read.hide()
            self.checkbox_write.hide()
            self.checkbox_admin.hide()
        self.read_start_state = read
        self.write_start_state = write
        self.admin_start_state = admin

    @property
    def admin_rights(self):
        return self.checkbox_admin.isChecked()

    @property
    def read_rights(self):
        return self.checkbox_read.isChecked()

    @property
    def write_rights(self):
        return self.checkbox_write.isChecked()

    @property
    def name(self):
        return self.label_name.text()

    def should_update(self):
        return (
            self.admin_rights != self.admin_start_state
            or self.write_rights != self.write_start_state
            or self.read_rights != self.read_start_state
        )


class WorkspaceSharingDialog(QDialog, Ui_WorkspaceSharingDialog):
    def __init__(self, name, core, portal, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.core = core
        self.portal = portal
        self.name = name
        self.setWindowFlags(Qt.SplashScreen)
        self.line_edit_share.textChanged.connect(self.text_changed)
        self.timer = QTimer()
        self.timer.timeout.connect(self.show_auto_complete)
        self.button_close.clicked.connect(self.close_requested)
        self.button_share.clicked.connect(self.add_user)
        self.button_apply.clicked.connect(self.apply_changes)
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
            users = self.portal.run(
                self.core.fs.backend_cmds.user_find, self.line_edit_share.text(), 1, 100, True
            )
            users = [u for u in users if u != self.core.device.user_id]
            completer = QCompleter(users)
            completer.setCaseSensitivity(Qt.CaseInsensitive)
            completer.popup().setStyleSheet("border: 1px solid rgb(30, 78, 162);")
            self.line_edit_share.setCompleter(completer)
            self.line_edit_share.completer().complete()

    def add_user(self):
        user = self.line_edit_share.text()
        if not user:
            return
        if user == self.core.device.user_id:
            show_warning(
                self,
                QCoreApplication.translate(
                    "WorkspaceSharing", "You can not share a workspace with yourself."
                ).format(user),
            )
            return
        for i in range(self.scroll_content.layout().count()):
            item = self.scroll_content.layout().itemAt(i)
            if item and item.widget() and item.widget().name == user:
                show_warning(
                    self,
                    QCoreApplication.translate(
                        "WorkspaceSharing", 'This workspace is already shared with "{}".'
                    ).format(user),
                )
                return
        try:
            self.portal.run(
                self.core.fs.share,
                os.path.join("/", self.name),
                user,
                self.checkbox_admin.isChecked(),
                self.checkbox_read.isChecked(),
                self.checkbox_write.isChecked(),
            )
            self.add_participant(user, False)
            self.checkbox_admin.setChecked(True)
            self.checkbox_read.setChecked(True)
            self.checkbox_write.setChecked(True)
        except SharingRecipientError:
            show_warning(
                self,
                QCoreApplication.translate(
                    "WorkspaceSharing", 'Can not share the workspace "{}" with this user.'
                ).format(self.name),
            )
        except:
            import traceback

            traceback.print_exc()
            show_error(
                self,
                QCoreApplication.translate(
                    "WorkspaceSharing", 'Can not share the workspace "{}" with "{}".'
                ).format(self.name, user),
            )

    def add_participant(self, participant, is_current_user):
        w = SharingWidget(
            name=participant,
            is_current_user=is_current_user,
            admin=True,
            read=True,
            write=True,
            parent=self,
        )
        self.scroll_content.layout().insertWidget(0, w)

    def apply_changes(self):
        for i in range(self.scroll_content.layout().count()):
            item = self.scroll_content.layout().itemAt(i)
            w = item.widget()
            if not w or not isinstance(w, SharingWidget):
                continue
            errors = []
            if not w.is_current_user and w.should_update():
                try:
                    self.portal.run(
                        self.core.fs.share,
                        os.path.join("/", self.name),
                        w.name,
                        w.admin_rights,
                        w.read_rights,
                        w.write_rights,
                    )
                except SharingRecipientError:
                    errors.append(w.name)
                    import traceback

                    traceback.print_exc()
                except:
                    errors.append(w.name)
                    import traceback

                    traceback.print_exc()
            if errors:
                show_error(
                    self,
                    QCoreApplication.translate(
                        "WorkspaceSharing",
                        "Permissions could not be updated for the following users: {}".format(
                            "\n".join(errors)
                        ),
                    ),
                )
            self.reset()

    def has_changes(self):
        for i in range(self.scroll_content.layout().count() - 1):
            item = self.scroll_content.layout().itemAt(i)
            w = item.widget()
            if not w.is_current_user and w.should_update():
                return True
        return False

    def close_requested(self):
        if self.has_changes():
            r = ask_question(
                parent=self,
                title=QCoreApplication.translate("WorkspaceSharing", "Are you sure?"),
                message=QCoreApplication.translate(
                    "WorkspaceSharing",
                    "You have made some modifications, but have not applied them by clicking "
                    '"Apply". Are you sure you want to close this window and discard these '
                    "modifications?",
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
        ws_infos = self.portal.run(self.core.fs.stat, os.path.join("/", self.name))
        for p in ws_infos["participants"]:
            self.add_participant(p, p == self.core.device.user_id)
