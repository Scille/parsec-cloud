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


class MessageDialog(QDialog, Ui_MessageDialog):
    def __init__(self, icon, title, message, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.label_title.setText(title)
        self.label_message.setText(message)
        self.label_icon.setPixmap(icon)
        self.button_close.clicked.connect(self.close)
        self.setWindowFlags(Qt.SplashScreen)


def show_info(parent, text):
    m = MessageDialog(
        QPixmap(":/icons/images/icons/info.png"),
        QCoreApplication.translate("Message", "Information"),
        text,
    )
    return m.exec_()


def show_warning(parent, text):
    m = MessageDialog(
        QPixmap(":/icons/images/icons/warning.png"),
        QCoreApplication.translate("Message", "Warning"),
        text,
    )
    return m.exec_()


def show_error(parent, text):
    m = MessageDialog(
        QPixmap(":/icons/images/icons/error.png"),
        QCoreApplication.translate("Message", "Error"),
        text,
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
