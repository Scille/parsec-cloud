# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import functools
from typing import Protocol

import pytest

from parsec import events
from parsec._parsec import (
    ActiveUsersLimit,
    DateTime,
    DeviceID,
    EnrollmentID,
    InvitationStatus,
    InvitationToken,
    OrganizationID,
    UserID,
    VlobID,
    authenticated_cmds,
)
from parsec.components.memory.events import MemoryEventsComponent
from parsec.events import Event, EventPinged
from tests.common import Backend, CoolorgRpcClients, MinimalorgRpcClients
from tests.common.client import AuthenticatedRpcClient


class GenerateEvent(Protocol):
    def __call__(
        self, organization_id: OrganizationID, realm_id: VlobID | None = None
    ) -> Event: ...


INVITATION_TOKEN = InvitationToken.from_hex("f22a1230c2d6463d85b1b7575e601d9f")
ENROLLMENT_ID = EnrollmentID.from_hex("be6510e4-3e0b-4144-b3a7-fd5ad2c01fd8")
VLOB_ID = VlobID.from_hex("cc7dca19-447c-4aca-9c99-b8205655afee")
DEVICE_ID = DeviceID.new()
TIMESTAMP = DateTime.from_rfc3339("2024-03-17T08:42:30Z")
ALICE_USER_ID = UserID("alice")


