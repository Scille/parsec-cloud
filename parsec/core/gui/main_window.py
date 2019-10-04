# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from functools import partial
from structlog import get_logger
from PyQt5.QtCore import QCoreApplication
from PyQt5.QtWidgets import QMainWindow

from parsec import __version__ as PARSEC_VERSION

from parsec.core.config import save_config
from parsec.core.gui.lang import translate as _
from parsec.core.gui.instance_widget import InstanceWidget
from parsec.core.gui import telemetry
from parsec.core.gui.custom_dialogs import QuestionDialog
from parsec.core.gui.starting_guide_dialog import StartingGuideDialog
from parsec.core.gui.ui.main_window import Ui_MainWindow


logger = get_logger()


class MainWindow(QMainWindow, Ui_MainWindow):
    def __init__(
        self, jobs_ctx, event_bus, config, daemon=None, minimize_on_close: bool = False, **kwargs
    ):
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
        self.tab_center.tabCloseRequested.connect(self.close_tab)
        self.daemon = daemon
        if daemon:
            self.daemon.new_instance_required.connect(self.add_instance)
        self.add_instance()

    def on_config_updated(self, event, **kwargs):
        self.config = self.config.evolve(**kwargs)
        save_config(self.config)
        telemetry.init(self.config)

    def show_starting_guide(self):
        s = StartingGuideDialog(parent=self)
        s.exec_()

    def showMaximized(self):
        super().showMaximized()
        QCoreApplication.processEvents()
        if self.config.gui_first_launch:
            self.show_starting_guide()
            r = QuestionDialog.ask(
                self, _("ASK_ERROR_REPORTING_TITLE"), _("ASK_ERROR_REPORTING_CONTENT")
            )
            self.event_bus.send("gui.config.changed", gui_first_launch=False, telemetry_enabled=r)
        telemetry.init(self.config)

    def show_top(self):
        self.show()
        self.raise_()

    def set_tab_title(self, tab, title):
        idx = self.tab_center.indexOf(tab)
        if idx == -1:
            return
        self.tab_center.setTabText(idx, title)

    def on_tab_state_changed(self, tab, state):
        if state == "login":
            self.set_tab_title(tab, _("TAB_TITLE_LOG_IN"))
        elif state == "connected":
            device = tab.current_device
            self.set_tab_title(
                tab, f"{device.organization_id}:{device.user_id}@{device.device_name}"

    def add_instance(self, cmd=None, url=None):
        tab = InstanceWidget(self.jobs_ctx, self.event_bus, self.config)
        self.tab_center.addTab(tab, "")
        tab.state_changed.connect(self.on_tab_state_changed)
        self.tab_center.setCurrentIndex(self.tab_center.count() - 1)
        if self.tab_center.count() > 1:
            self.tab_center.setTabsClosable(True)
        if not cmd or cmd == "new-window":
            tab.show_login_widget("log-in")
            self.set_tab_title(tab, _("TAB_TITLE_LOG_IN"))
        elif cmd == "bootstrap":
            tab.show_login_widget("bootstrap-org", url=url)
            self.set_tab_title(tab, _("TAB_TITLE_BOOTSTRAP"))
        elif cmd == "claim-user":
            tab.show_login_widget("claim-user", url=url)
            self.set_tab_title(tab, _("TAB_TITLE_CLAIM_USER"))
        elif cmd == "claim-device":
            tab.show_login_widget("claim-device", url=url)
            self.set_tab_title(tab, _("TAB_TITLE_CLAIM_DEVICE"))
        self.show_top()

    def close_app(self, force=False):
        self.need_close = True
        self.force_close = force
        self.daemon.clean_up()
        self.close()

    def close_all_tabs(self):
        for idx in range(self.tab_center.count()):
            self.close_tab(idx, force=True)

    def close_tab(self, index, force=False):
        tab = self.tab_center.widget(index)
        if not force:
            if tab and tab.logged_in:
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

        else:
            if self.config.gui_confirmation_before_close and not self.force_close:
                result = QuestionDialog.ask(self, _("ASK_QUIT_TITLE"), _("ASK_QUIT_CONTENT"))
                if not result:
                    event.ignore()
                    return
            msg = _("TRAY_PARSEC_RUNNING")

            # The Qt thread should never hit the core directly.
            # Synchronous calls can run directly in the job system
            # as they won't block the Qt loop for long
            self.jobs_ctx.run_sync(
                partial(self.event_bus.send, "gui.systray.notif", title="Parsec", msg=msg)
            )
            self.close_all_tabs()
            event.accept()
