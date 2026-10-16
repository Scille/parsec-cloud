# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

import time
from collections.abc import Generator
from contextlib import contextmanager
from dataclasses import dataclass, field
from enum import auto
from typing import Any
from uuid import UUID, uuid4

import anyio
from anyio.streams.memory import MemoryObjectReceiveStream, MemoryObjectSendStream

from parsec._parsec import DeviceID, OrganizationID, RealmRole, VlobID
from parsec.components.auth import EditicsAuthInfo
from parsec.config import BackendConfig
from parsec.editics_protocol import (
    AuthChangeRecord,
    CursorRecord,
    EditicsClientEvent,
    EditicsClientEventAuth,
    EditicsClientEventAuthChangesAck,
    EditicsClientEventClose,
    EditicsClientEventCursor,
    EditicsClientEventGetLock,
    EditicsClientEventGetMessages,
    EditicsClientEventIsSaveLock,
    EditicsClientEventMessage,
    EditicsClientEventSaveChanges,
    EditicsClientEventUnLockDocument,
    EditicsClientEventUnSaveLock,
    EditicsIndexUser,
    EditicsParticipantEntry,
    EditicsParticipantID,
    EditicsServerEvent,
    EditicsServerEventAuth,
    EditicsServerEventAuthChanges,
    EditicsServerEventConnectState,
    EditicsServerEventCursor,
    EditicsServerEventGetLock,
    EditicsServerEventMessage,
    EditicsServerEventReleaseLock,
    EditicsServerEventSaveChanges,
    EditicsServerEventSaveLock,
    EditicsServerEventSavePartChanges,
    EditicsServerEventUnSaveLock,
    EditicsServerEventWaitAuth,
    LockRecord,
    MessageRecord,
    ReleaseLockRecord,
    SaveChangeRecord,
)
from parsec.logging import get_logger
from parsec.types import BadOutcomeEnum

logger = get_logger()

PER_PARTICIPANT_MAX_BUFFER_EVENTS = 100
type EditicsSessionJoinEventStream = MemoryObjectReceiveStream[EditicsServerEvent | None]


def now() -> int:
    """
    Timestamp as milliseconds
    """
    return int(time.time() * 1000)


@dataclass(slots=True)
class ChatMessage:
    """One chat message kept in the session history.

    The author's identity is stored alongside (participant id + device id): the
    message can be replayed to a client that never saw its author (the author
    left the session before it joined), so the identity cannot be resolved
    through the current participants list.
    """

    time_ms: int
    author_participant_id: EditicsParticipantID
    author_device_id: DeviceID
    encrypted_message: tuple[int, bytes]


@dataclass(slots=True)
class StoredChange:
    """One change fragment kept in the session history (RFC §2.2).

    `encrypted_change` is an opaque encrypted blob, the server never inspects it.
    The change history is 0-indexed (like the OnlyOffice `changesIndex` save
    point) and can be truncated from a given index on UNDO.

    The author's identity is stored alongside (participant id + device id), like
    for a `ChatMessage`: the change can be replayed to a client that never saw
    its author.
    """

    time_ms: int
    author_participant_id: EditicsParticipantID
    author_device_id: DeviceID
    # Format: (<Key index>, <base64(ciphertext)>)
    encrypted_change: tuple[int, bytes]


@dataclass(slots=True)
class ParticipantAuthed:
    # Joined while the auth lock was held: the participant is queued to receive
    # its `auth` server event when the lock is released.
    # Note the lock is resolved optimistically for a second concurrent joiner:
    # such a participant gets its `auth` right away but is still considered
    # parked, so it also receives the post-unlock `auth`.
    parked: bool = False
    # The participant takes part in the session as a view (i.e. read-only mode).
    # This mode is mandatory for user with only READER role, but can also be used
    # for user with write access (e.g. CONTRIBUTOR role) for live preview.
    # When in view mode, the participant never co-edits, and hence never holds nor
    # waits for the locks.
    view: bool = True


