from PyQt5.QtCore import QCoreApplication
from PyQt5.QtWidgets import QMainWindow, QMessageBox

from parsec.core.gui import lang
from parsec.core.gui.home_widget import HomeWidget
from parsec.core.gui.files_widget import FilesWidget
from parsec.core.gui.users_widget import UsersWidget
from parsec.core.gui.settings_widget import SettingsWidget
from parsec.core.gui.about_dialog import AboutDialog
from parsec.core.gui.ui.main_window import Ui_MainWindow


class MainWindow(QMainWindow, Ui_MainWindow):
    def __init__(self, parsec_core, portal, cancel_scope, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self._parsec_core = parsec_core
        self._portal = portal
        self._cancel_scope = cancel_scope
        self.setupUi(self)
        self.about_dialog = None
        self.files_widget = None
        self.settings_widget = None
        self.users_widget = None
        self.home_widget = HomeWidget(parent=self)
        self.button_home.setChecked(True)
        self.main_widget_layout.addWidget(self.home_widget)
        self.label_title.setText(
            QCoreApplication.translate(
                'MainWindow','<span style="font-size:16pt;">Home</span> - Welcome to Parsec'))
        self.connect_all()

        self.init_language()

    def init_language():
        settings = QSettings()

        lang_key = settings.getValue('language')
        if not lang_key:
            if not lang.switch_to_locale():
                QMessageBox.information(
                    self,
                    lang.translate(self, 'Information'),
                    lang.translate(
                        self,
                        'Parsec is not available in your system language, '
                        'so it will use English by default. You can change this '
                        'setting in Settings/Global/Languages.'))
            else:
                settings.setValue('language', lang.get_current_language_key())
                settings.sync()
        elif lang_key and lang_key != lang.get_current_language_key():
            if not lang.switch_to_language_key(lang_key):
                QMessageBox.information(
                    self,
                    lang.translate(self, 'Information'),
                    lang.translate(
                        self,
                        'We tried to switch Parsec to language {} but it failed. '
                        'We will use English by default. You can change this setting '
                        'in Settings/Global/Languages.'.format(
                            lang.get_language_name_from_key(lang_key))))

    def connect_all(self):
        self.action_about_parsec.triggered.connect(self.show_about_dialog)
        self.button_home.clicked.connect(self.show_home_widget)
        self.button_files.clicked.connect(self.show_files_widget)
        self.button_users.clicked.connect(self.show_users_widget)
        self.button_settings.clicked.connect(self.show_settings_widget)

    def closeEvent(self, event):
        self._portal.run_sync(self._cancel_scope.cancel)

    def show_about_dialog(self):
        self.about_dialog = AboutDialog(parent=self)
        self.about_dialog.show()

    def show_home_widget(self):
        if not self.home_widget:
            self.home_widget = HomeWidget(parent=self)
            self.main_widget_layout.addWidget(self.home_widget)
        self._hide_all_central_widgets()
        self.label_title.setText(
            lang.translate(
                self,
                '<span style="font-size:16pt;">Home</span> - Welcome to Parsec'))
        self.button_home.setChecked(True)
        self.home_widget.show()

    def show_files_widget(self):
        if not self.files_widget:
            self.files_widget = FilesWidget(parent=self)
            self.main_widget_layout.addWidget(self.files_widget)
        self._hide_all_central_widgets()
        self.label_title.setText(
            lang.translate(
                self,
                '<span style="font-size:16pt;">Files</span> - Manage your files'))
        self.button_files.setChecked(True)
        self.files_widget.show()

    def show_users_widget(self):
        if not self.users_widget:
            self.users_widget = UsersWidget(parent=self)
            self.main_widget_layout.addWidget(self.users_widget)
        self._hide_all_central_widgets()
        self.label_title.setText(
            lang.translate(
                self,
                '<span style="font-size:16pt;">Users</span> - Manage the users'))
        self.button_users.setChecked(True)
        self.users_widget.show()

    def show_settings_widget(self):
        if not self.settings_widget:
            self.settings_widget = SettingsWidget(parent=self)
            self.main_widget_layout.addWidget(self.settings_widget)
        self._hide_all_central_widgets()
        self.label_title.setText(
            lang.translate(
                self,
                '<span style="font-size:16pt;">Settings</span> - Configure Parsec'))
        self.button_settings.setChecked(True)
        self.settings_widget.show()

    def _hide_all_central_widgets(self):
        if self.home_widget:
            self.home_widget.hide()
        if self.files_widget:
            self.files_widget.hide()
        if self.users_widget:
            self.users_widget.hide()
        if self.settings_widget:
            self.settings_widget.hide()
        self.button_home.setChecked(False)
        self.button_files.setChecked(False)
        self.button_users.setChecked(False)
        self.button_settings.setChecked(False)
