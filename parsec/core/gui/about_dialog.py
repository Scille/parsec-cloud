from PyQt5.QtWidgets import QDialog

from parsec.core.gui.ui.about_dialog import Ui_AboutDialog


class AboutDialog(QDialog, Ui_AboutDialog):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.setupUi(self)
