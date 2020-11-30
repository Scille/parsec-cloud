# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from PyQt5 import QtCore

from parsec.core.types import WorkspaceRole, UserInfo
from parsec.core.fs.workspacefs import ReencryptionNeed
from parsec.core.gui.workspace_button import WorkspaceButton
from parsec.core.gui.lang import switch_language, translate as _


@pytest.fixture
@pytest.mark.trio
async def workspace_fs(alice_user_fs, running_backend):
    wid = await alice_user_fs.workspace_create("Workspace")
    workspace = alice_user_fs.get_workspace(wid)
    return workspace


@pytest.fixture
def alice_user_info(alice):
    return UserInfo(
        user_id=alice.user_id,
        human_handle=alice.human_handle,
        profile=alice.profile,
        revoked_on=None,
        created_on=None,
    )


@pytest.fixture
def bob_user_info(bob):
    return UserInfo(
        user_id=bob.user_id,
        human_handle=bob.human_handle,
        profile=bob.profile,
        revoked_on=None,
        created_on=None,
    )


@pytest.mark.gui
@pytest.mark.trio
async def test_workspace_button(qtbot, workspace_fs, core_config, alice_user_info):
    switch_language(core_config, "en")

    roles = {alice_user_info.user_id: (WorkspaceRole.OWNER, alice_user_info)}
    w = WorkspaceButton(
        workspace_name="Workspace",
        workspace_fs=workspace_fs,
        users_roles=roles,
        is_mounted=True,
        metadata_size=20000,
        data_size=1800000,
    )
    qtbot.addWidget(w)
    w.show()

    assert w.label_owner.isVisible() is True
    assert w.label_shared.isVisible() is False
    assert w.name == "Workspace"
    assert w.label_title.text().startswith("Workspace")
    assert w.label_title.toolTip() == "Workspace (private)"
    assert w.label_role.text() == _("TEXT_WORKSPACE_ROLE_OWNER")

    assert w.label_user_count.text() == _("TEXT_WORKSPACE_USER_COUNT_count").format(count=1)
    assert w.label_data_size.text() == _("TEXT_WORKSPACE_DATA_SIZE_size").format(size="1 MB")
    assert w.label_metadata_size.text() == _("TEXT_WORKSPACE_METADATA_SIZE_size").format(
        size="19 KB"
    )
    assert w.label_total_size.text() == _("TEXT_WORKSPACE_TOTAL_SIZE_size").format(size="1 MB")


@pytest.mark.gui
@pytest.mark.trio
async def test_workspace_button_owned_by(
    qtbot, workspace_fs, core_config, bob, alice_user_info, bob_user_info
):
    switch_language(core_config, "en")

    roles = {
        bob.user_id: (WorkspaceRole.OWNER, bob_user_info),
        alice_user_info.user_id: (WorkspaceRole.READER, alice_user_info),
    }
    w = WorkspaceButton(
        workspace_name="Workspace",
        workspace_fs=workspace_fs,
        users_roles=roles,
        is_mounted=True,
        metadata_size=20000,
        data_size=1800000,
    )

    qtbot.addWidget(w)
    w.show()
    assert w.label_owner.isVisible() is False
    assert w.label_shared.isVisible() is True
    assert w.name == "Workspace"
    assert w.label_title.text().startswith("Workspace")
    assert w.label_title.toolTip() == "Workspace (owned by Boby McBobFace)"
    assert w.label_role.text() == _("TEXT_WORKSPACE_ROLE_READER")
    assert w.label_user_count.text() == _("TEXT_WORKSPACE_USER_COUNT_count").format(count=2)
    assert w.label_data_size.text() == _("TEXT_WORKSPACE_DATA_SIZE_size").format(size="1 MB")
    assert w.label_metadata_size.text() == _("TEXT_WORKSPACE_METADATA_SIZE_size").format(
        size="19 KB"
    )
    assert w.label_total_size.text() == _("TEXT_WORKSPACE_TOTAL_SIZE_size").format(size="1 MB")


@pytest.mark.gui
@pytest.mark.trio
async def test_workspace_button_shared_with(
    qtbot, workspace_fs, core_config, bob, alice_user_info, bob_user_info
):
    switch_language(core_config, "en")

    roles = {
        bob.user_id: (WorkspaceRole.READER, bob_user_info),
        alice_user_info.user_id: (WorkspaceRole.OWNER, alice_user_info),
    }
    w = WorkspaceButton(
        workspace_name="Workspace",
        workspace_fs=workspace_fs,
        users_roles=roles,
        is_mounted=True,
        metadata_size=20000,
        data_size=1800000,
    )

    qtbot.addWidget(w)
    w.show()
    assert w.label_owner.isVisible() is True
    assert w.label_shared.isVisible() is True
    assert w.name == "Workspace"
    assert w.label_title.text().startswith("Workspace")
    assert w.label_title.toolTip() == "Workspace (shared with Boby McBobFace)"
    assert w.label_role.text() == _("TEXT_WORKSPACE_ROLE_OWNER")
    assert w.label_user_count.text() == _("TEXT_WORKSPACE_USER_COUNT_count").format(count=2)
    assert w.label_data_size.text() == _("TEXT_WORKSPACE_DATA_SIZE_size").format(size="1 MB")
    assert w.label_metadata_size.text() == _("TEXT_WORKSPACE_METADATA_SIZE_size").format(
        size="19 KB"
    )
    assert w.label_total_size.text() == _("TEXT_WORKSPACE_TOTAL_SIZE_size").format(size="1 MB")


