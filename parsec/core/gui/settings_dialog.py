# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from PyQt5.QtWidgets import QDialog

from parsec.core.gui.settings_widget import SettingsWidget
from parsec.core.gui.ui.settings_dialog import Ui_SettingsDialog


class SettingsDialog(QDialog, Ui_SettingsDialog):
    def __init__(self, config, jobs_ctx, event_bus, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.event_bus = event_bus
        s = SettingsWidget(config, jobs_ctx, self.event_bus)
        s.layout().setContentsMargins(0, 0, 0, 0)
        self.layout().insertWidget(0, s)
        self.button_close.clicked.connect(self.accept)
