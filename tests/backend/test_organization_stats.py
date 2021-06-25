# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
from uuid import uuid4
from unittest.mock import ANY
from pendulum import now as pendulum_now

from parsec.core.backend_connection import BackendConnectionError
from parsec.api.data import UserProfile
from parsec.core.logged_core import OrganizationStats
from tests.common import customize_fixtures


@pytest.mark.trio
@customize_fixtures(adam_profile=UserProfile.OUTSIDER)
async def test_organization_stats(
    running_backend, backend, realm, alice, alice_core, bob_core, otheralice_core
):
    organization_stats = await alice_core.get_organization_stats()
    assert organization_stats == OrganizationStats(
        users=3,
        active_users=3,
        users_per_profile_detail={
            "ADMIN": {"active": 1, "revoked": 0},
            "STANDARD": {"active": 1, "revoked": 0},
            "OUTSIDER": {"active": 1, "revoked": 0},
        },
        data_size=0,
        metadata_size=ANY,
    )
    initial_metadata_size = organization_stats.metadata_size

    # Create new metadata
    await backend.vlob.create(
        organization_id=alice.organization_id,
        author=alice.device_id,
        encryption_revision=1,
        timestamp=pendulum_now(),
        realm_id=realm,
        vlob_id=uuid4(),
        blob=b"1234",
    )
    organization_stats = await alice_core.get_organization_stats()
    assert organization_stats == OrganizationStats(
        users=3,
        active_users=3,
        users_per_profile_detail={
            "ADMIN": {"active": 1, "revoked": 0},
            "STANDARD": {"active": 1, "revoked": 0},
            "OUTSIDER": {"active": 1, "revoked": 0},
        },
        data_size=0,
        metadata_size=initial_metadata_size + 4,
    )

    # Create new data
    await backend.block.create(
        organization_id=alice.organization_id,
        author=alice.device_id,
        realm_id=realm,
        block_id=uuid4(),
        block=b"1234",
    )
    organization_stats = await alice_core.get_organization_stats()
    assert organization_stats == OrganizationStats(
        users=3,
        active_users=3,
        users_per_profile_detail={
            "ADMIN": {"active": 1, "revoked": 0},
            "STANDARD": {"active": 1, "revoked": 0},
            "OUTSIDER": {"active": 1, "revoked": 0},
        },
        data_size=4,
        metadata_size=initial_metadata_size + 4,
    )

    # Bob is not admin, it should fail
    with pytest.raises(BackendConnectionError) as exc:
        await bob_core.get_organization_stats()
    assert (
        str(exc.value)
        == "Backend error: {'reason': 'User `bob` is not admin', 'status': 'not_allowed'}"
    )

    # Ensure organization isolation
    other_organization_stats = await otheralice_core.get_organization_stats()
    assert other_organization_stats == OrganizationStats(
        users=1,
        active_users=1,
        users_per_profile_detail={
            "ADMIN": {"active": 1, "revoked": 0},
            "STANDARD": {"active": 0, "revoked": 0},
            "OUTSIDER": {"active": 0, "revoked": 0},
        },
        data_size=0,
        metadata_size=ANY,
    )
