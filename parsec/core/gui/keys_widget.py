# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import shutil
import os
from pathlib import Path

from PyQt5.QtCore import pyqtSignal
from PyQt5.QtWidgets import QWidget, QFileDialog

from parsec.core.local_device import (
    list_available_devices,
    AvailableDevice,
    load_device_file,
    get_devices_dir,
)

from parsec.core.gui.ui.keys_widget import Ui_KeysWidget
from parsec.core.gui.ui.key_widget import Ui_KeyWidget
from parsec.core.gui.custom_dialogs import show_error, ask_question
from parsec.core.gui.lang import translate


class KeyWidget(QWidget, Ui_KeyWidget):
    export_clicked = pyqtSignal(AvailableDevice)

    def __init__(self, device, parent=None):
        super().__init__(parent=parent)
        self.device = device
        self.setupUi(self)
        self.label_org.setText(device.organization_id)
        self.label_device.setText(device.device_label)
        self.label_user.setText(device.human_handle.label)
        self.export_button.clicked.connect(self._on_export)

    def _on_export(self):
        self.export_clicked.emit(self.device)


class KeysWidget(QWidget, Ui_KeysWidget):
    key_imported = pyqtSignal()

    def __init__(self, config, parent):
        super().__init__(parent=parent)
        self.config = config
        self.setupUi(self)
        self.reload_devices()
        self.button_import_key.clicked.connect(self._on_import_key)

    def reload_devices(self):
        layout = self.scroll_content.layout()
        for _ in range(layout.count()):
            item = self.scroll_content.layout().takeAt(0)
            layout.removeItem(item)
            w = item.widget()
            if w is not None:
                w.setParent(None)
        devices = list_available_devices(self.config.config_dir)
        for device in devices:
            w = KeyWidget(device, parent=self)
            w.export_clicked.connect(self._on_export_key)
            self.scroll_content.layout().insertWidget(self.scroll_content.layout().count() - 1, w)

    def _on_export_key(self, device):
        output_directory = QFileDialog.getExistingDirectory()
        if not output_directory:
            return
        _, key_name = os.path.split(device.key_file_path)
        try:
            shutil.copyfile(device.key_file_path, os.path.join(output_directory, key_name))
        except IOError as err:
            show_error(self, translate("EXPORT_KEY_ERROR"), err)

    def _on_import_key(self):
        key_file, _ = QFileDialog.getOpenFileName(
            parent=self,
            caption=translate("ACTION_IMPORT_KEY"),
            filter=translate("IMPORT_KEY_FILTERS"),
            initialFilter=translate("IMPORT_KEY_INITIAL_FILTER"),
        )
        if not key_file:
            return
        new_device = load_device_file(Path(key_file))
        if new_device is None:
            show_error(self, translate("TEXT_INVALID_DEVICE_KEY"))
            return
        rep = ask_question(
            parent=self,
            title=translate("ASK_IMPORT_KEY"),
            message=(
                f"{new_device.organization_id}<br>{new_device.human_handle.label}<br>{new_device.device_label}"
            ),
            button_texts=(translate("ACTION_IMPORT_YES"), translate("ACTION_IMPORT_NO")),
        )
        if rep == translate("ACTION_IMPORT_YES"):
            _, key_name = os.path.split(new_device.key_file_path)
            shutil.copyfile(
                new_device.key_file_path,
                os.path.join(get_devices_dir(self.config.config_dir), key_name),
            )
            self.reload_devices()
            self.key_imported.emit()
