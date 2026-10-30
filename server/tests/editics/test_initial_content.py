# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

"""
Test the editics initial content routes (see `parsec/asgi/editics.py`).

The initial content is the archive of the document converted in the OnlyOffice
format (i.e. {bin, medias}), encrypted with the workspace key (hence opaque to
the server). It is the starting base over which all the patches created during
the edition session are to be applied.

The initial content is uploaded once (POST route) by the first editor
connecting, and fetched (GET route) by the participants joining later (the
viewers connecting prior to the upload are instead told to reload the document
once it becomes available, see the `parsecInitialContentUploaded` server event).

On the wire, the initial content is a bundle of encrypted parts (see the
`## Initial content` part of `parsec/asgi/editics.py`'s module docstring):
the first part is always the document bin, the subsequent ones are the medias.
"""

from __future__ import annotations

from struct import pack
from typing import Literal
from uuid import uuid4

import pytest

from parsec._parsec import RealmRole, VlobID
from parsec._parsec import testbed as tb
from parsec.asgi.editics import EditicsHttpStatus
from parsec.editics_protocol import (
    EDITICS_API_VERSION,
    EditicsClientEventAuth,
    EditicsClientEventMessage,
    EditicsClientEventSaveChanges,
    EditicsClientEventUnLockDocument,
    EditicsParticipantSessionID,
    EditicsServerEventAuth,
    EditicsServerEventConnectState,
    EditicsServerEventMessage,
    EditicsServerEventParsecInitialContentUploaded,
    EditicsServerEventUnSaveLock,
    EditicsServerEventWaitAuth,
)
from parsec.events import EventPinged
from tests.common import (
    INITIAL_CONTENT_MEDIA_TYPE,
    Backend,
    CoolorgRpcClients,
    EditicsSessionClient,
    RpcTransportError,
    pack_initial_content_bundle,
    wksp1_alice_gives_role,
)

# Any document ID would do here: the editics session is created when the first
# participant joins, and the server doesn't check that the document vlob
# actually exists.
DOCUMENT_ID = VlobID.from_hex("00000000000000000000000000000d0c")

# The editics routes negotiate the API version through the `Api-Version` header
API_VERSION = str(EDITICS_API_VERSION)

# The bundle's parts are opaque to the server, so any dummy ciphertexts do:
# first part is the document bin, subsequent ones are the medias.
INITIAL_CONTENT_PARTS = [
    b"<Alice's bin ciphertext>",
    b"<Alice's image 1 ciphertext>",
    b"<Alice's image 2 ciphertext>",
]


def _new_participant_session_id() -> EditicsParticipantSessionID:
    return uuid4().hex


def _auth_event(*, mode: Literal["view", "edit"]) -> EditicsClientEventAuth:
    return EditicsClientEventAuth(vlobVersion=1, reconnect=None, mode=mode)


def _save_changes_event() -> EditicsClientEventSaveChanges:
    return EditicsClientEventSaveChanges(
        encryptedChanges=[(1, b"dummy change")],
        startSaveChanges=True,
        endSaveChanges=True,
        deleteIndex=None,
        excelAdditionalInfoIndexCols=None,
        excelAdditionalInfoIndexRows=None,
        encryptedCursorAdditionalInfo=None,
        releaseLocks=False,
    )


def _realm_without_role_for(coolorg: CoolorgRpcClients) -> VlobID:
    # Bob's user realm exists, but Alice has no role in it
    for event in coolorg.testbed_template.events:
        if isinstance(event, tb.TestbedEventNewRealm) and event.author == coolorg.bob.device_id:
            return event.realm_id
    raise AssertionError("Bob's user realm not found in the coolorg testbed template")


# --- Transport-level handshake (API version & HTTP statuses) ------------------


