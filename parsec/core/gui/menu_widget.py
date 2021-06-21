# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtGui import QPainter
from PyQt5.QtWidgets import QWidget, QStyle, QStyleOption

from parsec.core.gui.ui.menu_widget import Ui_MenuWidget
from parsec.core.gui.lang import translate as _
from parsec.core.gui import file_size
from parsec.core.types import OrganizationStats


class MenuWidget(QWidget, Ui_MenuWidget):
    files_clicked = pyqtSignal()
    users_clicked = pyqtSignal()
    devices_clicked = pyqtSignal()

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.button_files.clicked.connect(self.files_clicked.emit)
        self.button_users.clicked.connect(self.users_clicked.emit)
        self.button_devices.clicked.connect(self.devices_clicked.emit)
        self.button_files.apply_style()
        self.button_users.apply_style()
        self.button_devices.apply_style()
        self.icon_connection.apply_style()

    def paintEvent(self, _):
        opt = QStyleOption()
        opt.initFrom(self)
        p = QPainter(self)
        self.style().drawPrimitive(QStyle.PE_Widget, opt, p, self)

    def activate_files(self):
        self._deactivate_all()
        self.button_files.setChecked(True)

    def activate_devices(self):
        self._deactivate_all()
        self.button_devices.setChecked(True)

    def activate_users(self):
        self._deactivate_all()
        self.button_users.setChecked(True)

    def _deactivate_all(self):
        self.button_files.setChecked(False)
        self.button_users.setChecked(False)
        self.button_devices.setChecked(False)

    def show_organization_stats(self, organization_id: str, organization_stats: OrganizationStats):
        self.label_organization_name.show()
        self.label_organization_size.show()
        self.label_organization_name.setText(organization_id)
        total = file_size.get_filesize(
            organization_stats.metadata_size + organization_stats.data_size
        )
        self.label_organization_size.setText(
            _("TEXT_ORGANIZATION_SIZE_organizationsize").format(organizationsize=total)
        )
        self.label_organization_size.setToolTip(
            _("TEXT_ORGANIZATION_SIZE_TOOLTIP_datasize-metadatasize").format(
                datasize=file_size.get_filesize(organization_stats.data_size),
                metadatasize=file_size.get_filesize(organization_stats.metadata_size),
            )
        )

    def set_connection_state(self, text, tooltip, icon):
        self.label_connection_state.setText(text)
        self.label_connection_state.setToolTip(tooltip)
        self.icon_connection.setPixmap(icon)
        self.icon_connection.apply_style()
