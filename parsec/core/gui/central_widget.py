# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtCore import pyqtSignal, QSize
from PyQt5.QtGui import QPixmap, QColor
from PyQt5.QtWidgets import QGraphicsDropShadowEffect, QWidget

from parsec.core.gui import desktop
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
from parsec.core.fs import (
    FSWorkspaceNoReadAccess,
    FSWorkspaceNoWriteAccess,
    FSWorkspaceInMaintenance,
)


class CentralWidget(QWidget, Ui_CentralWidget):
    NOTIFICATION_EVENTS = [
        "backend.connection.incompatible_version",
        "backend.connection.rvk_mismatch",
        "mountpoint.stopped",
        "mountpoint.remote_error",
        "mountpoint.unhandled_error",
        "sharing.updated",
        "fs.entry.file_update_conflicted",
    ]

    connection_state_changed = pyqtSignal(int)
    logout_requested = pyqtSignal()
    new_notification = pyqtSignal(str, str, str)

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
        self.button_notif.setToolTip(_("BUTTON_TASKBAR_NOTIFICATION"))
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
        self.label_mountpoint.clicked.connect(self.open_mountpoint)
        self.menu.organization = self.core.device.organization_addr.organization_id
        self.menu.username = self.core.device.user_id
        self.menu.device = self.core.device.device_name
        self.menu.organization_url = str(self.core.device.organization_addr)

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

        self.mount_widget = MountWidget(self.core, self.jobs_ctx, self.event_bus, parent=self)
        self.widget_central.layout().insertWidget(0, self.mount_widget)
        self.mount_widget.widget_switched.connect(self.set_taskbar_buttons)

        self.users_widget = UsersWidget(self.core, self.jobs_ctx, self.event_bus, parent=self)
        self.widget_central.layout().insertWidget(0, self.users_widget)

        self.devices_widget = DevicesWidget(self.core, self.jobs_ctx, self.event_bus, parent=self)
        self.widget_central.layout().insertWidget(0, self.devices_widget)

        self.settings_widget = SettingsWidget(
            self.core.config, self.jobs_ctx, self.event_bus, parent=self
        )
        self.widget_central.layout().insertWidget(0, self.settings_widget)

        self.show_mount_widget()

    def open_mountpoint(self, path):
        desktop.open_file(path)

    def handle_event(self, event, **kwargs):
        if event == "backend.connection.incompatible_version":
            self.new_notification.emit(event, "WARNING", _("NOTIF_WARN_INCOMPATIBLE_VERSION"))
        if event == "backend.connection.rvk_mismatch":
            self.new_notification.emit(event, "ERROR", _("NOTIF_WARN_RVK_MISMATCH"))
        elif event == "mountpoint.stopped":
            self.new_notification.emit(event, "WARNING", _("NOTIF_WARN_MOUNTPOINT_UNMOUNTED"))
        elif event == "mountpoint.remote_error":
            exc = kwargs["exc"]
            path = kwargs["path"]
            if isinstance(exc, FSWorkspaceNoReadAccess):
                msg = _("NOTIF_WARN_WORKSPACE_READ_ACCESS_LOST_{}").format(path)
            elif isinstance(exc, FSWorkspaceNoWriteAccess):
                msg = _("NOTIF_WARN_WORKSPACE_WRITE_ACCESS_LOST_{}").format(path)
            elif isinstance(exc, FSWorkspaceInMaintenance):
                msg = _("NOTIF_WARN_WORKSPACE_IN_MAINTENANCE_{}").format(path)
            else:
                msg = _("NOTIF_WARN_MOUNTPOINT_REMOTE_ERROR_{}_{}").format(path, str(exc))
            self.new_notification.emit(event, "WARNING", msg)
        elif event == "mountpoint.unhandled_error":
            exc = kwargs["exc"]
            path = kwargs["path"]
            operation = kwargs["operation"]
            self.new_notification.emit(
                event,
                "ERROR",
                _("NOTIF_ERR_MOUNTPOINT_UNEXPECTED_ERROR_{}_{}_{}").format(
                    operation, path, str(exc)
                ),
            )
        elif event == "sharing.updated":
            new_entry = kwargs["new_entry"]
            previous_entry = kwargs["previous_entry"]
            new_role = getattr(new_entry, "role", None)
            previous_role = getattr(previous_entry, "role", None)
            if new_role is not None and previous_role is None:
                self.new_notification.emit(
                    event, "INFO", _("NOTIF_INFO_WORKSPACE_SHARED_{}").format(new_entry.name)
                )
            elif new_role is not None and previous_role is not None:
                self.new_notification.emit(
                    event, "INFO", _("NOTIF_INFO_WORKSPACE_ROLE_UPDATED_{}").format(new_entry.name)
                )
            elif new_role is None and previous_role is not None:
                self.new_notification.emit(
                    event, "INFO", _("NOTIF_INFO_WORKSPACE_UNSHARED_{}").format(previous_entry.name)
                )
        elif event == "fs.entry.file_update_conflicted":
            self.new_notification.emit(
                event, "WARNING", _("NOTIF_WARN_SYNC_CONFLICT_{}").format(kwargs["path"])
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
            self.menu.label_connection_text.setText(_("BACKEND_STATE_CONNECTED"))
            self.menu.label_connection_icon.setPixmap(
                QPixmap(":/icons/images/icons/cloud_online.png")
            )
        elif state == BackendState.LOST.value:
            self.menu.label_connection_text.setText(_("BACKEND_STATE_DISCONNECTED"))
            self.menu.label_connection_icon.setPixmap(
                QPixmap(":/icons/images/icons/cloud_offline.png")
            )
        elif state == BackendState.INCOMPATIBLE_VERSION.value:
            self.menu.label_connection_text.setText(_("BACKEND_STATE_INCOMPATIBLE_VERSION"))
            self.menu.label_connection_icon.setPixmap(
                QPixmap(":/icons/images/icons/cloud_offline.png")
            )

    def on_new_notification(self, event, notif_type, msg):
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
        self.menu.activate_files()
        self.label_title.setText(_("MENU_DOCUMENTS"))
        self.set_taskbar_buttons(self.mount_widget.get_taskbar_buttons())
        self.mount_widget.show()
        self.mount_widget.show_workspaces_widget()

    def show_users_widget(self):
        self.clear_widgets()
        self.menu.activate_users()
        self.label_title.setText(_("MENU_USERS"))
        self.set_taskbar_buttons(self.users_widget.get_taskbar_buttons())
        self.users_widget.show()

    def show_devices_widget(self):
        self.clear_widgets()
        self.menu.activate_devices()
        self.label_title.setText(_("MENU_DEVICES"))
        self.set_taskbar_buttons(self.devices_widget.get_taskbar_buttons())
        self.devices_widget.show()

    def show_settings_widget(self):
        self.clear_widgets()
        self.menu.activate_settings()
        self.label_title.setText(_("MENU_SETTINGS"))
        self.set_taskbar_buttons([])
        self.settings_widget.show()

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
        self.users_widget.hide()
        self.mount_widget.hide()
        self.settings_widget.hide()
        self.devices_widget.hide()
