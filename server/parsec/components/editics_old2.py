# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

import time
from collections.abc import AsyncGenerator
from contextlib import asynccontextmanager
from dataclasses import dataclass, field
from enum import auto
from typing import Any
from uuid import UUID, uuid4

import anyio
from anyio.streams.memory import MemoryObjectReceiveStream

from parsec._parsec import DeviceID, OrganizationID, VlobID
from parsec.config import BackendConfig
from parsec.editics_protocol import (
    EditicsProtocolClientEvent,
    EditicsProtocolClientEventAuth,
    EditicsProtocolParticipantEntry,
    EditicsProtocolServerEvent,
    EditicsProtocolServerEventAuth,
    EditicsProtocolServerEventAuthRejected,
    EditicsProtocolServerEventConnectState,
)
from parsec.logging import get_logger
from parsec.types import BadOutcomeEnum

logger = get_logger()


# `indexUser`: 1-based, monotonic per session, assigned by order of join.
# OnlyOffice client code does arithmetic with this index (RFC §1.4), so it
# must stay a plain int.
EditicsIndexUser = int

# Buffer size for the per-participant SSE channel. Generous enough to absorb
# bursts of broadcasts without dropping the participant on backpressure.
_SSE_CHANNEL_BUFFER = 128


class EditicsSseChannel:
    """One participant's SSE connection (component-layer handle).

    The channel is created when the client opens the ``GET .../join`` SSE stream
    but is *pending* until the matching ``auth`` RPC arrives: until then it is
    not a participant of the session and does not receive ``connectState``
    broadcasts (todo step_0 §6).

    The server pushes server events by calling :meth:`send_nowait`; the ASGI
    route's generator consumes them from :attr:`receive` and frames them as SSE
    ``data:`` lines. This class lives in the components layer because it is the
    handle passed between the editics component and the ASGI route.
    """

    def __init__(self, participant_uuid: UUID, keepalive: float) -> None:
        self.participant_uuid = participant_uuid
        self.keepalive = keepalive
        self._send, self._recv = anyio.create_memory_object_stream[dict[str, Any]](
            max_buffer_size=_SSE_CHANNEL_BUFFER
        )
        # `pending` is True from creation until the matching `auth` RPC promotes
        # the channel to a full participant. While pending, the channel is
        # registered in `_pending` (not `Session.connections`).
        self.pending: bool = True
        self.closed: bool = False
        # Filled in when the channel is promoted to a full participant by the
        # `auth` RPC (used to build `ServerEventAuth.sessionTimeConnect` and to
        # find the participant on leave).
        self.index_user: EditicsIndexUser | None = None
        self.connect_time_ms: int | None = None

    @property
    def receive(self) -> MemoryObjectReceiveStream[dict[str, Any]]:
        return self._recv

    def send_nowait(self, event: dict[str, Any]) -> None:
        """Enqueue a server event to be delivered over this SSE stream.

        Returns silently if the channel is closed. Raises
        ``anyio.WouldBlock`` if the buffer is full (backpressure); the caller
        is then expected to drop the participant.
        """
        if self.closed:
            return
        self._send.send_nowait(event)

    def close(self) -> None:
        if self.closed:
            return
        self.closed = True
        self._send.close()
        self._recv.close()


