# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from unittest.mock import patch

import pytest
from PyQt5 import QtCore, QtGui, QtWidgets

from parsec._parsec import (
    DateTime,
    DeviceFileType,
    InvitationType,
    save_device_with_password,
    save_device_with_password_in_config,
)
from parsec.api.data import EntryName
from parsec.api.protocol import OrganizationID, UserProfile
from parsec.core.fs.workspacefs import WorkspaceFSTimestamped
from parsec.core.gui import desktop
from parsec.core.gui.lang import translate
from parsec.core.gui.login_widget import LoginPasswordInputWidget
from parsec.core.gui.workspace_roles import get_role_translation as _
from parsec.core.local_device import AvailableDevice
from parsec.core.types import (
    BackendInvitationAddr,
    BackendOrganizationAddr,
    BackendOrganizationBootstrapAddr,
    BackendOrganizationFileLinkAddr,
    EntryID,
    WorkspaceRole,
)
from tests.common import customize_fixtures, freeze_time


@pytest.fixture
def catch_create_org_widget(widget_catcher_factory):
    return widget_catcher_factory("parsec.core.gui.create_org_widget.CreateOrgWidget")


@pytest.fixture
def catch_org_info_widget(widget_catcher_factory):
    return widget_catcher_factory("parsec.core.gui.organization_info_widget.OrganizationInfoWidget")


@pytest.fixture
def catch_claim_device_widget(widget_catcher_factory):
    return widget_catcher_factory("parsec.core.gui.claim_device_widget.ClaimDeviceWidget")


@pytest.fixture
def catch_claim_user_widget(widget_catcher_factory):
    return widget_catcher_factory("parsec.core.gui.claim_user_widget.ClaimUserWidget")


@pytest.fixture
def catch_text_input_widget(widget_catcher_factory):
    return widget_catcher_factory("parsec.core.gui.custom_dialogs.TextInputWidget")


@pytest.fixture
async def organization_bootstrap_addr(running_backend):
    org_id = OrganizationID("ShinraElectricPowerCompany")
    org_token = "123"
    await running_backend.backend.organization.create(id=org_id, bootstrap_token=org_token)
    return BackendOrganizationBootstrapAddr.build(running_backend.addr, org_id, org_token)


@pytest.fixture
async def device_invitation_addr(backend, bob):
    invitation = await backend.invite.new_for_device(
        organization_id=bob.organization_id, greeter_user_id=bob.user_id
    )
    return BackendInvitationAddr.build(
        backend_addr=bob.organization_addr.get_backend_addr(),
        organization_id=bob.organization_id,
        invitation_type=InvitationType.DEVICE,
        token=invitation.token,
    )


@pytest.fixture
async def user_invitation_addr(backend, bob):
    invitation = await backend.invite.new_for_user(
        organization_id=bob.organization_id,
        greeter_user_id=bob.user_id,
        claimer_email="billy@billy.corp",
    )
    return BackendInvitationAddr.build(
        backend_addr=bob.organization_addr.get_backend_addr(),
        organization_id=bob.organization_id,
        invitation_type=InvitationType.USER,
        token=invitation.token,
    )


@pytest.fixture
@pytest.mark.trio
async def bob_available_device(bob, tmp_path):
    key_file_path = tmp_path / "bob_device.keys"
    await save_device_with_password(key_file=key_file_path, device=bob, password="", force=True)
    return AvailableDevice(
        key_file_path=key_file_path,
        organization_id=bob.organization_id,
        device_id=bob.device_id,
        human_handle=bob.human_handle,
        device_label=bob.device_label,
        slug=bob.slug,
        type=DeviceFileType.PASSWORD,
    )


