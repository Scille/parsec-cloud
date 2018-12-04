import os
import pathlib

from PyQt5.QtCore import QFileInfo, QCoreApplication
from PyQt5.QtWidgets import QWidget, QFileDialog

from parsec.core.gui import settings
from parsec.core.gui.custom_widgets import show_error, show_info, show_warning
from parsec.core.gui.ui.settings_widget import Ui_SettingsWidget


class SettingsWidget(QWidget, Ui_SettingsWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        mountpoint = settings.get_value("mountpoint", None)
        if mountpoint is None:
            mountpoint = os.path.join(str(pathlib.Path.home()), "parsec")
            settings.set_value("mountpoint", mountpoint)
        self.line_edit_mountpoint.setText(mountpoint)
        mountpoint_enabled = settings.get_value("mountpoint_enabled", None)
        if mountpoint_enabled is None:
            if os.name == "nt":
                settings.set_value("mountpoint_enabled", False)
                mountpoint_enabled = False
            else:
                settings.set_value("mountpoint_enabled", True)
                mountpoint_enabled = True
        if mountpoint_enabled:
            self.button_choose_mountpoint.setDisabled(False)
        else:
            self.button_choose_mountpoint.setDisabled(True)
        self.checkbox_enable_mountpoint.setChecked(mountpoint_enabled)
        self.button_choose_mountpoint.clicked.connect(self.choose_mountpoint)
        self.checkbox_enable_mountpoint.stateChanged.connect(self.enable_mountpoint)

    def enable_mountpoint(self, state):
        state = bool(state)
        if state and os.name == "nt":
            show_warning(
                self,
                QCoreApplication.translate(
                    "SettingsWidget", "Mountpoints are not handled on Windows right now."
                ),
            )
            return
        settings.set_value("mountpoint_enabled", state)
        if state:
            self.button_choose_mountpoint.setDisabled(False)
        else:
            self.button_choose_mountpoint.setDisabled(True)
        show_info(
            self,
            QCoreApplication.translate(
                "SettingsWidget", "You must log off and on again for the changes to take effect."
            ),
        )

    def choose_mountpoint(self):
        while True:
            path = QFileDialog.getExistingDirectory(
                self,
                QCoreApplication.translate("SettingsWidget", "Choose a mountpoint"),
                str(pathlib.Path.home()),
            )
            if not path:
                return None
            path_info = QFileInfo(path)
            if not path_info.isDir() or not path_info.isWritable():
                show_error(
                    self,
                    QCoreApplication.translate(
                        "SettingsWidget", "The choosen folder is not writable."
                    ),
                )
            else:
                self.line_edit_mountpoint.setText(path_info.absoluteFilePath())
                settings.set_value("mountpoint", path_info.absoluteFilePath())
                return path_info.absoluteFilePath()

    def reset(self):
        pass
