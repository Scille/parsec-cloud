from PyQt5.QtWidgets import QWidget
from PyQt5.QtGui import QPixmap, QIcon

from parsec.core.gui.ui.file_item_widget import Ui_FileItemWidget
from parsec.core.gui.ui.folder_item_widget import Ui_FolderItemWidget
from parsec.core.gui.ui.parent_item_widget import Ui_ParentItemWidget
from parsec.core.gui.file_size import get_filesize
from parsec.core.gui.desktop import get_locale_language


class ParentItemWidget(QWidget, Ui_ParentItemWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)

    def set_selected(self, selected):
        pass


class FolderItemWidget(QWidget, Ui_FolderItemWidget):
    def __init__(self, name, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.label_name.setText(name)

    def set_selected(self, selected):
        if selected:
            self.label_type.setPixmap(QPixmap(":/icons/images/icons/folder_selected.png"))
        else:
            self.label_type.setPixmap(QPixmap(":/icons/images/icons/folder.png"))

    @property
    def name(self):
        return self.label_name.text()

    @name.setter
    def name(self, v):
        self.label_name.setText(v)


class FileItemWidget(QWidget, Ui_FileItemWidget):
    def __init__(self, name, size, created, updated, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.label_name.setText(name)
        self.label_size.setText(get_filesize(size))
        creation_date = ""
        update_date = ""
        try:
            creation_date = created.format("%x %X", locale=get_locale_language())
            update_date = updated.format("%x %X", locale=get_locale_language())
        except ValueError:
            creation_date = created.format("%d %b %Y, %H:%M:%S")
            update_date = updated.format("%d %b %Y, %H:%M:%S")
        self.label_created.setText(creation_date)
        self.label_updated.setText(update_date)

    def set_selected(self, selected):
        if selected:
            self.setStyleSheet("background-color: rgb(0, 0, 255); color: rgb(255, 255, 255);")
            self.label_type.setPixmap(QPixmap(":/icons/images/icons/file_selected.png"))
            self.button_delete.setIcon(QIcon(":/icons/images/icons/garbage_selected.png"))
        else:
            self.setStyleSheet("background-color: rgb(255, 255, 255); color: rgb(0, 0, 0);")
            self.label_type.setPixmap(QPixmap(":/icons/images/icons/file.png"))
            self.button_delete.setIcon(QIcon(":/icons/images/icons/garbage.png"))

    @property
    def name(self):
        return self.label_name.text()

    @name.setter
    def name(self, v):
        self.label_name.setText(v)
