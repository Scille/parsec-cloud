# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import platform
from structlog import get_logger

# Inspired by
#
# - https://github.com/svenkle/google-drive-add-to-explorer/blob/develop/Google%20Drive.reg
# - https://www.codeproject.com/Tips/1116313/Creating-a-Link-in-the-Left-Pane-of-the-File-Explo
#

logger = get_logger()

APP_GUID = "6C37F945-7EFC-480A-A444-A6D44A3D107F"
APP_KEY = f"{{{APP_GUID}}}"

CLSID_PATH = "Software\\Classes\\CLSID"
CLSID_64_PATH = "Software\\Classes\\Wow6432Node\\CLSID"
EXPLORER_PATH = "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Desktop\\NameSpace"
DK_ICON_PATH = (
    "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\HideDesktopIcons\\NewStartPanel"
)


# Keys and values that will have to be removed when uninstalling Parsec on Windows
# HKEY_CURRENT_USER\Software\Classes\CLSID\{6C37F945-7EFC-480A-A444-A6D44A3D107F}\
# HKEY_CURRENT_USER\Software\Wow6432Node\CLSID\{6C37F945-7EFC-480A-A444-A6D44A3D107F}\
# HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{6C37F945-7EFC-480A-A444-A6D44A3D107F}\
# HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Desktop\HideDesktopIcons\NewStartPanel\{6C37F945-7EFC-480A-A444-A6D44A3D107F}


def _remove_nav_link():
    if platform.system() != "Windows":
        return
    try:
        import winreg
    except ImportError:
        logger.warning("OS is Windows but cannot import winreg")
        return

    def delete_key(handle, key):
        try:
            winreg.DeleteKey(handle, key)
        except FileNotFoundError:
            pass

    def delete_value(handle, value):
        try:
            winreg.DeleteValue(handle, value)
        except FileNotFoundError:
            pass

    def _remove_keys(handle):
        app_handle = winreg.OpenKey(handle, APP_KEY, access=winreg.KEY_SET_VALUE)
        delete_value(app_handle, None)
        delete_value(app_handle, "System.IsPinnedToNamespaceTree")
        delete_value(app_handle, "SortOrderIndex")
        with winreg.OpenKey(app_handle, "DefaultIcon", access=winreg.KEY_SET_VALUE) as icon_handle:
            delete_value(icon_handle, None)
        delete_key(app_handle, "DefaultIcon")
        with winreg.OpenKey(
            app_handle, "InProcServer32", access=winreg.KEY_SET_VALUE
        ) as proc_handle:
            delete_value(proc_handle, None)
        delete_key(app_handle, "InProcServer32")
        with winreg.OpenKey(app_handle, "Instance", access=winreg.KEY_SET_VALUE) as inst_handle:
            with winreg.OpenKey(
                inst_handle, "InitPropertyBag", access=winreg.KEY_SET_VALUE
            ) as pb_handle:
                delete_value(pb_handle, "TargetFolderPath")
                delete_value(pb_handle, "Attributes")
            delete_key(inst_handle, "InitPropertyBag")
        delete_key(app_handle, "Instance")
        with winreg.OpenKey(app_handle, "ShellFolder", access=winreg.KEY_SET_VALUE) as shell_handle:
            delete_value(shell_handle, "Attributes")
            delete_value(shell_handle, "FolderValueFlags")
        delete_key(app_handle, "ShellFolder")
        winreg.CloseKey(app_handle)
        delete_key(handle, APP_KEY)

    handle = winreg.ConnectRegistry(None, winreg.HKEY_CURRENT_USER)
    with winreg.OpenKey(handle, CLSID_PATH, access=winreg.KEY_SET_VALUE) as clsid_handle:
        _remove_keys(clsid_handle)
    try:
        with winreg.OpenKey(handle, CLSID_64_PATH, access=winreg.KEY_SET_VALUE) as clsid_64_handle:
            _remove_keys(clsid_64_handle)
    except FileNotFoundError:
        pass

    with winreg.OpenKey(handle, EXPLORER_PATH, access=winreg.KEY_SET_VALUE) as explorer_handle:
        with winreg.OpenKey(explorer_handle, APP_KEY, access=winreg.KEY_SET_VALUE) as parsec_handle:
            delete_value(parsec_handle, None)
        delete_key(explorer_handle, APP_KEY)

    with winreg.OpenKey(handle, DK_ICON_PATH, access=winreg.KEY_SET_VALUE) as dkicon_handle:
        delete_value(dkicon_handle, APP_KEY)


