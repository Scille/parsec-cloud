# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from typing import Literal
from uuid import uuid4

import pytest

from parsec._parsec import RealmRole, VlobID
from parsec.components.auth import EditicsToken
from parsec.editics_protocol import (
    EditicsClientEventAuth,
    EditicsClientEventAuthReconnect,
    EditicsClientEventClose,
    EditicsClientEventMessage,
    EditicsClientEventParsecFetchOriginalContent,
    EditicsClientEventParsecUploadOriginalContent,
    EditicsClientEventSaveChanges,
    EditicsClientEventUnLockDocument,
    EditicsParticipantID,
    EditicsServerEventAuth,
    EditicsServerEventConnectState,
    EditicsServerEventDisconnectReason,
    EditicsServerEventError,
    EditicsServerEventMessage,
    EditicsServerEventParsecFetchOriginalContent,
    EditicsServerEventParsecOriginalContentUploaded,
    EditicsServerEventParsecUploadOriginalContent,
    EditicsServerEventUnSaveLock,
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


def _auth_event(*, mode: Literal["view", "edit"]) -> EditicsClientEventAuth:
    return EditicsClientEventAuth(vlobVersion=1, reconnect=None, mode=mode)


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
        rep = await alice.send(_auth_event(mode="edit"))
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
                reconnect=EditicsClientEventAuthReconnect(
                    participantId=participant_id,
                    lastOtherSaveTime=-1,
                ),
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
        rep = await alice.send(_auth_event(mode="edit"))
        assert isinstance(rep, EditicsServerEventAuth)
        assert rep.indexUser == 2


async def test_participant_id_of_another_device_is_refused(
    coolorg: CoolorgRpcClients,
) -> None:
    # The participant ID is related to the device that created it: Bob cannot
    # connect with the participant ID of Alice (i.e. impersonation attempt ?)
    alice_participant_id = _new_participant_id()
    auth_event = _auth_event(mode="view")

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

    for expected_index_user, start_in_view_mode in enumerate((True, False), 1):
        initial_mode, reconnect_mode = ("view", "edit") if start_in_view_mode else ("edit", "view")
        participant_id = _new_participant_id()
        async with EditicsSessionClient.join(
            who=coolorg.alice,
            realm_id=coolorg.wksp1_id,
            document_id=DOCUMENT_ID,
            participant_id=participant_id,
        ) as alice:
            rep = await alice.send(_auth_event(mode=initial_mode))
            assert isinstance(rep, EditicsServerEventAuth)
            assert rep.indexUser == expected_index_user

        # Connection is lost...

        # ...reconnecting while asking to switch to the view mode is refused
        reconnect = EditicsClientEventAuthReconnect(
            participantId=participant_id,
            lastOtherSaveTime=-1,
        )
        async with EditicsSessionClient.join(
            who=coolorg.alice,
            realm_id=coolorg.wksp1_id,
            document_id=DOCUMENT_ID,
            participant_id=participant_id,
        ) as alice:
            with pytest.raises(RpcTransportError) as exc:
                await alice.send(
                    EditicsClientEventAuth(vlobVersion=1, reconnect=reconnect, mode=reconnect_mode)
                )
            assert exc.value.rep.status_code == 496  # TODO: dummy HTTP error code for now

            # The refused reconnection had no effect: Alice can reconnect, provided
            # she asks for her original mode
            rep = await alice.send(
                EditicsClientEventAuth(vlobVersion=1, reconnect=reconnect, mode=initial_mode)
            )
            assert isinstance(rep, EditicsServerEventAuth)
            assert rep.indexUser == expected_index_user


async def test_bad_auth(coolorg: CoolorgRpcClients) -> None:
    participant_id = _new_participant_id()

    send_url = f"{coolorg.alice.url}/editics/sessions/{coolorg.wksp1_id}/{DOCUMENT_ID.hex}/send/{participant_id}"
    listen_url = f"{coolorg.alice.url}/editics/sessions/{coolorg.wksp1_id}/{DOCUMENT_ID.hex}/listen/{participant_id}"

    # Send: missing `Authorization` header
    rep = await coolorg.alice.raw_client.post(
        send_url,
        content=_auth_event(mode="edit").model_dump_json(),
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
        content=_auth_event(mode="edit").model_dump_json(),
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
        rep = await bob.send(_auth_event(mode="view"))
        assert isinstance(rep, EditicsServerEventAuth)
        assert rep.indexUser == 1
        assert [p.view for p in rep.participants] == [True]

    async with EditicsSessionClient.join(
        who=coolorg.bob,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_id=_new_participant_id(),
    ) as bob:
        with pytest.raises(RpcTransportError) as exc:
            await bob.send(_auth_event(mode="edit"))
        assert exc.value.rep.status_code == 495  # TODO: dummy HTTP error code for now


async def test_write_realm_role_can_join_in_view_mode(
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
        rep = await alice.send(_auth_event(mode="view"))
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
            rep = await bob.send(_auth_event(mode="edit"))
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
        rep = await alice.send(_auth_event(mode="edit"))
        assert isinstance(rep, EditicsServerEventAuth)

        async with EditicsSessionClient.join(
            who=coolorg.bob,
            realm_id=coolorg.wksp1_id,
            document_id=DOCUMENT_ID,
            participant_id=bob_participant_id,
        ) as bob:
            # Bob joins while the auth lock is held: he is parked, i.e. his
            # handshake will complete when the lock will be released
            rep = await bob.send(_auth_event(mode="edit"))
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
        rep = await alice.send(_auth_event(mode="edit"))
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

    # Helper granting a role and making sure the corresponding realm certificate
    # has been dispatched by the event bus before going on: the bus dispatches
    # asynchronously, and a certificate landing while a participant has opened
    # its SSE connection but not yet sent its `auth` event kicks it (see the
    # `auth is None` cornercase in `EditicsComponent._on_event`).
    async def give_role_and_wait(new_role: RealmRole | None) -> None:
        with backend.event_bus.spy() as spy:
            await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, new_role)
            await spy.wait(EventRealmCertificate)

    # Any role change triggers the deco if the participant hasn't send the `auth`  event yet
    await assert_deco_on_role_change(coolorg.bob, new_role=None)
    await give_role_and_wait(RealmRole.CONTRIBUTOR)
    await assert_deco_on_role_change(coolorg.bob, new_role=RealmRole.OWNER)

    # Deco viewer losing read access
    await assert_deco_on_role_change(coolorg.bob, new_role=None, auth="view")
    await give_role_and_wait(RealmRole.CONTRIBUTOR)

    # Deco editor losing write access
    await assert_deco_on_role_change(coolorg.bob, new_role=RealmRole.READER, auth="edit")
    await give_role_and_wait(RealmRole.OWNER)
    await assert_deco_on_role_change(coolorg.bob, new_role=None, auth="edit")

    # No deco for viewer losing write access
    await give_role_and_wait(RealmRole.CONTRIBUTOR)
    await assert_deco_on_role_change(
        coolorg.bob, new_role=RealmRole.READER, auth="view", expected="no_deco"
    )

    # No deco for editor changing role but keeping write access
    await give_role_and_wait(RealmRole.MANAGER)
    await assert_deco_on_role_change(
        coolorg.bob, new_role=RealmRole.CONTRIBUTOR, auth="edit", expected="no_deco"
    )


# --- Original content (RFC 1030 §1.2) ---------------------------------------


def _upload_original_content_event(content: bytes) -> EditicsClientEventParsecUploadOriginalContent:
    # The original content is opaque to the server (it is only ever decrypted
    # client-side), so any bytes do for the tests.
    return EditicsClientEventParsecUploadOriginalContent(encryptedOriginalContent=(1, content))


def _save_changes_event(
    *, delete_index: int | None = None, changes: list[bytes] | None = None
) -> EditicsClientEventSaveChanges:
    return EditicsClientEventSaveChanges(
        encryptedChanges=[(1, change) for change in changes if change is not None]
        if changes is not None
        else [(1, b"dummy change")],
        startSaveChanges=True,
        endSaveChanges=True,
        deleteIndex=delete_index,
        excelAdditionalInfoIndexCols=None,
        excelAdditionalInfoIndexRows=None,
        encryptedCursorAdditionalInfo=None,
        releaseLocks=False,
    )


async def test_original_content_lifecycle(coolorg: CoolorgRpcClients, backend: Backend) -> None:
    alice_original_content = b"<Alice's original content>"

    # Bob is READER on wksp1: grant him a write role so he can join as an editor.
    # Note the event bus dispatches the realm certificate asynchronously: wait
    # for it before joining, since a certificate landing between a participant's
    # SSE registration and its `auth` would kick it.
    with backend.event_bus.spy() as spy:
        await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.CONTRIBUTOR)
        await spy.wait(EventRealmCertificate)

    # Alice is the first editor of the session: she must upload the original
    # content (her x2t conversion result).
    async with EditicsSessionClient.join(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_id=_new_participant_id(),
    ) as alice:
        rep = await alice.send(_auth_event(mode="edit"))
        assert isinstance(rep, EditicsServerEventAuth)
        assert rep.originalContentUploaded is False

        rep = await alice.send(_upload_original_content_event(alice_original_content))
        assert isinstance(rep, EditicsServerEventParsecUploadOriginalContent)
        assert rep.alreadyUploaded is False

        # Release the auth lock (i.e. switch to co-editing mode) so Bob's
        # handshake completes right away instead of being parked.
        rep = await alice.send(EditicsClientEventUnLockDocument(unlock=True, deleteIndex=None))
        assert rep is None

        # Bob joins the session: the original content is already there, so he
        # is expected to fetch it instead of converting the document himself.
        async with EditicsSessionClient.join(
            who=coolorg.bob,
            realm_id=coolorg.wksp1_id,
            document_id=DOCUMENT_ID,
            participant_id=_new_participant_id(),
        ) as bob:
            rep = await bob.send(_auth_event(mode="edit"))
            assert isinstance(rep, EditicsServerEventAuth)
            assert rep.originalContentUploaded is True

            rep = await bob.send(EditicsClientEventParsecFetchOriginalContent())
            assert isinstance(rep, EditicsServerEventParsecFetchOriginalContent)
            assert rep.encryptedOriginalContent == (1, alice_original_content)

            # First-upload-wins: a concurrent upload is refused, the loser must
            # use the existing content instead of its own conversion.
            rep = await bob.send(_upload_original_content_event(b"<Bob's original content>"))
            assert isinstance(rep, EditicsServerEventParsecUploadOriginalContent)
            assert rep.alreadyUploaded is True

            # The session's content is still Alice's one
            rep = await bob.send(EditicsClientEventParsecFetchOriginalContent())
            assert isinstance(rep, EditicsServerEventParsecFetchOriginalContent)
            assert rep.encryptedOriginalContent == (1, alice_original_content)


async def test_fetch_original_content_before_upload(coolorg: CoolorgRpcClients) -> None:
    # A fresh session has no original content yet
    async with EditicsSessionClient.join(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_id=_new_participant_id(),
    ) as alice:
        rep = await alice.send(_auth_event(mode="edit"))
        assert isinstance(rep, EditicsServerEventAuth)

        rep = await alice.send(EditicsClientEventParsecFetchOriginalContent())
        assert isinstance(rep, EditicsServerEventParsecFetchOriginalContent)
        assert rep.encryptedOriginalContent is None


async def test_upload_original_content_requires_auth(coolorg: CoolorgRpcClients) -> None:
    # The `parsecUploadOriginalContent` event, like any other event but `auth`,
    # cannot be the first one sent by a participant.
    async with EditicsSessionClient.join(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_id=_new_participant_id(),
    ) as alice:
        with pytest.raises(RpcTransportError) as exc:
            await alice.send(_upload_original_content_event(b"dummy"))
        assert exc.value.rep.status_code == 493  # TODO: dummy HTTP error code for now


async def test_viewer_cannot_upload_original_content(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    # Mallory is not part of wksp1: give her a read-only role so she can join
    # as a viewer. Only editors may upload the original content, and a viewer
    # trying to do so is kicked out of the session (like for the other
    # editor-only events, record 61).
    with backend.event_bus.spy() as spy:
        await wksp1_alice_gives_role(coolorg, backend, coolorg.mallory.user_id, RealmRole.READER)
        await spy.wait(EventRealmCertificate)

    async with EditicsSessionClient.join(
        who=coolorg.mallory,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_id=_new_participant_id(),
    ) as mallory:
        rep = await mallory.send(_auth_event(mode="view"))
        assert isinstance(rep, EditicsServerEventAuth)

        rep = await mallory.send(_upload_original_content_event(b"dummy"))
        assert isinstance(rep, EditicsServerEventDisconnectReason)
        assert rep.code == 4004

        # The kick also closed Mallory's SSE stream
        with pytest.raises(StopAsyncIteration):
            await mallory.recv()

    # The upload was refused, so nothing was stored (the session itself survives
    # the kick: Mallory stays in it as a disconnected participant, so she could
    # restore her connection)
    (session,) = backend.editics._sessions.values()
    assert session.encrypted_original_content is None


async def test_save_changes_discarded_before_original_content(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    # RFC 1030 §1.2: no document modification is accepted while the session has
    # no original content (client-side this cannot happen given an editor
    # uploads its conversion before loading the document into the editor).
    async with EditicsSessionClient.join(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_id=_new_participant_id(),
    ) as alice:
        rep = await alice.send(_auth_event(mode="edit"))
        assert isinstance(rep, EditicsServerEventAuth)

        # The `saveChanges` event is discarded...
        rep = await alice.send(_save_changes_event())
        assert rep is None

        # ...and nothing was stored
        (session,) = backend.editics._sessions.values()
        assert session.changes == []
        rep = await alice.send(EditicsClientEventParsecFetchOriginalContent())
        assert isinstance(rep, EditicsServerEventParsecFetchOriginalContent)
        assert rep.encryptedOriginalContent is None

        # Once the original content is uploaded, the changes are stored again
        rep = await alice.send(_upload_original_content_event(b"<Alice's original content>"))
        assert isinstance(rep, EditicsServerEventParsecUploadOriginalContent)
        assert rep.alreadyUploaded is False

        rep = await alice.send(_save_changes_event())
        assert isinstance(rep, EditicsServerEventUnSaveLock)
        assert rep.index == 0
        assert rep.syncChangesIndex == 1
        (session,) = backend.editics._sessions.values()
        assert len(session.changes) == 1


async def test_update_version_broadcast_to_viewers(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    # Bob is READER on wksp1: grant him a write role so he can join as an
    # editor. Mallory is not part of wksp1: give her a read-only role so she
    # can join as a viewer.
    with backend.event_bus.spy() as spy:
        await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.CONTRIBUTOR)
        await spy.wait(EventRealmCertificate)
        await wksp1_alice_gives_role(coolorg, backend, coolorg.mallory.user_id, RealmRole.READER)
        await spy.wait(EventRealmCertificate)

    # Alice joins first and takes the auth lock (single-editor mode)
    async with EditicsSessionClient.join(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_id=_new_participant_id(),
    ) as alice:
        rep = await alice.send(_auth_event(mode="edit"))
        assert isinstance(rep, EditicsServerEventAuth)

        # Release the auth lock so Mallory and Bob join without being parked
        rep = await alice.send(EditicsClientEventUnLockDocument(unlock=True, deleteIndex=None))
        assert rep is None

        async with EditicsSessionClient.join(
            who=coolorg.mallory,
            realm_id=coolorg.wksp1_id,
            document_id=DOCUMENT_ID,
            participant_id=_new_participant_id(),
        ) as mallory:
            rep = await mallory.send(_auth_event(mode="view"))
            assert isinstance(rep, EditicsServerEventAuth)

            async with EditicsSessionClient.join(
                who=coolorg.bob,
                realm_id=coolorg.wksp1_id,
                document_id=DOCUMENT_ID,
                participant_id=_new_participant_id(),
            ) as bob:
                rep = await bob.send(_auth_event(mode="edit"))
                assert isinstance(rep, EditicsServerEventAuth)

                # Alice uploads the session's original content
                rep = await alice.send(
                    _upload_original_content_event(b"<Alice's original content>")
                )
                assert isinstance(rep, EditicsServerEventParsecUploadOriginalContent)
                assert rep.alreadyUploaded is False

                # Mallory, a viewer having opened the document from her own
                # local conversion, is told to reload it with the session's
                # content (the other events on her stream are the `connectState`
                # broadcasts of the late joiners).
                while True:
                    event = await mallory.recv()
                    if isinstance(event, EditicsServerEventParsecOriginalContentUploaded):
                        assert event.success is True
                        break
                    assert isinstance(event, EditicsServerEventConnectState)

                # Bob (an editor: he would get `alreadyUploaded=True` if he
                # tried to upload his own conversion) and Alice (the uploader)
                # don't receive the `updateVersion` broadcast: their next
                # events are the regular ones (a chat message broadcast here).
                rep = await mallory.send(
                    EditicsClientEventMessage(encryptedMessage=(1, b"dummy message"))
                )
                assert isinstance(rep, EditicsServerEventMessage)

                event = await bob.recv()
                assert isinstance(event, EditicsServerEventMessage)
                event = await alice.recv()
                assert isinstance(event, EditicsServerEventConnectState)
                event = await alice.recv()
                assert isinstance(event, EditicsServerEventConnectState)
                event = await alice.recv()
                assert isinstance(event, EditicsServerEventMessage)


async def test_unlock_document_delete_index(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    disable_no_logs_gte_error_check: None,
) -> None:
    # RFC 1030 §2.2: `unLockDocument` carrying a `deleteIndex` asks the server
    # to truncate the stored change history above that index (the client drops
    # the matching history points on its side at the same time, so both stay
    # index-aligned). An index above the current change count is ignored.

    # Each part uses its own document (hence its own session)
    document_ids = [
        VlobID.from_hex("00000000000000000000000000000f01"),
        VlobID.from_hex("00000000000000000000000000000f02"),
        VlobID.from_hex("00000000000000000000000000000f03"),
    ]

    async def setup_changes(participant_id: EditicsParticipantID, document_id: VlobID) -> None:
        async with EditicsSessionClient.join(
            who=coolorg.alice,
            realm_id=coolorg.wksp1_id,
            document_id=document_id,
            participant_id=participant_id,
        ) as alice:
            rep = await alice.send(_auth_event(mode="edit"))
            assert isinstance(rep, EditicsServerEventAuth)
            # The invariant requires an original content before any change
            rep = await alice.send(_upload_original_content_event(b"<Alice's original content>"))
            assert isinstance(rep, EditicsServerEventParsecUploadOriginalContent)
            assert rep.alreadyUploaded is False
            for _ in range(3):
                rep = await alice.send(_save_changes_event())
                assert isinstance(rep, EditicsServerEventUnSaveLock)

    # 3 changes stored; truncate down to 1 with an unLockDocument{deleteIndex: 1}
    participant_id = _new_participant_id()
    await setup_changes(participant_id, document_ids[0])
    session = backend.editics._sessions[
        (coolorg.alice.organization_id, coolorg.wksp1_id, document_ids[0])
    ]
    assert len(session.changes) == 3
    async with EditicsSessionClient.join(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=document_ids[0],
        participant_id=participant_id,
    ) as alice:
        rep = await alice.send(EditicsClientEventUnLockDocument(unlock=True, deleteIndex=1))
        assert rep is None
    assert len(session.changes) == 1
    # The restore-gate token moved on the truncation
    assert session.last_other_save_time != -1

    # An out-of-range deleteIndex is ignored (and logged)
    participant_id = _new_participant_id()
    await setup_changes(participant_id, document_ids[1])
    session = backend.editics._sessions[
        (coolorg.alice.organization_id, coolorg.wksp1_id, document_ids[1])
    ]
    assert len(session.changes) == 3
    async with EditicsSessionClient.join(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=document_ids[1],
        participant_id=participant_id,
    ) as alice:
        rep = await alice.send(EditicsClientEventUnLockDocument(unlock=True, deleteIndex=42))
        assert rep is None
    assert len(session.changes) == 3

    # An index equal to the change count is a silent no-op (the records show
    # the client regularly sends it, e.g. at the end of a save cycle)
    async with EditicsSessionClient.join(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=document_ids[1],
        participant_id=participant_id,
    ) as alice:
        rep = await alice.send(EditicsClientEventUnLockDocument(unlock=True, deleteIndex=3))
        assert rep is None
    assert len(session.changes) == 3

    # The sentinel values (-1 / None) don't truncate
    async with EditicsSessionClient.join(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=document_ids[1],
        participant_id=participant_id,
    ) as alice:
        rep = await alice.send(EditicsClientEventUnLockDocument(unlock=True, deleteIndex=-1))
        assert rep is None
        rep = await alice.send(EditicsClientEventUnLockDocument(unlock=True, deleteIndex=None))
        assert rep is None
    assert len(session.changes) == 3


async def test_restore_gate_last_other_save_time(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    # The restore gate compares the session's strictly-monotonic change-history
    # token (`lastOtherSaveTime`) snapshotted at the participant's disconnection
    # against its current value: a restoration is refused whenever the history
    # mutated in the meantime.
    #
    # Note each sub-scenario uses its own document (and hence its own session),
    # so they don't have to care about each other's leftovers.

    # Bob is READER on wksp1: grant him a write role so he can join as an
    # editor (waiting for the realm certificate to be dispatched, see the
    # other tests for why).
    with backend.event_bus.spy() as spy:
        await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.CONTRIBUTOR)
        await spy.wait(EventRealmCertificate)

    async def alice_save_changes_and_disconnect(
        document_id: VlobID,
    ) -> tuple[EditicsParticipantID, int]:
        participant_id = _new_participant_id()
        async with EditicsSessionClient.join(
            who=coolorg.alice,
            realm_id=coolorg.wksp1_id,
            document_id=document_id,
            participant_id=participant_id,
        ) as alice:
            rep = await alice.send(_auth_event(mode="edit"))
            assert isinstance(rep, EditicsServerEventAuth)
            assert rep.lastOtherSaveTime == -1

            # The invariant requires an original content before any change
            rep = await alice.send(_upload_original_content_event(b"<Alice's original content>"))
            assert isinstance(rep, EditicsServerEventParsecUploadOriginalContent)
            assert rep.alreadyUploaded is False

            rep = await alice.send(_save_changes_event())
            assert isinstance(rep, EditicsServerEventUnSaveLock)
            # `time` carries the session's change-history token
            last_other_save_time = rep.time

        return (participant_id, last_other_save_time)

    async def alice_reconnects(
        document_id: VlobID, participant_id: EditicsParticipantID
    ) -> EditicsServerEventAuth | EditicsServerEventError:
        async with EditicsSessionClient.join(
            who=coolorg.alice,
            realm_id=coolorg.wksp1_id,
            document_id=document_id,
            participant_id=participant_id,
        ) as alice:
            return await alice.send(
                EditicsClientEventAuth(vlobVersion=1, reconnect=participant_id, mode="edit")
            )

    async def bob_saves_changes(document_id: VlobID, *, undo: bool = False) -> None:
        async with EditicsSessionClient.join(
            who=coolorg.bob,
            realm_id=coolorg.wksp1_id,
            document_id=document_id,
            participant_id=_new_participant_id(),
        ) as bob:
            rep = await bob.send(_auth_event(mode="edit"))
            assert isinstance(rep, EditicsServerEventAuth)

            rep = await bob.send(_save_changes_event())
            assert isinstance(rep, EditicsServerEventUnSaveLock)

            if undo:
                # Undo Bob's own change: the change-history *length* returns to
                # its value at Alice's disconnection, but the token only ever
                # moves forward, so the gate still detects the mutation (this
                # is the behavioral delta of the token-based gate, RFC §2.3).
                rep = await bob.send(
                    _save_changes_event(delete_index=rep.syncChangesIndex - 1, changes=[])
                )
                assert isinstance(rep, EditicsServerEventUnSaveLock)

    # 1) Nothing changed while Alice was away: the restoration is accepted,
    #    and the token is unchanged since her disconnection.
    document_id = VlobID.from_hex("00000000000000000000000000000e01")
    participant_id, last_other_save_time = await alice_save_changes_and_disconnect(document_id)
    rep = await alice_reconnects(document_id, participant_id)
    assert isinstance(rep, EditicsServerEventAuth)
    assert rep.lastOtherSaveTime == last_other_save_time

    # 2) Another participant's change while Alice was away: refused.
    document_id = VlobID.from_hex("00000000000000000000000000000e02")
    participant_id, last_other_save_time = await alice_save_changes_and_disconnect(document_id)
    await bob_saves_changes(document_id)
    rep = await alice_reconnects(document_id, participant_id)
    assert isinstance(rep, EditicsServerEventError)
    assert rep.code == 4010

    # 3) Same, but the change is followed by an undo truncating the history
    #    back to its initial length: the restore is refused all the same.
    document_id = VlobID.from_hex("00000000000000000000000000000e03")
    participant_id, last_other_save_time = await alice_save_changes_and_disconnect(document_id)
    await bob_saves_changes(document_id, undo=True)
    session = backend.editics._sessions[
        (coolorg.alice.organization_id, coolorg.wksp1_id, document_id)
    ]
    # The undo brought the history back to its length at Alice's disconnection
    assert len(session.changes) == 1
    # ...but the token moved on the truncation, hence the refusal
    assert session.last_other_save_time != last_other_save_time
    rep = await alice_reconnects(document_id, participant_id)
    assert isinstance(rep, EditicsServerEventError)
    assert rep.code == 4010
