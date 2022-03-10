# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from typing import Optional, cast
from PyQt5.QtCore import pyqtSignal, pyqtBoundSignal
from PyQt5.QtGui import QPixmap, QColor, QIcon
from PyQt5.QtWidgets import QGraphicsDropShadowEffect, QWidget, QMenu
from pathlib import PurePath

from parsec.event_bus import EventBus, EventCallback
from parsec.api.protocol import (
    HandshakeAPIVersionError,
    HandshakeRevokedDevice,
    HandshakeOrganizationExpired,
)
from parsec.api.data import EntryName
from parsec.api.data.manifest import WorkspaceEntry
from parsec.core.core_events import CoreEvent
from parsec.core.logged_core import LoggedCore, OrganizationStats
from parsec.core.types import UserInfo, BackendOrganizationFileLinkAddr
from parsec.core.fs import FsPath
from parsec.core.backend_connection import (
    BackendConnectionError,
    BackendNotAvailable,
    BackendConnStatus,
)
from parsec.core.fs import FSWorkspaceNotFoundError
from parsec.core.fs import (
    FSWorkspaceNoReadAccess,
    FSWorkspaceNoWriteAccess,
    FSWorkspaceInMaintenance,
)
from parsec.core.gui.trio_jobs import QtToTrioJobScheduler
from parsec.core.gui.snackbar_widget import SnackbarManager
from parsec.core.gui.mount_widget import MountWidget
from parsec.core.gui.users_widget import UsersWidget
from parsec.core.gui.devices_widget import DevicesWidget
from parsec.core.gui.menu_widget import MenuWidget
from parsec.core.gui import desktop
from parsec.core.gui.authentication_change_widget import AuthenticationChangeWidget
from parsec.core.gui.lang import translate as _
from parsec.core.gui.custom_widgets import Pixmap
from parsec.core.gui.custom_dialogs import show_error
from parsec.core.gui.ui.central_widget import Ui_CentralWidget
from parsec.core.gui.trio_jobs import JobResultError, QtToTrioJob
import time


async def _do_get_organization_stats(core: LoggedCore) -> OrganizationStats:
    try:
        return await core.get_organization_stats()
    except BackendNotAvailable as exc:
        raise JobResultError("offline") from exc
    except BackendConnectionError as exc:
        raise JobResultError("error") from exc


class GoToFileLinkError(Exception):
    pass


class GoToFileLinkBadOrganizationIDError(Exception):
    pass


class GoToFileLinkBadWorkspaceIDError(Exception):
    pass


class GoToFileLinkPathDecryptionError(Exception):
    pass


