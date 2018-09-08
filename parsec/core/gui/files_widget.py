from PyQt5.QtCore import QFileInfo, QUrl, Qt, QSize
from PyQt5.QtGui import QDesktopServices, QIcon, QPixmap
from PyQt5.QtWidgets import QWidget, QListWidgetItem, QLabel, QGridLayout

from parsec.core.gui.core_call import core_call
from parsec.core.gui.file_size import get_filesize
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

    def show(self, *args, **kwargs):
        super().show(*args, **kwargs)
        result = core_call().stat('/')
        for workspace in result.get('children', []):
            self._add_workspace(workspace)

    def _add_workspace(self, workspace_name):
        button = ToolButton()
        button.setIcon(QIcon(':/icons/images/icons/archive.png'))
        button.setText(workspace_name)
        button.setIconSize(QSize(64, 64))
        button.setToolButtonStyle(Qt.ToolButtonTextUnderIcon)
        button.setAutoRaise(True)
        button.setFixedSize(96, 96)
        button.clicked_name.connect(self.load_workspace)
        self.layout_workspaces.addWidget(
            button, int(self.workspaces_number / 4), int(self.workspaces_number % 4))
        self.workspaces_number += 1

    def add_workspace(self):
        workspace_name = self.line_edit_new_workspace.text()
        core_call().create_workspace(workspace_name)
        self._add_workspace(workspace_name)

    def disable_add_workspace(self, value):
        if len(value):
            self.button_add_workspace.setEnabled(True)
        else:
            self.button_add_workspace.setEnabled(False)

    def load_workspace(self, workspace):
        self.list_files.clear()
        result = core_call().stat(workspace)
        for file_name in result.get('children', []):
            file_infos = core_call().stat('{}/{}'.format(workspace, file_name))
            item = QListWidgetItem()
            widget = FileItemWidget(self, item)
            widget.label_file_name.setText(
                '<html><head/><body><p><span style="font-size:14pt;">{}'
                '</span></p></body></html>'.format(file_name))
            if file_infos['type'] == 'file':
                widget.label_file_type.setPixmap(QPixmap(':/icons/images/icons/file.png'))
                widget.label_file_size.setText(
                    '<html><head/><body><p><span style="font-style:italic;">{}'
                    '</span></p></body></html>'.format(get_filesize(file_infos['size']))
                )
            elif file_infos['type'] == 'folder':
                if file_infos.get('children', []):
                    widget.label_file_type.setPixmap(
                        QPixmap(':/icons/images/icons/folder_empty.png'))
                else:
                    widget.label_file_type.setPixmap(
                        QPixmap(':/icons/images/icons/folder_full.png'))
            widget.label_file_type.setScaledContents(True)
            widget.label_created.setText(
                '<html><head/><body><p><span style="font-style:italic;">'
                'Created on {}</span></p></body></html>'.format(file_infos['created']))
            widget.label_modified.setText(
                '<html><head/><body><p><span style="font-style:italic;">'
                'Updated on {}</span></p></body></html>'.format(file_infos['updated']))
            item.setSizeHint(widget.sizeHint())
            self.list_files.addItem(item)
            self.list_files.setItemWidget(item, widget)

        self.label_current_directory.setText(
            '<html><head/><body><p><span style="font-size:16pt;">/{}'
            '</span></p></body></html>'.format(workspace))
        self.label_cd_elems.setText('{} elements'.format(len(result.get('children', []))))
        self.widget_workspaces.hide()
        self.widget_files.show()

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
        self.widget_workspaces.layout().insertLayout(1, self.layout_workspaces)
        self.line_edit_new_workspace.setText('')
        self.button_add_workspace.setDisabled(True)
