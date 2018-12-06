import os
import pathlib

from PyQt5.QtCore import QFileInfo, QCoreApplication, Qt
from PyQt5.QtWidgets import QWidget, QFileDialog
from PyQt5.QtGui import QIntValidator

from parsec.core.gui import settings
from parsec.core.gui import lang
from parsec.core.gui.custom_widgets import show_error, show_info
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
        if os.name == "nt":
            settings.set_value("mountpoint_enabled", False)
            mountpoint_enabled = False
            self.checkbox_enable_mountpoint.setDisabled(True)
        elif mountpoint_enabled is None:
            settings.set_value("mountpoint_enabled", True)
            mountpoint_enabled = True

        self.button_choose_mountpoint.setDisabled(not mountpoint_enabled)
        self.checkbox_enable_mountpoint.setChecked(
            Qt.Checked if mountpoint_enabled else Qt.Unchecked
        )

        tray_enabled = settings.get_value("tray_enabled", None)
        if tray_enabled is None:
            settings.set_value("tray_enabled", True)
            tray_enabled = True
        self.checkbox_tray.setChecked(Qt.Checked if tray_enabled else Qt.Unchecked)

        self.button_choose_mountpoint.clicked.connect(self.choose_mountpoint)
        self.checkbox_enable_mountpoint.stateChanged.connect(self.enable_mountpoint)
        self.line_edit_proxy_port.setValidator(QIntValidator(1, 65536))
        self.combo_proxy_type.currentIndexChanged.connect(self.proxy_type_changed)
        if self.combo_proxy_type.currentIndex() == 0:
            self.widget_proxy_info.setDisabled(True)
        current = None
        for lg, key in lang.LANGUAGES.items():
            self.combo_languages.addItem(lg, key)
            if key == settings.get_value("language"):
                current = lg
        if current:
            self.combo_languages.setCurrentText(current)
        self.combo_languages.currentTextChanged.connect(self.current_language_changed)
        self.button_apply_language.clicked.connect(self.apply_language)

    def apply_language(self):
        ret = lang.switch_language(lang_key=self.combo_languages.currentData())
        if not ret:
            show_error(
                self,
                QCoreApplication.translate(
                    "SettingsWidget",
                    "Could not switch the language to {}".format(
                        self.combo_languages.currentText()
                    ),
                ),
            )
        else:
            self.button_apply_language.setDisabled(True)

    def current_language_changed(self, _):
        if self.combo_languages.currentData() != settings.get_value("language"):
            self.button_apply_language.setDisabled(False)
        else:
            self.button_apply_language.setDisabled(True)

    def proxy_type_changed(self, current_type):
        if current_type == 0:
            self.widget_proxy_info.setDisabled(True)
        else:
            self.widget_proxy_info.setDisabled(False)

    def enable_mountpoint(self, state):
        state = state == Qt.Checked
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
