# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import re
import platform
import trio
from urllib.request import urlopen, Request

from PyQt5.QtCore import Qt, pyqtSignal
from PyQt5.QtWidgets import QDialog

from parsec import __version__
from parsec.core.config import save_config
from parsec.core.gui import desktop
from parsec.core.gui.trio_thread import ThreadSafeQtSignal
from parsec.core.gui.ui.new_version_dialog import Ui_NewVersionDialog


RELEASE_URL = "https://github.com/Scille/parsec-build/releases/latest"


def _extract_version_tuple(raw):
    match = re.match(r"^.*([0-9]+)\.([0-9]+)\.([0-9]+)", raw)
    if match:
        return tuple(int(x) for x in match.groups())
    else:
        return None


async def _do_check_new_version(url):
    # urlopen automatically follows redirections
    resolved_url = await trio.run_sync_in_worker_thread(
        lambda: urlopen(Request(url, method="HEAD")).geturl()
    )

    lastest_version = _extract_version_tuple(resolved_url)
    current_version = _extract_version_tuple(__version__)
    if lastest_version and current_version and current_version < lastest_version:
        return lastest_version
    else:
        return None


async def _do_disable_check(event_bus):
    event_bus.send("gui.config.changed", gui_check_version_at_startup=False)


class CheckNewVersion(QDialog, Ui_NewVersionDialog):
    check_new_version_success = pyqtSignal()
    check_new_version_error = pyqtSignal()
    disable_check_success = pyqtSignal()
    disable_check_error = pyqtSignal()

    def __init__(self, jobs_ctx, event_bus, config, **kwargs):
        super().__init__(**kwargs)
        self.setupUi(self)

        if platform.system() != "Windows":
            return

        self.jobs_ctx = jobs_ctx
        self.event_bus = event_bus
        self.config = config

        self.check_new_version_success.connect(self.on_check_new_version_success)

        self.check_new_version_job = jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "check_new_version_success"),
            ThreadSafeQtSignal(self, "check_new_version_error"),
            _do_check_new_version,
            url=self.config.gui_check_version_url,
        )
        self.disable_check_job = None

        self.button_download.clicked.connect(self.download)
        self.button_ignore.clicked.connect(self.ignore)
        self.setWindowFlags(Qt.SplashScreen)

    def on_check_new_version_success(self):
        assert self.check_new_version_job.is_finished()
        assert self.check_new_version_job.status == "ok"
        new_version = self.check_new_version_job.ret
        if new_version:
            self.exec_()

    def download(self):
        desktop.open_url(self.config.gui_check_version_url)
        self.accept()

    def ignore(self):
        if self.check_box_no_reminder.isChecked():
            # TODO
            # self.disable_check_job = self.jobs_ctx.submit_job(
            #     self.disable_check_success,
            #     self.disable_check_error,
            #     _do_disable_check,
            # )
            self.config = self.config.evolve(gui_check_version_at_startup=False)
            save_config(self.config)
        self.reject()

    def closeEvent(self, event):
        self.check_new_version_job.cancel_and_join()
        if self.disable_check_job:
            self.disable_check_job.cancel_and_join()
        event.accept()