@dataclass(slots=True)
class Participant:
    """
    Participant represents a single OnlyOffice editor connected to a document edition session.

    - A device can have multiple participants.
    - Participant is created by having a client send a `auth` event.
    - Participant survives SSE disconnection: instead it is only destroyed when the
      client timeouts or sends a `close` event.
    """

    device_id: DeviceID
    index: EditicsIndexUser
    # In OnlyOffice participant ID is  `<device ID> + <participant index>`,
    # however here its is an arbitraty UUID controlled by the client.
    # This is because in OnlyOffice this ID is provided in the server `auth` event,
    # while here we must have it when we do the SSE connection (which must occur
    # before the HTTP `auth` request is done!).
    participant_id: EditicsParticipantID
    # TODO: does this correspond to the very first connection of the participant or to its last (re)connection ?
    connected_at: int  # ms timestamp of the join
    # `None` if no SSE connection is listening for the participant.
    # At most one SSE connection is possible for a given participant (any older
    # one is considered as a zombie and gets closed).
    sse_channel_sender: MemoryObjectSendStream[EditicsServerEvent] | None = None
    # Completed its `auth` handshake: only then does the participant appear in
    # the participants lists (before that, only its SSE connection is
    # registered in the session).
    authed: ParticipantAuthed | None = None


@dataclass(slots=True)
class EditicsSession:
    """
    In-memory state of a document edition session
    """

    # When the file edition session has been started
    opened_at: int  # ms
    session_id: UUID = field(default_factory=uuid4)
    next_participant_index: int = 1  # monotonic, starts at 1
    participants: dict[EditicsParticipantID, Participant] = field(default_factory=dict)
    chat_messages: list[ChatMessage] = field(default_factory=list)
    # At creation, the session is in single-editor mode: the first participant held
    # the auth lock.
    # Participants joining while this lock is held are parked (i.e. they receive
    # `waitAuth` from the server) until the first participant release it (i.e. send
    # `unLockDocument{unlock:true}`) which switch the session into co-editing mode.
    auth_lock_holder: EditicsParticipantID | None = None
    # Monotonic change counter (= OnlyOffice `puckerIndex`): advances on every
    # stored change fragment, and is only rewound by an undo (i.e. a `saveChanges`
    # carrying a `deleteIndex`).
    sync_changes_index: int = 0
    # Ordered change history (OnlyOffice change index = position in the list).
    changes: list[StoredChange] = field(default_factory=list)
    # Save lock (i.e. the participant currently saving).
    # Minimal handling for now: taken on `isSaveLock`/`saveChanges`, released on
    # save completion (i.e. `saveChanges{endSaveChanges:true}`) or cancellation
    # (i.e. `unSaveLock`/`unLockDocument{isSave:true}`); no expiration (OnlyOffice
    # expires it after 60s). Desynchronized clients are denied the lock until
    # they catch up (see the `isSaveLock` client event handler).
    save_lock_holder: EditicsParticipantID | None = None
    # --- Region locks (RFC §2.2) ---
    # Block key (the block itself for the document editor, its `guid` for the
    # other editors) -> lock record (the holder being the `user` field).
    # The block descriptors are opaque to the server (their shape depends on
    # the editor type, see `EditicsClientEventGetLock`).
    region_locks: dict[str, LockRecord] = field(default_factory=dict)

    # initial_version: int
    # latest_allowed_version: int
    # # Monotonic non-decreasing timestamp (ms) of the last `connectState`
    # # broadcast (todo step_1 §6.1 V-A3). Bumped on every participant-set change.
    # participants_timestamp: int = 0
    # # indexUser -> deviceId
    # participants: dict[IndexUser, DeviceID] = field(default_factory=dict)
    # # participant_uuid (client-generated) -> SSE channel
    # connections: dict[UUID, EditicsSseChannel] = field(default_factory=dict)
    # # Chat history (delivered to newcomers on join).
    # messages: list[ChatMessage] = field(default_factory=list)
    # # The participant that just released the save lock (one-round only, §6.9):
    # # only this participant may send `saveDone` to bump `latest_allowed_version`.
    # last_save_lock_holder: IndexUser | None = None
    # # In-flight multi-chunk save buffer for the current lock holder, or None.
    # in_flight_save: InFlightSave | None = None


