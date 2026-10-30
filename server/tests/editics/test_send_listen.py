# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from typing import Literal
from uuid import uuid4

import httpx_sse
import pytest

from parsec._parsec import RealmRole, VlobID
from parsec.asgi.editics import EditicsHttpStatus
from parsec.components.auth import EditicsToken
from parsec.components.user import UserInfo
from parsec.editics_protocol import (
    EDITICS_API_VERSION,
    EditicsClientEventAuth,
    EditicsClientEventAuthReconnect,
    EditicsClientEventClose,
    EditicsClientEventSaveChanges,
    EditicsClientEventUnLockDocument,
    EditicsServerEventAuth,
    EditicsServerEventDisconnectReason,
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
    alice_gives_profile,
    wksp1_alice_gives_role,
)


def _auth_event(*, mode: Literal["view", "edit"]) -> EditicsClientEventAuth:
    return EditicsClientEventAuth(vlobVersion=1, reconnect=None, mode=mode)


# --- Transport-level handshake (API version & HTTP statuses) ------------------


@pytest.mark.parametrize("bad_api_version", [None, "not a version", "2.0", "1"])
async def test_unsupported_api_version(
    alice_editics: EditicsSessionClient, bad_api_version: str | None
) -> None:
    headers = dict(alice_editics.base_headers)
    if bad_api_version is None:
        del headers["Api-Version"]
    else:
        headers["Api-Version"] = bad_api_version

    # Send

    rep = await alice_editics.raw_client.post(
        f"{alice_editics.base_url}/send",
        params=alice_editics.base_params,
        headers={**headers, "Content-Type": "application/json"},
        content=_auth_event(mode="edit").model_dump_json(),
    )
    assert rep.status_code == EditicsHttpStatus.UnsupportedApiVersion.value
    assert rep.headers["Supported-Api-Versions"] == str(EDITICS_API_VERSION)
    # No version could be settled, hence no `Api-Version` reply header
    assert "Api-Version" not in rep.headers

    # Listen

    rep = await alice_editics.raw_client.get(
        f"{alice_editics.base_url}/listen",
        params=alice_editics.base_params,
        # Note the `Accept` header is required, otherwise the listen route
        # would reply with a 406 before even looking at the API version
        headers={**headers, "Accept": "text/event-stream"},
    )
    assert rep.status_code == EditicsHttpStatus.UnsupportedApiVersion.value
    assert rep.headers["Supported-Api-Versions"] == str(EDITICS_API_VERSION)
    assert "Api-Version" not in rep.headers


async def test_compatible_api_version(alice_editics: EditicsSessionClient) -> None:
    # Only the major version breaks compatibility, so a client using a more
    # recent revision is accepted: the server replies with its own version.

    # Send

    rep = await alice_editics.raw_client.post(
        f"{alice_editics.base_url}/send",
        params=alice_editics.base_params,
        headers={
            **alice_editics.base_headers,
            "Content-Type": "application/json",
            "Api-Version": "1.42",
        },
        content=_auth_event(mode="view").model_dump_json(),
    )
    assert rep.status_code == 200
    assert rep.headers["Api-Version"] == str(EDITICS_API_VERSION)

    # Listen

    async with httpx_sse.aconnect_sse(
        alice_editics.raw_client,
        "GET",
        f"{alice_editics.base_url}/listen",
        params=alice_editics.base_params,
        headers={**alice_editics.base_headers, "Api-Version": "1.42"},
    ) as sse_event_source:
        # A refused request would be answered with a non-SSE error response
        assert sse_event_source.response.status_code == 200
        assert sse_event_source.response.headers["Content-Type"].startswith("text/event-stream")


