# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import QSize, Qt, QCoreApplication
from PyQt5.QtGui import QPixmap, QColor
from PyQt5.QtWidgets import QDialog, QGraphicsDropShadowEffect

from parsec.core.gui.ui.starting_guide_dialog import Ui_StartingGuideDialog


class StartingGuideDialog(QDialog, Ui_StartingGuideDialog):
    PAGES = [
        {
            "title": QCoreApplication.translate("StartingGuide", "Welcome!"),
            "text": QCoreApplication.translate(
                "StartingGuide",
                "Parsec allows you to store your data in the cloud with a very high security level."
                "\nFind out more by letting us guide you step by step!",
            ),
            "screenshot": QCoreApplication.translate(
                "StartingGuide", ":/screenshots/images/screenshots/page1.png"
            ),
        },
        {
            "title": QCoreApplication.translate("StartingGuide", "Workspaces"),
            "text": QCoreApplication.translate(
                "StartingGuide",
                "Arrange all your files by workspace, a clear interface for an easy to use and "
                "simple process.",
            ),
            "screenshot": QCoreApplication.translate(
                "StartingGuide", ":/screenshots/images/screenshots/workspaces.png"
            ),
        },
        {
            "title": QCoreApplication.translate("StartingGuide", "Documents"),
            "text": QCoreApplication.translate(
                "StartingGuide",
                "In your workspaces, all your files appear, just as a regular file explorer. "
                "A lightweight display to increase your productivity!",
            ),
            "screenshot": QCoreApplication.translate(
                "StartingGuide", ":/screenshots/images/screenshots/documents.png"
            ),
        },
        {
            "title": QCoreApplication.translate("StartingGuide", "Sharing"),
            "text": QCoreApplication.translate(
                "StartingGuide",
                "Share your workspaces with a few clicks. See who can access it, and what others "
                "share with you.",
            ),
            "screenshot": QCoreApplication.translate(
                "StartingGuide", ":/screenshots/images/screenshots/page4.png"
            ),
        },
        {
            "title": QCoreApplication.translate("StartingGuide", "Manage your devices"),
            "text": QCoreApplication.translate("StartingGuide", "See and manage your devices."),
            "screenshot": QCoreApplication.translate("StartingGuide", ":/screenshots/devices.png"),
        },
        {
            "title": QCoreApplication.translate("StartingGuide", "Manage the users"),
            "text": QCoreApplication.translate(
                "StartingGuide",
                "And for the administratots ? You can manage all users and their privileges, "
                "add users, or revoke them.",
            ),
            "screenshot": QCoreApplication.translate(
                "StartingGuide", ":/screenshots/images/screenshots/users.png"
            ),
        },
        {
            "title": QCoreApplication.translate("StartingGuide", "Even offline"),
            "text": QCoreApplication.translate(
                "StartingGuide",
                "No network ? Not a problem. You can access all your files. Modifications will "
                "be saved locally until your device comes online again.",
            ),
            "screenshot": QCoreApplication.translate(
                "StartingGuide", ":/screenshots/images/screenshots/page7.png"
            ),
        },
        {
            "title": QCoreApplication.translate("StartingGuide", "Now it's your turn !"),
            "text": QCoreApplication.translate(
                "StartingGuide",
                "Create your workspaces, import your files, share your documents.\nSecurity waits "
                "for no one!",
            ),
            "screenshot": QCoreApplication.translate(
                "StartingGuide", ":/screenshots/images/screenshots/page8.png"
            ),
        },
    ]

    def __init__(self, parent, *args, **kwargs):
        super().__init__(*args, parent=parent, **kwargs)
        self.setupUi(self)
        parent_size = parent.size()
        ratio = parent_size.width() / parent_size.height()
        if parent_size.width() > parent_size.height():
            height = parent_size.height() - 100
            self.setFixedSize(QSize(height * ratio, height))
            self.label_screenshot.setFixedSize(
                QSize(self.size().width() / 2, self.size().height() / 2)
            )
        else:
            width = parent_size.width() - 100
            self.setFixedSize(QSize(width, width * ratio))
        self.setWindowFlags(Qt.SplashScreen)
        self.page = 0
        self.max_page = 7
        self.label_page.max_page = 8
        self.label_title.setText(self.PAGES[self.page]["title"])
        self.label_doc.setText(self.PAGES[self.page]["text"])
        self.label_screenshot.setPixmap(QPixmap(self.PAGES[self.page]["screenshot"]))

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
            self.button_previous_placeholder.hide()
            self.button_next.setText("End")
        else:
            self.button_next.show()
            self.button_previous.show()
            self.button_previous_placeholder.hide()
            self.button_next.setText("Continue")
        self.label_page.page = self.page
        self.label_title.setText(self.PAGES[self.page]["title"])
        self.label_doc.setText(self.PAGES[self.page]["text"])
        self.label_screenshot.setPixmap(QPixmap(self.PAGES[self.page]["screenshot"]))
        self.label_page.repaint()

    def previous_page(self):
        self.page -= 1
        if self.page <= 0:
            self.page = 0
            self.button_next.show()
            self.button_previous.hide()
            self.button_previous_placeholder.show()
            self.button_next.setText("Start")
        else:
            self.button_next.show()
            self.button_previous.show()
            self.button_previous_placeholder.hide()
            self.button_next.setText("Continue")
        self.label_page.page = self.page
        self.label_title.setText(self.PAGES[self.page]["title"])
        self.label_doc.setText(self.PAGES[self.page]["text"])
        self.label_screenshot.setPixmap(QPixmap(self.PAGES[self.page]["screenshot"]))
        self.label_page.repaint()