def _participant_entries(session: EditicsSession) -> list[EditicsParticipantEntry]:
    return [
        EditicsParticipantEntry(
            id=participant.participant_id,
            indexUser=participant.index,
            deviceId=participant.device_id,
            view=participant.authed.view,
        )
        # Connections that haven't completed their `auth` handshake are not
        # participants yet (e.g. in record `02_auth_concurrent`, Bob and
        # Mallory both connect before either has authed).
        for participant in session.participants.values()
        if participant.authed
    ]


def _block_key(block: Any) -> str:
    """Key identifying a region lock's block (RFC §2.2).

    OnlyOffice keys the lock table by the block id itself for the document
    editor (a plain string), and by the block `guid` for the other editors.
    """
    return block if isinstance(block, str) else block["guid"]


def _release_participant_locks(
    session: EditicsSession, participant: Participant
) -> list[ReleaseLockRecord]:
    """Remove all the region locks held by `participant`, as records ready to
    be embedded in a `saveChanges` broadcast or a standalone `releaseLock`
    event (RFC §2.2)."""
    released: list[ReleaseLockRecord] = []
    for key, lock in list(session.region_locks.items()):
        if lock.holder[0] == participant.participant_id:
            released.append(
                ReleaseLockRecord(
                    block=lock.block,
                    holder=(participant.participant_id, participant.device_id),
                    time=now(),
                    changes=None,
                )
            )
            del session.region_locks[key]
    return released


def _push_changes_backlog(session: EditicsSession, participant: Participant) -> None:
    """Push on the participant's SSE channel the backlog of changes that
    occurred since the session was created (delivered as a single `authChanges`
    chunk, RFC §2.2), so a newcomer can catch up before its `auth` reply."""
    if not session.changes or participant.sse_channel_sender is None:
        return
    participant.sse_channel_sender.send_nowait(
        EditicsServerEventAuthChanges(
            changes=[
                AuthChangeRecord(
                    index=index + 1,
                    author=(change.author_participant_id, change.author_device_id),
                    time=change.time_ms,
                    encryptedChange=change.encrypted_change,
                )
                for index, change in enumerate(session.changes)
            ],
        )
    )


def _handshake_auth(session: EditicsSession, participant: Participant) -> EditicsServerEventAuth:
    """Build the newcomer's `auth` reply (its handshake completion, RFC §2.2)."""
    return EditicsServerEventAuth(
        participants=_participant_entries(session),
        indexUser=participant.index,
        participantId=participant.participant_id,
        participantTimeConnect=participant.connected_at,
        openedAt=session.opened_at,
        locks=list(session.region_locks.values()),
        messages=[
            MessageRecord(
                time=m.time_ms,
                author=(m.author_participant_id, m.author_device_id),
                encryptedMessage=m.encrypted_message,
            )
            for m in session.chat_messages
        ],
    )


class EditicsParticipantListenBadOutcome(BadOutcomeEnum):
    PARTICIPANT_DEVICE_MISMATCH = auto()


class EditicsParticipantSendBadOutcome(BadOutcomeEnum):
    # First event must be `auth`
    PARTICIPANT_NOT_AUTH = auto()
    # Subsequent `auth` are reconnection, and a device cannot do impersonation
    # by reconnecting with a participant ID related to another device!
    PARTICIPANT_DEVICE_MISMATCH = auto()
    # Participant has READER role and tries to edit
    READER_MUST_BE_VIEWER = auto()