@pytest.mark.parametrize("bad_api_version", [None, "not a version", "2.0", "1"])
async def test_unsupported_api_version(
    coolorg: CoolorgRpcClients, bad_api_version: str | None
) -> None:
    alice = EditicsSessionClient(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_session_id=_new_participant_session_id(),
    )
    headers = dict(alice.base_headers)
    if bad_api_version is None:
        del headers["Api-Version"]
    else:
        headers["Api-Version"] = bad_api_version

    # POST initial content

    rep = await coolorg.alice.raw_client.post(
        f"{alice.base_url}/initial_content",
        params=alice.base_params,
        headers=headers,
    )
    assert rep.status_code == EditicsHttpStatus.UnsupportedApiVersion.value
    assert rep.headers["Supported-Api-Versions"] == API_VERSION
    # No version could be settled, hence no `Api-Version` reply header
    assert "Api-Version" not in rep.headers

    # GET initial content

    rep = await coolorg.alice.raw_client.get(
        f"{alice.base_url}/initial_content",
        params=alice.base_params,
        headers=headers,
    )
    assert rep.status_code == EditicsHttpStatus.UnsupportedApiVersion.value
    assert rep.headers["Supported-Api-Versions"] == API_VERSION
    assert "Api-Version" not in rep.headers


async def test_compatible_api_version(coolorg: CoolorgRpcClients) -> None:
    alice = EditicsSessionClient(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_session_id=_new_participant_session_id(),
    )
    key_index = 1

    # Only the major version breaks compatibility, so a client using a more
    # recent revision is accepted: the server replies with its own version.

    # POST initial content

    rep = await coolorg.alice.raw_client.post(
        f"{alice.base_url}/initial_content",
        params=alice.base_params,
        headers={
            **alice.base_headers,
            "Api-Version": "1.42",
            "Content-Type": INITIAL_CONTENT_MEDIA_TYPE,
        },
        content=pack_initial_content_bundle(key_index, INITIAL_CONTENT_PARTS),
    )
    assert rep.status_code == 204
    assert rep.headers["Api-Version"] == API_VERSION

    # GET initial content

    rep = await coolorg.alice.raw_client.get(
        f"{alice.base_url}/initial_content",
        params=alice.base_params,
        headers={**alice.base_headers, "Api-Version": "1.42"},
    )
    assert rep.status_code == 200
    assert rep.headers["Api-Version"] == API_VERSION
    assert rep.headers["Content-Type"] == INITIAL_CONTENT_MEDIA_TYPE
    # The fetched bundle is byte-identical to the uploaded one
    assert await rep.aread() == pack_initial_content_bundle(key_index, INITIAL_CONTENT_PARTS)


async def test_missing_authorization(coolorg: CoolorgRpcClients) -> None:
    alice = EditicsSessionClient(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_session_id=_new_participant_session_id(),
    )

    # POST initial content

    rep = await coolorg.alice.raw_client.post(
        f"{alice.base_url}/initial_content",
        params=alice.base_params,
        headers={"Api-Version": API_VERSION},
    )
    assert rep.status_code == EditicsHttpStatus.MissingAuthenticationInfo.value
    assert rep.headers["Api-Version"] == API_VERSION
    assert rep.json()["detail"] == {"detail": "Missing authentication info"}

    # GET initial content

    rep = await coolorg.alice.raw_client.get(
        f"{alice.base_url}/initial_content",
        params=alice.base_params,
        headers={"Api-Version": API_VERSION},
    )
    assert rep.status_code == EditicsHttpStatus.MissingAuthenticationInfo.value
    assert rep.headers["Api-Version"] == API_VERSION
    assert rep.json()["detail"] == {"detail": "Missing authentication info"}


async def test_realm_not_found_is_indistinguishable_from_realm_without_role(
    coolorg: CoolorgRpcClients,
) -> None:
    outcomes = []
    for realm_id in (VlobID.new(), coolorg.wksp1_id):
        mallory = EditicsSessionClient(
            who=coolorg.mallory,
            realm_id=realm_id,
            document_id=DOCUMENT_ID,
            participant_session_id=_new_participant_session_id(),
        )

        # POST initial content

        rep = await coolorg.mallory.raw_client.post(
            f"{mallory.base_url}/initial_content",
            params=mallory.base_params,
            headers=mallory.base_headers,
        )
        outcomes.append(("post", rep.status_code, rep.json()))

        # GET initial content

        rep = await coolorg.mallory.raw_client.get(
            f"{mallory.base_url}/initial_content",
            params=mallory.base_params,
            headers=mallory.base_headers,
        )
        outcomes.append(("get", rep.status_code, rep.json()))

    (post_nonexistent, get_nonexistent, post_without_role, get_without_role) = outcomes

    # Information hiding: an organization member must not learn whether a realm
    # it has no access to exists or not
    assert post_nonexistent == post_without_role
    assert get_nonexistent == get_without_role
    assert post_nonexistent[1] == EditicsHttpStatus.UserNoRealmRole.value
    assert get_nonexistent[1] == EditicsHttpStatus.UserNoRealmRole.value


