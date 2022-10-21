# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import oscrypto
import pytest
from quart import Response

from parsec.serde import packb
from parsec.api.data import EntryName
from parsec.api.protocol import OrganizationID, DeviceLabel, HumanHandle, UserProfile
from parsec.core.types import BackendOrganizationBootstrapAddr
from parsec.core.invite import bootstrap_organization, InviteNotFoundError, InviteAlreadyUsedError
from parsec.core.fs.storage.user_storage import user_storage_non_speculative_init
from parsec.sequester_crypto import SequesterVerifyKeyDer, sequester_authority_sign


@pytest.mark.trio
@pytest.mark.parametrize("with_labels", [False, True])
@pytest.mark.parametrize("backend_version", ["2.5", "2.6", "latest"])
async def test_good(
    running_backend, backend, user_fs_factory, with_labels, data_base_dir, backend_version
):

    org_id = OrganizationID("NewOrg")
    org_token = "123456"
    await backend.organization.create(id=org_id, bootstrap_token=org_token)

    organization_addr = BackendOrganizationBootstrapAddr.build(
        running_backend.addr, org_id, org_token
    )

    if with_labels:
        human_handle = HumanHandle(email="zack@example.com", label="Zack")
        device_label = DeviceLabel("PC1")
    else:
        human_handle = None
        device_label = None

    if backend_version == "2.5":

        def _mock_anonymous_api(*args, **kwargs):
            return Response(response=packb({}), status=404, content_type="application/msgpack")

        running_backend.asgi_app.view_functions["anonymous_api.anonymous_api"] = _mock_anonymous_api

    if backend_version == "2.6":

        def _mock_anonymous_api(*args, **kwargs):
            return Response(
                response=packb({"status": "unknown_command"}),
                status=200,
                content_type="application/msgpack",
            )

        running_backend.asgi_app.view_functions["anonymous_api.anonymous_api"] = _mock_anonymous_api

    new_device = await bootstrap_organization(
        organization_addr, human_handle=human_handle, device_label=device_label
    )

    assert new_device is not None
    assert new_device.organization_id == org_id
    assert new_device.device_label == device_label
    assert new_device.human_handle == human_handle
    assert new_device.profile == UserProfile.ADMIN

    # This function should always be called as part of bootstrap organization
    # (yeah, we should improve the ergonomics...)
    await user_storage_non_speculative_init(data_base_dir=data_base_dir, device=new_device)

    # Test the behavior of this new device
    async with user_fs_factory(new_device, data_base_dir=data_base_dir) as newfs:
        # New user should start with a non-speculative user manifest
        um = newfs.get_user_manifest()
        assert um.is_placeholder
        assert not um.speculative

        await newfs.workspace_create(EntryName("wa"))
        await newfs.sync()
        um = newfs.get_user_manifest()
        assert not um.is_placeholder
        assert not um.speculative

    # Test the device in correct in the backend
    backend_user, backend_device = await backend.user.get_user_with_device(
        org_id, new_device.device_id
    )
    assert backend_user.user_id == new_device.user_id
    assert backend_user.human_handle == new_device.human_handle
    assert backend_user.profile == new_device.profile
    assert backend_user.user_certifier is None
    if with_labels:
        assert backend_user.user_certificate != backend_user.redacted_user_certificate
    else:
        assert backend_user.user_certificate == backend_user.redacted_user_certificate

    assert backend_device.device_id == new_device.device_id
    assert backend_device.device_label == new_device.device_label
    assert backend_device.device_certifier is None
    if with_labels:
        assert backend_device.device_certificate != backend_device.redacted_device_certificate
    else:
        assert backend_device.device_certificate == backend_device.redacted_device_certificate


@pytest.mark.trio
async def test_bootstrap_sequester_verify_key(running_backend, backend):
    org_id = OrganizationID("NewOrg")
    org_token = "123456"
    await backend.organization.create(id=org_id, bootstrap_token=org_token)

    organization_addr = BackendOrganizationBootstrapAddr.build(
        running_backend.addr, org_id, org_token
    )
    human_handle = HumanHandle(email="zack@example.com", label="Zack")
    device_label = DeviceLabel("PC1")

    verify_key, signing_key = oscrypto.asymmetric.generate_pair("rsa", bit_size=1024)
    ref_data = b"SomeData"
    ref_data_sign = sequester_authority_sign(signing_key, ref_data)
    der_verify_key = SequesterVerifyKeyDer(verify_key)

    await bootstrap_organization(
        organization_addr,
        human_handle=human_handle,
        device_label=device_label,
        sequester_authority_verify_key=der_verify_key,
    )

    organization = await backend.organization.get(org_id)
    assert organization.sequester_authority
    assert organization.sequester_authority.verify_key_der
    organization.sequester_authority.verify_key_der.verify(ref_data_sign)


@pytest.mark.trio
async def test_invalid_token(running_backend, backend):
    org_id = OrganizationID("NewOrg")
    old_token = "123456"
    new_token = "abcdef"
    await backend.organization.create(id=org_id, bootstrap_token=old_token)
    await backend.organization.create(id=org_id, bootstrap_token=new_token)

    organization_addr = BackendOrganizationBootstrapAddr.build(
        running_backend.addr, org_id, old_token
    )

    with pytest.raises(InviteNotFoundError):
        await bootstrap_organization(organization_addr, human_handle=None, device_label=None)


@pytest.mark.trio
async def test_already_bootstrapped(running_backend, backend):
    org_id = OrganizationID("NewOrg")
    org_token = "123456"
    await backend.organization.create(id=org_id, bootstrap_token=org_token)

    organization_addr = BackendOrganizationBootstrapAddr.build(
        running_backend.addr, org_id, org_token
    )

    await bootstrap_organization(organization_addr, human_handle=None, device_label=None)

    with pytest.raises(InviteAlreadyUsedError):
        await bootstrap_organization(organization_addr, human_handle=None, device_label=None)
