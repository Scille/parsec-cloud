# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec.api.protocol import (
    OrganizationBootstrapRepInvalidData,
    OrganizationBootstrapRepOk,
)
from tests.backend.common import organization_bootstrap
from tests.common import (
    LocalDevice,
    OrganizationFullData,
    customize_fixtures,
    local_device_to_backend_user,
)


@pytest.mark.trio
@customize_fixtures(backend_not_populated=True)
async def test_sequestered_organization_bootstrap(
    coolorg: OrganizationFullData,
    other_org: OrganizationFullData,
    alice: LocalDevice,
    anonymous_backend_ws,
    backend,
):
    # Create organization
    org_token = "123456"
    await backend.organization.create(id=coolorg.organization_id, bootstrap_token=org_token)

    backend_alice, backend_alice_first_device = local_device_to_backend_user(alice, coolorg)

    organization_bootstrap_args = {
        "bootstrap_token": org_token,
        "root_verify_key": coolorg.root_verify_key,
        "user_certificate": backend_alice.user_certificate,
        "device_certificate": backend_alice_first_device.device_certificate,
        "redacted_user_certificate": backend_alice.redacted_user_certificate,
        "redacted_device_certificate": backend_alice_first_device.redacted_device_certificate,
        "sequester_authority_certificate": None,
    }

    # Not redacted vs redacted errors in user/device certificates
    for field, value in [
        ("redacted_user_certificate", backend_alice.user_certificate),
        ("redacted_device_certificate", backend_alice_first_device.device_certificate),
        ("user_certificate", backend_alice.redacted_user_certificate),
        ("device_certificate", backend_alice_first_device.redacted_device_certificate),
    ]:
        rep = await organization_bootstrap(
            anonymous_backend_ws,
            check_rep=False,
            **{
                **organization_bootstrap_args,
                field: value,
            },
        )
        assert isinstance(rep, OrganizationBootstrapRepInvalidData)

    # TODO: test timestamp not in the ballpark
    # TODO: test timestamp mismatch between certificates
    # TODO: test author mismatch between certificates
    # TODO: test invalid profil in user certificate
    # TODO: test redacted and non redacted user/device certificates mismatch

    # Finally valid bootstrap
    rep = await organization_bootstrap(
        anonymous_backend_ws, check_rep=False, **organization_bootstrap_args
    )
    assert isinstance(rep, OrganizationBootstrapRepOk)
