# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
from pathlib import Path
from datetime import datetime

from parsec.core.core_events import CoreEvent
from parsec.api.protocol import RealmRole
from parsec.api.data import WorkspaceEntry, EntryName
from parsec.core.fs import (
    FsPath,
    FSWorkspaceNoReadAccess,
    FSWorkspaceNoWriteAccess,
    FSWorkspaceInMaintenance,
)
from parsec.core.gui.lang import translate


@pytest.mark.gui
@pytest.mark.trio
async def test_mountpoint_notifs(aqtbot, logged_gui, snackbar_catcher):
    c_w = logged_gui.test_get_central_widget()

    def _snackbar_shown(sb):
        print(snackbar_catcher.snackbars)
        assert snackbar_catcher.snackbars == sb

    kwargs = {
        "exc": FSWorkspaceNoReadAccess(),
        "mountpoint": Path("/tmp/unused"),
        "path": FsPath("/unused"),
    }
    wk_path = kwargs["path"].with_mountpoint(kwargs["mountpoint"])
    c_w.handle_event(CoreEvent.MOUNTPOINT_REMOTE_ERROR, **kwargs)
    await aqtbot.wait_until(
        lambda: _snackbar_shown(
            [
                (
                    "WARN",
                    translate("TEXT_NOTIF_WARN_WORKSPACE_READ_ACCESS_LOST_workspace").format(
                        workspace=wk_path
                    ),
                )
            ]
        )
    )

    snackbar_catcher.reset()
    kwargs["exc"] = FSWorkspaceNoWriteAccess()
    c_w.handle_event(CoreEvent.MOUNTPOINT_REMOTE_ERROR, **kwargs)
    await aqtbot.wait_until(
        lambda: _snackbar_shown(
            [
                (
                    "WARN",
                    translate("TEXT_NOTIF_WARN_WORKSPACE_WRITE_ACCESS_LOST_workspace").format(
                        workspace=wk_path
                    ),
                )
            ]
        )
    )

    snackbar_catcher.reset()
    kwargs["exc"] = FSWorkspaceInMaintenance()
    c_w.handle_event(CoreEvent.MOUNTPOINT_REMOTE_ERROR, **kwargs)
    await aqtbot.wait_until(
        lambda: _snackbar_shown(
            [
                (
                    "WARN",
                    translate("TEXT_NOTIF_WARN_WORKSPACE_IN_MAINTENANCE_workspace").format(
                        workspace=wk_path
                    ),
                )
            ]
        )
    )

    snackbar_catcher.reset()
    kwargs["exc"] = Exception()
    c_w.handle_event(CoreEvent.MOUNTPOINT_REMOTE_ERROR, **kwargs)
    await aqtbot.wait_until(
        lambda: _snackbar_shown(
            [
                (
                    "WARN",
                    translate("TEXT_NOTIF_WARN_MOUNTPOINT_REMOTE_ERROR_workspace-error").format(
                        workspace=wk_path, error="exception"
                    ),
                )
            ]
        )
    )

    snackbar_catcher.reset()
    kwargs["exc"] = Exception()
    kwargs["operation"] = "unused"
    c_w.handle_event(CoreEvent.MOUNTPOINT_UNHANDLED_ERROR, **kwargs)
    await aqtbot.wait_until(
        lambda: _snackbar_shown(
            [
                (
                    "WARN",
                    translate(
                        "TEXT_NOTIF_ERR_MOUNTPOINT_UNEXPECTED_ERROR_workspace_operation_error"
                    ).format(workspace=wk_path, operation="unused", error="exception"),
                )
            ]
        )
    )

    snackbar_catcher.reset()
    kwargs["exc"] = Exception()
    kwargs["operation"] = "unused"
    c_w.handle_event(CoreEvent.MOUNTPOINT_TRIO_DEADLOCK_ERROR, **kwargs)
    await aqtbot.wait_until(
        lambda: _snackbar_shown(
            [
                (
                    "WARN",
                    translate(
                        "TEXT_NOTIF_ERR_MOUNTPOINT_UNEXPECTED_ERROR_workspace_operation_error"
                    ).format(workspace=wk_path, operation="unused", error="exception"),
                )
            ]
        )
    )


@pytest.mark.gui
@pytest.mark.trio
async def test_sharing_notifs(aqtbot, logged_gui, snackbar_catcher, monkeypatch):
    c_w = logged_gui.test_get_central_widget()

    def _snackbar_shown(sb):
        assert snackbar_catcher.snackbars == sb

    ne = WorkspaceEntry.new(EntryName("Workspace"), datetime(2000, 1, 2))
    ne = ne.evolve(role=RealmRole.CONTRIBUTOR)
    pe = WorkspaceEntry.new(EntryName("Workspace"), datetime(2000, 1, 1))
    pe = pe.evolve(role=RealmRole.READER)

    c_w.handle_event(CoreEvent.SHARING_UPDATED, new_entry=ne, previous_entry=pe)
    await aqtbot.wait_until(
        lambda: _snackbar_shown(
            [
                (
                    "INFO",
                    translate("TEXT_NOTIF_INFO_WORKSPACE_ROLE_UPDATED_workspace").format(
                        workspace="Workspace"
                    ),
                )
            ]
        )
    )

    snackbar_catcher.reset()
    c_w.handle_event(CoreEvent.SHARING_UPDATED, new_entry=ne, previous_entry=None)
    await aqtbot.wait_until(
        lambda: _snackbar_shown(
            [
                (
                    "INFO",
                    translate("TEXT_NOTIF_INFO_WORKSPACE_SHARED_workspace").format(
                        workspace="Workspace"
                    ),
                )
            ]
        )
    )

    ne = ne.evolve(role=None)

    snackbar_catcher.reset()
    c_w.handle_event(CoreEvent.SHARING_UPDATED, new_entry=ne, previous_entry=pe)
    await aqtbot.wait_until(
        lambda: _snackbar_shown(
            [
                (
                    "INFO",
                    translate("TEXT_NOTIF_INFO_WORKSPACE_UNSHARED_workspace").format(
                        workspace="Workspace"
                    ),
                )
            ]
        )
    )