async def test_bad_auth(coolorg: CoolorgRpcClients, alice_editics: EditicsSessionClient) -> None:
    raw_bad_token = EditicsToken.generate_raw(
        device_id=coolorg.alice.device_id,
        timestamp=coolorg.alice.now_factory(),
        key=coolorg.bob.signing_key,
    )

    for bad_authorization_value, expected_status_code in [
        # Missing authorization header
        (None, EditicsHttpStatus.MissingAuthenticationInfo),
        # Malformed authorization header value
        ("Dummy", EditicsHttpStatus.MissingAuthenticationInfo),
        # Malformed authorization token
        ("Bearer bad", EditicsHttpStatus.MissingAuthenticationInfo),
        # Invalid authorization token
        (f"Bearer {raw_bad_token.decode('ascii')}", EditicsHttpStatus.BadAuthenticationInfo),
    ]:
        base_headers = alice_editics.base_headers.copy()
        if bad_authorization_value is None:
            del base_headers["Authorization"]
        else:
            base_headers["Authorization"] = bad_authorization_value

        # Send
        rep = await alice_editics.raw_client.post(
            f"{alice_editics.base_url}/send",
            params=alice_editics.base_params,
            headers={
                **base_headers,
                "Content-Type": "application/json",
            },
            content=_auth_event(mode="edit").model_dump_json(),
        )
        assert rep.status_code == expected_status_code.value

        # Listen
        rep = await alice_editics.raw_client.get(
            f"{alice_editics.base_url}/listen",
            params=alice_editics.base_params,
            headers={
                **base_headers,
                "Accept": "text/event-stream",
            },
        )
        assert rep.status_code == expected_status_code.value


async def test_user_revoked_frozen(
    backend: Backend, coolorg: CoolorgRpcClients, bob_editics: EditicsSessionClient
) -> None:
    async def _assert_requests(expected_status_code: EditicsHttpStatus) -> None:
        # Send

        rep = await bob_editics.raw_client.post(
            f"{bob_editics.base_url}/send",
            params=bob_editics.base_params,
            headers={
                **bob_editics.base_headers,
                "Content-Type": "application/json",
            },
            content=_auth_event(mode="edit").model_dump_json(),
        )
        assert rep.status_code == expected_status_code.value

        # Listen

        rep = await bob_editics.raw_client.get(
            f"{bob_editics.base_url}/listen",
            params=bob_editics.base_params,
            headers={
                **bob_editics.base_headers,
                "Accept": "text/event-stream",
            },
        )
        assert rep.status_code == expected_status_code.value

    # Frozen user

    outcome = await backend.user.freeze_user(
        organization_id=coolorg.organization_id,
        user_id=coolorg.bob.user_id,
        user_email=None,
        frozen=True,
    )
    assert isinstance(outcome, UserInfo)

    await _assert_requests(EditicsHttpStatus.UserFrozen)

    # Revoked user

    await alice_gives_profile(coolorg, backend, recipient=coolorg.bob.user_id, new_profile=None)

    await _assert_requests(EditicsHttpStatus.UserRevoked)


async def test_realm_not_found_is_indistinguishable_from_realm_without_role(
    coolorg: CoolorgRpcClients,
) -> None:
    outcomes = []
    for realm_id in (VlobID.new(), coolorg.wksp1_id):
        mallory_editics = EditicsSessionClient(
            who=coolorg.mallory,
            realm_id=realm_id,
            document_id=coolorg.wksp1_id,
        )

        # Send

        rep = await mallory_editics.raw_client.post(
            f"{mallory_editics.base_url}/send",
            params=mallory_editics.base_params,
            headers={**mallory_editics.base_headers, "Content-Type": "application/json"},
            content=_auth_event(mode="view").model_dump_json(),
        )
        outcomes.append(("send", rep.status_code, rep.json()))

        # Listen

        rep = await mallory_editics.raw_client.get(
            f"{mallory_editics.base_url}/listen",
            params=mallory_editics.base_params,
            headers={**mallory_editics.base_headers, "Accept": "text/event-stream"},
        )
        outcomes.append(("listen", rep.status_code, rep.json()))

    (send_nonexistent, listen_nonexistent, send_without_role, listen_without_role) = outcomes

    # Information hiding: an organization member must not learn whether a realm
    # it has no access to exists or not
    assert send_nonexistent == send_without_role
    assert listen_nonexistent == listen_without_role
    assert send_nonexistent[1] == EditicsHttpStatus.UserNoRealmRole.value
    assert listen_nonexistent[1] == EditicsHttpStatus.UserNoRealmRole.value


