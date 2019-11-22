# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import Optional
from structlog import get_logger
from PyQt5.QtCore import QCoreApplication, pyqtSignal
from PyQt5.QtWidgets import QMainWindow, QApplication

from parsec import __version__ as PARSEC_VERSION

from parsec.core.config import save_config
from parsec.core.types import (
    BackendActionAddr,
    BackendOrganizationBootstrapAddr,
    BackendOrganizationClaimUserAddr,
    BackendOrganizationClaimDeviceAddr,
)
from parsec.core.gui.lang import translate as _
from parsec.core.gui.instance_widget import InstanceWidget
from parsec.core.gui import telemetry
from parsec.core.gui.custom_dialogs import QuestionDialog, show_error
from parsec.core.gui.starting_guide_dialog import StartingGuideDialog
from parsec.core.gui.ui.main_window import Ui_MainWindow


logger = get_logger()


class MainWindow(QMainWindow, Ui_MainWindow):
    foreground_needed = pyqtSignal()
    new_instance_needed = pyqtSignal(object)
    systray_notification = pyqtSignal(str, str)

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
        if self.config.gui_first_launch:
            # self.show_starting_guide()
            r = QuestionDialog.ask(
                self, _("ASK_ERROR_REPORTING_TITLE"), _("ASK_ERROR_REPORTING_CONTENT")
            )
            self.event_bus.send("gui.config.changed", gui_first_launch=False, telemetry_enabled=r)
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
            if len(tab_name) > 15:
                tab_name = f"{tab_name[:12]}..."
            self.tab_center.setTabText(idx, tab_name)

    def add_instance(self, start_arg: Optional[str] = None):
        tab = InstanceWidget(self.jobs_ctx, self.event_bus, self.config)
        self.tab_center.addTab(tab, "")
        tab.state_changed.connect(self.on_tab_state_changed)

        if self.tab_center.count() > 1:
            self.tab_center.setTabsClosable(True)

        action_addr = None
        if start_arg:
            try:
                action_addr = BackendActionAddr.from_url(start_arg)
            except ValueError as exc:
                show_error(self, _("ERR_BAD_URL"), exception=exc)

        if isinstance(action_addr, BackendOrganizationBootstrapAddr):
            tab.show_login_widget(show_meth="show_bootstrap_widget", addr=action_addr)
            self.on_tab_state_changed(tab, "bootstrap")

        elif isinstance(action_addr, BackendOrganizationClaimUserAddr):
            tab.show_login_widget(show_meth="show_claim_user_widget", addr=action_addr)
            self.on_tab_state_changed(tab, "claim_user")

        elif isinstance(action_addr, BackendOrganizationClaimDeviceAddr):
            tab.show_login_widget(show_meth="show_claim_device_widget", addr=action_addr)
            self.on_tab_state_changed(tab, "claim_device")

        else:
            # Fallback to just create the default login windows
            tab.show_login_widget()
            self.on_tab_state_changed(tab, "login")

        if not QApplication.activeModalWidget():
            self.tab_center.setCurrentIndex(self.tab_center.count() - 1)

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
            if tab and tab.is_logged_in:
                r = QuestionDialog.ask(
                    self, _("ASK_CLOSE_TAB_TITLE"), _("ASK_CLOSE_TAB_CONTENT_LOGGED_IN")
                )
            else:
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
