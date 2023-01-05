# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from pathlib import Path

import pytest

from parsec._parsec import CoreEvent, DateTime
from parsec.api.data import EntryName, WorkspaceEntry
from parsec.api.protocol import RealmRole
from parsec.core.fs import (
    FsPath,
    FSWorkspaceInMaintenance,
    FSWorkspaceNoReadAccess,
    FSWorkspaceNoWriteAccess,
)
from parsec.core.gui.lang import translate


@pytest.mark.gui
@pytest.mark.trio
async def test_mountpoint_notifs(aqtbot, logged_gui, snackbar_catcher):
    c_w = logged_gui.test_get_central_widget()

    def _snackbar_shown(sb):
        assert snackbar_catcher.snackbars == sb

    kwargs = {
        "exc": FSWorkspaceNoReadAccess(),
        "mountpoint": Path("/tmp/unused"),
        "path": FsPath("/unused"),
    }
    c_w.handle_event(CoreEvent.MOUNTPOINT_REMOTE_ERROR, **kwargs)
    await aqtbot.wait_until(
        lambda: _snackbar_shown(
            [
                (
                    "WARN",
                    translate("TEXT_NOTIF_WARN_WORKSPACE_READ_ACCESS_LOST_workspace").format(
                        workspace=str(kwargs["mountpoint"])
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
                        workspace=str(kwargs["mountpoint"])
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
                        workspace=str(kwargs["mountpoint"])
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
                        workspace=str(kwargs["mountpoint"]), error="exception"
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
                    ).format(
                        workspace=str(kwargs["mountpoint"]), operation="unused", error="exception"
                    ),
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
                    ).format(
                        workspace=str(kwargs["mountpoint"]), operation="unused", error="exception"
                    ),
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

    ne = WorkspaceEntry.new(EntryName("Workspace"), DateTime(2000, 1, 2))
    ne = ne.evolve(role=RealmRole.CONTRIBUTOR)
    pe = WorkspaceEntry.new(EntryName("Workspace"), DateTime(2000, 1, 1))
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