async def test_bad_content_type_and_accept_headers(alice_editics: EditicsSessionClient) -> None:
    # Send (Content-Type)

    rep = await alice_editics.raw_client.post(
        f"{alice_editics.base_url}/send",
        params=alice_editics.base_params,
        headers={**alice_editics.base_headers, "Content-Type": "text/plain"},
        content=_auth_event(mode="edit").model_dump_json(),
    )
    assert rep.status_code == EditicsHttpStatus.BadContentTypeOrInvalidBody.value
    assert rep.headers["Api-Version"] == str(EDITICS_API_VERSION)
    assert rep.json()["detail"] == {"detail": "Bad content type or invalid body"}

    # Listen (Accept)

    rep = await alice_editics.raw_client.get(
        f"{alice_editics.base_url}/listen",
        params=alice_editics.base_params,
        headers={**alice_editics.base_headers, "Accept": "application/json"},
    )
    assert rep.status_code == EditicsHttpStatus.BadAcceptType.value
    assert rep.headers["Api-Version"] == str(EDITICS_API_VERSION)
    assert rep.json()["detail"] == {"detail": "Bad accept type"}


# --- Session & participants lifecycle ----------------------------------------


async def test_deco_reco(alice_editics: EditicsSessionClient) -> None:
    participant_session_id_1 = "e562e5fd21794d2b8e485d527d28b900"
    participant_session_id_2 = "b146ee2c144c485e924f6082c4ee66bf"

    alice_editics.base_params["psid"] = participant_session_id_1

    # Good initial auth

    async with alice_editics.listen():
        # The send route still authenticates through the `Authorization` header
        rep = await alice_editics.send(_auth_event(mode="edit"))
        assert isinstance(rep, EditicsServerEventAuth)
        assert rep.participantIndexUser == 1

    # Connection is lost...

    # ...reconnect with the same participant ID (so server should return the same `indexUser`)
    async with alice_editics.listen():
        rep = await alice_editics.send(
            EditicsClientEventAuth(
                vlobVersion=1,
                reconnect=EditicsClientEventAuthReconnect(
                    participantSessionId=participant_session_id_1,
                    lastOtherSaveTime=None,
                ),
                mode="edit",
            )
        )
        assert isinstance(rep, EditicsServerEventAuth)
        assert rep.participantIndexUser == 1

    # Connection is lost again...

    # ...reconnect with a different participant ID

    alice_editics.base_params["psid"] = participant_session_id_2

    async with alice_editics.listen():
        rep = await alice_editics.send(_auth_event(mode="edit"))
        assert isinstance(rep, EditicsServerEventAuth)
        assert rep.participantIndexUser == 2


async def test_participant_session_id_of_another_device_is_refused(
    alice_editics: EditicsSessionClient,
    bob_editics: EditicsSessionClient,
) -> None:
    # The participant ID is related to the device that created it: Bob cannot
    # connect with the participant ID of Alice (i.e. impersonation attempt ?)
    bob_editics.base_params["psid"] = alice_editics.participant_session_id

    auth_event = _auth_event(mode="view")

    # Bob's client, trying to use Alice's participant ID

    async def assert_bob_refused():
        # Send

        rep = await bob_editics.raw_client.post(
            f"{bob_editics.base_url}/send",
            params=bob_editics.base_params,
            headers={**bob_editics.base_headers, "Content-Type": "application/json"},
            content=auth_event.model_dump_json(),
        )
        assert rep.status_code == EditicsHttpStatus.ParticipantDeviceMismatch.value

        # Listen

        rep = await bob_editics.raw_client.get(
            f"{bob_editics.base_url}/listen",
            params=bob_editics.base_params,
            headers={**bob_editics.base_headers, "Accept": "text/event-stream"},
        )
        assert rep.status_code == EditicsHttpStatus.ParticipantDeviceMismatch.value

    async with alice_editics.listen():
        # ParticipantSession created on the server but not `auth` yet

        await assert_bob_refused()

        rep = await alice_editics.send(auth_event)
        assert isinstance(rep, EditicsServerEventAuth)

        # `auth` done

        await assert_bob_refused()

    # SSE connection is down

    await assert_bob_refused()

    # Alice's session is left unaffected by Bob's attempt
    rep = await alice_editics.send(EditicsClientEventClose())
    assert rep is None

    # Note that after this `close` event, the server has forgotten the participant ID
    # and hence Bob could re-use it (since it is no longer linked to Alice in any way).


