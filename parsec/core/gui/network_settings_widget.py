# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget
from PyQt5.QtGui import QIntValidator

from parsec.core.gui.validators import NetworkPortValidator
from parsec.core.gui.ui.network_settings_widget import Ui_NetworkSettingsWidget


class NetworkSettingsWidget(QWidget, Ui_NetworkSettingsWidget):
    save_clicked = pyqtSignal()

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.radio_upload_no_limit.toggled.connect(
            self.switch_network_limits(self.radio_upload_limit)
        )
        self.radio_upload_limit.toggled.connect(
            self.switch_network_limits(self.radio_upload_no_limit)
        )
        self.radio_download_no_limit.toggled.connect(
            self.switch_network_limits(self.radio_download_limit)
        )
        self.radio_download_limit.toggled.connect(
            self.switch_network_limits(self.radio_download_no_limit)
        )

        self.line_edit_proxy_port.setValidator(NetworkPortValidator())
        self.line_edit_upload_limit.setValidator(QIntValidator(1, 10000))
        self.line_edit_download_limit.setValidator(QIntValidator(1, 10000))
        self.combo_proxy_type.currentIndexChanged.connect(self.proxy_type_changed)
        self.button_save.clicked.connect(self.save_clicked)
        self.init()

    def init(self):
        pass

    def switch_network_limits(self, widget):
        def _inner_switch_network_limits(state):
            widget.setChecked(not state)

        return _inner_switch_network_limits

    def proxy_type_changed(self, current_type):
        if current_type == 0:
            self.widget_proxy_info.setDisabled(True)
        else:
            self.widget_proxy_info.setDisabled(False)

    def save(self):
        pass
