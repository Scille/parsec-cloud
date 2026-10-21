# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from typing import Literal
from uuid import uuid4

import pytest

from parsec._parsec import RealmRole, VlobID
from parsec.components.auth import EditicsToken
from parsec.editics_protocol import (
    EditicsClientEventAuth,
    EditicsClientEventClose,
    EditicsParticipantID,
    EditicsServerEventAuth,
    EditicsServerEventDisconnectReason,
    EditicsServerEventWaitAuth,
    EditicsServerEventWarning,
)
from parsec.events import EventRealmCertificate
from tests.common import (
    AuthenticatedRpcClient,
    Backend,
    CoolorgRpcClients,
    EditicsSessionClient,
    RpcTransportError,
    wksp1_alice_gives_role,
)

# Any document ID would do here: the editics session is created when the first
# participant joins, and the server doesn't check that the document vlob
# actually exists.
DOCUMENT_ID = VlobID.from_hex("00000000000000000000000000000d0c")


def _new_participant_id() -> EditicsParticipantID:
    return uuid4().hex


def _auth_event(*, view: bool = False) -> EditicsClientEventAuth:
    return EditicsClientEventAuth(vlobVersion=1, reconnect=None, mode="view" if view else "edit")


async def test_deco_reco(coolorg: CoolorgRpcClients) -> None:
    participant_id = _new_participant_id()

    # Good initial auth

    async with EditicsSessionClient.join(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_id=participant_id,
    ) as alice:
        # The send route still authenticates through the `Authorization` header
        rep = await alice.send(_auth_event())
        assert isinstance(rep, EditicsServerEventAuth)
        assert rep.indexUser == 1

    # Connection is lost...

    # ...reconnect with the same participant ID (so server should return the same `indexUser`)
    async with EditicsSessionClient.join(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_id=participant_id,
    ) as alice:
        rep = await alice.send(
            EditicsClientEventAuth(
                vlobVersion=1,
                reconnect=participant_id,
                mode="edit",
            )
        )
        assert isinstance(rep, EditicsServerEventAuth)
        assert rep.indexUser == 1

    # Connection is lost again...

    # ...reconnect with a different participant ID
    new_participant_id = _new_participant_id()
    async with EditicsSessionClient.join(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_id=new_participant_id,
    ) as alice:
        rep = await alice.send(_auth_event())
        assert isinstance(rep, EditicsServerEventAuth)
        assert rep.indexUser == 2


async def test_participant_id_of_another_device_is_refused(
    coolorg: CoolorgRpcClients,
) -> None:
    # The participant ID is related to the device that created it: Bob cannot
    # connect with the participant ID of Alice (i.e. impersonation attempt ?)
    alice_participant_id = _new_participant_id()
    auth_event = _auth_event(view=True)

    listen_url, send_url, authorization = EditicsSessionClient.build_urls_and_authorization(
        who=coolorg.bob,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_id=alice_participant_id,
    )

    async def assert_bob_refused():
        # Send

        rep = await coolorg.bob.raw_client.post(
            send_url,
            headers={"Authorization": authorization, "Content-Type": "application/json"},
            content=auth_event.model_dump_json(),
        )
        assert rep.status_code == 494  # TODO: dummy HTTP error code for now

        # Listen

        rep = await coolorg.bob.raw_client.get(
            listen_url,
            headers={"Accept": "text/event-stream"},
            params={"authorization": authorization},
        )
        assert rep.status_code == 494  # TODO: dummy HTTP error code for now

    async with EditicsSessionClient.join(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_id=alice_participant_id,
    ) as alice:
        # Participant created on the server but not `auth` yet

        await assert_bob_refused()

        rep = await alice.send(auth_event)
        assert isinstance(rep, EditicsServerEventAuth)

        # `auth` done

        await assert_bob_refused()

    # SSE connection is down

    await assert_bob_refused()

    # Alice's session is left unaffected by Bob's attempt
    rep = await alice.send(EditicsClientEventClose())
    assert rep is None

    # Note that after this `close` event, the server has forgotten the participant ID
    # and hence Bob could re-use it (since it is no longer linked to Alice in any way).


