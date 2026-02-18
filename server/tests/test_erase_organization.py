# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec._parsec import (
    BootstrapToken,
    DateTime,
    DeviceLabel,
    OrganizationID,
    SigningKey,
    UserProfile,
)
from parsec.components.organization import OrganizationEraseBadOutcome

from .common import (
    Backend,
    CoolorgRpcClients,
    RpcTransportError,
    generate_new_device_certificates,
    generate_new_user_certificates,
)


async def test_erase_organization_ok(backend: Backend, coolorg: CoolorgRpcClients) -> None:
    org_id = coolorg.organization_id

    dump = await backend.organization.test_dump_organizations()
    assert org_id in dump

    outcome = await backend.organization.erase(id=org_id)
    assert outcome is None

    dump = await backend.organization.test_dump_organizations()
    assert org_id not in dump

    with pytest.raises(RpcTransportError) as exc:
        await coolorg.alice.ping(ping="hello")
    assert exc.value.rep.status_code == 404

    # Try to re-erase the organization

    outcome = await backend.organization.erase(id=org_id)
    assert outcome == OrganizationEraseBadOutcome.ORGANIZATION_NOT_FOUND

    # Recreate an organization with the same name is now possible

    bootstrap_token = await backend.organization.create(
        now=DateTime.now(),
        id=coolorg.organization_id,
    )
    assert isinstance(bootstrap_token, BootstrapToken)

    root_key = SigningKey.generate()

    alice_user_certificates = generate_new_user_certificates(
        timestamp=DateTime.now(),
        user_id=coolorg.alice.user_id,
        human_handle=coolorg.alice.human_handle,
        profile=UserProfile.ADMIN,
        author_device_id=None,
        author_signing_key=root_key,
    )

    alice_device_certificates = generate_new_device_certificates(
        timestamp=alice_user_certificates.certificate.timestamp,
        user_id=coolorg.alice.user_id,
        device_id=coolorg.alice.device_id,
        device_label=DeviceLabel("Dev1"),
        author_device_id=None,
        author_signing_key=root_key,
    )

    outcome = await backend.organization.bootstrap(
        id=coolorg.organization_id,
        now=DateTime.now(),
        bootstrap_token=bootstrap_token,
        root_verify_key=root_key.verify_key,
        user_certificate=alice_user_certificates.signed_certificate,
        device_certificate=alice_device_certificates.signed_certificate,
        redacted_user_certificate=alice_user_certificates.signed_redacted_certificate,
        redacted_device_certificate=alice_device_certificates.signed_redacted_certificate,
        sequester_authority_certificate=None,
    )
    assert isinstance(outcome, tuple)


async def test_erase_organization_not_found(backend: Backend) -> None:
    dummy_id = OrganizationID("NonExistentOrg")

    outcome = await backend.organization.erase(id=dummy_id)
    assert outcome == OrganizationEraseBadOutcome.ORGANIZATION_NOT_FOUND