async def test_reconnection_cannot_change_participant_mode(
    alice_editics: EditicsSessionClient,
) -> None:
    # A participant's mode is fixed when it joins the session: a reconnecting
    # client cannot switch mode (changing mode is only possible by creating a
    # brand new participant), and a refused reconnection leaves the
    # participant untouched so it can still reconnect with its original mode.

    for start_in_view_mode in (True, False):
        initial_mode, reconnect_mode = ("view", "edit") if start_in_view_mode else ("edit", "view")
        async with alice_editics.listen():
            # Note for the 2nd loop step we change the mode while keeping the same
            # participant ID, which is allowed
            rep = await alice_editics.send(_auth_event(mode=initial_mode))
            assert isinstance(rep, EditicsServerEventAuth)
            assert rep.participantIndexUser == 1

        # Connection is lost...

        # ...reconnecting while asking to switch to the view mode is refused
        reconnect = EditicsClientEventAuthReconnect(
            participantSessionId=alice_editics.participant_session_id,
            lastOtherSaveTime=None,
        )
        async with alice_editics.listen():
            with pytest.raises(RpcTransportError) as exc:
                await alice_editics.send(
                    EditicsClientEventAuth(vlobVersion=1, reconnect=reconnect, mode=reconnect_mode)
                )
            assert exc.value.rep.status_code == EditicsHttpStatus.ParticipantModeMismatch.value

            # The refused reconnection had no effect: Alice can reconnect, provided
            # she asks for her original mode
            rep = await alice_editics.send(
                EditicsClientEventAuth(vlobVersion=1, reconnect=reconnect, mode=initial_mode)
            )
            assert isinstance(rep, EditicsServerEventAuth)
            assert rep.participantIndexUser == 1


async def test_read_only_realm_role_must_join_in_view_mode(
    bob_editics: EditicsSessionClient,
) -> None:
    async with bob_editics.listen():
        rep = await bob_editics.send(_auth_event(mode="view"))
        assert isinstance(rep, EditicsServerEventAuth)
        assert rep.participantIndexUser == 1
        assert [p.view for p in rep.participants] == [True]

        with pytest.raises(RpcTransportError) as exc:
            await bob_editics.send(_auth_event(mode="edit"))
        assert exc.value.rep.status_code == EditicsHttpStatus.ReaderMustBeViewer.value


async def test_write_realm_role_can_join_in_view_mode(
    backend: Backend,
    coolorg: CoolorgRpcClients,
    alice_editics: EditicsSessionClient,
    bob_editics: EditicsSessionClient,
) -> None:
    async with alice_editics.listen():
        rep = await alice_editics.send(_auth_event(mode="view"))
        assert isinstance(rep, EditicsServerEventAuth)
        assert [p.view for p in rep.participants] == [True]

        # Bob joins as a write-capable editor
        await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.CONTRIBUTOR)
        async with bob_editics.listen():
            # Alice the viewer holds no auth lock: Bob joins without waiting
            rep = await bob_editics.send(_auth_event(mode="edit"))
            assert isinstance(rep, EditicsServerEventAuth)
            assert [p.view for p in rep.participants] == [True, False]


async def test_full_event_channel_closes_the_sse_connection(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    alice_editics: EditicsSessionClient,
    bob_editics: EditicsSessionClient,
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    monkeypatch.setattr("parsec.components.editics.PER_PARTICIPANT_MAX_BUFFER_EVENTS", 0)

    await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.CONTRIBUTOR)

    # Alice joins first, and hence becomes the single editor holding the auth lock
    async with alice_editics.listen():
        rep = await alice_editics.send(_auth_event(mode="edit"))
        assert isinstance(rep, EditicsServerEventAuth)

        async with bob_editics.listen():
            # Bob joins while the auth lock is held: he is parked, i.e. his
            # handshake will complete when the lock will be released
            rep = await bob_editics.send(_auth_event(mode="edit"))
            assert isinstance(rep, EditicsServerEventWaitAuth)

            # Alice's leave releases the auth lock, completing Bob's handshake:
            # his `auth` reply is pushed on his SSE channel, then the new
            # participants list is broadcasted (`connectState`). Those two
            # events are pushed while handling Alice's request, without any
            # suspension point in between for Bob's SSE connection to consume
            # the first one, so the second push overflows the channel: the SSE
            # connection is then closed.
            rep = await alice_editics.send(EditicsClientEventClose())
            assert rep is None

            # Bob first receives the events already delivered to his channel
            # (only his `auth` reply: the `connectState` broadcast is the event
            # that overflowed the channel), then the SSE connection is closed.
            rep = await bob_editics.recv()
            assert isinstance(rep, EditicsServerEventAuth)
            with pytest.raises(StopAsyncIteration):
                await bob_editics.recv_raw()

    # The SSE connection teardown handled Bob as disconnected: he stays in the
    # session (hidden from the participants lists) so he can restore it
    (session,) = backend.editics._document_sessions.values()
    bob_participant = session.participants[bob_editics.participant_session_id]
    assert not bob_participant.connected


