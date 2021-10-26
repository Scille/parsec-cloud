# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
from PyQt5 import QtCore, QtWidgets

from parsec.api.protocol import InvitationType, OrganizationID
from parsec.api.data import UserProfile
from parsec.core.gui.lang import translate
from parsec.core.gui.login_widget import LoginPasswordInputWidget
from parsec.core.local_device import (
    AvailableDevice,
    DeviceFileType,
    _save_device_with_password,
    save_device_with_password,
)
from parsec.core.types import EntryID
from parsec.core.types import (
    BackendInvitationAddr,
    BackendOrganizationBootstrapAddr,
    BackendOrganizationFileLinkAddr,
    BackendOrganizationAddr,
    WorkspaceRole,
)

from tests.common import customize_fixtures


@pytest.fixture
def catch_create_org_widget(widget_catcher_factory):
    return widget_catcher_factory("parsec.core.gui.create_org_widget.CreateOrgWidget")


@pytest.fixture
def catch_claim_device_widget(widget_catcher_factory):
    return widget_catcher_factory("parsec.core.gui.claim_device_widget.ClaimDeviceWidget")


@pytest.fixture
def catch_claim_user_widget(widget_catcher_factory):
    return widget_catcher_factory("parsec.core.gui.claim_user_widget.ClaimUserWidget")


@pytest.fixture
async def organization_bootstrap_addr(running_backend):
    org_id = OrganizationID("ShinraElectricPowerCompany")
    org_token = "123"
    await running_backend.backend.organization.create(org_id, org_token)
    return BackendOrganizationBootstrapAddr.build(running_backend.addr, org_id, org_token)


@pytest.fixture
async def device_invitation_addr(backend, bob):
    invitation = await backend.invite.new_for_device(
        organization_id=bob.organization_id, greeter_user_id=bob.user_id
    )
    return BackendInvitationAddr.build(
        backend_addr=bob.organization_addr,
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
        backend_addr=bob.organization_addr,
        organization_id=bob.organization_id,
        invitation_type=InvitationType.USER,
        token=invitation.token,
    )


@pytest.fixture
def bob_available_device(bob, tmp_path):
    key_file_path = tmp_path / "bob_device.key"
    _save_device_with_password(key_file=key_file_path, device=bob, password="", force=True)
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
        assert f_w.workspace_fs.get_workspace_name() == "w1"
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

    logged_gui.add_instance(str(url))

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
async def test_link_file_unmounted(aqtbot, logged_gui_with_files):
    logged_gui, w_w, f_w = logged_gui_with_files

    core = logged_gui.test_get_core()
    url = f_w.workspace_fs.generate_file_link(f_w.current_directory)

    logged_gui.add_instance(str(url))

    def _folder_ready():
        assert f_w.isVisible()
        assert f_w.table_files.rowCount() == 2
        folder = f_w.table_files.item(1, 1)
        assert folder
        assert folder.text() == "dir1"

    await aqtbot.wait_until(_folder_ready)

    assert logged_gui.tab_center.count() == 1

    def _mounted():
        assert core.mountpoint_manager.is_workspace_mounted(f_w.workspace_fs.workspace_id)

    await aqtbot.wait_until(_mounted)
    await core.mountpoint_manager.unmount_workspace(f_w.workspace_fs.workspace_id)

    def _unmounted():
        assert not core.mountpoint_manager.is_workspace_mounted(f_w.workspace_fs.workspace_id)

    await aqtbot.wait_until(_unmounted)

    logged_gui.add_instance(str(url))

    await aqtbot.wait_until(_mounted)


