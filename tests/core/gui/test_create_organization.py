# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from PyQt5 import QtCore


@pytest.fixture
def catch_create_organization_widget(widget_catcher_factory):
    return widget_catcher_factory("parsec.core.gui.create_org_widget.CreateOrgWidget")


@pytest.mark.gui
@pytest.mark.trio
async def test_create_organization_name_fields(aqtbot, gui, catch_create_organization_widget):
    await aqtbot.key_click(gui, "N", modifier=QtCore.Qt.ControlModifier)
    co_w = await catch_create_organization_widget()
    assert co_w
    org_name = "Rick Sanchez Crew"
    email = "Jerry Smith@sanchez.com"
    await aqtbot.key_clicks(co_w.current_widget.line_edit_org_name, org_name)
    assert co_w.current_widget.line_edit_org_name.text() == org_name.replace(" ", "")
    await aqtbot.key_clicks(co_w.current_widget.line_edit_user_email, email)
    assert co_w.current_widget.line_edit_user_email.text() == email.replace(" ", "")
