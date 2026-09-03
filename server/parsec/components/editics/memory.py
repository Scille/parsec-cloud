# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
"""In-memory implementation of the editics component (step 0)."""

from __future__ import annotations

import time
from typing import TYPE_CHECKING
from uuid import UUID, uuid4

from parsec._parsec import OrganizationID, VlobID
from parsec.components.editics.base import (
    AuthClient,
    AuthRejected,
    AuthServer,
    BaseEditicsComponent,
    ConnectState,
    EditicsClientContext,
    EditicsSession,
    ParticipantEntry,
)
from parsec.config import BackendConfig
from parsec.logging import get_logger

if TYPE_CHECKING:
    from parsec.components.editics.transport import EditicsSseChannel

logger = get_logger()


class MemoryEditicsComponent(BaseEditicsComponent):
    """In-memory editics component.

    Sessions are kept in a process-local dict keyed by `(workspace_id, vlob_id)`.
    No PostgreSQL persistence in step 0: state is lost on server restart.

    Pending SSE connections (opened via `GET .../join` but not yet authenticated
    via the matching `auth` RPC) are tracked in a separate top-level dict keyed
    by `(workspace_id, vlob_id, participant_uuid)`. This keeps the `Session`
    model from todo §5 (where `pending` lives on the session) consistent while
    still allowing the SSE join to happen *before* the session is created: the
    session is created on the `auth` RPC, not on the SSE join (todo §6).

    Note on `organization_id`: it is accepted for API symmetry with the other
    components and for future per-organization isolation, but is *not* part of
    the session key in step 0 (a `(workspace_id, vlob_id)` pair is globally
    unique across organizations in practice). This is acceptable for the
    in-memory step-0 validation and will be revisited with persistence.
    """

    def __init__(self, config: BackendConfig) -> None:
        self._config = config
        self._sessions: dict[tuple[VlobID, VlobID], EditicsSession] = {}
        # (workspace_id, vlob_id, participant_uuid) -> channel
        self._pending: dict[tuple[VlobID, VlobID, UUID], EditicsSseChannel] = {}

    # --- Session helpers ----------------------------------------------------

    def _get_session(self, workspace_id: VlobID, vlob_id: VlobID) -> EditicsSession | None:
        return self._sessions.get((workspace_id, vlob_id))

    def _get_or_create_session(
        self, workspace_id: VlobID, vlob_id: VlobID, initial_version: int
    ) -> EditicsSession:
        key = (workspace_id, vlob_id)
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

    def _drop_session_if_empty(self, workspace_id: VlobID, vlob_id: VlobID) -> None:
        key = (workspace_id, vlob_id)
        session = self._sessions.get(key)
        if session is not None and not session.participants and not session.connections:
            # No connections and no participants: the session is inert. Pending
            # connections (if any) keep the session reachable, but they live in a
            # separate dict; a session with only pending connections is dropped
            # here and recreated on the `auth` RPC. This is fine for step 0.
            del self._sessions[key]

    @staticmethod
    def _participants_list(session: EditicsSession) -> list[ParticipantEntry]:
        return [
            ParticipantEntry(indexUser=index, deviceId=device_id)
            for index, device_id in sorted(session.participants.items())
        ]

    # --- Public API ---------------------------------------------------------

    async def join_sse(
        self,
        organization_id: OrganizationID,
        workspace_id: VlobID,
        vlob_id: VlobID,
        client_ctx: EditicsClientContext,
        channel: EditicsSseChannel,
    ) -> None:
        """Register a *pending* SSE connection for `(session, participant_uuid)`.

        The connection stays pending until the matching `auth` RPC promotes it
        to a full participant (todo step_0 §6). At most one pending connection
        per `(session, participant_uuid)` in step 0: a new join replaces (and
        closes) a stale pending connection for the same key.
        """
        key = (workspace_id, vlob_id, client_ctx.participant_uuid)
        old = self._pending.pop(key, None)
        if old is not None:
            old.close()
        self._pending[key] = channel

    async def handle_client_event(
        self,
        organization_id: OrganizationID,
        workspace_id: VlobID,
        vlob_id: VlobID,
        client_ctx: EditicsClientContext,
        event: AuthClient,
    ) -> AuthServer | AuthRejected:
        """Handle a client→server event.

        Step 0 only implements `auth` (the sole `ClientEvent`). The flow is
        (todo step_0 §6, §7):

        - Match the pending SSE connection for `(session, participant_uuid)`.
        - Create the session if absent (`initial_version = auth.vlobVersion`),
          or validate `auth.vlobVersion` against the existing session's
          `[initial_version, latest_allowed_version]` range (RFC §1.2).
        - Assign the participant a fresh `indexUser`, promote the pending SSE
          channel, build the `AuthServer` reply and enqueue a `connectState`
          broadcast to all participants (the newcomer included).
        """
        session = self._get_session(workspace_id, vlob_id)

        # --- Vlob-version validation (RFC §1.2) -----------------------------
        if session is None:
            # Session absent -> create it.
            session = self._get_or_create_session(
                workspace_id, vlob_id, initial_version=event.vlobVersion
            )
            session.latest_allowed_version = session.initial_version
        else:
            if event.vlobVersion < session.initial_version:
                return AuthRejected(latestAllowedVersion=session.initial_version)
            if event.vlobVersion > session.latest_allowed_version:
                return AuthRejected(latestAllowedVersion=session.latest_allowed_version)
            # else: accepted (in range)

        # --- Promote the pending SSE connection -----------------------------
        key = (workspace_id, vlob_id, client_ctx.participant_uuid)
        channel = self._pending.pop(key, None)
        if channel is None:
            # The `auth` RPC arrived without a matching pending SSE connection.
            # In step 0 this is a protocol violation; reject as a failed auth.
            return AuthRejected(latestAllowedVersion=session.latest_allowed_version)
        channel.pending = False
        channel.connect_time_ms = int(time.time() * 1000)
        session.connections[client_ctx.participant_uuid] = channel

        # --- Assign indexUser & register the participant --------------------
        index_user = session.next_index
        session.next_index += 1
        session.participants[index_user] = client_ctx.device_id

        # Track this participant's index on its channel so we can find it on
        # leave (the SSE disconnect only knows `participant_uuid`).
        channel.index_user = index_user

        # --- Build the AuthServer reply (returned as the RPC reply) ---------
        participants = self._participants_list(session)
        auth_reply = AuthServer(
            result=1,
            participants=participants,
            indexUser=index_user,
            sessionId=uuid4().hex,
            sessionTimeConnect=channel.connect_time_ms,
        )

        # --- Broadcast connectState to all participants (incl. newcomer) ---
        self._broadcast_connect_state(session)

        return auth_reply

    def _broadcast_connect_state(self, session: EditicsSession) -> None:
        participants = self._participants_list(session)
        event = ConnectState(
            participantsTimestamp=int(time.time() * 1000),
            participants=participants,
            waitAuth=False,
        )
        payload = event.model_dump()
        for channel in session.connections.values():
            try:
                channel.send_nowait(payload)
            except Exception:
                # Backpressure / closed channel: drop the participant (mirrors
                # the events SSE backpressure handling in `parsec/asgi/rpc.py`).
                logger.warning("editics: dropping participant due to backpressure")

    async def leave(
        self,
        workspace_id: VlobID,
        vlob_id: VlobID,
        client_ctx: EditicsClientContext,
    ) -> None:
        """Remove a participant on SSE disconnect (todo step_0 §8).

        There is no `close` event in step 0: the server relies on the SSE
        disconnect. The channel is removed from `connections` (or `pending` if
        it never authenticated), the participant is removed from the session,
        and a `connectState` is broadcast to the remaining participants (if any).
        """
        key3 = (workspace_id, vlob_id, client_ctx.participant_uuid)

        # Remove from pending (never authenticated).
        channel = self._pending.pop(key3, None)
        if channel is not None:
            channel.close()
            return

        session = self._get_session(workspace_id, vlob_id)
        if session is None:
            return

        # Remove a full participant.
        channel = session.connections.pop(client_ctx.participant_uuid, None)
        if channel is None:
            return
        channel.close()
        index_user: int | None = getattr(channel, "index_user", None)
        if index_user is not None:
            session.participants.pop(index_user, None)

        if session.participants:
            # Broadcast the updated participant set to the remaining ones.
            self._broadcast_connect_state(session)
        else:
            # No participants left: the session may be GC'd (implementation
            # choice, not required for correctness in step 0).
            self._drop_session_if_empty(workspace_id, vlob_id)
