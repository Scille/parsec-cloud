from PyQt5.QtCore import QCoreApplication
from PyQt5.QtWidgets import QMainWindow

from parsec.core.gui.core_call import core_call
from parsec.core.gui.login_widget import LoginWidget
from parsec.core.gui.files_widget import FilesWidget
from parsec.core.gui.users_widget import UsersWidget
from parsec.core.gui.settings_widget import SettingsWidget
from parsec.core.gui.about_dialog import AboutDialog
from parsec.core.gui.ui.main_window import Ui_MainWindow


class MainWindow(QMainWindow, Ui_MainWindow):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.setupUi(self)
        self.about_dialog = None
        self.files_widget = None
        self.settings_widget = None
        self.users_widget = None
        self.login_widget = LoginWidget(parent=self)
        self.main_widget_layout.insertWidget(1, self.login_widget)
        self.label_title.setText(
            QCoreApplication.translate(
                self.__class__.__name__,
                '<span style="font-size:16pt;">Home</span>'
                ' - Welcome to Parsec'))
        self.connect_all()

    def logged_in(self):
        self.button_files.setDisabled(False)
        self.button_users.setDisabled(False)
        self.button_settings.setDisabled(False)

    def connect_all(self):
        self.action_about_parsec.triggered.connect(self.show_about_dialog)
        self.button_files.clicked.connect(self.show_files_widget)
        self.button_users.clicked.connect(self.show_users_widget)
        self.button_settings.clicked.connect(self.show_settings_widget)
        self.login_widget.loginClicked.connect(self.login)
        self.login_widget.registerClicked.connect(self.register)

    def login(self):
        import os
        import shutil

        device_name = core_call().get_devices()[0]
        device = core_call().load_device(device_name)
        core_call().login(device)
        if os.path.exists('/home/max/osef'):
            shutil.rmtree('/home/max/osef')
        core_call().mount('/home/max/osef')
        self.logged_in()
        self.login_widget.hide()
        self.show_files_widget()

    def register(self, login, password, device):
        token = core_call().invite_user(login)
        privkey, signkey, manifest = core_call().claim_user(login, device, token)
        privkey = privkey.encode()
        signkey = signkey.encode()
        core_call().register_new_device('{}@{}'.format(login, device),
                                        privkey, signkey, manifest, password)
        core_call().load_device('{}@{}'.format(login, device), password)

    def closeEvent(self, event):
        if core_call().is_mounted():
            core_call().unmount()
        core_call().stop()

    def show_about_dialog(self):
        self.about_dialog = AboutDialog(parent=self)
        self.about_dialog.show()

    def show_files_widget(self):
        if not self.files_widget:
            self.files_widget = FilesWidget(parent=self)
            self.main_widget_layout.insertWidget(1, self.files_widget)
        self._hide_all_central_widgets()
        self.label_title.setText(
            QCoreApplication.translate(
                self.__class__.__name__,
                '<span style="font-size:16pt;">Files'
                '</span> - Manage your files'))
        self.button_files.setChecked(True)
        self.files_widget.show()

    def show_users_widget(self):
        if not self.users_widget:
            self.users_widget = UsersWidget(parent=self)
            self.main_widget_layout.insertWidget(1, self.users_widget)
        self._hide_all_central_widgets()
        self.label_title.setText(
            QCoreApplication.translate(
                self.__class__.__name__,
                '<span style="font-size:16pt;">Users'
                '</span> - Manage the users'))
        self.button_users.setChecked(True)
        self.users_widget.show()

    def show_settings_widget(self):
        if not self.settings_widget:
            self.settings_widget = SettingsWidget(parent=self)
            self.main_widget_layout.insertWidget(1, self.settings_widget)
        self._hide_all_central_widgets()
        self.label_title.setText(
            QCoreApplication.translate(
                self.__class__.__name__,
                '<span style="font-size:16pt;">'
                'Settings</span> - Configure Parsec'))
        self.button_settings.setChecked(True)
        self.settings_widget.show()

    def _hide_all_central_widgets(self):
        if self.files_widget:
            self.files_widget.hide()
        if self.users_widget:
            self.users_widget.hide()
        if self.settings_widget:
            self.settings_widget.hide()
        self.button_files.setChecked(False)
        self.button_users.setChecked(False)
        self.button_settings.setChecked(False)
