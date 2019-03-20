# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from PyQt5 import QtCore

from parsec.types import BackendOrganizationBootstrapAddr
from parsec.core.gui.main_window import MainWindow
from parsec.core.gui.trio_thread import run_trio_thread


@pytest.fixture
def gui(qtbot, core_config):
    with run_trio_thread() as portal:
        main_w = MainWindow(portal, core_config)
        qtbot.addWidget(main_w)
        yield main_w


@pytest.fixture
def gui_ready_for_bootstrap(qtbot, gui, backend_service):
    backend_service.start(populated=False)

    org_id = "NewOrg"
    org_token = "123456"
    user_id = "Zack"
    device_name = "pc1"
    password = "S3cr3tP@ss"
    organization_addr = BackendOrganizationBootstrapAddr.build(
        backend_service.get_url(), org_id, org_token
    )

    # Create organization in the backend

    async def _create_organization(backend):
        await backend.organization.create(org_id, org_token)

    backend_service.execute(_create_organization)

    # Go do the bootstrap

    login_w = gui.login_widget
    assert not login_w.bootstrap_organization.isVisible()
    qtbot.mouseClick(login_w.button_bootstrap_instead, QtCore.Qt.LeftButton)
    # assert login_w.bootstrap_organization.isVisible()

    qtbot.keyClicks(login_w.bootstrap_organization.line_edit_login, user_id)
    qtbot.keyClicks(login_w.bootstrap_organization.line_edit_password, password)
    qtbot.keyClicks(login_w.bootstrap_organization.line_edit_password_check, password)
    qtbot.keyClicks(login_w.bootstrap_organization.line_edit_url, organization_addr)
    qtbot.keyClicks(login_w.bootstrap_organization.line_edit_device, device_name)


@pytest.mark.gui
def test_bootstrap_organization(qtbot, gui, gui_ready_for_bootstrap, autoclose_dialog):
    login_w = gui.login_widget
    with qtbot.waitSignal(login_w.bootstrap_organization.organization_bootstrapped):
        qtbot.mouseClick(login_w.bootstrap_organization.button_bootstrap, QtCore.Qt.LeftButton)
    assert autoclose_dialog.dialogs == [
        ("Information", "The organization and the user have been created. You can now login.")
    ]


@pytest.mark.gui
def test_bootstrap_organization_backend_offline(
    qtbot, backend_service, unused_tcp_addr, gui, autoclose_dialog
):
    org_id = "NewOrg"
    org_token = "123456"
    user_id = "Zack"
    device_name = "pc1"
    password = "S3cr3tP@ss"
    organization_addr = BackendOrganizationBootstrapAddr.build(unused_tcp_addr, org_id, org_token)

    # Go do the bootstrap

    login_w = gui.login_widget
    assert not login_w.bootstrap_organization.isVisible()
    qtbot.mouseClick(login_w.button_bootstrap_instead, QtCore.Qt.LeftButton)
    # assert login_w.bootstrap_organization.isVisible()

    qtbot.keyClicks(login_w.bootstrap_organization.line_edit_login, user_id)
    qtbot.keyClicks(login_w.bootstrap_organization.line_edit_password, password)
    qtbot.keyClicks(login_w.bootstrap_organization.line_edit_password_check, password)
    qtbot.keyClicks(login_w.bootstrap_organization.line_edit_url, organization_addr)
    qtbot.keyClicks(login_w.bootstrap_organization.line_edit_device, device_name)

    login_w = gui.login_widget
    with qtbot.waitSignal(login_w.bootstrap_organization.bootstrap_error):
        qtbot.mouseClick(login_w.bootstrap_organization.button_bootstrap, QtCore.Qt.LeftButton)
    assert autoclose_dialog.dialogs == [("Error", "Can not bootstrap this organization.")]


@pytest.mark.gui
def test_bootstrap_organization_unknown_error(
    monkeypatch, qtbot, gui, gui_ready_for_bootstrap, autoclose_dialog
):
    login_w = gui.login_widget

    def _broken(*args, **kwargs):
        raise RuntimeError("Ooops...")

    monkeypatch.setattr(
        "parsec.core.gui.bootstrap_organization_widget.build_user_certificate", _broken
    )

    with qtbot.waitSignal(login_w.bootstrap_organization.bootstrap_error):
        qtbot.mouseClick(login_w.bootstrap_organization.button_bootstrap, QtCore.Qt.LeftButton)
    assert autoclose_dialog.dialogs == [("Error", "Can not bootstrap this organization.")]
    # TODO: Make a log is emitted
