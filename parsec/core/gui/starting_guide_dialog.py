# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import QSize, Qt
from PyQt5.QtGui import QPixmap
from PyQt5.QtWidgets import QDialog

from parsec.core.gui.lang import translate as _

from parsec.core.gui.ui.starting_guide_dialog import Ui_StartingGuideDialog


class StartingGuideDialog(QDialog, Ui_StartingGuideDialog):
    PAGES = None

    def init_pages(self):
        self.PAGES = [
            {
                "title": _("STARTING_GUIDE_WELCOME_TITLE"),
                "text": _("STARTING_GUIDE_WELCOME_CONTENT"),
                "screenshot": _("STARTING_GUIDE_WELCOME_SCREENSHOT"),
                "size": QSize(300, 300),
            },
            {
                "title": _("STARTING_GUIDE_WORKSPACES_TITLE"),
                "text": _("STARTING_GUIDE_WORKSPACES_CONTENT"),
                "screenshot": _("STARTING_GUIDE_WORKSPACES_SCREENSHOT"),
                "size": QSize(450, 300),
            },
            {
                "title": _("STARTING_GUIDE_DOCUMENTS_TITLE"),
                "text": _("STARTING_GUIDE_DOCUMENTS_CONTENT"),
                "screenshot": _("STARTING_GUIDE_DOCUMENTS_SCREENSHOT"),
                "size": QSize(450, 300),
            },
            {
                "title": _("STARTING_GUIDE_SHARING_TITLE"),
                "text": _("STARTING_GUIDE_SHARING_CONTENT"),
                "screenshot": _("STARTING_GUIDE_SHARING_SCREENSHOT"),
                "size": QSize(450, 300),
            },
            {
                "title": _("STARTING_GUIDE_DEVICES_TITLE"),
                "text": _("STARTING_GUIDE_DEVICES_CONTENT"),
                "screenshot": _("STARTING_GUIDE_DEVICES_SCREENSHOT"),
                "size": QSize(450, 300),
            },
            {
                "title": _("STARTING_GUIDE_USERS_TITLE"),
                "text": _("STARTING_GUIDE_USERS_CONTENT"),
                "screenshot": _("STARTING_GUIDE_USERS_SCREENSHOT"),
                "size": QSize(450, 300),
            },
            {
                "title": _("STARTING_GUIDE_OFFLINE_TITLE"),
                "text": _("STARTING_GUIDE_OFFLINE_CONTENT"),
                "screenshot": _("STARTING_GUIDE_OFFLINE_SCREENSHOT"),
                "size": QSize(450, 300),
            },
            {
                "title": _("STARTING_GUIDE_YOUR_TURN_TITLE"),
                "text": _("STARTING_GUIDE_YOUR_TURN_CONTENT"),
                "screenshot": _("STARTING_GUIDE_YOUR_TURN_SCREENSHOT"),
                "size": QSize(450, 300),
            },
        ]

    def __init__(self, parent, *args, **kwargs):
        super().__init__(*args, parent=parent, **kwargs)
        self.setupUi(self)
        self.init_pages()
        self.setWindowFlags(Qt.SplashScreen)
        self.page = 0
        self.max_page = 7
        self.label_page.max_page = 8
        self.label_title.setText(self.PAGES[self.page]["title"])
        self.label_doc.setText(self.PAGES[self.page]["text"])
        self.label_screenshot.setPixmap(QPixmap(self.PAGES[self.page]["screenshot"]))
        self.label_screenshot.setFixedSize(self.PAGES[self.page]["size"])

        self.button_next.clicked.connect(self.next_page)
        self.button_previous.clicked.connect(self.previous_page)
        self.button_previous.hide()
        self.label_page.repaint()

    def next_page(self):
        self.page += 1
        if self.page > self.max_page:
            self.accept()
        if self.page >= self.max_page:
            self.page = self.max_page
            self.button_next.show()
            self.button_previous.show()
            self.button_placeholder.hide()
            self.button_next.setText(_("BUTTON_END"))
        else:
            self.button_next.show()
            self.button_previous.show()
            self.button_placeholder.hide()
            self.button_next.setText(_("BUTTON_CONTINUE"))
        self.label_page.page = self.page
        self.label_title.setText(self.PAGES[self.page]["title"])
        self.label_doc.setText(self.PAGES[self.page]["text"])
        self.label_screenshot.setPixmap(QPixmap(self.PAGES[self.page]["screenshot"]))
        self.label_screenshot.setFixedSize(self.PAGES[self.page]["size"])
        self.label_page.repaint()

    def previous_page(self):
        self.page -= 1
        if self.page <= 0:
            self.page = 0
            self.button_next.show()
            self.button_previous.hide()
            self.button_placeholder.show()
            self.button_next.setText(_("BUTTON_START"))
        else:
            self.button_next.show()
            self.button_previous.show()
            self.button_placeholder.hide()
            self.button_next.setText(_("BUTTON_CONTINUE"))
        self.label_page.page = self.page
        self.label_title.setText(self.PAGES[self.page]["title"])
        self.label_doc.setText(self.PAGES[self.page]["text"])
        self.label_screenshot.setPixmap(QPixmap(self.PAGES[self.page]["screenshot"]))
        self.label_screenshot.setFixedSize(self.PAGES[self.page]["size"])
        self.label_page.repaint()
