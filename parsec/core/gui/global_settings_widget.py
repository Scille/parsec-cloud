# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os

from PyQt5.QtCore import pyqtSignal, QCoreApplication
from PyQt5.QtWidgets import QWidget

from parsec.core.gui import settings
from parsec.core.gui import lang
from parsec.core.gui.custom_widgets import show_info
from parsec.core.gui.new_version import NewVersionDialog, new_version_available
from parsec.core.gui.ui.global_settings_widget import Ui_GlobalSettingsWidget


class GlobalSettingsWidget(QWidget, Ui_GlobalSettingsWidget):
    save_clicked = pyqtSignal()

    def __init__(self, core_config, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.core_config = core_config
        self.setupUi(self)
        self.init()
        if os.name != "nt":
            self.widget_version.hide()
        self.button_save.clicked.connect(self.save_clicked)
        self.button_check_version.clicked.connect(self.check_version)

    def check_version(self):
        if new_version_available():
            d = NewVersionDialog(parent=self)
            d.exec_()
        else:
            show_info(
                self,
                QCoreApplication.translate(
                    "GlobalSettings", "You have the most recent version of Parsec."
                ),
            )

    def init(self):
        tray_enabled = settings.get_value("global/tray_enabled", None)
        if tray_enabled is None:
            settings.set_value("global/tray_enabled", True)
            tray_enabled = True
        self.checkbox_tray.setChecked(tray_enabled)
        current = None
        for lg, key in lang.LANGUAGES.items():
            self.combo_languages.addItem(lg, key)
            if key == settings.get_value("global/language"):
                current = lg
        if current:
            self.combo_languages.setCurrentText(current)
        no_check_version = settings.get_value("global/no_check_version", "false")
        self.check_box_check_at_startup.setChecked(not no_check_version)

    def save(self):
        settings.set_value("global/tray_enabled", self.checkbox_tray.isChecked())
        settings.set_value("global/language", self.combo_languages.currentData())
        settings.set_value(
            "global/no_check_version", not self.check_box_check_at_startup.isChecked()
        )
