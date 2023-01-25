# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations
import inspect

import sys
from distutils.version import LooseVersion
from typing import Awaitable, Callable, cast

import trio
from PyQt5.QtCore import QCoreApplication, QSize, Qt, pyqtSignal
from PyQt5.QtGui import QCloseEvent, QColor, QIcon, QKeySequence, QResizeEvent
from PyQt5.QtWidgets import QMainWindow, QMenu, QMenuBar, QShortcut, QWidget
from structlog import get_logger

from parsec import __version__ as PARSEC_VERSION
from parsec._parsec import (
    CoreEvent,
    DeviceFileType,
    InvitationType,
    get_available_device,
    list_available_devices,
)
from parsec.core import win_registry
from parsec.core.config import CoreConfig, save_config
from parsec.core.gui import desktop, telemetry, validators
from parsec.core.gui.about_widget import AboutWidget
from parsec.core.gui.central_widget import (
    GoToFileLinkBadOrganizationIDError,
    GoToFileLinkBadWorkspaceIDError,
    GoToFileLinkPathDecryptionError,
)
from parsec.core.gui.changelog_widget import ChangelogWidget
from parsec.core.gui.claim_device_widget import ClaimDeviceWidget
from parsec.core.gui.claim_user_widget import ClaimUserWidget
from parsec.core.gui.create_org_widget import CreateOrgWidget
from parsec.core.gui.custom_dialogs import (
    GreyedDialog,
    ask_question,
    get_text_input,
    show_error,
    show_info,
)
from parsec.core.gui.custom_widgets import Button, ensure_string_size
from parsec.core.gui.device_recovery_export_widget import DeviceRecoveryExportWidget
from parsec.core.gui.device_recovery_import_widget import DeviceRecoveryImportWidget
from parsec.core.gui.enrollment_query_widget import EnrollmentQueryWidget
from parsec.core.gui.instance_widget import InstanceWidget
from parsec.core.gui.lang import translate as _
from parsec.core.gui.license_widget import LicenseWidget
from parsec.core.gui.parsec_application import ParsecApp
from parsec.core.gui.settings_widget import SettingsWidget
from parsec.core.gui.snackbar_widget import SnackbarManager
from parsec.core.gui.trio_jobs import QtToTrioJob, QtToTrioJobScheduler
from parsec.core.gui.ui.main_window import Ui_MainWindow
from parsec.core.local_device import get_key_file
from parsec.core.pki import is_pki_enrollment_available
from parsec.core.types import (
    BackendActionAddr,
    BackendInvitationAddr,
    BackendOrganizationBootstrapAddr,
    BackendOrganizationFileLinkAddr,
    LocalDevice,
)
from parsec.core.types.backend_address import BackendPkiEnrollmentAddr
from parsec.event_bus import EventBus, EventCallback

logger = get_logger()


