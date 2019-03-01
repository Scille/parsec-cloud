# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import Qt, pyqtSignal
from PyQt5.QtWidgets import QWidget

from parsec.core.gui import settings
from parsec.core.gui import lang
from parsec.core.gui.ui.global_settings_widget import Ui_GlobalSettingsWidget


class GlobalSettingsWidget(QWidget, Ui_GlobalSettingsWidget):
    save_clicked = pyqtSignal()

    def __init__(self, core_config, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.core_config = core_config
        self.setupUi(self)
        self.init()
        self.button_save.clicked.connect(self.save_clicked)

    def init(self):
        tray_enabled = settings.get_value("global/tray_enabled", None)
        if tray_enabled is None:
            settings.set_value("global/tray_enabled", True)
            tray_enabled = True
        self.checkbox_tray.setChecked(Qt.Checked if tray_enabled else Qt.Unchecked)
        current = None
        for lg, key in lang.LANGUAGES.items():
            self.combo_languages.addItem(lg, key)
            if key == settings.get_value("global/language"):
                current = lg
        if current:
            self.combo_languages.setCurrentText(current)

    def save(self):
        settings.set_value("global/tray_enabled", self.checkbox_tray.checkState() == Qt.Checked)
        settings.set_value("global/language", self.combo_languages.currentData())
