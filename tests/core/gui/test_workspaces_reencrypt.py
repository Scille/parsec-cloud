# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from functools import partial

import pytest
from PyQt5 import QtCore

from parsec._parsec import EntryID, EntryName, LocalDevice, UserID
from parsec.core.fs import (
    FSBackendOfflineError,
    FSError,
    FSWorkspaceNoAccess,
    FSWorkspaceNotFoundError,
    UserFS,
)
from parsec.core.gui.lang import translate
from parsec.core.gui.workspaces_widget import WorkspaceButton, WorkspacesWidget
from parsec.core.types import WorkspaceRole
from tests.common import customize_fixtures
from tests.core.gui.conftest import AsyncQtBot, GuiFactory


# Helpers


async def revoke_user_workspace_right(
    workspace: EntryID, owner_user_fs: UserFS, invited_user_fs: UserFS, invited_user_id: UserID
):
    await owner_user_fs.workspace_share(workspace, invited_user_id, None)
    await owner_user_fs.process_last_messages()
    await invited_user_fs.process_last_messages()
    await owner_user_fs.sync()


async def display_reencryption_button(
    aqtbot: AsyncQtBot, monkeypatch: pytest.MonkeyPatch, workspace_widget: WorkspacesWidget
):
    def _workspace_displayed():
        assert workspace_widget.layout_workspaces.count() == 1
        wk_button = workspace_widget.layout_workspaces.itemAt(0).widget()
        assert isinstance(wk_button, WorkspaceButton)
        assert wk_button.name == EntryName("w1")

    await aqtbot.wait_until(_workspace_displayed)
    wk_button = workspace_widget.layout_workspaces.itemAt(0).widget()

    def _reencrypt_button_displayed():
        assert wk_button.button_reencrypt.isVisible()

    await aqtbot.wait_until(_reencrypt_button_displayed)

    monkeypatch.setattr(
        "parsec.core.gui.workspaces_widget.ask_question",
        lambda *args, **kwargs: translate("ACTION_WORKSPACE_REENCRYPTION_CONFIRM"),
    )


# Fixtures


@pytest.fixture
async def shared_workspace(
    running_backend, alice2_user_fs: UserFS, bob_user_fs: UserFS, bob: LocalDevice
) -> EntryID:
    wid = await alice2_user_fs.workspace_create(EntryName("w1"))
    await alice2_user_fs.sync()
    await alice2_user_fs.workspace_share(wid, bob.user_id, WorkspaceRole.READER)
    await alice2_user_fs.process_last_messages()
    await bob_user_fs.process_last_messages()
    await alice2_user_fs.sync()
    return wid


@pytest.fixture
async def reencryption_needed_workspace(
    running_backend,
    shared_workspace: EntryID,
    alice2_user_fs: UserFS,
    bob_user_fs: UserFS,
    bob: LocalDevice,
) -> EntryID:
    await revoke_user_workspace_right(shared_workspace, alice2_user_fs, bob_user_fs, bob.user_id)
    return shared_workspace


