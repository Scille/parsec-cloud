from PyQt5.QtCore import QCoreApplication, Qt
from PyQt5.QtWidgets import QDialog

from parsec.core.gui.ui.replace_dialog import Ui_ReplaceDialog


class ReplaceDialog(QDialog, Ui_ReplaceDialog):
    def __init__(self, dst, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.label_message.setText(
            QCoreApplication.translate(
                "ReplaceDialog", 'The file "{}" already exists.\nDo you want to replace it ?'
            ).format(dst)
        )
        self.button_skip.clicked.connect(self.skip_clicked)
        self.button_replace.clicked.connect(self.replace_clicked)
        self.skip = False
        self.replace = False
        self.setWindowFlags(Qt.SplashScreen)

    @property
    def all_files(self):
        return self.check_box_all.isChecked()

    def skip_clicked(self):
        self.skip = True
        self.replace = False
        self.accept()

    def replace_clicked(self):
        self.replace = True
        self.skip = False
        self.accept()
