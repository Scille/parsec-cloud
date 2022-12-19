# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import importlib.resources
import os
import string
import sys
from contextlib import contextmanager
from importlib.abc import Traversable
from types import ModuleType
from typing import Any, Iterator

from structlog import get_logger

from parsec.core import resources
from parsec.core.types import LocalDevice

logger = get_logger()

ACROBAT_READER_DC_PRIVILEGED = "Software\\Adobe\\Acrobat Reader\\DC\\Privileged"
ENABLE_APP_CONTAINER = "bEnableProtectedModeAppContainer"

PROCESS_ID = "ProcessID"
DRIVE_ICON_NAME = "parsec.ico"
EXPLORER_DRIVES = "Software\\Classes\\Applications\\Explorer.exe\\Drives"
EXPLORER_DRIVES_DEFAULT_ICON_TEMPLATE = EXPLORER_DRIVES + "\\{}\\DefaultIcon"


# Winreg helper


# Psutil is a dependency only on Windows, winreg is part of stdlib
# but only available on Windows. Hence must rely on dynamic import
# so that the current module can be imported from any OS.
#
# On top of that, we use good ol' `import foo` (nothing beats that !) instead
# of the fancy `importlib.import_module`. This is to make sure PyInstaller
# won't mess application packaging by missing this import during
# tree-shaking (see issue #1690).


def get_psutil() -> ModuleType:
    import psutil

    return psutil


def get_winreg() -> ModuleType:
    import winreg

    return winreg


def try_winreg() -> ModuleType | None:
    try:
        return get_winreg()
    except ImportError:
        logger.warning("OS is Windows but cannot import winreg")
        return None


def winreg_read_user_dword(key: str, name: str) -> object:
    winreg = get_winreg()
    hkcu = winreg.HKEY_CURRENT_USER
    with winreg.OpenKey(hkcu, key, access=winreg.KEY_READ) as key_obj:
        value, value_type = winreg.QueryValueEx(key_obj, name)
    if value_type != winreg.REG_DWORD:
        raise ValueError(value_type)
    return value


def winreg_write_user_dword(key: str, name: str, value: Any) -> None:
    winreg = get_winreg()
    hkcu = winreg.HKEY_CURRENT_USER
    with winreg.OpenKey(hkcu, key, access=winreg.KEY_SET_VALUE) as key_obj:
        winreg.SetValueEx(key_obj, name, None, winreg.REG_DWORD, value)


def winreg_delete_user_dword(key: str, name: str) -> None:
    winreg = get_winreg()
    hkcu = winreg.HKEY_CURRENT_USER
    with winreg.OpenKey(hkcu, key, access=winreg.KEY_SET_VALUE) as key_obj:
        winreg.DeleteValue(key_obj, name)


def winreg_has_user_key(key: str) -> bool:
    winreg = get_winreg()
    hkcu = winreg.HKEY_CURRENT_USER
    try:
        with winreg.OpenKey(hkcu, key, access=winreg.KEY_READ):
            return True
    except OSError:
        return False


# Acrobat container app workaround


def is_acrobat_reader_dc_present() -> bool:
    if sys.platform != "win32" or not try_winreg():
        return False

    return winreg_has_user_key(ACROBAT_READER_DC_PRIVILEGED)


def get_acrobat_app_container_enabled() -> bool:
    if not is_acrobat_reader_dc_present():
        return False
    try:
        value = winreg_read_user_dword(ACROBAT_READER_DC_PRIVILEGED, ENABLE_APP_CONTAINER)
    except OSError:
        # If the value doesn't exist, Acrobat considers it true
        return True

    return bool(value)


def set_acrobat_app_container_enabled(value: object) -> None:
    if not is_acrobat_reader_dc_present():
        return
    winreg_write_user_dword(ACROBAT_READER_DC_PRIVILEGED, ENABLE_APP_CONTAINER, value)


def del_acrobat_app_container_enabled() -> None:
    if not is_acrobat_reader_dc_present():
        return
    winreg_delete_user_dword(ACROBAT_READER_DC_PRIVILEGED, ENABLE_APP_CONTAINER)


# Drive icon management


def get_parsec_drive_icon(letter: str) -> tuple[str | None, object | None]:
    winreg = get_winreg()
    hkcu = winreg.HKEY_CURRENT_USER
    assert len(letter) == 1 and letter.upper() in string.ascii_uppercase
    key = EXPLORER_DRIVES_DEFAULT_ICON_TEMPLATE.format(letter.upper())

    # Get the drive icon path
    try:
        icon_path = winreg.QueryValue(hkcu, key)
    except OSError:
        return None, None

    # Get the process ID of the process that set the icon path
    try:
        pid = winreg_read_user_dword(key, PROCESS_ID)
    except (ValueError, OSError):
        pid = None

    # Return both value
    return icon_path, pid


def set_parsec_drive_icon(letter: str, drive_icon_path: Traversable) -> None:
    winreg = get_winreg()
    hkcu = winreg.HKEY_CURRENT_USER
    assert len(letter) == 1 and letter.upper() in string.ascii_uppercase
    key = EXPLORER_DRIVES_DEFAULT_ICON_TEMPLATE.format(letter.upper())

    # Write both the drive icon path and the current process id
    winreg.SetValue(hkcu, key, winreg.REG_SZ, str(drive_icon_path))
    winreg_write_user_dword(key, PROCESS_ID, os.getpid())


def del_parsec_drive_icon(letter: str) -> None:
    winreg = get_winreg()
    hkcu = winreg.HKEY_CURRENT_USER
    assert len(letter) == 1 and letter.upper() in string.ascii_uppercase
    key = EXPLORER_DRIVES_DEFAULT_ICON_TEMPLATE.format(letter.upper())

    # Delete the key if it exists
    try:
        winreg.DeleteKey(hkcu, key)
    except OSError:
        pass


def _get_drive_icon_path(device: LocalDevice) -> Traversable:
    # This function is just here so that other applications can
    # change the icon by monkeypatching it. `device` argument
    # can be used to add conditions, e.g. return different icons
    # for different devices.
    return importlib.resources.files(resources).joinpath(DRIVE_ICON_NAME)


@contextmanager
def parsec_drive_icon_context(letter: str, device: LocalDevice) -> Iterator[None]:
    # Winreg is not available for some reasons
    if sys.platform != "win32" or not try_winreg():
        yield
        return

    # Safe context for removing the key after usage
    drive_icon_path = _get_drive_icon_path(device)
    set_parsec_drive_icon(letter, drive_icon_path)
    try:
        yield
    finally:
        del_parsec_drive_icon(letter)


def cleanup_parsec_drive_icons() -> None:
    # Winreg is not available for some reasons
    if sys.platform != "win32" or not try_winreg():
        return

    # Loop over the 26 drives
    for letter in string.ascii_uppercase:

        # Perform some cleanup if necessary
        _, pid = get_parsec_drive_icon(letter)
        if pid and not get_psutil().pid_exists(pid):
            del_parsec_drive_icon(letter)
