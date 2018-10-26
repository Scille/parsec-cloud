import pathlib

from PyQt5.QtCore import QCoreApplication
from PyQt5.QtWidgets import QMessageBox, QFileDialog, QAbstractItemView, QListView, QTreeView


def show_info(parent, text):
    QMessageBox.information(parent, QCoreApplication.translate("messages", "Information"), text)


def show_warning(parent, text):
    QMessageBox.warning(parent, QCoreApplication.translate("messages", "Warning"), text)


def show_error(parent, text):
    QMessageBox.critical(parent, QCoreApplication.translate("messages", "Error"), text)


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
