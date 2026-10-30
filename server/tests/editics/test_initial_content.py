# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from collections.abc import AsyncIterator
from struct import pack
from typing import Literal
from uuid import uuid4

import pytest

from parsec._parsec import RealmRole, VlobID
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

    # POST initial content

    rep = await alice_editics.raw_client.post(
        f"{alice_editics.base_url}/initial_content",
        params=alice_editics.base_params,
        headers=headers,
    )
    assert rep.status_code == EditicsHttpStatus.UnsupportedApiVersion.value
    assert rep.headers["Supported-Api-Versions"] == API_VERSION
    # No version could be settled, hence no `Api-Version` reply header
    assert "Api-Version" not in rep.headers

    # GET initial content

    rep = await alice_editics.raw_client.get(
        f"{alice_editics.base_url}/initial_content",
        params=alice_editics.base_params,
        headers=headers,
    )
    assert rep.status_code == EditicsHttpStatus.UnsupportedApiVersion.value
    assert rep.headers["Supported-Api-Versions"] == API_VERSION
    assert "Api-Version" not in rep.headers


async def test_compatible_api_version(alice_editics: EditicsSessionClient) -> None:
    key_index = 1

    # Only the major version breaks compatibility, so a client using a more
    # recent revision is accepted: the server replies with its own version.

    # POST initial content

    rep = await alice_editics.raw_client.post(
        f"{alice_editics.base_url}/initial_content",
        params=alice_editics.base_params,
        headers={
            **alice_editics.base_headers,
            "Api-Version": "1.42",
            "Content-Type": INITIAL_CONTENT_MEDIA_TYPE,
        },
        content=pack_initial_content_bundle(key_index, INITIAL_CONTENT_PARTS),
    )
    assert rep.status_code == 204
    assert rep.headers["Api-Version"] == API_VERSION

    # GET initial content

    rep = await alice_editics.raw_client.get(
        f"{alice_editics.base_url}/initial_content",
        params=alice_editics.base_params,
        headers={**alice_editics.base_headers, "Api-Version": "1.42"},
    )
    assert rep.status_code == 200
    assert rep.headers["Api-Version"] == API_VERSION
    assert rep.headers["Content-Type"] == INITIAL_CONTENT_MEDIA_TYPE
    # The fetched bundle is byte-identical to the uploaded one
    assert await rep.aread() == pack_initial_content_bundle(key_index, INITIAL_CONTENT_PARTS)


