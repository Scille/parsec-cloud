from PyQt5.QtCore import QSize, Qt
from PyQt5.QtWidgets import QDialog

from parsec.core.gui.ui.starting_guide_dialog import Ui_StartingGuideDialog


class StartingGuideDialog(QDialog, Ui_StartingGuideDialog):
    def __init__(self, parent, *args, **kwargs):
        super().__init__(*args, parent=parent, **kwargs)
        self.setupUi(self)
        parent_size = parent.size()
        self.setFixedSize(QSize(parent_size.width() - 200, parent_size.height() - 200))
        self.setWindowFlags(Qt.SplashScreen)