@pytest.fixture
async def logged_gui_with_files(
    aqtbot, running_backend, backend, autoclose_dialog, logged_gui, bob, monkeypatch
):
    w_w = await logged_gui.test_switch_to_workspaces_widget()

    assert logged_gui.tab_center.count() == 1

    monkeypatch.setattr(
        "parsec.core.gui.workspaces_widget.get_text_input", lambda *args, **kwargs: "w1"
    )
    monkeypatch.setattr(
        "parsec.core.gui.files_widget.get_text_input", lambda *args, **kwargs: ("dir1")
    )

    aqtbot.mouse_click(w_w.button_add_workspace, QtCore.Qt.LeftButton)

    def _workspace_button_ready():
        assert w_w.layout_workspaces.count() == 1
        wk_button = w_w.layout_workspaces.itemAt(0).widget()
        assert not isinstance(wk_button, QtWidgets.QLabel)
        # Important: this is necessary for the incoming click to reliably work
        assert wk_button.switch_button.isChecked()

    await aqtbot.wait_until(_workspace_button_ready)

    f_w = logged_gui.test_get_files_widget()
    wk_button = w_w.layout_workspaces.itemAt(0).widget()

    aqtbot.mouse_click(wk_button, QtCore.Qt.LeftButton)

    def _entry_available():
        assert f_w.workspace_fs is not None
        assert f_w.workspace_fs.get_workspace_name() == EntryName("w1")
        assert f_w.table_files.rowCount() == 1

    await aqtbot.wait_until(_entry_available)

    def _folder_ready():
        assert f_w.isVisible()
        assert f_w.table_files.rowCount() == 2
        folder = f_w.table_files.item(1, 1)
        assert folder
        assert folder.text() == "dir1"

    aqtbot.mouse_click(f_w.button_create_folder, QtCore.Qt.LeftButton)

    await aqtbot.wait_until(_folder_ready)

    d_w = await logged_gui.test_switch_to_devices_widget()

    def _device_widget_ready():
        assert d_w.isVisible()

    await aqtbot.wait_until(_device_widget_ready)

    return logged_gui, w_w, f_w


@pytest.mark.gui
@pytest.mark.trio
async def test_link_file(aqtbot, logged_gui_with_files):
    logged_gui, w_w, f_w = logged_gui_with_files
    url = f_w.workspace_fs.generate_file_link(f_w.current_directory)

    await logged_gui.add_instance(url.to_url())

    def _folder_ready():
        assert f_w.isVisible()
        assert f_w.table_files.rowCount() == 2
        folder = f_w.table_files.item(1, 1)
        assert folder
        assert folder.text() == "dir1"

    await aqtbot.wait_until(_folder_ready)

    assert logged_gui.tab_center.count() == 1


