# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import trio
from typing import Iterable
from pathlib import PurePath
import re

from PyQt5.QtCore import QUrl, QSysInfo, QLocale
from PyQt5.QtGui import QDesktopServices, QGuiApplication, QClipboard


async def open_files_job(paths: Iterable[PurePath]):
    status = True
    for path in paths:
        assert path.is_absolute()
        # Run QDesktopServices in a thread in order to avoid any accidental access
        # to the parsec file system as this would cause a serious deadlock.
        status &= await trio.to_thread.run_sync(
            lambda: QDesktopServices.openUrl(QUrl.fromLocalFile(str(path)))
        )
    return status, paths


def open_url(url):
    return QDesktopServices.openUrl(QUrl(url))


def open_doc_link():
    return open_url("https://parsec-cloud.readthedocs.io")


def open_feedback_link():
    return open_url("https://my.parsec.cloud/feedback")


def open_user_guide():
    return open_url("https://parsec.cloud")


def get_default_device():
    device = QSysInfo.machineHostName()
    if device.lower() == "localhost":
        device = QSysInfo.productType()
    return "".join([c for c in device if re.match(r"[\w\-]", c)])


def get_locale_language():
    return QLocale.system().name()[:2].lower()


def copy_to_clipboard(text):
    clipboard = QGuiApplication.clipboard()
    clipboard.setText(text, QClipboard.Clipboard)
    if clipboard.supportsSelection():
        clipboard.setText(text, QClipboard.Selection)


def get_clipboard():
    return QGuiApplication.clipboard().text()
