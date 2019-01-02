import os
import queue
import threading
import trio

from PyQt5.QtCore import QCoreApplication, Qt, pyqtSignal
from PyQt5.QtWidgets import QMainWindow, QMessageBox, QSystemTrayIcon, QMenu
from PyQt5.QtGui import QIcon, QFontDatabase, QPixmap
from structlog import get_logger

from parsec import __version__ as PARSEC_VERSION

from parsec.core.backend_connection import BackendNotAvailable
from parsec.pkcs11_encryption_tool import DevicePKCS11Error
from parsec.core.devices_manager import DeviceManagerError, load_device_with_password

from parsec.core.gui import settings
from parsec.core import logged_core_factory
from parsec.core.gui.custom_widgets import show_error, show_info, ask_question
from parsec.core.gui.login_widget import LoginWidget
from parsec.core.gui.mount_widget import MountWidget
from parsec.core.gui.users_widget import UsersWidget
from parsec.core.gui.settings_widget import SettingsWidget
from parsec.core.gui.devices_widget import DevicesWidget
from parsec.core.gui.ui.main_window import Ui_MainWindow


logger = get_logger()


class MainWindow(QMainWindow, Ui_MainWindow):
    connection_state_changed = pyqtSignal(bool)

    def __init__(self, core_config, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.setupUi(self)
        self.core_config = core_config
        self.current_device = None
        self.portal = None
        self.core = None
        self.cancel_scope = None
        self.core_thread = None
        self.core_queue = queue.Queue(3)
        self.force_close = False
        self.close_requested = False
        self.tray = None
        self.widget_menu.hide()

        self.mount_widget = MountWidget(parent=self)
        self.main_widget_layout.addWidget(self.mount_widget)
        self.users_widget = UsersWidget(parent=self)
        self.main_widget_layout.addWidget(self.users_widget)
        self.devices_widget = DevicesWidget(parent=self)
        self.main_widget_layout.addWidget(self.devices_widget)
        self.settings_widget = SettingsWidget(parent=self)
        self.main_widget_layout.addWidget(self.settings_widget)
        self.login_widget = LoginWidget(core_config=self.core_config, parent=self)
        self.main_widget_layout.addWidget(self.login_widget)

        self.add_tray_icon()
        self.setWindowTitle("Parsec - Community Edition - v{}".format(PARSEC_VERSION))
        self.tray_message_shown = False
        self.connection_state_changed.connect(self._on_connection_state_changed)
        self.connect_all()
        self.show_login_widget()

    def add_tray_icon(self):
        if not QSystemTrayIcon.isSystemTrayAvailable() or not settings.get_value(
            "global/tray_enabled", "true"
        ):
            return
        self.tray = QSystemTrayIcon(self)
        menu = QMenu()
        action = menu.addAction(QCoreApplication.translate(self.__class__.__name__, "Show window"))
        action.triggered.connect(self.show_top)
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
            self.show_top()

    def show_top(self):
        self.show()
        self.raise_()

    def start_core(self):
        def _run_core():
            logger.info("Starting core thread")

            async def _run():
                portal = trio.BlockingTrioPortal()
                self.core_queue.put(portal)
                with trio.open_cancel_scope() as cancel_scope:
                    logger.info("Cancel scope created")
                    self.core_queue.put(cancel_scope)
                    async with logged_core_factory(self.core_config, self.current_device) as core:
                        logger.info("Core created")
                        self.core_queue.put(core)
                        await trio.sleep_forever()

            trio.run(_run)

        self.core_config = self.core_config.evolve(mountpoint_enabled=True)
        self.core_thread = threading.Thread(target=_run_core)
        self.core_thread.start()
        self.portal = self.core_queue.get()
        self.cancel_scope = self.core_queue.get()
        self.core = self.core_queue.get()
        self.mount_widget.set_core_attrs(portal=self.portal, core=self.core)
        self.users_widget.portal = self.portal
        self.users_widget.core = self.core
        self.label_mountpoint.setText(str(self.core.mountpoint))
        self.label_username.setText(self.core.device.user_id)
        self.label_device.setText(self.core.device.device_name)
        self.core.event_bus.connect("backend.connection.ready", self._on_connection_changed)
        self.core.event_bus.connect("backend.connection.lost", self._on_connection_changed)

    def stop_core(self):
        if not self.portal:
            return
        self.portal.run_sync(self.cancel_scope.cancel)
        self.core_thread.join()
        self.portal = None
        self.core = None
        self.cancel_scope = None
        self.core_thread = None
        self.mount_widget.set_core_attrs(None, None)
        self.users_widget.portal = None
        self.users_widget.core = None

    def logout(self):
        if self.core_thread:
            self.stop_core()
        self.current_device = None
        self.show_login_widget()

    def login_with_password(self, device_id, password):
        try:
            self.current_device = load_device_with_password(
                self.core_config.config_dir, device_id, password
            )
            self.start_core()
            self.show_mount_widget()
        except DeviceManagerError:
            show_error(self, QCoreApplication.translate("MainWindow", "Authentication failed."))

    def login_with_pkcs11(self, device_id, pkcs11_pin, pkcs11_key, pkcs11_token):
        try:
            device = load_device_with_pkcs11(self.core_config.config_dir, device_id)
            self.mount()
            self.show_mount_widget()
        except DeviceManagerError:
            show_error(self, QCoreApplication.translate("MainWindow", "Authentication failed."))

    def register_user(self, login):
        pass

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
        pass

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
        pass

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
            not settings.get_value("global/tray_enabled")
            or not QSystemTrayIcon.isSystemTrayAvailable()
            or self.close_requested
            or self.core_config.debug
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
            if self.mount_widget:
                self.mount_widget.stop()
            if self.tray:
                self.tray.hide()
            self.stop_core()
        else:
            if self.tray and not self.tray_message_shown:
                self.tray.showMessage(
                    "Parsec", QCoreApplication.translate("MainWindow", "Parsec is still running.")
                )
                self.tray_message_shown = True
            event.ignore()
            self.hide()

    def _on_connection_state_changed(self, state):
        if state:
            self.label_connection_text.setText(
                QCoreApplication.translate("MainWindow", "Connected")
            )
            self.label_connection_icon.setPixmap(QPixmap(":/icons/images/icons/cloud_online.png"))
        else:
            self.label_connection_text.setText(
                QCoreApplication.translate("MainWindow", "Disconnected")
            )
            self.label_connection_icon.setPixmap(QPixmap(":/icons/images/icons/cloud_offline.png"))

    def _on_connection_changed(self, event):
        if event == "backend.connection.ready":
            self.connection_state_changed.emit(True)
        elif event == "backend.connection.lost":
            self.connection_state_changed.emit(False)

    def show_mount_widget(self):
        self._hide_all_central_widgets()
        self.button_files.setChecked(True)
        self.label_title.setText(QCoreApplication.translate("MainWindow", "Documents"))
        self.widget_menu.show()
        self.widget_title.show()
        self.mount_widget.show()

    def show_users_widget(self):
        self._hide_all_central_widgets()
        self.button_users.setChecked(True)
        self.label_title.setText(QCoreApplication.translate("MainWindow", "Users"))
        self.widget_menu.show()
        self.widget_title.show()
        self.users_widget.reset()
        self.users_widget.show()

    def show_devices_widget(self):
        self._hide_all_central_widgets()
        self.button_devices.setChecked(True)
        self.label_title.setText(QCoreApplication.translate("MainWindow", "Devices"))
        self.widget_menu.show()
        self.widget_title.show()
        self.devices_widget.show()

    def show_settings_widget(self):
        self._hide_all_central_widgets()
        self.button_settings.setChecked(True)
        self.label_title.setText(QCoreApplication.translate("MainWindow", "Settings"))
        self.widget_menu.show()
        self.widget_title.show()
        self.settings_widget.show()

    def show_login_widget(self):
        self._hide_all_central_widgets()
        self.login_widget.show()

    def _hide_all_central_widgets(self):
        self.login_widget.hide()
        self.mount_widget.hide()
        self.settings_widget.hide()
        self.users_widget.hide()
        self.devices_widget.hide()
        self.widget_title.hide()
        self.widget_menu.hide()
        self.button_files.setChecked(False)
        self.button_users.setChecked(False)
        self.button_settings.setChecked(False)
        self.button_devices.setChecked(False)
