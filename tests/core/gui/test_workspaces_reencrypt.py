# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from PyQt5 import QtCore

from functools import partial

from parsec.core.gui.lang import translate
from parsec.core.gui.workspace_button import WorkspaceButton
from parsec.core.types import WorkspaceRole

from tests.common import customize_fixtures


async def revoke_user_workspace_right(workspace, owner_user_fs, invited_user_fs, invited_user_id):
    await owner_user_fs.workspace_share(workspace, invited_user_id, None)
    await owner_user_fs.process_last_messages()
    await invited_user_fs.process_last_messages()
    await owner_user_fs.sync()


@pytest.fixture
async def shared_workspace(running_backend, alice_user_fs, bob_user_fs, bob):
    wid = await alice_user_fs.workspace_create("w1")

    await alice_user_fs.sync()
    await alice_user_fs.workspace_share(wid, bob.user_id, WorkspaceRole.READER)
    await alice_user_fs.process_last_messages()
    await bob_user_fs.process_last_messages()
    await alice_user_fs.sync()
    return wid


@pytest.fixture
async def reencryption_needed_workspace(
    running_backend, shared_workspace, alice_user_fs, bob_user_fs, bob
):
    await revoke_user_workspace_right(shared_workspace, alice_user_fs, bob_user_fs, bob.user_id)
    return shared_workspace


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
    alice_user_fs,
    alice,
    bob,
):
    w_w = await logged_gui.test_switch_to_workspaces_widget()

    def _workspace_displayed(workspace_w):
        assert workspace_w.layout_workspaces.count() == 1
        wk_button = workspace_w.layout_workspaces.itemAt(0).widget()
        assert isinstance(wk_button, WorkspaceButton)
        assert wk_button.name == "w1"

    await aqtbot.wait_until(partial(_workspace_displayed, w_w), timeout=2000)
    wk_button = w_w.layout_workspaces.itemAt(0).widget()

    def _reencrypt_button_not_displayed():
        assert wk_button.button_reencrypt.isHidden()
        assert not wk_button.button_reencrypt.isVisible()

    await aqtbot.wait_until(_reencrypt_button_not_displayed, timeout=2000)

    await revoke_user_workspace_right(shared_workspace, alice_user_fs, bob_user_fs, bob.user_id)

    w_w = await logged_gui.test_switch_to_workspaces_widget()

    await aqtbot.wait_until(partial(_workspace_displayed, w_w), timeout=2000)
    wk_button = w_w.layout_workspaces.itemAt(0).widget()

    def _reencrypt_button_displayed():
        assert wk_button.button_reencrypt.isVisible()

    await aqtbot.wait_until(_reencrypt_button_displayed, timeout=2000)


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(logged_gui_as_admin=True)
async def test_workspace_reencryption(
    aqtbot,
    running_backend,
    logged_gui,
    autoclose_dialog,
    bob_user_fs,
    alice_user_fs,
    alice,
    bob,
    monkeypatch,
    reencryption_needed_workspace,
):

    w_w = await logged_gui.test_switch_to_workspaces_widget()

    def _workspace_displayed():
        assert w_w.layout_workspaces.count() == 1
        wk_button = w_w.layout_workspaces.itemAt(0).widget()
        assert isinstance(wk_button, WorkspaceButton)
        assert wk_button.name == "w1"

    await aqtbot.wait_until(_workspace_displayed, timeout=2000)
    wk_button = w_w.layout_workspaces.itemAt(0).widget()

    def _reencrypt_button_displayed():
        assert wk_button.button_reencrypt.isVisible()

    await aqtbot.wait_until(_reencrypt_button_displayed)

    monkeypatch.setattr(
        "parsec.core.gui.workspaces_widget.ask_question",
        lambda *args, **kwargs: translate("ACTION_WORKSPACE_REENCRYPTION_CONFIRM"),
    )
    async with aqtbot.wait_signals(
        [wk_button.button_reencrypt.clicked, wk_button.reencrypt_clicked]
    ):
        await aqtbot.mouse_click(wk_button.button_reencrypt, QtCore.Qt.LeftButton)

    def _reencrypt_button_not_displayed():
        assert not wk_button.button_reencrypt.isVisible()

    await aqtbot.wait_until(_reencrypt_button_not_displayed)
