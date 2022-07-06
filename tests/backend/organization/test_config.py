# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest

from tests.backend.common import organization_config


@pytest.mark.trio
async def test_organization_config_ok(coolorg, alice_ws, backend):
    rep = await organization_config(alice_ws)
    assert rep == {
        "status": "ok",
        "user_profile_outsider_allowed": True,
        "active_users_limit": backend.config.organization_initial_active_users_limit,
    }

    await backend.organization.update(
        id=coolorg.organization_id, user_profile_outsider_allowed=False, active_users_limit=42
    )
    rep = await organization_config(alice_ws)
    assert rep == {"status": "ok", "user_profile_outsider_allowed": False, "active_users_limit": 42}
