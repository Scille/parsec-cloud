from PyQt5.QtWidgets import QDialog, QVBoxLayout

from parsec.core.gui.settings_widget import SettingsWidget
from parsec.core.gui.ui.settings_dialog import Ui_SettingsDialog


class SettingsDialog(QDialog, Ui_SettingsDialog):
    def __init__(self, core_config, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        s = SettingsWidget(core_config)
        s.layout().setContentsMargins(0, 0, 0, 0)
        self.layout().insertWidget(0, s)
        self.button_close.clicked.connect(self.accept)
