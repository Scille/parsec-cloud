# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import QSize, Qt
from PyQt5.QtGui import QPixmap, QColor
from PyQt5.QtWidgets import QDialog, QGraphicsDropShadowEffect

from parsec.core.gui.lang import translate as _

from parsec.core.gui.ui.starting_guide_dialog import Ui_StartingGuideDialog


class StartingGuideDialog(QDialog, Ui_StartingGuideDialog):
    PAGES = None

    def init_pages(self):
        self.PAGES = [
            {
                "title": _("Welcome!"),
                "text": _(
                    "Parsec allows you to store data in the cloud with a very high security level."
                    "\nFind out more by letting us guide you step by step!"
                ),
                "screenshot": _(":/screenshots/images/screenshots/cloud-computing.png"),
                "size": QSize(300, 300),
            },
            {
                "title": _("Workspaces"),
                "text": _(
                    "Arrange all your files by workspaces, a clear interface for an easy to use "
                    "and simple process."
                ),
                "screenshot": _(":/screenshots/images/screenshots/workspaces.png"),
                "size": QSize(450, 300),
            },
            {
                "title": _("Documents"),
                "text": _(
                    "In your workspaces, see and manage all your files, just as with a regular "
                    "file explorer. A lightweight display to increase your productivity!"
                ),
                "screenshot": _(":/screenshots/images/screenshots/documents.png"),
                "size": QSize(450, 300),
            },
            {
                "title": _("Sharing"),
                "text": _(
                    "Share your workspaces with a few clicks. See who can access it, and what "
                    "others share with you.\nEverything shared with you automatically appears."
                ),
                "screenshot": _(":/screenshots/images/screenshots/page4.png"),
                "size": QSize(450, 300),
            },
            {
                "title": _("Manage your devices"),
                "text": _(
                    "See and manage thee devices you trust. Those are the key to the guaranted "
                    "security of your files."
                ),
                "screenshot": _(":/screenshots/images/screenshots/devices.png"),
                "size": QSize(450, 300),
            },
            {
                "title": _("Manage the users"),
                "text": _(
                    "And for the administrators ? You can manage all users and their privileges, "
                    "add users, or revoke them."
                ),
                "screenshot": _(":/screenshots/images/screenshots/users.png"),
                "size": QSize(450, 300),
            },
            {
                "title": _("Even offline"),
                "text": _(
                    "No network ? Not a problem. You can access all your files. Modifications will "
                    "be saved locally until your device comes online again."
                ),
                "screenshot": _(":/screenshots/images/screenshots/offline_en.png"),
                "size": QSize(450, 300),
            },
            {
                "title": _("Now it's your turn !"),
                "text": _(
                    "Create your first workspace, import your files, share your documents."
                    "\nSecurity waits for no one!"
                ),
                "screenshot": _(":/screenshots/images/screenshots/your_turn.png"),
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

        effect = QGraphicsDropShadowEffect(self)
        effect.setColor(QColor(164, 164, 164))
        effect.setBlurRadius(5)
        effect.setXOffset(0)
        effect.setYOffset(4)
        self.button_previous.setGraphicsEffect(effect)

        effect = QGraphicsDropShadowEffect(self)
        effect.setColor(QColor(164, 164, 164))
        effect.setBlurRadius(5)
        effect.setXOffset(0)
        effect.setYOffset(4)
        self.button_next.setGraphicsEffect(effect)

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
            self.button_next.setText(_("End"))
        else:
            self.button_next.show()
            self.button_previous.show()
            self.button_placeholder.hide()
            self.button_next.setText(_("Continue"))
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
            self.button_next.setText(_("Start"))
        else:
            self.button_next.show()
            self.button_previous.show()
            self.button_placeholder.hide()
            self.button_next.setText(_("Continue"))
        self.label_page.page = self.page
        self.label_title.setText(self.PAGES[self.page]["title"])
        self.label_doc.setText(self.PAGES[self.page]["text"])
        self.label_screenshot.setPixmap(QPixmap(self.PAGES[self.page]["screenshot"]))
        self.label_screenshot.setFixedSize(self.PAGES[self.page]["size"])
        self.label_page.repaint()