@dataclass
class EditicsSession:
    """In-memory state of a single edition session (RFC §1.3).

    A session is identified by the pair ``(workspace_id, vlob_id)`` (both are
    ``VlobID``). No PostgreSQL persistence in step 0: state is lost on server
    restart.
    """

    workspace_id: VlobID
    vlob_id: VlobID
    initial_version: int
    latest_allowed_version: int
    # Monotonic, 1-based, assigned by order of join.
    next_index: int = 1
    # Monotonic non-decreasing timestamp (ms) of the last `connectState`
    # broadcast. Bumped on every participant-set change.
    participants_timestamp: int = 0
    # indexUser -> deviceId
    participants: dict[EditicsIndexUser, DeviceID] = field(default_factory=dict)
    # participant_uuid (client-generated) -> SSE channel
    connections: dict[UUID, EditicsSseChannel] = field(default_factory=dict)


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
    """Editics component base (step 0: auth subset).

    Sessions are kept in a process-local dict keyed by
    ``(workspace_id, vlob_id)``. No PostgreSQL persistence: state is lost on
    server restart.

    Pending SSE connections (opened via ``GET .../join`` but not yet
    authenticated via the matching ``auth`` RPC) are tracked in a separate
    top-level dict keyed by ``(workspace_id, vlob_id, participant_uuid)``. This
    lets the SSE join happen *before* the session is created: the session is
    created on the ``auth`` RPC, not on the SSE join (todo step_0 §6).
    """

    def __init__(self, config: BackendConfig) -> None:
        self._config = config
        self._sessions: dict[tuple[OrganizationID, VlobID, VlobID], EditicsSession] = {}
        # (organization_id, workspace_id, vlob_id, participant_uuid) -> channel
        self._pending: dict[tuple[OrganizationID, VlobID, VlobID, UUID], EditicsSseChannel] = {}

    # --- Session helpers ----------------------------------------------------

    def _get_session(
        self, organization_id: OrganizationID, workspace_id: VlobID, vlob_id: VlobID
    ) -> EditicsSession | None:
        return self._sessions.get((organization_id, workspace_id, vlob_id))

    def _get_or_create_session(
        self,
        organization_id: OrganizationID,
        workspace_id: VlobID,
        vlob_id: VlobID,
        initial_version: int,
    ) -> EditicsSession:
        key = (organization_id, workspace_id, vlob_id)
        session = self._sessions.get(key)
        if session is None:
            session = EditicsSession(
                workspace_id=workspace_id,
                vlob_id=vlob_id,
                initial_version=initial_version,
                latest_allowed_version=initial_version,
            )
            self._sessions[key] = session
        return session

    def _drop_session_if_empty(
        self, organization_id: OrganizationID, workspace_id: VlobID, vlob_id: VlobID
    ) -> None:
        key = (organization_id, workspace_id, vlob_id)
        session = self._sessions.get(key)
        if session is not None and not session.participants and not session.connections:
            del self._sessions[key]

    def _drop_session(
        self, organization_id: OrganizationID, workspace_id: VlobID, vlob_id: VlobID
    ) -> None:
        # Unconditionally remove the session (and close any live participant
        # channels). Used to self-heal malformed sessions.
        key = (organization_id, workspace_id, vlob_id)
        session = self._sessions.pop(key, None)
        if session is None:
            return
        for channel in session.connections.values():
            channel.close()
        session.connections.clear()
        session.participants.clear()

    @staticmethod
    def _participants_list(
        session: EditicsSession,
    ) -> list[EditicsProtocolParticipantEntry]:
        return [
            EditicsProtocolParticipantEntry(indexUser=index, deviceId=device_id)
            for index, device_id in sorted(session.participants.items())
        ]

    def _now_ms(self, session: EditicsSession) -> int:
        # Wall-clock ms, but never let the session's participant-set timestamp
        # go backwards.
        now_ms = int(time.time() * 1000)
        if now_ms <= session.participants_timestamp:
            now_ms = session.participants_timestamp + 1
        return now_ms

    # --- Broadcast helpers --------------------------------------------------

    def _broadcast_connect_state(
        self,
        organization_id: OrganizationID,
        session: EditicsSession,
        waitAuth: bool = False,
    ) -> None:
        session.participants_timestamp = self._now_ms(session)
        event = EditicsProtocolServerEventConnectState(
            participantsTimestamp=session.participants_timestamp,
            participants=self._participants_list(session),
            waitAuth=waitAuth,
        )
        self._broadcast(organization_id, session, event)

    def _broadcast(
        self,
        organization_id: OrganizationID,
        session: EditicsSession,
        event: EditicsProtocolServerEvent,
        *,
        exclude: set[UUID] | None = None,
    ) -> None:
        # `model_dump(mode="json")` applies the pydantic serializers so the
        # payload is JSON-serializable by the SSE framer.
        payload = event.model_dump(mode="json")
        for participant_uuid, channel in list(session.connections.items()):
            if exclude and participant_uuid in exclude:
                continue
            try:
                channel.send_nowait(payload)
            except anyio.WouldBlock:
                logger.warning("editics: dropping participant due to backpressure")
                self._drop_participant(organization_id, session, channel)
            except Exception:
                logger.warning("editics: dropping participant due to send error")
                self._drop_participant(organization_id, session, channel)

    def _send_to(self, channel: EditicsSseChannel, event: EditicsProtocolServerEvent) -> None:
        try:
            channel.send_nowait(event.model_dump(mode="json"))
        except anyio.WouldBlock:
            logger.warning("editics: dropping participant due to backpressure")
        except Exception:
            logger.warning("editics: dropping participant due to send error")

    def _drop_participant(
        self,
        organization_id: OrganizationID,
        session: EditicsSession,
        channel: EditicsSseChannel,
    ) -> None:
        # Best-effort cleanup of a backpressure-failing channel; full leave is
        # handled by the route's `finally` on disconnect. Close the channel so
        # the SSE generator returns and the leave flow runs.
        channel.close()

    def stop(self) -> None:
        """Release all in-memory session state (close every SSE channel).

        Called on backend teardown so the server-side SSE generators depending
        on the channels' receive streams terminate (they get `EndOfStream`)
        instead of blocking forever on a participant that never disconnected.
        """
        for session in self._sessions.values():
            for channel in session.connections.values():
                channel.close()
            session.connections.clear()
            session.participants.clear()
        self._sessions.clear()
        for channel in self._pending.values():
            channel.close()
        self._pending.clear()

    # --- Public API: SSE join ------------------------------------------------

    @asynccontextmanager
    async def sse_api_join_session(
        self,
        organization_id: OrganizationID,
        device_id: DeviceID,
        participant_id: UUID,
        realm_id: VlobID,
        document_id: VlobID,
        last_event_id: int | None = None,
    ) -> AsyncGenerator[EditicsSseChannel | EditicsJoinSessionBadOutcome, None]:
        """Register a *pending* SSE connection for ``(session, participant_uuid)``.

        The connection stays pending until the matching ``auth`` RPC promotes it
        to a full participant. A new join replaces (and closes) a stale pending
        connection for the same key.

        Yields the :class:`EditicsSseChannel` the route consumes to stream
        server events to the client (or a bad outcome to abort the handshake).
        The channel is cleaned up (leave flow) when the context manager exits.
        """
        # TODO step_0: real authorization. For step 0 the `Authorization: Editics`
        # header is intentionally insecure (see todo §3.3).
        keepalive = self._config.sse_keepalive if self._config.sse_keepalive is not None else 30.0
        channel = EditicsSseChannel(participant_id, keepalive)

        key = (organization_id, realm_id, document_id, participant_id)
        old = self._pending.pop(key, None)
        if old is not None:
            old.close()
        self._pending[key] = channel

        try:
            yield channel
        finally:
            # On disconnect/cancel, run the leave flow.
            self._on_sse_disconnect(organization_id, realm_id, document_id, participant_id)

    def _on_sse_disconnect(
        self,
        organization_id: OrganizationID,
        realm_id: VlobID,
        document_id: VlobID,
        participant_id: UUID,
    ) -> None:
        # Remove the pending connection if it never got promoted.
        pkey = (organization_id, realm_id, document_id, participant_id)
        pending = self._pending.pop(pkey, None)
        if pending is not None:
            pending.close()
            return

        session = self._get_session(organization_id, realm_id, document_id)
        if session is None:
            return
        channel = session.connections.pop(participant_id, None)
        if channel is None:
            return
        channel.close()
        index_user = channel.index_user
        if index_user is not None:
            session.participants.pop(index_user, None)
        # Broadcast the updated participant set to the remaining participants.
        if session.participants:
            self._broadcast_connect_state(organization_id, session, waitAuth=False)
        else:
            self._drop_session_if_empty(organization_id, realm_id, document_id)

    # --- Public API: client event dispatch -----------------------------------

    async def api_send_in_session(
        self,
        organization_id: OrganizationID,
        device_id: DeviceID,
        participant_id: UUID,
        realm_id: VlobID,
        document_id: VlobID,
        event: EditicsProtocolClientEvent,
    ) -> EditicsProtocolServerEvent | EditicsSendInSessionBadOutcome | None:
        if isinstance(event, EditicsProtocolClientEventAuth):
            return self._handle_auth(
                organization_id, realm_id, document_id, device_id, participant_id, event
            )
        # All other events require an established participant; not part of the
        # step 0 "just enough" subset. Ignore them (204) for now.
        return None

    def _handle_auth(
        self,
        organization_id: OrganizationID,
        workspace_id: VlobID,
        vlob_id: VlobID,
        device_id: DeviceID,
        participant_id: UUID,
        event: EditicsProtocolClientEventAuth,
    ) -> EditicsProtocolServerEvent:
        session = self._get_session(organization_id, workspace_id, vlob_id)

        # A Parsec vlob version is always >= 1. RFC §1.3 requires a document to
        # exist as a vlob before editing.
        if event.vlobVersion < 1:
            return EditicsProtocolServerEventAuthRejected(latestAllowedVersion=0)
        # Self-heal malformed sessions left with an invalid `initial_version < 1`.
        if session is not None and session.initial_version < 1:
            self._drop_session(organization_id, workspace_id, vlob_id)
            session = None

        if session is None:
            session = self._get_or_create_session(
                organization_id, workspace_id, vlob_id, initial_version=event.vlobVersion
            )
            session.latest_allowed_version = session.initial_version
        else:
            if event.vlobVersion < session.initial_version:
                return EditicsProtocolServerEventAuthRejected(
                    latestAllowedVersion=session.initial_version
                )
            if event.vlobVersion > session.latest_allowed_version:
                return EditicsProtocolServerEventAuthRejected(
                    latestAllowedVersion=session.latest_allowed_version
                )

        # Promote the pending SSE connection.
        pkey = (organization_id, workspace_id, vlob_id, participant_id)
        channel = self._pending.pop(pkey, None)
        if channel is None:
            # No pending SSE connection for this participant: cannot join.
            return EditicsProtocolServerEventAuthRejected(
                latestAllowedVersion=session.latest_allowed_version
            )
        channel.pending = False
        channel.connect_time_ms = int(time.time() * 1000)
        session.connections[participant_id] = channel

        # Assign indexUser & register the participant.
        index_user = session.next_index
        session.next_index += 1
        session.participants[index_user] = device_id
        channel.index_user = index_user

        participants = self._participants_list(session)
        auth_reply = EditicsProtocolServerEventAuth(
            result=1,
            participants=participants,
            indexUser=index_user,
            sessionId=uuid4().hex,
            sessionTimeConnect=channel.connect_time_ms,
        )

        # Broadcast connectState to all participants (including the newcomer).
        # In step 0's single-client happy path the newcomer is the only
        # participant, so it is the sole recipient (this still exercises the
        # SSE delivery path).
        self._broadcast_connect_state(organization_id, session, waitAuth=False)

        return auth_reply
