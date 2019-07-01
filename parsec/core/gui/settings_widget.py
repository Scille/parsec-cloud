# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtWidgets import QWidget
from PyQt5.QtGui import QIcon

from parsec.core.gui.custom_dialogs import show_info
from parsec.core.gui.lang import translate as _
from parsec.core.gui.global_settings_widget import GlobalSettingsWidget
from parsec.core.gui.ui.settings_widget import Ui_SettingsWidget


class SettingsWidget(QWidget, Ui_SettingsWidget):
    def __init__(self, config, event_bus, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.config = config
        self.event_bus = event_bus
        self.global_settings = GlobalSettingsWidget(config, self.event_bus)
        self.tab_settings.addTab(
            self.global_settings,
            QIcon(":/icons/images/icons/settings_on.png"),
            _("MENU_SETTINGS_GLOBAL"),
        )
        # self.network_settings = NetworkSettingsWidget()
        # self.tab_settings.addTab(
        #     self.network_settings, QIcon(":/icons/images/icons/wifi_on.png"), _("Network")
        # )
        self.global_settings.save_clicked.connect(self.save)
        # self.network_settings.save_clicked.connect(self.save)

    def disconnect_all(self):
        pass

    def save(self):
        self.global_settings.save()
        # self.network_settings.save()
        show_info(self, _("SETTINGS_NEED_RESTART"))
