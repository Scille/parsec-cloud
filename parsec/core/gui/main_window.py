# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import platform
from typing import Optional
from structlog import get_logger

from PyQt5.QtCore import QCoreApplication, pyqtSignal, Qt, QSize
from PyQt5.QtGui import QColor, QIcon
from PyQt5.QtWidgets import QMainWindow, QPushButton, QApplication, QMenu

from parsec import __version__ as PARSEC_VERSION

from parsec.core.config import save_config
from parsec.core.types import (
    BackendActionAddr,
    BackendOrganizationBootstrapAddr,
    BackendOrganizationClaimUserAddr,
    BackendOrganizationClaimDeviceAddr,
    BackendOrganizationFileLinkAddr,
)
from parsec.core.gui.lang import translate as _
from parsec.core.gui.instance_widget import InstanceWidget
from parsec.core.gui import telemetry
from parsec.core.gui import desktop
from parsec.core.gui import win_registry
from parsec.core.gui.changelog_widget import ChangelogWidget
from parsec.core.gui.bootstrap_organization_widget import BootstrapOrganizationWidget
from parsec.core.gui.claim_user_widget import ClaimUserWidget
from parsec.core.gui.claim_device_widget import ClaimDeviceWidget
from parsec.core.gui.license_widget import LicenseWidget
from parsec.core.gui.about_widget import AboutWidget
from parsec.core.gui.settings_widget import SettingsWidget
from parsec.core.gui.custom_dialogs import ask_question, show_error, GreyedDialog, get_text_input
from parsec.core.gui.custom_widgets import Button
from parsec.core.gui.ui.main_window import Ui_MainWindow


logger = get_logger()


