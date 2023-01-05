# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest
from PyQt5 import QtCore, QtWidgets

from parsec.api.data import EntryName
from parsec.api.protocol import UserProfile
from parsec.core.gui.lang import translate
from parsec.core.gui.workspace_button import SharingStatus, WorkspaceButton
from parsec.core.local_device import save_device_with_password_in_config
from parsec.core.types import WorkspaceRole
from tests.common import customize_fixtures


@pytest.fixture
def catch_share_workspace_widget(widget_catcher_factory):
    return widget_catcher_factory("parsec.core.gui.workspace_sharing_widget.WorkspaceSharingWidget")


@pytest.fixture
async def gui_workspace_sharing(
    logged_gui, catch_share_workspace_widget, monkeypatch, aqtbot, autoclose_dialog
):
    w_w = await logged_gui.test_switch_to_workspaces_widget()

    monkeypatch.setattr(
        "parsec.core.gui.workspaces_widget.get_text_input", lambda *args, **kwargs: ("Workspace")
    )
    aqtbot.mouse_click(w_w.button_add_workspace, QtCore.Qt.LeftButton)

    def _workspace_added():
        assert w_w.layout_workspaces.count() == 1
        wk_button = w_w.layout_workspaces.itemAt(0).widget()
        assert isinstance(wk_button, WorkspaceButton)
        assert wk_button.name == EntryName("Workspace")
        assert wk_button.label_title.toolTip() == "Workspace"
        assert wk_button.label_title.text() == "Workspace"
        assert wk_button.label_shared_info.toolTip() == "private"
        assert wk_button.label_shared_info.text() == "private"
        assert not autoclose_dialog.dialogs

    await aqtbot.wait_until(_workspace_added)
    wk_button = w_w.layout_workspaces.itemAt(0).widget()

    aqtbot.mouse_click(wk_button.button_share, QtCore.Qt.LeftButton)
    share_w_w = await catch_share_workspace_widget()
    yield logged_gui, w_w, share_w_w


@pytest.mark.gui
@pytest.mark.trio
async def test_workspace_sharing_list_users(
    aqtbot, running_backend, gui_workspace_sharing, autoclose_dialog
):
    logged_gui, w_w, share_w_w = gui_workspace_sharing

    def _users_listed():
        assert share_w_w.scroll_content.layout().count() == 4

    await aqtbot.wait_until(_users_listed)

    main_user_w = share_w_w.scroll_content.layout().itemAt(0).widget()
    assert main_user_w.user_info.short_user_display == "Boby McBobFace"
    assert main_user_w.label_name.text() == "<b>Boby McBobFace</b>"
    assert main_user_w.label_email.text() == "bob@example.com"
    assert main_user_w.combo_role.currentIndex() == 0
    assert main_user_w.combo_role.currentText() == translate("TEXT_WORKSPACE_ROLE_OWNER")

    for i in range(1, 3):
        user_w = share_w_w.scroll_content.layout().itemAt(i).widget()
        assert user_w.combo_role.currentIndex() == 0
        assert user_w.combo_role.currentText() == translate("TEXT_WORKSPACE_ROLE_NOT_SHARED")
        assert user_w.isEnabled() is True


