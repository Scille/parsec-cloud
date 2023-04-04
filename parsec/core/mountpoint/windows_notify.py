# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations
import ctypes
from enum import IntEnum

class _SHCNEEvent(IntEnum):
    SHCNE_RENAMEITEM = 1 << 0
    SHCNE_CREATE = 1 << 1
    SHCNE_DELETE = 1 << 2
    SHCNE_MKDIR = 1 << 3
    SHCNE_RMDIR = 1 << 4
    SHCNE_DRIVEREMOVED = 1 << 7
    SHCNE_DRIVEADD = 1 << 8
    SHCNE_RENAMEFOLDER = 1 << 17


class _SHCNFNotifyFlags(IntEnum):
    SHCNF_IDLIST = 0
    SHCNF_PATH = 5


class FileChangeEvent(IntEnum):
    FileCreated = 1
    FileDeleted = 2
    FileRenamed = 3
    FolderCreated = 4
    FolderRenamed = 5
    FolderDeleted = 6


def _do_notify(event: _SHCNEEvent, flags: _SHCNFNotifyFlags, arg1: str, arg2: str | None = None):
    shell32 = ctypes.OleDLL('shell32')
    shell32.SHChangeNotify.restype = None
    shell32.SHChangeNotify(event, flags, arg1, arg2)


def notify_mountpoint_state(drive: str, mounted: bool):
    event = _SHCNEEvent.SHCNE_DRIVEADD if mounted else _SHCNEEvent.SHCNE_DRIVEREMOVED
    flags = _SHCNFNotifyFlags.SHCNF_IDLIST | _SHCNFNotifyFlags.SHCNF_PATH
    _do_notify(event, flags, drive)


def notify_file_changed(event: FileChangeEvent, path: str, old_path: str | None = None):
    EVENT_MAP = {
        FileChangeEvent.FileCreated: _SHCNEEvent.SHCNE_CREATE,
        FileChangeEvent.FileDeleted: _SHCNEEvent.SHCNE_DELETE,
        FileChangeEvent.FileRenamed: _SHCNEEvent.SHCNE_RENAMEITEM,
        FileChangeEvent.FolderCreated: _SHCNEEvent.SHCNE_MKDIR,
        FileChangeEvent.FolderRenamed: _SHCNEEvent.SHCNE_RENAMEFOLDER,
        FileChangeEvent.FolderDeleted: _SHCNEEvent.SHCNE_RMDIR,
    }

    flags = _SHCNFNotifyFlags.SHCNF_IDLIST | _SHCNFNotifyFlags.SHCNF_PATH
    _do_notify(EVENT_MAP[event], flags, path, old_path)
