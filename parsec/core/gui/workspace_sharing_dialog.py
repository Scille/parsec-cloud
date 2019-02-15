import os

from PyQt5.QtCore import QTimer, Qt, QCoreApplication, pyqtSignal
from PyQt5.QtWidgets import QDialog, QCompleter, QWidget

from parsec.core.fs.sharing import SharingRecipientError

from parsec.core.gui.custom_widgets import show_error, show_warning

from parsec.core.gui.ui.workspace_sharing_dialog import Ui_WorkspaceSharingDialog
from parsec.core.gui.ui.sharing_widget import Ui_SharingWidget


class SharingWidget(QWidget, Ui_SharingWidget):
    remove_clicked = pyqtSignal(QWidget)

    def __init__(self, name, read, write, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.label_name.setText(name)
        self.checkbox_read.setChecked(read)
        self.checkbox_write.setChecked(write)
        self.button_remove.clicked.connect(self.remove)

    def remove(self):
        self.remove_clicked.emit(self)


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
        self.button_close.clicked.connect(self.validate)
        self.button_share.clicked.connect(self.add_user)
        ws_infos = self.portal.run(self.core.fs.stat, os.path.join("/", self.name))
        for p in ws_infos["participants"]:
            if p != self.core.device.user_id:
                self.add_participant(p)

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
                self.core.fs.backend_cmds.user_find, self.line_edit_share.text()
            )
            users = [u for u in users if u != self.core.device.user_id]
            completer = QCompleter(users)
            completer.setCaseSensitivity(Qt.CaseInsensitive)
            completer.popup().setStyleSheet("border: 1px solid rgb(30, 78, 162);")
            self.line_edit_share.setCompleter(completer)
            self.line_edit_share.completer().complete()

    def add_user(self):
        user = self.line_edit_share.text()
        print(user)
        print(self.name)
        if not user:
            return
        try:
            self.portal.run(self.core.fs.share, os.path.join("/", self.name), user)
        except SharingRecipientError:
            show_warning(
                self,
                QCoreApplication.translate(
                    "WorkspacesWidget", 'Can not share the workspace "{}" with this user.'
                ).format(self.name),
            )
        except:
            import traceback

            traceback.print_exc()
            show_error(
                self,
                QCoreApplication.translate(
                    "WorkspacesWidget", 'Can not share the workspace "{}" with "{}".'
                ).format(self.name, user),
            )

    def add_participant(self, participant):
        w = SharingWidget(name=participant, read=True, write=True, parent=self)
        self.scroll_content.layout().insertWidget(0, w)
        w.remove_clicked.connect(self.remove_participant)

    def remove_participant(self, participant):
        self.scroll_content.layout().removeWidget(participant)
        participant.setParent(None)

    def validate(self):
        self.accept()
