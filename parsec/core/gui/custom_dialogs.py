# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pathlib

from PyQt5.QtCore import Qt, QTimer
from PyQt5.QtGui import QPixmap
from PyQt5.QtWidgets import (
    QCompleter,
    QDialog,
    QTreeView,
    QAbstractItemView,
    QFileDialog,
    QListView,
)

import trio

from parsec.core.gui.lang import translate as _
from parsec.core.gui import desktop

from parsec.core.gui.ui.message_dialog import Ui_MessageDialog
from parsec.core.gui.ui.input_dialog import Ui_InputDialog
from parsec.core.gui.ui.question_dialog import Ui_QuestionDialog
from parsec.core.gui.ui.misc_dialog import Ui_MiscDialog


class MiscDialog(QDialog, Ui_MiscDialog):
    def __init__(self, title, main_widget, parent):
        super().__init__(parent=parent)
        self.setupUi(self)
        if not title:
            self.label_title.hide()
        else:
            self.label_title.setText(title)
        self.setWindowFlags(Qt.SplashScreen)
        self.resize(main_widget.sizeHint())
        self.layout.insertWidget(0, main_widget)
        self.button_close.clicked.connect(self.accept)


class TextInputDialog(QDialog, Ui_InputDialog):
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

    @classmethod
    def get_text(cls, parent, title, message, placeholder="", default_text="", completion=None):

        m = cls(
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

    @property
    def text(self):
        return self.line_edit_text.text()


# TODO: If this ever gets used again, it needs to transition to the new job system
class UserInputDialog(QDialog, Ui_InputDialog):
    def __init__(self, trio_token, core, title, message, exclude=None, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.core = core
        self._trio_token = trio_token
        self.label_title.setText(title)
        self.label_message.setText(message)
        self.line_edit_text.setPlaceholderText(_("LABEL_USER_NAME"))
        self.setWindowFlags(Qt.SplashScreen)
        self.exclude = exclude or []
        self.timer = QTimer()
        self.timer.timeout.connect(self.show_auto_complete)
        self.line_edit_text.setFocus()
        self.line_edit_text.textChanged.connect(self.text_changed)

    def text_changed(self, text):
        # In order to avoid a segfault by making too many requests,
        # we wait a little bit after the user has stopped pressing keys
        # to make the query.
        if len(text):
            self.timer.start(500)

    def show_auto_complete(self):
        self.timer.stop()
        if len(self.line_edit_text.text()):
            rep = trio.from_thread.run(
                self.core.backend_cmds.user_find,
                self.line_edit_text.text(),
                trio_token=self._trio_token,
            )
            if rep["status"] == "ok":
                users = rep["results"]
            else:
                users = []
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


def get_user_name(parent, trio_token, core, title, message, exclude=None):
    m = UserInputDialog(
        core=core,
        trio_token=trio_token,
        title=title,
        message=message,
        exclude=exclude,
        parent=parent,
    )
    status = m.exec_()
    if status == QDialog.Accepted:
        return m.text
    return None


class QuestionDialog(QDialog, Ui_QuestionDialog):
    def __init__(self, title, message, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.label_title.setText(title)
        self.label_message.setText(message)
        self.setWindowFlags(Qt.SplashScreen)
        message_size = self.label_message.sizeHint()
        self.resize(self.sizeHint().width(), message_size.height() * 2 + 85)

    @classmethod
    def ask(cls, parent, title, message):
        m = cls(title=title, message=message, parent=parent)
        status = m.exec_()
        if status == QDialog.Accepted:
            return True
        return False


class MessageDialog(QDialog, Ui_MessageDialog):
    def __init__(self, icon, title, message, exception=None, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.label_title.setText(title)
        self.label_message.setText(message)
        self.label_icon.setPixmap(icon)
        self.setWindowFlags(Qt.SplashScreen)
        self.text_details.hide()
        if not exception:
            self.button_details.hide()
        else:
            import traceback

            stack = traceback.format_tb(exception.__traceback__)
            if not stack:
                self.button_details.hide()
            else:
                except_text = "<b>{}</b><br /><br />{}".format(
                    str(exception).replace("\n", "<br />"), "<br />".join(stack)
                )
                except_text = except_text.replace("\n", "<br />")
                self.text_details.setHtml(except_text)
        self.button_details.clicked.connect(self.show_details)
        self.button_copy.clicked.connect(self.copy_to_clipboard)
        self.button_copy.hide()
        message_size = self.label_message.sizeHint()
        self.resize(435, message_size.height() * 2 + 100)

    def copy_to_clipboard(self):
        desktop.copy_to_clipboard(self.text_details.toPlainText())

    def show_details(self):
        if self.text_details.isVisible():
            self.text_details.hide()
            self.button_copy.hide()
            message_size = self.label_message.sizeHint()
            self.resize(435, message_size.height() * 2 + 100)
        else:
            self.text_details.show()
            self.button_copy.show()


def show_info(parent, text, exception=None):
    m = MessageDialog(
        QPixmap(":/icons/images/icons/info.png"),
        _("INFO_DIALOG_TITLE"),
        text,
        exception=exception,
        parent=parent,
    )
    m.button_continue.setText(_("BUTTON_CONTINUE"))
    return m.exec_()


def show_warning(parent, text, exception=None):
    m = MessageDialog(
        QPixmap(":/icons/images/icons/warning.png"),
        _("WARN_DIALOG_TITLE"),
        text,
        exception=exception,
        parent=parent,
    )
    m.button_continue.setText(_("BUTTON_CLOSE"))
    return m.exec_()


def show_error(parent, text, exception=None):
    m = MessageDialog(
        QPixmap(":/icons/images/icons/error.png"),
        _("ERR_DIALOG_TITLE"),
        text,
        exception=exception,
        parent=parent,
    )
    m.button_continue.setText(_("BUTTON_CLOSE"))
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
