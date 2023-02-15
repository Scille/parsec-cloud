# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import re
from collections import defaultdict
from contextlib import asynccontextmanager
from typing import Optional, Union

import pytest

from parsec._parsec import LocalDevice
from parsec.api.protocol import DeviceID, DeviceLabel, HumanHandle, OrganizationID, UserProfile
from parsec.core.types import BackendOrganizationBootstrapAddr
from parsec.crypto import SigningKey
from tests.common.binder import OrganizationFullData
from tests.common.freeze_time import freeze_time
from tests.common.sequester import sequester_authority_factory


@pytest.fixture
def organization_factory(backend_addr):
    organizations = set()
    count = 0

    def _organization_factory(orgname=None, sequestered: bool = False):
        nonlocal count

        if not orgname:
            count += 1
            orgname = f"Org{count}"

        organization_id = OrganizationID(orgname)
        assert organization_id not in organizations
        organizations.add(organization_id)
        bootstrap_token = f"<{orgname}-bootstrap-token>"
        bootstrap_addr = BackendOrganizationBootstrapAddr.build(
            backend_addr, organization_id=organization_id, token=bootstrap_token
        )
        root_signing_key = SigningKey.generate()
        addr = bootstrap_addr.generate_organization_addr(root_signing_key.verify_key)

        if sequestered:
            sequester_authority = sequester_authority_factory(
                organization_root_signing_key=root_signing_key
            )
        else:
            sequester_authority = None

        return OrganizationFullData(
            bootstrap_addr=bootstrap_addr,
            addr=addr,
            root_signing_key=root_signing_key,
            sequester_authority=sequester_authority,
        )

    return _organization_factory


@pytest.fixture
def local_device_factory(coolorg):
    devices = defaultdict(list)
    count = 0

    def _local_device_factory(
        base_device_id: Optional[Union[str, DeviceID]] = None,
        org: OrganizationFullData = coolorg,
        profile: Optional[UserProfile] = None,
        has_human_handle: bool = True,
        base_human_handle: Optional[Union[str, HumanHandle]] = None,
        has_device_label: bool = True,
        base_device_label: Optional[Union[str, DeviceLabel]] = None,
    ):
        nonlocal count

        if not base_device_id:
            count += 1
            base_device_id = f"user{count}@dev0"

        org_devices = devices[org.organization_id]
        if isinstance(base_device_id, DeviceID):
            device_id = base_device_id
        else:
            device_id = DeviceID(base_device_id)
        assert not any(d for d in org_devices if d.device_id == device_id)

        if not has_device_label:
            assert base_device_label is None
            device_label = None
        elif not base_device_label:
            device_label = DeviceLabel(f"My {device_id.device_name.str} machine")
        elif isinstance(base_device_label, DeviceLabel):
            device_label = base_device_label
        else:
            device_label = DeviceLabel(base_device_label)

        if not has_human_handle:
            assert base_human_handle is None
            human_handle = None
        elif not base_human_handle:
            name = device_id.user_id.str.capitalize()
            human_handle = HumanHandle(
                email=f"{device_id.user_id.str}@example.com", label=f"{name}y Mc{name}Face"
            )
        elif isinstance(base_human_handle, HumanHandle):
            human_handle = base_human_handle
        else:
            match = re.match(r"(.*) <(.*)>", base_human_handle)
            if match:
                label, email = match.groups()
            else:
                label = base_human_handle
                email = f"{device_id.user_id.str}@example.com"
            human_handle = HumanHandle(email=email, label=label)

        parent_device = None
        try:
            # If the user already exists, we must retrieve it data
            parent_device = next(d for d in org_devices if d.user_id == device_id.user_id)
            if profile is not None and profile != parent_device.profile:
                raise ValueError(
                    "profile is set but user already exists, with a different profile value."
                )
            profile = parent_device.profile

        except StopIteration:
            profile = profile or UserProfile.STANDARD

        device = LocalDevice.generate_new_device(
            organization_addr=org.addr,
            device_id=device_id,
            profile=profile,
            human_handle=human_handle,
            device_label=device_label,
        )
        if parent_device is not None:
            device = device.evolve(
                private_key=parent_device.private_key,
                user_manifest_id=parent_device.user_manifest_id,
                user_manifest_key=parent_device.user_manifest_key,
            )
        org_devices.append(device)
        return device

    return _local_device_factory


@pytest.fixture
def coolorg(fixtures_customization, organization_factory):
    # Fonzie approve this
    return organization_factory(
        "CoolOrg",
        sequestered=fixtures_customization.get("coolorg_is_sequestered_organization", False),
    )


@pytest.fixture
def otherorg(organization_factory):
    return organization_factory("OtherOrg")