class EditicsComponent:
    def __init__(self, config: BackendConfig) -> None:
        self._config = config
        # Sessions key is (organization ID, realm ID, document ID)
        self._sessions: dict[tuple[OrganizationID, VlobID, VlobID], EditicsSession] = {}
        # # (workspace_id, vlob_id, participant_uuid) -> channel
        # self._pending: dict[tuple[VlobID, VlobID, UUID], EditicsSseChannel] = {}

    def _get_or_create_session(
        self,
        key: tuple[OrganizationID, VlobID, VlobID],
    ) -> EditicsSession:
        try:
            return self._sessions[key]
        except KeyError:
            session = self._sessions[key] = EditicsSession(opened_at=now())
            return session

    def _get_or_insert_participant(
        self,
        device_id: DeviceID,
        participant_id: EditicsParticipantID,
        session: EditicsSession,
    ) -> Participant | None:
        """
        Return `None` if the participant exists but is related to another device
        (i.e. impersonation attempt ?)
        """
        participant = session.participants.get(participant_id)
        if participant is not None:
            if participant.device_id == device_id:
                return participant
            else:
                return None

        participant_index = session.next_participant_index
        session.next_participant_index += 1
        participant = session.participants[participant_id] = Participant(
            device_id=device_id,
            index=participant_index,
            participant_id=participant_id,
            connected_at=now(),
        )

        return participant

    @contextmanager
    def participant_listen(
        self,
        auth_info: EditicsAuthInfo,
        document_id: VlobID,
        participant_id: EditicsParticipantID,
    ) -> Generator[EditicsSessionJoinEventStream | EditicsParticipantListenBadOutcome]:
        session = self._get_or_create_session(
            (auth_info.organization_id, auth_info.realm_id, document_id)
        )
        participant = self._get_or_insert_participant(
            auth_info.device_id,
            participant_id,
            session,
        )
        if participant is None:
            yield EditicsParticipantListenBadOutcome.PARTICIPANT_DEVICE_MISMATCH
            return

        # A participant can only have a single SSE connection, we consider any
        # previous one to be a zombie that we can close.
        if participant.sse_channel_sender is not None:
            participant.sse_channel_sender.close()

        sse_channel_sender, channel_receiver = anyio.create_memory_object_stream(
            max_buffer_size=PER_PARTICIPANT_MAX_BUFFER_EVENTS
        )
        participant.sse_channel_sender = sse_channel_sender

        try:
            yield channel_receiver
        finally:
            sse_channel_sender.close()
            # New listen request replaces the previous one, so make sure we are
            # not overwritting the channel of a newer listen!
            if participant.sse_channel_sender is sse_channel_sender:
                participant.sse_channel_sender = None

    async def participant_send(
        self,
        auth_info: EditicsAuthInfo,
        document_id: VlobID,
        participant_id: EditicsParticipantID,
        event: EditicsClientEvent,
    ) -> EditicsServerEvent | EditicsParticipantSendBadOutcome | None:
        session_key = (auth_info.organization_id, auth_info.realm_id, document_id)
        session = self._get_or_create_session(session_key)
        participant = self._get_or_insert_participant(
            auth_info.device_id,
            participant_id,
            session,
        )
        if participant is None:
            return EditicsParticipantSendBadOutcome.PARTICIPANT_DEVICE_MISMATCH

        # Special case for participant creation, achieved by the first `auth`
        if not participant.authed and not isinstance(event, EditicsClientEventAuth):
            return EditicsParticipantSendBadOutcome.PARTICIPANT_NOT_AUTH

        match event:
            case EditicsClientEventAuth():
                if participant.device_id != auth_info.device_id:
                    return EditicsParticipantSendBadOutcome.PARTICIPANT_DEVICE_MISMATCH

                # TODO: should we update `participant.connected_at` when in reconnection ?

                participant_view: bool

                match event.mode:
                    case "edit":
                        match auth_info.realm_role:
                            case RealmRole.READER:
                                return EditicsParticipantSendBadOutcome.READER_MUST_BE_VIEWER

                            case RealmRole.OWNER | RealmRole.MANAGER | RealmRole.CONTRIBUTOR:
                                participant_view = False

                            case unknown:
                                # TODO: Implement `Enum` on `RealmRole` so we can use `assert_never` here
                                # (see https://github.com/Scille/parsec-cloud/issues/12725)
                                assert False, unknown

                    case "view":
                        participant_view = True

                # The handshake is complete: from now on the participant appears
                # in the participants lists (i.e. the `auth` replies and
                # `connectState` broadcasts), before that only its SSE
                # connection was registered in the session.
                participant.authed = ParticipantAuthed(view=participant_view)

                # Viewers are ignored from the auth lock logic (don't take or wait for lock)
                if participant_view:
                    reply = _handshake_auth(session, participant)
                    _push_changes_backlog(session, participant)

                else:
                    # Auth lock logic according to OnlyOffice 9.3.0:
                    # - First non-viewer takes the auth lock
                    # - Next non-viewer gets parked (i.e. receive a `waitAuth` server
                    #   event). It presence is broadcasted (`connectState` server
                    #   event with a `waitAuth=true` field).
                    # - Subsequent non-viewers *do not* get parked: each `connectState`
                    #   server event with `waitAuth=true` would trigger an `unLockDocument`
                    #   event.
                    #   So instead the lock is optimistically considered resolved
                    #   and `auth` is send right away.
                    # - Lock holder releases the lock (`unLockDocument{unlock:true}`
                    #   client event).
                    # - The server send a `auth` event to *all* newcomers, including
                    #   the subsequent non-viewer (for whom it's the second `auth`
                    #   server event they receive, but the client is fine with that).
                    # - Once the auth lock released, newly connecting non-viewers
                    #   always get a `auth` server event right away.

                    session_already_has_parked_participants = any(
                        other_authed.parked
                        for other in session.participants.values()
                        if (
                            other.participant_id != participant.participant_id
                            and (other_authed := other.authed) is not None
                        )
                    )

                    if session.auth_lock_holder:
                        # This parked flag is used
                        participant.authed.parked = True

                    if session.auth_lock_holder and not session_already_has_parked_participants:
                        # An established editor holds the auth lock, must parked
                        # and wait until the holder releases it.
                        assert session.auth_lock_holder is not None
                        lock_holder = session.participants[session.auth_lock_holder]
                        assert (
                            lock_holder.authed is not None
                        )  # Lock gets dropped whenever the participant leaves
                        reply = EditicsServerEventWaitAuth(
                            authLockedBy=EditicsParticipantEntry(
                                id=lock_holder.participant_id,
                                deviceId=lock_holder.device_id,
                                indexUser=lock_holder.index,
                                view=lock_holder.authed.view,  # Expected to be always be `False`
                            )
                        )
                    else:
                        # Lock is not held, or somebedy else has already asked for
                        # it to be released.

                        session_already_has_editors = any(
                            not other_authed.view
                            for other in session.participants.values()
                            if (
                                other.participant_id != participant.participant_id
                                and (other_authed := other.authed) is not None
                            )
                        )
                        if not session_already_has_editors:
                            # Since are the first non-viewer connecting the session,
                            # we can take the auth lock.
                            session.auth_lock_holder = participant.participant_id

                        reply = _handshake_auth(session, participant)
                        _push_changes_backlog(session, participant)

                broadcasted_event = EditicsServerEventConnectState(
                    # TODO: have a single function that handles timestamp generation,
                    #       and ensure time is monotonic and have the expected time
                    #       granularity (to ms ? to second ?).
                    participantsTimestamp=now(),
                    participants=_participant_entries(session),
                    # This `waitAuth` flag triggers the lock holder to release it.
                    waitAuth=isinstance(reply, EditicsServerEventWaitAuth),
                )
                for other_participant in session.participants.values():
                    if other_participant.participant_id == participant.participant_id:
                        continue
                    if other_participant.sse_channel_sender:
                        # TODO: close the SSE if the channel is full
                        other_participant.sse_channel_sender.send_nowait(broadcasted_event)

                return reply

            case EditicsClientEventGetMessages():
                return EditicsServerEventMessage(
                    messages=[
                        MessageRecord(
                            time=m.time_ms,
                            author=(m.author_participant_id, m.author_device_id),
                            encryptedMessage=m.encrypted_message,
                        )
                        for m in session.chat_messages
                    ],
                )

            case EditicsClientEventMessage():
                message_record = MessageRecord(
                    time=now(),
                    author=(participant.participant_id, participant.device_id),
                    encryptedMessage=event.encryptedMessage,
                )
                session.chat_messages.append(
                    ChatMessage(
                        time_ms=message_record.time,
                        author_participant_id=participant.participant_id,
                        author_device_id=participant.device_id,
                        encrypted_message=event.encryptedMessage,
                    ),
                )
                # Message is broadcasted to everyone (including the author), we
                # broadcast to the auth in the response since it is faster.
                broadcasted_event = EditicsServerEventMessage(messages=[message_record])
                for other_participant in session.participants.values():
                    if other_participant.participant_id == participant.participant_id:
                        continue
                    if other_participant.sse_channel_sender:
                        # TODO: close the SSE if the channel is full
                        other_participant.sse_channel_sender.send_nowait(broadcasted_event)
                return broadcasted_event

            case EditicsClientEventUnLockDocument():
                reply = None
                # `isSave` asks to cancel an in-progress save: release the save
                # lock and acknowledge with the cancellation sentinel values.
                if event.isSave and session.save_lock_holder == participant.participant_id:
                    session.save_lock_holder = None
                    reply = EditicsServerEventUnSaveLock(index=-1, time=-1, syncChangesIndex=-1)
                # `releaseLocks` asks to release the region locks held by the
                # participant: the other participants are notified with a
                # `releaseLock` event (RFC §2.2).
                if event.releaseLocks:
                    released_locks = _release_participant_locks(session, participant)
                    if released_locks:
                        broadcasted_event = EditicsServerEventReleaseLock(locks=released_locks)
                        for other_participant in session.participants.values():
                            if other_participant.participant_id == participant.participant_id:
                                continue
                            if other_participant.sse_channel_sender:
                                # TODO: close the SSE if the channel is full
                                other_participant.sse_channel_sender.send_nowait(broadcasted_event)
                # Note there is no holder check: with the OnlyOffice 9.3.0
                # optimistic resolution (see the `auth` event handling) the auth
                # lock can already be considered released when the holder sends
                # this event, while participants are still queued waiting for
                # their `auth`.
                if event.unlock:
                    session.auth_lock_holder = None
                    for other in session.participants.values():
                        if (other_authed := other.authed) and other_authed.parked:
                            other_authed.parked = False
                            if other.sse_channel_sender:
                                # TODO: close the SSE if the channel is full
                                #
                                # The `auth` reply is pushed before the changes
                                # backlog, unlike the RFC §2.2 wire ordering:
                                # the client holds the SSE events back until
                                # the `auth` handshake is delivered (see
                                # `client/editics/collaborative.ts`), and it
                                # needs the participant map it carries to
                                # translate the backlog's `user` fields.
                                other.sse_channel_sender.send_nowait(
                                    _handshake_auth(session, other)
                                )
                                _push_changes_backlog(session, other)
                return reply

            case EditicsClientEventGetLock():
                # Region locks (RFC §2.2): try to acquire the requested blocks for
                # the requester (the blocks already held, possibly by the
                # requester itself, are left as-is), then reply with the resulting
                # lock table. The other participants are broadcasted the same
                # table so they learn about the new locks.
                for block in event.block:
                    key = _block_key(block)
                    if key not in session.region_locks:
                        session.region_locks[key] = LockRecord(
                            time=now(),
                            holder=(participant.participant_id, participant.device_id),
                            block=block,
                        )
                reply = EditicsServerEventGetLock(locks=session.region_locks)
                for other_participant in session.participants.values():
                    if other_participant.participant_id == participant.participant_id:
                        continue
                    if other_participant.sse_channel_sender:
                        # TODO: close the SSE if the channel is full
                        other_participant.sse_channel_sender.send_nowait(reply)
                return reply

            case EditicsClientEventAuthChangesAck():
                # The newcomer acknowledges the changes backlog (delivered as a
                # single chunk in step 1, so there is nothing to track here).
                return None

            case EditicsClientEventClose():
                # Voluntary leave (unlike `unLockDocument` which keeps the connection
                # alive): the participant is removed from the session and the locks it
                # holds are released.
                #
                # Note the leaving participant's SSE channel is left as-is: the client
                # closes it right after the `close` (and it could not receive events
                # anymore anyway, given broadcasts only target the participants present
                # in the session).

                del session.participants[participant_id]

                # Release the save lock if the leaving participant held it (no `unSaveLock`
                # is sent: the only participant it is relevant to is the one leaving!).
                if session.save_lock_holder == participant_id:
                    session.save_lock_holder = None

                # Release the region locks if the leaving participant held some: the
                # other participants are notified with a `releaseLock` event (RFC §2.2).
                released_locks = _release_participant_locks(session, participant)
                if released_locks:
                    broadcasted_event = EditicsServerEventReleaseLock(locks=released_locks)
                    for other in session.participants.values():
                        if other.sse_channel_sender:
                            # TODO: close the SSE if the channel is full
                            other.sse_channel_sender.send_nowait(broadcasted_event)

                # Release the auth lock if the leaving participant held it: the
                # participants parked behind the lock can then complete their
                # handshake (like for a `unLockDocument{unlock:true}`, RFC §2.2).
                if session.auth_lock_holder == participant_id:
                    session.auth_lock_holder = None
                    for other in session.participants.values():
                        if (other_authed := other.authed) and other_authed.parked:
                            other_authed.parked = False
                            if other.sse_channel_sender:
                                # The `auth` reply is pushed before the changes
                                # backlog, unlike the RFC §2.2 wire ordering: the
                                # client holds the SSE events back until the
                                # `auth` handshake is delivered (see
                                # `client/editics/collaborative.ts`), and it
                                # needs the participant map it carries to
                                # translate the backlog's `user` fields.
                                other.sse_channel_sender.send_nowait(
                                    _handshake_auth(session, other)
                                )
                                _push_changes_backlog(session, other)

                # Broadcast the new participant list to the remaining participants
                # (including the ones just un-parked, like any other participant).
                broadcasted_event = EditicsServerEventConnectState(
                    participantsTimestamp=now(),
                    participants=_participant_entries(session),
                    waitAuth=session.auth_lock_holder is not None,
                )
                for other in session.participants.values():
                    if other.sse_channel_sender:
                        # TODO: close the SSE if the channel is full
                        other.sse_channel_sender.send_nowait(broadcasted_event)

                # The session only lives through its participants: destroy it once the
                # last one is gone (a future participant would recreate it from scratch
                # anyway, given a participant always loads the document from the vlob
                # and never from the session).
                if not session.participants:
                    del self._sessions[session_key]

                # Fire-and-forget: nothing left to reply to the leaving client.
                return None

            case EditicsClientEventIsSaveLock():
                # Try to take the save lock.
                # Lock is denied as long as the client's `syncChangesIndex` is
                # desynchronized.
                # Note `syncChangesIndex == 0` means the client has just joined
                # the session and hasn't loaded any changes, and hence is never
                # considered desynchronized.
                desynchronized = event.syncChangesIndex not in (0, session.sync_changes_index)
                if not desynchronized and session.save_lock_holder in (
                    None,
                    participant.participant_id,
                ):
                    session.save_lock_holder = participant.participant_id
                    return EditicsServerEventSaveLock(lockedBySomebodyElse=False)
                else:
                    return EditicsServerEventSaveLock(lockedBySomebodyElse=True)

            case EditicsClientEventSaveChanges():
                # Saving requires the save lock: OnlyOffice (re)acquires it at
                # the start of `saveChanges`, so do the same as a safety net.
                if session.save_lock_holder is None:
                    session.save_lock_holder = participant.participant_id
                elif session.save_lock_holder != participant.participant_id:
                    # Another participant holds the save lock: discard the event,
                    # like OnlyOffice does (should not happen given the client
                    # is expected to acquire the lock with `isSaveLock` first).
                    logger.warning(
                        "Discarding `saveChanges` event: save lock held by another participant"
                    )
                    return None

                # UNDO support: truncate the change history from `deleteIndex`
                # (OnlyOffice semantics: only honored on the first chunk of the
                # save, `null`/`-1` meaning no truncation).
                delete_index = -1
                if event.startSaveChanges and event.deleteIndex is not None:
                    delete_index = event.deleteIndex
                    if delete_index != -1 and delete_index < session.sync_changes_index:
                        del session.changes[delete_index:]
                        session.sync_changes_index = delete_index

                # Store the new changes (one entry per change fragment, i.e.
                # OnlyOffice `puckerIndex += len(encryptedChanges)`)
                start_index = session.sync_changes_index
                changes_time = now()
                session.changes.extend(
                    StoredChange(
                        time_ms=changes_time,
                        author_participant_id=participant.participant_id,
                        author_device_id=participant.device_id,
                        encrypted_change=encrypted_change,
                    )
                    for encrypted_change in event.encryptedChanges
                )
                session.sync_changes_index += len(event.encryptedChanges)
                # Save point for the saver (OnlyOffice semantics: only the first
                # chunk of a non-truncating save establishes a new save point)
                changes_index = start_index if delete_index == -1 and event.startSaveChanges else -1

                # `releaseLocks` asks to release the region locks held by the
                # saver after the save: the released blocks are carried in the
                # broadcast so the other participants update their lock table
                # (RFC §2.2).
                released_locks = (
                    _release_participant_locks(session, participant) if event.releaseLocks else []
                )

                # Broadcast the changes to the other participants (the saver gets
                # `unSaveLock`/`savePartChanges` replies instead)
                broadcasted_event = EditicsServerEventSaveChanges(
                    changes=[
                        SaveChangeRecord(
                            time=changes_time,
                            author=(participant.participant_id, participant.device_id),
                            encryptedChange=encrypted_change,
                        )
                        for encrypted_change in event.encryptedChanges
                    ],
                    changesIndex=session.sync_changes_index,
                    syncChangesIndex=session.sync_changes_index,
                    endSaveChanges=event.endSaveChanges,
                    excel_info=event.excel_info,
                    encryptedCursor=event.encryptedCursor,
                    locks=released_locks,
                )
                for other_participant in session.participants.values():
                    if other_participant.participant_id == participant.participant_id:
                        continue
                    if other_participant.sse_channel_sender:
                        # TODO: close the SSE if the channel is full
                        other_participant.sse_channel_sender.send_nowait(broadcasted_event)

                if event.endSaveChanges:
                    # Save complete: release the save lock and reply to the saver
                    # with its new save point (RFC §2.2).
                    if session.save_lock_holder == participant.participant_id:
                        session.save_lock_holder = None
                    return EditicsServerEventUnSaveLock(
                        index=changes_index,
                        time=changes_time,
                        syncChangesIndex=session.sync_changes_index,
                    )
                else:
                    # Intermediate chunk of a save too big for a single event:
                    # acknowledge it and wait for the next one.
                    return EditicsServerEventSavePartChanges(
                        changesIndex=changes_index,
                        syncChangesIndex=session.sync_changes_index,
                    )

            case EditicsClientEventUnSaveLock():
                # Cancel the in-progress save: release the save lock and reply
                # with the cancellation sentinel values (RFC §2.2).
                if session.save_lock_holder == participant.participant_id:
                    session.save_lock_holder = None
                return EditicsServerEventUnSaveLock(index=-1, time=-1, syncChangesIndex=-1)

            case EditicsClientEventCursor():
                # Cursor messages are ephemeral (i.e. not part of the session
                # history like chat messages) and are only broadcasted to the
                # other participants.
                broadcasted_event = EditicsServerEventCursor(
                    messages=[
                        CursorRecord(
                            time=now(),
                            author=(participant.participant_id, participant.device_id),
                            encryptedCursor=event.encryptedCursor,
                        )
                    ],
                )
                for other_participant in session.participants.values():
                    if other_participant.participant_id == participant.participant_id:
                        continue
                    if other_participant.sse_channel_sender:
                        # TODO: close the SSE if the channel is full
                        other_participant.sse_channel_sender.send_nowait(broadcasted_event)
                return None

            case _:
                raise NotImplementedError(event)  # TODO
