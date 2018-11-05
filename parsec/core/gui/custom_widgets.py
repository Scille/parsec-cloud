import pathlib

from PyQt5.QtCore import QCoreApplication, Qt
from PyQt5.QtGui import QPixmap
from PyQt5.QtWidgets import (
    QMessageBox,
    QFileDialog,
    QAbstractItemView,
    QListView,
    QTreeView,
    QDialog,
)

from parsec.core.gui.ui.message_dialog import Ui_MessageDialog
from parsec.core.gui.ui.input_dialog import Ui_InputDialog
from parsec.core.gui.ui.question_dialog import Ui_QuestionDialog


def get_text(parent, title, message, placeholder="", default_text=""):
    class InputDialog(QDialog, Ui_InputDialog):
        def __init__(self, title, message, placeholder="", default_text="", *args, **kwargs):
            super().__init__(*args, **kwargs)
            self.setupUi(self)
            self.label_title.setText(title)
            self.label_message.setText(message)
            self.line_edit_text.setPlaceholderText(placeholder)
            self.setWindowFlags(Qt.SplashScreen)
            self.line_edit_text.setFocus()
            self.line_edit_text.setText(default_text)

        @property
        def text(self):
            return self.line_edit_text.text()

    m = InputDialog(
        title=title,
        message=message,
        placeholder=placeholder,
        parent=parent,
        default_text=default_text,
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
            l = self.findChild(QListView, "listView")
            if l:
                l.setSelectionMode(QAbstractItemView.MultiSelection)
            t = self.findChild(QTreeView)
            if t:
                t.setSelectionMode(QAbstractItemView.MultiSelection)

    f = FileDialog(parent=parent)
    result = f.exec_()
    return bool(result), f.selectedFiles()