@pytest.mark.parametrize(
    ["gen_event", "expected"],
    [
        # Note we don't test `EventOrganizationConfig` given it is a special case
        # (never actually dispatched, and instead always sent once as first SSE event).
        pytest.param(
            functools.partial(events.EventPinged, ping="ping"),
            authenticated_cmds.v4.events_listen.APIEventPinged(ping="ping"),
            id="ping",
        ),
        pytest.param(
            functools.partial(
                events.EventInvitation,
                token=INVITATION_TOKEN,
                greeter=ALICE_USER_ID,
                status=InvitationStatus.READY,
            ),
            authenticated_cmds.v4.events_listen.APIEventInvitation(
                token=INVITATION_TOKEN,
                invitation_status=InvitationStatus.READY,
            ),
            id="invitation",
        ),
        pytest.param(
            functools.partial(events.EventPkiEnrollment, enrollment_id=ENROLLMENT_ID),
            authenticated_cmds.v4.events_listen.APIEventPkiEnrollment(),
            id="pki_enrollment",
        ),
        pytest.param(
            functools.partial(events.EventCommonCertificate, timestamp=TIMESTAMP),
            authenticated_cmds.v4.events_listen.APIEventCommonCertificate(timestamp=TIMESTAMP),
            id="common_certificate",
        ),
        pytest.param(
            functools.partial(events.EventSequesterCertificate, timestamp=TIMESTAMP),
            authenticated_cmds.v4.events_listen.APIEventSequesterCertificate(timestamp=TIMESTAMP),
            id="sequester_certificate",
        ),
        pytest.param(
            functools.partial(
                events.EventShamirRecoveryCertificate,
                timestamp=TIMESTAMP,
                # Alice should be in the participants to receive the event.
                participants=(ALICE_USER_ID, UserID("bob"), UserID("carol")),
            ),
            authenticated_cmds.v4.events_listen.APIEventShamirRecoveryCertificate(
                timestamp=TIMESTAMP
            ),
            id="shamir_recovery_certificate",
        ),
        pytest.param(
            functools.partial(
                events.EventRealmCertificate,
                timestamp=TIMESTAMP,
                realm_id=VLOB_ID,
                user_id=ALICE_USER_ID,
                role_removed=False,
            ),
            authenticated_cmds.v4.events_listen.APIEventRealmCertificate(
                timestamp=TIMESTAMP, realm_id=VLOB_ID
            ),
            id="realm_certificate",
        ),
        pytest.param(
            functools.partial(
                events.EventVlob,
                author=DEVICE_ID,
                timestamp=TIMESTAMP,
                vlob_id=VLOB_ID,
                version=42,
                blob=b"foobar",
                last_common_certificate_timestamp=TIMESTAMP,
                last_realm_certificate_timestamp=TIMESTAMP,
            ),
            functools.partial(
                authenticated_cmds.v4.events_listen.APIEventVlob,
                vlob_id=VLOB_ID,
                author=DEVICE_ID,
                timestamp=TIMESTAMP,
                version=42,
                blob=b"foobar",
                last_common_certificate_timestamp=TIMESTAMP,
                last_realm_certificate_timestamp=TIMESTAMP,
            ),
            id="vlob",
        ),
    ],
)
async def test_ok(
    gen_event: GenerateEvent,
    expected: authenticated_cmds.v4.events_listen.APIEvent,
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> None:
    async with coolorg.alice.events_listen() as alice_sse:
        # First event is always ServiceConfig
        event = await alice_sse.next_event()
        assert event == authenticated_cmds.v4.events_listen.RepOk(
            authenticated_cmds.v4.events_listen.APIEventServerConfig(
                active_users_limit=ActiveUsersLimit.NO_LIMIT,
                user_profile_outsider_allowed=True,
            )
        )

        # Actual event to test

        # Vlob event must reference an existing realm ID, this is a hack to patch accordingly
        if callable(expected):
            expected = expected(realm_id=coolorg.wksp1_id)
            event = gen_event(organization_id=coolorg.organization_id, realm_id=coolorg.wksp1_id)
        else:
            event = gen_event(organization_id=coolorg.organization_id)

        await backend.event_bus.send(event)
        event = await alice_sse.next_event()
        assert event == authenticated_cmds.v4.events_listen.RepOk(expected)


async def test_receive_server_config_as_first_event(
    minimalorg: MinimalorgRpcClients, backend: Backend
) -> None:
    async with minimalorg.alice.events_listen() as alice_sse:
        event = await alice_sse.next_event()
        assert event == authenticated_cmds.v4.events_listen.RepOk(
            authenticated_cmds.v4.events_listen.APIEventServerConfig(
                active_users_limit=ActiveUsersLimit.NO_LIMIT,
                user_profile_outsider_allowed=True,
            )
        )


async def test_user_not_receive_event_before_listen(
    minimalorg: MinimalorgRpcClients,
    backend: Backend,
) -> None:
    # Use `_dispatch_incoming_event` instead of `send` to ensure the event
    # has been handle by the callbacks once the function returns.
    backend.event_bus._dispatch_incoming_event(
        events.EventPinged(
            organization_id=minimalorg.organization_id,
            ping="event1",
        )
    )
    async with minimalorg.alice.events_listen() as alice_sse:
        backend.event_bus._dispatch_incoming_event(
            events.EventPinged(
                organization_id=minimalorg.organization_id,
                ping="event2",
            )
        )

        # First event is always ServiceConfig
        event = await alice_sse.next_event()
        assert event == authenticated_cmds.v4.events_listen.RepOk(
            authenticated_cmds.v4.events_listen.APIEventServerConfig(
                active_users_limit=ActiveUsersLimit.NO_LIMIT,
                user_profile_outsider_allowed=True,
            )
        )

        event = await alice_sse.next_event()
        assert event == authenticated_cmds.v4.events_listen.RepOk(
            authenticated_cmds.v4.events_listen.APIEventPinged(ping="event2")
        )


async def test_http_error_404_not_found(backend: Backend, coolorg: CoolorgRpcClients) -> None:
    import httpx_sse

    mod_alice = AuthenticatedRpcClient(
        coolorg.alice.raw_client,
        OrganizationID("foobar"),
        coolorg.alice.device_id,
        coolorg.alice.signing_key,
        coolorg.alice.event,
    )

    with pytest.raises(httpx_sse.SSEError) as exception:
        async with mod_alice.events_listen() as mod_alice_sse:
            await mod_alice_sse.next_event()

    assert exception.traceback.pop().frame.f_locals["self"].response.status_code == 404


async def test_conn_closed_on_bad_outcome(
    backend: Backend, minimalorg: MinimalorgRpcClients
) -> None:
    async with minimalorg.alice.events_listen() as alice_sse:
        # First event is always ServiceConfig
        event = await alice_sse.next_event()
        assert event == authenticated_cmds.v4.events_listen.RepOk(
            authenticated_cmds.v4.events_listen.APIEventServerConfig(
                active_users_limit=ActiveUsersLimit.NO_LIMIT,
                user_profile_outsider_allowed=True,
            )
        )

    memory_events = backend.events
    assert isinstance(memory_events, MemoryEventsComponent)
    del memory_events._data.organizations[minimalorg.organization_id]

    with pytest.raises(StopAsyncIteration):
        async with minimalorg.alice.events_listen() as alice_sse:
            backend.event_bus._dispatch_incoming_event(
                events.EventPinged(
                    organization_id=minimalorg.organization_id,
                    ping="event2",
                )
            )

            assert await alice_sse.next_event(), "The connection should have been closed"


async def test_non_sse_request(minimalorg: MinimalorgRpcClients) -> None:
    alice = minimalorg.alice
    alice_client = alice.raw_client
    res = await alice_client.get(
        f"{alice.url}/events",
        headers={
            **alice.headers,
            "Accept": "application/json",
        },
    )

    assert res.status_code == 406
    assert res.json() == {"detail": "Bad accept type"}


async def test_receive_event_of_newly_shared_realm(
    minimalorg: MinimalorgRpcClients,
    backend: Backend,
) -> None:
    async with minimalorg.alice.events_listen() as alice_sse:
        # First event is always ServiceConfig
        event = await alice_sse.next_event()
        assert event == authenticated_cmds.v4.events_listen.RepOk(
            authenticated_cmds.v4.events_listen.APIEventServerConfig(
                active_users_limit=ActiveUsersLimit.NO_LIMIT,
                user_profile_outsider_allowed=True,
            )
        )

        async def send_vlob_event(org: OrganizationID, version: int):
            await backend.event_bus.send(
                events.EventVlob(
                    organization_id=org,
                    author=minimalorg.alice.device_id,
                    realm_id=VLOB_ID,
                    timestamp=TIMESTAMP,
                    vlob_id=VLOB_ID,
                    version=version,
                    blob=b"ok" if org == minimalorg.organization_id else b"wrong organization !",
                    last_common_certificate_timestamp=TIMESTAMP,
                    last_realm_certificate_timestamp=TIMESTAMP,
                )
            )

        # Trigger events in two organization to ensure they don't get mixed
        orgs = (OrganizationID("DummyOrg"), minimalorg.organization_id)
        for org in orgs:
            # 1) Share for other user, should be ignored

            await backend.event_bus.send(
                events.EventRealmCertificate(
                    organization_id=org,
                    timestamp=TIMESTAMP,
                    realm_id=VLOB_ID,
                    user_id=UserID("DummyUser"),
                    role_removed=False,
                )
            )
            await send_vlob_event(org, 1)

            # 2) Share for Alice, now events should be received

            await backend.event_bus.send(
                events.EventRealmCertificate(
                    organization_id=org,
                    timestamp=TIMESTAMP,
                    realm_id=VLOB_ID,
                    user_id=minimalorg.alice.user_id,
                    role_removed=False,
                )
            )
            await send_vlob_event(org, 2)

            # 3) Unshare for other user, events should still be received

            await backend.event_bus.send(
                events.EventRealmCertificate(
                    organization_id=org,
                    timestamp=TIMESTAMP,
                    realm_id=VLOB_ID,
                    user_id=UserID("DummyUser"),
                    role_removed=True,
                )
            )
            await send_vlob_event(org, 3)

            # 4) Unshare for Alice, now events should no longer be received

            await backend.event_bus.send(
                events.EventRealmCertificate(
                    organization_id=org,
                    timestamp=TIMESTAMP,
                    realm_id=VLOB_ID,
                    user_id=minimalorg.alice.user_id,
                    role_removed=True,
                )
            )
            await send_vlob_event(org, 4)

            # 5) Event always received for Alice, once we have received this event
            # we know the previous one has been ignored as expected

            await backend.event_bus.send(
                events.EventCommonCertificate(
                    organization_id=org,
                    timestamp=TIMESTAMP,
                )
            )

        async def receive_realm_certificate_event():
            event = await alice_sse.next_event()
            assert event == authenticated_cmds.v4.events_listen.RepOk(
                authenticated_cmds.v4.events_listen.APIEventRealmCertificate(
                    realm_id=VLOB_ID,
                    timestamp=TIMESTAMP,
                )
            )

        async def receive_vlob_event(version):
            event = await alice_sse.next_event()
            assert event == authenticated_cmds.v4.events_listen.RepOk(
                authenticated_cmds.v4.events_listen.APIEventVlob(
                    realm_id=VLOB_ID,
                    author=minimalorg.alice.device_id,
                    timestamp=TIMESTAMP,
                    vlob_id=VLOB_ID,
                    version=version,
                    blob=b"ok",
                    last_common_certificate_timestamp=TIMESTAMP,
                    last_realm_certificate_timestamp=TIMESTAMP,
                )
            )

        await receive_realm_certificate_event()
        await receive_vlob_event(2)

        await receive_realm_certificate_event()
        await receive_vlob_event(3)

        await receive_realm_certificate_event()

        event = await alice_sse.next_event()
        assert event == authenticated_cmds.v4.events_listen.RepOk(
            authenticated_cmds.v4.events_listen.APIEventCommonCertificate(
                timestamp=TIMESTAMP,
            )
        )


# TODO: test `Last-Event-ID`

async def test_last_event_id(minimalorg: MinimalorgRpcClients, backend: Backend) -> None:
    async with minimalorg.alice.events_listen() as alice_sse:
        raw_source = alice_sse._iter_events  # pyright: ignore[reportPrivateUsage]

        # 1. Start by sending 4 ping event
        for ping in ("event1", "event2", "event3", "event4"):
            backend.event_bus._dispatch_incoming_event(  # pyright: ignore[reportPrivateUsage]
                EventPinged(
                    organization_id=minimalorg.organization_id,
                    ping=ping,
                )
            )

        # First event is always ServiceConfig
        event = await alice_sse.next_event()
        assert event == authenticated_cmds.v4.events_listen.RepOk(
            authenticated_cmds.v4.events_listen.APIEventServerConfig(
                active_users_limit=ActiveUsersLimit.NO_LIMIT,
                user_profile_outsider_allowed=True,
            )
        )

        event = await alice_sse.next_event()
        assert event == authenticated_cmds.v4.events_listen.RepOk(
            authenticated_cmds.v4.events_listen.APIEventPinged(ping="event1")
        )

        event2_id = (await anext(raw_source)).id

        for ping in ("event3", "event4"):
            event = await alice_sse.next_event()
            assert event == authenticated_cmds.v4.events_listen.RepOk(
                authenticated_cmds.v4.events_listen.APIEventPinged(ping=ping)
            )

    # 3. Now ask for events *after* the second EventPinged
    async with minimalorg.alice.events_listen(last_event_id=event2_id) as alice_sse:
        # First event is always ServiceConfig
        event = await alice_sse.next_event()
        assert event == authenticated_cmds.v4.events_listen.RepOk(
            authenticated_cmds.v4.events_listen.APIEventServerConfig(
                active_users_limit=ActiveUsersLimit.NO_LIMIT,
                user_profile_outsider_allowed=True,
            )
        )

        # 4. Only the last two EventPinged should be obtained
        for ping in ("event3", "event4"):
            event = await alice_sse.next_event()
            assert event == authenticated_cmds.v4.events_listen.RepOk(
                authenticated_cmds.v4.events_listen.APIEventPinged(ping=ping)
            )
