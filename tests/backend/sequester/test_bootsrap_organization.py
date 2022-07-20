# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
from unittest.mock import ANY
from pendulum import now as pendulum_now, datetime

from parsec.core.types import LocalDevice

from tests.common import (
    customize_fixtures,
    OrganizationFullData,
    local_device_to_backend_user,
    sequester_authority_factory,
)
from tests.backend.common import organization_bootstrap


# Sequester service modification is exposed as server API, so we only test the internals
# TODO: improve tests once asgi backend is available
@pytest.mark.trio
@customize_fixtures(backend_not_populated=True)
@customize_fixtures(coolorg_is_sequestered_organization=True)
async def test_sequestered_organization_bootstrap(
    coolorg: OrganizationFullData,
    otherorg: OrganizationFullData,
    alice: LocalDevice,
    anonymous_backend_ws,
    backend,
    running_backend,
):
    # Create organization
    org_token = "123456"
    await backend.organization.create(coolorg.organization_id, org_token)

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
        **{**organization_bootstrap_args, "sequester_authority_certificate": b"dummy"}
    )
    assert rep == {
        "status": "invalid_data",
        "reason": "Invalid signature for sequester authority certificate",
    }

    # Authority certificate not signed by the root key
    bad_sequester_authority_certificate = coolorg.sequester_authority.certif_data.dump_and_sign(
        otherorg.root_signing_key
    )
    rep = await organization_bootstrap(
        anonymous_backend_ws,
        check_rep=False,
        **{
            **organization_bootstrap_args,
            "sequester_authority_certificate": bad_sequester_authority_certificate,
        }
    )
    assert rep == {
        "status": "invalid_data",
        "reason": "Invalid signature for sequester authority certificate",
    }

    # Timestamp out of ballpark in authority certificate
    timestamp_out_of_ballpark = datetime(2000, 1, 1)
    authority_certif_bad_timestamp = sequester_authority_factory(
        coolorg.root_signing_key, timestamp=timestamp_out_of_ballpark
    ).certif
    rep = await organization_bootstrap(
        anonymous_backend_ws,
        check_rep=False,
        **{
            **organization_bootstrap_args,
            "sequester_authority_certificate": authority_certif_bad_timestamp,
        }
    )
    assert rep == {
        "status": "bad_timestamp",
        "client_timestamp": ANY,
        "backend_timestamp": ANY,
        "ballpark_client_early_offset": 50.0,
        "ballpark_client_late_offset": 70.0,
    }

    # Timestamp in authority certificate different than user/device certificates
    different_timestamp = pendulum_now()
    authority_certif_different_timestamp = sequester_authority_factory(
        coolorg.root_signing_key, timestamp=different_timestamp
    ).certif
    rep = await organization_bootstrap(
        anonymous_backend_ws,
        check_rep=False,
        **{
            **organization_bootstrap_args,
            "sequester_authority_certificate": authority_certif_different_timestamp,
        }
    )
    assert rep == {
        "status": "invalid_data",
        "reason": "Device, user and sequester authority certificates must have the same timestamp.",
    }

    # Finally valid bootstrap
    rep = await organization_bootstrap(
        anonymous_backend_ws, check_rep=False, **organization_bootstrap_args
    )
    assert rep == {"status": "ok"}