async def test_upload_bad_content_type(coolorg: CoolorgRpcClients) -> None:
    alice = EditicsSessionClient(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_session_id=_new_participant_session_id(),
    )

    rep = await coolorg.alice.raw_client.post(
        f"{alice.base_url}/initial_content",
        params=alice.base_params,
        headers={**alice.base_headers, "Content-Type": "application/octet-stream"},
        content=pack_initial_content_bundle(1, INITIAL_CONTENT_PARTS),
    )
    assert rep.status_code == EditicsHttpStatus.BadContentTypeOrInvalidBody.value
    assert rep.headers["Api-Version"] == API_VERSION


@pytest.mark.parametrize(
    "raw_bundle",
    [
        # Missing the key index
        b"",
        b"\x00\x00",
        # No part (the document bin is expected first)
        pack("!I", 1),
        # Truncated part size
        pack("!I", 1) + b"\x00",
        # Truncated part ciphertext
        pack("!I", 1) + pack("!I", 42) + b"<truncated ciphertext>",
    ],
)
async def test_upload_invalid_bundle(coolorg: CoolorgRpcClients, raw_bundle: bytes) -> None:
    # The server doesn't care about the parts' content (opaque ciphertexts),
    # but the outer envelope of the bundle must be well-formed (see the
    # `### format` part of `parsec/asgi/editics.py`'s module docstring).
    alice = EditicsSessionClient(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_session_id=_new_participant_session_id(),
    )

    rep = await coolorg.alice.raw_client.post(
        f"{alice.base_url}/initial_content",
        params=alice.base_params,
        headers={**alice.base_headers, "Content-Type": INITIAL_CONTENT_MEDIA_TYPE},
        content=raw_bundle,
    )
    assert rep.status_code == EditicsHttpStatus.BadContentTypeOrInvalidBody.value
    assert rep.headers["Api-Version"] == API_VERSION
    assert rep.json()["detail"] == {"detail": "Bad content type or invalid body"}


async def test_upload_payload_too_large(
    coolorg: CoolorgRpcClients, monkeypatch: pytest.MonkeyPatch
) -> None:
    # The bundle size is limited to 100Mo (see `parsec/asgi/editics.py`'s module
    # docstring): shrink the limit so the test doesn't have to upload 100Mo.
    monkeypatch.setattr("parsec.asgi.editics.MAX_INITIAL_CONTENT_SIZE", 16)

    alice = EditicsSessionClient(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_session_id=_new_participant_session_id(),
    )

    with pytest.raises(RpcTransportError) as exc:
        await alice.upload_initial_content(key_index=1, parts=[b"x" * 32])
    assert exc.value.rep.status_code == EditicsHttpStatus.PayloadTooLarge.value


# --- Initial content lifecycle -------------------------------------------------


async def test_initial_content_can_be_uploaded_before_any_participant(
    coolorg: CoolorgRpcClients,
) -> None:
    # The upload route is independent from the session handshake: the initial
    # content can be provided even before any participant has joined (and even
    # before the editics session itself exists).
    alice = EditicsSessionClient(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_session_id=_new_participant_session_id(),
    )
    await alice.upload_initial_content(key_index=1, parts=INITIAL_CONTENT_PARTS)

    async with alice.listen():
        # The late-joining editor is told the initial content is already there
        rep = await alice.send(_auth_event(mode="edit"))
        assert isinstance(rep, EditicsServerEventAuth)
        assert rep.originalContentUploaded is True
        assert await alice.fetch_initial_content() == (1, INITIAL_CONTENT_PARTS)