@pytest.mark.gui
@pytest.mark.trio
async def test_workspace_button_clicked(qtbot, workspace_fs, core_config, alice_user_info):
    switch_language(core_config, "en")

    roles = {alice_user_info.user_id: (WorkspaceRole.OWNER, alice_user_info)}
    w = WorkspaceButton(
        workspace_name="Workspace",
        workspace_fs=workspace_fs,
        users_roles=roles,
        is_mounted=True,
        metadata_size=20000,
        data_size=1800000,
    )

    qtbot.addWidget(w)
    with qtbot.waitSignal(w.clicked, timeout=500) as blocker:
        qtbot.mouseClick(w, QtCore.Qt.LeftButton)
    assert blocker.args == [workspace_fs]


@pytest.mark.gui
@pytest.mark.trio
async def test_workspace_button_share_clicked(qtbot, workspace_fs, core_config, alice_user_info):
    switch_language(core_config, "en")

    roles = {alice_user_info.user_id: (WorkspaceRole.OWNER, alice_user_info)}
    w = WorkspaceButton(
        workspace_name="Workspace",
        workspace_fs=workspace_fs,
        users_roles=roles,
        is_mounted=True,
        metadata_size=20000,
        data_size=1800000,
    )
    qtbot.addWidget(w)
    with qtbot.waitSignal(w.share_clicked, timeout=500) as blocker:
        qtbot.mouseClick(w.button_share, QtCore.Qt.LeftButton)
    assert blocker.args == [workspace_fs]


@pytest.mark.gui
@pytest.mark.trio
async def test_workspace_button_rename_clicked(qtbot, workspace_fs, core_config, alice_user_info):
    switch_language(core_config, "en")

    roles = {alice_user_info.user_id: (WorkspaceRole.OWNER, alice_user_info)}
    w = WorkspaceButton(
        workspace_name="Workspace",
        workspace_fs=workspace_fs,
        users_roles=roles,
        is_mounted=True,
        metadata_size=20000,
        data_size=1800000,
    )
    qtbot.addWidget(w)
    with qtbot.waitSignal(w.rename_clicked, timeout=500) as blocker:
        qtbot.mouseClick(w.button_rename, QtCore.Qt.LeftButton)
    assert blocker.args == [w]


@pytest.mark.gui
@pytest.mark.trio
async def test_workspace_button_reencrypt_clicked(
    qtbot, workspace_fs, core_config, alice_user_info
):
    switch_language(core_config, "en")

    roles = {workspace_fs.device.user_id: (WorkspaceRole.OWNER, alice_user_info)}
    w = WorkspaceButton(
        workspace_name="Workspace",
        workspace_fs=workspace_fs,
        users_roles=roles,
        is_mounted=True,
        metadata_size=20000,
        data_size=1800000,
    )
    w.reencryption_needs = ReencryptionNeed(
        user_revoked=True, role_revoked=False, reencryption_already_in_progress=False
    )

    qtbot.addWidget(w)

    assert not w.button_reencrypt.isHidden()

    w.reencrypting = (8, 4)
    assert w.widget_actions.isHidden()
    assert not w.widget_reencryption.isHidden()
    assert w.progress_reencryption.value() == 50
    assert w.progress_reencryption.text() == "Reencrypting... 50%"

    w.reencrypting = None
    assert not w.widget_actions.isHidden()
    assert w.widget_reencryption.isHidden()


@pytest.mark.gui
@pytest.mark.trio
async def test_workspace_button_delete_clicked(qtbot, workspace_fs, core_config, alice_user_info):
    switch_language(core_config, "en")

    roles = {alice_user_info.user_id: (WorkspaceRole.OWNER, alice_user_info)}
    w = WorkspaceButton(
        workspace_name="Workspace",
        workspace_fs=workspace_fs,
        users_roles=roles,
        is_mounted=True,
        metadata_size=20000,
        data_size=1800000,
    )
    qtbot.addWidget(w)
    with qtbot.waitSignal(w.delete_clicked, timeout=500) as blocker:
        qtbot.mouseClick(w.button_delete, QtCore.Qt.LeftButton)
    assert blocker.args == [workspace_fs]