@pytest.mark.gui
@pytest.mark.trio
async def test_link_file_with_timestamp(aqtbot, logged_gui_with_files):
    logged_gui, w_w, f_w = logged_gui_with_files
    url = f_w.workspace_fs.generate_file_link(f_w.current_directory, DateTime.now())

    await logged_gui.add_instance(url.to_url())

    def _folder_ready():
        assert f_w.isVisible()
        # A timestamped workspace is readonly
        assert f_w.label_role.text() == _(WorkspaceRole.READER)
        assert isinstance(f_w.workspace_fs, WorkspaceFSTimestamped)

    await aqtbot.wait_until(_folder_ready)

    assert logged_gui.tab_center.count() == 1


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.parametrize("timestamp", [True, False])
async def test_link_file_unmounted(aqtbot, logged_gui_with_files, timestamp, autoclose_dialog):
    logged_gui, w_w, f_w = logged_gui_with_files
    await f_w.workspace_fs.sync()

    with freeze_time(DateTime.now().add(seconds=1)):
        if timestamp:
            timestamp = DateTime.now()
        else:
            timestamp = None
        core = logged_gui.test_get_core()

        def _mounted(ts):
            assert autoclose_dialog.dialogs == []
            assert core.mountpoint_manager.is_workspace_mounted(f_w.workspace_fs.workspace_id, ts)

        # Workspace should be mounted
        await aqtbot.wait_until(lambda: _mounted(None))

        # Checking that it has files
        def _folder_ready(is_timestamped):
            if not is_timestamped:
                assert f_w.table_files.rowCount() == 2
                folder = f_w.table_files.item(1, 1)
                assert folder
                assert folder.text() == "dir1"
            else:
                assert f_w.label_role.text() == _(WorkspaceRole.READER)
                assert isinstance(f_w.workspace_fs, WorkspaceFSTimestamped)

        await aqtbot.wait_until(lambda: _folder_ready(False))

        assert logged_gui.tab_center.count() == 1

        # Generate a file link
        url = f_w.workspace_fs.generate_file_link(f_w.current_directory, timestamp)

        # Unmount the original workspace
        await core.mountpoint_manager.unmount_workspace(f_w.workspace_fs.workspace_id, None)

        def _unmounted(ts):
            assert not core.mountpoint_manager.is_workspace_mounted(
                f_w.workspace_fs.workspace_id, ts
            )

        # Making sure that it is unmounted
        await aqtbot.wait_until(lambda: _unmounted(None))

        # Add an instance with the file link
        await logged_gui.add_instance(url.to_url())

        # Workspace should be mounted
        await aqtbot.wait_until(lambda: _mounted(timestamp))

        # If the link was created with a timestamp, the workspace should be a
        # TimestampedWorkspace, otherwise it's just a normal workspace
        await aqtbot.wait_until(lambda: _folder_ready(timestamp is not None))


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.parametrize("timestamp", [True, False])
async def test_link_file_invalid_path(aqtbot, autoclose_dialog, logged_gui_with_files, timestamp):
    logged_gui, w_w, f_w = logged_gui_with_files
    await f_w.workspace_fs.sync()

    with freeze_time(DateTime.now().add(seconds=1)):
        if timestamp:
            timestamp = DateTime.now()
        else:
            timestamp = None
        url = f_w.workspace_fs.generate_file_link("/unknown", timestamp)

        await logged_gui.add_instance(url.to_url())

        def _assert_dialogs():
            assert len(autoclose_dialog.dialogs) == 1
            assert autoclose_dialog.dialogs == [
                ("Error", translate("TEXT_FILE_GOTO_LINK_NOT_FOUND_file").format(file="unknown"))
            ]

        await aqtbot.wait_until(_assert_dialogs)

        assert logged_gui.tab_center.count() == 1


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.parametrize("kind", ["bad_workspace_id", "legacy_url_format", "bad_timestamp"])
async def test_link_file_invalid_url(aqtbot, autoclose_dialog, logged_gui_with_files, kind):
    logged_gui, w_w, f_w = logged_gui_with_files
    org_addr = f_w.core.device.organization_addr
    if kind == "bad_workspace_id":
        url = f"parsec://{org_addr.netloc}/{org_addr.organization_id.str}?action=file_link&workspace_id=not_a_uuid&path=HRSW4Y3SPFYHIZLEL5YGC6LMN5QWIPQs"  # cspell: disable-line
    elif kind == "legacy_url_format":
        url = f"parsec://{org_addr.netloc}/{org_addr.organization_id.str}?action=file_link&workspace_id=449977b2-889a-4a62-bc54-f89c26175e90&path=%2Fbar.txt&no_ssl=true&rvk=ZY3JDUOCOKTLCXWS6CJTAELDZSMZYFK5QLNJAVY6LFJV5IRJWAIAssss"  # cspell: disable-line
    elif kind == "bad_timestamp":
        url = f"parsec://{org_addr.netloc}/{org_addr.organization_id.str}?action=file_link&workspace_id=449977b2-889a-4a62-bc54-f89c26175e90&path=HRSW4Y3SPFYHIZLEL5YGC6LMN5QWIPQs&timestamp=not_a_ts"  # cspell: disable-line
    else:
        assert False

    await logged_gui.add_instance(url)

    def _assert_dialogs():
        assert len(autoclose_dialog.dialogs) == 1
        assert autoclose_dialog.dialogs == [("Error", translate("TEXT_INVALID_URL"))]

    await aqtbot.wait_until(_assert_dialogs)


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.parametrize("timestamp", [None, DateTime.now()])
async def test_link_file_disconnected(
    aqtbot,
    autoclose_dialog,
    logged_gui_with_files,
    bob,
    monkeypatch,
    bob_available_device,
    snackbar_catcher,
    timestamp,
):
    gui, w_w, f_w = logged_gui_with_files
    addr = f_w.workspace_fs.generate_file_link("/dir1", timestamp)

    async def _test_list_available_devices(*args, **kwargs) -> list[AvailableDevice]:
        return [bob_available_device]

    monkeypatch.setattr(
        "parsec.core.gui.main_window.list_available_devices", _test_list_available_devices
    )

    snackbar_catcher.reset()

    # Log out and send link
    await gui.test_logout_and_switch_to_login_widget()
    await gui.add_instance(addr.to_url())

    def _assert_snackbars():
        assert len(snackbar_catcher.snackbars) == 1
        assert snackbar_catcher.snackbars == [
            (
                "INFO",
                translate("TEXT_FILE_LINK_PLEASE_LOG_IN_organization").format(
                    organization=bob.organization_id.str
                ),
            )
        ]

    await aqtbot.wait_until(_assert_snackbars)

    # Assert login widget is displayed
    lw = gui.test_get_login_widget()

    def _password_widget_shown():
        assert lw.widget.layout().count() == 1
        assert isinstance(lw.widget.layout().itemAt(0).widget(), LoginPasswordInputWidget)
        password_w = lw.widget.layout().itemAt(0).widget()
        assert password_w.button_login.isEnabled()

    await aqtbot.wait_until(_password_widget_shown)

    password_w = lw.widget.layout().itemAt(0).widget()

    tabw = gui.test_get_tab()
    async with aqtbot.wait_signals([lw.login_with_password_clicked, tabw.logged_in]):
        aqtbot.mouse_click(password_w.button_login, QtCore.Qt.LeftButton)

    def _wait():
        central_widget = gui.test_get_central_widget()
        assert central_widget is not None

    await aqtbot.wait_until(_wait)

    # Assert file link has been followed
    f_w = gui.test_get_files_widget()

    def _folder_ready():
        assert f_w.isVisible()
        if timestamp is None:
            assert f_w.table_files.rowCount() == 2
            folder = f_w.table_files.item(1, 1)
            assert folder
            assert folder.text() == "dir1"
        else:
            assert f_w.label_role.text() == _(WorkspaceRole.READER)
            assert isinstance(f_w.workspace_fs, WorkspaceFSTimestamped)

    await aqtbot.wait_until(_folder_ready)


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.parametrize("timestamp", [None, DateTime.now()])
async def test_link_file_disconnected_cancel_login(
    aqtbot,
    autoclose_dialog,
    logged_gui_with_files,
    bob,
    monkeypatch,
    bob_available_device,
    snackbar_catcher,
    timestamp,
):
    gui, w_w, f_w = logged_gui_with_files
    url = f_w.workspace_fs.generate_file_link("/dir1", timestamp)

    async def _list_available_devices(*args, **kwargs) -> list[AvailableDevice]:
        return [bob_available_device]

    monkeypatch.setattr(
        "parsec.core.gui.main_window.list_available_devices", _list_available_devices
    )

    snackbar_catcher.reset()

    # Log out and send link
    await gui.test_logout_and_switch_to_login_widget()
    await gui.add_instance(url.to_url())

    def _assert_snackbars():
        assert len(snackbar_catcher.snackbars) == 1
        assert snackbar_catcher.snackbars == [
            (
                "INFO",
                translate("TEXT_FILE_LINK_PLEASE_LOG_IN_organization").format(
                    organization=bob.organization_id.str
                ),
            )
        ]

    await aqtbot.wait_until(_assert_snackbars)

    # Assert login widget is displayed
    lw = gui.test_get_login_widget()

    def _password_widget_shown():
        assert isinstance(lw.widget.layout().itemAt(0).widget(), LoginPasswordInputWidget)
        password_w = lw.widget.layout().itemAt(0).widget()
        assert password_w.button_login.isEnabled()

    await aqtbot.wait_until(_password_widget_shown)

    password_w = lw.widget.layout().itemAt(0).widget()

    # Cancel login
    async with aqtbot.wait_signal(lw.login_canceled):
        aqtbot.mouse_click(password_w.button_back, QtCore.Qt.LeftButton)

    await gui.test_switch_to_logged_in(bob)

    # Assert previous file link is not followed
    f_w = gui.test_get_files_widget()

    def _folder_ready():
        assert not f_w.isVisible()

    await aqtbot.wait_until(_folder_ready)


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.parametrize("timestamp", [None, DateTime.now()])
async def test_link_file_unknown_workspace(
    aqtbot, core_config, gui_factory, autoclose_dialog, running_backend, alice, timestamp
):
    password = "P@ssw0rd"
    await save_device_with_password_in_config(core_config.config_dir, alice, password)

    file_link = BackendOrganizationFileLinkAddr.build(
        organization_addr=alice.organization_addr,
        workspace_id=EntryID.new(),
        encrypted_path=b"<whatever>",
        encrypted_timestamp=b"snip",
    )

    gui = await gui_factory(core_config=core_config, start_arg=file_link.to_url())
    lw = gui.test_get_login_widget()

    def _password_prompt():
        assert len(autoclose_dialog.dialogs) == 0
        lpi_w = lw.widget.layout().itemAt(0).widget()
        assert isinstance(lpi_w, LoginPasswordInputWidget)

    await aqtbot.wait_until(_password_prompt)

    lpi_w = lw.widget.layout().itemAt(0).widget()
    await aqtbot.key_clicks(lpi_w.line_edit_password, password)
    await aqtbot.wait_until(lambda: lpi_w.line_edit_password.text() == password)

    tabw = gui.test_get_tab()
    async with aqtbot.wait_signals([lw.login_with_password_clicked, tabw.logged_in]):
        aqtbot.mouse_click(lpi_w.button_login, QtCore.Qt.LeftButton)

    def _error_shown():
        assert len(autoclose_dialog.dialogs) == 1
        assert autoclose_dialog.dialogs[0] == (
            "Error",
            "You do not have access to the workspace containing the file. It may not have been shared with you.",
        )

    await aqtbot.wait_until(_error_shown)