async def test_initial_content_lifecycle(coolorg: CoolorgRpcClients, backend: Backend) -> None:
    await wksp1_alice_gives_role(
        coolorg, backend, coolorg.bob.user_id, RealmRole.CONTRIBUTOR, wait_events=True
    )

    # Alice is the first editor of the session: she must upload the initial content
    alice = EditicsSessionClient(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_session_id=_new_participant_session_id(),
    )
    # Bob joins the session in step 4, but his client is also used before that
    # to attempt a concurrent initial content upload in step 2.
    bob = EditicsSessionClient(
        who=coolorg.bob,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_session_id=_new_participant_session_id(),
    )
    async with alice.listen():
        # 1. Joining as the first editor: no initial content yet, so Alice is
        #    expected to upload her own conversion of the document before
        #    anything else.

        rep = await alice.send(_auth_event(mode="edit"))
        assert isinstance(rep, EditicsServerEventAuth)
        assert rep.originalContentUploaded is False

        # The initial content is not available for fetching yet
        with pytest.raises(RpcTransportError) as exc:
            await alice.fetch_initial_content()
        assert exc.value.rep.status_code == EditicsHttpStatus.OriginalContentMissing.value

        # Any other editor event but `auth`/`close` is refused until the upload is done
        with pytest.raises(RpcTransportError) as exc:
            await alice.send(EditicsClientEventMessage(encryptedMessage=(1, b"dummy message")))
        assert exc.value.rep.status_code == EditicsHttpStatus.OriginalContentMissing.value

        # 2. Actual initial content upload

        await alice.upload_initial_content(key_index=1, parts=INITIAL_CONTENT_PARTS)
        assert await alice.fetch_initial_content() == (1, INITIAL_CONTENT_PARTS)

        # First-upload-wins: a concurrent upload is refused, the loser must use
        # the existing content instead of its own conversion.
        with pytest.raises(RpcTransportError) as exc:
            await bob.upload_initial_content(key_index=1, parts=[b"<Bob's bin ciphertext>"])
        assert exc.value.rep.status_code == EditicsHttpStatus.OriginalContentAlreadyUploaded.value
        # The session's content is still Alice's one
        assert await alice.fetch_initial_content() == (1, INITIAL_CONTENT_PARTS)

        # 3. Release the auth lock (i.e. switch to co-editing mode) so Bob's
        #    handshake completes right away instead of being parked.

        rep = await alice.send(EditicsClientEventUnLockDocument(unlock=True, deleteIndex=None))
        assert rep is None

        # 4. Bob joins the session: the initial content is already there, so he
        #    is expected to fetch it instead of converting the document himself.

        async with bob.listen():
            rep = await bob.send(_auth_event(mode="edit"))
            assert isinstance(rep, EditicsServerEventAuth)
            assert rep.originalContentUploaded is True

            assert await bob.fetch_initial_content() == (1, INITIAL_CONTENT_PARTS)