@pytest.mark.gui
@pytest.mark.trio
async def test_link_file_invalid_path(aqtbot, autoclose_dialog, logged_gui_with_files):
    logged_gui, w_w, f_w = logged_gui_with_files
    url = f_w.workspace_fs.generate_file_link("/unknown")

    logged_gui.add_instance(str(url))

    def _assert_dialogs():
        assert len(autoclose_dialog.dialogs) == 1
        assert autoclose_dialog.dialogs == [("Error", translate("TEXT_FILE_GOTO_LINK_NOT_FOUND"))]

    await aqtbot.wait_until(_assert_dialogs)

    assert logged_gui.tab_center.count() == 1


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.parametrize("kind", ["bad_workspace_id", "legacy_url_format"])
async def test_link_file_invalid_url(aqtbot, autoclose_dialog, logged_gui_with_files, kind):
    logged_gui, w_w, f_w = logged_gui_with_files
    org_addr = f_w.core.device.organization_addr
    if kind == "bad_workspace_id":
        url = f"parsec://{org_addr.netloc}/{org_addr.organization_id}?action=file_link&workspace_id=not_a_uuid&path=HRSW4Y3SPFYHIZLEL5YGC6LMN5QWIPQs"
    elif kind == "legacy_url_format":
        url = f"parsec://{org_addr.netloc}/{org_addr.organization_id}?action=file_link&workspace_id=449977b2-889a-4a62-bc54-f89c26175e90&path=%2Fbar.txt&no_ssl=true&rvk=ZY3JDUOCOKTLCXWS6CJTAELDZSMZYFK5QLNJAVY6LFJV5IRJWAIAssss"
    else:
        assert False

    logged_gui.add_instance(url)

    def _assert_dialogs():
        assert len(autoclose_dialog.dialogs) == 1
        assert autoclose_dialog.dialogs == [("Error", translate("TEXT_INVALID_URL"))]

    await aqtbot.wait_until(_assert_dialogs)


@pytest.mark.gui
@pytest.mark.trio
async def test_link_file_disconnected(
    aqtbot, autoclose_dialog, logged_gui_with_files, bob, monkeypatch, bob_available_device
):
    gui, w_w, f_w = logged_gui_with_files
    addr = f_w.workspace_fs.generate_file_link("/dir1")

    monkeypatch.setattr(
        "parsec.core.gui.main_window.list_available_devices",
        lambda *args, **kwargs: [bob_available_device],
    )

    # Log out and send link
    await gui.test_logout_and_switch_to_login_widget()
    gui.add_instance(addr.to_url())

    def _assert_dialogs():
        assert len(autoclose_dialog.dialogs) == 1
        assert autoclose_dialog.dialogs == [
            (
                "",
                translate("TEXT_FILE_LINK_PLEASE_LOG_IN_organization").format(
                    organization=bob.organization_id
                ),
            )
        ]

    await aqtbot.wait_until(_assert_dialogs)

    # Assert login widget is displayed
    lw = gui.test_get_login_widget()

    def _password_widget_shown():
        assert isinstance(lw.widget.layout().itemAt(0).widget(), LoginPasswordInputWidget)
        password_w = lw.widget.layout().itemAt(0).widget()
        assert password_w.button_login.isEnabled()

    await aqtbot.wait_until(_password_widget_shown)

    password_w = lw.widget.layout().itemAt(0).widget()

    # Connect to the organization
    aqtbot.mouse_click(password_w.button_login, QtCore.Qt.LeftButton)

    def _wait():
        central_widget = gui.test_get_central_widget()
        assert central_widget is not None

    await aqtbot.wait_until(_wait)

    # Assert file link has been followed
    f_w = gui.test_get_files_widget()

    def _folder_ready():
        assert f_w.isVisible()
        assert f_w.table_files.rowCount() == 2
        folder = f_w.table_files.item(1, 1)
        assert folder
        assert folder.text() == "dir1"

    await aqtbot.wait_until(_folder_ready)


