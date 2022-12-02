# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import time
from pathlib import PurePath
from typing import cast

from PyQt5.QtCore import pyqtBoundSignal, pyqtSignal
from PyQt5.QtGui import QColor, QIcon, QPixmap
from PyQt5.QtWidgets import QGraphicsDropShadowEffect, QMenu, QWidget

from parsec.api.data import EntryName
from parsec.api.data.manifest import WorkspaceEntry
from parsec.api.protocol import (
    HandshakeOrganizationExpired,
    HandshakeRevokedDevice,
    IncompatibleAPIVersionsError,
)
from parsec.core.backend_connection import (
    BackendConnectionError,
    BackendConnStatus,
    BackendNotAvailable,
)
from parsec.core.core_events import CoreEvent
from parsec.core.fs import (
    FsPath,
    FSWorkspaceInMaintenance,
    FSWorkspaceNoReadAccess,
    FSWorkspaceNotFoundError,
    FSWorkspaceNoWriteAccess,
    WorkspaceFSTimestamped,
)
from parsec.core.gui import desktop
from parsec.core.gui.authentication_change_widget import AuthenticationChangeWidget
from parsec.core.gui.commercial import is_saas_addr
from parsec.core.gui.custom_dialogs import show_error
from parsec.core.gui.custom_widgets import Pixmap
from parsec.core.gui.devices_widget import DevicesWidget
from parsec.core.gui.enrollment_widget import EnrollmentWidget
from parsec.core.gui.lang import translate as _
from parsec.core.gui.menu_widget import MenuWidget
from parsec.core.gui.mount_widget import MountWidget
from parsec.core.gui.organization_info_widget import OrganizationInfoWidget
from parsec.core.gui.snackbar_widget import SnackbarManager
from parsec.core.gui.trio_jobs import QtToTrioJobScheduler
from parsec.core.gui.ui.central_widget import Ui_CentralWidget
from parsec.core.gui.users_widget import UsersWidget
from parsec.core.logged_core import LoggedCore
from parsec.core.pki import is_pki_enrollment_available
from parsec.core.types import BackendOrganizationFileLinkAddr, UserInfo
from parsec.event_bus import EventBus, EventCallback


class GoToFileLinkError(Exception):
    pass


class GoToFileLinkBadOrganizationIDError(GoToFileLinkError):
    pass


class GoToFileLinkBadWorkspaceIDError(GoToFileLinkError):
    pass


class GoToFileLinkPathDecryptionError(GoToFileLinkError):
    pass


