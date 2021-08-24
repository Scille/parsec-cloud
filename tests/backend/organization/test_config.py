# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest

from parsec.api.protocol import organization_config_serializer


async def organization_config(sock):
    raw_rep = await sock.send(
        organization_config_serializer.req_dumps({"cmd": "organization_config"})
    )
    raw_rep = await sock.recv()
    return organization_config_serializer.rep_loads(raw_rep)


@pytest.mark.trio
async def test_organization_config_ok(coolorg, alice_backend_sock, backend):
    rep = await organization_config(alice_backend_sock)
    assert rep == {
        "status": "ok",
        "user_profile_outsider_allowed": True,
        "active_users_limit": backend.config.organization_initial_active_users_limit,
    }

    await backend.organization.update(
        id=coolorg.organization_id, user_profile_outsider_allowed=False, active_users_limit=42
    )
    rep = await organization_config(alice_backend_sock)
    assert rep == {"status": "ok", "user_profile_outsider_allowed": False, "active_users_limit": 42}
