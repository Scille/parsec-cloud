# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

import time
from collections.abc import AsyncGenerator
from contextlib import asynccontextmanager
from dataclasses import dataclass, field
from enum import auto
from uuid import UUID, uuid4
from anyio.streams.memory import MemoryObjectReceiveStream, MemoryObjectSendStream
import anyio

from parsec._parsec import DeviceID, OrganizationID, VlobID
from parsec.config import BackendConfig
from parsec.editics_protocol import EditicsProtocolClientEvent, EditicsProtocolParticipantEntry, EditicsProtocolServerEvent, EditicsProtocolServerEventAuth
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
    index: int
    device_id: DeviceID
    sse_channel_sender: MemoryObjectSendStream[EditicsProtocolServerEvent]


@dataclass(slots=True)
class EditicsSession:
    """
    In-memory state of a single edition session
    """

    session_id: UUID = field(default_factory=uuid4)
    next_participant_index: int = 1  # monotonic, starts at 1
    participants: dict[UUID, Participant] = field(default_factory=dict)
    chat_messages: list[ChatMessage] = field(default_factory=list)

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
    # # Auth lock: the participant index currently holding it, or None.
    # # Held by the first non-view participant until `unLockDocument{unlock:true}`.
    # auth_lock_holder: IndexUser | None = None
    # # Participants parked behind the auth lock (in join order), waiting for
    # # the holder to release. Each entry is the participant's channel + index.
    # parked: list[tuple[EditicsSseChannel, IndexUser]] = field(default_factory=list)
    # # Region locks, keyed by the JSON-serialized block descriptor.
    # region_locks: dict[str, RegionLock] = field(default_factory=dict)


class EditicsJoinSessionBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    STOPPED = auto()


class EditicsSendInSessionBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    STOPPED = auto()


class BaseEditicsComponent:
    def __init__(self, config: BackendConfig) -> None:
        self._config = config
        self._sessions: dict[tuple[OrganizationID, VlobID, VlobID], EditicsSession] = {}
        # # (workspace_id, vlob_id, participant_uuid) -> channel
        # self._pending: dict[tuple[VlobID, VlobID, UUID], EditicsSseChannel] = {}

    @asynccontextmanager
    async def sse_api_join_session(
        self,
        organization_id: OrganizationID,
        device_id: DeviceID,
        participant_id: UUID,
        realm_id: VlobID,
        document_id: VlobID,
        last_event_id: int | None = None,
    ) -> AsyncGenerator[EditicsSessionJoinEventStream | EditicsJoinSessionBadOutcome]:
        try:
            session = self._sessions[(organization_id, realm_id, document_id)]
        except KeyError:
            session = self._sessions[(organization_id, realm_id, document_id)] = EditicsSession()

        channel_sender, channel_receiver = anyio.create_memory_object_stream(
            max_buffer_size=PER_PARTICIPANT_MAX_BUFFER_EVENTS
        )

        assert participant_id not in session.participants  # TODO: error handling
        participant_index = session.next_participant_index
        session.next_participant_index += 1
        session.participants[participant_id] = Participant(
            index=participant_index,
            device_id=device_id,
            sse_channel_sender=channel_sender,
        )
        try:
            channel_sender.send_nowait(
                EditicsProtocolServerEventAuth(
                    participants=[
                        EditicsProtocolParticipantEntry(
                            indexUser=p.index,
                            deviceId=p.device_id,
                            view=False,
                        ) for p in session.participants.values()
                    ],
                    indexUser=participant_index,
                    sessionId=session.session_id.hex,
                    sessionTimeConnect=int(time.time() * 1000),
                )
            )

            yield channel_receiver

        finally:
            session.participants.pop(participant_id)

    async def api_send_in_session(
        self,
        organization_id: OrganizationID,
        device_id: DeviceID,
        participant_id: UUID,
        realm_id: VlobID,
        document_id: VlobID,
        event: EditicsProtocolClientEvent,
    ) -> EditicsProtocolServerEvent | EditicsSendInSessionBadOutcome | None:
        match event.type:
            case "auth":
                pass
            case "message":
                pass
            case _:
                raise NotImplementedError(event)  # TODO
