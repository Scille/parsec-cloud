from PyQt5.QtWidgets import QWidget

from parsec.core.gui.ui.files_widget import Ui_FilesWidget


class FilesWidget(QWidget, Ui_FilesWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.setupUi(self)
