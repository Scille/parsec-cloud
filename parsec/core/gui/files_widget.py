from PyQt5.QtCore import QFileInfo, QUrl
from PyQt5.QtGui import QDesktopServices
from PyQt5.QtWidgets import QWidget, QListWidgetItem, QLabel

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

        self.label_current_directory.setText('/home/max/test')
        self.label_cd_size.setText('167 Ko')
        self.label_cd_elems.setText('8 elements')

        item = QListWidgetItem()
        widget = ParentFolderWidget()
        item.setSizeHint(widget.sizeHint())
        self.list_files.addItem(item)
        self.list_files.setItemWidget(item, widget)

        for i in range(1, 10):
            item = QListWidgetItem()
            widget = FileItemWidget(self, item)
            widget.label_file_name.setText('File_{}.txt'.format(i))
            item.setSizeHint(widget.sizeHint())
            self.list_files.addItem(item)
            self.list_files.setItemWidget(item, widget)

    def remove_item(self, item):
        self.list_files.takeItem(self.list_files.row(item))

    def open_file(self, path):
        QDesktopServices.openUrl(QUrl(QFileInfo(path).absoluteFilePath()))