class MainWindow(QMainWindow, Ui_MainWindow):
    foreground_needed = pyqtSignal()
    new_instance_needed = pyqtSignal(object)
    systray_notification = pyqtSignal(str, str)

    TAB_NOTIFICATION_COLOR = QColor(46, 146, 208)
    TAB_NOT_SELECTED_COLOR = QColor(123, 132, 163)
    TAB_SELECTED_COLOR = QColor(12, 65, 159)

    def __init__(self, jobs_ctx, event_bus, config, minimize_on_close: bool = False, **kwargs):
        super().__init__(**kwargs)
        self.setupUi(self)

        self.setMenuBar(None)
        self.jobs_ctx = jobs_ctx
        self.event_bus = event_bus
        self.config = config
        self.minimize_on_close = minimize_on_close
        self.force_close = False
        self.need_close = False
        self.event_bus.connect("gui.config.changed", self.on_config_updated)
        self.setWindowTitle(_("TEXT_PARSEC_WINDOW_TITLE_version").format(version=PARSEC_VERSION))
        self.foreground_needed.connect(self._on_foreground_needed)
        self.new_instance_needed.connect(self._on_new_instance_needed)
        self.tab_center.tabCloseRequested.connect(self.close_tab)
        self.button_send_feedback = QPushButton(_("ACTION_FEEDBACK_SEND"))
        self.button_send_feedback.clicked.connect(self._on_send_feedback_clicked)
        self.button_send_feedback.setStyleSheet("border: 0; border-radius: 0px;")
        self.tab_center.setCornerWidget(self.button_send_feedback, Qt.TopRightCorner)
        self.menu_button = Button()
        self.menu_button.setCursor(Qt.PointingHandCursor)
        self.menu_button.setIcon(QIcon(":/icons/images/material/menu.svg"))
        self.menu_button.setIconSize(QSize(24, 24))
        self.menu_button.setProperty("color", QColor(0x00, 0x92, 0xFF))
        self.menu_button.setProperty("hover_color", QColor(0x00, 0x70, 0xDD))
        self.menu_button.setStyleSheet("background-color: none; border: none;")
        self.menu_button.apply_style()
        self.menu_button.clicked.connect(self._show_menu)
        self.tab_center.setCornerWidget(self.menu_button, Qt.TopLeftCorner)
        self.tab_center.currentChanged.connect(self.on_current_tab_changed)
        self.ensurePolished()

    def resizeEvent(self, event):
        super().resizeEvent(event)
        for win in self.children():
            if win.objectName() == "GreyedDialog":
                win.resize(event.size())
                win.move(0, 0)

    def _show_menu(self):
        menu = QMenu(self)
        menu.setObjectName("MainMenu")
        action = None

        idx = self._get_login_tab_index()
        action = menu.addAction(_("ACTION_MAIN_MENU_ADD_INSTANCE"))
        action.triggered.connect(self._on_add_instance_clicked)

        if idx != -1:
            action.setDisabled(True)

        action = menu.addAction(_("ACTION_MAIN_MENU_BOOTSTRAP_ORGANIZATION"))
        action.triggered.connect(self._on_bootstrap_org_clicked)
        action = menu.addAction(_("ACTION_MAIN_MENU_CLAIM_USER"))
        action.triggered.connect(self._on_claim_user_clicked)
        action = menu.addAction(_("ACTION_MAIN_MENU_CLAIM_DEVICE"))
        action.triggered.connect(self._on_claim_device_clicked)
        menu.addSeparator()

        action = menu.addAction(_("ACTION_MAIN_MENU_SETTINGS"))
        action.triggered.connect(self._show_settings)
        action = menu.addAction(_("ACTION_MAIN_MENU_OPEN_DOCUMENTATION"))
        action.triggered.connect(self._on_show_doc_clicked)
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
        pos = self.menu_button.pos()
        pos.setY(pos.y() + self.menu_button.size().height())
        pos = self.mapToGlobal(pos)
        menu.exec_(pos)

    def _show_about(self):
        w = AboutWidget()
        d = GreyedDialog(w, title="", parent=self)
        d.exec_()

    def _show_license(self):
        w = LicenseWidget()
        d = GreyedDialog(w, title=_("TEXT_LICENSE_TITLE"), parent=self)
        d.exec_()

    def _show_changelog(self):
        w = ChangelogWidget()
        d = GreyedDialog(w, title=_("TEXT_CHANGELOG_TITLE"), parent=self)
        d.exec_()

    def _show_settings(self):
        w = SettingsWidget(self.config, self.jobs_ctx, self.event_bus)
        d = GreyedDialog(w, title=_("TEXT_SETTINGS_TITLE"), parent=self)
        d.exec_()

    def _on_show_doc_clicked(self):
        desktop.open_doc_link()

    def _on_send_feedback_clicked(self):
        desktop.open_feedback_link()

    def _on_add_instance_clicked(self):
        self.add_instance()

    def _on_bootstrap_org_clicked(self, action_addr=None):
        if not action_addr:
            url = get_text_input(
                parent=self,
                title=_("TEXT_BOOTSTRAP_ORG_URL_TITLE"),
                message=_("TEXT_BOOTSTRAP_ORG_URL_INSTRUCTIONS"),
                placeholder=_("TEXT_BOOTSTRAP_ORG_URL_PLACEHOLDER"),
            )
            if url is None:
                return
            elif url == "":
                show_error(self, _("TEXT_BOOTSTRAP_ORG_INVALID_URL"))
                return

            action_addr = None
            try:
                action_addr = BackendOrganizationBootstrapAddr.from_url(url)
            except ValueError as exc:
                show_error(self, _("TEXT_BOOTSTRAP_ORG_INVALID_URL"), exception=exc)
                return
        ret = BootstrapOrganizationWidget.exec_modal(
            jobs_ctx=self.jobs_ctx, config=self.config, addr=action_addr, parent=self
        )
        if ret:
            self.reload_login_devices()

    def _on_claim_user_clicked(self, action_addr=None):
        if not action_addr:
            url = get_text_input(
                parent=self,
                title=_("TEXT_CLAIM_USER_URL_TITLE"),
                message=_("TEXT_CLAIM_USER_URL_INSTRUCTIONS"),
                placeholder=_("TEXT_CLAIM_USER_URL_PLACEHOLDER"),
            )
            if url is None:
                return
            elif url == "":
                show_error(self, _("TEXT_CLAIM_USER_INVALID_URL"))
                return

            action_addr = None
            try:
                action_addr = BackendOrganizationClaimUserAddr.from_url(url)
            except ValueError as exc:
                show_error(self, _("TEXT_CLAIM_USER_INVALID_URL"), exception=exc)
                return
        ret = ClaimUserWidget.exec_modal(
            jobs_ctx=self.jobs_ctx, config=self.config, addr=action_addr, parent=self
        )
        if ret:
            self.reload_login_devices()

    def _on_claim_device_clicked(self, action_addr=None):
        if not action_addr:
            url = get_text_input(
                parent=self,
                title=_("TEXT_CLAIM_DEVICE_URL_TITLE"),
                message=_("TEXT_CLAIM_DEVICE_URL_INSTRUCTIONS"),
                placeholder=_("TEXT_CLAIM_DEVICE_URL_PLACEHOLDER"),
            )
            if url is None:
                return
            elif url == "":
                show_error(self, _("TEXT_CLAIM_DEVICE_INVALID_URL"))
                return

            action_addr = None
            try:
                action_addr = BackendOrganizationClaimDeviceAddr.from_url(url)
            except ValueError as exc:
                show_error(self, _("TEXT_CLAIM_DEVICE_INVALID_URL"), exception=exc)
                return
        ret = ClaimDeviceWidget.exec_modal(
            jobs_ctx=self.jobs_ctx, config=self.config, addr=action_addr, parent=self
        )
        if ret:
            self.reload_login_devices()

    def reload_login_devices(self):
        idx = self._get_login_tab_index()
        if idx == -1:
            return
        w = self.tab_center.widget(idx)
        if not w:
            return
        item = w.layout().itemAt(0)
        if not item:
            return
        login_w = item.widget()
        if not login_w:
            return
        login_w.reload_devices()

    def on_current_tab_changed(self, index):
        for i in range(self.tab_center.tabBar().count()):
            if i != index:
                if self.tab_center.tabBar().tabTextColor(i) != MainWindow.TAB_NOTIFICATION_COLOR:
                    self.tab_center.tabBar().setTabTextColor(i, MainWindow.TAB_NOT_SELECTED_COLOR)
            else:
                self.tab_center.tabBar().setTabTextColor(i, MainWindow.TAB_SELECTED_COLOR)

    def _on_foreground_needed(self):
        self.show_top()

    def _on_new_instance_needed(self, start_arg):
        self.add_instance(start_arg)
        self.show_top()

    def on_config_updated(self, event, **kwargs):
        self.config = self.config.evolve(**kwargs)
        save_config(self.config)
        telemetry.init(self.config)

    def showMaximized(self, skip_dialogs=False):
        super().showMaximized()
        QCoreApplication.processEvents()

        # Used with the --diagnose option
        if skip_dialogs:
            return

        # At the very first launch
        if self.config.gui_first_launch:
            r = ask_question(
                self,
                _("TEXT_ERROR_REPORTING_TITLE"),
                _("TEXT_ERROR_REPORTING_INSTRUCTIONS"),
                [_("ACTION_ERROR_REPORTING_ACCEPT"), _("ACTION_NO")],
            )

            # Acknowledge the changes
            self.event_bus.send(
                "gui.config.changed",
                gui_first_launch=False,
                gui_last_version=PARSEC_VERSION,
                telemetry_enabled=r == _("ACTION_ERROR_REPORTING_ACCEPT"),
            )

        # For each parsec update
        if self.config.gui_last_version != PARSEC_VERSION:

            # Ask for acrobat reader workaround
            if (
                platform.system() == "Windows"
                and win_registry.is_acrobat_reader_dc_present()
                and win_registry.get_acrobat_app_container_enabled()
            ):
                r = ask_question(
                    self,
                    _("TEXT_ACROBAT_CONTAINERS_DISABLE_TITLE"),
                    _("TEXT_ACROBAT_CONTAINERS_DISABLE_INSTRUCTIONS"),
                    [_("ACTION_ACROBAT_CONTAINERS_DISABLE_ACCEPT"), _("ACTION_NO")],
                )
                if r == _("ACTION_ACROBAT_CONTAINERS_DISABLE_ACCEPT"):
                    win_registry.set_acrobat_app_container_enabled(False)

            # Acknowledge the changes
            self.event_bus.send("gui.config.changed", gui_last_version=PARSEC_VERSION)

        telemetry.init(self.config)

    def show_top(self):
        self.activateWindow()
        self.setWindowState((self.windowState() & ~Qt.WindowMinimized) | Qt.WindowActive)
        self.raise_()
        self.show()

    def on_tab_state_changed(self, tab, state):
        idx = self.tab_center.indexOf(tab)
        if idx == -1:
            return
        if state == "login":
            if self._get_login_tab_index() != -1:
                self.tab_center.removeTab(idx)
            else:
                self.tab_center.setTabToolTip(idx, _("TEXT_TAB_TITLE_LOG_IN_SCREEN"))
                self.tab_center.setTabText(idx, _("TEXT_TAB_TITLE_LOG_IN_SCREEN"))
        elif state == "connected":
            device = tab.current_device
            tab_name = f"{device.organization_id}:{device.user_id}@{device.device_name}"
            self.tab_center.setTabToolTip(idx, tab_name)
            self.tab_center.setTabText(idx, tab_name)
        if self.tab_center.count() == 1:
            self.tab_center.setTabsClosable(False)

    def on_tab_notification(self, widget, event):
        idx = self.tab_center.indexOf(widget)
        if idx == -1 or idx == self.tab_center.currentIndex():
            return
        if event in ["sharing.updated"]:
            self.tab_center.tabBar().setTabTextColor(idx, MainWindow.TAB_NOTIFICATION_COLOR)

    def _get_login_tab_index(self):
        for idx in range(self.tab_center.count()):
            if self.tab_center.tabText(idx) == _("TEXT_TAB_TITLE_LOG_IN_SCREEN"):
                return idx
        return -1

    def add_new_tab(self):
        tab = InstanceWidget(self.jobs_ctx, self.event_bus, self.config)
        self.tab_center.addTab(tab, "")
        tab.state_changed.connect(self.on_tab_state_changed)
        self.tab_center.setCurrentIndex(self.tab_center.count() - 1)
        if self.tab_center.count() > 1:
            self.tab_center.setTabsClosable(True)
        else:
            self.tab_center.setTabsClosable(False)
        return tab

    def switch_to_tab(self, idx):
        if not QApplication.activeModalWidget():
            self.tab_center.setCurrentIndex(idx)

    def go_to_file_link(self, action_addr):
        for idx in range(self.tab_center.count()):
            if self.tab_center.tabText(idx) == _("TEXT_TAB_TITLE_LOG_IN_SCREEN"):
                continue
            w = self.tab_center.widget(idx)
            if (
                not w
                or not w.core
                or w.core.device.organization_addr.organization_id != action_addr.organization_id
            ):
                continue
            user_manifest = w.core.user_fs.get_user_manifest()
            for wk in user_manifest.workspaces:
                if not wk.role:
                    continue
                if wk.id != action_addr.workspace_id:
                    continue
                central_widget = w.get_central_widget()
                try:
                    central_widget.show_mount_widget()
                    central_widget.mount_widget.show_files_widget(
                        w.core.user_fs.get_workspace(wk.id), action_addr.path, selected=True
                    )
                    self.switch_to_tab(idx)
                except AttributeError:
                    logger.exception("Central widget is not available")
                return
        show_error(
            self,
            _("TEXT_FILE_LINK_NOT_FOUND_organization").format(
                organization=action_addr.organization_id
            ),
        )

    def add_instance(self, start_arg: Optional[str] = None):
        action_addr = None
        if start_arg:
            try:
                action_addr = BackendActionAddr.from_url(start_arg)
            except ValueError as exc:
                show_error(self, _("TEXT_INVALID_URL"), exception=exc)

        idx = self._get_login_tab_index()
        if idx != -1:
            self.switch_to_tab(idx)
        else:
            tab = self.add_new_tab()
            tab.show_login_widget()
            self.on_tab_state_changed(tab, "login")
            self.switch_to_tab(self.tab_center.count() - 1)
            idx = self.tab_center.count() - 1

        self.show_top()
        if action_addr and isinstance(action_addr, BackendOrganizationFileLinkAddr):
            self.go_to_file_link(action_addr)
        elif action_addr:
            if isinstance(action_addr, BackendOrganizationBootstrapAddr):
                self._on_bootstrap_org_clicked(action_addr)
            elif isinstance(action_addr, BackendOrganizationClaimUserAddr):
                self._on_claim_user_clicked(action_addr)
            elif isinstance(action_addr, BackendOrganizationClaimDeviceAddr):
                self._on_claim_device_clicked(action_addr)

    def close_app(self, force=False):
        self.need_close = True
        self.force_close = force
        self.close()

    def close_all_tabs(self):
        for idx in range(self.tab_center.count()):
            self.close_tab(idx, force=True)

    def close_tab(self, index, force=False):
        tab = self.tab_center.widget(index)
        if not force:
            r = True
            if tab and tab.is_logged_in:
                r = ask_question(
                    self,
                    _("TEXT_TAB_CLOSE_TITLE"),
                    _("TEXT_TAB_CLOSE_INSTRUCTIONS"),
                    [_("ACTION_TAB_CLOSE_CONFIRM"), _("ACTION_CANCEL")],
                )
            elif self.tab_center.tabText(index) != _("TEXT_TAB_TITLE_LOG_IN_SCREEN"):
                r = ask_question(
                    self,
                    _("TEXT_TAB_CLOSE_TITLE"),
                    _("TEXT_TAB_CLOSE_INSTRUCTIONS"),
                    [_("ACTION_TAB_CLOSE_CONFIRM"), _("ACTION_CANCEL")],
                )
            if r != _("ACTION_TAB_CLOSE_CONFIRM"):
                return
        self.tab_center.removeTab(index)
        if not tab:
            return
        tab.logout()
        if self.tab_center.count() == 1:
            self.tab_center.setTabsClosable(False)

    def closeEvent(self, event):
        if self.minimize_on_close and not self.need_close:
            self.hide()
            event.ignore()
            self.systray_notification.emit("Parsec", _("TEXT_TRAY_PARSEC_STILL_RUNNING_MESSAGE"))
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

            self.close_all_tabs()
            event.accept()
            QApplication.quit()
