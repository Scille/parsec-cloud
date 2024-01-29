# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import authenticated_cmds
from parsec._parsec import testbed as tb
from parsec.events import EventCommonCertificate
from tests.common import AuthenticatedRpcClient, MinimalorgRpcClients, TestbedBackend


async def test_authenticated_device_create_ok(
    minimalorg: MinimalorgRpcClients,
    testbed: TestbedBackend,
    ballpark_always_ok: None,
) -> None:
    # Simpler to get Alice dev2's certificate from an existing template than to
    # generate it from scratch
    _, _, template = await testbed.get_template("coolorg")
    for event in template.events:
        if isinstance(event, tb.TestbedEventNewDevice) and event.device_id.str == "alice@dev2":
            break
    else:
        assert False

    with testbed.backend.event_bus.spy() as spy:
        rep = await minimalorg.alice.device_create(
            device_certificate=event.raw_certificate,
            redacted_device_certificate=event.raw_redacted_certificate,
        )
        assert rep == authenticated_cmds.v4.device_create.RepOk()

        await spy.wait_event_occurred(
            EventCommonCertificate(
                organization_id=minimalorg.organization_id,
                timestamp=event.certificate.timestamp,
            )
        )

    # Now alice dev2 can connect
    alice2_rpc = AuthenticatedRpcClient(
        raw_client=minimalorg.raw_client,
        organization_id=minimalorg.organization_id,
        device_id=event.device_id,
        signing_key=event.signing_key,
    )
    rep = await alice2_rpc.ping(ping="hello")
    assert rep == authenticated_cmds.v4.ping.RepOk(pong="hello")