@pytest.mark.gui
@pytest.mark.trio
async def test_link_organization(
    aqtbot, logged_gui, catch_create_org_widget, organization_bootstrap_addr
):
    await logged_gui.add_instance(organization_bootstrap_addr.to_url())
    co_w = await catch_create_org_widget()
    assert co_w
    assert logged_gui.tab_center.count() == 2


@pytest.mark.gui
@pytest.mark.trio
async def test_link_organization_disconnected(
    aqtbot, logged_gui, catch_create_org_widget, organization_bootstrap_addr
):
    await logged_gui.test_logout_and_switch_to_login_widget()
    await logged_gui.add_instance(organization_bootstrap_addr.to_url())
    co_w = await catch_create_org_widget()
    assert co_w
    assert logged_gui.tab_center.count() == 1


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.parametrize("http_redirection_url", (True, False))
async def test_link_claim_device(
    aqtbot, logged_gui, catch_claim_device_widget, device_invitation_addr, http_redirection_url
):
    if http_redirection_url:
        url = device_invitation_addr.to_http_redirection_url()
    else:
        url = device_invitation_addr.to_url()

    await logged_gui.add_instance(url)
    cd_w = await catch_claim_device_widget()
    assert cd_w
    assert logged_gui.tab_center.count() == 2


@pytest.mark.gui
@pytest.mark.trio
async def test_link_claim_device_disconnected(
    aqtbot, logged_gui, catch_claim_device_widget, device_invitation_addr
):
    await logged_gui.test_logout_and_switch_to_login_widget()
    await logged_gui.add_instance(device_invitation_addr.to_url())
    cd_w = await catch_claim_device_widget()
    assert cd_w
    assert logged_gui.tab_center.count() == 1


