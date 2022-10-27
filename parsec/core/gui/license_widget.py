# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations
from typing import Optional

from PyQt5.QtWidgets import QWidget

from parsec.core.gui.lang import translate as _
from parsec.core.gui.ui.license_widget import Ui_LicenseWidget


class LicenseWidget(QWidget, Ui_LicenseWidget):
    def __init__(self, parent: Optional[QWidget] = None) -> None:
        super().__init__(parent)
        self.setupUi(self)
        self.text_license.setHtml(_("TEXT_PARSEC_LICENSE"))
