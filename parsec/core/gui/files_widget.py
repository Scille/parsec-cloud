import os
import pathlib

from PyQt5.QtCore import Qt, QSize, QCoreApplication, QDir, pyqtSignal, QPoint
from PyQt5.QtGui import QIcon, QPixmap
from PyQt5.QtWidgets import QWidget, QListWidgetItem, QMenu, QMessageBox, QInputDialog, QFileDialog

from parsec.core.gui import desktop
from parsec.core.gui.core_call import core_call
from parsec.core.gui.file_size import get_filesize
from parsec.core.gui.custom_widgets import ToolButton
from parsec.core.gui.ui.parent_folder_widget import Ui_ParentFolderWidget
from parsec.core.gui.ui.files_widget import Ui_FilesWidget
from parsec.core.gui.ui.file_item_widget import Ui_FileItemWidget
from parsec.core.fs import FSManifestLocalMiss
from parsec.core.fs.sharing import SharingRecipientError


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
            "</span></p></body></html>".format(file_name)
        )
        if self.file_infos["type"] == "file":
            self.label_file_type.setPixmap(QPixmap(":/icons/images/icons/file.png"))
            self.label_file_size.setText(
                '<html><head/><body><p><span style="font-style:italic;">{}'
                "</span></p></body></html>".format(get_filesize(self.file_infos["size"]))
            )
        elif self.file_infos["type"] == "folder":
            if file_infos.get("children", []):
                self.label_file_type.setPixmap(QPixmap(":/icons/images/icons/folder_full.png"))
            else:
                self.label_file_type.setPixmap(QPixmap(":/icons/images/icons/folder_empty.png"))
        self.label_file_type.setScaledContents(True)
        creation_date = None
        update_date = None
        try:
            creation_date = self.file_infos["created"].format(
                "%a %d %b %Y, %H:%M:%S", locale=desktop.get_locale_language()
            )
            update_date = self.file_infos["updated"].format(
                "%a %d %b %Y, %H:%M:%S", locale=desktop.get_locale_language()
            )
        except ValueError:
            creation_date = self.file_infos["created"].format("%a %d %b %Y, %H:%M:%S")
            update_date = self.file_infos["updated"].format("%a %d %b %Y, %H:%M:%S")

        self.label_created.setText(
            QCoreApplication.translate(
                "FilesWidget",
                '<html><head/><body><p><span style="font-style:italic;">'
                "Created on {}</span></p></body></html>".format(creation_date),
            )
        )
        self.label_modified.setText(
            QCoreApplication.translate(
                "FilesWidget",
                '<html><head/><body><p><span style="font-style:italic;">'
                "Updated on {}</span></p></body></html>".format(update_date),
            )
        )

    @property
    def file_type(self):
        return self.file_infos["type"]


