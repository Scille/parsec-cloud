# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import DateTime, UserProfile, UserUpdateCertificate, authenticated_cmds
from parsec.events import EventUserUpdated
from tests.common import Backend, CoolorgRpcClients


async def test_authenticated_user_update_ok(coolorg: CoolorgRpcClients, backend: Backend) -> None:
    now = DateTime.now()
    certif = UserUpdateCertificate(
        author=coolorg.alice.device_id,
        timestamp=now,
        user_id=coolorg.bob.device_id.user_id,
        new_profile=UserProfile.ADMIN,
    )

    expected_dump = await backend.user.test_dump_current_users(coolorg.organization_id)
    expected_dump[coolorg.bob.device_id.user_id].current_profile = UserProfile.ADMIN

    with backend.event_bus.spy() as spy:
        rep = await coolorg.alice.user_update(
            user_update_certificate=certif.dump_and_sign(coolorg.alice.signing_key)
        )
        assert rep == authenticated_cmds.v4.user_update.RepOk()

        await spy.wait_event_occurred(
            EventUserUpdated(
                organization_id=coolorg.organization_id,
                user_id=coolorg.bob.device_id.user_id,
                new_profile=UserProfile.ADMIN,
            )
        )

    dump = await backend.user.test_dump_current_users(coolorg.organization_id)
    assert dump == expected_dump
