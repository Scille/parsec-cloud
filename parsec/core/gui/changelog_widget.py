# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from PyQt5.QtWidgets import QWidget
from PyQt5.QtCore import QFile, QIODevice

from structlog import get_logger

from parsec.core.gui.lang import translate as _
from parsec.core.gui.snackbar_widget import SnackbarManager

from parsec.core.gui.ui.changelog_widget import Ui_ChangelogWidget


logger = get_logger()


class ChangelogWidget(QWidget, Ui_ChangelogWidget):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.setupUi(self)
        rc_file = QFile(":/generated_misc/generated_misc/history.html")
        if not rc_file.open(QIODevice.ReadOnly):
            logger.warning("Unable to read the changelog")
            SnackbarManager.warn(_("TEXT_CANNOT_READ_CHANGELOG"))
        else:
            self.text_changelog.setHtml(rc_file.readAll().data().decode("utf-8"))
            rc_file.close()
