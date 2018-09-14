import os

from PyQt5.QtCore import Qt, QSize, QCoreApplication
from PyQt5.QtGui import QIcon, QPixmap
from PyQt5.QtWidgets import (QWidget, QListWidgetItem, QMenu,
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


class WorkspaceWidget(ToolButton):
    def __init__(self, workspace_name, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.setIcon(QIcon(':/icons/images/icons/workspace.png'))
        self.setText(workspace_name)
        self.setIconSize(QSize(64, 64))
        self.setToolButtonStyle(Qt.ToolButtonTextUnderIcon)
        self.setAutoRaise(True)
        self.setFixedSize(96, 96)


class ParentFolderWidget(QWidget, Ui_ParentFolderWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.label_icon.setPixmap(
            QPixmap(':/icons/images/icons/folder_parent.png'))


class FilesWidget(QWidget, Ui_FilesWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.workspaces_number = 0
        self.button_add_workspace.clicked.connect(self.create_workspace_clicked)
        self.list_files.setContextMenuPolicy(Qt.CustomContextMenu)
        self.list_files.customContextMenuRequested.connect(self.show_context_menu)
        self.list_files.itemDoubleClicked.connect(self.item_double_clicked)
        self.button_create_folder.clicked.connect(self.create_folder_clicked)
        self.line_edit_search.textChanged.connect(self.filter_files)
        self.workspaces = []
        self.current_workspace = None
        self.current_directory = None

    def filter_files(self, pattern):
        for i in range(self.list_files.count()):
            item = self.list_files.item(i)
            widget = self.list_files.itemWidget(item)
            if pattern not in widget.file_name:
                item.setHidden(True)
            else:
                item.setHidden(False)

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
                                                               'Open'))
        elif widget.file_type == 'folder':
            action = menu.addAction(QCoreApplication.translate(self.__class__.__name__,
                                                               'Open in file explorer'))
        action.triggered.connect(self.action_open_file_clicked)
        action = menu.addAction(QCoreApplication.translate(self.__class__.__name__,
                                                           'Delete'))
        action.triggered.connect(self.action_delete_file_clicked)
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

    def action_delete_file_clicked(self):
        item = self.list_files.currentItem()
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
        self.label_cd_elems.setText(
            QCoreApplication.translate(self.__class__.__name__,
                                       '{} element(s)'.format(self.list_files.count())))

    def open_file(self, item):
        widget = self.list_files.itemWidget(item)
        file_name = widget.file_name
        desktop.open_file(os.path.join(
            self.label_mountpoint.text(), self.current_workspace,
            self.current_directory, file_name))

    def show(self, *args, **kwargs):
        super().show(*args, **kwargs)
        self.reset()
        result = core_call().stat('/')
        for workspace in result.get('children', []):
            self._add_workspace(workspace)

    def _add_workspace(self, workspace_name):
        button = WorkspaceWidget(workspace_name)
        button.clicked_name.connect(self.load_workspace)
        self.layout_workspaces.addWidget(
            button, int(self.workspaces_number / 4), int(self.workspaces_number % 4))
        self.workspaces_number += 1
        self.workspaces.append(button)

    def create_workspace_clicked(self):
        workspace_name, ok = QInputDialog.getText(self, 'New workspace', 'Enter new workspace name')
        if not ok or not workspace_name:
            return
        try:
            core_call().create_workspace(workspace_name)
            self._add_workspace(workspace_name)
        except FileExistsError:
            QMessageBox.warning(self, 'Error', 'A workspace with the same name already exists.')
            return

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

    def reset(self):
        self.widget_files.hide()
        self.widget_workspaces.show()
        self.workspaces_number = 0
        layout = self.widget_workspaces.layout().itemAt(1).layout()
        for ws in self.workspaces:
            layout.removeWidget(ws)
            ws.setParent(None)
        self.workspaces = []
        self.current_directory = None
        self.current_workspace = None
