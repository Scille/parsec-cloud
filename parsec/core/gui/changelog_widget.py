# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from PyQt5.QtWidgets import QWidget
from PyQt5.QtCore import QFile, QIODevice

from structlog import get_logger

from parsec.core.gui.ui.changelog_widget import Ui_ChangelogWidget


logger = get_logger()


class ChangelogWidget(QWidget, Ui_ChangelogWidget):
    def __init__(self, parent: QWidget | None = None) -> None:
        super().__init__(parent)
        self.setupUi(self)
        rc_file = QFile(":/generated_misc/generated_misc/history.html")
        if not rc_file.open(QIODevice.ReadOnly):
            logger.warning("Unable to read the changelog")
        else:
            self.text_changelog.setHtml(rc_file.readAll().data().decode("utf-8"))
            rc_file.close()
