# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import platform
from typing import Optional
from structlog import get_logger

from PyQt5.QtCore import QCoreApplication, pyqtSignal, Qt
from PyQt5.QtGui import QColor
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
from parsec.core.gui.license_widget import LicenseWidget
from parsec.core.gui.about_widget import AboutWidget
from parsec.core.gui.settings_widget import SettingsWidget
from parsec.core.gui.custom_dialogs import QuestionDialog, show_error, MiscDialog, show_warning
from parsec.core.gui.custom_widgets import MenuButton
from parsec.core.gui.starting_guide_dialog import StartingGuideDialog
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

        self.jobs_ctx = jobs_ctx
        self.event_bus = event_bus
        self.config = config
        self.minimize_on_close = minimize_on_close
        self.force_close = False
        self.need_close = False
        self.event_bus.connect("gui.config.changed", self.on_config_updated)
        self.setWindowTitle(_("PARSEC_WINDOW_TITLE").format(PARSEC_VERSION))
        self.foreground_needed.connect(self._on_foreground_needed)
        self.new_instance_needed.connect(self._on_new_instance_needed)
        self.tab_center.tabCloseRequested.connect(self.close_tab)
        self.button_send_feedback = QPushButton(_("LABEL_FEEDBACK_LINK"))
        self.button_send_feedback.clicked.connect(self._on_send_feedback_clicked)
        self.button_send_feedback.setStyleSheet("border: 0;")
        self.tab_center.setCornerWidget(self.button_send_feedback, Qt.TopRightCorner)
        self.menu_button = MenuButton()
        self.menu_button.clicked.connect(self._show_menu)
        self.tab_center.setCornerWidget(self.menu_button, Qt.TopLeftCorner)
        self.tab_center.currentChanged.connect(self.on_current_tab_changed)

    def _show_menu(self):
        menu = QMenu(self)
        action = None
        idx = self._get_login_tab_index()
        if idx == -1:
            action = menu.addAction(_("BUTTON_ADD_INSTANCE"))
        else:
            action = menu.addAction(_("BUTTON_ADD_INSTANCE"))
            action.setDisabled(True)
        action.triggered.connect(self._on_add_instance_clicked)
        action = menu.addAction(_("MENU_SETTINGS"))
        action.triggered.connect(self._show_settings)
        menu.addSeparator()
        action = menu.addAction(_("BUTTON_DOCUMENTATION"))
        action.triggered.connect(self._on_show_doc_clicked)
        action = menu.addAction(_("BUTTON_ABOUT"))
        action.triggered.connect(self._show_about)
        action = menu.addAction(_("BUTTON_CHANGELOG"))
        action.triggered.connect(self._show_changelog)
        action = menu.addAction(_("BUTTON_LICENSE"))
        action.triggered.connect(self._show_license)
        action = menu.addAction(_("LABEL_FEEDBACK_LINK"))
        action.triggered.connect(self._on_send_feedback_clicked)
        menu.addSeparator()
        action = menu.addAction(_("BUTTON_QUIT_PARSEC"))
        action.triggered.connect(self.close_app)
        pos = self.menu_button.pos()
        pos.setY(pos.y() + self.menu_button.size().height())
        pos = self.mapToGlobal(pos)
        menu.exec_(pos)

    def _show_about(self):
        w = AboutWidget()
        d = MiscDialog(None, w, parent=self)
        d.exec_()

    def _show_license(self):
        w = LicenseWidget()
        d = MiscDialog(_("LICENSE_TITLE"), w, parent=self)
        d.exec_()

    def _show_changelog(self):
        w = ChangelogWidget()
        d = MiscDialog(_("CHANGELOG_TITLE"), w, parent=self)
        d.exec_()

    def _show_settings(self):
        w = SettingsWidget(self.config, self.jobs_ctx, self.event_bus)
        d = MiscDialog(_("SETTINGS_TITLE"), w, parent=self)
        d.exec_()

    def _on_show_doc_clicked(self):
        desktop.open_doc_link()

    def _on_send_feedback_clicked(self):
        desktop.open_feedback_link()

    def _on_add_instance_clicked(self):
        self.add_instance()

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

    def show_starting_guide(self):
        s = StartingGuideDialog(parent=self)
        x = (self.width() - s.width()) / 2
        y = (self.height() - s.height()) / 2
        s.move(x, y)
        s.exec_()

    def showMaximized(self):
        super().showMaximized()
        QCoreApplication.processEvents()
        if self.config.gui_first_launch or self.config.gui_last_version != PARSEC_VERSION:
            # self.show_starting_guide()
            r = QuestionDialog.ask(
                self, _("ASK_ERROR_REPORTING_TITLE"), _("ASK_ERROR_REPORTING_CONTENT")
            )
            self.event_bus.send(
                "gui.config.changed",
                gui_first_launch=False,
                gui_last_version=PARSEC_VERSION,
                telemetry_enabled=r,
            )
            if (
                platform.system() == "Windows"
                and win_registry.is_acrobat_reader_dc_present()
                and win_registry.get_acrobat_app_container_enabled()
            ):
                r = QuestionDialog.ask(
                    self,
                    _("ASK_DISABLE_ACROBAT_CONTAINER_TITLE"),
                    _("ASK_DISABLE_ACROBAT_CONTAINER_CONTENT"),
                )
                if r:
                    win_registry.set_acrobat_app_container_enabled(False)
        telemetry.init(self.config)

    def show_top(self):
        self.show()
        self.raise_()

    def on_tab_state_changed(self, tab, state):
        idx = self.tab_center.indexOf(tab)
        if idx == -1:
            return
        if state == "login":
            self.tab_center.setTabToolTip(idx, _("TAB_TITLE_LOG_IN"))
            self.tab_center.setTabText(idx, _("TAB_TITLE_LOG_IN"))
        elif state == "bootstrap":
            self.tab_center.setTabToolTip(idx, _("TAB_TITLE_BOOTSTRAP"))
            self.tab_center.setTabText(idx, _("TAB_TITLE_BOOTSTRAP"))
        elif state == "claim_user":
            self.tab_center.setTabToolTip(idx, _("TAB_TITLE_CLAIM_USER"))
            self.tab_center.setTabText(idx, _("TAB_TITLE_CLAIM_USER"))
        elif state == "claim_device":
            self.tab_center.setTabToolTip(idx, _("TAB_TITLE_CLAIM_DEVICE"))
            self.tab_center.setTabText(idx, _("TAB_TITLE_CLAIM_DEVICE"))
        elif state == "connected":
            device = tab.current_device
            tab_name = f"{device.organization_id}:{device.user_id}@{device.device_name}"
            self.tab_center.setTabToolTip(idx, tab_name)
            self.tab_center.setTabText(idx, tab_name)

    def on_tab_notification(self, widget, event):
        idx = self.tab_center.indexOf(widget)
        if idx == -1 or idx == self.tab_center.currentIndex():
            return
        if event in ["sharing.updated"]:
            self.tab_center.tabBar().setTabTextColor(idx, MainWindow.TAB_NOTIFICATION_COLOR)

    def _get_login_tab_index(self):
        for idx in range(self.tab_center.count()):
            if self.tab_center.tabText(idx) == _("TAB_TITLE_LOG_IN"):
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
            if self.tab_center.tabText(idx) == _("TAB_TITLE_LOG_IN"):
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
        show_warning(self, _("WARN_FILE_LINK_LOG_IN_{}").format(action_addr.organization_id))

        tab = self.add_new_tab()
        tab.show_login_widget()
        self.on_tab_state_changed(tab, "login")
        self.switch_to_tab(self._get_login_tab_index())

    def add_instance(self, start_arg: Optional[str] = None):
        action_addr = None
        if start_arg:
            try:
                action_addr = BackendActionAddr.from_url(start_arg)
            except ValueError as exc:
                show_error(self, _("ERR_BAD_URL"), exception=exc)

        if not action_addr:
            idx = self._get_login_tab_index()
            if idx != -1:
                # There's already a login tab, just put it in front
                self.switch_to_tab(idx)
                return
            else:
                tab = self.add_new_tab()
                tab.show_login_widget()
                self.on_tab_state_changed(tab, "login")
                self.switch_to_tab(self.tab_center.count() - 1)
            return

        if isinstance(action_addr, BackendOrganizationFileLinkAddr):
            self.go_to_file_link(action_addr)
            return

        action = None
        method = None
        if isinstance(action_addr, BackendOrganizationBootstrapAddr):
            action = "bootstrap"
            method = "show_bootstrap_widget"
        elif isinstance(action_addr, BackendOrganizationClaimUserAddr):
            action = "claim_user"
            method = "show_claim_user_widget"
        elif isinstance(action_addr, BackendOrganizationClaimDeviceAddr):
            action = "claim_device"
            method = "show_claim_device_widget"

        tab = self.add_new_tab()

        if action:
            tab.show_login_widget(show_meth=method, addr=action_addr)
            self.on_tab_state_changed(tab, action)
        else:
            tab.show_login_widget()
            self.on_tab_state_changed(tab, "login")
        self.switch_to_tab(self.tab_center.count() - 1)

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
                r = QuestionDialog.ask(
                    self, _("ASK_CLOSE_TAB_TITLE"), _("ASK_CLOSE_TAB_CONTENT_LOGGED_IN")
                )
            elif self.tab_center.tabText(index) != _("TAB_TITLE_LOG_IN"):
                r = QuestionDialog.ask(self, _("ASK_CLOSE_TAB_TITLE"), _("ASK_CLOSE_TAB_CONTENT"))
            if not r:
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
            self.systray_notification.emit("Parsec", _("TRAY_PARSEC_RUNNING"))
        else:
            if self.config.gui_confirmation_before_close and not self.force_close:
                result = QuestionDialog.ask(self, _("ASK_QUIT_TITLE"), _("ASK_QUIT_CONTENT"))
                if not result:
                    event.ignore()
                    return

            self.close_all_tabs()
            event.accept()
