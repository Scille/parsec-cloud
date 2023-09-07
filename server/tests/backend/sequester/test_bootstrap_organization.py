# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec._parsec import DateTime
from parsec.api.protocol import (
    OrganizationBootstrapRepBadTimestamp,
    OrganizationBootstrapRepInvalidData,
    OrganizationBootstrapRepOk,
)
from tests.backend.common import organization_bootstrap
from tests.common import (
    LocalDevice,
    OrganizationFullData,
    customize_fixtures,
    local_device_to_backend_user,
    sequester_authority_factory,
)


# Sequester service modification is exposed as server API, so we only test the internals
@pytest.mark.trio
@customize_fixtures(backend_not_populated=True)
@customize_fixtures(coolorg_is_sequestered_organization=True)
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

    backend_alice, backend_alice_first_device = local_device_to_backend_user(
        alice, coolorg, timestamp=coolorg.sequester_authority.certif_data.timestamp
    )

    organization_bootstrap_args = {
        "bootstrap_token": org_token,
        "root_verify_key": coolorg.root_verify_key,
        "user_certificate": backend_alice.user_certificate,
        "device_certificate": backend_alice_first_device.device_certificate,
        "redacted_user_certificate": backend_alice.redacted_user_certificate,
        "redacted_device_certificate": backend_alice_first_device.redacted_device_certificate,
        "sequester_authority_certificate": coolorg.sequester_authority.certif,
    }

    # Bad authority certificate
    rep = await organization_bootstrap(
        anonymous_backend_ws,
        check_rep=False,
        **{**organization_bootstrap_args, "sequester_authority_certificate": b"dummy"},
    )
    assert isinstance(rep, OrganizationBootstrapRepInvalidData)

    # Authority certificate not signed by the root key
    bad_sequester_authority_certificate = coolorg.sequester_authority.certif_data.dump_and_sign(
        other_org.root_signing_key
    )
    rep = await organization_bootstrap(
        anonymous_backend_ws,
        check_rep=False,
        **{
            **organization_bootstrap_args,
            "sequester_authority_certificate": bad_sequester_authority_certificate,
        },
    )
    assert isinstance(rep, OrganizationBootstrapRepInvalidData)

    # Timestamp out of ballpark in authority certificate
    timestamp_out_of_ballpark = DateTime(2000, 1, 1)
    authority_certif_bad_timestamp = sequester_authority_factory(
        coolorg.root_signing_key, timestamp=timestamp_out_of_ballpark
    ).certif
    rep = await organization_bootstrap(
        anonymous_backend_ws,
        check_rep=False,
        **{
            **organization_bootstrap_args,
            "sequester_authority_certificate": authority_certif_bad_timestamp,
        },
    )
    assert isinstance(rep, OrganizationBootstrapRepBadTimestamp)
    assert rep.ballpark_client_early_offset == 300.0
    assert rep.ballpark_client_late_offset == 320.0

    # Timestamp in authority certificate different than user/device certificates
    different_timestamp = DateTime.now()
    authority_certif_different_timestamp = sequester_authority_factory(
        coolorg.root_signing_key, timestamp=different_timestamp
    ).certif
    rep = await organization_bootstrap(
        anonymous_backend_ws,
        check_rep=False,
        **{
            **organization_bootstrap_args,
            "sequester_authority_certificate": authority_certif_different_timestamp,
        },
    )
    assert isinstance(rep, OrganizationBootstrapRepInvalidData)

    # Finally valid bootstrap
    rep = await organization_bootstrap(
        anonymous_backend_ws, check_rep=False, **organization_bootstrap_args
    )
    assert isinstance(rep, OrganizationBootstrapRepOk)