async def test_missing_authorization(alice_editics: EditicsSessionClient) -> None:
    # POST initial content

    rep = await alice_editics.raw_client.post(
        f"{alice_editics.base_url}/initial_content",
        params=alice_editics.base_params,
        headers={"Api-Version": API_VERSION},
    )
    assert rep.status_code == EditicsHttpStatus.MissingAuthenticationInfo.value
    assert rep.headers["Api-Version"] == API_VERSION
    assert rep.json()["detail"] == {"detail": "Missing authentication info"}

    # GET initial content

    rep = await alice_editics.raw_client.get(
        f"{alice_editics.base_url}/initial_content",
        params=alice_editics.base_params,
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
            document_id=coolorg.wksp1_id,
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


async def test_upload_bad_content_type(alice_editics: EditicsSessionClient) -> None:
    rep = await alice_editics.raw_client.post(
        f"{alice_editics.base_url}/initial_content",
        params=alice_editics.base_params,
        headers={**alice_editics.base_headers, "Content-Type": "application/octet-stream"},
        content=pack_initial_content_bundle(1, INITIAL_CONTENT_PARTS),
    )
    assert rep.status_code == EditicsHttpStatus.BadContentTypeOrInvalidBody.value
    assert rep.headers["Api-Version"] == API_VERSION


async def _iter_chunks(raw: bytes, chunk_size: int) -> AsyncIterator[bytes]:
    for offset in range(0, len(raw), chunk_size):
        yield raw[offset : offset + chunk_size]


@pytest.mark.parametrize("chunk_size", [1, 2, 3, 7, 1024])
async def test_upload_streamed_bundle_is_chunking_agnostic(
    alice_editics: EditicsSessionClient, chunk_size: int
) -> None:
    bundle = pack_initial_content_bundle(1, INITIAL_CONTENT_PARTS)
    rep = await alice_editics.raw_client.post(
        f"{alice_editics.base_url}/initial_content",
        params=alice_editics.base_params,
        headers={**alice_editics.base_headers, "Content-Type": INITIAL_CONTENT_MEDIA_TYPE},
        content=_iter_chunks(bundle, chunk_size),
    )
    assert rep.status_code == 204
    assert rep.headers["Api-Version"] == API_VERSION
    # The parts survived the streaming round-trip uncorrupted
    assert await alice_editics.fetch_initial_content() == (1, INITIAL_CONTENT_PARTS)


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
async def test_upload_invalid_bundle(
    alice_editics: EditicsSessionClient, raw_bundle: bytes
) -> None:
    # Do a request with and without `Content-Type` header
    for content in [raw_bundle, _iter_chunks(raw_bundle, 1)]:
        rep = await alice_editics.raw_client.post(
            f"{alice_editics.base_url}/initial_content",
            params=alice_editics.base_params,
            headers={**alice_editics.base_headers, "Content-Type": INITIAL_CONTENT_MEDIA_TYPE},
            content=content,
        )
        assert rep.status_code == EditicsHttpStatus.BadContentTypeOrInvalidBody.value
        assert rep.headers["Api-Version"] == API_VERSION
        assert rep.json()["detail"] == {"detail": "Bad content type or invalid body"}


async def test_upload_payload_too_large(
    alice_editics: EditicsSessionClient, backend: Backend
) -> None:
    backend.config.editics_max_document_size = 16

    # Regular request with a `Content-Length` header
    with pytest.raises(RpcTransportError) as exc:
        await alice_editics.upload_initial_content(key_index=1, parts=[b"x" * 32])
    assert exc.value.rep.status_code == EditicsHttpStatus.ContentTooLarge.value

    # Streamed request without `Content-Length` header: the size check
    # must be done while streaming the body
    async def async_iterator():
        yield b"x" * 16
        yield b"x" * 16

    rep = await alice_editics.raw_client.post(
        f"{alice_editics.base_url}/initial_content",
        params=alice_editics.base_params,
        headers={**alice_editics.base_headers, "Content-Type": INITIAL_CONTENT_MEDIA_TYPE},
        content=async_iterator(),
    )
    assert rep.status_code == EditicsHttpStatus.ContentTooLarge.value


# --- Initial content lifecycle -------------------------------------------------


async def test_initial_content_can_be_uploaded_before_any_participant(
    alice_editics: EditicsSessionClient,
) -> None:
    # The upload route is independent from the session handshake: the initial
    # content can be provided even before any participant has joined (and even
    # before the editics session itself exists).
    await alice_editics.upload_initial_content(key_index=1, parts=INITIAL_CONTENT_PARTS)

    async with alice_editics.listen():
        # The late-joining editor is told the initial content is already there
        rep = await alice_editics.send(_auth_event(mode="edit"))
        assert isinstance(rep, EditicsServerEventAuth)
        assert rep.initialContentUploaded is True
        assert await alice_editics.fetch_initial_content() == (1, INITIAL_CONTENT_PARTS)


async def test_initial_content_lifecycle(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    alice_editics: EditicsSessionClient,
    bob_editics: EditicsSessionClient,
) -> None:
    await wksp1_alice_gives_role(
        coolorg, backend, coolorg.bob.user_id, RealmRole.CONTRIBUTOR, wait_events=True
    )

    # Alice is the first editor of the session: she must upload the initial content
    # Bob joins the session in step 4, but his client is also used before that
    # to attempt a concurrent initial content upload in step 2.
    async with alice_editics.listen():
        # 1. Joining as the first editor: no initial content yet, so Alice is
        #    expected to upload her own conversion of the document before
        #    anything else.

        rep = await alice_editics.send(_auth_event(mode="edit"))
        assert isinstance(rep, EditicsServerEventAuth)
        assert rep.initialContentUploaded is False

        # The initial content is not available for fetching yet
        with pytest.raises(RpcTransportError) as exc:
            await alice_editics.fetch_initial_content()
        assert exc.value.rep.status_code == EditicsHttpStatus.InitialContentMissing.value

        # Any other editor event but `auth`/`close` is refused until the upload is done
        with pytest.raises(RpcTransportError) as exc:
            await alice_editics.send(
                EditicsClientEventMessage(encryptedMessage=(1, b"dummy message"))
            )
        assert exc.value.rep.status_code == EditicsHttpStatus.InitialContentMissing.value

        # 2. Actual initial content upload

        await alice_editics.upload_initial_content(key_index=1, parts=INITIAL_CONTENT_PARTS)
        assert await alice_editics.fetch_initial_content() == (1, INITIAL_CONTENT_PARTS)

        # First-upload-wins: a concurrent upload is refused, the loser must use
        # the existing content instead of its own conversion.
        with pytest.raises(RpcTransportError) as exc:
            await bob_editics.upload_initial_content(key_index=1, parts=[b"<Bob's bin ciphertext>"])
        assert exc.value.rep.status_code == EditicsHttpStatus.InitialContentAlreadyUploaded.value
        # The session's content is still Alice's one
        assert await alice_editics.fetch_initial_content() == (1, INITIAL_CONTENT_PARTS)

        # 3. Release the auth lock (i.e. switch to co-editing mode) so Bob's
        #    handshake completes right away instead of being parked.

        rep = await alice_editics.send(
            EditicsClientEventUnLockDocument(unlock=True, deleteIndex=None)
        )
        assert rep is None

        # 4. Bob joins the session: the initial content is already there, so he
        #    is expected to fetch it instead of converting the document himself.

        async with bob_editics.listen():
            rep = await bob_editics.send(_auth_event(mode="edit"))
            assert isinstance(rep, EditicsServerEventAuth)
            assert rep.initialContentUploaded is True

            assert await bob_editics.fetch_initial_content() == (1, INITIAL_CONTENT_PARTS)


async def test_save_changes_refused_before_initial_content(
    backend: Backend,
    alice_editics: EditicsSessionClient,
) -> None:
    # RFC 1030 §1.2: no document modification is accepted while the session has
    # no initial content (client-side this cannot happen given an editor
    # uploads its conversion before loading the document into the editor).
    async with alice_editics.listen():
        rep = await alice_editics.send(_auth_event(mode="edit"))
        assert isinstance(rep, EditicsServerEventAuth)

        # The `saveChanges` event is refused...
        with pytest.raises(RpcTransportError) as exc:
            await alice_editics.send(_save_changes_event())
        assert exc.value.rep.status_code == EditicsHttpStatus.InitialContentMissing.value

        # ...and nothing was stored
        (session,) = backend.editics._document_sessions.values()
        assert session.changes == []

        # Once the initial content is uploaded, the changes are stored again
        await alice_editics.upload_initial_content(key_index=1, parts=INITIAL_CONTENT_PARTS)

        rep = await alice_editics.send(_save_changes_event())
        assert isinstance(rep, EditicsServerEventUnSaveLock)
        assert rep.index == 0
        assert rep.syncChangesIndex == 1
        (session,) = backend.editics._document_sessions.values()
        assert len(session.changes) == 1


async def test_viewer_cannot_upload_initial_content(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    bob_editics: EditicsSessionClient,
) -> None:
    # Bob is READER on wksp1, so he cannot provide the session's initial content.
    with pytest.raises(RpcTransportError) as exc:
        await bob_editics.upload_initial_content(key_index=1, parts=[b"<Bob's bin ciphertext>"])
    assert exc.value.rep.status_code == EditicsHttpStatus.ReaderMustBeViewer.value
    # The refused upload didn't even create the session
    assert backend.editics._document_sessions == {}


async def test_initial_content_upload_is_broadcast_to_viewers(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    alice_editics: EditicsSessionClient,
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
    async with alice_editics.listen():
        rep = await alice_editics.send(_auth_event(mode="edit"))
        assert isinstance(rep, EditicsServerEventAuth)

        # Viewers are never parked behind the auth lock, so Mallory can join
        # (and her join is broadcasted to Alice)
        mallory = EditicsSessionClient(
            who=coolorg.mallory,
            realm_id=coolorg.wksp1_id,
            document_id=alice_editics.document_id,
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
                document_id=alice_editics.document_id,
                participant_session_id=_new_participant_session_id(),
            )
            async with bob.listen():
                rep = await bob.send(_auth_event(mode="edit"))
                assert isinstance(rep, EditicsServerEventWaitAuth)

                # Alice uploads the session's initial content

                await alice_editics.upload_initial_content(key_index=1, parts=INITIAL_CONTENT_PARTS)

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
                rep = await alice_editics.send(
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

                event = await alice_editics.recv()
                assert isinstance(event, EditicsServerEventConnectState)  # Mallory's join
                event = await alice_editics.recv()
                assert isinstance(event, EditicsServerEventConnectState)  # Bob's join
                event = await alice_editics.recv()
                assert isinstance(event, EditicsServerEventMessage)
