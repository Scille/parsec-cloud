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
from parsec.events import Event
from tests.common import Backend, CoolorgRpcClients, MinimalorgRpcClients


class GenerateEvent(Protocol):
    def __call__(self, organization_id: OrganizationID) -> Event: ...


INVITATION_TOKEN = InvitationToken.from_hex("f22a1230-c2d6-463d-85b1-b7575e601d9f")
ENROLLMENT_ID = EnrollmentID.from_hex("be6510e4-3e0b-4144-b3a7-fd5ad2c01fd8")
VLOB_ID = VlobID.from_hex("cc7dca19-447c-4aca-9c99-b8205655afee")
DEVICE_ID = DeviceID.new()
TIMESTAMP = DateTime.from_rfc3339("2024-03-17T08:42:30Z")
ALICE_USER_ID = UserID("alice")


@pytest.mark.parametrize(
    ["gen_event", "expected"],
    [
        pytest.param(
            functools.partial(events.EventPinged, ping="ping"),
            authenticated_cmds.v4.events_listen.APIEventPinged(ping="ping"),
            id="ping",
        ),
        # FIXME: Did not finish https://github.com/Scille/parsec-cloud/issues/6847
        # pytest.param(
        #     functools.partial(
        #         events.EventOrganizationConfig,
        #         user_profile_outsider_allowed=False,
        #         active_users_limit=ActiveUsersLimit.limited_to(42),
        #     ),
        #     authenticated_cmds.v4.events_listen.APIEventServerConfig(
        #         active_users_limit=ActiveUsersLimit.limited_to(42),
        #         user_profile_outsider_allowed=False,
        #     ),
        #     id="server_config",
        # ),
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
        ),
        pytest.param(
            functools.partial(events.EventSequesterCertificate, timestamp=TIMESTAMP),
            authenticated_cmds.v4.events_listen.APIEventSequesterCertificate(timestamp=TIMESTAMP),
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
        ),
        # FIXME: Did not finish https://github.com/Scille/parsec-cloud/issues/6846
        # pytest.param(
        #     functools.partial(
        #         events.EventVlob,
        #         author=DEVICE_ID,
        #         realm_id=VLOB_ID,
        #         timestamp=TIMESTAMP,
        #         vlob_id=VLOB_ID,
        #         version=42,
        #         blob=b"foobar",
        #         last_common_certificate_timestamp=TIMESTAMP,
        #         last_realm_certificate_timestamp=TIMESTAMP,
        #     ),
        #     authenticated_cmds.v4.events_listen.APIEventVlob(
        #         realm_id=VLOB_ID,
        #         vlob_id=VLOB_ID,
        #         author=DEVICE_ID,
        #         timestamp=TIMESTAMP,
        #         version=42,
        #         blob=b"foobar",
        #         last_common_certificate_timestamp=TIMESTAMP,
        #         last_realm_certificate_timestamp=TIMESTAMP,
        #     ),
        # ),
    ],
)
async def test_ok(
    gen_event: GenerateEvent,
    expected: authenticated_cmds.v4.events_listen.APIEvent,
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> None:
    await backend.event_bus.send(gen_event(organization_id=coolorg.organization_id))
    async with coolorg.alice.events_listen() as alice_sse:
        # First event is always ServiceConfig
        event = await alice_sse.next_event()
        assert event == authenticated_cmds.v4.events_listen.RepOk(
            authenticated_cmds.v4.events_listen.APIEventServerConfig(
                active_users_limit=ActiveUsersLimit.NO_LIMIT,
                user_profile_outsider_allowed=True,
            )
        )

        # skip coolorg event queue.
        assert isinstance(
            await alice_sse.next_event(),
            authenticated_cmds.v4.events_listen.RepOk,
        )

        event = await alice_sse.next_event()
        assert event == authenticated_cmds.v4.events_listen.RepOk(expected)


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


# TODO: test keepalive
# TODO: test `Last-Event-ID`
# TODO: test connection gets closed due to SseAPiEventsListenBadOutcome