# Tests


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(logged_gui_as_admin=True)
async def test_workspace_reencryption_display(
    aqtbot,
    running_backend,
    logged_gui,
    autoclose_dialog,
    shared_workspace,
    bob_user_fs,
    alice2_user_fs,
    bob,
):
    w_w = await logged_gui.test_switch_to_workspaces_widget()

    def _workspace_displayed(workspace_w):
        assert workspace_w.layout_workspaces.count() == 1
        wk_button = workspace_w.layout_workspaces.itemAt(0).widget()
        assert isinstance(wk_button, WorkspaceButton)
        assert wk_button.name == EntryName("w1")

    await aqtbot.wait_until(partial(_workspace_displayed, w_w))
    wk_button = w_w.layout_workspaces.itemAt(0).widget()

    def _reencrypt_button_not_displayed():
        assert wk_button.button_reencrypt.isHidden()
        assert not wk_button.button_reencrypt.isVisible()

    await aqtbot.wait_until(_reencrypt_button_not_displayed)

    await revoke_user_workspace_right(shared_workspace, alice2_user_fs, bob_user_fs, bob.user_id)

    w_w = await logged_gui.test_switch_to_workspaces_widget()

    await aqtbot.wait_until(partial(_workspace_displayed, w_w))
    wk_button = w_w.layout_workspaces.itemAt(0).widget()

    def _reencrypt_button_displayed():
        assert wk_button.button_reencrypt.isVisible()

    await aqtbot.wait_until(_reencrypt_button_displayed)


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(logged_gui_as_admin=True)
async def test_workspace_reencryption(
    aqtbot: AsyncQtBot,
    running_backend,
    logged_gui,
    autoclose_dialog,
    monkeypatch: pytest.MonkeyPatch,
    reencryption_needed_workspace: EntryID,
):

    w_w: WorkspacesWidget = await logged_gui.test_switch_to_workspaces_widget()

    await display_reencryption_button(aqtbot, monkeypatch, w_w)
    wk_button = w_w.layout_workspaces.itemAt(0).widget()

    async with aqtbot.wait_signals(
        [wk_button.button_reencrypt.clicked, wk_button.reencrypt_clicked]
    ):
        aqtbot.mouse_click(wk_button.button_reencrypt, QtCore.Qt.LeftButton)

    def _reencrypt_button_not_displayed():
        assert not wk_button.button_reencrypt.isVisible()

    await aqtbot.wait_until(_reencrypt_button_not_displayed)


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(logged_gui_as_admin=True)
async def test_workspace_reencryption_offline_backend(
    aqtbot,
    running_backend,
    logged_gui,
    autoclose_dialog,
    monkeypatch,
    reencryption_needed_workspace,
):

    w_w = await logged_gui.test_switch_to_workspaces_widget()

    await display_reencryption_button(aqtbot, monkeypatch, w_w)
    wk_button = w_w.layout_workspaces.itemAt(0).widget()
    with running_backend.offline():
        aqtbot.mouse_click(wk_button.button_reencrypt, QtCore.Qt.LeftButton)

        def _assert_error():
            assert len(autoclose_dialog.dialogs) == 1
            assert autoclose_dialog.dialogs == [
                ("Error", translate("TEXT_WORKSPACE_REENCRYPT_OFFLINE_ERROR"))
            ]
            assert wk_button.button_reencrypt.isVisible()

        await aqtbot.wait_until(_assert_error)


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(logged_gui_as_admin=True)
async def test_workspace_reencryption_fs_error(
    aqtbot,
    running_backend,
    logged_gui,
    autoclose_dialog,
    alice2_user_fs,
    monkeypatch,
    reencryption_needed_workspace,
):

    w_w = await logged_gui.test_switch_to_workspaces_widget()

    await display_reencryption_button(aqtbot, monkeypatch, w_w)
    wk_button = w_w.layout_workspaces.itemAt(0).widget()

    await alice2_user_fs.workspace_start_reencryption(wk_button.workspace_id)
    aqtbot.mouse_click(wk_button.button_reencrypt, QtCore.Qt.LeftButton)

    def _assert_error():
        assert len(autoclose_dialog.dialogs) == 1
        assert autoclose_dialog.dialogs == [
            ("Error", translate("TEXT_WORKSPACE_REENCRYPT_FS_ERROR"))
        ]
        assert wk_button.button_reencrypt.isVisible()

    await aqtbot.wait_until(_assert_error)


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(logged_gui_as_admin=True)
async def test_workspace_reencryption_access_error(
    aqtbot,
    running_backend,
    logged_gui,
    autoclose_dialog,
    alice2_user_fs,
    alice,
    user_fs_factory,
    adam,
    monkeypatch,
    reencryption_needed_workspace,
):

    w_w = await logged_gui.test_switch_to_workspaces_widget()

    await display_reencryption_button(aqtbot, monkeypatch, w_w)
    wk_button = w_w.layout_workspaces.itemAt(0).widget()

    # Make adam a workspace owner
    await alice2_user_fs.workspace_share(
        reencryption_needed_workspace, adam.user_id, WorkspaceRole.OWNER
    )

    # This weasel removes us from our own workspace!
    async with user_fs_factory(adam) as adam_user_fs:
        await adam_user_fs.process_last_messages()
        await adam_user_fs.workspace_share(
            reencryption_needed_workspace, alice.user_id, WorkspaceRole.READER
        )

    def _role_changed(expected_role):
        user_id = wk_button.workspace_fs.device.user_id
        assert user_id in wk_button.users_roles
        role, _ = wk_button.users_roles[user_id]
        assert role == expected_role

    # Now we're nothing more than a reader :(
    await aqtbot.wait_until(lambda: _role_changed(WorkspaceRole.READER))

    def _demoted_message_shown():
        assert autoclose_dialog.dialogs[0][1] == translate(
            "TEXT_FILE_SHARING_DEMOTED_TO_READER_workspace"
        ).format(workspace="w1")
        assert not wk_button.button_reencrypt.isVisible()

    await aqtbot.wait_until(_demoted_message_shown)


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(logged_gui_as_admin=True)
async def test_workspace_reencryption_not_found_error(
    aqtbot,
    running_backend,
    logged_gui,
    autoclose_dialog,
    monkeypatch,
    reencryption_needed_workspace,
):

    w_w = await logged_gui.test_switch_to_workspaces_widget()

    await display_reencryption_button(aqtbot, monkeypatch, w_w)
    wk_button = w_w.layout_workspaces.itemAt(0).widget()

    def mocked_start_reencryption(self, workspace_id):
        raise FSWorkspaceNotFoundError("")

    # I did not found another way than mocks to trigger the FSWorkspaceNotFoundError from the GUI
    w_w.core.user_fs.workspace_start_reencryption = mocked_start_reencryption.__get__(
        w_w.core.user_fs
    )
    aqtbot.mouse_click(wk_button.button_reencrypt, QtCore.Qt.LeftButton)

    def _assert_error():
        assert len(autoclose_dialog.dialogs) == 1
        assert autoclose_dialog.dialogs == [
            ("Error", translate("TEXT_WORKSPACE_REENCRYPT_NOT_FOUND_ERROR"))
        ]
        assert wk_button.button_reencrypt.isVisible()

    await aqtbot.wait_until(_assert_error)


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.parametrize(
    "error_type",
    [FSBackendOfflineError, FSError, FSWorkspaceNoAccess, FSWorkspaceNotFoundError, Exception],
)
@customize_fixtures(logged_gui_as_admin=True)
async def test_workspace_reencryption_do_one_batch_error(
    caplog,
    aqtbot: AsyncQtBot,
    running_backend,
    logged_gui,
    autoclose_dialog,
    monkeypatch: pytest.MonkeyPatch,
    reencryption_needed_workspace: EntryID,
    error_type: FSBackendOfflineError
    | FSError
    | FSWorkspaceNoAccess
    | FSWorkspaceNotFoundError
    | Exception,
):

    expected_errors = {
        FSBackendOfflineError: translate("TEXT_WORKSPACE_REENCRYPT_OFFLINE_ERROR"),
        FSError: translate("TEXT_WORKSPACE_REENCRYPT_FS_ERROR"),
        FSWorkspaceNoAccess: translate("TEXT_WORKSPACE_REENCRYPT_ACCESS_ERROR"),
        FSWorkspaceNotFoundError: translate("TEXT_WORKSPACE_REENCRYPT_NOT_FOUND_ERROR"),
        Exception: translate("TEXT_WORKSPACE_REENCRYPT_UNKOWN_ERROR"),
    }

    w_w: WorkspacesWidget = await logged_gui.test_switch_to_workspaces_widget()
    await display_reencryption_button(aqtbot, monkeypatch, w_w)

    wk_button = w_w.layout_workspaces.itemAt(0).widget()

    async def mocked_start_reencryption(self, workspace_id):
        class Job:
            async def do_one_batch(self, *args, **kwargs):
                raise error_type("")

        return Job()

    w_w.core.user_fs.workspace_start_reencryption = mocked_start_reencryption.__get__(
        w_w.core.user_fs
    )
    aqtbot.mouse_click(wk_button.button_reencrypt, QtCore.Qt.LeftButton)

    def _assert_error():
        assert len(autoclose_dialog.dialogs) == 1
        assert autoclose_dialog.dialogs == [("Error", expected_errors[error_type])]
        assert wk_button.button_reencrypt.isVisible()

    await aqtbot.wait_until(_assert_error)
    # Unexpected error is logged
    if error_type is Exception:
        caplog.assert_occurred_once(
            "[error    ] Uncatched error in Qt/trio job [parsec.core.gui.trio_jobs]"
        )


