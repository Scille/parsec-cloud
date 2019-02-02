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
        self.page = 0
        self.max_page = 7
        self.label_page.max_page = 8
        self.button_next.clicked.connect(self.next_page)
        self.button_previous.clicked.connect(self.previous_page)
        self.button_previous.hide()
        self.button_next_placeholder.hide()
        self.label_page.repaint()

    def next_page(self):
        self.page += 1
        self.label_page.page = self.page
        if self.page >= self.max_page:
            self.page = self.max_page
            self.button_next.hide()
            self.button_next_placeholder.show()
            self.button_previous.show()
            self.button_previous_placeholder.hide()
            self.button_next.setText("End")
        else:
            self.button_next.show()
            self.button_next_placeholder.hide()
            self.button_previous.show()
            self.button_previous_placeholder.hide()
            self.button_next.setText("Continue")
        self.label_page.repaint()

    def previous_page(self):
        self.page -= 1
        self.label_page.page = self.page
        if self.page <= 0:
            self.page = 0
            self.button_next.show()
            self.button_next_placeholder.hide()
            self.button_previous.hide()
            self.button_previous_placeholder.show()
            self.button_next.setText("Start")
        else:
            self.button_next.show()
            self.button_next_placeholder.hide()
            self.button_previous.show()
            self.button_previous_placeholder.hide()
            self.button_next.setText("Continue")
        self.label_page.repaint()
