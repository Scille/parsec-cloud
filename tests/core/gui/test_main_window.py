# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from PyQt5 import QtCore, QtWidgets

from parsec.core.gui.lang import translate
from parsec.core.types import BackendOrganizationFileLinkAddr


@pytest.fixture
async def logged_gui_with_files(
    aqtbot, running_backend, backend, autoclose_dialog, logged_gui, bob, monkeypatch
):
    w_w = await logged_gui.test_switch_to_workspaces_widget()

    assert logged_gui.tab_center.count() == 1

    monkeypatch.setattr(
        "parsec.core.gui.workspaces_widget.get_text_input", lambda *args, **kwargs: ("w1")
    )
    monkeypatch.setattr(
        "parsec.core.gui.files_widget.get_text_input", lambda *args, **kwargs: ("dir1")
    )

    await aqtbot.mouse_click(w_w.button_add_workspace, QtCore.Qt.LeftButton)

    def workspace_button_ready():
        assert w_w.layout_workspaces.count() == 1
        wk_button = w_w.layout_workspaces.itemAt(0).widget()
        assert not isinstance(wk_button, QtWidgets.QLabel)

    await aqtbot.wait_until(workspace_button_ready)

    f_w = logged_gui.test_get_files_widget()
    wk_button = w_w.layout_workspaces.itemAt(0).widget()
    async with aqtbot.wait_exposed(f_w), aqtbot.wait_signal(f_w.folder_changed):
        await aqtbot.mouse_click(wk_button, QtCore.Qt.LeftButton)

    def _entry_available():
        assert f_w.workspace_fs.get_workspace_name() == "w1"
        assert f_w.table_files.rowCount() == 1

    await aqtbot.wait_until(_entry_available)

    def folder_ready():
        assert f_w.isVisible()
        assert f_w.table_files.rowCount() == 2
        folder = f_w.table_files.item(1, 1)
        assert folder
        assert folder.text() == "dir1"

    async with aqtbot.wait_signal(f_w.folder_stat_success, timeout=3000):
        await aqtbot.mouse_click(f_w.button_create_folder, QtCore.Qt.LeftButton)

    await aqtbot.wait_until(folder_ready)

    d_w = await logged_gui.test_switch_to_devices_widget()

    def device_widget_ready():
        assert d_w.isVisible()

    await aqtbot.wait_until(device_widget_ready)

    return logged_gui, w_w, f_w


@pytest.mark.gui
@pytest.mark.trio
async def test_file_link(
    aqtbot, running_backend, backend, autoclose_dialog, logged_gui_with_files, bob, monkeypatch
):
    logged_gui, w_w, f_w = logged_gui_with_files
    url = BackendOrganizationFileLinkAddr.build(
        f_w.core.device.organization_addr, f_w.workspace_fs.workspace_id, f_w.current_directory
    )

    class AvailableDevice:
        def __init__(self, org_id):
            self.organization_id = org_id

    device = AvailableDevice(bob.organization_id)
    monkeypatch.setattr(
        "parsec.core.gui.main_window.list_available_devices", lambda *args, **kwargs: [device]
    )

    await aqtbot.run(logged_gui.add_instance, str(url))

    def folder_ready():
        assert f_w.isVisible()
        assert f_w.table_files.rowCount() == 2
        folder = f_w.table_files.item(1, 1)
        assert folder
        assert folder.text() == "dir1"

    await aqtbot.wait_until(folder_ready)

    assert logged_gui.tab_center.count() == 1


@pytest.mark.gui
@pytest.mark.trio
async def test_file_link_invalid_path(
    aqtbot, running_backend, backend, autoclose_dialog, logged_gui_with_files, bob, monkeypatch
):
    logged_gui, w_w, f_w = logged_gui_with_files
    url = BackendOrganizationFileLinkAddr.build(
        f_w.core.device.organization_addr, f_w.workspace_fs.workspace_id, "/not_a_valid_path"
    )

    class AvailableDevice:
        def __init__(self, org_id):
            self.organization_id = org_id

    device = AvailableDevice(bob.organization_id)
    monkeypatch.setattr(
        "parsec.core.gui.main_window.list_available_devices", lambda *args, **kwargs: [device]
    )

    await aqtbot.run(logged_gui.add_instance, str(url))

    def assert_dialogs():
        assert len(autoclose_dialog.dialogs) == 1
        assert autoclose_dialog.dialogs == [("Error", translate("TEXT_FILE_GOTO_LINK_NOT_FOUND"))]

    await aqtbot.wait_until(assert_dialogs)

    assert logged_gui.tab_center.count() == 1


@pytest.mark.gui
@pytest.mark.trio
async def test_file_link_invalid_workspace(
    aqtbot, running_backend, backend, autoclose_dialog, logged_gui_with_files, bob, monkeypatch
):
    logged_gui, w_w, f_w = logged_gui_with_files
    url = BackendOrganizationFileLinkAddr.build(
        f_w.core.device.organization_addr, "not_a_workspace", "/dir1"
    )

    class AvailableDevice:
        def __init__(self, org_id):
            self.organization_id = org_id

    device = AvailableDevice(bob.organization_id)
    monkeypatch.setattr(
        "parsec.core.gui.main_window.list_available_devices", lambda *args, **kwargs: [device]
    )

    await aqtbot.run(logged_gui.add_instance, str(url))

    def assert_dialogs():
        assert len(autoclose_dialog.dialogs) == 1
        assert autoclose_dialog.dialogs == [("Error", translate("TEXT_INVALID_URL"))]

    await aqtbot.wait_until(assert_dialogs)


@pytest.mark.gui
@pytest.mark.trio
async def test_file_link_disconnected(
    aqtbot, running_backend, backend, autoclose_dialog, logged_gui_with_files, bob, monkeypatch
):
    logged_gui, w_w, f_w = logged_gui_with_files
    url = BackendOrganizationFileLinkAddr.build(
        f_w.core.device.organization_addr, f_w.workspace_fs.workspace_id, "/dir1"
    )

    class AvailableDevice:
        def __init__(self, org_id):
            self.organization_id = org_id

    device = AvailableDevice(bob.organization_id)
    monkeypatch.setattr(
        "parsec.core.gui.main_window.list_available_devices", lambda *args, **kwargs: [device]
    )

    await logged_gui.test_logout_and_switch_to_login_widget()

    await aqtbot.run(logged_gui.add_instance, str(url))

    def assert_dialogs():
        assert len(autoclose_dialog.dialogs) == 1
        assert autoclose_dialog.dialogs == [
            (
                "Error",
                translate("TEXT_FILE_LINK_PLEASE_LOG_IN_organization").format(
                    organization=bob.organization_id
                ),
            )
        ]

    await aqtbot.wait_until(assert_dialogs)

    assert logged_gui.tab_center.count() == 1