async def test_save_changes_refused_before_initial_content(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    # RFC 1030 §1.2: no document modification is accepted while the session has
    # no initial content (client-side this cannot happen given an editor
    # uploads its conversion before loading the document into the editor).
    alice = EditicsSessionClient(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_session_id=_new_participant_session_id(),
    )
    async with alice.listen():
        rep = await alice.send(_auth_event(mode="edit"))
        assert isinstance(rep, EditicsServerEventAuth)

        # The `saveChanges` event is refused...
        with pytest.raises(RpcTransportError) as exc:
            await alice.send(_save_changes_event())
        assert exc.value.rep.status_code == EditicsHttpStatus.OriginalContentMissing.value

        # ...and nothing was stored
        (session,) = backend.editics._document_sessions.values()
        assert session.changes == []

        # Once the initial content is uploaded, the changes are stored again
        await alice.upload_initial_content(key_index=1, parts=INITIAL_CONTENT_PARTS)

        rep = await alice.send(_save_changes_event())
        assert isinstance(rep, EditicsServerEventUnSaveLock)
        assert rep.index == 0
        assert rep.syncChangesIndex == 1
        (session,) = backend.editics._document_sessions.values()
        assert len(session.changes) == 1


async def test_viewer_cannot_upload_initial_content(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    # Bob is READER on wksp1, so he cannot provide the session's initial content.
    bob = EditicsSessionClient(
        who=coolorg.bob,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_session_id=_new_participant_session_id(),
    )
    with pytest.raises(RpcTransportError) as exc:
        await bob.upload_initial_content(key_index=1, parts=[b"<Bob's bin ciphertext>"])
    assert exc.value.rep.status_code == EditicsHttpStatus.ReaderMustBeViewer.value
    # The refused upload didn't even create the session
    assert backend.editics._document_sessions == {}


async def test_initial_content_upload_is_broadcast_to_viewers(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    # A viewer having opened the document from its own local conversion must
    # reload it once the session's initial content becomes available: it is
    # told so by a dedicated server event, broadcast only to the viewers (an
    # editor is either the uploader, or will learn the content is already
    # there from its own upload attempt).
    #
    # Bob is READER on wksp1: grant him a write role so he can join as an
    # editor. Mallory is not part of wksp1: give her a read-only role so she
    # can join as a viewer.
    #
    # The event bus dispatches its events asynchronously: first drain the
    # events still queued from the testbed template loading (they are
    # unrelated to our organization, but would make the waits below return
    # early), then wait for each role change to be actually dispatched before
    # going on. This is needed since a realm certificate landing while a
    # participant has opened its SSE connection but not yet sent its `auth`
    # event kicks it (see `EditicsComponent._on_event`).
    with backend.event_bus.spy() as spy:
        await backend.event_bus.test_send(
            EventPinged(organization_id=coolorg.organization_id, ping="drain")
        )
        await spy.wait(EventPinged)
    await wksp1_alice_gives_role(
        coolorg, backend, coolorg.bob.user_id, RealmRole.CONTRIBUTOR, wait_events=True
    )
    await wksp1_alice_gives_role(
        coolorg, backend, coolorg.mallory.user_id, RealmRole.READER, wait_events=True
    )

    # Alice joins first and takes the auth lock (single-editor mode)
    alice = EditicsSessionClient(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=DOCUMENT_ID,
        participant_session_id=_new_participant_session_id(),
    )
    async with alice.listen():
        rep = await alice.send(_auth_event(mode="edit"))
        assert isinstance(rep, EditicsServerEventAuth)

        # Viewers are never parked behind the auth lock, so Mallory can join
        # (and her join is broadcasted to Alice)
        mallory = EditicsSessionClient(
            who=coolorg.mallory,
            realm_id=coolorg.wksp1_id,
            document_id=DOCUMENT_ID,
            participant_session_id=_new_participant_session_id(),
        )
        async with mallory.listen():
            rep = await mallory.send(_auth_event(mode="view"))
            assert isinstance(rep, EditicsServerEventAuth)

            # Bob joins as an editor while the auth lock is held: he is parked,
            # i.e. his handshake will complete when the lock will be released
            bob = EditicsSessionClient(
                who=coolorg.bob,
                realm_id=coolorg.wksp1_id,
                document_id=DOCUMENT_ID,
                participant_session_id=_new_participant_session_id(),
            )
            async with bob.listen():
                rep = await bob.send(_auth_event(mode="edit"))
                assert isinstance(rep, EditicsServerEventWaitAuth)

                # Alice uploads the session's initial content

                await alice.upload_initial_content(key_index=1, parts=INITIAL_CONTENT_PARTS)

                # Mallory, a viewer having opened the document from her own
                # local conversion, is told to reload it with the session's
                # content (the other events on her stream are the `connectState`
                # broadcasts of the late joiners).
                while True:
                    event = await mallory.recv()
                    if isinstance(event, EditicsServerEventParsecInitialContentUploaded):
                        break
                    assert isinstance(event, EditicsServerEventConnectState)

                # Bob (an editor, even a parked one) doesn't receive the
                # broadcast: releasing the auth lock completes his handshake
                # with his `auth` reply, then the regular events (a chat message
                # broadcast here) resume.
                rep = await alice.send(
                    EditicsClientEventUnLockDocument(unlock=True, deleteIndex=None)
                )
                assert rep is None

                # Alice (the uploader) doesn't receive the broadcast either: her
                # next events are the `connectState` broadcasts of the late
                # joiners, then the chat message.
                rep = await mallory.send(
                    EditicsClientEventMessage(encryptedMessage=(1, b"dummy message"))
                )
                assert isinstance(rep, EditicsServerEventMessage)

                event = await bob.recv()
                assert isinstance(event, EditicsServerEventAuth)
                event = await bob.recv()
                assert isinstance(event, EditicsServerEventMessage)

                event = await alice.recv()
                assert isinstance(event, EditicsServerEventConnectState)  # Mallory's join
                event = await alice.recv()
                assert isinstance(event, EditicsServerEventConnectState)  # Bob's join
                event = await alice.recv()
                assert isinstance(event, EditicsServerEventMessage)
