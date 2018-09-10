import os

from PyQt5.QtCore import Qt, QSize, QCoreApplication
from PyQt5.QtGui import QIcon, QPixmap
from PyQt5.QtWidgets import (QWidget, QListWidgetItem, QGridLayout, QMenu,
                             QMessageBox, QInputDialog)

from parsec.core.gui import desktop
from parsec.core.gui.core_call import core_call
from parsec.core.gui.file_size import get_filesize
from parsec.core.gui.custom_widgets import ToolButton
from parsec.core.gui.ui.parent_folder_widget import Ui_ParentFolderWidget
from parsec.core.gui.ui.files_widget import Ui_FilesWidget
from parsec.core.gui.ui.file_item_widget import Ui_FileItemWidget


class FileItemWidget(QWidget, Ui_FileItemWidget):
    def __init__(self, parent, item, file_name, file_infos, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.item = item
        self.parent = parent
        self.file_infos = file_infos
        self.file_name = file_name
        self.button_delete.clicked.connect(self.delete_file)

        self.label_file_name.setText(
            '<html><head/><body><p><span style="font-size:14pt;">{}'
            '</span></p></body></html>'.format(file_name))
        if self.file_infos['type'] == 'file':
            self.label_file_type.setPixmap(QPixmap(':/icons/images/icons/file.png'))
            self.label_file_size.setText(
                '<html><head/><body><p><span style="font-style:italic;">{}'
                '</span></p></body></html>'.format(get_filesize(self.file_infos['size']))
            )
        elif self.file_infos['type'] == 'folder':
            if file_infos.get('children', []):
                self.label_file_type.setPixmap(
                    QPixmap(':/icons/images/icons/folder_full.png'))
            else:
                self.label_file_type.setPixmap(
                    QPixmap(':/icons/images/icons/folder_empty.png'))
        self.label_file_type.setScaledContents(True)
        self.label_created.setText(
            QCoreApplication.translate(self.__class__.__name__,
            '<html><head/><body><p><span style="font-style:italic;">'
            'Created on {}</span></p></body></html>'.format(self.file_infos['created'])))
        self.label_modified.setText(
            QCoreApplication.translate(self.__class__.__name__,
            '<html><head/><body><p><span style="font-style:italic;">'
            'Updated on {}</span></p></body></html>'.format(self.file_infos['updated'])))

    @property
    def file_type(self):
        return self.file_infos['type']

    def delete_file(self):
        self.parent.remove_item(self.item)


class ParentFolderWidget(QWidget, Ui_ParentFolderWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.label_icon.setPixmap(
            QPixmap(':/icons/images/icons/up_arrow.png'))


class FilesWidget(QWidget, Ui_FilesWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.reset()
        self.workspaces_number = 0
        self.line_edit_new_workspace.textChanged.connect(self.disable_add_workspace)
        self.button_add_workspace.clicked.connect(self.add_workspace)
        self.list_files.setContextMenuPolicy(Qt.CustomContextMenu)
        self.list_files.customContextMenuRequested.connect(self.show_context_menu)
        self.list_files.itemDoubleClicked.connect(self.item_double_clicked)
        self.button_create_folder.clicked.connect(self.create_folder_clicked)
        self.current_workspace = None
        self.current_directory = None

    def create_folder_clicked(self):
        dir_name, ok = QInputDialog.getText(self, 'New folder', 'Enter new folder name')
        if not ok or not dir_name:
            return
        try:
            core_call().create_folder(
                os.path.join(self.current_workspace, self.current_directory, dir_name))
            self.load_directory(self.current_workspace, self.current_directory)
        except FileExistsError:
            QMessageBox.warning(self, 'Error', 'A folder with the same name already exists.')

    def set_mountpoint(self, mountpoint):
        self.label_mountpoint.setText(mountpoint)

    def show_context_menu(self, pos):
        if not self.list_files.itemAt(pos):
            return
        global_pos = self.list_files.mapToGlobal(pos)
        menu = QMenu()
        item = self.list_files.itemAt(pos)
        widget = self.list_files.itemWidget(item)
        if isinstance(widget, ParentFolderWidget):
            return
        if widget.file_type == 'file':
            action = menu.addAction(QCoreApplication.translate(self.__class__.__name__,
                                                               "Open"))
        elif widget.file_type == 'folder':
            action = menu.addAction(QCoreApplication.translate(self.__class__.__name__,
                                                               "Open in file explorer"))
        action.triggered.connect(self.action_open_file_clicked)
        menu.exec(global_pos)

    def item_double_clicked(self, item):
        widget = self.list_files.itemWidget(item)
        if isinstance(widget, ParentFolderWidget):
            self.load_directory(
                self.current_workspace,
                os.path.dirname(self.current_directory))
        else:
            if widget.file_type == 'file':
                self.open_file(item)
            else:
                self.load_directory(
                    self.current_workspace,
                    os.path.join(self.current_directory, widget.file_name))

    def action_open_file_clicked(self):
        item = self.list_files.currentItem()
        self.open_file(item)

    def open_file(self, item):
        widget = self.list_files.itemWidget(item)
        file_name = widget.file_name
        desktop.open_file(os.path.join(
            self.label_mountpoint.text(), self.current_workspace,
            self.current_directory, file_name))

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

    def load_directory(self, workspace, directory):
        self.current_directory = directory
        self.list_files.clear()
        result = core_call().stat(os.path.join(workspace, directory))

        if len(self.current_directory):
            item = QListWidgetItem()
            widget = ParentFolderWidget()
            item.setSizeHint(widget.sizeHint())
            self.list_files.addItem(item)
            self.list_files.setItemWidget(item, widget)

        for file_name in result.get('children', []):
            file_infos = core_call().stat(os.path.join(workspace, directory, file_name))
            item = QListWidgetItem()
            widget = FileItemWidget(self, item, file_name, file_infos)
            item.setSizeHint(widget.sizeHint())
            self.list_files.addItem(item)
            self.list_files.setItemWidget(item, widget)

        self.label_current_directory.setText(
            '<html><head/><body><p><span style="font-size:16pt;">/{}'
            '</span></p></body></html>'.format(workspace))
        self.label_cd_elems.setText(
            QCoreApplication.translate(self.__class__.__name__,
                                       '{} element(s)'.format(len(result.get('children', [])))))

    def load_workspace(self, workspace):
        self.current_workspace = workspace
        self.load_directory(workspace, '')
        self.widget_workspaces.hide()
        self.widget_files.show()

    def remove_item(self, item):
        widget = self.list_files.itemWidget(item)
        file_path = os.path.join(self.current_workspace,
                                 self.current_directory,
                                 widget.file_name)
        if widget.file_type == 'folder':
            result = QMessageBox.question(
                self, 'Confirmation',
                'Are you sure you want to delete folder "{}"'.format(widget.file_name))
            if result == QMessageBox.Yes:
                core_call().delete_folder(file_path)
                self.list_files.takeItem(self.list_files.row(item))
        elif widget.file_type == 'file':
            result = QMessageBox.question(
                self, 'Confirmation',
                'Are you sure you want to delete file "{}"'.format(widget.file_name))
            if result == QMessageBox.Yes:
                core_call().delete_file(file_path)
                self.list_files.takeItem(self.list_files.row(item))

    def reset(self):
        self.widget_files.hide()
        self.widget_workspaces.show()
        self.workspaces_number = 0
        self.layout_workspaces = QGridLayout()
        self.widget_workspaces.layout().takeAt(1)
        self.widget_workspaces.layout().insertLayout(1, self.layout_workspaces)
        self.line_edit_new_workspace.setText('')
        self.button_add_workspace.setDisabled(True)
        self.current_directory = None
        self.current_workspace = None
