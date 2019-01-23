from PyQt5.QtCore import QCoreApplication
from PyQt5.QtGui import QIcon

from parsec.core.gui.custom_widgets import show_info
from parsec.core.gui.core_widget import CoreWidget
from parsec.core.gui.global_settings_widget import GlobalSettingsWidget
from parsec.core.gui.network_settings_widget import NetworkSettingsWidget
from parsec.core.gui.ui.settings_widget import Ui_SettingsWidget


class SettingsWidget(CoreWidget, Ui_SettingsWidget):
    def __init__(self, core_config, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.core_config = core_config
        self.global_settings = GlobalSettingsWidget(core_config)
        self.tab_settings.addTab(
            self.global_settings,
            QIcon(":/icons/images/icons/settings_on.png"),
            QCoreApplication.translate("SettingsWidget", "Global"),
        )
        self.network_settings = NetworkSettingsWidget()
        self.tab_settings.addTab(
            self.network_settings,
            QIcon(":/icons/images/icons/wifi_on.png"),
            QCoreApplication.translate("SettingsWidget", "Network"),
        )
        self.global_settings.save_clicked.connect(self.save)
        self.network_settings.save_clicked.connect(self.save)

    def save(self):
        self.global_settings.save()
        self.network_settings.save()
        show_info(
            self,
            QCoreApplication.translate(
                "SettingsWidget",
                "Modification will take effect the next time you start the application.",
            ),
        )

    def reset(self):
        pass
