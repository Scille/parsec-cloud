# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtWidgets import QWidget
from PyQt5.QtGui import QIcon

from parsec.core.gui.custom_dialogs import show_info
from parsec.core.gui.lang import translate as _
from parsec.core.gui.global_settings_widget import GlobalSettingsWidget
from parsec.core.gui.about_widget import AboutWidget
from parsec.core.gui.ui.settings_widget import Ui_SettingsWidget


class SettingsWidget(QWidget, Ui_SettingsWidget):
    def __init__(self, config, jobs_ctx, event_bus, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.config = config
        self.event_bus = event_bus
        self.global_settings = GlobalSettingsWidget(config, jobs_ctx, self.event_bus)
        self.tab_settings.addTab(
            self.global_settings,
            QIcon(":/icons/images/icons/settings_on.png"),
            _("MENU_SETTINGS_GLOBAL"),
        )
        self.about_settings = AboutWidget()
        self.tab_settings.addTab(
            self.about_settings,
            QIcon(":/icons/images/icons/question.png"),
            _("MENU_SETTINGS_ABOUT"),
        )
        self.global_settings.save_clicked.connect(self.save)

    def save(self):
        self.global_settings.save()
        show_info(self, _("SETTINGS_NEED_RESTART"))