# This test has been detected as flaky.
# Using re-runs is a valid temporary solutions but the problem should be investigated in the future.
@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.flaky(reruns=3)
@pytest.mark.parametrize("http_redirection_url", (True, False))
async def test_link_claim_user(
    aqtbot, logged_gui, catch_claim_user_widget, user_invitation_addr, http_redirection_url
):
    if http_redirection_url:
        url = user_invitation_addr.to_http_redirection_url()
    else:
        url = user_invitation_addr.to_url()

    await logged_gui.add_instance(url)
    cd_w = await catch_claim_user_widget()
    assert cd_w
    assert logged_gui.tab_center.count() == 2


@pytest.mark.gui
@pytest.mark.trio
async def test_link_claim_user_disconnected(
    aqtbot, logged_gui, catch_claim_user_widget, user_invitation_addr
):
    await logged_gui.test_logout_and_switch_to_login_widget()
    await logged_gui.add_instance(user_invitation_addr.to_url())
    cd_w = await catch_claim_user_widget()
    assert cd_w
    assert logged_gui.tab_center.count() == 1


@pytest.mark.gui
@pytest.mark.trio
async def test_tab_login_logout(gui_factory, core_config, alice, monkeypatch):
    password = "P@ssw0rd"
    await save_device_with_password_in_config(core_config.config_dir, alice, password)
    gui = await gui_factory()

    # Fix the return value of ensure_string_size, because it can depend of the size of the window
    monkeypatch.setattr(
        "parsec.core.gui.main_window.ensure_string_size", lambda s, size, font: (s[:16] + "...")
    )

    assert gui.tab_center.count() == 1
    assert gui.tab_center.tabText(0) == translate("TEXT_TAB_TITLE_LOG_IN_SCREEN")
    assert not gui.add_tab_button.isEnabled()
    first_created_tab = gui.test_get_tab()

    await gui.test_switch_to_logged_in(alice)
    assert gui.tab_center.count() == 1
    assert gui.tab_center.tabText(0) == "CoolOrg - Alicey..."
    assert gui.add_tab_button.isEnabled()
    assert gui.test_get_tab() == first_created_tab

    await gui.test_logout()
    assert gui.tab_center.count() == 1
    assert gui.tab_center.tabText(0) == translate("TEXT_TAB_TITLE_LOG_IN_SCREEN")
    assert not gui.add_tab_button.isEnabled()
    assert gui.test_get_tab() != first_created_tab


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(logged_gui_as_admin=True)
async def test_show_org_info(
    aqtbot, logged_gui, running_backend, snackbar_catcher, catch_org_info_widget
):
    c_w = logged_gui.test_get_central_widget()
    assert c_w is not None
    c_w.button_user.menu().actions()[0].trigger()
    oi_w = await catch_org_info_widget()
    assert oi_w

    assert (
        oi_w.label_backend_addr.text()
        == logged_gui.test_get_core().device.organization_addr.to_url()
    )
    assert oi_w.label_data_size.text() == "Data size: <b>0 B</b>"
    # Metadata size vary slightly between runs
    assert oi_w.label_metadata_size.text().startswith("Metadata size: <b>")
    assert oi_w.label_total_size.text().startswith("Total size: <b>")
    assert oi_w.label_user_active.text() == "3 active user(s)"
    assert oi_w.label_user_revoked.text() == "0 revoked user(s)"
    assert oi_w.label_user_admin.text() == "2 administrator(s)"
    assert oi_w.label_user_standard.text() == "1 standard user(s)"
    assert oi_w.label_user_outsider.text() == "0 outsider(s)"

    assert oi_w.label_outsider_allowed.text() == translate("TEXT_ORG_INFO_OUTSIDER_ALLOWED")
    assert oi_w.label_user_limit.text() == translate("TEXT_ORG_INFO_USER_LIMIT_UNLIMITED")
    assert not oi_w.label_sequestration_state.isVisible()

    aqtbot.mouse_click(oi_w.button_copy_to_clipboard, QtCore.Qt.LeftButton)
    assert snackbar_catcher.snackbars == [
        ("INFO", translate("TEXT_BACKEND_ADDR_COPIED_TO_CLIPBOARD"))
    ]
    clipboard = QtGui.QGuiApplication.clipboard()
    assert clipboard.text() == c_w.core.device.organization_addr.to_url()


