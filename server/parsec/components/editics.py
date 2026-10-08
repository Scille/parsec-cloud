# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

import time
from collections.abc import AsyncGenerator, Generator
from contextlib import asynccontextmanager, contextmanager
from dataclasses import dataclass, field
from enum import auto
from uuid import UUID, uuid4

import anyio
from anyio.streams.memory import MemoryObjectReceiveStream, MemoryObjectSendStream

from parsec._parsec import DeviceID, OrganizationID, VlobID
from parsec.components.auth import EditicsAuthInfo
from parsec.config import BackendConfig
from parsec.editics_protocol import (
    EditicsProtocolClientEvent,
    EditicsProtocolClientEventAuth,
    EditicsProtocolClientEventGetMessages,
    EditicsProtocolClientEventUnLockDocument,
    EditicsProtocolIndexUser,
    EditicsProtocolParticipantEntry,
    EditicsProtocolParticipantID,
    EditicsProtocolServerEvent,
    EditicsProtocolServerEventAuth,
    EditicsProtocolServerEventConnectState,
    EditicsProtocolServerEventMessage,
    EditicsProtocolServerEventWaitAuth,
    MessageRecord,
)
from parsec.logging import get_logger
from parsec.types import BadOutcomeEnum

logger = get_logger()

PER_PARTICIPANT_MAX_BUFFER_EVENTS = 100
type EditicsSessionJoinEventStream = MemoryObjectReceiveStream[EditicsProtocolServerEvent | None]


@dataclass(slots=True)
class ChatMessage:
    """One chat message kept in the session history."""

    time_ms: int
    author_index_user: int
    key_index: int
    encrypted_message: bytes


@dataclass(slots=True)
class Participant:
    """
    Participant represents a single OnlyOffice editor connected to a document edition session.

    - A device can have multiple participants.
    - Participant is created by having a client send a `auth` event.
    - Participant survives SSE disconnection: instead it is only destroyed when the
      client timeout or send a `close` event.
    """

    device_id: DeviceID
    index: EditicsProtocolIndexUser
    # In OnlyOffice participant ID is  `<device ID> + <participant index>`,
    # however here its is an arbitraty UUID controlled by the client.
    # This is because in OnlyOffice this ID is provided in the server `auth` event,
    # while here we must have it when we do the SSE connection (which must occur
    # before the HTTP `auth` request is done!).
    participant_id: EditicsProtocolParticipantID
    # TODO: does this correspond to the very first connection of the participant or to its last (re)connection ?
    connected_at: int  # ms timestamp of the join
    # `None` if no SSE connection is listening for the participant.
    # At most one SSE connection is possible for a given participant (any older
    # one is considered as a zombie and gets closed).
    sse_channel_sender: MemoryObjectSendStream[EditicsProtocolServerEvent] | None = None
    # Joined while the auth lock was held, the participant won't receive its
    # `auth` server event until the lock is released.
    parked: bool = False


@dataclass(slots=True)
class EditicsSession:
    """
    In-memory state of a document edition session
    """

    session_id: UUID = field(default_factory=uuid4)
    next_participant_index: int = 1  # monotonic, starts at 1
    participants: dict[EditicsProtocolParticipantID, Participant] = field(default_factory=dict)
    chat_messages: list[ChatMessage] = field(default_factory=list)
    # At creation, the session is in single-editor mode: the first participant held
    # the auth lock.
    # Participants joining while this lock is held are parked (i.e. they receive
    # `waitAuth` from the server) until the first participant release it (i.e. send
    # `unLockDocument{unlock:true}`) which switch the session into co-editing mode.
    auth_lock_holder: EditicsProtocolIndexUser | None = None

    # initial_version: int
    # latest_allowed_version: int
    # # Monotonic non-decreasing timestamp (ms) of the last `connectState`
    # # broadcast (todo step_1 §6.1 V-A3). Bumped on every participant-set change.
    # participants_timestamp: int = 0
    # # indexUser -> deviceId
    # participants: dict[IndexUser, DeviceID] = field(default_factory=dict)
    # # participant_uuid (client-generated) -> SSE channel
    # connections: dict[UUID, EditicsSseChannel] = field(default_factory=dict)
    # # --- step 1 additions ---
    # # Monotonic change counter (= OnlyOffice `puckerIndex`).
    # sync_changes_index: int = 0
    # # Ordered change history (1-based OnlyOffice index = position + 1).
    # changes: list[StoredChange] = field(default_factory=list)
    # # Chat history (delivered to newcomers on join).
    # messages: list[ChatMessage] = field(default_factory=list)
    # # Save lock: the participant index currently holding it, or None.
    # save_lock_holder: IndexUser | None = None
    # # The participant that just released the save lock (one-round only, §6.9):
    # # only this participant may send `saveDone` to bump `latest_allowed_version`.
    # last_save_lock_holder: IndexUser | None = None
    # # In-flight multi-chunk save buffer for the current lock holder, or None.
    # in_flight_save: InFlightSave | None = None
    # # Region locks, keyed by the JSON-serialized block descriptor.
    # region_locks: dict[str, RegionLock] = field(default_factory=dict)


