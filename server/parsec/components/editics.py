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
from anyio import BrokenResourceError, ClosedResourceError, WouldBlock
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
    EditicsParticipantEntry,
    EditicsParticipantID,
    EditicsParticipantIndex,
    EditicsServerEvent,
    EditicsServerEventAuth,
    EditicsServerEventAuthChanges,
    EditicsServerEventConnectState,
    EditicsServerEventCursor,
    EditicsServerEventDisconnectReason,
    EditicsServerEventError,
    EditicsServerEventGetLock,
    EditicsServerEventMessage,
    EditicsServerEventReleaseLock,
    EditicsServerEventSaveChanges,
    EditicsServerEventSaveLock,
    EditicsServerEventSavePartChanges,
    EditicsServerEventUnSaveLock,
    EditicsServerEventWaitAuth,
    EditicsExcelRecalcIndex,
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
    time_ms: int
    author_participant_id: EditicsParticipantID
    author_device_id: DeviceID
    encrypted_message: tuple[int, bytes]


@dataclass(slots=True)
class StoredChange:
    time_ms: int
    author_participant_id: EditicsParticipantID
    author_device_id: DeviceID
    # Format: (<Key index>, <base64(ciphertext)>)
    encrypted_change: tuple[int, bytes]


@dataclass(slots=True)
class ParticipantAuthed:
    # Joined while the auth lock was held: the participant is queued to receive
    # its `auth` server event when the lock is released.
    # Note this also marks newcomers that got their `auth` reply right away
    # (i.e. joining as a third+ non-viewer while the lock is held): the OnlyOffice
    # server completes the handshake of *all* newcomers waiting on the lock when
    # it is released, so they receive a second `auth` server event (which the
    # client tolerates).
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
    index: EditicsParticipantIndex
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
    # The session's `sync_changes_index` when the participant lost its connection:
    # used to refuse a restoration when the document was modified while the
    # participant was away.
    disconnected_changes_index: int | None = None

    # Whether the participant currently has a live SSE connection: a participant
    # whose connection dropped is hidden from the participants lists and loses
    # the locks it holds, but stays in the session so the connection can be
    # restored (an `auth` client event carrying reconnection info, see the
    # `disconnection` records).
    # Derived from `sse_channel_sender` (at most one SSE connection is possible
    # for a given participant, any older one is a zombie that gets replaced):
    # the participant is connected from the moment its SSE listen is registered
    # (i.e. even before its `auth` with reconnection info is processed) until
    # the connection drops.
    @property
    def connected(self) -> bool:
        return self.sse_channel_sender is not None

    def send_event(self, event: EditicsServerEvent) -> None:
        """
        Push `event` on the participant's SSE channel (no-op if it has no
        live SSE connection).
        """
        if self.sse_channel_sender is None:
            return
        try:
            self.sse_channel_sender.send_nowait(event)
        except WouldBlock:
            # The channel is full: close it so the coroutine serving the SSE
            # connection ends.
            logger.warning(
                f"Closing the SSE connection of participant {self.participant_id}: "
                "its event channel is full"
            )
            self.sse_channel_sender.close()
        except (BrokenResourceError, ClosedResourceError):
            # The channel is already closed (e.g. it was full and closed by a
            # previous event, or the SSE connection teardown is in progress).
            pass


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
    # The session is in single-editor mode as long as this is set: the OnlyOffice
    # server has no such persistent state, but modeling it this way reproduces its
    # parking rule (a fresh non-viewer joining as the 2nd non-viewer of the session
    # gets parked behind the lock, see the `auth` handling).
    # Participants joining in single-editor mode are parked (i.e. they receive a
    # `waitAuth` from the server) until the single editor releases the lock (i.e.
    # sends `unLockDocument{unlock:true}`, which its client does when a
    # `connectState` switches it to co-editing mode), which switches the session
    # into co-editing mode.
    auth_lock_holder: EditicsParticipantID | None = None
    # Monotonic change counter (= OnlyOffice `puckerIndex`): advances on every
    # stored change fragment, and is only rewound by an undo (i.e. a `saveChanges`
    # carrying a `deleteIndex`).
    sync_changes_index: int = 0
    # Ordered change history, can be truncated from a given index on undo.
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
        # Mallory both connect before either has authed), and participants
        # whose connection dropped are hidden until they restore it.
        for participant in session.participants.values()
        if participant.authed and participant.connected
    ]