@pytest.mark.gui
@pytest.mark.trio
async def test_link_file_disconnected_cancel_login(
    aqtbot, autoclose_dialog, logged_gui_with_files, bob, monkeypatch, bob_available_device
):
    gui, w_w, f_w = logged_gui_with_files
    url = f_w.workspace_fs.generate_file_link("/dir1")

    monkeypatch.setattr(
        "parsec.core.gui.main_window.list_available_devices",
        lambda *args, **kwargs: [bob_available_device],
    )

    # Log out and send link
    await gui.test_logout_and_switch_to_login_widget()
    gui.add_instance(str(url))

    def _assert_dialogs():
        assert len(autoclose_dialog.dialogs) == 1
        assert autoclose_dialog.dialogs == [
            (
                "",
                translate("TEXT_FILE_LINK_PLEASE_LOG_IN_organization").format(
                    organization=bob.organization_id
                ),
            )
        ]

    await aqtbot.wait_until(_assert_dialogs)

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
async def test_link_organization(
    aqtbot, logged_gui, catch_create_org_widget, organization_bootstrap_addr
):
    logged_gui.add_instance(organization_bootstrap_addr.to_url())
    co_w = await catch_create_org_widget()
    assert co_w
    assert logged_gui.tab_center.count() == 2


@pytest.mark.gui
@pytest.mark.trio
async def test_link_organization_disconnected(
    aqtbot, logged_gui, catch_create_org_widget, organization_bootstrap_addr
):
    await logged_gui.test_logout_and_switch_to_login_widget()
    logged_gui.add_instance(organization_bootstrap_addr.to_url())
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

    logged_gui.add_instance(url)
    cd_w = await catch_claim_device_widget()
    assert cd_w
    assert logged_gui.tab_center.count() == 2


@pytest.mark.gui
@pytest.mark.trio
async def test_link_claim_device_disconnected(
    aqtbot, logged_gui, catch_claim_device_widget, device_invitation_addr
):
    await logged_gui.test_logout_and_switch_to_login_widget()
    logged_gui.add_instance(device_invitation_addr.to_url())
    cd_w = await catch_claim_device_widget()
    assert cd_w
    assert logged_gui.tab_center.count() == 1


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.parametrize("http_redirection_url", (True, False))
async def test_link_claim_user(
    aqtbot, logged_gui, catch_claim_user_widget, user_invitation_addr, http_redirection_url
):
    if http_redirection_url:
        url = user_invitation_addr.to_http_redirection_url()
    else:
        url = user_invitation_addr.to_url()

    logged_gui.add_instance(url)
    cd_w = await catch_claim_user_widget()
    assert cd_w
    assert logged_gui.tab_center.count() == 2


@pytest.mark.gui
@pytest.mark.trio
async def test_link_claim_user_disconnected(
    aqtbot, logged_gui, catch_claim_user_widget, user_invitation_addr
):
    await logged_gui.test_logout_and_switch_to_login_widget()
    logged_gui.add_instance(user_invitation_addr.to_url())
    cd_w = await catch_claim_user_widget()
    assert cd_w
    assert logged_gui.tab_center.count() == 1


@pytest.mark.gui
@pytest.mark.trio
async def test_tab_login_logout(gui_factory, core_config, alice, monkeypatch):
    password = "P@ssw0rd"
    save_device_with_password(core_config.config_dir, alice, password)
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
async def test_tab_login_logout_two_tabs(aqtbot, gui_factory, core_config, alice, monkeypatch):
    password = "P@ssw0rd"
    save_device_with_password(core_config.config_dir, alice, password)
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
    save_device_with_password(core_config.config_dir, alice, password)
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

    save_device_with_password(core_config.config_dir, bob, password)
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
    core_config, gui_factory, autoclose_dialog, running_backend, alice
):
    password = "P@ssw0rd"
    save_device_with_password(core_config.config_dir, alice, password)

    # Cheating a bit but it does not matter, we just want a link that appears valid with
    # an unknown organization
    org_addr = BackendOrganizationAddr.build(
        running_backend.addr, "UnknownOrg", alice.organization_addr.root_verify_key
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

    accounts_w = lw.widget.layout().itemAt(0).widget()
    assert accounts_w

    assert isinstance(accounts_w, LoginPasswordInputWidget)


@pytest.mark.gui
@pytest.mark.trio
@customize_fixtures(adam_profile=UserProfile.OUTSIDER)
async def test_outsider_profil_limit(
    aqtbot, running_backend, adam, core_config, gui_factory, alice_user_fs
):
    wid = await alice_user_fs.workspace_create("workspace1")
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