# This test has been detected as flaky.
# Using re-runs is a valid temporary solutions but the problem should be investigated in the future.
@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.flaky(reruns=3)
async def test_share_workspace(
    aqtbot,
    running_backend,
    logged_gui,
    gui_workspace_sharing,
    autoclose_dialog,
    core_config,
    alice,
    bob,
    adam,
    catch_share_workspace_widget,
    monkeypatch,
    snackbar_catcher,
):

    _, w_w, share_w_w = gui_workspace_sharing

    # 1) Logged as Bob, we share our workspace with Adam

    # Fix the return value of ensure_string_size, because it can depend of the size of the window
    monkeypatch.setattr(
        "parsec.core.gui.workspace_button.ensure_string_size",
        lambda s, size, font: (s[:16] + "..."),
    )

    def _users_listed():
        assert share_w_w.scroll_content.layout().count() == 4

    await aqtbot.wait_until(_users_listed)

    user_w = share_w_w.scroll_content.layout().itemAt(1).widget()
    assert user_w.combo_role.currentIndex() == 0
    assert user_w.user_info.short_user_display == adam.human_handle.label
    user_w.status_timer.setInterval(200)

    snackbar_catcher.reset()

    def _sharing_updated():
        assert snackbar_catcher.snackbars == [
            ("INFO", "The workspace <b>Workspace</b> has been shared with <b>Adamy McAdamFace</b>.")
        ]

    async with aqtbot.wait_signal(share_w_w.share_success):
        user_w.combo_role.setCurrentIndex(3)

    await aqtbot.wait_until(_sharing_updated)

    async with aqtbot.wait_signal(user_w.status_timer.timeout):

        def _timer_started():
            assert not user_w.label_status.pixmap().isNull()
            assert user_w.status_timer.isActive()

        await aqtbot.wait_until(_timer_started)

    def _timer_stopped():
        assert user_w.label_status.pixmap().isNull()
        assert not user_w.status_timer.isActive()

    await aqtbot.wait_until(_timer_stopped)

    # 2) Sharing info should now be displayed in the workspaces list view

    # We have to be careful about keeping a reference to the parent.
    # Otherwise, it's garbage collected later on and can trigger a
    # sporadic segfault, causing the test to become inconsistent
    parent = share_w_w.parent().parent()
    async with aqtbot.wait_signals([parent.closing, w_w.list_success]):
        parent.reject()

    def _workspace_listed():
        assert w_w.layout_workspaces.count() == 1
        wk_button = w_w.layout_workspaces.itemAt(0).widget()
        assert isinstance(wk_button, WorkspaceButton)
        assert wk_button.name == EntryName("Workspace")
        assert wk_button.label_title.toolTip() == "Workspace"
        assert wk_button.label_title.text() == "Workspace..."
        assert wk_button.label_shared_info.toolTip() == "shared with Adamy McAdamFace"
        assert wk_button.label_shared_info.text() == "shared with Adam..."
        assert not autoclose_dialog.dialogs

    await aqtbot.wait_until(_workspace_listed)

    # 3) Now loggin as Adam and check the workspaces view

    password = "P@ssw0rd"
    save_device_with_password_in_config(core_config.config_dir, adam, password)
    await logged_gui.test_logout()
    await logged_gui.test_proceed_to_login(adam, password)

    w_w = await logged_gui.test_switch_to_workspaces_widget()

    def _workspace_listed():
        assert w_w.layout_workspaces.count() == 1
        wk_button = w_w.layout_workspaces.itemAt(0).widget()
        assert isinstance(wk_button, WorkspaceButton)
        assert wk_button.name == EntryName("Workspace")
        assert not autoclose_dialog.dialogs

    await aqtbot.wait_until(_workspace_listed)

    w_b = w_w.layout_workspaces.itemAt(0).widget()
    assert isinstance(w_b, WorkspaceButton)
    assert w_b.workspace_name == EntryName("Workspace")
    assert w_b.is_owner is False

    # Also check the workspace shared with view

    aqtbot.mouse_click(w_b.button_share, QtCore.Qt.LeftButton)
    share_w_w = await catch_share_workspace_widget()

    def _users_listed():
        assert share_w_w.scroll_content.layout().count() == 4

    await aqtbot.wait_until(_users_listed)

    user_w = share_w_w.scroll_content.layout().itemAt(0).widget()
    assert user_w.user_info.user_id == bob.user_id
    assert user_w.role == WorkspaceRole.OWNER
    assert not user_w.is_current_user
    assert user_w.combo_role.currentIndex() == 0
    assert user_w.isEnabled() is False

    user_w = share_w_w.scroll_content.layout().itemAt(1).widget()
    assert user_w.user_info.user_id == adam.user_id
    assert user_w.role == WorkspaceRole.MANAGER
    assert user_w.is_current_user
    assert user_w.combo_role.currentIndex() == 0
    assert user_w.isEnabled() is False

    user_w = share_w_w.scroll_content.layout().itemAt(2).widget()
    assert user_w.user_info.user_id == alice.user_id
    assert user_w.role is None
    assert not user_w.is_current_user
    assert user_w.combo_role.currentIndex() == 0
    assert user_w.isEnabled() is True


