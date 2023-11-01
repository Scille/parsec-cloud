# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import UserProfile, authenticated_cmds
from parsec._parsec import testbed as tb
from parsec.components.user import UserDump
from parsec.events import EventCommonCertificate
from tests.common import AuthenticatedRpcClient, Backend, MinimalorgRpcClients, TestbedBackend


async def test_authenticated_user_create_ok(
    minimalorg: MinimalorgRpcClients,
    testbed: TestbedBackend,
    backend: Backend,
    ballpark_always_ok: None,
) -> None:
    # Simpler to get Bob's certificate from an existing template than to generate it from scratch
    _, _, template = await testbed.get_template("coolorg")
    for event in template.events:
        if isinstance(event, tb.TestbedEventNewUser) and event.device_id.user_id.str == "bob":
            break
    else:
        assert False

    expected_dump = await testbed.backend.user.test_dump_current_users(minimalorg.organization_id)
    expected_dump[event.device_id.user_id] = UserDump(
        user_id=event.device_id.user_id,
        devices=[event.device_id.device_name],
        current_profile=UserProfile.STANDARD,
        is_revoked=False,
    )

    with backend.event_bus.spy() as spy:
        rep = await minimalorg.alice.user_create(
            user_certificate=event.user_raw_certificate,
            device_certificate=event.first_device_raw_certificate,
            redacted_device_certificate=event.first_device_raw_redacted_certificate,
            redacted_user_certificate=event.user_raw_redacted_certificate,
        )
        assert rep == authenticated_cmds.v4.user_create.RepOk()

        await spy.wait_event_occurred(
            EventCommonCertificate(
                organization_id=minimalorg.organization_id,
                timestamp=event.timestamp,
            )
        )

    dump = await testbed.backend.user.test_dump_current_users(minimalorg.organization_id)
    assert dump == expected_dump

    # Now Bob can connect
    bob_rpc = AuthenticatedRpcClient(
        raw_client=minimalorg.raw_client,
        organization_id=minimalorg.organization_id,
        device_id=event.device_id,
        signing_key=event.first_device_signing_key,
    )
    rep = await bob_rpc.ping(ping="hello")
    assert rep == authenticated_cmds.v4.ping.RepOk(pong="hello")