@pytest.mark.gui
@pytest.mark.trio
async def test_tab_login_logout_two_tabs(aqtbot, gui_factory, core_config, alice, monkeypatch):
    password = "P@ssw0rd"
    await save_device_with_password_in_config(core_config.config_dir, alice, password)
    gui = await gui_factory()

    # Fix the return value of ensure_string_size, because it can depend of the size of the window
    monkeypatch.setattr(
        "parsec.core.gui.main_window.ensure_string_size", lambda s, size, font: (s[:16] + "...")
    )

    assert gui.tab_center.count() == 1
    assert gui.tab_center.tabText(0) == translate("TEXT_TAB_TITLE_LOG_IN_SCREEN")
    first_created_tab = gui.test_get_tab()

    await gui.test_switch_to_logged_in(alice)
    assert gui.tab_center.count() == 1
    assert gui.tab_center.tabText(0) == "CoolOrg - Alicey..."
    logged_tab = gui.test_get_tab()

    aqtbot.mouse_click(gui.add_tab_button, QtCore.Qt.LeftButton)
    assert gui.tab_center.count() == 2
    assert gui.tab_center.tabText(0) == "CoolOrg - Alicey..."
    assert gui.tab_center.tabText(1) == translate("TEXT_TAB_TITLE_LOG_IN_SCREEN")

    gui.switch_to_tab(0)

    def _logged_tab_displayed():
        assert logged_tab == gui.test_get_tab()

    await aqtbot.wait_until(_logged_tab_displayed)
    await gui.test_logout()
    assert gui.tab_center.count() == 1
    assert gui.tab_center.tabText(0) == translate("TEXT_TAB_TITLE_LOG_IN_SCREEN")
    assert gui.test_get_tab() != first_created_tab


