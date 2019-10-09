# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtWidgets import QWidget

from parsec.core.gui.lang import translate as _

from parsec.core.gui.ui.about_widget import Ui_AboutWidget


class AboutWidget(QWidget, Ui_AboutWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.text_license.setHtml(_("PARSEC_LICENSE_CONTENT"))
        self.text_changelog.setHtml(_("PARSEC_CHANGELOG_CONTENT"))
