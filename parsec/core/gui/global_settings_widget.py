import os
import pathlib

from PyQt5.QtCore import Qt, QCoreApplication, QFileInfo, pyqtSignal
from PyQt5.QtWidgets import QWidget, QFileDialog

from parsec.core.gui import settings
from parsec.core.gui import lang
from parsec.core.gui.custom_widgets import show_error
from parsec.core.gui.ui.global_settings_widget import Ui_GlobalSettingsWidget


class GlobalSettingsWidget(QWidget, Ui_GlobalSettingsWidget):
    save_clicked = pyqtSignal()

    def __init__(self, core_config, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.core_config = core_config
        self.setupUi(self)
        self.init()
        self.button_choose_mountpoint.clicked.connect(self.choose_mountpoint)
        self.checkbox_enable_mountpoint.stateChanged.connect(self.enable_mountpoint)
        self.button_save.clicked.connect(self.save_clicked)

    def init(self):
        mountpoint = self.core_config.mountpoint_base_dir
        settings.set_value("global/mountpoint", str(mountpoint))
        self.line_edit_mountpoint.setText(str(mountpoint))
        mountpoint_enabled = settings.get_value("global/mountpoint_enabled", None)
        if os.name == "nt":
            settings.set_value("global/mountpoint_enabled", False)
            mountpoint_enabled = False
            self.checkbox_enable_mountpoint.setDisabled(True)
        elif mountpoint_enabled is None:
            settings.set_value("global/mountpoint_enabled", True)
            mountpoint_enabled = True

        self.button_choose_mountpoint.setDisabled(not mountpoint_enabled)
        self.checkbox_enable_mountpoint.setChecked(
            Qt.Checked if mountpoint_enabled else Qt.Unchecked
        )
        tray_enabled = settings.get_value("global/tray_enabled", None)
        if tray_enabled is None:
            settings.set_value("global/tray_enabled", True)
            tray_enabled = True
        self.checkbox_tray.setChecked(Qt.Checked if tray_enabled else Qt.Unchecked)
        current = None
        for lg, key in lang.LANGUAGES.items():
            self.combo_languages.addItem(lg, key)
            if key == settings.get_value("global/language"):
                current = lg
        if current:
            self.combo_languages.setCurrentText(current)

    def enable_mountpoint(self, state):
        self.button_choose_mountpoint.setEnabled(state == Qt.Checked)

    def save(self):
        settings.set_value("global/tray_enabled", self.checkbox_tray.checkState() == Qt.Checked)
        settings.set_value("global/language", self.combo_languages.currentData())
        settings.set_value("global/mountpoint", self.line_edit_mountpoint.text())
        settings.set_value(
            "global/mountpoint_enabled", self.checkbox_enable_mountpoint.checkState() == Qt.Checked
        )

    def choose_mountpoint(self):
        while True:
            path = QFileDialog.getExistingDirectory(
                self,
                QCoreApplication.translate("GlobalSettingsWidget", "Choose a mountpoint"),
                str(pathlib.Path.home()),
            )
            if not path:
                return
            path_info = QFileInfo(path)
            if not path_info.isDir() or not path_info.isWritable():
                show_error(
                    self,
                    QCoreApplication.translate(
                        "GlobalSettingsWidget", "The choosen folder is not writable."
                    ),
                )
            else:
                self.line_edit_mountpoint.setText(path_info.absoluteFilePath())
                return
