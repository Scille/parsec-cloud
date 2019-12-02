# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtWidgets import QWidget
from PyQt5.QtCore import QFile, QIODevice

from structlog import get_logger

from parsec import __version__

from parsec.core.gui.lang import translate as _

from parsec.core.gui.ui.about_widget import Ui_AboutWidget

logger = get_logger()


class AboutWidget(QWidget, Ui_AboutWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.label_version.setText(_("PARSEC_VERSION_LABEL_{}").format(version=__version__))
        self.text_license.setHtml(_("PARSEC_LICENSE_CONTENT"))

        rc_file = QFile(":/generated_misc/generated_misc/history.html")
        if not rc_file.open(QIODevice.ReadOnly):
            logger.warning("Unable to read the changelog")
        else:
            self.text_changelog.setHtml(rc_file.readAll().data().decode("utf-8"))
            rc_file.close()
