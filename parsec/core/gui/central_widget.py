# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import pyqtSignal, QSize
from PyQt5.QtGui import QPixmap, QColor
from PyQt5.QtWidgets import QGraphicsDropShadowEffect, QWidget

from parsec.core.gui.mount_widget import MountWidget
from parsec.core.gui.users_widget import UsersWidget
from parsec.core.gui.settings_widget import SettingsWidget
from parsec.core.gui.devices_widget import DevicesWidget
from parsec.core.gui.menu_widget import MenuWidget
from parsec.core.gui.lang import translate as _
from parsec.core.gui.custom_widgets import NotificationTaskbarButton
from parsec.core.gui.notification_center_widget import NotificationCenterWidget
from parsec.core.gui.ui.central_widget import Ui_CentralWidget

from parsec.core.backend_connection.monitor import BackendState, current_backend_connection_state


class CentralWidget(QWidget, Ui_CentralWidget):
    NOTIFICATION_EVENTS = [
        "backend.connection.lost",
        "backend.connection.ready",
        "backend.connection.incompatible_version",
        "mountpoint.stopped",
        "sharing.granted",
        "sharing.revoked",
        "sharing.updated",
        "fs.entry.file_update_conflicted",
        "fs.access_backend_offline",
        "fs.access_crypto_error",
    ]

    connection_state_changed = pyqtSignal(int)
    logout_requested = pyqtSignal()
    new_notification = pyqtSignal(str, str)

    def __init__(self, core, jobs_ctx, event_bus, **kwargs):
        super().__init__(**kwargs)
        self.setupUi(self)

        self.jobs_ctx = jobs_ctx
        self.core = core
        self.event_bus = event_bus

        self.menu = MenuWidget(parent=self)
        self.widget_menu.layout().addWidget(self.menu)
        self.notification_center = NotificationCenterWidget(parent=self)
        self.button_notif = NotificationTaskbarButton()
        self.widget_notif.layout().addWidget(self.notification_center)
        self.notification_center.hide()

        self.event_bus.connect("backend.connection.ready", self._on_connection_changed)
        self.event_bus.connect("backend.connection.lost", self._on_connection_changed)
        self.event_bus.connect(
            "backend.connection.incompatible_version", self._on_connection_changed
        )
        for e in self.NOTIFICATION_EVENTS:
            self.event_bus.connect(e, self.handle_event)

        self._on_connection_state_changed(current_backend_connection_state(event_bus).value)
        self.label_mountpoint.setText(str(self.core.config.mountpoint_base_dir))
        self.menu.organization = self.core.device.organization_addr.organization_id
        self.menu.username = self.core.device.user_id
        self.menu.device = self.core.device.device_name

        self.new_notification.connect(self.on_new_notification)
        self.menu.files_clicked.connect(self.show_mount_widget)
        self.menu.users_clicked.connect(self.show_users_widget)
        self.menu.settings_clicked.connect(self.show_settings_widget)
        self.menu.devices_clicked.connect(self.show_devices_widget)
        self.menu.logout_clicked.connect(self.logout_requested.emit)
        self.button_notif.clicked.connect(self.show_notification_center)
        self.connection_state_changed.connect(self._on_connection_state_changed)
        self.notification_center.close_requested.connect(self.close_notification_center)
        if current_backend_connection_state(event_bus) == BackendState.INCOMPATIBLE_VERSION:
            self.handle_event("backend.connection.incompatible_version")

        effect = QGraphicsDropShadowEffect(self)
        effect.setColor(QColor(100, 100, 100))
        effect.setBlurRadius(4)
        effect.setXOffset(-2)
        effect.setYOffset(2)
        self.widget_notif.setGraphicsEffect(effect)

        self.show_mount_widget()

    def disconnect_all(self):
        self.event_bus.disconnect("backend.connection.ready", self._on_connection_changed)
        self.event_bus.disconnect("backend.connection.lost", self._on_connection_changed)
        self.event_bus.disconnect(
            "backend.connection.incompatible_version", self._on_connection_changed
        )
        for e in self.NOTIFICATION_EVENTS:
            self.event_bus.disconnect(e, self.handle_event)
        item = self.widget_central.layout().itemAt(0)
        if item:
            item.widget().disconnect_all()

    def handle_event(self, event, **kwargs):
        if event == "backend.connection.lost":
            self.new_notification.emit("WARNING", _("Disconnected from the backend."))
        elif event == "backend.connection.ready":
            self.new_notification.emit("INFO", _("Connected to the backend."))
        elif event == "backend.connection.incompatible_version":
            self.new_notification.emit(
                "WARNING", _("Cannot connect to backend : incompatible version detected.")
            )
        elif event == "mountpoint.stopped":
            self.new_notification.emit("WARNING", _("Mountpoint has been unmounted."))
        elif event == "sharing.granted":
            self.new_notification.emit(
                "INFO", _("Workspace '{}' shared with you").format(kwargs["new_entry"].name)
            )
        elif event == "sharing.updated":
            self.new_notification.emit(
                "INFO",
                _("Your role on Workspace '{}' has changed changed").format(
                    kwargs["new_entry"].name
                ),
            )
        elif event == "sharing.revoked":
            self.new_notification.emit(
                "INFO",
                _("Workspace '{}' no longer shared with you").format(kwargs["previous_entry"].name),
            )
        elif event == "fs.entry.file_update_conflicted":
            self.new_notification.emit(
                "WARNING", _("Conflict while syncing file '{}'.").format(kwargs["path"])
            )
        elif event == "fs.access_backend_offline":
            self.new_notification.emit(
                "WARNING",
                _("File {} is not currently available in local: {}").format(
                    kwargs["path"], kwargs["msg"]
                ),
            )
        elif event == "fs.access_crypto_error":
            self.new_notification.emit(
                "ERROR",
                _("Cryptographic error on file {}: {}").format(kwargs["path"], kwargs["msg"]),
            )

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
            self.button_notif.reset_notif_count()

    def _on_connection_state_changed(self, state):
        if state == BackendState.READY.value:
            self.menu.label_connection_text.setText(_("Connected"))
            self.menu.label_connection_icon.setPixmap(
                QPixmap(":/icons/images/icons/cloud_online.png")
            )
        elif state == BackendState.LOST.value:
            self.menu.label_connection_text.setText(_("Disconnected"))
            self.menu.label_connection_icon.setPixmap(
                QPixmap(":/icons/images/icons/cloud_offline.png")
            )
        elif state == BackendState.INCOMPATIBLE_VERSION.value:
            self.menu.label_connection_text.setText(_("Bad Version"))
            self.menu.label_connection_icon.setPixmap(
                QPixmap(":/icons/images/icons/cloud_offline.png")
            )

    def on_new_notification(self, notif_type, msg):
        self.notification_center.add_notification(notif_type, msg)
        if self.notification_center.isHidden():
            self.button_notif.inc_notif_count()
            self.button_notif.repaint()

    def _on_connection_changed(self, event):
        if event == "backend.connection.ready":
            self.connection_state_changed.emit(BackendState.READY.value)
        elif event == "backend.connection.lost":
            self.connection_state_changed.emit(BackendState.LOST.value)
        elif event == "backend.connection.incompatible_version":
            self.connection_state_changed.emit(BackendState.INCOMPATIBLE_VERSION.value)

    def show_mount_widget(self):
        self.clear_widgets()
        mount_widget = MountWidget(self.core, self.jobs_ctx, self.event_bus, parent=self)
        self.widget_central.layout().insertWidget(0, mount_widget)
        self.menu.activate_files()
        self.label_title.setText(_("Documents"))
        self.set_taskbar_buttons(mount_widget.get_taskbar_buttons())
        mount_widget.widget_switched.connect(self.set_taskbar_buttons)
        mount_widget.show()

    def show_users_widget(self):
        self.clear_widgets()
        users_widget = UsersWidget(self.core, self.jobs_ctx, self.event_bus, parent=self)
        self.widget_central.layout().insertWidget(0, users_widget)
        self.menu.activate_users()
        self.label_title.setText(_("Users"))
        self.set_taskbar_buttons(users_widget.get_taskbar_buttons())
        users_widget.show()

    def show_devices_widget(self):
        self.clear_widgets()
        devices_widget = DevicesWidget(self.core, self.jobs_ctx, self.event_bus, parent=self)
        self.widget_central.layout().insertWidget(0, devices_widget)
        self.menu.activate_devices()
        self.label_title.setText(_("Devices"))
        self.set_taskbar_buttons(devices_widget.get_taskbar_buttons())
        devices_widget.show()

    def show_settings_widget(self):
        self.clear_widgets()
        settings_widget = SettingsWidget(self.core.config, self.event_bus, parent=self)
        self.widget_central.layout().insertWidget(0, settings_widget)
        self.menu.activate_settings()
        self.label_title.setText(_("Settings"))
        self.set_taskbar_buttons([])
        settings_widget.show()

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

    def clear_widgets(self):
        item = self.widget_central.layout().takeAt(0)
        if item:
            item.widget().disconnect_all()
            item.widget().hide()
            item.widget().setParent(None)
