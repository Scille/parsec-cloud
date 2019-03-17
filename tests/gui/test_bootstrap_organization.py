# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from PyQt5 import QtCore

from parsec.types import BackendOrganizationBootstrapAddr
from parsec.core.gui.main_window import MainWindow


@pytest.mark.gui
def test_bootstrap_organization(qtbot, autoclose_dialog, backend_service, core_config):
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

    # Start GUI

    main_w = MainWindow(core_config)
    qtbot.addWidget(main_w)
    login_w = main_w.login_widget

    # Go do the bootstrap

    assert not login_w.bootstrap_organization.isVisible()
    qtbot.mouseClick(login_w.button_bootstrap_instead, QtCore.Qt.LeftButton)
    # assert login_w.bootstrap_organization.isVisible()

    qtbot.keyClicks(login_w.bootstrap_organization.line_edit_login, user_id)
    qtbot.keyClicks(login_w.bootstrap_organization.line_edit_password, password)
    qtbot.keyClicks(login_w.bootstrap_organization.line_edit_password_check, password)
    qtbot.keyClicks(login_w.bootstrap_organization.line_edit_url, organization_addr)
    qtbot.keyClicks(login_w.bootstrap_organization.line_edit_device, device_name)

    with qtbot.waitSignal(login_w.bootstrap_organization.organization_bootstrapped):
        qtbot.mouseClick(login_w.bootstrap_organization.button_bootstrap, QtCore.Qt.LeftButton)
    assert autoclose_dialog.dialogs == [
        ("Information", "The organization and the user have been created. You can now login.")
    ]
