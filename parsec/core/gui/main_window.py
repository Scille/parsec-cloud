import os

from PyQt5.QtCore import QCoreApplication
from PyQt5.QtWidgets import QMainWindow, QSystemTrayIcon, QMenu, QMessageBox
from PyQt5.QtGui import QIcon

from parsec.core.devices_manager import (
    DeviceLoadingError,
    DeviceConfigureBackendError,
    DeviceConfigureOutOfDate,
    DeviceConfigureNoInvitation,
    DeviceConfigureAlreadyExists,
)

from parsec.core.gui import settings
from parsec.core.gui.core_call import core_call
from parsec.core.gui.custom_widgets import show_error, show_info
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
        self.tray.activated.connect(self.tray_activated)
        self.tray.show()

    def connect_all(self):
        self.action_about_parsec.triggered.connect(self.show_about_dialog)
        self.button_files.clicked.connect(self.show_files_widget)
        self.button_users.clicked.connect(self.show_users_widget)
        self.button_settings.clicked.connect(self.show_settings_widget)
        self.login_widget.login_with_password_clicked.connect(self.login_with_password)
        self.login_widget.login_with_nitrokey_clicked.connect(self.login_with_nitrokey)
        self.login_widget.register_user_with_password_clicked.connect(
            self.register_user_with_password
        )
        self.login_widget.register_user_with_nitrokey_clicked.connect(
            self.register_user_with_nitrokey
        )
        self.login_widget.register_device_with_password_clicked.connect(
            self.register_device_with_password
        )
        self.login_widget.register_device_with_nitrokey_clicked.connect(
            self.register_device_with_nitrokey
        )
        self.action_disconnect.triggered.connect(self.logout)
        self.users_widget.registerUserClicked.connect(self.register_user)
        self.action_remount.triggered.connect(self.remount)
        self.action_login.triggered.connect(self.show_login_widget)
        self.action_quit.triggered.connect(self.close)

    def tray_activated(self, reason):
        if reason == QSystemTrayIcon.DoubleClick:
            self.show()
            self.raise_()

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
            core_call().mount(mountpoint)
            self.files_widget.set_mountpoint(mountpoint)
            return mountpoint
        except (RuntimeError, PermissionError) as exc:
            import traceback

            traceback.print_exc(exc)
            return None

    def remount(self):
        mountpoint = self.mount()
        if mountpoint is None:
            show_error(
                self,
                QCoreApplication.translate(
                    "MainWindow",
                    'Can not mount in "{}" (permissions problems ?). Go '
                    "to Settings/Global to a set mountpoint, then File/Remount to "
                    "mount it.",
                ).format(settings.get_value("mountpoint")),
            )
            self.show_settings_widget()
            return

        self.files_widget.reset()
        self.files_widget.set_mountpoint(mountpoint)
        self.button_files.setDisabled(False)
        self.show_files_widget()

    def perform_login(
        self, device_id, password=None, nitrokey_pin=None, nitrokey_key=None, nitrokey_token=None
    ):
        try:
            device = core_call().load_device(
                device_id,
                password=password,
                nitrokey_pin=nitrokey_pin,
                nitrokey_token_id=nitrokey_token,
                nitrokey_key_id=nitrokey_key,
            )
            self.current_device = device
            core_call().logout()
            core_call().login(device)
            self.button_users.setDisabled(False)
            self.action_remount.setDisabled(False)
            self.action_disconnect.setDisabled(False)
            self.action_login.setDisabled(True)
            mountpoint = self.mount()
            if mountpoint is None:
                show_error(
                    self,
                    QCoreApplication.translate(
                        "MainWindow",
                        'Can not mount in "{}" (permissions problems ?). Go '
                        "to Settings/Global to a set mountpoint, then File/Remount to "
                        "mount it.",
                    ).format(settings.get_value("mountpoint")),
                )
                self.show_settings_widget()
                return
            self.button_files.setDisabled(False)
            self.show_files_widget()
        except DeviceLoadingError:
            return QCoreApplication.translate(self.__class__.__name__, "Invalid password")

    def login_with_password(self, device_id, password):
        err = self.perform_login(device_id, password=password)
        if err:
            show_error(self.login_widget, err)

    def login_with_nitrokey(self, device_id, nitrokey_pin, nitrokey_key, nitrokey_token):
        err = self.perform_login(
            device_id,
            nitrokey_pin=nitrokey_pin,
            nitrokey_key=nitrokey_key,
            nitrokey_token=nitrokey_token,
        )
        if err:
            show_error(self.login_widget, err)

    def register_user(self, login):
        try:
            token = core_call().invite_user(login)
            self.users_widget.set_claim_infos(login, token)
        except DeviceConfigureBackendError:
            self.users_widget.set_error(
                QCoreApplication.translate("MainWindow", "Can not register the new user.")
            )

    def handle_register_user(
        self,
        user_id,
        device_name,
        token,
        use_nitrokey=False,
        password=None,
        nitrokey_key=None,
        nitrokey_token=None,
    ):
        try:
            privkey, signkey, manifest = core_call().claim_user(user_id, device_name, token)
            privkey = privkey.encode()
            signkey = signkey.encode()
            device_id = f"{user_id}@{device_name}"
            if not use_nitrokey:
                core_call().register_new_device(
                    device_id, privkey, signkey, manifest, use_nitrokey=False, password=password
                )
            else:
                core_call().register_new_device(
                    device_id,
                    privkey,
                    signkey,
                    manifest,
                    use_nitrokey=True,
                    nitrokey_token_id=nitrokey_token,
                    nitrokey_key_id=nitrokey_key,
                )
            self.login_widget.add_device(device_id)
            show_info(
                self,
                QCoreApplication.translate(
                    "MainWindow", "User has been successfully registered. You can now login."
                ),
            )
            self.login_widget.reset()
        except DeviceConfigureOutOfDate:
            show_error(
                self.login_widget,
                QCoreApplication.translate("MainWindow", "The token has expired."),
            )
        except DeviceConfigureNoInvitation:
            show_error(
                self.login_widget,
                QCoreApplication.translate("MainWindow", "No invitation found for this user."),
            )
        except DeviceConfigureAlreadyExists:
            show_error(
                self.login_widget,
                QCoreApplication.translate("MainWindow", "User has already been registered."),
            )

    def register_user_with_password(self, user_id, password, device_name, token):
        self.handle_register_user(
            user_id, device_name, token, password=password, use_nitrokey=False
        )

    def register_user_with_nitrokey(
        self, user_id, device_name, token, nitrokey_key, nitrokey_token
    ):
        self.handle_register_user(
            user_id,
            device_name,
            token,
            use_nitrokey=True,
            nitrokey_key=nitrokey_key,
            nitrokey_token=nitrokey_token,
        )

    def handle_register_device(
        self,
        user_id,
        device_name,
        token,
        use_nitrokey=False,
        password=None,
        nitrokey_key=None,
        nitrokey_token=None,
    ):
        try:
            device_id = f"{user_id}@{device_name}"
            if not use_nitrokey:
                privkey, signkey, manifest = core_call().configure_new_device(
                    device_id=device_id,
                    configure_device_token=token,
                    password=password,
                    use_nitrokey=False,
                )
                privkey = privkey.encode()
                signkey = signkey.encode()
                core_call().register_new_device(
                    device_id, privkey, signkey, manifest, password, False
                )
            else:
                privkey, signkey, manifest = core_call().configure_new_device(
                    device_id,
                    token,
                    password=None,
                    use_nitrokey=True,
                    nitrokey_token_id=nitrokey_token,
                    nitrokey_key_id=nitrokey_key,
                )
                privkey = privkey.encode()
                signkey = signkey.encode()
                core_call().register_new_device(
                    device_id, privkey, signkey, manifest, None, True, nitrokey_token, nitrokey_key
                )
            self.login_widget.add_device(device_id)
            show_info(
                self,
                QCoreApplication.translate(
                    "MainWindow", "Device has been successfully registered. You can now login."
                ),
            )
            self.login_widget.reset()
        except Exception as exc:
            # TODO: better error handling
            show_error(self.login_widget, str(exc))

    def register_device_with_password(self, user_id, password, device_name, token):
        self.handle_register_device(
            user_id, device_name, token, use_nitrokey=False, password=password
        )

    def register_device_with_nitrokey(self, user_id, device_name, nitrokey_key, nitrokey_token):
        self.handle_register_device(
            user_id,
            device_name,
            use_nitrokey=True,
            nitrokey_key=nitrokey_key,
            nitrokey_token=nitrokey_token,
        )

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
            if self.tray:
                self.tray.hide()
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