@pytest.mark.gui
@pytest.mark.trio
async def test_tab_login_logout_two_tabs_logged_in(
    aqtbot, gui_factory, core_config, alice, bob, monkeypatch
):
    password = "P@ssw0rd"
    await save_device_with_password_in_config(core_config.config_dir, alice, password)
    gui = await gui_factory()

    # Fix the return value of ensure_string_size, because it can depend of the size of the window
    monkeypatch.setattr(
        "parsec.core.gui.main_window.ensure_string_size", lambda s, size, font: (s[:16] + "...")
    )

    assert gui.tab_center.count() == 1
    assert gui.tab_center.tabText(0) == translate("TEXT_TAB_TITLE_LOG_IN_SCREEN")

    await gui.test_switch_to_logged_in(alice)
    assert gui.tab_center.count() == 1
    assert gui.tab_center.tabText(0) == "CoolOrg - Alicey..."
    alice_logged_tab = gui.test_get_tab()

    aqtbot.mouse_click(gui.add_tab_button, QtCore.Qt.LeftButton)
    assert gui.tab_center.count() == 2
    assert gui.tab_center.tabText(0) == "CoolOrg - Alicey..."
    assert gui.tab_center.tabText(1) == translate("TEXT_TAB_TITLE_LOG_IN_SCREEN")

    await save_device_with_password_in_config(core_config.config_dir, bob, password)
    await gui.test_switch_to_logged_in(bob)
    assert gui.tab_center.count() == 2
    assert gui.tab_center.tabText(0) == "CoolOrg - Alicey..."
    assert gui.tab_center.tabText(1) == "CoolOrg - Boby M..."
    bob_logged_tab = gui.test_get_tab()
    assert bob_logged_tab != alice_logged_tab

    gui.switch_to_tab(0)

    def _logged_tab_displayed():
        assert alice_logged_tab == gui.test_get_tab()

    await aqtbot.wait_until(_logged_tab_displayed)

    await gui.test_logout()
    assert gui.tab_center.count() == 2
    assert gui.tab_center.tabText(0) == "CoolOrg - Boby M..."
    assert gui.tab_center.tabText(1) == translate("TEXT_TAB_TITLE_LOG_IN_SCREEN")


