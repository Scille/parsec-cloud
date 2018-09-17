import os
import pathlib

from PyQt5.QtCore import QFileInfo, QCoreApplication
from PyQt5.QtWidgets import QWidget, QFileDialog, QMessageBox

from parsec.core.gui import settings
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

    def choose_mountpoint(self):
        while True:
            path = QFileDialog.getExistingDirectory(
                self, "Choose a mountpoint", str(pathlib.Path.home())
            )
            if not path:
                return None
            path_info = QFileInfo(path)
            if not path_info.isDir() or not path_info.isWritable():
                QMessageBox.warning(
                    self,
                    QCoreApplication.translate("SettingsWidget", "Error"),
                    QCoreApplication.translate(
                        "SettingsWidget", "The choosen folder is not writable."
                    ),
                )
            else:
                self.line_edit_mountpoint.setText(path_info.absoluteFilePath())
                settings.set_value("mountpoint", path_info.absoluteFilePath())
                return path_info.absoluteFilePath()
