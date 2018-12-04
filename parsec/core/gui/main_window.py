import os

from PyQt5.QtCore import QCoreApplication, Qt
from PyQt5.QtWidgets import QMainWindow, QMessageBox, QSystemTrayIcon, QMenu
from PyQt5.QtGui import QIcon, QFontDatabase
from structlog import get_logger

from parsec import __version__ as PARSEC_VERSION

from parsec.pkcs11_encryption_tool import DevicePKCS11Error
from parsec.core.devices_manager import (
    DeviceLoadingError,
    DeviceConfigureBackendError,
    DeviceConfigureOutOfDate,
    DeviceConfigureNoInvitation,
    DeviceSavingAlreadyExists,
    DeviceConfigureError,
    DeviceSavingError,
)

from parsec.core.gui import settings
from parsec.core.gui.core_call import core_call
from parsec.core.gui.custom_widgets import show_error, show_info, ask_question
from parsec.core.gui.login_widget import LoginWidget
from parsec.core.gui.mount_widget import MountWidget
from parsec.core.gui.users_widget import UsersWidget
from parsec.core.gui.settings_widget import SettingsWidget
from parsec.core.gui.devices_widget import DevicesWidget
from parsec.core.gui.ui.main_window import Ui_MainWindow


logger = get_logger()