async def test_reconnection_cannot_change_participant_mode(
    coolorg: CoolorgRpcClients,
) -> None:
    # A participant's mode is fixed when it joins the session: a reconnecting
    # client cannot switch mode (changing mode is only possible by creating a
    # brand new participant), and a refused reconnection leaves the
    # participant untouched so it can still reconnect with its original mode.

    # An editor cannot reconnect as a viewer
    participant_id = _new_participant_id()
    async with EditicsSessionClient.join(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_id=participant_id,
    ) as alice:
        rep = await alice.send(_auth_event())
        assert isinstance(rep, EditicsServerEventAuth)
        assert rep.indexUser == 1

    # Connection is lost...

    # ...reconnecting while asking to switch to the view mode is refused
    async with EditicsSessionClient.join(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_id=participant_id,
    ) as alice:
        with pytest.raises(RpcTransportError) as exc:
            await alice.send(
                EditicsClientEventAuth(vlobVersion=1, reconnect=participant_id, mode="view")
            )
        assert exc.value.rep.status_code == 496  # TODO: dummy HTTP error code for now

        # The refused reconnection had no effect: Alice can reconnect, provided
        # she asks for her original mode
        rep = await alice.send(
            EditicsClientEventAuth(vlobVersion=1, reconnect=participant_id, mode="edit")
        )
        assert isinstance(rep, EditicsServerEventAuth)
        assert rep.indexUser == 1

    # A viewer cannot reconnect as an editor
    participant_id = _new_participant_id()
    async with EditicsSessionClient.join(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_id=participant_id,
    ) as alice:
        rep = await alice.send(_auth_event(view=True))
        assert isinstance(rep, EditicsServerEventAuth)

    # Connection is lost...

    # ...reconnecting while asking to switch to the edit mode is refused
    async with EditicsSessionClient.join(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_id=participant_id,
    ) as alice:
        with pytest.raises(RpcTransportError) as exc:
            await alice.send(
                EditicsClientEventAuth(vlobVersion=1, reconnect=participant_id, mode="edit")
            )
        assert exc.value.rep.status_code == 496  # TODO: dummy HTTP error code for now

        # The refused reconnection had no effect: Alice can reconnect with her
        # original mode
        rep = await alice.send(
            EditicsClientEventAuth(vlobVersion=1, reconnect=participant_id, mode="view")
        )
        assert isinstance(rep, EditicsServerEventAuth)


async def test_bad_auth(coolorg: CoolorgRpcClients) -> None:
    participant_id = _new_participant_id()

    send_url = f"{coolorg.alice.url}/editics/sessions/{coolorg.wksp1_id}/{DOCUMENT_ID.hex}/send/{participant_id}"
    listen_url = f"{coolorg.alice.url}/editics/sessions/{coolorg.wksp1_id}/{DOCUMENT_ID.hex}/listen/{participant_id}"

    # Send: missing `Authorization` header
    rep = await coolorg.alice.raw_client.post(
        send_url,
        content=_auth_event().model_dump_json(),
    )
    assert rep.status_code == 401

    # Listen: missing `authorization` parameter (SSE connection don't use `Authorization` header)
    rep = await coolorg.alice.raw_client.get(
        listen_url,
        headers={
            "Accept": "text/event-stream",
            "Cache-Control": "no-store",
        },
    )
    assert rep.status_code == 401

    raw_bad_token = EditicsToken.generate_raw(
        device_id=coolorg.alice.device_id,
        timestamp=coolorg.alice.now_factory(),
        key=coolorg.bob.signing_key,
    )

    # Send: invalid authorization token
    rep = await coolorg.alice.raw_client.post(
        send_url,
        headers={"Authorization": f"Bearer {raw_bad_token.decode('ascii')}"},
        content=_auth_event().model_dump_json(),
    )
    assert rep.status_code == 403

    # Listen: invalid authorization token
    rep = await coolorg.alice.raw_client.get(
        listen_url,
        headers={
            "Accept": "text/event-stream",
            "Cache-Control": "no-store",
        },
        params={"authorization": raw_bad_token},
    )
    assert rep.status_code == 401


