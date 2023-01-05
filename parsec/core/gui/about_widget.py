# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Any

from PyQt5.QtWidgets import QWidget

from parsec import __version__
from parsec.core.gui.lang import translate as _
from parsec.core.gui.ui.about_widget import Ui_AboutWidget


class AboutWidget(QWidget, Ui_AboutWidget):
    def __init__(self, *args: Any, **kwargs: Any) -> None:
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.label_version.setText(_("TEXT_PARSEC_VERSION_version").format(version=__version__))