@pytest.mark.gui
@pytest.mark.trio
async def test_share_workspace_offline(
    aqtbot, running_backend, logged_gui, gui_workspace_sharing, snackbar_catcher
):
    _, w_w, share_w_w = gui_workspace_sharing

    def _users_listed():
        assert share_w_w.scroll_content.layout().count() == 4

    await aqtbot.wait_until(_users_listed)

    user_w = share_w_w.scroll_content.layout().itemAt(1).widget()
    assert user_w.combo_role.currentIndex() == 0

    with running_backend.offline():
        user_w.combo_role.setCurrentIndex(3)

        def _error_shown():
            assert snackbar_catcher.snackbars == [
                ("WARN", translate("TEXT_WORKSPACE_SHARING_OFFLINE"))
            ]

        await aqtbot.wait_until(_error_shown)


@pytest.mark.gui
@pytest.mark.trio
# Only bob can be set as outsider (given Alice and Adam are used to invite news users),
# so we have to login as Alice (hence the `logged_gui_as_admin`...)
@customize_fixtures(logged_gui_as_admin=True)
@customize_fixtures(bob_profile=UserProfile.OUTSIDER)
async def test_share_with_outsider_limit_roles(
    aqtbot, running_backend, logged_gui, gui_workspace_sharing, snackbar_catcher
):
    _, w_w, share_w_w = gui_workspace_sharing

    def _users_listed():
        assert share_w_w.scroll_content.layout().count() == 4

    await aqtbot.wait_until(_users_listed)

    for role_index, role_name in [(3, "Manager"), (4, "Owner")]:
        select_bob_w = share_w_w.scroll_content.layout().itemAt(2).widget()
        assert select_bob_w.label_email.text() == "bob@example.com"
        assert select_bob_w.combo_role.itemText(role_index) == role_name
        assert select_bob_w.combo_role.model().item(role_index).isEnabled() is False

        select_bob_w.combo_role.setCurrentIndex(3)

        def _error_shown():
            assert snackbar_catcher.snackbars == [
                (
                    "WARN",
                    translate("TEXT_WORKSPACE_SHARING_SHARE_ERROR_workspace-user").format(
                        workspace="Workspace", user="Boby McBobFace"
                    ),
                )
            ]

        await aqtbot.wait_until(_error_shown)


@pytest.mark.gui
@pytest.mark.trio
async def test_workspace_sharing_filter_users(
    aqtbot, running_backend, gui_workspace_sharing, autoclose_dialog
):
    logged_gui, w_w, share_w_w = gui_workspace_sharing

    def _users_listed():
        assert share_w_w.scroll_content.layout().count() == 4

    await aqtbot.wait_until(_users_listed)

    def _users_visible():
        visible = 0
        for i in range(share_w_w.scroll_content.layout().count() - 1):
            if share_w_w.scroll_content.layout().itemAt(i).widget().isVisible():
                visible += 1
        return visible

    def _reset_input():
        share_w_w.line_edit_filter.setText("")

    assert _users_visible() == 3

    await aqtbot.key_clicks(share_w_w.line_edit_filter, "face")
    await aqtbot.wait_until(lambda: _users_visible() == 3)
    _reset_input()

    await aqtbot.key_clicks(share_w_w.line_edit_filter, "mca")
    await aqtbot.wait_until(lambda: _users_visible() == 2)
    _reset_input()

    await aqtbot.key_clicks(share_w_w.line_edit_filter, "bob")
    await aqtbot.wait_until(lambda: _users_visible() == 1)
    _reset_input()

    await aqtbot.key_clicks(share_w_w.line_edit_filter, "zoidberg")
    await aqtbot.wait_until(lambda: _users_visible() == 0)
    _reset_input()


@pytest.mark.gui
@pytest.mark.trio
async def test_share_workspace_while_connected(
    aqtbot, running_backend, logged_gui, autoclose_dialog, alice_user_fs, bob
):
    w_w = await logged_gui.test_switch_to_workspaces_widget()
    wid = await alice_user_fs.workspace_create(EntryName("Workspace"))

    def _no_workspace_listed():
        assert w_w.layout_workspaces.count() == 1
        label = w_w.layout_workspaces.itemAt(0).widget()
        assert isinstance(label, QtWidgets.QLabel)

    await aqtbot.wait_until(_no_workspace_listed)

    await alice_user_fs.workspace_share(wid, bob.user_id, WorkspaceRole.MANAGER)

    def _one_workspace_listed():
        assert w_w.layout_workspaces.count() == 1
        wk_button = w_w.layout_workspaces.itemAt(0).widget()
        assert isinstance(wk_button, WorkspaceButton)
        wk_button.name == "Workspace"

    await aqtbot.wait_until(_one_workspace_listed)