async def test_read_only_realm_role_must_join_in_view_mode(
    coolorg: CoolorgRpcClients,
) -> None:
    # Bob is READER on wksp1: he joins as a viewer when requesting the view
    # mode, and the edit mode is refused (a viewer never co-edits).
    async with EditicsSessionClient.join(
        who=coolorg.bob,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_id=_new_participant_id(),
    ) as bob:
        rep = await bob.send(_auth_event(view=True))
        assert isinstance(rep, EditicsServerEventAuth)
        assert rep.indexUser == 1
        assert [p.view for p in rep.participants] == [True]

    async with EditicsSessionClient.join(
        who=coolorg.bob,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_id=_new_participant_id(),
    ) as bob:
        # TODO: the route should answer with a proper HTTP error status instead
        # of letting the `NotImplementedError` placeholder propagate
        with pytest.raises(RpcTransportError) as exc:
            await bob.send(_auth_event(view=False))
        assert exc.value.rep.status_code == 495  # TODO: dummy HTTP error code for now


async def test_write_role_participant_requesting_view_mode(
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> None:
    # A write-capable participant can join as a (live) viewer, e.g. for a
    # preview: the client-requested mode is honored, and such a viewer never
    # takes the auth lock.
    async with EditicsSessionClient.join(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_id=_new_participant_id(),
    ) as alice:
        rep = await alice.send(_auth_event(view=True))
        assert isinstance(rep, EditicsServerEventAuth)
        assert [p.view for p in rep.participants] == [True]

        # Bob joins as a write-capable editor (he is READER in the coolorg
        # fixture, so grant him a write role first)
        await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.CONTRIBUTOR)
        async with EditicsSessionClient.join(
            who=coolorg.bob,
            realm_id=coolorg.wksp1_id,
            document_id=DOCUMENT_ID,
            participant_id=_new_participant_id(),
        ) as bob:
            # Alice the viewer holds no auth lock: Bob joins without waiting
            rep = await bob.send(_auth_event(view=False))
            assert isinstance(rep, EditicsServerEventAuth)
            assert [p.view for p in rep.participants] == [True, False]


async def test_full_event_channel_closes_the_sse_connection(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    monkeypatch.setattr("parsec.components.editics.PER_PARTICIPANT_MAX_BUFFER_EVENTS", 0)

    # Bob is READER on wksp1: grant him a write role so he can join as an editor
    await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.CONTRIBUTOR)

    bob_participant_id = _new_participant_id()

    # Alice joins first, and hence becomes the single editor holding the auth lock
    async with EditicsSessionClient.join(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_id=_new_participant_id(),
    ) as alice:
        rep = await alice.send(_auth_event())
        assert isinstance(rep, EditicsServerEventAuth)

        async with EditicsSessionClient.join(
            who=coolorg.bob,
            realm_id=coolorg.wksp1_id,
            document_id=DOCUMENT_ID,
            participant_id=bob_participant_id,
        ) as bob:
            # Bob joins while the auth lock is held: he is parked, i.e. his
            # handshake completes when the lock gets released
            rep = await bob.send(_auth_event())
            assert isinstance(rep, EditicsServerEventWaitAuth)

            # Alice's leave releases the auth lock, completing Bob's handshake:
            # his `auth` reply is pushed on his SSE channel, then the new
            # participants list is broadcasted (`connectState`). Those two
            # events are pushed while handling Alice's request, without any
            # suspension point in between for Bob's SSE connection to consume
            # the first one, so the second push overflows the channel: the SSE
            # connection is then closed.
            rep = await alice.send(EditicsClientEventClose())
            assert rep is None

            # Bob first receives the events already delivered to his channel
            # (only his `auth` reply: the `connectState` broadcast is the event
            # that overflowed the channel), then the SSE connection is closed.
            rep = await bob.recv()
            assert isinstance(rep, EditicsServerEventAuth)
            with pytest.raises(StopAsyncIteration):
                await bob.recv_raw()

    # The SSE connection teardown handled Bob as disconnected: he stays in the
    # session (hidden from the participants lists) so he can restore it
    (session,) = backend.editics._sessions.values()
    bob_participant = session.participants[bob_participant_id]
    assert not bob_participant.connected
    assert bob_participant.disconnected_changes_index == 0


