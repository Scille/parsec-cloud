import os
import pathlib

from PyQt5.QtCore import QFileInfo, QCoreApplication
from PyQt5.QtWidgets import QWidget, QFileDialog

from parsec.core.gui import settings
from parsec.core.gui.custom_widgets import show_error
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
        self.button_choose_mountpoint.clicked.connect(self.choose_mountpoint)

        mountpoint_enabled = settings.get_value("mountpoint_enabled", None)
        if mountpoint_enabled is None:
            settings.set_value("mountpoint_enabled", True)
            mountpoint_enabled = True
        self.checkbox_enable_mountpoint.setChecked(mountpoint_enabled)
        self.checkbox_enable_mountpoint.stateChanged.connect(self.enable_mountpoint)

        self.notice_mountpoint.setEnabled(True)
        self.notice_mountpoint.hide()

    def enable_mountpoint(self, state):
        state = bool(state)
        settings.set_value("mountpoint_enabled", state)
        self.notice_mountpoint.show()

    def choose_mountpoint(self):
        self.notice_mountpoint.show()
        while True:
            path = QFileDialog.getExistingDirectory(
                self, "Choose a mountpoint", str(pathlib.Path.home())
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