@pytest.mark.gui
@pytest.mark.trio
async def test_unshare_workspace_while_connected(
    aqtbot, running_backend, logged_gui, autoclose_dialog, alice_user_fs, bob, snackbar_catcher
):
    w_w = await logged_gui.test_switch_to_workspaces_widget()
    wid = await alice_user_fs.workspace_create(EntryName("Workspace"))

    await alice_user_fs.workspace_share(wid, bob.user_id, WorkspaceRole.MANAGER)

    def _one_workspace_listed():
        assert w_w.layout_workspaces.count() == 1
        wk_button = w_w.layout_workspaces.itemAt(0).widget()
        assert isinstance(wk_button, WorkspaceButton)
        wk_button.name == EntryName("Workspace")

    await aqtbot.wait_until(_one_workspace_listed)
    assert snackbar_catcher.snackbars == [
        ("INFO", "The workspace <b>Workspace</b> has been shared with you.")
    ]
    snackbar_catcher.reset()

    await alice_user_fs.workspace_share(wid, bob.user_id, None)

    def _no_workspace_listed():
        assert w_w.layout_workspaces.count() == 1
        label = w_w.layout_workspaces.itemAt(0).widget()
        assert isinstance(label, QtWidgets.QLabel)

    await aqtbot.wait_until(_no_workspace_listed)

    assert autoclose_dialog.dialogs[0] == (
        "Error",
        translate("TEXT_FILE_SHARING_REVOKED_workspace").format(workspace="Workspace"),
    )
    assert snackbar_catcher.snackbars == [
        ("INFO", "The workspace <b>Workspace</b> is no longer shared with you.")
    ]


@pytest.mark.skip(
    "Following some optimization on workspaces loading, a workspace shared with only revoked users still appears as shared"
)
@customize_fixtures(logged_gui_as_admin=True)
@pytest.mark.gui
@pytest.mark.trio
async def test_rename_workspace_when_revoked(
    aqtbot, running_backend, logged_gui, autoclose_dialog, qt_thread_gateway, bob, monkeypatch
):
    w_w = await logged_gui.test_switch_to_workspaces_widget()

    core = logged_gui.test_get_tab().core
    wid = await core.user_fs.workspace_create(EntryName("Workspace"))

    def _workspace_not_shared_listed():
        assert w_w.layout_workspaces.count() == 1
        wk_button = w_w.layout_workspaces.itemAt(0).widget()
        assert isinstance(wk_button, WorkspaceButton)
        assert wk_button.label_title.text() == "Workspace"
        assert wk_button.label_title.toolTip() == "Workspace"
        assert wk_button.label_shared_info.tooltip() == "private"
        assert wk_button.label_shared_info.text() == "private"
        assert wk_button.sharing_status != SharingStatus.Shared
        assert wk_button.name == EntryName("Workspace")

    await aqtbot.wait_until(_workspace_not_shared_listed)

    wid = w_w.layout_workspaces.itemAt(0).widget().workspace_fs.workspace_id

    await core.user_fs.workspace_share(wid, bob.user_id, WorkspaceRole.MANAGER)

    def _workspace_shared_listed():
        assert w_w.layout_workspaces.count() == 1
        wk_button = w_w.layout_workspaces.itemAt(0).widget()
        assert isinstance(wk_button, WorkspaceButton)
        assert wk_button.sharing_status == SharingStatus.Shared
        assert wk_button.name == EntryName("Workspace")
        assert wk_button.label_title.toolTip() == "Workspace"
        assert wk_button.label_title.text() == "Workspace"
        assert wk_button.label_shared_info.tooltip() == "shared with Boby McBobFace"
        assert wk_button.label_shared_info.text() == "shared with Boby McBobyFace"

    await aqtbot.wait_until(_workspace_shared_listed)

    await core.revoke_user(bob.user_id)

    w_w = await logged_gui.test_switch_to_users_widget()
    w_w = await logged_gui.test_switch_to_workspaces_widget()

    await aqtbot.wait_until(_workspace_not_shared_listed)