async def test_close_by_last_participant_destroys_session(
    alice_editics: EditicsSessionClient,
    backend: Backend,
) -> None:
    assert backend.editics._document_sessions == {}
    async with alice_editics.listen():
        rep = await alice_editics.send(_auth_event(mode="edit"))
        assert isinstance(rep, EditicsServerEventAuth)
        assert len(backend.editics._document_sessions) == 1

        # `close` is a voluntary leave: nothing is expected in reply
        rep = await alice_editics.send(EditicsClientEventClose())
        assert rep is None

        # The session only lives through its participants
        assert backend.editics._document_sessions == {}


async def test_sse_deco_on_role_change(
    backend: Backend,
    coolorg: CoolorgRpcClients,
    bob_editics: EditicsSessionClient,
) -> None:

    def send_event_warning(participant_session_id: str, message: str) -> EditicsServerEventWarning:
        assert len(backend.editics._document_sessions) == 1
        participant = next(iter(backend.editics._document_sessions.values())).participants[
            participant_session_id
        ]
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
        participant_session_id = uuid4().hex
        bob_editics.base_params["psid"] = participant_session_id
        async with bob_editics.listen():
            if auth:
                await bob_editics.send(
                    EditicsClientEventAuth(vlobVersion=1, reconnect=None, mode=auth)
                )

            expected_before_event = send_event_warning(participant_session_id, "before")
            with backend.event_bus.spy() as spy:
                await wksp1_alice_gives_role(coolorg, backend, who.user_id, new_role)
                # Wait for the event that told the editics system about the revocation
                await spy.wait(EventRealmCertificate)

            # Bob still receive the events sent before the revocation...
            event = await bob_editics.recv()
            assert event == expected_before_event

            if expected == "deco":
                # ...then a notification that he was kicked out...
                event = await bob_editics.recv()
                assert event == EditicsServerEventDisconnectReason(
                    code=4004, description="access deny"
                )

                # ...and then the connection is closed
                with pytest.raises(StopAsyncIteration):
                    event = await bob_editics.recv()

                # Also cannot send request with this participant: the role change
                # either removed Bob's realm access outright (the request's
                # authentication is then refused) or kicked him out (which reset
                # his participant's authentication, so any event but `auth` is
                # refused)
                with pytest.raises(RpcTransportError) as exc:
                    await bob_editics.send(EditicsClientEventClose())
                expected_status = (
                    EditicsHttpStatus.UserNoRealmRole
                    if new_role is None
                    else EditicsHttpStatus.ParticipantNotAuth
                )
                assert exc.value.rep.status_code == expected_status.value

            else:
                # Bob is not kicked out: the connection stays functional
                expected_after_event = send_event_warning(participant_session_id, "after")
                event = await bob_editics.recv()
                assert event == expected_after_event

    async def give_role(new_role: RealmRole | None) -> None:
        # Wait for server events to settle:: the event bus dispatches asynchronously,
        # and a certificate landing while a participant has opened its SSE connection
        # but not yet sent its `auth` event kicks it.
        await wksp1_alice_gives_role(
            coolorg, backend, coolorg.bob.user_id, new_role, wait_events=True
        )

    # Any role change triggers the deco if the participant hasn't send the `auth`  event yet
    await assert_deco_on_role_change(coolorg.bob, new_role=None)
    await give_role(RealmRole.CONTRIBUTOR)
    await assert_deco_on_role_change(coolorg.bob, new_role=RealmRole.OWNER)

    # Deco viewer losing read access
    await assert_deco_on_role_change(coolorg.bob, new_role=None, auth="view")
    await give_role(RealmRole.CONTRIBUTOR)

    # Deco editor losing write access
    await assert_deco_on_role_change(coolorg.bob, new_role=RealmRole.READER, auth="edit")
    await give_role(RealmRole.OWNER)
    await assert_deco_on_role_change(coolorg.bob, new_role=None, auth="edit")

    # No deco for viewer losing write access
    await give_role(RealmRole.CONTRIBUTOR)
    await assert_deco_on_role_change(
        coolorg.bob, new_role=RealmRole.READER, auth="view", expected="no_deco"
    )

    # No deco for editor changing role but keeping write access
    await give_role(RealmRole.MANAGER)
    await assert_deco_on_role_change(
        coolorg.bob, new_role=RealmRole.CONTRIBUTOR, auth="edit", expected="no_deco"
    )