class MainWindow(QMainWindow, Ui_MainWindow):
    foreground_needed = pyqtSignal()
    new_instance_needed = pyqtSignal(object)
    systray_notification = pyqtSignal(str, str, int)

    TAB_NOTIFICATION_COLOR = QColor(46, 146, 208)
    TAB_NOT_SELECTED_COLOR = QColor(123, 132, 163)
    TAB_SELECTED_COLOR = QColor(12, 65, 159)

    def __init__(
        self,
        jobs_ctx: QtToTrioJobScheduler,
        quit_callback: Callable[[], None],
        event_bus: EventBus,
        config: CoreConfig,
        minimize_on_close: bool = False,
        parent: QWidget | None = None,
    ):
        super().__init__(parent=parent)
        self.setupUi(self)

        # TODO: is that really ok? The type checker does not agree
        self.setMenuBar(None)  # type: ignore[arg-type]
        self.jobs_ctx = jobs_ctx
        self.quit_callback = quit_callback
        self.event_bus = event_bus
        self.config = config
        self.minimize_on_close = minimize_on_close
        # Explain only once that the app stays in background
        self.minimize_on_close_notif_already_send = False
        self.force_close = False
        self.need_close = False
        self.event_bus.connect(
            CoreEvent.GUI_CONFIG_CHANGED, cast(EventCallback, self.on_config_updated)
        )
        self.setWindowTitle(_("TEXT_PARSEC_WINDOW_TITLE_version").format(version=PARSEC_VERSION))
        self.foreground_needed.connect(self._on_foreground_needed)
        self.new_instance_needed.connect(self._on_new_instance_needed)
        self.tab_center.tabCloseRequested.connect(self.close_tab)

        self.menu_button = Button()
        self.menu_button.setCursor(Qt.CursorShape.PointingHandCursor)
        self.menu_button.setIcon(QIcon(":/icons/images/material/menu.svg"))
        self.menu_button.setIconSize(QSize(24, 24))
        self.menu_button.setText(_("ACTION_MAIN_MENU_SHOW"))
        self.menu_button.setObjectName("MenuButton")
        self.menu_button.setProperty("color", QColor(0x00, 0x92, 0xFF))
        self.menu_button.setProperty("hover_color", QColor(0x00, 0x70, 0xDD))
        self.menu_button.setStyleSheet(
            "#MenuButton {background: none; border: none; color: #0092FF;}"
            "#MenuButton:hover {color: #0070DD;}"
        )
        self.menu_button.apply_style()
        self.menu_button.clicked.connect(self._show_menu)
        self.tab_center.setCornerWidget(self.menu_button, Qt.Corner.TopRightCorner)

        self.add_tab_button = Button()
        self.add_tab_button.setCursor(Qt.CursorShape.PointingHandCursor)
        self.add_tab_button.setIcon(QIcon(":/icons/images/material/add.svg"))
        self.add_tab_button.setIconSize(QSize(24, 24))
        self.add_tab_button.setProperty("color", QColor(0x00, 0x92, 0xFF))
        self.add_tab_button.setProperty("hover_color", QColor(0x00, 0x70, 0xDD))
        self.add_tab_button.setStyleSheet("background: none; border: none;")
        self.add_tab_button.apply_style()
        self.add_tab_button.clicked.connect(self._on_add_instance_clicked)
        self.tab_center.setCornerWidget(self.add_tab_button, Qt.Corner.TopLeftCorner)

        self.tab_center.currentChanged.connect(self.on_current_tab_changed)
        self.snackbar_manager = SnackbarManager(self)
        self._define_shortcuts()
        self.ensurePolished()

        if sys.platform == "darwin":
            # Native menu bar on MacOS
            self._create_mac_menu_bar()

    def _define_shortcuts(self) -> None:
        self.shortcut_close = QShortcut(QKeySequence(QKeySequence.StandardKey.Close), self)
        self.shortcut_close.activated.connect(self._shortcut_proxy(self.close_current_tab))
        self.shortcut_new_tab = QShortcut(QKeySequence(QKeySequence.StandardKey.AddTab), self)
        self.shortcut_new_tab.activated.connect(self._shortcut_proxy(self._on_add_instance_clicked))
        self.shortcut_settings = QShortcut(QKeySequence(_("Ctrl+K")), self)
        self.shortcut_settings.activated.connect(self._shortcut_proxy(self._show_settings))
        self.shortcut_recovery = QShortcut(QKeySequence(_("Ctrl+I")), self)
        self.shortcut_recovery.activated.connect(self._shortcut_proxy(self._on_manage_keys))
        self.shortcut_menu = QShortcut(QKeySequence(_("Alt+E")), self)
        self.shortcut_menu.activated.connect(self._shortcut_proxy(self._show_menu))
        self.shortcut_help = QShortcut(QKeySequence(QKeySequence.StandardKey.HelpContents), self)
        self.shortcut_help.activated.connect(self._shortcut_proxy(self._on_show_doc_clicked))
        self.shortcut_quit = QShortcut(QKeySequence(QKeySequence.StandardKey.Quit), self)
        self.shortcut_quit.activated.connect(self._shortcut_proxy(self.close_app))
        self.shortcut_create_org = QShortcut(QKeySequence(QKeySequence.StandardKey.New), self)
        self.shortcut_create_org.activated.connect(
            self._shortcut_proxy(self._on_create_org_clicked)
        )
        self.shortcut_join_org = QShortcut(QKeySequence(QKeySequence.StandardKey.Open), self)
        self.shortcut_join_org.activated.connect(self._shortcut_proxy(self._on_join_org_clicked))
        shortcut = QShortcut(QKeySequence(QKeySequence.StandardKey.NextChild), self)
        shortcut.activated.connect(self._shortcut_proxy(self._cycle_tabs(1)))
        shortcut = QShortcut(QKeySequence(QKeySequence.StandardKey.PreviousChild), self)
        shortcut.activated.connect(self._shortcut_proxy(self._cycle_tabs(-1)))

    def _shortcut_proxy(self, funct: Callable[[], None] | Callable[[], Awaitable[None]]) -> Callable[[], None] | Callable[[], Awaitable[None]]:
        async def _async_inner_proxy(self) -> None:
            if ParsecApp.has_active_modal():
                return
            f = funct.__get__(self)
            await f()

        def _inner_proxy() -> None:
            if ParsecApp.has_active_modal():
                return
            funct()

        return _inner_proxy if not inspect.iscoroutinefunction(funct) else _async_inner_proxy.__get__(self)

    def _cycle_tabs(self, offset: int) -> Callable[[], None]:
        def _inner_cycle_tabs() -> None:
            idx = self.tab_center.currentIndex()
            idx += offset
            if idx >= self.tab_center.count():
                idx = 0
            if idx < 0:
                idx = self.tab_center.count() - 1
            self.switch_to_tab(idx)

        return _inner_cycle_tabs

    def _toggle_add_tab_button(self) -> None:
        if self._get_login_tab_index() == -1:
            self.add_tab_button.setDisabled(False)
        else:
            self.add_tab_button.setDisabled(True)

    def resizeEvent(self, event: QResizeEvent) -> None:
        super().resizeEvent(event)
        for win in self.children():
            if isinstance(win, GreyedDialog):
                win.resize(event.size())
                win.move(0, 0)

    def _create_mac_menu_bar(self) -> None:
        menuBar = QMenuBar()

        fileMenu = QMenu(_("TEXT_MENU_FILE"), self)
        menuBar.addMenu(fileMenu)

        # 'settings' and 'about' are key words processed by Qt to make standard
        # MacOS submenus associated with standard key bindings. 'quit' links
        # cmd+Q to the close confirmation widget, and leaves the red X to its
        # standard behaviour depending on the `minimize_on_close` option.

        action = fileMenu.addAction("about")
        action.triggered.connect(self._show_about)
        action = fileMenu.addAction("settings")
        action.triggered.connect(self._show_settings)
        action = fileMenu.addAction("quit")
        action.triggered.connect(self.close_app)

        action = fileMenu.addAction(_("ACTION_MAIN_MENU_CREATE_ORGANIZATION"))
        action.triggered.connect(self._on_create_org_clicked)
        action.setShortcut(self.shortcut_create_org.key())
        action = fileMenu.addAction(_("ACTION_MAIN_MENU_JOIN_ORGANIZATION"))
        action.triggered.connect(self._on_join_org_clicked)
        action.setShortcut(self.shortcut_join_org.key())

        deviceMenu = QMenu(_("TEXT_MENU_DEVICE"), self)
        menuBar.addMenu(deviceMenu)

        action = deviceMenu.addAction(_("ACTION_MAIN_MENU_MANAGE_KEYS"))
        action.triggered.connect(self._on_manage_keys)
        action.setShortcut(self.shortcut_recovery.key())

        helpMenu = QMenu(_("TEXT_MENU_HELP"), self)
        menuBar.addMenu(helpMenu)

        action = helpMenu.addAction(_("ACTION_MAIN_MENU_OPEN_DOCUMENTATION"))
        action.triggered.connect(self._on_show_doc_clicked)
        action = helpMenu.addAction(_("ACTION_MAIN_MENU_CHANGELOG"))
        action.triggered.connect(self._show_changelog)

        helpMenu.addSeparator()

        action = helpMenu.addAction(_("ACTION_MAIN_MENU_LICENSE"))
        action.triggered.connect(self._show_license)

        helpMenu.addSeparator()

        action = helpMenu.addAction(_("ACTION_MAIN_MENU_FEEDBACK_SEND"))
        action.triggered.connect(self._on_send_feedback_clicked)

        self.setMenuBar(menuBar)

    def _show_menu(self) -> None:
        menu = QMenu(self)
        menu.setObjectName("MainMenu")
        action = None

        idx = self._get_login_tab_index()
        action = menu.addAction(_("ACTION_MAIN_MENU_ADD_INSTANCE"))
        action.triggered.connect(self._on_add_instance_clicked)
        action.setShortcut(self.shortcut_new_tab.key())
        action.setShortcutVisibleInContextMenu(True)

        if idx != -1:
            action.setDisabled(True)

        action = menu.addAction(_("ACTION_MAIN_MENU_CREATE_ORGANIZATION"))
        action.triggered.connect(self._on_create_org_clicked)
        action.setShortcut(self.shortcut_create_org.key())
        action.setShortcutVisibleInContextMenu(True)

        action = menu.addAction(_("ACTION_MAIN_MENU_JOIN_ORGANIZATION"))
        action.triggered.connect(self._on_join_org_clicked)
        action.setShortcut(self.shortcut_join_org.key())
        action.setShortcutVisibleInContextMenu(True)

        action = menu.addAction(_("ACTION_MAIN_MENU_MANAGE_KEYS"))
        action.triggered.connect(self._on_manage_keys)
        action.setShortcut(self.shortcut_recovery.key())
        action.setShortcutVisibleInContextMenu(True)

        menu.addSeparator()

        action = menu.addAction(_("ACTION_MAIN_MENU_SETTINGS"))
        action.triggered.connect(self._show_settings)
        action.setShortcut(self.shortcut_settings.key())
        action.setShortcutVisibleInContextMenu(True)

        action = menu.addAction(_("ACTION_MAIN_MENU_OPEN_DOCUMENTATION"))
        action.triggered.connect(self._on_show_doc_clicked)
        action.setShortcut(self.shortcut_help.key())
        action.setShortcutVisibleInContextMenu(True)

        action = menu.addAction(_("ACTION_MAIN_MENU_ABOUT"))
        action.triggered.connect(self._show_about)
        action = menu.addAction(_("ACTION_MAIN_MENU_CHANGELOG"))
        action.triggered.connect(self._show_changelog)
        action = menu.addAction(_("ACTION_MAIN_MENU_LICENSE"))
        action.triggered.connect(self._show_license)
        action = menu.addAction(_("ACTION_MAIN_MENU_FEEDBACK_SEND"))
        action.triggered.connect(self._on_send_feedback_clicked)
        menu.addSeparator()
        action = menu.addAction(_("ACTION_MAIN_MENU_QUIT_PARSEC"))
        action.triggered.connect(self.close_app)
        action.setShortcut(self.shortcut_quit.key())
        action.setShortcutVisibleInContextMenu(True)

        pos = self.menu_button.pos()
        pos.setY(pos.y() + self.menu_button.size().height())
        pos = self.mapToGlobal(pos)
        menu.exec_(pos)
        menu.setParent(None)  # type: ignore[call-overload]

    def _show_about(self) -> None:
        w = AboutWidget()
        d = GreyedDialog(w, title="", parent=self, width=1000)
        d.exec_()

    def _show_license(self) -> None:
        w = LicenseWidget()
        d = GreyedDialog(w, title=_("TEXT_LICENSE_TITLE"), parent=self, width=1000)
        d.exec_()

    def _show_changelog(self) -> None:
        w = ChangelogWidget()
        d = GreyedDialog(w, title=_("TEXT_CHANGELOG_TITLE"), parent=self, width=1000)
        d.exec_()

    def _show_settings(self) -> None:
        w = SettingsWidget(self.config, self.jobs_ctx, self.event_bus)
        d = GreyedDialog(w, title=_("TEXT_SETTINGS_TITLE"), parent=self, width=1000)
        d.exec_()

    async def _on_manage_keys(self) -> None:
        devices = await list_available_devices(self.config.config_dir)
        options = [_("ACTION_CANCEL"), _("ACTION_RECOVER_DEVICE")]
        if len(devices):
            options.append(_("ACTION_CREATE_RECOVERY_DEVICE"))
        result = ask_question(
            self, _("TEXT_DEVICE_RECOVERY_TITLE"), _("TEXT_DEVICE_RECOVERY_QUESTION"), options
        )
        if result == _("ACTION_RECOVER_DEVICE"):
            DeviceRecoveryImportWidget.show_modal(
                self.config, self.jobs_ctx, parent=self, on_finished=self.reload_login_devices
            )
        elif result == _("ACTION_CREATE_RECOVERY_DEVICE"):
            DeviceRecoveryExportWidget.show_modal(self.config, self.jobs_ctx, devices, parent=self)

    def _on_show_doc_clicked(self) -> None:
        desktop.open_doc_link()

    def _on_send_feedback_clicked(self) -> None:
        desktop.open_feedback_link()

    async def _on_add_instance_clicked(self) -> None:
        await self.add_instance()

    def _bind_async_callback(
        self, callback: Callable[[], Awaitable[None]]
    ) -> Callable[[], Awaitable[None]]:
        """Async callbacks need to be bound to the MainWindow instance
        in order to be able to be scheduled in the job context.
        """

        async def wrapper(instance: "MainWindow") -> None:
            return await callback()

        return wrapper.__get__(self)

    def _on_create_org_clicked(self, addr: BackendOrganizationBootstrapAddr | None = None) -> None:
        widget: CreateOrgWidget

        @self._bind_async_callback
        async def _on_finished() -> None:
            from parsec._parsec import get_available_device

            nonlocal widget
            # It's safe to access the widget status here since this does not perform a Qt call.
            # But the underlying C++ widget might already be deleted so we should make sure not
            # not do anything Qt related with this widget.
            if widget.status is None:
                return
            await self.reload_login_devices()
            device, auth_method, password = widget.status
            await self.try_login(device, auth_method, password)
            answer = ask_question(
                self,
                _("TEXT_BOOTSTRAP_ORG_SUCCESS_TITLE"),
                _("TEXT_BOOTSTRAP_ORG_SUCCESS_organization").format(
                    organization=device.organization_id.str
                ),
                [_("ACTION_CREATE_RECOVERY_DEVICE"), _("ACTION_NO")],
                oriented_question=True,
            )
            if answer == _("ACTION_CREATE_RECOVERY_DEVICE"):
                DeviceRecoveryExportWidget.show_modal(
                    self.config,
                    self.jobs_ctx,
                    [await get_available_device(self.config.config_dir, device.slug)],
                    parent=self,
                )

        widget = CreateOrgWidget.show_modal(
            self.jobs_ctx, self.config, self, on_finished=_on_finished, start_addr=addr
        )

    def _on_join_org_clicked(self) -> None:
        default_url = ""
        try:
            # Get the clipboard text, try and convert it to parsec addr and use it as default value
            default_url = BackendActionAddr.from_url(desktop.get_clipboard()).to_url()
        except ValueError:
            pass
        url = get_text_input(
            parent=self,
            title=_("TEXT_JOIN_ORG_URL_TITLE"),
            message=_("TEXT_JOIN_ORG_URL_INSTRUCTIONS"),
            placeholder=_("TEXT_JOIN_ORG_URL_PLACEHOLDER"),
            default_text=default_url,
            validator=validators.BackendActionAddrValidator(),
        )
        if url is None:
            return
        elif url == "":
            show_error(self, _("TEXT_JOIN_ORG_INVALID_URL"))
            return

        action_addr = None
        try:
            action_addr = BackendActionAddr.from_url(url, allow_http_redirection=True)
        except ValueError as exc:
            show_error(self, _("TEXT_INVALID_URL"), exception=exc)
            return

        if isinstance(action_addr, BackendOrganizationBootstrapAddr):
            self._on_create_org_clicked(action_addr)
        elif isinstance(action_addr, BackendInvitationAddr):
            if action_addr.invitation_type == InvitationType.USER:
                self._on_claim_user_clicked(action_addr)
            elif action_addr.invitation_type == InvitationType.DEVICE:
                self._on_claim_device_clicked(action_addr)
        elif isinstance(action_addr, BackendPkiEnrollmentAddr):
            if not is_pki_enrollment_available():
                show_error(self, _("TEXT_PKI_ENROLLMENT_NOT_AVAILABLE"))
                return
            self._on_claim_pki_clicked(action_addr)
        else:
            show_error(self, _("TEXT_INVALID_URL"))
            return

    def _on_recover_device_clicked(self) -> None:
        DeviceRecoveryImportWidget.show_modal(
            self.config, self.jobs_ctx, parent=self, on_finished=self.reload_login_devices
        )

    def _on_claim_pki_clicked(self, action_addr: BackendPkiEnrollmentAddr) -> None:
        widget: EnrollmentQueryWidget

        def _on_finished() -> None:
            nonlocal widget
            # It's safe to access the widget status here since this does not perform a Qt call.
            # But the underlying C++ widget might already be deleted so we should make sure not
            # not do anything Qt related with this widget.
            if not widget.status:
                return
            show_info(self, _("TEXT_ENROLLMENT_QUERY_SUCCEEDED"))
            self.reload_login_devices()

        widget = EnrollmentQueryWidget.show_modal(
            jobs_ctx=self.jobs_ctx,
            config=self.config,
            addr=action_addr,
            parent=self,
            on_finished=_on_finished,
        )

    def _on_claim_user_clicked(self, action_addr: BackendInvitationAddr) -> None:
        widget: ClaimUserWidget

        @self._bind_async_callback
        async def _on_finished() -> None:
            nonlocal widget
            # It's safe to access the widget status here since this does not perform a Qt call.
            # But the underlying C++ widget might already be deleted so we should make sure not
            # not do anything Qt related with this widget.
            if not widget.status:
                return
            device, auth_method, password = widget.status
            self.reload_login_devices()
            await self.try_login(device, auth_method, password)
            answer = ask_question(
                self,
                _("TEXT_CLAIM_USER_SUCCESSFUL_TITLE"),
                _("TEXT_CLAIM_USER_SUCCESSFUL"),
                [_("ACTION_CREATE_RECOVERY_DEVICE"), _("ACTION_NO")],
                oriented_question=True,
            )
            if answer == _("ACTION_CREATE_RECOVERY_DEVICE"):
                DeviceRecoveryExportWidget.show_modal(
                    self.config,
                    self.jobs_ctx,
                    [await get_available_device(self.config.config_dir, device.slug)],
                    self,
                )

        widget = ClaimUserWidget.show_modal(
            jobs_ctx=self.jobs_ctx,
            config=self.config,
            addr=action_addr,
            parent=self,
            on_finished=_on_finished,
        )

    def _on_claim_device_clicked(self, action_addr: BackendInvitationAddr) -> None:
        widget: ClaimDeviceWidget

        @self._bind_async_callback
        async def _on_finished() -> None:
            nonlocal widget
            # It's safe to access the widget status here since this does not perform a Qt call.
            # But the underlying C++ widget might already be deleted so we should make sure not
            # not do anything Qt related with this widget.
            if not widget.status:
                return
            device, auth_method, password = widget.status
            await self.reload_login_devices()
            await self.try_login(device, auth_method, password)

        widget = ClaimDeviceWidget.show_modal(
            jobs_ctx=self.jobs_ctx,
            config=self.config,
            addr=action_addr,
            parent=self,
            on_finished=_on_finished,
        )

    async def try_login(
        self, device: LocalDevice, auth_method: DeviceFileType, password: str
    ) -> None:
        idx = self._get_login_tab_index()
        if idx == -1:
            tab = self.add_new_tab()
        else:
            tab = self.tab_center.widget(idx)
        kf = await get_key_file(self.config.config_dir, device.slug)
        if auth_method == DeviceFileType.PASSWORD:
            await tab.login_with_password(kf, password)
        elif auth_method == DeviceFileType.SMARTCARD:
            await tab.login_with_smartcard(kf)

    async def reload_login_devices(self) -> None:
        idx = self._get_login_tab_index()
        if idx == -1:
            return
        w = self.tab_center.widget(idx)
        if not w:
            return
        await w.show_login_widget()

    def on_current_tab_changed(self, index: int) -> None:
        for i in range(self.tab_center.tabBar().count()):
            if i != index:
                if self.tab_center.tabBar().tabTextColor(i) != MainWindow.TAB_NOTIFICATION_COLOR:
                    self.tab_center.tabBar().setTabTextColor(i, MainWindow.TAB_NOT_SELECTED_COLOR)
            else:
                self.tab_center.tabBar().setTabTextColor(i, MainWindow.TAB_SELECTED_COLOR)

    def _on_foreground_needed(self) -> None:
        self.show_top()

    async def _on_new_instance_needed(self, start_arg: str | None) -> None:
        await self.add_instance(start_arg)
        self.show_top()

    def on_config_updated(self, event: CoreEvent, **kwargs: object) -> None:
        self.config = self.config.evolve(**kwargs)
        save_config(self.config)
        telemetry.init(self.config)

    def show_window(self, skip_dialogs: bool = False) -> None:
        try:
            if self.config.gui_geometry is not None:
                self.restoreGeometry(self.config.gui_geometry)
            else:
                self.showMaximized()
        except TypeError:
            self.showMaximized()

        QCoreApplication.processEvents()

        # Used with the --diagnose option
        if skip_dialogs:
            return

        # At the very first launch
        if self.config.gui_first_launch:
            r = ask_question(
                self,
                _("TEXT_ENABLE_TELEMETRY_TITLE"),
                _("TEXT_ENABLE_TELEMETRY_INSTRUCTIONS"),
                [_("ACTION_ENABLE_TELEMETRY_ACCEPT"), _("ACTION_ENABLE_TELEMETRY_REFUSE")],
                oriented_question=True,
            )

            # Acknowledge the changes
            self.event_bus.send(
                CoreEvent.GUI_CONFIG_CHANGED,
                gui_first_launch=False,
                gui_last_version=PARSEC_VERSION,
                telemetry_enabled=r == _("ACTION_ENABLE_TELEMETRY_ACCEPT"),
            )

        # For each parsec update
        if self.config.gui_last_version and self.config.gui_last_version != PARSEC_VERSION:

            # Update from parsec `<1.14` to `>=1.14`
            if LooseVersion(self.config.gui_last_version) < "1.14":

                # Revert the acrobat reader workaround
                if (
                    sys.platform == "win32"
                    and win_registry.is_acrobat_reader_dc_present()
                    and not win_registry.get_acrobat_app_container_enabled()
                ):
                    win_registry.del_acrobat_app_container_enabled()

            # Acknowledge the changes
            self.event_bus.send(CoreEvent.GUI_CONFIG_CHANGED, gui_last_version=PARSEC_VERSION)

        telemetry.init(self.config)

    def show_top(self) -> None:
        self.activateWindow()
        state: Qt.WindowStates = (
            self.windowState() & ~Qt.WindowStates(Qt.WindowState.WindowMinimized)
        ) | Qt.WindowStates(Qt.WindowState.WindowActive)
        self.setWindowState(state)
        self.raise_()
        self.show()

    async def on_tab_state_changed(self, tab: InstanceWidget, state: str) -> None:
        idx = self.tab_center.indexOf(tab)
        if idx == -1:
            if state == "logout":
                await self.reload_login_devices()
            return
        if state == "login":
            if self._get_login_tab_index() != -1:
                self.tab_center.removeTab(idx)
            else:
                self.tab_center.setTabToolTip(idx, _("TEXT_TAB_TITLE_LOG_IN_SCREEN"))
                self.tab_center.setTabText(idx, _("TEXT_TAB_TITLE_LOG_IN_SCREEN"))
        elif state == "logout":
            self.tab_center.removeTab(idx)
            idx = self._get_login_tab_index()
            if idx == -1:
                await self.add_instance()
            else:
                tab_widget = self.tab_center.widget(idx)
                log_widget = None if not tab_widget else tab_widget.get_login_widget()
                if log_widget:
                    await log_widget.reload_devices()
        elif state == "connected":
            device = tab.current_device
            assert device is not None
            tab_name = f"{device.organization_id.str} - {device.short_user_display} - {device.device_display}"
            self.tab_center.setTabToolTip(idx, tab_name)
            self.tab_center.setTabText(
                idx, ensure_string_size(tab_name, 150, self.tab_center.tabBar().font())
            )
        if self.tab_center.count() == 1:
            self.tab_center.setTabsClosable(False)
        self._toggle_add_tab_button()

    def on_tab_notification(self, tab: InstanceWidget, event: CoreEvent) -> None:
        idx = self.tab_center.indexOf(tab)
        if idx == -1 or idx == self.tab_center.currentIndex():
            return
        if event == CoreEvent.SHARING_UPDATED:
            self.tab_center.tabBar().setTabTextColor(idx, MainWindow.TAB_NOTIFICATION_COLOR)

    def _get_login_tab_index(self) -> int:
        for idx in range(self.tab_center.count()):
            if self.tab_center.tabText(idx) == _("TEXT_TAB_TITLE_LOG_IN_SCREEN"):
                return idx
        return -1

    def add_new_tab(self) -> InstanceWidget:
        tab = InstanceWidget(self.jobs_ctx, self.event_bus, self.config, self.systray_notification)
        tab.join_organization_clicked.connect(self._on_join_org_clicked)
        tab.create_organization_clicked.connect(self._on_create_org_clicked)
        tab.recover_device_clicked.connect(self._on_recover_device_clicked)
        idx = self.tab_center.addTab(tab, "")
        tab.state_changed.connect(self.on_tab_state_changed)
        self.tab_center.setCurrentIndex(idx)
        if self.tab_center.count() > 1:
            self.tab_center.setTabsClosable(True)
        else:
            self.tab_center.setTabsClosable(False)
        return tab

    def switch_to_tab(self, idx: int) -> None:
        if not ParsecApp.has_active_modal():
            self.tab_center.setCurrentIndex(idx)

    async def switch_to_login_tab(
        self, file_link_addr: BackendOrganizationFileLinkAddr | None = None
    ) -> None:
        # Retrieve the login tab
        idx = self._get_login_tab_index()
        if idx != -1:
            self.switch_to_tab(idx)
        else:
            # No login tab, create one
            tab = self.add_new_tab()
            await tab.show_login_widget()
            await self.on_tab_state_changed(tab, "login")
            idx = self.tab_center.count() - 1
            self.switch_to_tab(idx)

        if not file_link_addr:
            # We're done here
            return

        # Find the device corresponding to the organization in the link
        available_devices = await list_available_devices(self.config.config_dir)
        for available_device in available_devices:
            if available_device.organization_id == file_link_addr.organization_id:
                break

        else:
            # Cannot reach this organization with our available devices
            show_error(
                self,
                _("TEXT_FILE_LINK_NOT_IN_ORG_organization").format(
                    organization=file_link_addr.organization_id.str
                ),
            )
            return

        # Pre-select the corresponding device
        login_w = self.tab_center.widget(idx).get_login_widget()
        login_w._on_account_clicked(available_device)

        # Set the path
        instance_widget = self.tab_center.widget(idx)
        instance_widget.set_workspace_path(file_link_addr)

        # Prompt the user for the need to log in first
        SnackbarManager.inform(
            _("TEXT_FILE_LINK_PLEASE_LOG_IN_organization").format(
                organization=file_link_addr.organization_id.str
            )
        )

    async def go_to_file_link(self, addr: BackendOrganizationFileLinkAddr) -> None:
        # Try to use the file link on the already logged in cores
        for idx in range(self.tab_center.count()):
            if self.tab_center.tabText(idx) == _("TEXT_TAB_TITLE_LOG_IN_SCREEN"):
                continue

            w = self.tab_center.widget(idx)
            if (
                not w
                or not w.core
                or w.core.device.organization_addr.organization_id != addr.organization_id
            ):
                continue

            central_widget = w.get_central_widget()
            if not central_widget:
                continue

            try:
                central_widget.go_to_file_link(addr)

            except GoToFileLinkBadOrganizationIDError:
                continue
            except GoToFileLinkBadWorkspaceIDError:
                # Switch tab so user understand where the error comes from
                self.switch_to_tab(idx)
                show_error(
                    self,
                    _("TEXT_FILE_LINK_WORKSPACE_NOT_FOUND_organization").format(
                        organization=addr.organization_id.str
                    ),
                )
                return
            except GoToFileLinkPathDecryptionError:
                # Switch tab so user understand where the error comes from
                self.switch_to_tab(idx)
                show_error(self, _("TEXT_INVALID_URL"))
                return
            else:
                self.switch_to_tab(idx)
                return

        # The file link is from an organization we'r not currently logged in
        # or we don't have any device related to
        await self.switch_to_login_tab(addr)

    async def show_create_org_widget(self, action_addr: BackendOrganizationBootstrapAddr) -> None:
        await self.switch_to_login_tab()
        self._on_create_org_clicked(action_addr)

    async def show_claim_user_widget(self, action_addr: BackendInvitationAddr) -> None:
        await self.switch_to_login_tab()
        self._on_claim_user_clicked(action_addr)

    async def show_claim_device_widget(self, action_addr: BackendInvitationAddr) -> None:
        await self.switch_to_login_tab()
        self._on_claim_device_clicked(action_addr)

    async def show_claim_pki_widget(self, action_addr: BackendPkiEnrollmentAddr) -> None:
        await self.switch_to_login_tab()
        self._on_claim_pki_clicked(action_addr)

    async def add_instance(self, start_arg: str | None = None) -> None:
        action_addr = None
        if start_arg:
            try:
                action_addr = BackendActionAddr.from_url(start_arg, allow_http_redirection=True)
            except ValueError as exc:
                show_error(self, _("TEXT_INVALID_URL"), exception=exc)

        self.show_top()
        if not action_addr:
            await self.switch_to_login_tab()
        elif isinstance(action_addr, BackendOrganizationFileLinkAddr):
            await self.go_to_file_link(action_addr)
        elif isinstance(action_addr, BackendOrganizationBootstrapAddr):
            await self.show_create_org_widget(action_addr)
        elif (
            isinstance(action_addr, BackendInvitationAddr)
            and action_addr.invitation_type == InvitationType.USER
        ):
            await self.show_claim_user_widget(action_addr)
        elif (
            isinstance(action_addr, BackendInvitationAddr)
            and action_addr.invitation_type == InvitationType.DEVICE
        ):
            await self.show_claim_device_widget(action_addr)
        elif isinstance(action_addr, BackendPkiEnrollmentAddr):
            await self.show_claim_pki_widget(action_addr)
        else:
            show_error(self, _("TEXT_INVALID_URL"))

    async def close_current_tab(self, force: bool = False) -> None:
        if self.tab_center.count() == 1:
            self.close_app()
        else:
            idx = self.tab_center.currentIndex()
            await self.close_tab(idx, force=force)

    def close_app(self, force: bool = False) -> None:
        self.show_top()
        self.need_close = True
        self.force_close = force
        self.close()

    async def close_all_tabs(self) -> None:
        for idx in range(self.tab_center.count()):
            await self.close_tab(idx, force=True)

    async def close_tab(self, index: int, force: bool = False) -> None:
        tab = self.tab_center.widget(index)
        if not force:
            r: str | None = _("ACTION_TAB_CLOSE_CONFIRM")
            if tab and tab.is_logged_in:
                r = ask_question(
                    self,
                    _("TEXT_TAB_CLOSE_TITLE"),
                    _("TEXT_TAB_CLOSE_INSTRUCTIONS_device").format(
                        device=f"{tab.core.device.short_user_display} - {tab.core.device.device_display}"
                    ),
                    [_("ACTION_TAB_CLOSE_CONFIRM"), _("ACTION_CANCEL")],
                )
            if r != _("ACTION_TAB_CLOSE_CONFIRM"):
                return
        self.tab_center.removeTab(index)
        if not tab:
            return
        tab.logout()
        await self.reload_login_devices()
        if self.tab_center.count() == 1:
            self.tab_center.setTabsClosable(False)
        self._toggle_add_tab_button()

    def closeEvent(self, event: QCloseEvent) -> None:
        if self.minimize_on_close and not self.need_close:
            self.hide()
            event.ignore()

            # This notification is disabled on Mac since minimizing apps on
            # close is the standard behaviour on this OS.
            if not self.minimize_on_close_notif_already_send and sys.platform != "darwin":
                self.minimize_on_close_notif_already_send = True
                self.systray_notification.emit(
                    "Parsec", _("TEXT_TRAY_PARSEC_STILL_RUNNING_MESSAGE"), 2000
                )
        else:
            if self.config.gui_confirmation_before_close and not self.force_close:
                result = ask_question(
                    self if self.isVisible() else None,
                    _("TEXT_PARSEC_QUIT_TITLE"),
                    _("TEXT_PARSEC_QUIT_INSTRUCTIONS"),
                    [_("ACTION_PARSEC_QUIT_CONFIRM"), _("ACTION_CANCEL")],
                )
                if result != _("ACTION_PARSEC_QUIT_CONFIRM"):
                    event.ignore()
                    self.force_close = False
                    self.need_close = False
                    return

            state = self.saveGeometry()
            self.event_bus.send(CoreEvent.GUI_CONFIG_CHANGED, gui_geometry=state)
            self.close_all_tabs()
            self.quit_callback()
            event.ignore()
