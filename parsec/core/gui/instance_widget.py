# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations
from pathlib import Path

import trio
from typing import Any, Optional, Tuple, cast
from structlog import get_logger
from PyQt5.QtCore import QObject, pyqtSignal, pyqtBoundSignal
from PyQt5.QtWidgets import QWidget, QVBoxLayout, QApplication, QMainWindow
from packaging.version import Version
import exceptiongroup

from parsec.core.logged_core import LoggedCore
from parsec.core.types import BackendActionAddr, BackendOrganizationFileLinkAddr, LocalDevice
from parsec.event_bus import EventBus, EventCallback
from parsec.api.protocol import HandshakeRevokedDevice
from parsec.core import logged_core_factory
from parsec.core.local_device import (
    LocalDeviceError,
    LocalDeviceCertificatePinCodeUnavailableError,
    load_device_with_password,
    load_device_with_smartcard,
)
from parsec.core.mountpoint import (
    MountpointConfigurationError,
    MountpointDriverCrash,
    MountpointFuseNotAvailable,
    MountpointWinfspNotAvailable,
)

from parsec.core.config import CoreConfig
from parsec.core.core_events import CoreEvent
from parsec.core.gui.trio_jobs import QtToTrioJobScheduler, QtToTrioJob, run_trio_job_scheduler
from parsec.core.gui.parsec_application import ParsecApp
from parsec.core.gui.custom_dialogs import show_error, show_info_link
from parsec.core.gui.lang import translate as _
from parsec.core.gui.login_widget import LoginWidget
from parsec.core.gui.central_widget import CentralWidget


logger = get_logger()

MIN_MACFUSE_VERSION = Version("4.4.0")


async def _do_run_core(
    config: CoreConfig, device: LocalDevice, qt_on_ready: Tuple[QObject, str]
) -> None:
    qt_on_ready: pyqtBoundSignal = getattr(qt_on_ready[0], qt_on_ready[1])  # Retreive the signal
    # Quick fix to avoid BaseExceptionGroup<Cancelled, ...> exception bubbling up
    # TODO: is it still necessary?
    with exceptiongroup.catch({trio.Cancelled: lambda _: None}):
        async with logged_core_factory(config=config, device=device, event_bus=None) as core:
            # Create our own job scheduler allows us to cancel all pending
            # jobs depending on us when we logout
            async with run_trio_job_scheduler() as core_jobs_ctx:
                qt_on_ready.emit(core, core_jobs_ctx)
                await trio.sleep_forever()


def check_macfuse_version() -> bool:
    import os

    macfuse_plist_path = "/Library/Filesystems/macfuse.fs/Contents/Info.plist"
    if not os.path.exists(macfuse_plist_path):
        # If macFUSE is installed, this file should exist at this path. If not,
        # it probably means that its previous version, osxfuse, is installed
        # instead (in a "osxfuse.fs" directory). Any current MacOS version is
        # incompatible with osxfuse.
        return False

    import plistlib

    try:
        with open(macfuse_plist_path, "rb") as plist_file:
            local_version = Version(plistlib.load(plist_file)["CFBundleVersion"])
    except ValueError:
        return False

    return local_version >= MIN_MACFUSE_VERSION


def ensure_macfuse_available_or_show_dialogue(window: QMainWindow) -> None:
    try:
        import fuse  # noqa
    except OSError:
        show_info_link(
            window,
            _("TEXT_MACFUSE_INSTALL_WINDOW_TITLE"),
            _("TEXT_LOGIN_ERROR_MACFUSE_NOT_AVAILABLE"),
            _("TEXT_MACFUSE_DOWNLOAD_BUTTON"),
            "https://osxfuse.github.io",
        )
    else:
        if not check_macfuse_version():
            show_info_link(
                window,
                _("TEXT_UPDATE_MACFUSE_TITLE"),
                _("TEXT_UPDATE_MACFUSE"),
                _("TEXT_UPDATE_MACFUSE_BUTTON"),
                "https://osxfuse.github.io",
            )