class CentralWidget(QWidget, Ui_CentralWidget):  # type: ignore[misc]
    NOTIFICATION_EVENTS = [
        CoreEvent.BACKEND_CONNECTION_CHANGED,
        CoreEvent.MOUNTPOINT_STOPPED,
        CoreEvent.MOUNTPOINT_REMOTE_ERROR,
        CoreEvent.MOUNTPOINT_UNHANDLED_ERROR,
        CoreEvent.MOUNTPOINT_TRIO_DEADLOCK_ERROR,
        CoreEvent.MOUNTPOINT_READONLY,
        CoreEvent.SHARING_UPDATED,
    ]

    organization_stats_success = pyqtSignal(QtToTrioJob)
    organization_stats_error = pyqtSignal(QtToTrioJob)

    connection_state_changed = pyqtSignal(object, object)
    logout_requested = pyqtSignal()
    new_notification = pyqtSignal(str, str)

    REFRESH_ORGANIZATION_STATS_DELAY = 5  # 5s

    def __init__(
        self,
        core: LoggedCore,
        jobs_ctx: QtToTrioJobScheduler,
        event_bus: EventBus,
        systray_notification: pyqtBoundSignal,
        file_link_addr: Optional[BackendOrganizationFileLinkAddr] = None,
        parent: Optional[QWidget] = None,
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

        self.event_bus.connect(CoreEvent.FS_ENTRY_SYNCED, self._on_vlobs_updated)
        self.event_bus.connect(CoreEvent.BACKEND_REALM_VLOBS_UPDATED, self._on_vlobs_updated)

        self.set_user_info()
        menu = QMenu()
        copy_backend_addr_act = menu.addAction(_("ACTION_COPY_BACKEND_ADDR"))
        copy_backend_addr_act.triggered.connect(self._on_copy_backend_addr)
        menu.addSeparator()
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
        self.menu.files_clicked.connect(self.show_mount_widget)
        if self.core.device.is_outsider:
            self.menu.button_users.hide()
        else:
            self.menu.users_clicked.connect(self.show_users_widget)
        self.menu.devices_clicked.connect(self.show_devices_widget)
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

        self.organization_stats_success.connect(self._on_organization_stats_success)
        self.organization_stats_error.connect(self._on_organization_stats_error)

        self.users_widget = UsersWidget(self.core, self.jobs_ctx, self.event_bus, parent=self)
        self.users_widget.filter_shared_workspaces_request.connect(self.show_mount_widget)
        self.widget_central.layout().insertWidget(0, self.users_widget)

        self.devices_widget = DevicesWidget(self.core, self.jobs_ctx, self.event_bus, parent=self)
        self.widget_central.layout().insertWidget(0, self.devices_widget)

        self._on_connection_state_changed(
            self.core.backend_status, self.core.backend_status_exc, allow_systray=False
        )
        if file_link_addr is not None:
            try:
                self.go_to_file_link(file_link_addr)
            except FSWorkspaceNotFoundError:
                show_error(
                    self,
                    _("TEXT_FILE_LINK_WORKSPACE_NOT_FOUND_organization").format(
                        organization=file_link_addr.organization_id
                    ),
                )

                self.show_mount_widget()
        else:
            self.show_mount_widget()

    def _show_user_menu(self) -> None:
        self.button_user.showMenu()

    def _on_copy_backend_addr(self) -> None:
        desktop.copy_to_clipboard(self.core.device.organization_addr.to_url())
        SnackbarManager.inform(_("TEXT_BACKEND_ADDR_COPIED_TO_CLIPBOARD"))

    def set_user_info(self) -> None:
        org = self.core.device.organization_id
        username = self.core.device.short_user_display
        user_text = f"{org}\n{username}"
        self.button_user.setText(user_text)
        self.button_user.setToolTip(self.core.device.organization_addr.to_url())

    def change_authentication(self) -> None:
        AuthenticationChangeWidget.show_modal(
            core=self.core, jobs_ctx=self.jobs_ctx, parent=self, on_finished=None
        )

    def _on_route_clicked(self, path: FsPath) -> None:
        self.mount_widget.load_path(path)

    def _on_folder_changed(self, workspace_name: Optional[EntryName], path: Optional[str]) -> None:
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
            abspath = kwargs["path"].with_mountpoint(kwargs["mountpoint"])
            if isinstance(exc, FSWorkspaceNoReadAccess):
                msg = _("TEXT_NOTIF_WARN_WORKSPACE_READ_ACCESS_LOST_workspace").format(
                    workspace=abspath
                )
            elif isinstance(exc, FSWorkspaceNoWriteAccess):
                msg = _("TEXT_NOTIF_WARN_WORKSPACE_WRITE_ACCESS_LOST_workspace").format(
                    workspace=abspath
                )
            elif isinstance(exc, FSWorkspaceInMaintenance):
                msg = _("TEXT_NOTIF_WARN_WORKSPACE_IN_MAINTENANCE_workspace").format(
                    workspace=abspath
                )
            else:
                msg = _("TEXT_NOTIF_WARN_MOUNTPOINT_REMOTE_ERROR_workspace-error").format(
                    workspace=abspath, error=str(exc)
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
            abspath = kwargs["path"].with_mountpoint(kwargs["mountpoint"])
            self.new_notification.emit(
                "WARN",
                _("TEXT_NOTIF_ERR_MOUNTPOINT_UNEXPECTED_ERROR_workspace_operation_error").format(
                    operation=kwargs["operation"], workspace=abspath, error=str(kwargs["exc"])
                ),
            )
        elif event == CoreEvent.SHARING_UPDATED:
            assert isinstance(kwargs["new_entry"], WorkspaceEntry)
            assert kwargs["previous_entry"] is None or isinstance(
                kwargs["previous_entry"], WorkspaceEntry
            )
            new_entry: WorkspaceEntry = kwargs["new_entry"]
            previous_entry: Optional[WorkspaceEntry] = kwargs["previous_entry"]
            new_role = new_entry.role
            previous_role = previous_entry.role if previous_entry is not None else None
            print(previous_role, new_role)
            if new_role is not None and previous_role is None:
                self.new_notification.emit(
                    "INFO",
                    _("TEXT_NOTIF_INFO_WORKSPACE_SHARED_workspace").format(
                        workspace=new_entry.name
                    ),
                )
            elif new_role is not None and previous_role is not None and new_role != previous_role:
                self.new_notification.emit(
                    "INFO",
                    _("TEXT_NOTIF_INFO_WORKSPACE_ROLE_UPDATED_workspace").format(
                        workspace=new_entry.name
                    ),
                )
            elif new_role is None and previous_role is not None:
                name = previous_entry.name  # type: ignore
                self.new_notification.emit(
                    "INFO", _("TEXT_NOTIF_INFO_WORKSPACE_UNSHARED_workspace").format(workspace=name)
                )

    def _load_organization_stats(self, delay: float = 0) -> None:
        self.jobs_ctx.submit_throttled_job(
            "central_widget.load_organization_stats",
            delay,
            self.organization_stats_success,
            self.organization_stats_error,
            _do_get_organization_stats,
            core=self.core,
        )

    def _on_vlobs_updated(self, *args: object, **kwargs: object) -> None:
        self._load_organization_stats(delay=self.REFRESH_ORGANIZATION_STATS_DELAY)

    def _on_connection_state_changed(
        self, status: BackendConnStatus, status_exc: Optional[Exception], allow_systray: bool = True
    ) -> None:
        text = None
        icon = None
        tooltip = None
        notif = None
        disconnected = None

        self.menu.label_organization_name.hide()
        self.menu.label_organization_size.clear()
        if status in (BackendConnStatus.READY, BackendConnStatus.INITIALIZING):
            if status == BackendConnStatus.READY and self.core.device.is_admin:
                self._load_organization_stats()
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
            if isinstance(cause, HandshakeAPIVersionError):
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

        self.menu.set_connection_state(text, tooltip, icon)
        if notif:
            self.new_notification.emit(*notif)
        if allow_systray and disconnected:
            self.systray_notification.emit(
                "Parsec",
                _("TEXT_SYSTRAY_BACKEND_DISCONNECT_organization").format(
                    organization=self.core.device.organization_id
                ),
                5000,
            )

    def _on_organization_stats_success(self, job: QtToTrioJob) -> None:
        assert job.is_finished()
        assert job.status == "ok"

        organization_stats = job.ret
        self.menu.show_organization_stats(
            organization_id=self.core.device.organization_id, organization_stats=organization_stats
        )

    def _on_organization_stats_error(self, job: QtToTrioJob) -> None:
        assert job.is_finished()
        assert job.status != "ok"
        self.menu.label_organization_name.hide()
        self.menu.label_organization_size.clear()

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
        except ValueError as exc:
            raise GoToFileLinkPathDecryptionError from exc

        self.show_mount_widget()
        self.mount_widget.show_files_widget(workspace, path, selected=True, mount_it=mount)

    def show_mount_widget(self, user_info: Optional[UserInfo] = None) -> None:
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

    def clear_widgets(self) -> None:
        self.navigation_bar_widget.clear()
        self.users_widget.hide()
        self.mount_widget.hide()
        self.devices_widget.hide()