async def test_close_by_last_participant_destroys_session(
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> None:
    assert backend.editics._sessions == {}

    async with EditicsSessionClient.join(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_id=_new_participant_id(),
    ) as alice:
        rep = await alice.send(_auth_event())
        assert isinstance(rep, EditicsServerEventAuth)
        assert len(backend.editics._sessions) == 1

        # `close` is a voluntary leave: nothing is expected in reply
        rep = await alice.send(EditicsClientEventClose())
        assert rep is None

        # The session only lives through its participants
        assert backend.editics._sessions == {}


async def test_sse_deco_on_role_change(backend: Backend, coolorg: CoolorgRpcClients) -> None:

    def send_event_warning(participant_id: str, message: str) -> EditicsServerEventWarning:
        assert len(backend.editics._sessions) == 1
        participant = next(iter(backend.editics._sessions.values())).participants[participant_id]
        assert participant.sse_channel_sender is not None
        event = EditicsServerEventWarning(code=0, message=message)
        participant.sse_channel_sender.send_nowait(event)
        return event

    async def assert_deco_on_role_change(
        who: AuthenticatedRpcClient,
        new_role: RealmRole | None,
        auth: Literal["view", "edit"] | None = None,
        expected: Literal["deco", "no_deco"] = "deco",
    ):
        participant_id = _new_participant_id()
        async with EditicsSessionClient.join(
            who=who,
            realm_id=coolorg.wksp1_id,
            document_id=DOCUMENT_ID,
            participant_id=participant_id,
        ) as bob:
            if auth:
                await bob.send(EditicsClientEventAuth(vlobVersion=1, reconnect=None, mode=auth))

            expected_before_event = send_event_warning(participant_id, "before")
            with backend.event_bus.spy() as spy:
                await wksp1_alice_gives_role(coolorg, backend, who.user_id, new_role)
                # Wait for the event that told the editics system about the revocation
                await spy.wait(EventRealmCertificate)

            # Bob still receive the events sent before the revocation...
            event = await bob.recv()
            assert event == expected_before_event

            if expected == "deco":
                # ...then a notification that he was kicked out...
                event = await bob.recv()
                assert event == EditicsServerEventDisconnectReason(
                    code=4004, description="access deny"
                )

                # ...and then the connection is closed
                with pytest.raises(StopAsyncIteration):
                    event = await bob.recv()

                # Also cannot send request with this participant
                with pytest.raises(RpcTransportError) as exc:
                    await bob.send(EditicsClientEventClose())
                assert exc.value.rep.status_code in (
                    492,
                    493,
                )  # TODO: dummy HTTP error code for now

            else:
                # Bob is not kicked out: the connection stays functional
                expected_after_event = send_event_warning(participant_id, "after")
                event = await bob.recv()
                assert event == expected_after_event

    # Any role change triggers the deco if the participant hasn't send the `auth`  event yet
    await assert_deco_on_role_change(coolorg.bob, new_role=None)
    await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.CONTRIBUTOR)
    await assert_deco_on_role_change(coolorg.bob, new_role=RealmRole.OWNER)

    # Deco viewer losing read access
    await assert_deco_on_role_change(coolorg.bob, new_role=None, auth="view")
    await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.CONTRIBUTOR)

    # Deco editor losing write access
    await assert_deco_on_role_change(coolorg.bob, new_role=RealmRole.READER, auth="edit")
    await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.OWNER)
    await assert_deco_on_role_change(coolorg.bob, new_role=None, auth="edit")

    # No deco for viewer losing write access
    await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.CONTRIBUTOR)
    await assert_deco_on_role_change(
        coolorg.bob, new_role=RealmRole.READER, auth="view", expected="no_deco"
    )

    # No deco for editor changing role but keeping write access
    await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.MANAGER)
    await assert_deco_on_role_change(
        coolorg.bob, new_role=RealmRole.CONTRIBUTOR, auth="edit", expected="no_deco"
    )
