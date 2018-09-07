from PyQt5.QtCore import QFileInfo, QUrl, Qt, QSize
from PyQt5.QtGui import QDesktopServices, QIcon
from PyQt5.QtWidgets import QWidget, QListWidgetItem, QLabel, QGridLayout

from parsec.core.gui.custom_widgets import ToolButton
from parsec.core.gui.ui.parent_folder_widget import Ui_ParentFolderWidget
from parsec.core.gui.ui.files_widget import Ui_FilesWidget
from parsec.core.gui.ui.file_item_widget import Ui_FileItemWidget


class FileItemWidget(QWidget, Ui_FileItemWidget):
    def __init__(self, parent, item, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.item = item
        self.parent = parent
        self.button_delete.clicked.connect(self.delete_file)

    def delete_file(self):
        self.parent.remove_item(self.item)


class ParentFolderWidget(QWidget, Ui_ParentFolderWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)


class FilesWidget(QWidget, Ui_FilesWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.reset()
        self.workspaces_number = 0
        self.line_edit_new_workspace.textChanged.connect(self.disable_add_workspace)
        self.button_add_workspace.clicked.connect(self.add_workspace)

    def add_workspace(self):
        workspace_name = self.line_edit_new_workspace.text()
        button = ToolButton()
        button.setIcon(QIcon(':/icons/images/icons/archive.png'))
        button.setText(workspace_name)
        button.setIconSize(QSize(64, 64))
        button.setToolButtonStyle(Qt.ToolButtonTextUnderIcon)
        button.setAutoRaise(True)
        button.setFixedSize(96, 96)
        self.layout_workspaces.addWidget(
            button, int(self.workspaces_number / 4), int(self.workspaces_number % 4))
        self.workspaces_number += 1

    def disable_add_workspace(self, value):
        if len(value):
            self.button_add_workspace.setEnabled(True)
        else:
            self.button_add_workspace.setEnabled(False)

        # item = QListWidgetItem()
        # widget = ParentFolderWidget()
        # item.setSizeHint(widget.sizeHint())
        # self.list_files.addItem(item)
        # self.list_files.setItemWidget(item, widget)

        # for i in range(1, 10):
        #     item = QListWidgetItem()
        #     widget = FileItemWidget(self, item)
        #     widget.label_file_name.setText('File_{}.txt'.format(i))
        #     item.setSizeHint(widget.sizeHint())
        #     self.list_files.addItem(item)
        #     self.list_files.setItemWidget(item, widget)

    def load_workspace(self, workspace):
        print(workspace)

    def remove_item(self, item):
        self.list_files.takeItem(self.list_files.row(item))

    def open_file(self, path):
        QDesktopServices.openUrl(QUrl(QFileInfo(path).absoluteFilePath()))

    def reset(self):
        self.widget_files.hide()
        self.widget_workspaces.show()
        self.workspaces_number = 0
        self.layout_workspaces = QGridLayout()
        self.widget_workspaces.layout().takeAt(1)
        self.widget_workspaces.layout().addLayout(self.layout_workspaces)
        self.line_edit_new_workspace.setText('')
        self.button_add_workspace.setDisabled(True)