def _participant_entries(session: EditicsSession) -> list[EditicsProtocolParticipantEntry]:
    return [
        EditicsProtocolParticipantEntry(
            id=participant.participant_id,
            indexUser=participant.index,
            deviceId=participant.device_id,
            view=False,
        )
        for participant in session.participants.values()
    ]


class EditicsParticipantListenBadOutcome(BadOutcomeEnum):
    PARTICIPANT_DEVICE_MISMATCH = auto()


class EditicsParticipantSendBadOutcome(BadOutcomeEnum):
    # First event must be `auth`
    PARTICIPANT_NOT_AUTH = auto()
    # Subsequent `auth` are reconnection, and a device cannot do impersonation
    # by reconnecting with a participant ID related to another device!
    PARTICIPANT_DEVICE_MISMATCH = auto()


class BaseEditicsComponent:
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
            session = self._sessions[key] = EditicsSession()
            return session

    def _get_or_insert_participant(
        self,
        device_id: DeviceID,
        participant_id: EditicsProtocolParticipantID,
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
            connected_at=int(time.time() * 1000),
        )

        return participant

    @asynccontextmanager
    async def sse_api_session_listen(
        self,
        organization_id: OrganizationID,
        device_id: DeviceID,
        participant_id: EditicsProtocolParticipantID,
        realm_id: VlobID,
        document_id: VlobID,
    ) -> AsyncGenerator[EditicsSessionJoinEventStream | EditicsJoinSessionBadOutcome]:
        session = self._get_or_create_session((organization_id, realm_id, document_id))
        participant = session.get_or_insert_participant(participant_id, device_id)

        channel_sender, channel_receiver = anyio.create_memory_object_stream(
            max_buffer_size=PER_PARTICIPANT_MAX_BUFFER_EVENTS
        )

        assert participant_id not in session.participants  # TODO: error handling
        participant = Participant(
            index=session.next_participant_index,
            device_id=device_id,
            connected_at=int(time.time() * 1000),
            sse_channel_sender=channel_sender,
            parked=False,
        )
        session.next_participant_index += 1
        session.participants[participant_id] = participant

        if session.auth_lock_holder is None:
            # No auth lock held: the newcomer completes its handshake right
            # away. If it is also the first participant, it becomes the single
            # editor and takes the auth lock (RFC §6.2).
            if len(session.participants) == 1:
                session.auth_lock_holder = participant.index
            self._send_auth(session, participant)
        else:
            # An established editor holds the auth lock: the newcomer is parked
            # until the holder releases it.
            participant.parked = True

        if len(session.participants) > 1:
            # The participant set changed: broadcast it to everyone, the newcomer
            # included (this is how it discovers the current participants before
            # receiving its `waitAuth`/`auth` event).
            self._broadcast_connect_state(session)

        try:
            yield channel_receiver

        finally:
            del session.participants[participant_id]
            # Never leave participants parked forever: if the auth lock holder
            # leaves without releasing it, release it on its behalf.
            if session.auth_lock_holder == participant.index:
                self._release_auth_lock(session)
            if session.participants:
                self._broadcast_connect_state(session)

    # --- Session events ------------------------------------------------------

    # def _send(self, participant: Participant, event: EditicsProtocolServerEvent) -> None:
    #     try:
    #         participant.sse_channel_sender.send_nowait(event)
    #     except anyio.BrokenResourceError:
    #         # The participant's SSE stream is already closed (e.g. it left while
    #         # we were broadcasting), nothing to do.
    #         pass

    # def _send_auth(self, session: EditicsSession, participant: Participant) -> None:
    #     """Send the `auth` (s→c) event completing a participant's handshake."""
    #     self._send(
    #         participant,
    #         EditicsProtocolServerEventAuth(
    #             participants=_participant_entries(session),
    #             indexUser=participant.index,
    #             sessionId=session.session_id.hex,
    #             sessionTimeConnect=participant.connected_at,
    #         ),
    #     )

    # def _broadcast_connect_state(self, session: EditicsSession) -> None:
    #     """Broadcast the updated participant set (todo step_1 §6.1)."""
    #     event = EditicsProtocolServerEventConnectState(
    #         participantsTimestamp=int(time.time() * 1000),
    #         participants=_participant_entries(session),
    #         waitAuth=session.auth_lock_holder is not None,
    #     )
    #     for participant in session.participants.values():
    #         self._send(participant, event)

    # def _release_auth_lock(self, session: EditicsSession) -> None:
    #     """Release the auth lock and let the parked participants complete their
    #     handshake (todo step_1 §6.2)."""
    #     session.auth_lock_holder = None
    #     for participant in session.participants.values():
    #         if participant.parked:
    #             participant.parked = False
    #             self._send_auth(session, participant)

    @contextmanager
    def participant_listen(
        self,
        auth_info: EditicsAuthInfo,
        document_id: VlobID,
        participant_id: EditicsProtocolParticipantID,
    ) -> Generator[EditicsSessionJoinEventStream | EditicsParticipantListenBadOutcome]:
        session = self._get_or_create_session(
            (auth_info.organization_id, auth_info.realm_id, document_id)
        )
        participant = self._get_or_insert_participant(auth_info.device_id, participant_id, session)
        if participant is None:
            yield EditicsParticipantListenBadOutcome.PARTICIPANT_DEVICE_MISMATCH
            return

        # A participant can only have a single SSE connection, we consider any
        # previous one to be a zombie that we can close.
        if participant.sse_channel_sender is not None:
            participant.sse_channel_sender.close()

        participant.sse_channel_sender, channel_receiver = anyio.create_memory_object_stream(
            max_buffer_size=PER_PARTICIPANT_MAX_BUFFER_EVENTS
        )

        try:
            yield channel_receiver
        finally:
            participant.sse_channel_sender.close()
            participant.sse_channel_sender = None

    async def participant_send(
        self,
        auth_info: EditicsAuthInfo,
        document_id: VlobID,
        participant_id: EditicsProtocolParticipantID,
        event: EditicsProtocolClientEvent,
    ) -> EditicsProtocolServerEvent | EditicsParticipantSendBadOutcome | None:
        session = self._get_or_create_session(
            (auth_info.organization_id, auth_info.realm_id, document_id)
        )
        participant = self._get_or_insert_participant(auth_info.device_id, participant_id, session)
        if participant is None:
            return EditicsParticipantSendBadOutcome.PARTICIPANT_DEVICE_MISMATCH

        # Participant is actually added to the session by client's `auth` event
        participant = session.participants.get(participant_id)

        # Special case for participant creation, achieved by the first `auth`
        if participant is None:
            if isinstance(event, EditicsProtocolClientEventAuth):
                participant_index = session.next_participant_index
                session.next_participant_index += 1
                participant = session.participants[participant_id] = Participant(
                    device_id=auth_info.device_id,
                    index=participant_index,
                    participant_id=participant_id,
                    connected_at=int(time.time() * 1000),
                )

            else:
                # `auth` must be the first client event
                return EditicsParticipantSendBadOutcome.PARTICIPANT_NOT_AUTH

        match event:
            case EditicsProtocolClientEventAuth():
                if participant.device_id != auth_info.device_id:
                    return EditicsParticipantSendBadOutcome.PARTICIPANT_DEVICE_MISMATCH

                # TODO: should we update `participant.connected_at` when in reconnection ?

                if session.auth_lock_holder is None:
                    # No auth lock held: the newcomer completes its handshake right
                    # away. If it is also the first participant, it becomes the single
                    # editor and takes the auth lock (RFC §6.2).
                    if len(session.participants) == 1:
                        session.auth_lock_holder = participant.index
                else:
                    # An established editor holds the auth lock: the newcomer is parked
                    # until the holder releases it.
                    participant.parked = True

                broadcasted_event = EditicsProtocolServerEventConnectState(
                    # TODO: have a single function that handles timestamp generation,
                    #       and ensure time is monotonic and have the expected time
                    #       granularity (to ms ? to second ?).
                    participantsTimestamp=int(time.time() * 1000),
                    participants=_participant_entries(session),
                    waitAuth=session.auth_lock_holder is not None,
                )
                for other_participant in session.participants.values():
                    if other_participant.participant_id == participant.participant_id:
                        continue
                    if other_participant.sse_channel_sender:
                        # TODO: close the SSE if the channel is full
                        other_participant.sse_channel_sender.send_nowait(broadcasted_event)

                # Send `waitAuth` if another participant holds the auth lock (i.e.
                # the newcomer must wait until the other participant sends a
                # `unLockDocument{unlock:true}`)
                if session.auth_lock_holder is None:
                    return EditicsProtocolServerEventAuth(
                        participants=_participant_entries(session),
                        indexUser=participant.index,
                        participantTimeConnect=participant.connected_at,
                    )

                else:
                    return EditicsProtocolServerEventWaitAuth(authLockedBy=session.auth_lock_holder)

            case EditicsProtocolClientEventGetMessages():
                return EditicsProtocolServerEventMessage(
                    messages=[
                        MessageRecord(
                            time=m.time_ms,
                            authorIndexUser=m.author_index_user,
                            encryptedMessage=m.encrypted_message,
                        )
                        for m in session.chat_messages
                    ],
                )

            case EditicsProtocolClientEventUnLockDocument():
                # Only the auth lock is handled for now: `isSave`, `deleteIndex`
                # and `releaseLocks` cover the save & region lock systems.
                if event.unlock and session.auth_lock_holder == participant.index:
                    self._release_auth_lock(session)
                return None

            case _:
                raise NotImplementedError(event)  # TODO
