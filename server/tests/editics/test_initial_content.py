# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from collections.abc import AsyncIterator
from struct import pack
from typing import Literal

import pytest

from parsec._parsec import RealmRole
from parsec.asgi.editics import EditicsHttpStatus
from parsec.editics_protocol import (
    EditicsClientEventAuth,
    EditicsClientEventMessage,
    EditicsClientEventSaveChanges,
    EditicsClientEventUnLockDocument,
    EditicsServerEventAuth,
    EditicsServerEventConnectState,
    EditicsServerEventMessage,
    EditicsServerEventParsecInitialContentUploaded,
    EditicsServerEventUnSaveLock,
    EditicsServerEventWaitAuth,
)
from tests.common import (
    INITIAL_CONTENT_MEDIA_TYPE,
    Backend,
    CoolorgRpcClients,
    EditicsSessionClient,
    RpcTransportError,
    pack_initial_content_bundle,
    wksp1_alice_gives_role,
)

# The bundle's parts are opaque to the server, so any dummy ciphertexts do:
# first part is the document bin, subsequent ones are the medias.
INITIAL_CONTENT_PARTS = [
    b"<Alice's bin ciphertext>",
    b"<Alice's image 1 ciphertext>",
    b"<Alice's image 2 ciphertext>",
]


def _auth_event(*, mode: Literal["view", "edit"]) -> EditicsClientEventAuth:
    return EditicsClientEventAuth(reconnect=None, mode=mode)


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
            EditicsClientEventUnLockDocument(
                isSave=False, unlock=True, deleteIndex=None, releaseLocks=False
            )
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
    backend: Backend,
    bob_editics: EditicsSessionClient,
) -> None:
    with pytest.raises(RpcTransportError) as exc:
        await bob_editics.upload_initial_content(key_index=1, parts=[b"<Bob's bin ciphertext>"])
    assert exc.value.rep.status_code == EditicsHttpStatus.ReaderMustBeViewer.value
    # The refused upload didn't even create the session
    assert backend.editics._document_sessions == {}


async def test_initial_content_upload_is_broadcast_to_viewers(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    alice_editics: EditicsSessionClient,
    bob_editics: EditicsSessionClient,
) -> None:
    await wksp1_alice_gives_role(
        coolorg, backend, coolorg.mallory.user_id, RealmRole.CONTRIBUTOR, wait_events=True
    )

    # Alice joins first and takes the auth lock (single-editor mode)
    async with alice_editics.listen():
        rep = await alice_editics.send(_auth_event(mode="edit"))
        assert isinstance(rep, EditicsServerEventAuth)

        # Viewers are never parked behind the auth lock, so Bob can join
        # (and her join is broadcasted to Alice)
        async with bob_editics.listen():
            rep = await bob_editics.send(_auth_event(mode="view"))
            assert isinstance(rep, EditicsServerEventAuth)
            event = await alice_editics.recv()
            assert isinstance(event, EditicsServerEventConnectState)

            # Mallory joins as an editor while the auth lock is held: she is parked,
            # i.e. his handshake will complete when the lock will be released
            mallory_editics = EditicsSessionClient(
                who=coolorg.mallory,
                realm_id=coolorg.wksp1_id,
                document_id=alice_editics.document_id,
            )
            async with mallory_editics.listen():
                rep = await mallory_editics.send(_auth_event(mode="edit"))
                assert isinstance(rep, EditicsServerEventWaitAuth)
                for client in (alice_editics, bob_editics):
                    event = await client.recv()
                    assert isinstance(event, EditicsServerEventConnectState)

                # Alice uploads the session's initial content

                await alice_editics.upload_initial_content(key_index=1, parts=INITIAL_CONTENT_PARTS)

                # Bob, as viewer, gets the initial content uploaded notification...
                event = await bob_editics.recv()
                assert isinstance(event, EditicsServerEventParsecInitialContentUploaded)

                # ...but not Mallory as she is waiting on the auth lock...
                # ...and neither Alice as she is the author!

                # Now release the auth lock, and send a dummy event to ensure it is the
                # next one received (i.e. no unexpected one has been send prior to it)

                rep = await alice_editics.send(
                    EditicsClientEventUnLockDocument(
                        isSave=False, unlock=True, deleteIndex=None, releaseLocks=False
                    )
                )
                assert rep is None
                event = await mallory_editics.recv()
                assert isinstance(event, EditicsServerEventAuth)

                rep = await bob_editics.send(
                    EditicsClientEventMessage(encryptedMessage=(1, b"dummy message"))
                )
                assert isinstance(rep, EditicsServerEventMessage)

                event = await alice_editics.recv()
                assert isinstance(event, EditicsServerEventMessage)
                event = await mallory_editics.recv()
                assert isinstance(event, EditicsServerEventMessage)


async def test_participant_session_id_of_another_device_is_refused(
    alice_editics: EditicsSessionClient,
    bob_editics: EditicsSessionClient,
) -> None:
    alice_editics.base_params["psid"] = bob_editics.participant_session_id

    async with bob_editics.listen():
        with pytest.raises(RpcTransportError) as exc:
            await alice_editics.upload_initial_content(key_index=1, parts=INITIAL_CONTENT_PARTS)
        assert exc.value.rep.status_code == EditicsHttpStatus.ParticipantDeviceMismatch.value
