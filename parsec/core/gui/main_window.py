import os
import shutil

from PyQt5.QtCore import QCoreApplication
from PyQt5.QtWidgets import QMainWindow, QMessageBox, QSystemTrayIcon, QMenu
from PyQt5.QtGui import QIcon

from parsec.core.devices_manager import DeviceLoadingError, DeviceConfigureBackendError

from parsec.core.gui import settings
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

        self.close_requested = False
        self.setupUi(self)
        self.about_dialog = None
        self.files_widget = None
        self.settings_widget = None
        self.users_widget = None
        self.tray = None
        self.login_widget = LoginWidget(parent=self)
        for device_name in core_call().get_devices():
            self.login_widget.add_device(device_name)
        self.main_widget_layout.insertWidget(1, self.login_widget)
        self.users_widget = UsersWidget(parent=self)
        self.main_widget_layout.insertWidget(1, self.users_widget)
        self.users_widget.hide()
        self.settings_widget = SettingsWidget(parent=self)
        self.settings_widget.hide()
        self.main_widget_layout.insertWidget(1, self.settings_widget)
        self.files_widget = FilesWidget(parent=self)
        self.main_widget_layout.insertWidget(1, self.files_widget)
        self.files_widget.hide()
        self.current_device = None
        self.add_tray_icon()
        self.connect_all()

    def add_tray_icon(self):
        if not QSystemTrayIcon.isSystemTrayAvailable():
            self.action_put_in_tray.setDisabled(True)
            return
        self.tray = QSystemTrayIcon(self)
        menu = QMenu()
        action = menu.addAction(QCoreApplication.translate(self.__class__.__name__, "Show window"))
        action.triggered.connect(self.show)
        action = menu.addAction(QCoreApplication.translate(self.__class__.__name__, "Exit"))
        action.triggered.connect(self.close_app)
        self.tray.setContextMenu(menu)
        self.tray.setIcon(QIcon(":/icons/images/icons/parsec.png"))
        self.tray.show()

    def connect_all(self):
        self.action_about_parsec.triggered.connect(self.show_about_dialog)
        self.button_files.clicked.connect(self.show_files_widget)
        self.button_users.clicked.connect(self.show_users_widget)
        self.button_settings.clicked.connect(self.show_settings_widget)
        self.login_widget.loginClicked.connect(self.login)
        self.login_widget.claimClicked.connect(self.claim)
        self.action_disconnect.triggered.connect(self.logout)
        self.users_widget.registerClicked.connect(self.register)
        self.action_remount.triggered.connect(self.remount)
        self.action_login.triggered.connect(self.show_login_widget)
        self.action_quit.triggered.connect(self.close)

    def logout(self):
        self.files_widget.set_mountpoint("")
        if core_call().is_mounted():
            core_call().unmount()
        core_call().logout()
        self.login_widget.reset()
        self.users_widget.reset()
        self.files_widget.reset()
        self.show_login_widget()
        self.action_disconnect.setDisabled(True)
        self.button_files.setDisabled(True)
        self.button_users.setDisabled(True)
        self.action_disconnect.setDisabled(True)
        self.action_remount.setDisabled(True)
        self.action_login.setDisabled(False)
        device = core_call().load_device("johndoe@test")
        core_call().login(device)
        self.current_device = device

    def mount(self):
        base_mountpoint = settings.get_value("mountpoint")
        if not base_mountpoint:
            return None
        mountpoint = os.path.join(base_mountpoint, self.current_device.id)
        if core_call().is_mounted():
            core_call().unmount()
        try:
            if os.path.exists(mountpoint):
                shutil.rmtree(mountpoint)
            core_call().mount(mountpoint)
            self.files_widget.set_mountpoint(mountpoint)
            return mountpoint
        except (RuntimeError, PermissionError):
            return None

    def remount(self):
        mountpoint = self.mount()
        if mountpoint is None:
            QMessageBox.warning(
                self,
                QCoreApplication.translate(self.__class__.__name__, "Error"),
                QCoreApplication.translate(
                    self.__class__.__name__,
                    'Can not mount in "{}" (permissions problems ?). Go '
                    "to Settings/Global to a set mountpoint, then File/Remount to "
                    "mount it.".format(settings.get_value("mountpoint")),
                ),
            )
            self.show_settings_widget()
            return

        self.files_widget.reset()
        self.files_widget.set_mountpoint(mountpoint)
        self.button_files.setDisabled(False)
        self.show_files_widget()

    def perform_login(self, device_id, password):
        try:
            device = core_call().load_device(device_id, password)
            self.current_device = device
            core_call().logout()
            core_call().login(device)
            self.button_users.setDisabled(False)
            self.action_remount.setDisabled(False)
            self.action_disconnect.setDisabled(False)
            self.action_login.setDisabled(True)
            mountpoint = self.mount()
            if mountpoint is None:
                QMessageBox.warning(
                    self,
                    QCoreApplication.translate(self.__class__.__name__, "Error"),
                    QCoreApplication.translate(
                        self.__class__.__name__,
                        'Can not mount in "{}" (permissions problems ?). Go '
                        "to Settings/Global to a set mountpoint, then File/Remount to "
                        "mount it.".format(settings.get_value("mountpoint")),
                    ),
                )
                self.show_settings_widget()
                return
            self.button_files.setDisabled(False)
            self.show_files_widget()
        except DeviceLoadingError:
            return QCoreApplication.translate(self.__class__.__name__, "Invalid password")

    def login(self, device_id, password):
        err = self.perform_login(device_id, password)
        if err:
            self.login_widget.set_login_error(err)

    def register(self, login):
        try:
            token = core_call().invite_user(login)
            self.users_widget.set_claim_infos(login, token)
        except DeviceConfigureBackendError:
            self.users_widget.set_error(
                QCoreApplication.translate("MainWindow", "Can not register the new user.")
            )

    def claim(self, login, password, device, token):
        try:
            privkey, signkey, manifest = core_call().claim_user(login, device, token)
            privkey = privkey.encode()
            signkey = signkey.encode()
            full_device_name = "{}@{}".format(login, device)
            core_call().register_new_device(full_device_name, privkey, signkey, manifest, password)
            self.login_widget.add_device(full_device_name)
            err = self.perform_login(full_device_name, password)
            if err:
                self.login_widget.set_register_error(err)
        except DeviceLoadingError:
            pass

    def close_app(self):
        self.close_requested = True
        self.close()

    def closeEvent(self, event):
        if (
            not QSystemTrayIcon.isSystemTrayAvailable()
            or self.close_requested
            or core_call().is_debug()
        ):
            result = QMessageBox.question(
                self,
                QCoreApplication.translate(self.__class__.__name__, "Confirmation"),
                QCoreApplication.translate("MainWindow", "Are you sure you want to quit ?"),
            )
            if result != QMessageBox.Yes:
                event.ignore()
                return
            event.accept()
            if core_call().is_mounted():
                core_call().unmount()
                core_call().logout()
                core_call().stop()
        else:
            if self.tray:
                self.tray.showMessage(
                    "Parsec",
                    QCoreApplication.translate(self.__class__.__name__, "Parsec is still running."),
                )
            event.ignore()
            self.hide()

    def show_about_dialog(self):
        self.about_dialog = AboutDialog(parent=self)
        self.about_dialog.show()

    def show_files_widget(self):
        self._hide_all_central_widgets()
        self.button_files.setChecked(True)
        self.files_widget.show()

    def show_users_widget(self):
        self._hide_all_central_widgets()
        self.button_users.setChecked(True)
        self.users_widget.show()

    def show_settings_widget(self):
        self._hide_all_central_widgets()
        self.button_settings.setChecked(True)
        self.settings_widget.show()

    def show_login_widget(self):
        self._hide_all_central_widgets()
        self.login_widget.show()

    def _hide_all_central_widgets(self):
        self.files_widget.hide()
        self.users_widget.hide()
        self.settings_widget.hide()
        self.login_widget.hide()
        self.button_files.setChecked(False)
        self.button_users.setChecked(False)
        self.button_settings.setChecked(False)