def remove_nav_link():
    try:
        _remove_nav_link()
    except OSError as exc:
        logger.error("Error while removing navbar registry values: {}".format(str(exc)))


def _add_nav_link(mountpoint):
    if platform.system() != "Windows":
        return
    try:
        import winreg
    except ImportError:
        logger.warning("OS is Windows but cannot import winreg")
        return

    EXE_PATH = os.path.join(os.getcwd(), "..", "parsec.exe")
    MOUNTPOINT = mountpoint

    def _add_keys(handle, x64):
        app_handle = winreg.CreateKey(handle, APP_KEY)
        winreg.SetValueEx(app_handle, None, 0, winreg.REG_SZ, "Parsec")
        winreg.SetValueEx(app_handle, "System.IsPinnedToNamespaceTree", 0, winreg.REG_DWORD, 0x1)
        winreg.SetValueEx(app_handle, "SortOrderIndex", 0, winreg.REG_DWORD, 0x42)
        with winreg.CreateKey(app_handle, "DefaultIcon") as icon_handle:
            winreg.SetValueEx(icon_handle, None, 0, winreg.REG_SZ, f"{EXE_PATH},0")
        with winreg.CreateKey(app_handle, "InProcServer32") as proc_handle:
            if not x64:
                winreg.SetValueEx(
                    proc_handle,
                    None,
                    0,
                    winreg.REG_EXPAND_SZ,
                    "%SYSTEMROOT%\\system32\\shell32.dll",
                )
            else:
                winreg.SetValueEx(
                    proc_handle,
                    None,
                    0,
                    winreg.REG_EXPAND_SZ,
                    "%SYSTEMROOT%\\SysWow64\\shell32.dll",
                )
        with winreg.CreateKey(app_handle, "Instance") as inst_handle:
            winreg.SetValueEx(
                inst_handle, "CLSID", 0, winreg.REG_SZ, "{0E5AAE11-A475-4C5B-AB00-C66DE400274E}"
            )
            with winreg.CreateKey(inst_handle, "InitPropertyBag") as pb_handle:
                winreg.SetValueEx(pb_handle, "TargetFolderPath", 0, winreg.REG_SZ, f"{MOUNTPOINT}")
                winreg.SetValueEx(pb_handle, "Attributes", 0, winreg.REG_DWORD, 0x11)
        with winreg.CreateKey(app_handle, "ShellFolder") as shell_handle:
            winreg.SetValueEx(shell_handle, "Attributes", 0, winreg.REG_DWORD, 0xF080004D)
            winreg.SetValueEx(shell_handle, "FolderValueFlags", 0, winreg.REG_DWORD, 0x28)
        winreg.CloseKey(app_handle)

    handle = winreg.ConnectRegistry(None, winreg.HKEY_CURRENT_USER)
    with winreg.OpenKey(handle, CLSID_PATH) as clsid_handle:
        _add_keys(clsid_handle, False)
    try:
        with winreg.OpenKey(handle, CLSID_64_PATH) as clsid_64_handle:
            _add_keys(clsid_64_handle, True)
    except FileNotFoundError:
        pass

    with winreg.OpenKey(handle, EXPLORER_PATH) as explorer_handle:
        with winreg.CreateKey(explorer_handle, APP_KEY) as parsec_handle:
            winreg.SetValueEx(parsec_handle, None, 0, winreg.REG_SZ, "Parsec")

    with winreg.OpenKey(handle, DK_ICON_PATH, access=winreg.KEY_SET_VALUE) as dkicon_handle:
        winreg.SetValueEx(dkicon_handle, APP_KEY, 0, winreg.REG_DWORD, 0x1)


def add_nav_link(mountpoint):
    try:
        _add_nav_link(mountpoint)
    except OSError as exc:
        logger.error("Error while adding navbar registry values: {}".format(str(exc)))