@pytest.mark.gui
@pytest.mark.trio
async def test_link_file_unknown_org(
    aqtbot, core_config, gui_factory, autoclose_dialog, running_backend, alice
):
    password = "P@ssw0rd"
    await save_device_with_password_in_config(core_config.config_dir, alice, password)

    # Cheating a bit but it does not matter, we just want a link that appears valid with
    # an unknown organization
    org_addr = BackendOrganizationAddr.build(
        running_backend.addr, OrganizationID("UnknownOrg"), alice.organization_addr.root_verify_key
    )

    file_link = BackendOrganizationFileLinkAddr.build(
        organization_addr=org_addr, workspace_id=EntryID.new(), encrypted_path=b"<whatever>"
    )

    gui = await gui_factory(core_config=core_config, start_arg=file_link.to_url())
    lw = gui.test_get_login_widget()

    assert len(autoclose_dialog.dialogs) == 1
    assert autoclose_dialog.dialogs[0][0] == "Error"
    assert autoclose_dialog.dialogs[0][1] == translate(
        "TEXT_FILE_LINK_NOT_IN_ORG_organization"
    ).format(organization="UnknownOrg")

    def _devices_listed():
        assert lw.widget.layout().count() > 0

    await aqtbot.wait_until(_devices_listed)

    accounts_w = lw.widget.layout().itemAt(0).widget()
    assert accounts_w

    assert isinstance(accounts_w, LoginPasswordInputWidget)


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(adam_profile=UserProfile.OUTSIDER)
async def test_outsider_profil_limit(
    aqtbot, running_backend, adam, core_config, gui_factory, alice_user_fs
):
    wid = await alice_user_fs.workspace_create(EntryName("workspace1"))
    await alice_user_fs.workspace_share(wid, adam.user_id, WorkspaceRole.READER)
    await alice_user_fs.process_last_messages()
    await alice_user_fs.sync()

    gui = await gui_factory()
    await gui.test_switch_to_logged_in(adam)

    w_w = await gui.test_switch_to_workspaces_widget()

    def _workspace_button_shown():
        layout_workspace = w_w.layout_workspaces.itemAt(0)
        assert layout_workspace is not None
        workspace_button = layout_workspace.widget()
        assert not isinstance(workspace_button, QtWidgets.QLabel)

    await aqtbot.wait_until(_workspace_button_shown)
    layout_workspace = w_w.layout_workspaces.itemAt(0)
    workspace_button = layout_workspace.widget()
    assert workspace_button.button_share.isVisible() is False
    assert w_w.button_add_workspace.isVisible() is False

    c_w = gui.test_get_central_widget()
    assert c_w.menu.button_users.isVisible() is False


@pytest.fixture
async def random_clipboard_data(running_backend):
    return "Still sane, Exile?"


@pytest.fixture
async def clipboard_text_provider(
    organization_bootstrap_addr,
    device_invitation_addr,
    user_invitation_addr,
):
    texts = [
        organization_bootstrap_addr.to_url(),
        device_invitation_addr.to_url(),
        user_invitation_addr.to_url(),
        "Still sane, Exile?",
    ]

    def _select_clipboard_text(idx):
        return texts[idx]

    return _select_clipboard_text


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.parametrize("clipboard_text_index", (0, 1, 2, 3))
async def test_join_organization_text_in_clipboard(
    aqtbot,
    running_backend,
    backend,
    autoclose_dialog,
    gui,
    catch_text_input_widget,
    clipboard_text_provider,
    clipboard_text_index,
):
    clipboard_text = clipboard_text_provider(clipboard_text_index)

    desktop.copy_to_clipboard(clipboard_text)

    aqtbot.key_click(gui, "o", QtCore.Qt.ControlModifier, 200)
    text_input_w = await catch_text_input_widget()
    assert text_input_w

    if clipboard_text_index == 3:
        assert text_input_w.line_edit_text.text() == ""
    else:
        assert text_input_w.line_edit_text.text() == clipboard_text


@pytest.mark.gui
@pytest.mark.trio
async def test_commercial_open_switch_offer(aqtbot, gui_factory, alice, monkeypatch):
    gui = await gui_factory()
    await gui.test_switch_to_logged_in(alice)
    c_w = gui.test_get_central_widget()
    assert c_w is not None

    actions = c_w.button_user.menu().actions()
    assert len(actions) == 4
    assert "Update subscription" not in [a.text() for a in actions]

    await gui.test_logout_and_switch_to_login_widget()

    monkeypatch.setattr("parsec.core.gui.central_widget.is_saas_addr", lambda _: True)

    await gui.test_switch_to_logged_in(alice)
    c_w = gui.test_get_central_widget()
    assert c_w is not None

    actions = c_w.button_user.menu().actions()
    assert len(actions) == 6
    assert actions[0].text() == "Update subscription"

    with patch("parsec.core.gui.central_widget.desktop.open_url") as mock:
        actions[0].triggered.emit()
        mock.assert_called_once()