class MainWindow(QMainWindow, Ui_MainWindow):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.setupUi(self)
        self.force_close = False
        self.close_requested = False
        QFontDatabase.addApplicationFont(":/fonts/fonts/ProximaNova.otf")
        self.files_widget = None
        self.settings_widget = None
        self.users_widget = None
        self.tray = None
        self.widget_menu.hide()
        self.login_widget = LoginWidget(parent=self.widget_main)
        self.main_widget_layout.insertWidget(1, self.login_widget)
        self.users_widget = UsersWidget(parent=self.widget_main)
        self.main_widget_layout.insertWidget(1, self.users_widget)
        self.devices_widget = DevicesWidget(parent=self.widget_main)
        self.main_widget_layout.insertWidget(1, self.devices_widget)
        self.settings_widget = SettingsWidget(parent=self.widget_main)
        self.main_widget_layout.insertWidget(1, self.settings_widget)
        self.mount_widget = MountWidget(parent=self.widget_main)
        self.main_widget_layout.insertWidget(1, self.mount_widget)
        self.show_login_widget()
        self.current_device = None
        self.add_tray_icon()
        self.connect_all()
        self.setWindowTitle("Parsec - Community Edition - v{}".format(PARSEC_VERSION))

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
        self.button_files.clicked.connect(self.show_mount_widget)
        self.button_users.clicked.connect(self.show_users_widget)
        self.button_settings.clicked.connect(self.show_settings_widget)
        self.button_devices.clicked.connect(self.show_devices_widget)
        self.button_logout.clicked.connect(self.logout)
        self.login_widget.login_with_password_clicked.connect(self.login_with_password)
        self.login_widget.login_with_pkcs11_clicked.connect(self.login_with_pkcs11)
        self.login_widget.register_user_with_password_clicked.connect(
            self.register_user_with_password
        )
        self.login_widget.register_user_with_pkcs11_clicked.connect(self.register_user_with_pkcs11)
        self.login_widget.register_device_with_password_clicked.connect(
            self.register_device_with_password
        )
        self.login_widget.register_device_with_pkcs11_clicked.connect(
            self.register_device_with_pkcs11
        )
        self.users_widget.register_user_clicked.connect(self.register_user)

    def tray_activated(self, reason):
        if reason == QSystemTrayIcon.DoubleClick:
            self.show()
            self.raise_()

    def logout(self):
        self.unmount()
        core_call().logout()
        device = core_call().load_device("johndoe@test")
        core_call().login(device)
        self.current_device = device
        self.show_login_widget()

    def unmount(self):
        if core_call().is_mounted():
            try:
                core_call().unmount()
            except Exception as exc:
                logger.warning("Mountpoint stop failed", exc_info=exc)
            else:
                logger.info("Mountpoint stopped")

    def mount(self):
        base_mountpoint = settings.get_value("mountpoint")
        if not base_mountpoint:
            return None
        mountpoint = os.path.join(base_mountpoint, self.current_device.id)
        self.unmount()
        try:
            core_call().mount(mountpoint)
        except Exception as exc:
            logger.warning("Mountpoint start failed", mountpoint=mountpoint, exc_info=exc)
            return None
        self.mount_widget.set_mountpoint(mountpoint)
        self.label_mountpoint.setText(mountpoint)
        logger.info("Mountpoint started", mountpoint=mountpoint)
        return mountpoint

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

        self.mount_widget.reset()
        self.mount_widget.set_mountpoint(mountpoint)
        self.show_mount_widget()

    def perform_login(
        self, device_id, password=None, pkcs11_pin=None, pkcs11_key=None, pkcs11_token=None
    ):
        try:
            device = core_call().load_device(
                device_id,
                password=password,
                pkcs11_pin=pkcs11_pin,
                pkcs11_token_id=pkcs11_token,
                pkcs11_key_id=pkcs11_key,
            )
            self.current_device = device
            core_call().logout()
            core_call().login(device)
            settings.set_value("last_device", device_id)
            if settings.get_value("mountpoint_enabled"):
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
            self.widget_menu.show()
            self.button_user.setText("    " + device_id.split("@")[0])
            self.show_mount_widget()
        except (DeviceLoadingError, DevicePKCS11Error):
            show_error(self, QCoreApplication.translate("MainWindow", "Authentication failed."))

    def login_with_password(self, device_id, password):
        self.perform_login(device_id, password=password)

    def login_with_pkcs11(self, device_id, pkcs11_pin, pkcs11_key, pkcs11_token):
        self.perform_login(
            device_id, pkcs11_pin=pkcs11_pin, pkcs11_key=pkcs11_key, pkcs11_token=pkcs11_token
        )

    def register_user(self, login):
        try:
            token = core_call().invite_user(login)
            self.users_widget.set_claim_infos(login, token)
        except DeviceConfigureBackendError:
            show_error(
                self.users_widget,
                QCoreApplication.translate("MainWindow", "Can not register the user."),
            )

    def handle_register_user(
        self,
        user_id,
        device_name,
        token,
        use_pkcs11=False,
        password=None,
        pkcs11_key=None,
        pkcs11_token=None,
    ):
        try:
            privkey, signkey, manifest = core_call().claim_user(user_id, device_name, token)
            privkey = privkey.encode()
            signkey = signkey.encode()
            device_id = f"{user_id}@{device_name}"
            if not use_pkcs11:
                core_call().register_new_device(
                    device_id, privkey, signkey, manifest, password, False
                )
            else:
                core_call().register_new_device(
                    device_id, privkey, signkey, manifest, None, True, pkcs11_token, pkcs11_key
                )
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
        except DeviceSavingAlreadyExists:
            show_error(
                self.login_widget,
                QCoreApplication.translate("MainWindow", "User has already been registered."),
            )
        except:
            show_error(
                self.login_widget,
                QCoreApplication.translate("MainWindow", "Can not register this user."),
            )

    def register_user_with_password(self, user_id, password, device_name, token):
        self.handle_register_user(user_id, device_name, token, password=password, use_pkcs11=False)

    def register_user_with_pkcs11(self, user_id, device_name, token, pkcs11_key, pkcs11_token):
        self.handle_register_user(
            user_id,
            device_name,
            token,
            use_pkcs11=True,
            pkcs11_key=pkcs11_key,
            pkcs11_token=pkcs11_token,
        )

    def handle_register_device(
        self,
        user_id,
        device_name,
        token,
        use_pkcs11=False,
        password=None,
        pkcs11_key=None,
        pkcs11_token=None,
    ):
        try:
            device_id = f"{user_id}@{device_name}"
            if not use_pkcs11:
                privkey, signkey, manifest = core_call().configure_new_device(
                    device_id=device_id,
                    configure_device_token=token,
                    password=password,
                    use_pkcs11=False,
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
                    use_pkcs11=True,
                    pkcs11_token_id=pkcs11_token,
                    pkcs11_key_id=pkcs11_key,
                )
                privkey = privkey.encode()
                signkey = signkey.encode()
                core_call().register_new_device(
                    device_id, privkey, signkey, manifest, None, True, pkcs11_token, pkcs11_key
                )
            show_info(
                self,
                QCoreApplication.translate(
                    "MainWindow", "Device has been successfully registered. You can now login."
                ),
            )
            self.login_widget.reset()
        except DeviceSavingAlreadyExists:
            show_error(
                self.login_widget,
                QCoreApplication.translate("MainWindow", "The device already exists."),
            )
        except DevicePKCS11Error:
            show_error(
                self.login_widget,
                QCoreApplication.translate("MainWindow", "Invalid PKCS #11 information."),
            )
        except (DeviceSavingError, DeviceConfigureError):
            show_error(
                self.login_widget,
                QCoreApplication.translate("MainWindow", "Can not create the new device."),
            )

    def register_device_with_password(self, user_id, password, device_name, token):
        self.handle_register_device(
            user_id, device_name, token, use_pkcs11=False, password=password
        )

    def register_device_with_pkcs11(self, user_id, device_name, pkcs11_key, pkcs11_token):
        self.handle_register_device(
            user_id, device_name, use_pkcs11=True, pkcs11_key=pkcs11_key, pkcs11_token=pkcs11_token
        )

    def close_app(self):
        self.close_requested = True
        self.close()

    def closeEvent(self, event):
        if (
            not QSystemTrayIcon.isSystemTrayAvailable()
            or self.close_requested
            or core_call().is_debug()
            or self.force_close
        ):
            if not self.force_close:
                result = ask_question(
                    self,
                    QCoreApplication.translate(self.__class__.__name__, "Confirmation"),
                    QCoreApplication.translate("MainWindow", "Are you sure you want to quit ?"),
                )
                if not result:
                    event.ignore()
                    return
                event.accept()
            else:
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

    def show_mount_widget(self):
        self._hide_all_central_widgets()
        self.widget_menu.show()
        self.button_files.setChecked(True)
        self.widget_title.show()
        self.label_title.setText(QCoreApplication.translate("MainWindow", "Documents"))
        self.mount_widget.reset()
        self.mount_widget.show()

    def show_users_widget(self):
        self._hide_all_central_widgets()
        self.widget_menu.show()
        self.button_users.setChecked(True)
        self.widget_title.show()
        self.label_title.setText(QCoreApplication.translate("MainWindow", "Users"))
        self.users_widget.reset()
        self.users_widget.show()

    def show_devices_widget(self):
        self._hide_all_central_widgets()
        self.widget_menu.show()
        self.button_devices.setChecked(True)
        self.widget_title.show()
        self.label_title.setText(QCoreApplication.translate("MainWindow", "Devices"))
        self.devices_widget.reset()
        self.devices_widget.show()

    def show_settings_widget(self):
        self._hide_all_central_widgets()
        self.widget_menu.show()
        self.button_settings.setChecked(True)
        self.widget_title.show()
        self.label_title.setText(QCoreApplication.translate("MainWindow", "Settings"))
        self.settings_widget.reset()
        self.settings_widget.show()

    def show_login_widget(self):
        self._hide_all_central_widgets()
        self.login_widget.reset()
        self.login_widget.show()

    def _hide_all_central_widgets(self):
        self.widget_menu.hide()
        self.mount_widget.hide()
        self.users_widget.hide()
        self.settings_widget.hide()
        self.login_widget.hide()
        self.devices_widget.hide()
        self.widget_title.hide()
        self.button_files.setChecked(False)
        self.button_users.setChecked(False)
        self.button_settings.setChecked(False)
        self.button_devices.setChecked(False)