class CentralWidget(QWidget, Ui_CentralWidget):
    NOTIFICATION_EVENTS = [
        CoreEvent.BACKEND_CONNECTION_CHANGED,
        CoreEvent.MOUNTPOINT_STOPPED,
        CoreEvent.MOUNTPOINT_REMOTE_ERROR,
        CoreEvent.MOUNTPOINT_UNHANDLED_ERROR,
        CoreEvent.MOUNTPOINT_TRIO_DEADLOCK_ERROR,
        CoreEvent.MOUNTPOINT_READONLY,
        CoreEvent.SHARING_UPDATED,
    ]

    connection_state_changed = pyqtSignal(object, object)
    logout_requested = pyqtSignal()
    new_notification = pyqtSignal(str, str)

    def __init__(
        self,
        core: LoggedCore,
        jobs_ctx: QtToTrioJobScheduler,
        event_bus: EventBus,
        systray_notification: pyqtBoundSignal,
        file_link_addr: BackendOrganizationFileLinkAddr | None = None,
        parent: QWidget | None = None,
    ):
        super().__init__(parent=parent)

        self.setupUi(self)
        self.jobs_ctx = jobs_ctx
        self.core = core
        self.event_bus = event_bus
        self.systray_notification = systray_notification
        self.last_notification = 0.0
        self.desync_notified = False

        self.menu = MenuWidget(parent=self)
        self.widget_menu.layout().addWidget(self.menu)

        for e in self.NOTIFICATION_EVENTS:
            self.event_bus.connect(e, cast(EventCallback, self.handle_event))

        self.set_user_info()
        menu = QMenu()
        if self.core.device.is_admin and is_saas_addr(self.core.device.organization_addr):
            update_sub_act = menu.addAction(_("ACTION_UPDATE_SUBSCRIPTION"))
            update_sub_act.triggered.connect(self._on_update_subscription_clicked)
            menu.addSeparator()
        org_info_act = menu.addAction(_("ACTION_DEVICE_MENU_THIS_ORGANIZATION"))
        org_info_act.triggered.connect(self._on_organization_clicked)
        change_auth_act = menu.addAction(_("ACTION_DEVICE_MENU_CHANGE_AUTHENTICATION"))
        change_auth_act.triggered.connect(self.change_authentication)
        menu.addSeparator()
        log_out_act = menu.addAction(_("ACTION_LOG_OUT"))
        log_out_act.triggered.connect(self.logout_requested.emit)
        self.button_user.setMenu(menu)
        pix = Pixmap(":/icons/images/material/person.svg")
        pix.replace_color(QColor(0, 0, 0), QColor(0x00, 0x92, 0xFF))
        self.button_user.setIcon(QIcon(pix))
        self.button_user.clicked.connect(self._show_user_menu)

        self.new_notification.connect(self.on_new_notification)
        self.menu.button_enrollment.setVisible(
            self.core.device.is_admin and is_pki_enrollment_available()
        )
        if self.core.device.is_outsider:
            self.menu.button_users.hide()
        self.menu.files_clicked.connect(self.show_mount_widget)
        self.menu.users_clicked.connect(self.show_users_widget)
        self.menu.devices_clicked.connect(self.show_devices_widget)
        self.menu.enrollment_clicked.connect(self.show_enrollment_widget)
        self.connection_state_changed.connect(self._on_connection_state_changed)

        self.navigation_bar_widget.clear()
        self.navigation_bar_widget.route_clicked.connect(self._on_route_clicked)

        effect = QGraphicsDropShadowEffect(self)
        effect.setColor(QColor(100, 100, 100))
        effect.setBlurRadius(4)
        effect.setXOffset(-2)
        effect.setYOffset(2)
        self.widget_notif.setGraphicsEffect(effect)

        self.mount_widget = MountWidget(self.core, self.jobs_ctx, self.event_bus, parent=self)
        self.widget_central.layout().insertWidget(0, self.mount_widget)
        self.mount_widget.folder_changed.connect(self._on_folder_changed)

        self.users_widget = UsersWidget(self.core, self.jobs_ctx, self.event_bus, parent=self)
        self.users_widget.filter_shared_workspaces_request.connect(self.show_mount_widget)
        self.widget_central.layout().insertWidget(0, self.users_widget)

        self.devices_widget = DevicesWidget(self.core, self.jobs_ctx, self.event_bus, parent=self)
        self.widget_central.layout().insertWidget(0, self.devices_widget)

        self.enrollment_widget = EnrollmentWidget(
            self.core, self.jobs_ctx, self.event_bus, parent=self
        )
        self.widget_central.layout().insertWidget(0, self.enrollment_widget)

        self._on_connection_state_changed(
            self.core.backend_status, self.core.backend_status_exc, allow_systray=False
        )
        if file_link_addr is not None:
            try:
                self.go_to_file_link(file_link_addr)
            except GoToFileLinkBadWorkspaceIDError:
                show_error(
                    self,
                    _("TEXT_FILE_LINK_WORKSPACE_NOT_FOUND_organization").format(
                        organization=file_link_addr.organization_id.str
                    ),
                )
                self.show_mount_widget()
            except GoToFileLinkPathDecryptionError:
                show_error(self, _("TEXT_INVALID_URL"))
                self.show_mount_widget()
            except GoToFileLinkBadOrganizationIDError:
                show_error(
                    self,
                    _("TEXT_FILE_LINK_NOT_IN_ORG_organization").format(
                        organization=file_link_addr.organization_id.str
                    ),
                )
                self.show_mount_widget()
        else:
            self.show_mount_widget()

    def _show_user_menu(self) -> None:
        self.button_user.showMenu()

    def _on_update_subscription_clicked(self) -> None:
        desktop.open_url(_("TEXT_SAAS_UPDATE_SUBSCRIPTION_URL"))

    async def _on_organization_clicked(self) -> None:
        stats = None
        config = None
        try:
            stats = await self.core.get_organization_stats()
        except (BackendNotAvailable, BackendConnectionError):
            pass
        try:
            config = self.core.get_organization_config()
        except (BackendNotAvailable, BackendConnectionError):
            pass

        await OrganizationInfoWidget.show_modal(
            profile=self.core.device.profile,
            org_id=self.core.device.organization_id,
            org_addr=self.core.device.organization_addr.to_url(),
            stats=stats,
            config=config,
            parent=self,
        )

    def set_user_info(self) -> None:
        org = self.core.device.organization_id.str
        username = self.core.device.short_user_display
        user_text = f"{org}\n{username}"
        self.button_user.setText(user_text)
        self.button_user.setToolTip(self.core.device.organization_addr.to_url())

    async def change_authentication(self) -> None:
        await AuthenticationChangeWidget.show_modal(
            core=self.core, jobs_ctx=self.jobs_ctx, parent=self, on_finished=None
        )

    def _on_route_clicked(self, path: FsPath) -> None:
        self.mount_widget.load_path(path)

    def _on_folder_changed(self, workspace_name: EntryName | None, path: str | None) -> None:
        if workspace_name and path:
            self.navigation_bar_widget.from_path(workspace_name, path)
        else:
            self.navigation_bar_widget.clear()

    def handle_event(self, event: CoreEvent, **kwargs: object) -> None:
        if event == CoreEvent.BACKEND_CONNECTION_CHANGED:
            assert isinstance(kwargs["status"], BackendConnStatus)
            assert kwargs["status_exc"] is None or isinstance(kwargs["status_exc"], Exception)
            self.connection_state_changed.emit(kwargs["status"], kwargs["status_exc"])
        elif event == CoreEvent.MOUNTPOINT_READONLY:
            current_time = time.time()
            if self.last_notification == 0.0 or current_time >= self.last_notification + 3:  # 3s
                self.systray_notification.emit(
                    "", _("TEXT_NOTIF_INFO_WORKSPACE_READ_ONLY"), 3000
                )  # 3000ms
                self.last_notification = current_time
        elif event == CoreEvent.MOUNTPOINT_STOPPED:
            pass
            # Not taking this event into account since we cannot yet distinguish
            # whether the workspace has been unmounted by the user or not.
            # self.new_notification.emit("INFO", _("NOTIF_WARN_MOUNTPOINT_UNMOUNTED"))
        elif event == CoreEvent.MOUNTPOINT_REMOTE_ERROR:
            assert isinstance(kwargs["exc"], Exception)
            assert isinstance(kwargs["mountpoint"], PurePath)
            assert isinstance(kwargs["path"], FsPath)
            exc = kwargs["exc"]
            mountpoint = kwargs["mountpoint"]
            if isinstance(exc, FSWorkspaceNoReadAccess):
                msg = _("TEXT_NOTIF_WARN_WORKSPACE_READ_ACCESS_LOST_workspace").format(
                    workspace=str(mountpoint)
                )
            elif isinstance(exc, FSWorkspaceNoWriteAccess):
                msg = _("TEXT_NOTIF_WARN_WORKSPACE_WRITE_ACCESS_LOST_workspace").format(
                    workspace=str(mountpoint)
                )
            elif isinstance(exc, FSWorkspaceInMaintenance):
                msg = _("TEXT_NOTIF_WARN_WORKSPACE_IN_MAINTENANCE_workspace").format(
                    workspace=str(mountpoint)
                )
            else:
                msg = _("TEXT_NOTIF_WARN_MOUNTPOINT_REMOTE_ERROR_workspace-error").format(
                    workspace=str(mountpoint), error=str(exc)
                )
            self.new_notification.emit("WARN", msg)
        elif event in (
            CoreEvent.MOUNTPOINT_UNHANDLED_ERROR,
            CoreEvent.MOUNTPOINT_TRIO_DEADLOCK_ERROR,
        ):
            assert isinstance(kwargs["exc"], Exception)
            assert isinstance(kwargs["operation"], str)
            assert isinstance(kwargs["mountpoint"], PurePath)
            assert isinstance(kwargs["path"], FsPath)
            exc = kwargs["exc"]
            self.new_notification.emit(
                "WARN",
                _("TEXT_NOTIF_ERR_MOUNTPOINT_UNEXPECTED_ERROR_workspace_operation_error").format(
                    operation=kwargs["operation"],
                    workspace=str(kwargs["mountpoint"]),
                    error=str(kwargs["exc"]),
                ),
            )
        elif event == CoreEvent.SHARING_UPDATED:
            assert isinstance(kwargs["new_entry"], WorkspaceEntry)
            assert kwargs["previous_entry"] is None or isinstance(
                kwargs["previous_entry"], WorkspaceEntry
            )
            new_entry: WorkspaceEntry = kwargs["new_entry"]
            previous_entry: WorkspaceEntry | None = kwargs["previous_entry"]
            new_role = new_entry.role
            previous_role = previous_entry.role if previous_entry is not None else None
            if new_role is not None and previous_role is None:
                self.new_notification.emit(
                    "INFO",
                    _("TEXT_NOTIF_INFO_WORKSPACE_SHARED_workspace").format(
                        workspace=new_entry.name.str
                    ),
                )
            elif new_role is not None and previous_role is not None and new_role != previous_role:
                self.new_notification.emit(
                    "INFO",
                    _("TEXT_NOTIF_INFO_WORKSPACE_ROLE_UPDATED_workspace").format(
                        workspace=new_entry.name.str
                    ),
                )
            elif new_role is None and previous_role is not None:
                name = previous_entry.name  # type: ignore
                self.new_notification.emit(
                    "INFO",
                    _("TEXT_NOTIF_INFO_WORKSPACE_UNSHARED_workspace").format(workspace=name.str),
                )

    def _on_connection_state_changed(
        self, status: BackendConnStatus, status_exc: Exception | None, allow_systray: bool = True
    ) -> None:
        text: str | None = None
        icon: QPixmap | None = None
        tooltip: str | None = None
        notif = None
        disconnected = None

        if status in (BackendConnStatus.READY, BackendConnStatus.INITIALIZING):
            tooltip = text = _("TEXT_BACKEND_STATE_CONNECTED")
            icon = QPixmap(":/icons/images/material/cloud_queue.svg")

        elif status == BackendConnStatus.LOST:
            tooltip = text = _("TEXT_BACKEND_STATE_DISCONNECTED")
            icon = QPixmap(":/icons/images/material/cloud_off.svg")
            disconnected = True

        elif status == BackendConnStatus.REFUSED:
            disconnected = True
            text = _("TEXT_BACKEND_STATE_DISCONNECTED")
            icon = QPixmap(":/icons/images/material/cloud_off.svg")
            assert isinstance(status_exc, Exception)
            cause = status_exc.__cause__
            if isinstance(cause, IncompatibleAPIVersionsError):
                tooltip = text = _("TEXT_BACKEND_STATE_API_MISMATCH_versions").format(
                    versions=", ".join([str(v.version) for v in cause.backend_versions])
                )
            elif isinstance(cause, HandshakeRevokedDevice):
                tooltip = _("TEXT_BACKEND_STATE_REVOKED_DEVICE")
                notif = ("WARN", tooltip)
            elif isinstance(cause, HandshakeOrganizationExpired):
                tooltip = _("TEXT_BACKEND_STATE_ORGANIZATION_EXPIRED")
                notif = ("WARN", tooltip)
            else:
                tooltip = _("TEXT_BACKEND_STATE_UNKNOWN")
                notif = ("WARN", tooltip)

        elif status == BackendConnStatus.CRASHED:
            assert isinstance(status_exc, Exception)
            text = _("TEXT_BACKEND_STATE_DISCONNECTED")
            tooltip = _("TEXT_BACKEND_STATE_CRASHED_cause").format(cause=str(status_exc.__cause__))
            icon = QPixmap(":/icons/images/material/cloud_off.svg")
            notif = ("WARN", tooltip)
            disconnected = True

        elif status == BackendConnStatus.DESYNC:
            assert isinstance(status_exc, Exception)
            text = _("TEXT_BACKEND_STATE_DISCONNECTED")
            tooltip = _("TEXT_BACKEND_STATE_DESYNC")
            icon = QPixmap(":/icons/images/material/cloud_off.svg")
            notif = None
            disconnected = False

            # The disconnection for being out-of-sync with the backend
            # is only shown once per login. This is useful in the case
            # of backends with API version 2.3 and older as it's going
            # to successfully connect every 10 seconds before being
            # thrown off by the sync monitor.
            if not self.desync_notified:
                self.desync_notified = True
                notif = ("WARN", tooltip)
                disconnected = True

        # Theses variables should not be None as they're assigned above
        assert text is not None
        assert tooltip is not None
        assert icon is not None
        self.menu.set_connection_state(text, tooltip, icon)
        if notif:
            self.new_notification.emit(*notif)
        if allow_systray and disconnected:
            self.systray_notification.emit(
                "Parsec",
                _("TEXT_SYSTRAY_BACKEND_DISCONNECT_organization").format(
                    organization=self.core.device.organization_id.str
                ),
                5000,
            )

    def on_new_notification(self, notif_type: str, msg: str) -> None:
        if notif_type == "CONGRATULATE":
            SnackbarManager.congratulate(msg)
        elif notif_type == "WARN":
            SnackbarManager.warn(msg)
        else:
            SnackbarManager.inform(msg)

    def go_to_file_link(self, addr: BackendOrganizationFileLinkAddr, mount: bool = True) -> None:
        """
        Raises:
            GoToFileLinkBadOrganizationIDError
            GoToFileLinkBadWorkspaceIDError
            GoToFileLinkPathDecryptionError
        """
        if addr.organization_id != self.core.device.organization_id:
            raise GoToFileLinkBadOrganizationIDError
        try:
            workspace = self.core.user_fs.get_workspace(addr.workspace_id)
        except FSWorkspaceNotFoundError as exc:
            raise GoToFileLinkBadWorkspaceIDError from exc
        try:
            path = workspace.decrypt_file_link_path(addr)
            ts = workspace.decrypt_timestamp(addr)
        except ValueError as exc:
            raise GoToFileLinkPathDecryptionError from exc

        self.show_mount_widget()
        self.mount_widget.show_files_widget(
            WorkspaceFSTimestamped(workspace, ts) if ts is not None else workspace,
            path,
            selected=True,
            mount_it=mount,
            timestamp=ts,
        )

    def show_mount_widget(self, user_info: UserInfo | None = None) -> None:
        self.clear_widgets()
        self.menu.activate_files()
        self.label_title.setText(_("ACTION_MENU_DOCUMENTS"))
        if user_info is not None:
            self.mount_widget.workspaces_widget.set_user_info(user_info)
        self.mount_widget.show()
        self.mount_widget.show_workspaces_widget()

    def show_users_widget(self) -> None:
        self.clear_widgets()
        self.menu.activate_users()
        self.label_title.setText(_("ACTION_MENU_USERS"))
        self.users_widget.show()

    def show_devices_widget(self) -> None:
        self.clear_widgets()
        self.menu.activate_devices()
        self.label_title.setText(_("ACTION_MENU_DEVICES"))
        self.devices_widget.show()

    def show_enrollment_widget(self) -> None:
        self.clear_widgets()
        self.menu.activate_enrollment()
        self.label_title.setText(_("ACTION_MENU_ENROLLMENT"))
        self.enrollment_widget.show()

    def clear_widgets(self) -> None:
        self.navigation_bar_widget.clear()
        self.users_widget.hide()
        self.mount_widget.hide()
        self.devices_widget.hide()
        self.enrollment_widget.hide()
