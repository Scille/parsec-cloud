# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from parsec.api.data import UserProfile
from parsec.api.protocol import HumanHandle
from parsec.core.backend_connection import apiv1_backend_anonymous_cmds_factory
from parsec.core.types import BackendOrganizationBootstrapAddr
from parsec.core.invite import bootstrap_organization


@pytest.mark.trio
@pytest.mark.parametrize("empty_labels", [False, True])
async def test_good(
    running_backend, backend, alice, bob, alice_backend_cmds, user_fs_factory, empty_labels
):
    org_id = "NewOrg"
    org_token = "123456"
    await backend.organization.create(org_id, org_token)

    organization_addr = BackendOrganizationBootstrapAddr.build(
        running_backend.addr, org_id, org_token
    )

    if empty_labels:
        human_handle = HumanHandle(email="zack@example.com", label="Zack")
        device_label = "PC1"
    else:
        human_handle = None
        device_label = None

    async with apiv1_backend_anonymous_cmds_factory(addr=organization_addr) as cmds:
        new_device = await bootstrap_organization(
            cmds, human_handle=human_handle, device_label=device_label
        )

    assert new_device is not None
    assert new_device.organization_id == org_id
    assert new_device.device_label == device_label
    assert new_device.human_handle == human_handle
    assert new_device.profile == UserProfile.ADMIN

    # Test the behavior of this new device
    async with user_fs_factory(new_device, initialize_in_v0=True) as newfs:
        await newfs.workspace_create("wa")
        await newfs.sync()
