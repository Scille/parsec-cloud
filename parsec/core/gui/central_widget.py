from PyQt5.QtCore import pyqtSignal, QCoreApplication, QSize
from PyQt5.QtGui import QPixmap, QColor
from PyQt5.QtWidgets import QGraphicsDropShadowEffect

from parsec.core.gui.core_widget import CoreWidget
from parsec.core.gui.mount_widget import MountWidget
from parsec.core.gui.users_widget import UsersWidget
from parsec.core.gui.settings_widget import SettingsWidget
from parsec.core.gui.devices_widget import DevicesWidget
from parsec.core.gui.menu_widget import MenuWidget
from parsec.core.gui.custom_widgets import TaskbarButton
from parsec.core.gui.notification_center_widget import NotificationCenterWidget
from parsec.core.gui.ui.central_widget import Ui_CentralWidget


class CentralWidget(CoreWidget, Ui_CentralWidget):
    connection_state_changed = pyqtSignal(bool)
    logout_requested = pyqtSignal()

    def __init__(self, core_config, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)

        self.menu = MenuWidget(parent=self)
        self.widget_menu.layout().addWidget(self.menu)

        self.mount_widget = MountWidget(parent=self)
        self.mount_widget.reset_taskbar.connect(self.reset_taskbar)
        self.widget_central.layout().insertWidget(0, self.mount_widget)
        self.users_widget = UsersWidget(parent=self)
        self.widget_central.layout().insertWidget(0, self.users_widget)
        self.devices_widget = DevicesWidget(parent=self)
        self.widget_central.layout().insertWidget(0, self.devices_widget)
        self.settings_widget = SettingsWidget(core_config=core_config, parent=self)
        self.widget_central.layout().insertWidget(0, self.settings_widget)
        self.notification_center = NotificationCenterWidget(parent=self)
        self.button_notif = TaskbarButton(icon_path=":/icons/images/icons/menu_settings.png")

        self.widget_notif.layout().addWidget(self.notification_center)
        self.notification_center.hide()

        effect = QGraphicsDropShadowEffect(self)
        effect.setColor(QColor(100, 100, 100))
        effect.setBlurRadius(4)
        effect.setXOffset(-2)
        effect.setYOffset(2)
        self.widget_notif.setGraphicsEffect(effect)

        self.menu.button_files.clicked.connect(self.show_mount_widget)
        self.menu.button_users.clicked.connect(self.show_users_widget)
        self.menu.button_settings.clicked.connect(self.show_settings_widget)
        self.menu.button_devices.clicked.connect(self.show_devices_widget)
        self.menu.button_logout.clicked.connect(self.logout_requested.emit)
        self.button_notif.clicked.connect(self.show_notification_center)
        self.connection_state_changed.connect(self._on_connection_state_changed)
        self.notification_center.close_requested.connect(self.close_notification_center)

        # self.notification_center.add_notification(
        #     "ERROR", "An error message to test how it looks like."
        # )
        # self.notification_center.add_notification(
        #     "WARNING", "Another message but this time its a warning."
        # )
        # self.notification_center.add_notification(
        #     "INFO", "An information message, because we gotta test them all."
        # )
        # self.notification_center.add_notification(
        #     "ERROR",
        #     "And another error message, but this one will be a little bit longer just "
        #     "to see if the GUI can handle it.",
        # )

        self.reset()

    def set_core_attributes(self, core, portal):
        super().set_core_attributes(core, portal)
        self.mount_widget.set_core_attributes(core, portal)
        self.users_widget.set_core_attributes(core, portal)
        self.devices_widget.set_core_attributes(core, portal)
        self.settings_widget.set_core_attributes(core, portal)

    @CoreWidget.core.setter
    def core(self, c):
        if self._core:
            self._core.fs.event_bus.disconnect(
                "backend.connection.ready", self._on_connection_changed
            )
            self._core.fs.event_bus.disconnect(
                "backend.connection.lost", self._on_connection_changed
            )
        self._core = c
        if self._core:
            self._core.fs.event_bus.connect("backend.connection.ready", self._on_connection_changed)
            self._core.fs.event_bus.connect("backend.connection.lost", self._on_connection_changed)
            self._on_connection_state_changed(True)
            self.label_mountpoint.setText(str(self._core.mountpoint))
            self.menu.label_username.setText(self._core.device.user_id)
            self.menu.label_device.setText(self._core.device.device_name)

    def close_notification_center(self):
        self.notification_center.hide()
        self.button_notif.setChecked(False)

    def show_notification_center(self):
        if self.notification_center.isVisible():
            self.notification_center.hide()
            self.button_notif.setChecked(False)
        else:
            self.notification_center.show()
            self.button_notif.setChecked(True)

    def _on_connection_state_changed(self, state):
        if state:
            self.menu.label_connection_text.setText(
                QCoreApplication.translate("CentralWidget", "Connected")
            )
            self.menu.label_connection_icon.setPixmap(
                QPixmap(":/icons/images/icons/cloud_online.png")
            )
        else:
            self.menu.label_connection_text.setText(
                QCoreApplication.translate("CentralWidget", "Disconnected")
            )
            self.menu.label_connection_icon.setPixmap(
                QPixmap(":/icons/images/icons/cloud_offline.png")
            )

    def _on_connection_changed(self, event):
        if event == "backend.connection.ready":
            self.connection_state_changed.emit(True)
        elif event == "backend.connection.lost":
            self.connection_state_changed.emit(False)

    def set_taskbar_buttons(self, buttons):
        while self.widget_taskbar.layout().count() != 0:
            item = self.widget_taskbar.layout().takeAt(0)
            if item:
                w = item.widget()
                self.widget_taskbar.layout().removeWidget(w)
                w.setParent(None)
        buttons.append(self.button_notif)
        total_width = 0
        if len(buttons) == 0:
            self.widget_taskbar.hide()
        else:
            self.widget_taskbar.show()
            for b in buttons:
                self.widget_taskbar.layout().addWidget(b)
                total_width += b.size().width()
            self.widget_taskbar.setFixedSize(QSize(total_width + 44, 68))

    def show_mount_widget(self):
        self.hide_all_widgets()
        self.menu.button_files.setChecked(True)
        self.label_title.setText(QCoreApplication.translate("CentralWidget", "Documents"))
        self.mount_widget.reset()
        self.mount_widget.show()
        self.reset_taskbar()

    def show_users_widget(self):
        self.hide_all_widgets()
        self.menu.button_users.setChecked(True)
        self.label_title.setText(QCoreApplication.translate("CentralWidget", "Users"))
        self.users_widget.reset()
        self.users_widget.show()
        self.reset_taskbar()

    def show_devices_widget(self):
        self.hide_all_widgets()
        self.menu.button_devices.setChecked(True)
        self.label_title.setText(QCoreApplication.translate("CentralWidget", "Devices"))
        self.devices_widget.reset()
        self.devices_widget.show()
        self.reset_taskbar()

    def show_settings_widget(self):
        self.hide_all_widgets()
        self.menu.button_settings.setChecked(True)
        self.label_title.setText(QCoreApplication.translate("CentralWidget", "Settings"))
        self.settings_widget.reset()
        self.settings_widget.show()
        self.reset_taskbar()

    def reset_taskbar(self):
        if self.mount_widget.isVisible():
            self.set_taskbar_buttons(self.mount_widget.get_taskbar_buttons().copy())
        elif self.devices_widget.isVisible():
            self.set_taskbar_buttons(self.devices_widget.get_taskbar_buttons().copy())
        elif self.users_widget.isVisible():
            self.set_taskbar_buttons(self.users_widget.get_taskbar_buttons().copy())
        elif self.settings_widget.isVisible():
            self.set_taskbar_buttons(self.settings_widget.get_taskbar_buttons().copy())

    def hide_all_widgets(self):
        self.mount_widget.hide()
        self.settings_widget.hide()
        self.users_widget.hide()
        self.devices_widget.hide()
        self.menu.button_files.setChecked(False)
        self.menu.button_users.setChecked(False)
        self.menu.button_settings.setChecked(False)
        self.menu.button_devices.setChecked(False)

    def reset(self):
        self.hide_all_widgets()
        self.show_mount_widget()
