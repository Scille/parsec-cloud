from PyQt5.QtCore import QCoreApplication, QSettings
from PyQt5.QtWidgets import QWidget, QMessageBox

from parsec.core.gui import lang
from parsec.core.gui.ui.settings_widget import Ui_SettingsWidget


class SettingsWidget(QWidget, Ui_SettingsWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.setupUi(self)

        self.combo_languages.addItem('English')
        self.combo_languages.addItem('Français')
        self.combo_languages.addItem('Deutsch')
        self.combo_languages.addItem('Español')

        self.button_switch_language.clicked.connect(self.switch_language)
        self.combo_languages.currentTextChanged.connect(self.deactivate_lang_button)

        if self.combo_languages.currentText == lang.get_current_language_name():
            self.button_switch_language.setDisabled(True)
        self.label_current_language.setText(lang.get_current_language_name())

    def deactivate_lang_button(self, text):
        self.button_switch_language.setDisabled(lang.get_current_language_name == text)

    def switch_language(self):
        if not lang.switch_to_language(self.combo_languages.currentText()):
            QMessageBox.information(
                self,
                lang.translate(self, 'Information'),
                lang.translate(
                    self,
                    'We could not switch to the selected language. Sorry ! :('))
        else:
            settings = QSettings()
            settings.setValue('language', lang.get_current_language_name)
            settings.sync()
            self.label_current_language.setText(lang.get_current_language_name())