class InstanceWidget(QWidget):
    run_core_success = pyqtSignal(QtToTrioJob)
    run_core_error = pyqtSignal(QtToTrioJob)
    run_core_ready = pyqtSignal(object, object)
    logged_in = pyqtSignal()
    logged_out = pyqtSignal()
    state_changed = pyqtSignal(QWidget, str)
    login_failed = pyqtSignal()
    join_organization_clicked = pyqtSignal()
    create_organization_clicked = pyqtSignal()
    recover_device_clicked = pyqtSignal()

    def __init__(
        self,
        jobs_ctx: QtToTrioJobScheduler,
        event_bus: EventBus,
        config: CoreConfig,
        systray_notification: pyqtBoundSignal,
        **kwargs: Any,
    ) -> None:
        super().__init__(**kwargs)
        self.jobs_ctx = jobs_ctx
        self.event_bus = event_bus
        self.config = config
        self.systray_notification = systray_notification

        self.core: Optional[LoggedCore] = None
        self.core_jobs_ctx: Optional[QtToTrioJobScheduler] = None
        self.running_core_job = None
        self.workspace_path: Optional[BackendActionAddr] = None

        self.run_core_success.connect(self.on_core_run_done)
        self.run_core_error.connect(self.on_core_run_error)
        self.run_core_ready.connect(self.on_run_core_ready)
        self.logged_in.connect(self.on_logged_in)
        self.logged_out.connect(self.on_logged_out)

        layout = QVBoxLayout()
        layout.setContentsMargins(0, 0, 0, 0)
        self.setLayout(layout)

    @property
    def current_device(self) -> Optional[LocalDevice]:
        if self.core:
            return self.core.device
        return None

    @property
    def is_logged_in(self) -> bool:
        return self.running_core_job is not None

    def set_workspace_path(self, action_addr: BackendActionAddr) -> None:
        self.workspace_path = action_addr

    def reset_workspace_path(self) -> None:
        self.workspace_path = None

    def on_core_config_updated(self, event: CoreEvent, **kwargs: dict[object, object]) -> None:
        self.event_bus.send(CoreEvent.GUI_CONFIG_CHANGED, **kwargs)

    def start_core(self, device: LocalDevice) -> None:
        assert not self.running_core_job
        assert not self.core
        assert not self.core_jobs_ctx

        main_win = ParsecApp.get_main_window()
        assert main_win is not None, "Main window is None"
        self.config = main_win.config

        self.running_core_job = self.jobs_ctx.submit_job(
            (self, "run_core_success"),
            (self, "run_core_error"),
            _do_run_core,
            self.config,
            device,
            (self, "run_core_ready"),
        )

    def on_run_core_ready(self, core: LoggedCore, core_jobs_ctx: QtToTrioJobScheduler) -> None:
        self.core = core
        self.core_jobs_ctx = core_jobs_ctx
        self.core.event_bus.connect(
            CoreEvent.GUI_CONFIG_CHANGED, cast(EventCallback, self.on_core_config_updated)
        )
        self.event_bus.send(
            CoreEvent.GUI_CONFIG_CHANGED, gui_last_device=self.core.device.device_id.str
        )
        ParsecApp.add_connected_device(
            self.core.device.organization_addr.organization_id, self.core.device.device_id
        )
        self.logged_in.emit()

    def on_core_run_error(self, job: QtToTrioJob) -> None:
        assert job is self.running_core_job
        assert self.running_core_job.is_finished()
        if self.core:
            self.core.event_bus.disconnect(
                CoreEvent.GUI_CONFIG_CHANGED, self.on_core_config_updated
            )
        if self.running_core_job.status is not None:
            if isinstance(self.running_core_job.exc, HandshakeRevokedDevice):
                show_error(
                    self, _("TEXT_LOGIN_ERROR_DEVICE_REVOKED"), exception=self.running_core_job.exc
                )
            elif isinstance(self.running_core_job.exc, MountpointWinfspNotAvailable):
                show_error(
                    self,
                    _("TEXT_LOGIN_ERROR_WINFSP_NOT_AVAILABLE"),
                    exception=self.running_core_job.exc,
                )
            elif isinstance(self.running_core_job.exc, MountpointFuseNotAvailable):
                show_error(
                    self,
                    _("TEXT_LOGIN_ERROR_FUSE_NOT_AVAILABLE"),
                    exception=self.running_core_job.exc,
                )
            else:
                logger.exception("Unhandled error", exc_info=self.running_core_job.exc)
                show_error(self, _("TEXT_LOGIN_UNKNOWN_ERROR"), exception=self.running_core_job.exc)
        self.running_core_job = None
        self.logged_out.emit()

    def on_core_run_done(self, job: QtToTrioJob) -> None:
        assert job is self.running_core_job
        assert self.running_core_job.is_finished()
        if self.core:
            ParsecApp.remove_connected_device(
                self.core.device.organization_addr.organization_id, self.core.device.device_id
            )
            self.core.event_bus.disconnect(
                CoreEvent.GUI_CONFIG_CHANGED, self.on_core_config_updated
            )
        self.running_core_job = None
        self.logged_out.emit()

    def stop_core(self) -> None:
        if self.running_core_job:
            self.running_core_job.cancel()

    def on_logged_out(self) -> None:
        self.state_changed.emit(self, "logout")
        self.show_login_widget()

    def on_logged_in(self) -> None:
        self.state_changed.emit(self, "connected")
        self.show_central_widget()

    def logout(self) -> None:
        self.stop_core()

    def login_with_password(self, key_file: Path, password: str) -> None:
        message = None
        exception: Optional[Exception] = None
        try:
            device = load_device_with_password(key_file, password)
            if ParsecApp.is_device_connected(
                device.organization_addr.organization_id, device.device_id
            ):
                message = _("TEXT_LOGIN_ERROR_ALREADY_CONNECTED")
            else:
                self.start_core(device)
        except LocalDeviceError as exc:
            message = _("TEXT_LOGIN_ERROR_AUTHENTICATION_FAILED")
            exception = exc

        except (RuntimeError, MountpointConfigurationError, MountpointDriverCrash) as exc:
            message = _("TEXT_LOGIN_MOUNTPOINT_ERROR")
            exception = exc

        except Exception as exc:
            message = _("TEXT_LOGIN_UNKNOWN_ERROR")
            exception = exc
            logger.exception("Unhandled error during login")
        finally:
            if message:
                show_error(self, message, exception=exception)
                self.login_failed.emit()

    async def login_with_smartcard(self, key_file: Path) -> None:
        message = None
        exception: Optional[Exception] = None
        try:
            device = await load_device_with_smartcard(key_file)
            if ParsecApp.is_device_connected(
                device.organization_addr.organization_id, device.device_id
            ):
                message = _("TEXT_LOGIN_ERROR_ALREADY_CONNECTED")
            else:
                self.start_core(device)
        except LocalDeviceCertificatePinCodeUnavailableError:
            # User cancelled the prompt
            self.login_failed.emit()
        except LocalDeviceError as exc:
            message = _("TEXT_LOGIN_ERROR_AUTHENTICATION_FAILED")
            exception = exc
        except ModuleNotFoundError as exc:
            message = _("TEXT_LOGIN_SMARTCARD_NOT_AVAILABLE")
            exception = exc
        except (RuntimeError, MountpointConfigurationError, MountpointDriverCrash) as exc:
            message = _("TEXT_LOGIN_MOUNTPOINT_ERROR")
            exception = exc
        except Exception as exc:
            message = _("TEXT_LOGIN_UNKNOWN_ERROR")
            exception = exc
            logger.exception("Unhandled error during login")
        finally:
            if message:
                show_error(self, message, exception=exception)
                self.login_failed.emit()

    def show_central_widget(self) -> None:
        self.clear_widgets()
        # The core can be set to None at any time if do_run_core get an error, is cancelled or
        # terminate.
        core = self.core
        core_jobs_ctx = self.core_jobs_ctx
        if core is None or core_jobs_ctx is None:
            return
        central_widget = CentralWidget(
            core=core,
            jobs_ctx=core_jobs_ctx,
            event_bus=core.event_bus,
            systray_notification=self.systray_notification,
            file_link_addr=cast(BackendOrganizationFileLinkAddr, self.workspace_path),
            parent=self,
        )
        self.layout().addWidget(central_widget)
        central_widget.logout_requested.connect(self.logout)
        central_widget.show()

    def show_login_widget(self) -> None:
        self.clear_widgets()
        login_widget = LoginWidget(
            self.jobs_ctx, self.event_bus, self.config, self.login_failed, parent=self
        )
        self.layout().addWidget(login_widget)

        login_widget.login_with_password_clicked.connect(self.login_with_password)
        login_widget.login_with_smartcard_clicked.connect(self.login_with_smartcard)
        login_widget.join_organization_clicked.connect(self.join_organization_clicked.emit)
        login_widget.create_organization_clicked.connect(self.create_organization_clicked.emit)
        login_widget.recover_device_clicked.connect(self.recover_device_clicked.emit)
        login_widget.login_canceled.connect(self.reset_workspace_path)
        login_widget.show()

    def get_central_widget(self) -> Optional[CentralWidget]:
        item = self.layout().itemAt(0)
        if item:
            if isinstance(item.widget(), CentralWidget):
                return cast(CentralWidget, item.widget())
        return None

    def get_login_widget(self) -> Optional[LoginWidget]:
        item = self.layout().itemAt(0)
        if item:
            if isinstance(item.widget(), LoginWidget):
                return cast(LoginWidget, item.widget())
        return None

    def clear_widgets(self) -> None:
        item = self.layout().takeAt(0)
        if item:
            item.widget().hide()
            item.widget().setParent(None)  # type: ignore[call-overload]
        QApplication.processEvents()