@pytest.fixture
def expiredorg(organization_factory):
    expired_org = organization_factory("ExpiredOrg")
    return expired_org


@pytest.fixture
def otheralice(fixtures_customization, local_device_factory, otherorg):
    return local_device_factory(
        "alice@dev1",
        otherorg,
        # otheralice mimics alice
        profile=fixtures_customization.get("alice_profile", UserProfile.ADMIN),
        has_human_handle=fixtures_customization.get("alice_has_human_handle", True),
        has_device_label=fixtures_customization.get("alice_has_device_label", True),
    )


@pytest.fixture
def alice(fixtures_customization, local_device_factory, initial_user_manifest_state) -> LocalDevice:
    device = local_device_factory(
        "alice@dev1",
        profile=fixtures_customization.get("alice_profile", UserProfile.ADMIN),
        has_human_handle=fixtures_customization.get("alice_has_human_handle", True),
        has_device_label=fixtures_customization.get("alice_has_device_label", True),
    )
    # Force alice user manifest v1 to be signed by user alice@dev1
    # This is needed given backend_factory bind alice@dev1 then alice@dev2,
    # hence user manifest v1 is stored in backend at a time when alice@dev2
    # doesn't exists.
    with freeze_time("2000-01-01"):
        initial_user_manifest_state.force_user_manifest_v1_generation(device)
    return device


@pytest.fixture
def expiredorgalice(
    fixtures_customization, local_device_factory, initial_user_manifest_state, expiredorg
):
    device = local_device_factory(
        "alice@dev1",
        expiredorg,
        # expiredorgalice mimics alice
        profile=fixtures_customization.get("alice_profile", UserProfile.ADMIN),
        has_human_handle=fixtures_customization.get("alice_has_human_handle", True),
        has_device_label=fixtures_customization.get("alice_has_device_label", True),
    )
    # Force alice user manifest v1 to be signed by user alice@dev1
    # This is needed given backend_factory bind alice@dev1 then alice@dev2,
    # hence user manifest v1 is stored in backend at a time when alice@dev2
    # doesn't exists.
    with freeze_time("2000-01-01"):
        initial_user_manifest_state.force_user_manifest_v1_generation(device)
    return device


@pytest.fixture
def alice2(fixtures_customization, local_device_factory):
    return local_device_factory(
        "alice@dev2",
        profile=fixtures_customization.get("alice_profile", UserProfile.ADMIN),
        has_human_handle=fixtures_customization.get("alice_has_human_handle", True),
        has_device_label=fixtures_customization.get("alice_has_device_label", True),
    )


@pytest.fixture
def adam(fixtures_customization, local_device_factory):
    return local_device_factory(
        "adam@dev1",
        profile=fixtures_customization.get("adam_profile", UserProfile.ADMIN),
        has_human_handle=fixtures_customization.get("adam_has_human_handle", True),
        has_device_label=fixtures_customization.get("adam_has_device_label", True),
    )


@pytest.fixture
def bob(fixtures_customization, local_device_factory):
    return local_device_factory(
        "bob@dev1",
        profile=fixtures_customization.get("bob_profile", UserProfile.STANDARD),
        has_human_handle=fixtures_customization.get("bob_has_human_handle", True),
        has_device_label=fixtures_customization.get("bob_has_device_label", True),
    )


@pytest.fixture
def mallory(fixtures_customization, local_device_factory):
    return local_device_factory(
        "mallory@dev1",
        profile=fixtures_customization.get("mallory_profile", UserProfile.STANDARD),
        has_human_handle=fixtures_customization.get("mallory_has_human_handle", True),
        has_device_label=fixtures_customization.get("mallory_has_device_label", True),
    )


@pytest.fixture
def ws_from_other_organization_factory(
    backend_authenticated_ws_factory,
    backend_data_binder_factory,
    organization_factory,
    local_device_factory,
):
    @asynccontextmanager
    async def _ws_from_other_organization_factory(
        backend_asgi_app,
        mimick: Optional[str] = None,
        anonymous: bool = False,
        profile: UserProfile = UserProfile.STANDARD,
    ):
        binder = backend_data_binder_factory(backend_asgi_app.backend)

        other_org = organization_factory()
        if mimick:
            other_device = local_device_factory(
                base_device_id=mimick, org=other_org, profile=profile
            )
        else:
            other_device = local_device_factory(org=other_org, profile=profile)
        await binder.bind_organization(other_org, other_device)

        if anonymous:
            auth_as = other_org.organization_id
        else:
            auth_as = other_device
        async with backend_authenticated_ws_factory(backend_asgi_app, auth_as) as sock:
            sock.device = other_device
            yield sock

    return _ws_from_other_organization_factory