def _block_key(block: Any) -> str:
    """Key identifying a region lock's block (RFC §2.2).

    OnlyOffice keys the lock table by the block id itself for the document
    editor (a plain string), and by the block `guid` for the other editors.
    """
    return block if isinstance(block, str) else block["guid"]


# OnlyOffice lock element types (`c_oAscLockTypeElem`): only the range locks
# are subject to the lock recalculation, and the structural column/row locks
# (`c_oAscLockTypeElemSubType`: InsertColumns=2 / InsertRows=4) are the
# geometry-change markers themselves, so they are left as-is.
_LOCK_RECALC_RANGE_TYPE = 1
_LOCK_RECALC_SKIP_SUB_TYPES = (2, 4)  # InsertColumns, InsertRows


def _recalculate_locks(
    session: EditicsSession,
    saver_participant_id: EditicsParticipantID,
    index_cols: EditicsExcelRecalcIndex | None,
    index_rows: EditicsExcelRecalcIndex | None,
) -> None:
    """Recalculate the stored locked ranges of the other participants after a
    spreadsheet structural change, per the OnlyOffice server's
    `_recalcLockArray` (see the `saveChanges` handling).

    The recalc records (see `EditicsExcelRecalcIndexElement`) carry the
    inserted/deleted columns and rows per sheet: an insert record at position
    `P` with count `N` shifts the coordinates at or after `P` by `+N`, a
    remove record by `-N` (a coordinate inside the removed range stops at the
    position just before it). The records flagged as save-index markers are
    skipped: they are the markers a client re-emits to acknowledge the
    structural changes it just applied, not new geometry changes (record 53:
    Alice's follow-up save re-sends Bob's insertion as a marker).

    Only the range locks are shifted (the structural column/row locks are the
    change markers themselves, and the other editors' blocks are not
    coordinates), and only those held by the other participants (the saver's
    own locks are about to be released, or kept by its client which knows the
    new geometry).
    """
    for axes, changes in ((("c1", "c2"), index_cols), (("r1", "r2"), index_rows)):
        if changes is None:
            continue
        for sheet_id, sheet_changes in changes.items():
            real_changes = [change for change in sheet_changes.elements if not change.is_save_index]
            if not real_changes:
                continue
            for lock in session.region_locks.values():
                block = lock.block
                if (
                    not isinstance(block, dict)
                    or block.get("type") != _LOCK_RECALC_RANGE_TYPE
                    or block.get("subType") in _LOCK_RECALC_SKIP_SUB_TYPES
                    or str(block.get("sheetId")) != sheet_id
                    or lock.holder[0] == saver_participant_id
                ):
                    continue
                # Note the shift is applied record by record (in list order):
                # each record describes a single geometry change, applied on
                # top of the previous ones.
                range_or_object_id = block["rangeOrObjectId"]
                for change in real_changes:
                    if change.recalc_type == 1:  # Insert (`RecalcIndexAdd`)
                        for axis in axes:
                            if range_or_object_id[axis] >= change.position:
                                range_or_object_id[axis] += change.count
                    else:  # Delete (`RecalcIndexRemove`)
                        for axis in axes:
                            if range_or_object_id[axis] >= change.position:
                                range_or_object_id[axis] = max(
                                    change.position - 1, range_or_object_id[axis] - change.count
                                )


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
    if not session.changes:
        return
    participant.send_event(
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
        session_key = (auth_info.organization_id, auth_info.realm_id, document_id)
        session = self._get_or_create_session(session_key)
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
                # The SSE connection dropped (i.e. the transport-level connection
                # was lost): handle the participant's departure. Note a zombie
                # connection replaced by a newer listen doesn't reach this branch.
                self._handle_participant_disconnected(session, participant)

    def _handle_participant_disconnected(
        self,
        session: EditicsSession,
        participant: Participant,
    ) -> None:
        """Handle the participant's SSE connection dropping (i.e. the
        transport-level connection was lost, like a websocket dying without a
        close frame).

        The participant is *not* destroyed: it stays in the session (hidden from
        the participants lists) so the connection can be restored with an `auth`
        client event carrying reconnection info (see the `disconnection` records).
        TODO: a timeout should eventually destroy the participants that don't
        come back (it must be long enough to never fire during the tests, which
        replay the records without their original timing).
        """
        # The participant may have already left the session (e.g. it sent a
        # `close` client event before closing its SSE connection).
        if session.participants.get(participant.participant_id) is not participant:
            return

        # Snapshot of the remaining established editors (i.e. connected
        # participants that completed their handshake and are not waiting for
        # the auth lock) and of the parked participants, before the departure
        # handling below potentially un-parks anyone: a participant un-parked
        # by this departure is not "established" (record 610: Alice's later
        # restoration goes through without being parked behind Bob).
        established_editors = [
            other
            for other in session.participants.values()
            if (
                other.participant_id != participant.participant_id
                and other.connected
                and other.authed is not None
                and not other.authed.parked
                and not other.authed.view
            )
        ]
        remaining_parked = any(
            other_authed.parked
            for other in session.participants.values()
            if (
                other.participant_id != participant.participant_id
                and (other_authed := other.authed) is not None
            )
        )

        participant.disconnected_changes_index = session.sync_changes_index

        # Release the save lock if the participant held it (no `unSaveLock` is
        # sent: the only participant it is relevant to is the one leaving, see
        # record 609 where the save is cancelled so another participant can
        # save).
        if session.save_lock_holder == participant.participant_id:
            session.save_lock_holder = None

        # Release the region locks held by the participant: the records show no
        # `releaseLock` broadcast for a disconnect (record 606: the lock is
        # released silently, the other participants learn about it when they
        # next try to take the lock).
        _release_participant_locks(session, participant)

        # Release the auth lock if the participant held it: the participants
        # parked behind it can then complete their handshake (record 610).
        if session.auth_lock_holder == participant.participant_id:
            session.auth_lock_holder = None
            for other in session.participants.values():
                if (other_authed := other.authed) and other_authed.parked:
                    other_authed.parked = False
                    # The `auth` reply is pushed before the changes backlog,
                    # like for a `unLockDocument{unlock:true}` (see there).
                    other.send_event(_handshake_auth(session, other))
                    _push_changes_backlog(session, other)

        # Back to single-editor mode: when the departure leaves exactly one
        # established editor, it takes the auth lock again. The OnlyOffice server
        # has no such persistent state: this re-take is how our model reproduces
        # its parking rule (record 607: a fresh non-viewer rejoining is parked
        # behind the remaining editor, as it would be behind any single editor).
        if (
            session.auth_lock_holder is None
            and not remaining_parked
            and len(established_editors) == 1
        ):
            session.auth_lock_holder = established_editors[0].participant_id

        # Broadcast the new participant list (without the departed participant)
        # to the remaining ones (records 607/608/609). Like the OnlyOffice server,
        # the broadcast always carries `waitAuth=false`: the flag means "a
        # newcomer got parked behind the auth lock" and is only ever set by the
        # broadcast announcing such a newcomer (see the `auth` handling), not
        # here (record 607: the broadcast carries `false` although the remaining
        # editor just re-took the lock).
        broadcasted_event = EditicsServerEventConnectState(
            participantsTimestamp=now(),
            participants=_participant_entries(session),
            waitAuth=False,
        )
        for other in session.participants.values():
            if other.participant_id == participant.participant_id:
                continue
            other.send_event(broadcasted_event)

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

        # Viewers are not allowed to send editor-only events...
        if (
            (participant_authed := participant.authed) is not None
            and participant_authed.view
            and isinstance(
                event,
                EditicsClientEventGetLock | EditicsClientEventSaveChanges | EditicsClientEventIsSaveLock,
            )
        ):
            # ...and get their SSE connection kicked out for this!
            if participant.sse_channel_sender is not None:
                participant.sse_channel_sender.close()
            return EditicsServerEventDisconnectReason.access_denied()

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

                # Handle transport-level reconnection of a participant.
                # Note we check the provided participant ID. This is expected to
                # be the same value the transport already carries (the participant ID
                # in the listen/send routes URL), but the transport *identifies* the connection
                # while this field is the client's *statement of intent*: the server only
                # restores a participant's session when the client explicitly says so, and
                # echoing the ID lets it validate the client's bookkeeping.
                if participant.authed is not None and event.reconnect == participant_id:
                    # Reconnection is only possible if no modification occured in the meantime
                    if (
                        participant.disconnected_changes_index is not None
                        and session.sync_changes_index != participant.disconnected_changes_index
                    ):
                        # The client should reacts to this event by closing the co-authoring
                        # session (i.e. sending a `close` client event).
                        return EditicsServerEventError(
                            code=4010,
                            description="Restore error. Document modified.",
                        )

                    # Reconnect

                    participant.authed.view = participant_view
                    participant.disconnected_changes_index = None
                    # The OnlyOffice server re-adds a restored participant at the
                    # end of the participants lists (its entry was dropped when
                    # the connection was lost, see the `disconnection` records).
                    del session.participants[participant_id]
                    session.participants[participant_id] = participant
                    # Back to single-editor mode: a restored non-viewer takes the
                    # auth lock when no other established editor remains (record
                    # 612: Alice restores while only a viewer is connected, so
                    # Mallory's later fresh join gets parked behind her, exactly
                    # like if Alice had never disconnected). Note the lock is
                    # left as-is when already held (record 611: Bob took it while
                    # Alice was away): the holder keeps it until its client
                    # switches to co-editing mode, and the participants parked
                    # behind it keep waiting for its release.
                    if (
                        not participant_view
                        and session.auth_lock_holder is None
                        and not any(
                            not other_authed.view
                            for other in session.participants.values()
                            if (
                                other.participant_id != participant.participant_id
                                and other.connected
                                and (other_authed := other.authed) is not None
                                and not other_authed.parked
                            )
                        )
                    ):
                        session.auth_lock_holder = participant.participant_id
                    # Push the lock table (the OnlyOffice server pushes it as a
                    # `getLock` event on restoration) to the restorer and the
                    # non-viewer participants: its restore path re-checks the
                    # blocks of a non-viewer restorer only, and the broadcast
                    # targets the non-viewer connections (records 610/612: Bob
                    # receives it as an editor, but not as a viewer). Then
                    # broadcast the new participant list, and complete with the
                    # `auth` reply: no changes backlog, the participant never
                    # left the session logically so it already has the changes.
                    if not participant_view:
                        lock_table_event = EditicsServerEventGetLock(locks=session.region_locks)
                        for other in session.participants.values():
                            other_authed = other.authed
                            if other.participant_id != participant.participant_id and (
                                other_authed is None or other_authed.view
                            ):
                                continue
                            other.send_event(lock_table_event)
                    # Restored participants don't take part in the auth lock logic
                    # (the OnlyOffice server's `endAuth` only considers *fresh*
                    # connections), so this broadcast always carries `waitAuth=false`
                    # (record 611), even when the auth lock is held: the flag means
                    # "a newcomer got parked behind the auth lock", not "the lock
                    # is held" (see the `auth` handling).
                    broadcasted_event = EditicsServerEventConnectState(
                        participantsTimestamp=now(),
                        participants=_participant_entries(session),
                        waitAuth=False,
                    )
                    for other in session.participants.values():
                        if other.participant_id == participant.participant_id:
                            continue
                        other.send_event(broadcasted_event)
                    return _handshake_auth(session, participant)

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
                    # Auth lock logic according to OnlyOffice 9.3.0 (`endAuth` in
                    # the OnlyOffice server's `DocsCoServer.js`):
                    # - Restored participants (i.e. `auth` with reconnection info,
                    #   see above) never take part in the auth lock logic: they get
                    #   their `auth` reply right away (record 611).
                    # - The server parks a non-viewer newcomer only when it connects
                    #   as exactly the *second* non-viewer of the session (`2 ===
                    #   countNoView`): the newcomer receives a `waitAuth` server
                    #   event, and the lock is taken on the established editor's
                    #   behalf.
                    # - Subsequent non-viewers *do not* get parked (the server-side
                    #   `2 === countNoView` condition no longer holds): like the
                    #   restore case, they get their `auth` reply right away.
                    # - The lock holder releases the lock with an
                    #   `unLockDocument{unlock:true}` client event. The client sends
                    #   it when a `connectState` server event switches it from
                    #   single-editor to co-editing mode (i.e. a participant list
                    #   change leaving two non-viewers or more), *not* because of
                    #   the `connectState`'s `waitAuth` flag, which the OnlyOffice
                    #   client only ever uses for error logging (record 611: the
                    #   holder releases the lock although the broadcast carried
                    #   `waitAuth=false`).
                    # - Once the lock is released, the server completes the handshake
                    #   of *all* newcomers waiting on it: the subsequent non-viewer
                    #   receives a second `auth` server event (the client is fine with
                    #   that), hence the `parked` flag below marking newcomers that
                    #   already got their `auth` reply.

                    session_already_has_parked_participants = any(
                        other_authed.parked
                        for other in session.participants.values()
                        if (
                            other.participant_id != participant.participant_id
                            and other.connected
                            and (other_authed := other.authed) is not None
                        )
                    )

                    if session.auth_lock_holder:
                        # Mark the newcomer as waiting on the auth lock: the holder's
                        # `unLockDocument{unlock:true}` completes its handshake (see
                        # the auth lock logic comment above for why this also marks
                        # newcomers that already got their `auth` reply).
                        participant.authed.parked = True

                    if session.auth_lock_holder and not session_already_has_parked_participants:
                        # An established editor holds the auth lock, the newcomer
                        # must be parked and wait until the holder releases it.
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
                        # Lock is not held, or a newcomer is already parked behind
                        # it (i.e. this newcomer is at least the third non-viewer:
                        # the OnlyOffice server doesn't park it, but it still takes
                        # part in the lock release completion below).

                        session_already_has_editors = any(
                            not other_authed.view
                            for other in session.participants.values()
                            if (
                                other.participant_id != participant.participant_id
                                and other.connected
                                and (other_authed := other.authed) is not None
                            )
                        )
                        if not session_already_has_editors:
                            # We are the first non-viewer connecting to the session,
                            # so it switches to single-editor mode with us as the
                            # auth lock holder.
                            session.auth_lock_holder = participant.participant_id

                        reply = _handshake_auth(session, participant)
                        _push_changes_backlog(session, participant)

                broadcasted_event = EditicsServerEventConnectState(
                    # TODO: have a single function that handles timestamp generation,
                    #       and ensure time is monotonic and have the expected time
                    #       granularity (to ms ? to second ?).
                    participantsTimestamp=now(),
                    participants=_participant_entries(session),
                    # The OnlyOffice server sets this flag only in this broadcast,
                    # and only when the newcomer got parked behind the auth lock
                    # (it is `false` in every other case, e.g. the restore and
                    # disconnection broadcasts). Note the client does not use it
                    # as a trigger: the lock holder releases the lock because of
                    # the co-editing mode transition (see the auth lock logic
                    # comment above), and only ever uses the flag for error logging.
                    waitAuth=isinstance(reply, EditicsServerEventWaitAuth),
                )
                for other_participant in session.participants.values():
                    if other_participant.participant_id == participant.participant_id:
                        continue
                    other_participant.send_event(broadcasted_event)

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
                    other_participant.send_event(broadcasted_event)
                return broadcasted_event

            case EditicsClientEventUnLockDocument():
                reply = None
                # `isSave` asks to cancel an in-progress save: release the save
                # lock and acknowledge with the cancellation sentinel values.
                if event.isSave and session.save_lock_holder == participant.participant_id:
                    session.save_lock_holder = None
                    reply = EditicsServerEventUnSaveLock(index=-1, time=-1, syncChangesIndex=-1)
                # `releaseLocks` asks to release the region locks held by the
                # participant: like the OnlyOffice server, the releasing
                # participant is notified too (a `releaseLock` event carrying
                # its own released descriptors, acknowledging what was
                # released), and the other participants receive the same
                # event broadcast (RFC §2.2).
                if event.releaseLocks:
                    released_locks = _release_participant_locks(session, participant)
                    if released_locks:
                        broadcasted_event = EditicsServerEventReleaseLock(locks=released_locks)
                        for other_participant in session.participants.values():
                            other_participant.send_event(broadcasted_event)
                # Note there is no holder check: the client sends
                # `unLockDocument{unlock:true}` when it switches to co-editing
                # mode (see the auth lock logic in the `auth` handling), which
                # may happen even though nobody holds the auth lock (record 611:
                # Bob sends it while, in the OnlyOffice server's model, the lock
                # was never taken) — in which case this is a no-op, like for the
                # OnlyOffice server's fencing-token check. Conversely, the client
                # may send it after the lock was already released (e.g. a save
                # was in progress when it switched to co-editing mode), while
                # participants are still queued waiting for their `auth`.
                if event.unlock:
                    session.auth_lock_holder = None
                    for other in session.participants.values():
                        if (other_authed := other.authed) and other_authed.parked:
                            other_authed.parked = False
                            # The `auth` reply is pushed before the changes
                            # backlog, unlike the RFC §2.2 wire ordering:
                            # the client holds the SSE events back until
                            # the `auth` handshake is delivered (see
                            # `client/editics/collaborative.ts`), and it
                            # needs the participant map it carries to
                            # translate the backlog's `user` fields.
                            other.send_event(_handshake_auth(session, other))
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
                # The OnlyOffice server broadcasts the lock table to the
                # non-viewer connections only (its `getLock` publish excludes
                # the viewers, record 44: a viewer present in the session
                # doesn't receive the broadcast).
                for other_participant in session.participants.values():
                    other_authed = other_participant.authed
                    if (
                        other_participant.participant_id == participant.participant_id
                        or other_authed is None
                        or other_authed.view
                    ):
                        continue
                    other_participant.send_event(reply)
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

                # Snapshot of the remaining established editors (i.e. connected
                # participants that completed their handshake and are not waiting
                # for the auth lock) and of the parked participants, before the
                # departure handling below potentially un-parks anyone: a
                # participant un-parked by this departure is not "established"
                # (same rationale as for a disconnect, record 610).
                established_editors = [
                    other
                    for other in session.participants.values()
                    if (
                        other.connected
                        and other.authed is not None
                        and not other.authed.parked
                        and not other.authed.view
                    )
                ]
                remaining_parked = any(
                    other_authed.parked
                    for other in session.participants.values()
                    if (other_authed := other.authed) is not None
                )

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
                        other.send_event(broadcasted_event)

                # Release the auth lock if the leaving participant held it: the
                # participants parked behind the lock can then complete their
                # handshake (like for a `unLockDocument{unlock:true}`, RFC §2.2).
                if session.auth_lock_holder == participant_id:
                    session.auth_lock_holder = None
                    for other in session.participants.values():
                        if (other_authed := other.authed) and other_authed.parked:
                            other_authed.parked = False
                            # The `auth` reply is pushed before the changes
                            # backlog, unlike the RFC §2.2 wire ordering: the
                            # client holds the SSE events back until the
                            # `auth` handshake is delivered (see
                            # `client/editics/collaborative.ts`), and it
                            # needs the participant map it carries to
                            # translate the backlog's `user` fields.
                            other.send_event(_handshake_auth(session, other))
                            _push_changes_backlog(session, other)

                # Back to single-editor mode: when the departure leaves exactly
                # one established editor, it takes the auth lock again (like for
                # a disconnect, see `_handle_participant_disconnected`).
                if (
                    session.auth_lock_holder is None
                    and not remaining_parked
                    and len(established_editors) == 1
                ):
                    session.auth_lock_holder = established_editors[0].participant_id

                # Broadcast the new participant list to the remaining participants
                # (including the ones just un-parked, like any other participant).
                # Like the OnlyOffice server, the broadcast always carries
                # `waitAuth=false`: the flag means "a newcomer got parked behind
                # the auth lock" and is only ever set by the broadcast announcing
                # such a newcomer (see the `auth` handling).
                broadcasted_event = EditicsServerEventConnectState(
                    participantsTimestamp=now(),
                    participants=_participant_entries(session),
                    waitAuth=False,
                )
                for other in session.participants.values():
                    other.send_event(broadcasted_event)

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

                # A spreadsheet structural change (columns/rows inserted or
                # deleted) shifts the geometry: like the OnlyOffice server, the
                # stored locked ranges of the OTHER participants are
                # recalculated so they follow the shifted cells (record 56:
                # Alice's B3 block is stored as C3 after Bob inserted a column
                # at A). This is a no-op for the saves without structural
                # change info (`_recalculate_locks` skips the `None` axes).
                if event.endSaveChanges:
                    _recalculate_locks(
                        session,
                        participant.participant_id,
                        index_cols=event.excelAdditionalInfoIndexCols,
                        index_rows=event.excelAdditionalInfoIndexRows,
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
                    excelAdditionalInfoIndexCols=event.excelAdditionalInfoIndexCols,
                    excelAdditionalInfoIndexRows=event.excelAdditionalInfoIndexRows,
                    encryptedCursorAdditionalInfo=event.encryptedCursorAdditionalInfo,
                    locks=released_locks,
                )
                for other_participant in session.participants.values():
                    if other_participant.participant_id == participant.participant_id:
                        continue
                    other_participant.send_event(broadcasted_event)

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
                    other_participant.send_event(broadcasted_event)
                return None

            case _:
                raise NotImplementedError(event)  # TODO