@pytest.mark.gui
@pytest.mark.trio
async def test_workspace_reencryption_continue(
    aqtbot,
    running_backend,
    gui_factory: GuiFactory,
    monkeypatch: pytest.MonkeyPatch,
    bob_user_fs: UserFS,
    alice: LocalDevice,
    user_fs_factory,
):
    # Create a shared workspace
    wid = await bob_user_fs.workspace_create(EntryName("w1"))
    workspace = bob_user_fs.get_workspace(wid)
    await workspace.touch("/foo.txt")
    await workspace.sync()
    await bob_user_fs.sync()
    await bob_user_fs.workspace_share(wid, alice.user_id, WorkspaceRole.OWNER)

    # FIXME: see https://github.com/Scille/parsec-cloud/issues/4050
    # Make sure the workspace sharing message is processed and part of a
    # user manifest synced in server. This is to avoid concurrent operation
    # when message monitor starts.
    async with user_fs_factory(alice) as alice_user_fs:
        await alice_user_fs.process_last_messages()
        await alice_user_fs.sync()

    await bob_user_fs.workspace_start_reencryption(wid)

    gui = await gui_factory()
    await gui.test_switch_to_logged_in(alice)
    w_w = gui.test_get_workspaces_widget()

    await display_reencryption_button(aqtbot, monkeypatch, w_w)

    monkeypatch.setattr(
        "parsec.core.gui.workspaces_widget.ask_question",
        lambda *args, **kwargs: translate("ACTION_WORKSPACE_REENCRYPTION_CONFIRM"),
    )

    wk_button = w_w.layout_workspaces.itemAt(0).widget()
    async with aqtbot.wait_signals(
        [wk_button.button_reencrypt.clicked, wk_button.reencrypt_clicked]
    ):
        aqtbot.mouse_click(wk_button.button_reencrypt, QtCore.Qt.LeftButton)

    def _reencrypt_button_not_displayed():
        assert not wk_button.button_reencrypt.isVisible()

    await aqtbot.wait_until(_reencrypt_button_not_displayed)
