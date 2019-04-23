# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import pyqtSignal, QSize
from PyQt5.QtGui import QPixmap, QColor
from PyQt5.QtWidgets import QGraphicsDropShadowEffect

from parsec.core.gui.core_widget import CoreWidget
from parsec.core.gui.mount_widget import MountWidget
from parsec.core.gui.users_widget import UsersWidget
from parsec.core.gui.settings_widget import SettingsWidget
from parsec.core.gui.devices_widget import DevicesWidget
from parsec.core.gui.menu_widget import MenuWidget
from parsec.core.gui.lang import translate as _
from parsec.core.gui.custom_widgets import NotificationTaskbarButton
from parsec.core.gui.notification_center_widget import NotificationCenterWidget
from parsec.core.gui.ui.central_widget import Ui_CentralWidget


class CentralWidget(CoreWidget, Ui_CentralWidget):
    NOTIFICATION_EVENTS = [
        "backend.connection.lost",
        "backend.connection.ready",
        "mountpoint.stopped",
        "sharing.new",
        "fs.entry.file_update_conflicted",
    ]

    connection_state_changed = pyqtSignal(bool)
    logout_requested = pyqtSignal()
    new_notification = pyqtSignal(str, str)

    def __init__(self, jobs_ctx, event_bus, config, **kwargs):
        super().__init__(**kwargs)
        self.setupUi(self)

        self.jobs_ctx = jobs_ctx
        self.event_bus = event_bus
        self.config = config

        self.menu = MenuWidget(parent=self)
        self.widget_menu.layout().addWidget(self.menu)

        self.mount_widget = MountWidget(jobs_ctx, event_bus, config, parent=self)
        self.mount_widget.reset_taskbar.connect(self.reset_taskbar)
        self.widget_central.layout().insertWidget(0, self.mount_widget)
        self.users_widget = UsersWidget(parent=self)
        self.widget_central.layout().insertWidget(0, self.users_widget)
        self.devices_widget = DevicesWidget(parent=self)
        self.widget_central.layout().insertWidget(0, self.devices_widget)
        self.settings_widget = SettingsWidget(core_config=config, parent=self)
        self.widget_central.layout().insertWidget(0, self.settings_widget)
        self.notification_center = NotificationCenterWidget(parent=self)
        self.button_notif = NotificationTaskbarButton()

        self.widget_notif.layout().addWidget(self.notification_center)
        self.notification_center.hide()

        effect = QGraphicsDropShadowEffect(self)
        effect.setColor(QColor(100, 100, 100))
        effect.setBlurRadius(4)
        effect.setXOffset(-2)
        effect.setYOffset(2)
        self.widget_notif.setGraphicsEffect(effect)

        self.new_notification.connect(self.on_new_notification)
        self.menu.files_clicked.connect(self.show_mount_widget)
        self.menu.users_clicked.connect(self.show_users_widget)
        self.menu.settings_clicked.connect(self.show_settings_widget)
        self.menu.devices_clicked.connect(self.show_devices_widget)
        self.menu.logout_clicked.connect(self.logout_requested.emit)
        self.button_notif.clicked.connect(self.show_notification_center)
        self.connection_state_changed.connect(self._on_connection_state_changed)
        self.notification_center.close_requested.connect(self.close_notification_center)
        self.reset()

    def set_core_attributes(self, core, portal):
        super().set_core_attributes(core, portal)
        self.mount_widget.set_core_attributes(core, portal)
        self.users_widget.set_core_attributes(core, portal)
        self.devices_widget.set_core_attributes(core, portal)
        self.settings_widget.set_core_attributes(core, portal)
        self.notification_center.clear()
        self.button_notif.reset_notif_count()

    @CoreWidget.core.setter
    def core(self, c):
        if self._core:
            self._core.event_bus.disconnect("backend.connection.ready", self._on_connection_changed)
            self._core.event_bus.disconnect("backend.connection.lost", self._on_connection_changed)
            for e in self.NOTIFICATION_EVENTS:
                self._core.event_bus.disconnect(e, self.handle_event)
        self._core = c
        if self._core:
            self._core.event_bus.connect("backend.connection.ready", self._on_connection_changed)
            self._core.event_bus.connect("backend.connection.lost", self._on_connection_changed)
            for e in self.NOTIFICATION_EVENTS:
                self._core.event_bus.connect(e, self.handle_event)
            self._on_connection_state_changed(True)
            self.label_mountpoint.setText(str(self._core.config.mountpoint_base_dir))
            self.menu.organization = self._core.device.organization_addr.organization_id
            self.menu.username = self._core.device.user_id
            self.menu.device = self._core.device.device_name

    def handle_event(self, event, **kwargs):
        if event == "backend.connection.lost":
            self.new_notification.emit("WARNING", _("Disconnected from the backend."))
        elif event == "backend.connection.ready":
            self.new_notification.emit("INFO", _("Connected to the backend."))
        elif event == "mountpoint.stopped":
            self.new_notification.emit("ERROR", _("Mountpoint has been unmounted."))
        elif event == "sharing.granted":
            self.new_notification.emit(
                "INFO", _("Workspace '{}' shared with you").format(kwargs["new_entry"].name)
            )
        elif event == "sharing.updated":
            self.new_notification.emit(
                "INFO",
                _("Workspace '{}' sharing rights have changed").format(kwargs["new_entry"].name),
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
        if state:
            self.menu.label_connection_text.setText(_("Connected"))
            self.menu.label_connection_icon.setPixmap(
                QPixmap(":/icons/images/icons/cloud_online.png")
            )
        else:
            self.menu.label_connection_text.setText(_("Disconnected"))
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
        self.menu.activate_files()
        self.label_title.setText(_("Documents"))
        self.mount_widget.reset()
        self.mount_widget.show()
        self.reset_taskbar()

    def show_users_widget(self):
        self.hide_all_widgets()
        self.menu.activate_users()
        self.label_title.setText(_("Users"))
        self.users_widget.reset()
        self.users_widget.show()
        self.reset_taskbar()

    def show_devices_widget(self):
        self.hide_all_widgets()
        self.menu.activate_devices()
        self.label_title.setText(_("Devices"))
        self.devices_widget.reset()
        self.devices_widget.show()
        self.reset_taskbar()

    def show_settings_widget(self):
        self.hide_all_widgets()
        self.menu.activate_settings()
        self.label_title.setText(_("Settings"))
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

    def reset(self):
        self.notification_center.clear()
        self.button_notif.reset_notif_count()
        self.hide_all_widgets()
        self.show_mount_widget()
