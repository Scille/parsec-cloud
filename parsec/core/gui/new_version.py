# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
from distutils.version import StrictVersion
from urllib.parse import urlsplit
import requests

from PyQt5.QtCore import Qt
from PyQt5.QtWidgets import QDialog

from parsec import __version__

from parsec.core.config import save_config
from parsec.core.gui import desktop
from parsec.core.gui.ui.new_version_dialog import Ui_NewVersionDialog


RELEASE_URL = "https://github.com/Scille/parsec-build/releases/latest"


class NewVersionDialog(QDialog, Ui_NewVersionDialog):
    def __init__(self, core_config, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.core_config = core_config
        self.button_download.clicked.connect(self.download)
        self.button_ignore.clicked.connect(self.ignore)
        self.setWindowFlags(Qt.SplashScreen)

    def download(self):
        desktop.open_url(RELEASE_URL)
        self.accept()

    def ignore(self):
        if self.check_box_no_reminder.isChecked():
            self.core_config = self.core_config.evolve(
                gui=self.core_config.evolve(gui_check_version_at_startup=False)
            )
            save_config(self.core_config)
        self.reject()


def new_version_available():
    if os.name != "nt":
        return False
    r = requests.head(RELEASE_URL)
    s = urlsplit(r.headers["LOCATION"])
    version = s.path.split("/")[-1:][0].split("-")[:1][0]
    new_version = version.replace("v", "")
    current_version = __version__.split("-")[0].replace("v", "")
    return StrictVersion(current_version) < StrictVersion(new_version)