async def test_delete_index_defensive_handling(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    alice_editics: EditicsSessionClient,
) -> None:
    def _save_changes_event(
        *, delete_index: int | None = None, changes: list[tuple[int, bytes]] | None = None
    ) -> EditicsClientEventSaveChanges:
        return EditicsClientEventSaveChanges(
            encryptedChanges=[] if changes is None else changes,
            deleteIndex=delete_index,
            startSaveChanges=True,
            endSaveChanges=True,
            excelAdditionalInfoIndexCols=None,
            excelAdditionalInfoIndexRows=None,
            encryptedCursorAdditionalInfo=None,
            releaseLocks=False,
        )

    async with alice_editics.listen():
        session = backend.editics._document_sessions[
            (coolorg.alice.organization_id, coolorg.wksp1_id, alice_editics.document_id)
        ]

        rep = await alice_editics.send(_auth_event(mode="edit"))
        assert isinstance(rep, EditicsServerEventAuth)

        # The invariant requires an initial content before any change

        await alice_editics.upload_initial_content(
            key_index=1, parts=[b"<Alice's initial content>"]
        )
        rep = await alice_editics.send(_save_changes_event(changes=[(1, b"<change1>")]))
        assert isinstance(rep, EditicsServerEventUnSaveLock)
        rep = await alice_editics.send(
            _save_changes_event(changes=[(1, b"<change2>"), (1, b"<change3>")])
        )
        assert isinstance(rep, EditicsServerEventUnSaveLock)
        assert len(session.changes) == 3

        # `deleteIndex` can be passed either in `saveChanges` or `unLockDocument`,
        # we test both here

        # An out-of-range `deleteIndex` is accepted but is a noop

        rep = await alice_editics.send(_save_changes_event(delete_index=42))
        assert isinstance(rep, EditicsServerEventUnSaveLock)
        assert len(session.changes) == 3

        rep = await alice_editics.send(
            EditicsClientEventUnLockDocument(unlock=True, deleteIndex=42)
        )
        assert rep is None
        assert len(session.changes) == 3

        # Negative value causes a validation error

        with pytest.raises(RpcTransportError) as exc:
            await alice_editics.send_event_raw(
                '{"type":"saveChanges",'
                '"encryptedChanges":[],'
                '"startSaveChanges":true,'
                '"endSaveChanges":true,'
                '"deleteIndex":-1,'  # Here!
                '"excelAdditionalInfoIndexCols":null,'
                '"excelAdditionalInfoIndexRows":null,'
                '"encryptedCursorAdditionalInfo":null,'
                '"releaseLocks":false}'
            )
        assert exc.value.rep.status_code == 415
        assert len(session.changes) == 3

        with pytest.raises(RpcTransportError) as exc:
            await alice_editics.send_event_raw(
                '{"type":"unLockDocument",'
                '"isSave":false,'
                '"unlock":true,'
                '"deleteIndex":-1,'  # Here!
                '"releaseLocks":false}'
            )
        assert exc.value.rep.status_code == 415
        assert len(session.changes) == 3


async def test_send_too_large(
    backend: Backend,
    alice_editics: EditicsSessionClient,
) -> None:

    rep = await alice_editics.send(_auth_event(mode="edit"))
    assert isinstance(rep, EditicsServerEventAuth)

    event = (
        '{"type":"saveChanges",'
        '"encryptedChanges":[[1,"abcdef=="]],'
        '"startSaveChanges":true,'
        '"endSaveChanges":true,'
        '"deleteIndex":null,'
        '"excelAdditionalInfoIndexCols":null,'
        '"excelAdditionalInfoIndexRows":null,'
        '"encryptedCursorAdditionalInfo":null,'
        '"releaseLocks":false}'
    )
    backend.config.editics_max_part_size = len(event) - 1

    # Regular request with `Content-Length` header
    with pytest.raises(RpcTransportError) as exc:
        await alice_editics.send_event_raw(event)
    assert exc.value.rep.status_code == 413

    # Request without `Content-Length` header
    async def async_iterator():
        yield event[:50].encode()
        yield event[50:].encode()

    with pytest.raises(RpcTransportError) as exc:
        await alice_editics.send_event_raw(async_iterator())
    assert exc.value.rep.status_code == 413
