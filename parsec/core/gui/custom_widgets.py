import pathlib

from PyQt5.QtCore import QCoreApplication, Qt, QTimer
from PyQt5.QtGui import QPixmap
from PyQt5.QtWidgets import (
    QMessageBox,
    QFileDialog,
    QAbstractItemView,
    QListView,
    QTreeView,
    QDialog,
    QCompleter,
)

from parsec.core.gui.ui.message_dialog import Ui_MessageDialog
from parsec.core.gui.ui.input_dialog import Ui_InputDialog
from parsec.core.gui.ui.question_dialog import Ui_QuestionDialog


def get_text(parent, title, message, placeholder="", default_text="", completion=None):
    class InputDialog(QDialog, Ui_InputDialog):
        def __init__(
            self, title, message, placeholder="", default_text="", completion=None, *args, **kwargs
        ):
            super().__init__(*args, **kwargs)
            self.setupUi(self)
            self.label_title.setText(title)
            self.label_message.setText(message)
            self.line_edit_text.setPlaceholderText(placeholder)
            self.setWindowFlags(Qt.SplashScreen)
            self.line_edit_text.setFocus()
            self.line_edit_text.setText(default_text)
            if completion:
                completer = QCompleter(completion)
                completer.setCaseSensitivity(Qt.CaseInsensitive)
                completer.popup().setStyleSheet("border: 1px solid rgb(30, 78, 162);")
                self.line_edit_text.setCompleter(completer)

        @property
        def text(self):
            return self.line_edit_text.text()

    m = InputDialog(
        title=title,
        message=message,
        placeholder=placeholder,
        parent=parent,
        default_text=default_text,
        completion=completion,
    )
    status = m.exec_()
    if status == QDialog.Accepted:
        return m.text
    return None


def get_user_name(parent, portal, core, title, message, exclude=None):
    class InputDialog(QDialog, Ui_InputDialog):
        def __init__(self, portal, core, title, message, exclude=None, *args, **kwargs):
            super().__init__(*args, **kwargs)
            self.setupUi(self)
            self.core = core
            self.portal = portal
            self.label_title.setText(title)
            self.label_message.setText(message)
            self.line_edit_text.setPlaceholderText(
                QCoreApplication.translate("GetUserName", "User name")
            )
            self.setWindowFlags(Qt.SplashScreen)
            self.exclude = exclude or []
            self.timer = QTimer()
            self.timer.timeout.connect(self.show_auto_complete)
            self.line_edit_text.setFocus()
            self.line_edit_text.textChanged.connect(self.text_changed)

        def text_changed(self, text):
            # In order to avoid a segfault by making to many requests,
            # we wait a little bit after the user has stopped pressing keys
            # to make the query.
            if len(text):
                self.timer.start(500)

        def show_auto_complete(self):
            self.timer.stop()
            if len(self.line_edit_text.text()):
                users = self.portal.run(
                    self.core.fs.backend_cmds.user_find, self.line_edit_text.text()
                )
                if self.exclude:
                    users = [u for u in users if u not in self.exclude]
                completer = QCompleter(users)
                completer.setCaseSensitivity(Qt.CaseInsensitive)
                completer.popup().setStyleSheet("border: 1px solid rgb(30, 78, 162);")
                self.line_edit_text.setCompleter(completer)
                self.line_edit_text.completer().complete()

        @property
        def text(self):
            return self.line_edit_text.text()

    m = InputDialog(
        core=core, portal=portal, title=title, message=message, exclude=exclude, parent=parent
    )
    status = m.exec_()
    if status == QDialog.Accepted:
        return m.text
    return None


def ask_question(parent, title, message):
    class QuestionDialog(QDialog, Ui_QuestionDialog):
        def __init__(self, title, message, *args, **kwargs):
            super().__init__(*args, **kwargs)
            self.setupUi(self)
            self.label_title.setText(title)
            self.label_message.setText(message)
            self.setWindowFlags(Qt.SplashScreen)

    m = QuestionDialog(title=title, message=message, parent=parent)
    status = m.exec_()
    if status == QDialog.Accepted:
        return True
    return False


class MessageDialog(QDialog, Ui_MessageDialog):
    def __init__(self, icon, title, message, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.label_title.setText(title)
        self.label_message.setText(message)
        self.label_icon.setPixmap(icon)
        self.setWindowFlags(Qt.SplashScreen)


def show_info(parent, text):
    m = MessageDialog(
        QPixmap(":/icons/images/icons/info.png"),
        QCoreApplication.translate("Message", "Information"),
        text,
        parent=parent,
    )
    return m.exec_()


def show_warning(parent, text):
    m = MessageDialog(
        QPixmap(":/icons/images/icons/warning.png"),
        QCoreApplication.translate("Message", "Warning"),
        text,
        parent=parent,
    )
    return m.exec_()


def show_error(parent, text):
    m = MessageDialog(
        QPixmap(":/icons/images/icons/error.png"),
        QCoreApplication.translate("Message", "Error"),
        text,
        parent=parent,
    )
    return m.exec_()


def get_open_files(parent):
    class FileDialog(QFileDialog):
        def __init__(self, parent):
            super().__init__(
                parent=parent, caption="Select files", directory=str(pathlib.Path.home())
            )
            self.setFileMode(QFileDialog.AnyFile)
            self.setOption(QFileDialog.DontUseNativeDialog, True)
            list_view = self.findChild(QListView, "listView")
            if list_view:
                list_view.setSelectionMode(QAbstractItemView.MultiSelection)
            tree_view = self.findChild(QTreeView)
            if tree_view:
                tree_view.setSelectionMode(QAbstractItemView.MultiSelection)

    file_dialog = FileDialog(parent=parent)
    result = file_dialog.exec_()
    return bool(result), file_dialog.selectedFiles()
