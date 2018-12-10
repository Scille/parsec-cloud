from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget
from PyQt5.QtGui import QIntValidator

from parsec.core.gui import settings
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

        self.line_edit_proxy_port.setValidator(QIntValidator(1, 65536))
        self.line_edit_upload_limit.setValidator(QIntValidator(1, 10000))
        self.line_edit_download_limit.setValidator(QIntValidator(1, 10000))
        self.combo_proxy_type.currentIndexChanged.connect(self.proxy_type_changed)
        self.button_save.clicked.connect(self.save_clicked)
        self.init()

    def init(self):
        proxy_type = settings.get_value("network/proxy_type", None)
        if proxy_type is None:
            self.combo_proxy_type.setCurrentIndex(0)
        else:
            self.combo_proxy_type.setCurrentText(proxy_type)
            self.line_edit_proxy_host.setText(settings.get_value("network/proxy_host", ""))
            self.line_edit_proxy_port.setText(str(settings.get_value("network/proxy_port", 8080)))
            self.line_edit_proxy_username.setText(settings.get_value("network/proxy_username", ""))
        if self.combo_proxy_type.currentIndex() == 0:
            self.widget_proxy_info.setDisabled(True)
        if settings.get_value("network/upload_limit", -1) != -1:
            self.radio_upload_limit.setChecked(True)
            self.line_edit_upload_limit.setText(str(settings.get_value("network/upload_limit", 10)))
        if settings.get_value("network/download_limit", -1) != -1:
            self.radio_download_limit.setChecked(True)
            self.line_edit_download_limit.setText(
                str(settings.get_value("network/download_limit", 10))
            )

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
        if self.combo_proxy_type.currentIndex() == 0:
            settings.set_value("network/proxy_type", "")
        if self.combo_proxy_type.currentIndex() != 0:
            settings.set_value("network/proxy_type", self.combo_proxy_type.currentText())
        settings.set_value("network/proxy_host", self.line_edit_proxy_host.text())
        settings.set_value("network/proxy_port", int(self.line_edit_proxy_port.text()))
        settings.set_value("network/proxy_username", self.line_edit_proxy_username.text())
        if self.radio_upload_no_limit.isChecked():
            settings.set_value("network/upload_limit", -1)
        else:
            settings.set_value("network/upload_limit", int(self.line_edit_upload_limit.text()))
        if self.radio_download_no_limit.isChecked():
            settings.set_value("network/download_limit", -1)
        else:
            settings.set_value("network/download_limit", int(self.line_edit_download_limit.text()))