class WorkspaceWidget(ToolButton):
    context_menu_requested = pyqtSignal(ToolButton, QPoint)

    def __init__(self, workspace_name, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.setIcon(QIcon(":/icons/images/icons/workspace.png"))
        self.setText(workspace_name)
        self.setIconSize(QSize(64, 64))
        self.setToolButtonStyle(Qt.ToolButtonTextUnderIcon)
        self.setAutoRaise(True)
        self.setMinimumSize(96, 96)
        self.setContextMenuPolicy(Qt.CustomContextMenu)
        self.customContextMenuRequested.connect(self.emit_context_menu_requested)

    def emit_context_menu_requested(self, pos):
        self.context_menu_requested.emit(self, pos)


class ParentFolderWidget(QWidget, Ui_ParentFolderWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.label_icon.setPixmap(QPixmap(":/icons/images/icons/folder_parent.png"))


class FilesWidget(QWidget, Ui_FilesWidget):
    current_directory_changed = pyqtSignal()

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
        self.button_import_files.clicked.connect(self.import_files_clicked)
        self.button_import_folder.clicked.connect(self.import_folder_clicked)
        self.workspaces = []
        self.current_workspace = None
        self.current_directory = None
        core_call().connect_signal("fs.entry.updated", self._on_fs_entry_updated)
        self.current_directory_changed.connect(self.reload_current_directory)

    def reload_current_directory(self):
        self.load_directory(self.current_workspace, self.current_directory)

    def _on_fs_entry_updated(self, event, id):
        if self.current_directory is None:
            return
        try:
            path = core_call().get_entry_path(id)
        except FSManifestLocalMiss:
            return
        # TODO: too cumbersome...
        modified_hops = [x for x in path.split("/") if x]
        current_dir_hops = [self.current_workspace] + [
            x for x in self.current_directory.split("/") if x
        ]
        # Only direct children to current directory require reloading
        if modified_hops[:-1] == current_dir_hops:
            self.current_directory_changed.emit()

    def delete_all_subs(self, dir_path):
        result = core_call().stat(dir_path)
        for child in result.get("children", []):
            file_infos = core_call().stat(os.path.join(dir_path, child))
            if file_infos["type"] == "folder":
                self.delete_all_subs(os.path.join(dir_path, child))
            else:
                core_call().delete_file(os.path.join(dir_path, child))
        core_call().delete_folder(dir_path)

    def import_folder_clicked(self):
        path = QFileDialog.getExistingDirectory(
            self, "Select a directory to import", str(pathlib.Path.home())
        )
        if not path:
            return None

        files = self._current_file_names()
        filename = os.path.basename(path)

        if filename in files:
            QMessageBox.error(
                self,
                QCoreApplication.translate("FilesWidget", "{} already exists").format(filename),
                QCoreApplication.translate(
                    "FilesWidget",
                    "A folder with the same name already exists. "
                    "Please delete the existing directory before importing the new one.",
                ),
            )
            return
        QMessageBox.warning(
            self,
            QCoreApplication.translate("FilesWidget", "Importing the folder"),
            QCoreApplication.translate(
                "FilesWidget", "Sub-folders will not be imported to prevent big data imports."
            ),
        )
        core_call().create_folder(
            os.path.join(self.current_workspace, self.current_directory, filename)
        )
        finfo = QDir(path)
        files = finfo.entryInfoList(filters=QDir.Files)
        errors = []
        for f in files:
            ret = self._import_file(
                f.absoluteFilePath(),
                os.path.join(
                    self.current_workspace, self.current_directory, filename, f.fileName()
                ),
            )
            if not ret:
                errors.append(f.absoluteFilePath())
        if errors:
            QMessageBox.warning(
                self,
                QCoreApplication.translate("FilesWidget", "Error"),
                QCoreApplication.translate("FilesWidget", "Can not import\n{}.").format(
                    "\n".join(errors)
                ),
            )
        self.load_directory(self.current_workspace, self.current_directory)

    def _import_file(self, source, dest):
        fd_out = None
        try:
            core_call().file_create(dest)
            fd_out = core_call().file_open(dest)
            with open(source, "rb") as fd_in:
                while True:
                    chunk = fd_in.read(8192)
                    if not chunk:
                        break
                    core_call().file_write(fd_out, chunk)
            return True
        except OSError as exc:
            return False
        finally:
            if fd_out:
                core_call().file_close(fd_out)

    def _current_file_names(self):
        current_files = []
        for i in range(self.list_files.count()):
            item = self.list_files.item(i)
            widget = self.list_files.itemWidget(item)
            if not isinstance(widget, ParentFolderWidget):
                current_files.append(widget.file_name)
        return current_files

    def import_files_clicked(self):
        paths, _ = QFileDialog.getOpenFileNames(
            self, "Select files to import", str(pathlib.Path.home())
        )
        if not paths:
            return None
        errors = []
        current_files = self._current_file_names()

        for path in paths:
            filename = os.path.basename(path)
            out_path = os.path.join(self.current_workspace, self.current_directory, filename)
            if filename in current_files:
                result = QMessageBox.question(
                    self,
                    QCoreApplication.translate("FilesWidget", "{} already exists").format(filename),
                    QCoreApplication.translate(
                        "FilesWidget",
                        "A file with the same name already exists. " "Do you wish to replace it ?",
                    ),
                )
                if result == QMessageBox.Yes:
                    core_call().delete_file(out_path)
                else:
                    continue
            result = self._import_file(path, out_path)
            if not result:
                errors.append(filename)
        if errors:
            QMessageBox.warning(
                self,
                QCoreApplication.translate("FilesWidget", "Error"),
                QCoreApplication.translate("FilesWidget", "Can not import\n{}.").format(
                    "\n".join(errors)
                ),
            )
        self.load_directory(self.current_workspace, self.current_directory)

    def filter_files(self, pattern):
        for i in range(self.list_files.count()):
            item = self.list_files.item(i)
            widget = self.list_files.itemWidget(item)
            if pattern not in widget.file_name:
                item.setHidden(True)
            else:
                item.setHidden(False)

    def create_folder_clicked(self):
        dir_name, ok = QInputDialog.getText(
            self,
            QCoreApplication.translate("FilesWidget", "New folder"),
            QCoreApplication.translate("FilesWidget", "Enter new folder name"),
        )
        if not ok or not dir_name:
            return
        try:
            core_call().create_folder(
                os.path.join(self.current_workspace, self.current_directory, dir_name)
            )
            self.load_directory(self.current_workspace, self.current_directory)
        except FileExistsError:
            QMessageBox.warning(
                self,
                QCoreApplication.translate("FilesWidget", "Error"),
                QCoreApplication.translate(
                    "FilesWidget", "A folder with the same name already exists."
                ),
            )

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
        if widget.file_type == "file":
            action = menu.addAction(QCoreApplication.translate("FilesWidget", "Open"))
        elif widget.file_type == "folder":
            action = menu.addAction(
                QCoreApplication.translate("FilesWidget", "Open in file explorer")
            )
        action.triggered.connect(self.action_open_file_clicked)
        action = menu.addAction(QCoreApplication.translate("FilesWidget", "Delete"))
        action.triggered.connect(self.action_delete_file_clicked)
        menu.exec(global_pos)

    def item_double_clicked(self, item):
        widget = self.list_files.itemWidget(item)
        if isinstance(widget, ParentFolderWidget):
            self.load_directory(self.current_workspace, os.path.dirname(self.current_directory))
        else:
            if widget.file_type == "file":
                self.open_file(item)
            else:
                self.load_directory(
                    self.current_workspace, os.path.join(self.current_directory, widget.file_name)
                )

    def action_open_file_clicked(self):
        item = self.list_files.currentItem()
        self.open_file(item)

    def action_delete_file_clicked(self):
        item = self.list_files.currentItem()
        widget = self.list_files.itemWidget(item)
        file_path = os.path.join(self.current_workspace, self.current_directory, widget.file_name)
        if widget.file_type == "folder":
            result = QMessageBox.question(
                self,
                QCoreApplication.translate("FilesWidget", "Confirmation"),
                QCoreApplication.translate(
                    "FilesWidget", 'Are you sure you want to delete folder "{}" ?'
                ).format(widget.file_name),
            )
            if result == QMessageBox.Yes:
                self.delete_all_subs(file_path)
                self.list_files.takeItem(self.list_files.row(item))
        elif widget.file_type == "file":
            result = QMessageBox.question(
                self,
                QCoreApplication.translate("FilesWidget", "Confirmation"),
                QCoreApplication.translate(
                    "FilesWidget", 'Are you sure you want to delete file "{}" ?'
                ).format(widget.file_name),
            )
            if result == QMessageBox.Yes:
                core_call().delete_file(file_path)
                self.list_files.takeItem(self.list_files.row(item))
        self.label_cd_elems.setText(
            QCoreApplication.translate("FilesWidget", "{} element(s)").format(
                self.list_files.count()
            )
        )

    def open_file(self, item):
        widget = self.list_files.itemWidget(item)
        file_name = widget.file_name
        desktop.open_file(
            os.path.join(
                self.label_mountpoint.text(),
                self.current_workspace,
                self.current_directory,
                file_name,
            )
        )

    def show(self, *args, **kwargs):
        super().show(*args, **kwargs)
        self.reset()
        result = core_call().stat("/")
        for workspace in result.get("children", []):
            self._add_workspace(workspace)

    def _add_workspace(self, workspace_name):
        button = WorkspaceWidget(workspace_name)
        button.clicked_name.connect(self.load_workspace)
        self.layout_workspaces.addWidget(
            button, int(self.workspaces_number / 4), int(self.workspaces_number % 4)
        )
        self.workspaces_number += 1
        self.workspaces.append(button)
        button.context_menu_requested.connect(self.workspace_context_menu_clicked)

    def workspace_context_menu_clicked(self, workspace_button, pos):
        global_pos = workspace_button.mapToGlobal(pos)
        menu = QMenu(workspace_button)
        action = menu.addAction(QCoreApplication.translate("FilesWidget", "Share"))
        action.triggered.connect(self.share_workspace(workspace_button.text()))
        menu.exec(global_pos)

    def share_workspace(self, workspace_name):
        def _inner_share_workspace():
            user, ok = QInputDialog.getText(
                self,
                QCoreApplication.translate("FilesWidget", "Share a workspace"),
                QCoreApplication.translate(
                    "FilesWidget",
                    "Give a user name to share the workspace {} with.".format(workspace_name),
                ),
            )
            if not ok or not user:
                return
            try:
                core_call().share_workspace("/" + workspace_name, user)
            except SharingRecipientError:
                QMessageBox.warning(
                    self,
                    QCoreApplication.translate("FilesWidget", "Error"),
                    QCoreApplication.translate(
                        "FilesWidget",
                        'Can not share the workspace "{}" with yourself.'.format(workspace_name),
                    ),
                )
            except:
                QMessageBox.warning(
                    self,
                    QCoreApplication.translate("FilesWidget", "Error"),
                    QCoreApplication.translate(
                        "FilesWidget",
                        'Can not share the workspace "{}" with "{}".'.format(workspace_name, user),
                    ),
                )

        return _inner_share_workspace

    def create_workspace_clicked(self):
        workspace_name, ok = QInputDialog.getText(
            self,
            QCoreApplication.translate("FilesWidget", "New workspace"),
            QCoreApplication.translate("FilesWidget", "Enter new workspace name"),
        )
        if not ok or not workspace_name:
            return
        try:
            core_call().create_workspace(workspace_name)
            self._add_workspace(workspace_name)
        except FileExistsError:
            QMessageBox.warning(
                self,
                QCoreApplication.translate("FilesWidget", "Error"),
                QCoreApplication.translate(
                    "FilesWidget", "A workspace with the same name already exists."
                ),
            )
            return

    def load_directory(self, workspace, directory):
        self.current_directory = directory
        self.list_files.clear()
        result = core_call().stat(os.path.join(workspace, directory))

        if not directory:
            print(result)

        if len(self.current_directory):
            item = QListWidgetItem()
            widget = ParentFolderWidget()
            item.setSizeHint(widget.sizeHint())
            self.list_files.addItem(item)
            self.list_files.setItemWidget(item, widget)

        for file_name in result.get("children", []):
            file_infos = core_call().stat(os.path.join(workspace, directory, file_name))
            item = QListWidgetItem()
            widget = FileItemWidget(self, item, file_name, file_infos)
            item.setSizeHint(widget.sizeHint())
            self.list_files.addItem(item)
            self.list_files.setItemWidget(item, widget)

        self.label_current_directory.setText(
            '<html><head/><body><p><span style="font-size:16pt;">/{}'
            "</span></p></body></html>".format(workspace)
        )
        self.label_cd_elems.setText(
            QCoreApplication.translate("FilesWidget", "{} element(s)").format(
                len(result.get("children", []))
            )
        )

    def load_workspace(self, workspace):
        self.current_workspace = workspace
        self.load_directory(workspace, "")
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
