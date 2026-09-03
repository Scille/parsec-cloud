# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
"""
Editics collaborative edition component (step 0: auth subset).

This implements the in-memory server state for collaborative document edition
sessions over SSE + RPC, as described in `docs/rfcs/1030-collaborative-editics.md`
and specified in `todo/step_0.md`.

Step 0 only covers the *auth* part of the protocol: a client joins a fresh
edition session, is assigned a participant index (`indexUser`), receives an
`auth` server event (as the RPC reply) and a `connectState` SSE event (broadcast
to all participants, the newcomer included). There are no document modifications,
no chat, no cursors, no save and no region/auth locks.

OnlyOffice event/field names are kept verbatim and documented at the definition
site, they are *not* renamed even when they are known-bad (this keeps the
client-side translation layer thin, see RFC §2).
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Annotated, Literal
from uuid import UUID

from pydantic import BaseModel, BeforeValidator, Field, PlainSerializer

from parsec._parsec import DeviceID, VlobID


def _device_id_from_any(value: object) -> DeviceID:
    """Pydantic before-validator: accept a `DeviceID` as-is, or a hex string."""
    if isinstance(value, DeviceID):
        return value
    if isinstance(value, str):
        return DeviceID.from_hex(value)
    raise ValueError(f"expected a DeviceID or hex string, got {type(value).__name__}")


# `DeviceID` is a Rust-backed builtins type pydantic can't handle natively. We
# serialize it to its hex form (matching the `Authorization: Editics` header and
# the TS client `deviceId: string // DeviceID hex`) and parse it back from hex.
DeviceIDField = Annotated[
    DeviceID,
    BeforeValidator(_device_id_from_any),
    PlainSerializer(lambda x: x.hex, return_type=str),
]


# `indexUser`: 1-based, monotonic per session, assigned by order of join.
# OnlyOffice client code does arithmetic with this index (RFC §1.4), so it
# must stay a plain int.
IndexUser = int


class ParticipantEntry(BaseModel):
    """One entry of the `connectState.participants` list.

    OnlyOffice `connectState.participants[]` is a rich object
    ({ id, idOriginal, username, view, connectionId, isCloseCoAuthoring,
       isLiveViewer, encrypted }).
    In the editics protocol the server is NOT trusted for user names (RFC §3.3),
    so each entry only carries the participant index and the device id. The
    client resolves deviceId -> user_name through libparsec.
    """

    model_config = {"arbitrary_types_allowed": True}

    # OnlyOffice name `indexUser` kept (bad name documented, not renamed).
    indexUser: IndexUser
    # Editics addition (replaces OnlyOffice's idOriginal/username/etc.).
    # Serialized as its hex form; parsed from a `DeviceID` or a hex string.
    deviceId: DeviceIDField


# --- Client -> server events -------------------------------------------------


class AuthClient(BaseModel):
    """OnlyOffice client `auth` event, trimmed.

    See RFC §2.2 / todo step_0 §4 for the fields dropped from the OnlyOffice
    `auth` (client->server): jwtOpen, jwtSession, token, user, documentFormatSave,
    headingsColor, lang, openCmd, encrypted, IsAnonymousUser, timezoneOffset,
    time, supportAuthChangesAck, lastOtherSaveTime, block, sessionId,
    sessionTimeConnect, sessionTimeIdle, isCloseCoAuthoring, mode, permissions,
    coEditingMode, docid (carried by the URL path).
    """

    # OnlyOffice name `auth` kept.
    type: Literal["auth"] = "auth"
    # -1 on first open; server-assigned index on reconnect. Always -1 in step 0.
    indexUser: int = -1
    # 0=Word, 1=Spreadsheet, 2=Presentation, 3=Visio.
    # Informational; server stores, ignores. (Kept for now; may be dropped later.)
    editorType: int
    # The vlob version the client has loaded locally (RFC §1.2). For a fresh
    # session this becomes the session's initial version.
    # (OnlyOffice has no such field; editics addition.)
    vlobVersion: int


ClientEvent = AuthClient


# --- Server -> client events -------------------------------------------------


class AuthServer(BaseModel):
    """OnlyOffice server `auth` reply, trimmed. Name kept.

    See RFC §2.2 / todo step_0 §4.4 for the fields dropped from the OnlyOffice
    `auth` (server->client): jwt, messages, locks, hasForgotten,
    g_cAscSpellCheckUrl, buildVersion, buildNumber, licenseType, settings,
    openedAt, docid (carried by the URL path). The change backlog is NOT folded
    in: it is delivered as a separate `authChanges` SSE event (RFC §2.2).
    """

    # OnlyOffice server `auth` reply, trimmed. Name kept.
    type: Literal["auth"] = "auth"
    result: int = 1  # 1 = success (OnlyOffice convention)
    participants: list[ParticipantEntry]  # current participant map
    indexUser: IndexUser  # this connection's assigned index
    # Reconnect info (forward-compat; unused in step 0 but kept).
    sessionId: str
    sessionTimeConnect: int  # server timestamp (ms) at connect
    # NOTE: the change backlog is NOT here. It is delivered as a separate
    # `authChanges` SSE event (RFC §2.2: backlog can exceed one event).
    # For a fresh session the backlog is empty -> no `authChanges` is sent.


class ConnectState(BaseModel):
    """OnlyOffice `connectState`, trimmed. Name kept.

    See RFC §2.2 / todo step_0 §4.5 for the fields dropped from the OnlyOffice
    `connectState`: the rich per-participant objects are replaced by
    `ParticipantEntry { indexUser, deviceId }` (the client resolves names via
    libparsec; the server isn't trusted for them).
    """

    # OnlyOffice `connectState`, trimmed. Name kept.
    type: Literal["connectState"] = "connectState"
    # Monotonic ms timestamp of this participant-set update (OnlyOffice name).
    participantsTimestamp: int
    participants: list[ParticipantEntry]
    waitAuth: bool = False  # always false in step 0 (no auth lock)


class AuthChanges(BaseModel):
    """OnlyOffice `authChanges`. Name kept.

    Defined for completeness; NOT sent in step 0 (fresh session -> empty
    backlog).
    """

    # OnlyOffice `authChanges`. Name kept.
    type: Literal["authChanges"] = "authChanges"
    # Each entry: (change index, encrypted change blob). Empty for fresh session.
    # The server serializes `bytes` as base64 when it eventually sends
    # `authChanges` (JSON cannot carry raw bytes).
    changes: list[tuple[int, bytes]] = Field(default_factory=list)


ServerEvent = AuthServer | ConnectState | AuthChanges


# --- Rejection response (RPC reply) -----------------------------------------


class AuthRejected(BaseModel):
    """`auth` reply shape reused for rejection (RFC §1.2).

    On rejection the RPC returns this instead of `AuthServer`, with a
    non-success `result` and the allowed version. OnlyOffice uses `result`
    codes; we reuse the field (bad name documented at the definition site).
    """

    # OnlyOffice `auth` reply shape, reused for rejection. Name `result` kept.
    type: Literal["auth"] = "auth"
    result: int = 0  # 0 = rejected (OnlyOffice: non-1 = failure)
    # RFC §1.2: the version the client should reload to before retrying.
    latestAllowedVersion: int


# --- In-memory session state (step 0) ---------------------------------------


@dataclass
class EditicsSession:
    """In-memory state of a single edition session (RFC §1.3, todo §5).

    A session is identified by the pair `(workspace_id, vlob_id)` (both are
    `VlobID`). No PostgreSQL persistence in step 0: state is lost on server
    restart.
    """

    workspace_id: VlobID
    vlob_id: VlobID
    initial_version: int
    latest_allowed_version: int
    next_index: int = 1  # monotonic, starts at 1
    # indexUser -> deviceId
    participants: dict[IndexUser, DeviceID] = field(default_factory=dict)
    # participant_uuid (client-generated) -> SSE channel
    connections: dict[UUID, EditicsSseChannel] = field(default_factory=dict)
    # participant_uuid -> pending SSE connection (opened, auth not yet received)
    pending: dict[UUID, EditicsSseChannel] = field(default_factory=dict)


from parsec.components.editics.transport import EditicsSseChannel


@dataclass
class EditicsClientContext:
    """Parsed `Authorization: Editics` header.

    For step 0 we do NOT use the Parsec `PARSEC-SIGN-ED25519` token. Instead the
    client sends a lightweight identity header on both routes
    (`Authorization: Editics <device_id_hex>.<participant_uuid_hex>`).

    ⚠️ This is NOT a secure authorization system. It is intentionally simple
    and convenient for step 0. It will be replaced by proper Parsec
    authentication in a later step. The server must not trust this header for
    access-control decisions beyond "this is the participant UUID the client
    wants to be known by".
    """

    device_id: DeviceID
    participant_uuid: UUID


class BaseEditicsComponent:
    """Base class for the editics component.

    Step 0 only has an in-memory implementation; no PostgreSQL backend is
    needed.
    """

    async def join_sse(
        self,
        organization_id: object,
        workspace_id: VlobID,
        vlob_id: VlobID,
        client_ctx: EditicsClientContext,
    ) -> EditicsSseChannel:
        raise NotImplementedError

    async def handle_client_event(
        self,
        organization_id: object,
        workspace_id: VlobID,
        vlob_id: VlobID,
        client_ctx: EditicsClientContext,
        event: ClientEvent,
    ) -> AuthServer | AuthRejected | None:
        raise NotImplementedError

    async def leave(
        self,
        workspace_id: VlobID,
        vlob_id: VlobID,
        client_ctx: EditicsClientContext,
    ) -> None:
        raise NotImplementedError
