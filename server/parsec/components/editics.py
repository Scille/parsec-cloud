# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
"""
Editics collaborative edition component (step 1: full command set).

This implements the server state for collaborative document edition sessions
over SSE + RPC, as described in `docs/rfcs/1030-collaborative-editics.md` and
specified in `todo/step_1.md`.

Step 1 implements the full editics protocol command set (RFC §2.2): join
(`auth`), auth lock / `waitAuth`, `authChanges` backlog, chat (`message`),
cursors (`cursor`), region locks (`getLock`/`releaseLock`), the save flow
(`isSaveLock`/`saveChanges`/`savePartChanges`/`unSaveLock`), `unLockDocument`,
`close`, `drop`, `warning`, and the `saveDone` vlob-version bump. Everything
stays in memory (no PostgreSQL); state is lost on server restart.

OnlyOffice event/field names are kept verbatim and documented at the definition
site, they are *not* renamed even when they are known-bad (this keeps the
client-side translation layer thin, see RFC §2).
"""

from __future__ import annotations

import time
from dataclasses import dataclass, field
from typing import Annotated, Any, Literal
from uuid import UUID, uuid4

import anyio
from anyio.streams.memory import MemoryObjectReceiveStream
from pydantic import BaseModel, BeforeValidator, Field, PlainSerializer

from parsec._parsec import DeviceID, OrganizationID, VlobID
from parsec.config import BackendConfig
from parsec.logging import get_logger

logger = get_logger()


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
    # OnlyOffice name `view` kept. Step 1 addition (forward-compat, todo
    # step_1 §4.1 / §2.5): whether the participant is a viewer (read-only).
    # Always False in step 1 (all participants are treated as editors; real
    # realm-role-based access control is deferred). The field is present in
    # the schema so the client translation layer can be written against the
    # final shape.
    view: bool = False


# --- Client -> server events -------------------------------------------------


class ClientEventAuth(BaseModel):
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


class ClientEventAuthChangesAck(BaseModel):
    """OnlyOffice `authChangesAck` (c->s). Name kept.

    Acknowledges one `authChanges` chunk (RFC §2.2). In step 1 the backlog is
    delivered in a single chunk (todo step_1 §6.3), so at most one ack is
    expected per join.
    """

    type: Literal["authChangesAck"] = "authChangesAck"


class ClientEventMessage(BaseModel):
    """OnlyOffice `message` (c->s). Name kept. The chat message is encrypted
    (§2.4); renamed to `encryptedMessage` per RFC §2.2 editics changes."""

    type: Literal["message"] = "message"
    # base64 over JSON (§2.4). Opaque; the server never inspects the content.
    encryptedMessage: bytes


class ClientEventCursor(BaseModel):
    """OnlyOffice `cursor` (c->s). Name kept. The cursor is encrypted (§2.4);
    renamed to `encryptedCursor` per RFC §2.2 editics changes."""

    type: Literal["cursor"] = "cursor"
    # base64 over JSON (§2.4). Opaque; the server never inspects the content.
    encryptedCursor: bytes


class ClientEventGetLock(BaseModel):
    """OnlyOffice `getLock` (c->s). Name kept. Kept as-is per RFC §2.2 (the
    `block` array is opaque to the server; keyed by JSON serialization)."""

    type: Literal["getLock"] = "getLock"
    # Opaque block descriptors (shape depends on editor type).
    block: list[Any]


class ClientEventIsSaveLock(BaseModel):
    """OnlyOffice `isSaveLock` (c->s). Name kept. The client's current
    always-advancing sync point (§2.2). Used by the server to detect a
    desynchronized client (keep denying the lock until it catches up)."""

    type: Literal["isSaveLock"] = "isSaveLock"
    syncChangesIndex: int


class ClientEventSaveChanges(BaseModel):
    """OnlyOffice `saveChanges` (c->s). Name kept. See RFC §2.2 editics changes
    for the field deltas:

    - `changes` (a JSON-encoded string in default mode) -> `encryptedChanges`
      (list[bytes], one per fragment; requires binary changes mode, see §2.2).
    - `excelAdditionalInfo` split into `encryptedCursor` (bytes, the
      `CursorInfo` part, encrypted) and `excel_info` (the `indexCols` /
      `indexRows` part, cleartext; null if not the spreadsheet editor).
    - `unlock` removed (relies on `unLockDocument`).
    - `reSave`, `isExcel` removed.
    - `isCoAuthoring` removed (server knows).
    """

    type: Literal["saveChanges"] = "saveChanges"
    # One entry per change fragment, each independently encrypted + base64-
    # encoded (see §2.2 fragment granularity, §2.4). Requires binary changes
    # mode on the OnlyOffice side (`editorConfig.settings.binaryChanges: true`)
    # so the editor produces `changes` as a real JSON array instead of a
    # JSON-encoded string; the editics client then encrypts each fragment.
    # The server advances `syncChangesIndex` by `len(encryptedChanges)` and
    # stores one `StoredChange` per entry (== OnlyOffice `puckerIndex += k`).
    # Each element: base64 over JSON (§2.4). Opaque; server never inspects.
    encryptedChanges: list[bytes]
    startSaveChanges: bool
    endSaveChanges: bool
    # UNDO support: null/-1 = no truncate; <int> = absolute index, the server
    # deletes all stored changes with index >= deleteIndex. Only honored when
    # startSaveChanges is true (OnlyOffice semantics).
    deleteIndex: int | None = None
    # Spreadsheet-only cleartext lock-shift info (null for non-spreadsheet).
    # Shape: { indexCols: ..., indexRows: ... } (OnlyOffice names kept).
    excel_info: dict[str, Any] | None = None
    # Encrypted cursor carried alongside the save (the `CursorInfo` part of
    # the old `excelAdditionalInfo`), broadcast as-is to others. base64 over
    # JSON (§2.4). Opaque; server never inspects.
    encryptedCursor: bytes | None = None
    # Release this user's region locks after the save (OnlyOffice name kept).
    releaseLocks: bool = False


class ClientEventUnSaveLock(BaseModel):
    """OnlyOffice `unSaveLock` (c->s). Name kept. Cancel an in-progress save and
    release the save lock without saving (RFC §2.2). Kept as-is."""

    type: Literal["unSaveLock"] = "unSaveLock"


class ClientEventUnLockDocument(BaseModel):
    """OnlyOffice `unLockDocument` (c->s). Name kept. Fire-and-forget cleanup
    combining up to three independent actions (RFC §2.2). Kept as-is."""

    type: Literal["unLockDocument"] = "unLockDocument"
    isSave: bool = False
    unlock: bool = False
    deleteIndex: int | None = None
    releaseLocks: bool = False


class ClientEventClose(BaseModel):
    """OnlyOffice `close` (c->s). Name kept. Voluntary leave + close (unlike
    `unLockDocument` which keeps the connection alive). Kept as-is."""

    type: Literal["close"] = "close"


class ClientEventSaveDone(BaseModel):
    """Editics addition (no OnlyOffice equivalent). Sent by the client after it
    has uploaded a new vlob version, to bump the session's
    `latest_allowed_version` (RFC §1.2 step 4.3). Must be sent only by the
    participant that just released the save lock (todo step_1 §6.9)."""

    type: Literal["saveDone"] = "saveDone"
    savedUpToIndex: int  # all changes up to this index are in the new version
    newVersion: int  # the new vlob version just uploaded


ClientEvent = Annotated[
    ClientEventAuth
    | ClientEventAuthChangesAck
    | ClientEventMessage
    | ClientEventCursor
    | ClientEventGetLock
    | ClientEventIsSaveLock
    | ClientEventSaveChanges
    | ClientEventUnSaveLock
    | ClientEventUnLockDocument
    | ClientEventClose
    | ClientEventSaveDone,
    Field(discriminator="type"),
]


# --- Server -> client events -------------------------------------------------


class ServerEventAuth(BaseModel):
    """OnlyOffice server `auth` reply, trimmed. Name kept.

    See RFC §2.2 / todo step_0 §4.4 for the fields dropped from the OnlyOffice
    `auth` (server->client): jwt, messages, locks, hasForgotten,
    g_cAscSpellCheckUrl, buildVersion, buildNumber, licenseType, settings,
    openedAt, docid (carried by the URL path). The change backlog is NOT folded
    in: it is delivered as a separate `authChanges` SSE event (RFC §2.2).
    """

    type: Literal["auth"] = "auth"
    result: int = 1  # 1 = success (OnlyOffice convention)
    participants: list[ParticipantEntry]  # current participant map
    indexUser: IndexUser  # this connection's assigned index
    # Reconnect info (forward-compat; unused in step 0 but kept).
    sessionId: str
    sessionTimeConnect: int  # server timestamp (ms) at connect


class ServerEventAuthRejected(BaseModel):
    """`auth` reply shape reused for rejection (RFC §1.2).

    On rejection the RPC returns this instead of `ServerEventAuth`, with a
    non-success `result` and the allowed version. OnlyOffice uses `result`
    codes; we reuse the field (bad name documented at the definition site).
    """

    type: Literal["auth"] = "auth"
    result: int = 0  # 0 = rejected (OnlyOffice: non-1 = failure)
    # RFC §1.2: the version the client should reload to before retrying.
    latestAllowedVersion: int


class ServerEventConnectState(BaseModel):
    """OnlyOffice `connectState`, trimmed. Name kept.

    See RFC §2.2 / todo step_0 §4.5 for the fields dropped from the OnlyOffice
    `connectState`: the rich per-participant objects are replaced by
    `ParticipantEntry { indexUser, deviceId, view }` (the client resolves names
    via libparsec; the server isn't trusted for them).
    """

    type: Literal["connectState"] = "connectState"
    # Monotonic ms timestamp of this participant-set update (OnlyOffice name).
    participantsTimestamp: int
    participants: list[ParticipantEntry]
    # True while the document auth lock is held (todo step_1 §6.2): tells the
    # established editor it must send `unLockDocument{unlock:true}` to release
    # the lock and let newcomers proceed.
    waitAuth: bool = False


class ServerEventAuthChanges(BaseModel):
    """OnlyOffice `authChanges` (s->c). Name kept.

    Delivered to a joining client (after the auth lock is released, if any)
    as the backlog of changes that occurred since the session was created
    (RFC §1.2 step 3.4). In step 1 the backlog is delivered in a single chunk.
    """

    type: Literal["authChanges"] = "authChanges"
    # Each entry: (change index, encrypted change blob). The index is 1-based
    # and monotonic; the blob is base64 over JSON (§2.4), opaque to the server.
    changes: list[tuple[int, bytes]] = Field(default_factory=list)


class ServerEventWaitAuth(BaseModel):
    """OnlyOffice `waitAuth` (s->c). Name kept. Per RFC §2.2 editics changes,
    `lockDocument` is replaced by `authLockedBy` (the indexUser holding the
    auth lock). Sent to a joining non-view participant when the auth lock is
    held (§6.2), as the RPC reply (the newcomer is "parked")."""

    type: Literal["waitAuth"] = "waitAuth"
    authLockedBy: IndexUser


class MessageRecord(BaseModel):
    """One record in a server `message` event's `messages` array (todo §4.4)."""

    model_config = {"arbitrary_types_allowed": True}
    time: int  # server timestamp (ms)
    authorIndexUser: IndexUser
    # base64 over JSON (§2.4). Opaque; server never inspects.
    encryptedMessage: bytes


class ServerEventMessage(BaseModel):
    """OnlyOffice `message` (s->c). Name kept. Per RFC §2.2 editics changes:
    drop `docid`; replace `user`/`useridoriginal`/`username` by `authorIndexUser`;
    `message` -> `encryptedMessage` (bytes, §2.4). OnlyOffice wraps the payload
    in `messages: [...]`; we keep the array shape (bad name documented) for
    translation-layer symmetry. In step 1 the server sends exactly one entry
    per broadcast (to all participants, including the sender, §6.4)."""

    type: Literal["message"] = "message"
    messages: list[MessageRecord]


class CursorRecord(BaseModel):
    """One record in a server `cursor` event's `messages` array (todo §4.4)."""

    model_config = {"arbitrary_types_allowed": True}
    time: int
    authorIndexUser: IndexUser
    # base64 over JSON (§2.4). Opaque; server never inspects.
    encryptedCursor: bytes


class ServerEventCursor(BaseModel):
    """OnlyOffice `cursor` (s->c). Name kept. Per RFC §2.2 editics changes:
    `cursor` -> `encryptedCursor` (bytes); `user`/`useridoriginal` ->
    `authorIndexUser`. OnlyOffice wraps the payload in `messages: [...]`; kept
    for symmetry. Broadcast to other participants (§6.4)."""

    type: Literal["cursor"] = "cursor"
    messages: list[CursorRecord]


class ServerEventGetLock(BaseModel):
    """OnlyOffice `getLock` (s->c). Name kept. Kept as-is per RFC §2.2. The full
    lock table as it stands after the server attempted to acquire the requested
    blocks for the requester. Broadcast to *everyone* (including the sender).
    `locks` is an object keyed by the block key; each record is
    { time, user, block } (OnlyOffice names kept: `user` here is the indexUser
    of the holder, despite the bad name -- documented at the definition site)."""

    type: Literal["getLock"] = "getLock"
    locks: dict[str, dict[str, Any]]  # block_key -> { time, user, block }


class ReleaseLockRecord(BaseModel):
    """One record in a `releaseLock` event / the `locks` field of a
    `saveChanges` broadcast (todo §4.4)."""

    model_config = {"arbitrary_types_allowed": True}
    block: Any  # opaque block descriptor (re-broadcast as-is)
    # OnlyOffice name `user` kept (bad name documented): the indexUser of the
    # holder who released the lock.
    user: IndexUser
    time: int
    # Always null here (OnlyOffice shape; present for consistency with
    # `saveChanges`'s `locks` field).
    changes: None = None


class ServerEventReleaseLock(BaseModel):
    """OnlyOffice `releaseLock` (s->c). Name kept. Broadcast to others when a
    user releases region locks outside of a `saveChanges` (i.e. from
    `unLockDocument{releaseLocks:true}` or disconnect cleanup). Per RFC §2.2 the
    `user` field is the holder's indexUser (bad name documented)."""

    type: Literal["releaseLock"] = "releaseLock"
    locks: list[ReleaseLockRecord]


class ServerEventSaveLock(BaseModel):
    """OnlyOffice `saveLock` (s->c). Name kept. Reply to `isSaveLock` (c->s).
    `saveLock: true` means denied (someone holds it / client is desynced);
    `false` means granted."""

    type: Literal["saveLock"] = "saveLock"
    saveLock: bool


class SaveChangeRecord(BaseModel):
    """One record in a server `saveChanges` broadcast's `changes` array."""

    model_config = {"arbitrary_types_allowed": True}
    time: int
    authorIndexUser: IndexUser
    # Opaque encrypted blob (base64 over JSON, §2.4); server never inspects.
    change: bytes


class ServerEventSaveChanges(BaseModel):
    """OnlyOffice `saveChanges` (s->c, broadcast to *other* participants). Name
    kept. Per RFC §2.2 editics changes: `changes` -> list of records each
    carrying the opaque encrypted blob (not the OnlyOffice `{docid, change,
    time, user, useridoriginal}`); `excelAdditionalInfo` split into
    `encryptedCursor` + `excel_info`."""

    type: Literal["saveChanges"] = "saveChanges"
    # One record per change fragment across all chunks of this save.
    changes: list[SaveChangeRecord]
    changesIndex: int  # new save point after this save (§2.2)
    syncChangesIndex: int  # always-advancing total (§2.2)
    endSaveChanges: bool  # mirrors the originator's flag
    # Locks released by the originator in this save (only when its
    # `releaseLocks` was true). Same shape as `releaseLock` records.
    locks: list[ReleaseLockRecord] = Field(default_factory=list)
    excel_info: dict[str, Any] | None = None
    # base64 over JSON (§2.4). Opaque; server never inspects.
    encryptedCursor: bytes | None = None


class ServerEventSavePartChanges(BaseModel):
    """OnlyOffice `savePartChanges` (s->c, reply to the saver for intermediate
    chunks). Name kept. `changesIndex` is -1 except for the first chunk of a
    non-truncating save (§2.2). `syncChangesIndex` always advances."""

    type: Literal["savePartChanges"] = "savePartChanges"
    changesIndex: int  # -1 except first non-truncating chunk
    syncChangesIndex: int  # always-advancing total


class ServerEventUnSaveLock(BaseModel):
    """OnlyOffice `unSaveLock` (s->c). Name kept. Two uses (RFC §2.2):
    1. Cancellation: reply to `unSaveLock` (c->s) -> index/time/sync = -1.
    2. Success: reply to a final `saveChanges` chunk -> real values."""

    type: Literal["unSaveLock"] = "unSaveLock"
    index: int  # save point, or -1 on cancel
    time: int  # last change time, or -1 on cancel
    syncChangesIndex: int  # new total, or -1 on cancel


class ServerEventDrop(BaseModel):
    """OnlyOffice `drop` (s->c). Name kept. Kept as-is (RFC §2.2). Sent to a
    participant the server is force-removing (e.g. duplicate participant
    detection, or future permission revocation)."""

    type: Literal["drop"] = "drop"
    code: int = 4007  # OnlyOffice DROP_CODE constant
    description: str = ""


class ServerEventWarning(BaseModel):
    """OnlyOffice `warning` (s->c). Name kept. Kept as-is (RFC §2.2). Shape only
    in step 1 (not actively triggered)."""

    type: Literal["warning"] = "warning"
    code: int
    message: str


ServerEvent = Annotated[
    ServerEventAuth
    | ServerEventAuthRejected
    | ServerEventConnectState
    | ServerEventAuthChanges
    | ServerEventWaitAuth
    | ServerEventMessage
    | ServerEventCursor
    | ServerEventGetLock
    | ServerEventReleaseLock
    | ServerEventSaveLock
    | ServerEventSaveChanges
    | ServerEventSavePartChanges
    | ServerEventUnSaveLock
    | ServerEventDrop
    | ServerEventWarning,
    Field(discriminator="type"),
]


# --- SSE channel (component-layer handle) ---------------------------------


# Buffer size for the per-connection SSE event queue. Step 1 produces more
# events than step 0 (change broadcasts, cursors, lock tables), so keep a
# comfortable buffer to avoid backpressure drops during bursts.
SSE_CHANNEL_BUFFER = 64


class EditicsSseChannel:
    """One participant's SSE connection (component-layer handle).

    The channel is created when the client opens the `GET .../join` SSE stream
    but is *pending* until the matching `auth` RPC arrives: until then it is
    not a participant of the session and does not receive `connectState`
    broadcasts (todo step_0 §6).

    The server pushes server events by calling `send_nowait` (or `send`); the
    ASGI route's `EditicsStreamingResponse` (in `parsec/asgi/editics.py`)
    consumes them from the receive stream and frames them as SSE `data:` lines.

    This class lives in the components layer (and not next to the SSE framing
    in `parsec/asgi/editics.py`) because it is the handle passed between the
    editics component and the ASGI route — mirroring how
    `ClientBroadcastableEventStream` for the events SSE route is a components
    type consumed by `StreamingResponseMiddleware` in `parsec/asgi/rpc.py`.
    It only depends on `anyio`, so there is no import cycle.
    """

    def __init__(self, participant_uuid: UUID, keepalive: float) -> None:
        self.participant_uuid = participant_uuid
        self.keepalive = keepalive
        self._send, self._recv = anyio.create_memory_object_stream[dict[str, Any]](
            max_buffer_size=SSE_CHANNEL_BUFFER
        )
        # `pending` is True from creation until the matching `auth` RPC promotes
        # the channel to a full participant. While pending, the channel is
        # registered in `Session.pending` (not `Session.connections`).
        self.pending: bool = True
        self.closed: bool = False
        # Filled in when the channel is promoted to a full participant by the
        # `auth` RPC (used to build the `ServerEventAuth.sessionTimeConnect` field
        # and to find the participant on leave). They are not part of the SSE
        # protocol itself.
        self.index_user: int | None = None
        self.connect_time_ms: int | None = None

    @property
    def receive(self) -> MemoryObjectReceiveStream[dict[str, Any]]:
        return self._recv

    def send_nowait(self, event: dict[str, Any]) -> None:
        """Enqueue a server event to be delivered over this SSE stream.

        Raises `anyio.WouldBlock` if the buffer is full (backpressure); the
        component closes the channel in that case (mirrors the events SSE
        backpressure handling in `parsec/asgi/rpc.py`).
        """
        if self.closed:
            return
        self._send.send_nowait(event)

    async def send(self, event: dict[str, Any]) -> None:
        if self.closed:
            return
        await self._send.send(event)

    def close(self) -> None:
        if self.closed:
            return
        self.closed = True
        self._send.close()
        self._recv.close()


# --- In-memory session state (step 1) ---------------------------------------


@dataclass
class StoredChange:
    """One change fragment stored in the session history (RFC §2.2).

    The `blob` is an opaque encrypted blob (§2.4); the server never inspects
    it. `index` is 1-based and monotonic across the session.
    """

    index: int  # 1-based, monotonic
    time_ms: int  # server timestamp at store time
    author_index_user: IndexUser  # who produced it (participant index)
    blob: bytes  # opaque encrypted change fragment


@dataclass
class RegionLock:
    """One region lock (RFC §2.1). `block_key` is the JSON-serialized block
    descriptor used as the dict key in `session.region_locks`."""

    holder_index_user: IndexUser
    time_ms: int
    block: Any  # opaque block descriptor (re-broadcast as-is)


@dataclass
class ChatMessage:
    """One chat message kept in the session history."""

    time_ms: int
    author_index_user: IndexUser
    encrypted_message: bytes  # opaque (§2.4)


@dataclass
class InFlightSave:
    """Buffered fragments for an in-flight multi-chunk save (RFC §2.2).

    The server buffers per-fragment `StoredChange`s across chunks and emits a
    single `saveChanges` broadcast on the final chunk, matching OnlyOffice
    (other participants see one `saveChanges` per save, not per chunk).
    """

    # Save point computed on the first chunk (OnlyOffice semantics, §2.2):
    # a real index only for the first chunk of a non-truncating save; -1
    # otherwise. Broadcast as `changesIndex` on the final chunk.
    changes_index: int
    # Accumulated per-fragment records (in chunk order) to be broadcast.
    fragments: list[StoredChange] = field(default_factory=list)
    # Last seen `excel_info` / `encryptedCursor` (taken from the final chunk;
    # OnlyOffice sends them on every chunk but the broadcast only happens on
    # the final one).
    excel_info: dict[str, Any] | None = None
    encrypted_cursor: bytes | None = None


@dataclass
class EditicsSession:
    """In-memory state of a single edition session (RFC §1.3, todo §5).

    A session is identified by the pair `(workspace_id, vlob_id)` (both are
    `VlobID`). No PostgreSQL persistence in step 1: state is lost on server
    restart.
    """

    workspace_id: VlobID
    vlob_id: VlobID
    initial_version: int
    latest_allowed_version: int
    next_index: int = 1  # monotonic, starts at 1
    # Monotonic non-decreasing timestamp (ms) of the last `connectState`
    # broadcast (todo step_1 §6.1 V-A3). Bumped on every participant-set change.
    participants_timestamp: int = 0
    # indexUser -> deviceId
    participants: dict[IndexUser, DeviceID] = field(default_factory=dict)
    # participant_uuid (client-generated) -> SSE channel
    connections: dict[UUID, EditicsSseChannel] = field(default_factory=dict)
    # --- step 1 additions ---
    # Monotonic change counter (= OnlyOffice `puckerIndex`).
    sync_changes_index: int = 0
    # Ordered change history (1-based OnlyOffice index = position + 1).
    changes: list[StoredChange] = field(default_factory=list)
    # Chat history (delivered to newcomers on join).
    messages: list[ChatMessage] = field(default_factory=list)
    # Save lock: the participant index currently holding it, or None.
    save_lock_holder: IndexUser | None = None
    # The participant that just released the save lock (one-round only, §6.9):
    # only this participant may send `saveDone` to bump `latest_allowed_version`.
    last_save_lock_holder: IndexUser | None = None
    # In-flight multi-chunk save buffer for the current lock holder, or None.
    in_flight_save: InFlightSave | None = None
    # Auth lock: the participant index currently holding it, or None.
    # Held by the first non-view participant until `unLockDocument{unlock:true}`.
    auth_lock_holder: IndexUser | None = None
    # Participants parked behind the auth lock (in join order), waiting for
    # the holder to release. Each entry is the participant's channel + index.
    parked: list[tuple[EditicsSseChannel, IndexUser]] = field(default_factory=list)
    # Region locks, keyed by the JSON-serialized block descriptor.
    region_locks: dict[str, RegionLock] = field(default_factory=dict)


@dataclass
class EditicsClientContext:
    """Parsed `Authorization: Editics` header.

    For step 0/1 we do NOT use the Parsec `PARSEC-SIGN-ED25519` token. Instead the
    client sends a lightweight identity header on both routes
    (`Authorization: Editics <device_id_hex>.<participant_uuid_hex>`).

    ⚠️ This is NOT a secure authorization system. It is intentionally simple
    and convenient. It will be replaced by proper Parsec authentication in a
    later step. The server must not trust this header for access-control
    decisions beyond "this is the participant UUID the client wants to be
    known by".
    """

    device_id: DeviceID
    participant_uuid: UUID


# --- Component ---------------------------------------------------------------
#
# The logic below is in-memory by nature (live session state is transient). It
# lives on the base class so that both `MemoryEditicsComponent` and
# `PostgresEditicsComponent` share it; the postgres implementation will only
# diverge once durable events (document changes / the change backlog) are
# added in a later step.


class BaseEditicsComponent:
    """Editics component base (step 1).

    Sessions are kept in a process-local dict keyed by `(workspace_id, vlob_id)`.
    No PostgreSQL persistence in step 1: state is lost on server restart.

    Pending SSE connections (opened via `GET .../join` but not yet authenticated
    via the matching `auth` RPC) are tracked in a separate top-level dict keyed
    by `(workspace_id, vlob_id, participant_uuid)`. This keeps the `Session`
    model consistent while still allowing the SSE join to happen *before* the
    session is created: the session is created on the `auth` RPC, not on the
    SSE join (todo §6).

    Note on `organization_id`: it is accepted for API symmetry with the other
    components and for future per-organization isolation, but is *not* part of
    the session key in step 1 (a `(workspace_id, vlob_id)` pair is globally
    unique across organizations in practice). This is acceptable for the
    in-memory validation and will be revisited with persistence.
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
            del self._sessions[key]

    def _drop_session(self, workspace_id: VlobID, vlob_id: VlobID) -> None:
        # Unconditionally remove the session (and close any live participant
        # channels). Used to self-heal malformed sessions (e.g. an
        # `initial_version < 1` session left by a buggy client) so the next
        # valid join recreates a proper session.
        key = (workspace_id, vlob_id)
        session = self._sessions.pop(key, None)
        if session is None:
            return
        for channel in session.connections.values():
            channel.close()
        session.connections.clear()
        session.participants.clear()

    @staticmethod
    def _participants_list(session: EditicsSession) -> list[ParticipantEntry]:
        return [
            ParticipantEntry(indexUser=index, deviceId=device_id)
            for index, device_id in sorted(session.participants.items())
        ]

    def _now_ms(self, session: EditicsSession) -> int:
        # Wall-clock ms, but never let the session's participant-set timestamp
        # go backwards (todo step_1 §6.1 V-A3).
        now_ms = int(time.time() * 1000)
        if now_ms <= session.participants_timestamp:
            now_ms = session.participants_timestamp + 1
        return now_ms

    # --- Broadcast helpers --------------------------------------------------

    def _broadcast_connect_state(self, session: EditicsSession, waitAuth: bool = False) -> None:
        session.participants_timestamp = self._now_ms(session)
        event = ServerEventConnectState(
            participantsTimestamp=session.participants_timestamp,
            participants=self._participants_list(session),
            waitAuth=waitAuth,
        )
        self._broadcast(session, event)

    def _broadcast(
        self, session: EditicsSession, event: BaseModel, *, exclude: set[UUID] | None = None
    ) -> None:
        # `model_dump(mode="json")` applies the pydantic serializers (e.g. bytes
        # -> base64 str) so the payload is JSON-serializable by the SSE framer.
        payload = event.model_dump(mode="json")
        for participant_uuid, channel in session.connections.items():
            if exclude and participant_uuid in exclude:
                continue
            try:
                channel.send_nowait(payload)
            except anyio.WouldBlock:
                logger.warning("editics: dropping participant due to backpressure")
                self._drop_participant(session, channel)
            except Exception:
                logger.warning("editics: dropping participant due to send error")
                self._drop_participant(session, channel)

    def _send_to(self, channel: EditicsSseChannel, event: BaseModel) -> None:
        try:
            channel.send_nowait(event.model_dump(mode="json"))
        except anyio.WouldBlock:
            logger.warning("editics: dropping participant due to backpressure")
        except Exception:
            logger.warning("editics: dropping participant due to send error")

    def _drop_participant(self, session: EditicsSession, channel: EditicsSseChannel) -> None:
        # Best-effort cleanup of a backpressure-failing channel; full leave is
        # handled by the route's `finally` on disconnect. Close the channel so
        # the SSE generator returns and the leave flow runs.
        channel.close()

    # --- Public API: SSE join ------------------------------------------------

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
        to a full participant. A new join replaces (and closes) a stale pending
        connection for the same key.
        """
        key = (workspace_id, vlob_id, client_ctx.participant_uuid)
        old = self._pending.pop(key, None)
        if old is not None:
            old.close()
        self._pending[key] = channel

    # --- Public API: client event dispatch -----------------------------------

    async def handle_client_event(
        self,
        organization_id: OrganizationID,
        workspace_id: VlobID,
        vlob_id: VlobID,
        client_ctx: EditicsClientContext,
        event: ClientEvent,  # type: ignore[override]
    ) -> BaseModel | None:
        """Handle a client->server event and return the RPC reply (a server
        event to the sender, or None for 204). Broadcasts to other participants
        are enqueued on their SSE channels as a side effect."""
        session = self._get_session(workspace_id, vlob_id)
        # `auth` is the only event allowed before the session/participant exists.
        if isinstance(event, ClientEventAuth):
            return await self._handle_auth(workspace_id, vlob_id, client_ctx, event, session)
        # All other events require an established participant.
        if session is None:
            return None
        channel = session.connections.get(client_ctx.participant_uuid)
        if channel is None or channel.index_user is None:
            return None
        index_user: IndexUser = channel.index_user
        match event:
            case ClientEventAuthChangesAck():
                return await self._handle_auth_changes_ack(session, channel, index_user)
            case ClientEventMessage():
                return self._handle_message(session, index_user, event)
            case ClientEventCursor():
                return self._handle_cursor(session, index_user, event)
            case ClientEventGetLock():
                return self._handle_get_lock(session, index_user, event)
            case ClientEventIsSaveLock():
                return self._handle_is_save_lock(session, index_user, event)
            case ClientEventSaveChanges():
                return self._handle_save_changes(session, index_user, event)
            case ClientEventUnSaveLock():
                return self._handle_un_save_lock(session, index_user, event)
            case ClientEventUnLockDocument():
                return self._handle_un_lock_document(session, index_user, event)
            case ClientEventClose():
                return await self._handle_close(workspace_id, vlob_id, client_ctx, session, channel)
            case ClientEventSaveDone():
                return self._handle_save_done(session, index_user, event)
            case _:  # pragma: no cover
                return None

    # --- Substep A: auth / join --------------------------------------------

    async def _handle_auth(
        self,
        workspace_id: VlobID,
        vlob_id: VlobID,
        client_ctx: EditicsClientContext,
        event: ClientEventAuth,
        session: EditicsSession | None,
    ) -> BaseModel:
        # A Parsec vlob version is always >= 1; `base_version: 0` is a purely
        # local-manifest placeholder and is never a valid server vlob version.
        # RFC §1.3 requires a document to exist as a vlob before editing.
        if event.vlobVersion < 1:
            return ServerEventAuthRejected(latestAllowedVersion=0)
        # Self-heal malformed sessions left with an invalid `initial_version < 1`
        # (only producible by a buggy pre-fix client) so the next valid join
        # recreates a proper session without a server restart.
        if session is not None and session.initial_version < 1:
            self._drop_session(workspace_id, vlob_id)
            session = None
        if session is None:
            session = self._get_or_create_session(
                workspace_id, vlob_id, initial_version=event.vlobVersion
            )
            session.latest_allowed_version = session.initial_version
        else:
            if event.vlobVersion < session.initial_version:
                return ServerEventAuthRejected(latestAllowedVersion=session.initial_version)
            if event.vlobVersion > session.latest_allowed_version:
                return ServerEventAuthRejected(latestAllowedVersion=session.latest_allowed_version)

        # Promote the pending SSE connection.
        key = (workspace_id, vlob_id, client_ctx.participant_uuid)
        channel = self._pending.pop(key, None)
        if channel is None:
            return ServerEventAuthRejected(latestAllowedVersion=session.latest_allowed_version)
        channel.pending = False
        channel.connect_time_ms = int(time.time() * 1000)
        session.connections[client_ctx.participant_uuid] = channel

        # Assign indexUser & register the participant.
        index_user = session.next_index
        session.next_index += 1
        session.participants[index_user] = client_ctx.device_id
        channel.index_user = index_user

        # Substep B: auth lock. The first non-view participant holds the auth
        # lock until it sends `unLockDocument{unlock:true}`. (In step 1 all
        # participants are non-view, §2.5.)
        auth_lock_just_taken = False
        if session.auth_lock_holder is None and len(session.participants) == 1:
            session.auth_lock_holder = index_user
            auth_lock_just_taken = True

        participants = self._participants_list(session)
        auth_reply = ServerEventAuth(
            result=1,
            participants=participants,
            indexUser=index_user,
            sessionId=uuid4().hex,
            sessionTimeConnect=channel.connect_time_ms,
        )

        # If the auth lock is held by someone else, the newcomer is parked:
        # reply `waitAuth` (RPC) and broadcast `connectState{waitAuth:true}`.
        if session.auth_lock_holder is not None and session.auth_lock_holder != index_user:
            # Park the newcomer: it has joined the participant map but its
            # editor handshake is paused until the holder releases the lock.
            session.parked.append((channel, index_user))
            # Broadcast waitAuth:true to everyone (holder included) -- the
            # nudge for the holder to release the lock.
            self._broadcast_connect_state(session, waitAuth=True)
            return ServerEventWaitAuth(authLockedBy=session.auth_lock_holder)

        # No auth lock held by another: proceed normally.
        # Broadcast connectState (waitAuth reflects whether *someone* is
        # parked behind the lock the newcomer may have just taken).
        waitAuth = bool(session.parked)
        self._broadcast_connect_state(session, waitAuth=waitAuth)

        # Substep C: deliver the change backlog to this participant, except
        # for the very first participant of a fresh session (step-0 behavior).
        if not auth_lock_just_taken and (
            session.changes or session.messages or len(session.participants) > 1
        ):
            self._send_auth_changes(session, channel)
        elif not session.changes and len(session.participants) == 1:
            # First participant of a fresh session with no changes: no
            # authChanges (step-0 behavior preserved).
            pass
        else:
            # Subsequent participant: send an (possibly empty) authChanges so
            # the client's handshake state machine gets a defined "backlog done"
            # signal (todo step_1 §6.3 V-C3).
            self._send_auth_changes(session, channel)

        return auth_reply

    # --- Substep C: authChanges backlog -------------------------------------

    def _send_auth_changes(self, session: EditicsSession, channel: EditicsSseChannel) -> None:
        changes = [(c.index, c.blob) for c in session.changes]
        self._send_to(channel, ServerEventAuthChanges(changes=changes))

    async def _handle_auth_changes_ack(
        self,
        session: EditicsSession,
        channel: EditicsSseChannel,
        index_user: IndexUser,
    ) -> None:
        # In step 1 the backlog is delivered in a single chunk; the ack just
        # confirms the client consumed it (todo step_1 §6.3). No reply (204).
        return None

    # --- Substep D: chat messages -------------------------------------------

    def _handle_message(
        self, session: EditicsSession, index_user: IndexUser, event: ClientEventMessage
    ) -> None:
        now_ms = int(time.time() * 1000)
        session.messages.append(
            ChatMessage(
                time_ms=now_ms,
                author_index_user=index_user,
                encrypted_message=event.encryptedMessage,
            )
        )
        record = MessageRecord(
            time=now_ms, authorIndexUser=index_user, encryptedMessage=event.encryptedMessage
        )
        # Broadcast to ALL participants including the sender (OnlyOffice echoes
        # the sender's own message back, §6.4).
        self._broadcast(session, ServerEventMessage(messages=[record]))
        return None

    # --- Substep D (cursor) -------------------------------------------------

    def _handle_cursor(
        self, session: EditicsSession, index_user: IndexUser, event: ClientEventCursor
    ) -> None:
        now_ms = int(time.time() * 1000)
        record = CursorRecord(
            time=now_ms, authorIndexUser=index_user, encryptedCursor=event.encryptedCursor
        )
        # Broadcast to other participants (not the sender).
        self._broadcast(
            session,
            ServerEventCursor(messages=[record]),
            exclude={self._uuid_for(session, index_user)},
        )
        return None

    def _uuid_for(self, session: EditicsSession, index_user: IndexUser) -> UUID | None:
        for participant_uuid, channel in session.connections.items():
            if channel.index_user == index_user:
                return participant_uuid
        return None

    # --- Substep F: region locks -------------------------------------------

    @staticmethod
    def _block_key(block: Any) -> str:
        # OnlyOffice keys its internal lock table (`_locks`/`_lockCallbacks`) by:
        #  - the plain block id string for the Document editor (Word);
        #  - `block["guid"]` for the Spreadsheet / Presentation / PDF editors.
        # The `getLock` reply's `locks` dict MUST use the same key, otherwise the
        # editor's pending-lock callback (registered under that key) never fires
        # and the lock is stuck (RFC §6.6: "keyed by their JSON key -- matching
        # OnlyOffice's per-editor re-keying"). The `block` descriptor is opaque
        # to the server, but its *type* is stable per editor type, so this rule
        # is derivable without inspecting content.
        if isinstance(block, str):
            return block
        if isinstance(block, dict):
            guid = block.get("guid")
            if isinstance(guid, str):
                return guid
        # Fallback: a stable JSON key (should not happen for the supported
        # editor types, but keeps the table consistent).
        return __import__("json").dumps(block, sort_keys=True, separators=(",", ":"))

    def _handle_get_lock(
        self, session: EditicsSession, index_user: IndexUser, event: ClientEventGetLock
    ) -> BaseModel:
        now_ms = int(time.time() * 1000)
        locks: dict[str, dict[str, Any]] = {}
        for block in event.block:
            key = self._block_key(block)
            existing = session.region_locks.get(key)
            if existing is None:
                # Acquire for the requester (if free).
                session.region_locks[key] = RegionLock(
                    holder_index_user=index_user, time_ms=now_ms, block=block
                )
                locks[key] = {"time": now_ms, "user": index_user, "block": block}
            else:
                # Already held (by someone, possibly the requester): report
                # the current state.
                locks[key] = {
                    "time": existing.time_ms,
                    "user": existing.holder_index_user,
                    "block": existing.block,
                }
        # Broadcast the resulting lock table to the OTHER participants (the
        # sender gets it as the RPC reply, §6.6). In step 1 the views are
        # identical, so the RPC body == the broadcast payload.
        reply = ServerEventGetLock(locks=locks)
        self._broadcast(session, reply, exclude={self._uuid_for(session, index_user)})
        return reply

    def _release_user_region_locks(
        self, session: EditicsSession, index_user: IndexUser
    ) -> list[ReleaseLockRecord]:
        """Remove all region locks held by `index_user`; return records for a
        standalone `releaseLock` broadcast (or to embed in a `saveChanges`
        broadcast)."""
        now_ms = int(time.time() * 1000)
        released: list[ReleaseLockRecord] = []
        for key in list(session.region_locks.keys()):
            lock = session.region_locks[key]
            if lock.holder_index_user == index_user:
                released.append(ReleaseLockRecord(block=lock.block, user=index_user, time=now_ms))
                del session.region_locks[key]
        return released

    # --- Substep E: save flow ----------------------------------------------

    def _handle_is_save_lock(
        self, session: EditicsSession, index_user: IndexUser, event: ClientEventIsSaveLock
    ) -> BaseModel:
        if (
            session.save_lock_holder is not None
            or event.syncChangesIndex != session.sync_changes_index
        ):
            # Someone holds the lock, or the client is desynced -> deny.
            return ServerEventSaveLock(saveLock=True)
        # Grant.
        session.save_lock_holder = index_user
        return ServerEventSaveLock(saveLock=False)

    def _handle_save_changes(
        self, session: EditicsSession, index_user: IndexUser, event: ClientEventSaveChanges
    ) -> BaseModel:
        # The sender must hold the save lock.
        if session.save_lock_holder != index_user:
            # Protocol violation; reject the RPC (the client must re-take the
            # lock). Step 1 returns an HTTP 400 via the ASGI layer by raising,
            # but the component returns a warning-shaped reply is not used; the
            # ASGI route turns a None-with-flag into 400. Simpler: return None
            # and let the ASGI layer 400. We signal the violation by returning
            # a sentinel handled in the ASGI route. For now, return None (204)
            # is wrong; instead raise so the ASGI route maps to 400.
            raise _SaveChangesRejected()

        now_ms = int(time.time() * 1000)

        # First chunk: compute the save point and handle UNDO truncation.
        if event.startSaveChanges:
            delete_index = event.deleteIndex
            is_truncate = delete_index is not None and delete_index != -1
            changes_index = -1 if is_truncate else (session.sync_changes_index + 1)
            if is_truncate:
                self._truncate_changes(session, delete_index)  # type: ignore[arg-type]
            session.in_flight_save = InFlightSave(changes_index=changes_index)

        in_flight = session.in_flight_save
        if in_flight is None:
            # A saveChanges without a start chunk: protocol violation.
            raise _SaveChangesRejected()

        # Store one StoredChange per fragment and advance sync_changes_index.
        for blob in event.encryptedChanges:
            session.sync_changes_index += 1
            fragment = StoredChange(
                index=session.sync_changes_index,
                time_ms=now_ms,
                author_index_user=index_user,
                blob=blob,
            )
            session.changes.append(fragment)
            in_flight.fragments.append(fragment)

        # Carry through excel_info / encryptedCursor (taken from the final
        # chunk; OnlyOffice sends them on every chunk but the broadcast only
        # happens on the final one).
        if event.excel_info is not None:
            in_flight.excel_info = event.excel_info
        if event.encryptedCursor is not None:
            in_flight.encrypted_cursor = event.encryptedCursor

        # Intermediate chunk: ack with savePartChanges, no broadcast.
        if not event.endSaveChanges:
            return ServerEventSavePartChanges(
                changesIndex=in_flight.changes_index,
                syncChangesIndex=session.sync_changes_index,
            )

        # Final chunk: broadcast to others, reply unSaveLock to the saver.
        released_locks: list[ReleaseLockRecord] = []
        if event.releaseLocks:
            released_locks = self._release_user_region_locks(session, index_user)

        save_point = in_flight.changes_index
        last_time = in_flight.fragments[-1].time_ms if in_flight.fragments else now_ms
        # Build the broadcast (one record per fragment across all chunks).
        broadcast_changes = [
            SaveChangeRecord(time=f.time_ms, authorIndexUser=f.author_index_user, change=f.blob)
            for f in in_flight.fragments
        ]
        broadcast = ServerEventSaveChanges(
            changes=broadcast_changes,
            changesIndex=save_point,
            syncChangesIndex=session.sync_changes_index,
            endSaveChanges=True,
            locks=released_locks,
            excel_info=in_flight.excel_info,
            encryptedCursor=in_flight.encrypted_cursor,
        )
        # Broadcast to other participants (not the saver).
        self._broadcast(session, broadcast, exclude={self._uuid_for(session, index_user)})

        # Clear the save lock and remember the last holder for `saveDone`.
        session.save_lock_holder = None
        session.in_flight_save = None
        session.last_save_lock_holder = index_user

        return ServerEventUnSaveLock(
            index=save_point,
            time=last_time,
            syncChangesIndex=session.sync_changes_index,
        )

    def _handle_un_save_lock(
        self, session: EditicsSession, index_user: IndexUser, event: ClientEventUnSaveLock
    ) -> BaseModel:
        # Only the current lock holder may cancel.
        if session.save_lock_holder != index_user:
            return None
        session.save_lock_holder = None
        session.in_flight_save = None
        # Cancellation: index/time/sync = -1 (RFC §2.2).
        return ServerEventUnSaveLock(index=-1, time=-1, syncChangesIndex=-1)

    def _truncate_changes(self, session: EditicsSession, delete_index: int) -> None:
        # OnlyOffice semantics: delete all stored changes with index >=
        # deleteIndex; recompute sync_changes_index = deleteIndex - 1 (the new
        # total is the index of the last surviving change).
        session.changes = [c for c in session.changes if c.index < delete_index]
        session.sync_changes_index = max(delete_index - 1, 0)

    # --- Substep G: unLockDocument ------------------------------------------

    def _handle_un_lock_document(
        self, session: EditicsSession, index_user: IndexUser, event: ClientEventUnLockDocument
    ) -> BaseModel | None:
        reply: BaseModel | None = None
        # 1. isSave: release the save lock if held (cancellation semantics).
        if event.isSave and session.save_lock_holder == index_user:
            session.save_lock_holder = None
            session.in_flight_save = None
            reply = ServerEventUnSaveLock(index=-1, time=-1, syncChangesIndex=-1)
        # 2. deleteIndex: UNDO while leaving.
        if event.deleteIndex is not None and event.deleteIndex != -1:
            self._truncate_changes(session, event.deleteIndex)
        # 3. releaseLocks: release this user's region locks (standalone
        #    releaseLock broadcast to others).
        if event.releaseLocks:
            released = self._release_user_region_locks(session, index_user)
            if released:
                self._broadcast(
                    session,
                    ServerEventReleaseLock(locks=released),
                    exclude={self._uuid_for(session, index_user)},
                )
        # 4. unlock: clear the auth lock if held by the sender, unblock parked.
        if event.unlock and session.auth_lock_holder == index_user:
            session.auth_lock_holder = None
            self._unblock_parked(session)
            self._broadcast_connect_state(session, waitAuth=False)
        # Fire-and-forget otherwise: 204 unless isSave produced an unSaveLock.
        return reply

    def _unblock_parked(self, session: EditicsSession) -> None:
        # Deliver the deferred join sequence to each parked participant (in
        # join order): authChanges (backlog), then connectState{waitAuth:false}
        # is broadcast below. The parked participant's `auth` RPC reply was
        # the `waitAuth`; the completion is delivered over SSE.
        parked = session.parked
        session.parked = []
        for channel, _index_user in parked:
            self._send_auth_changes(session, channel)
        # The connectState{waitAuth:false} broadcast is issued by the caller.

    # --- Substep I: saveDone (vlob version bump) ----------------------------

    def _handle_save_done(
        self, session: EditicsSession, index_user: IndexUser, event: ClientEventSaveDone
    ) -> None:
        # Only the participant that just released the save lock may bump.
        if session.last_save_lock_holder != index_user:
            return None
        session.latest_allowed_version = max(session.latest_allowed_version, event.newVersion)
        # One-round only: clear the marker.
        session.last_save_lock_holder = None
        return None

    # --- Substep H: close / leave ------------------------------------------

    async def _handle_close(
        self,
        workspace_id: VlobID,
        vlob_id: VlobID,
        client_ctx: EditicsClientContext,
        session: EditicsSession,
        channel: EditicsSseChannel,
    ) -> None:
        # Voluntary leave: run the same cleanup as SSE disconnect.
        self._leave_session(workspace_id, vlob_id, client_ctx, session, channel)
        return None

    async def leave(
        self,
        workspace_id: VlobID,
        vlob_id: VlobID,
        client_ctx: EditicsClientContext,
    ) -> None:
        """Remove a participant on SSE disconnect (todo step_0 §8 / step_1 §6.8)."""
        key3 = (workspace_id, vlob_id, client_ctx.participant_uuid)
        # Remove from pending (never authenticated).
        channel = self._pending.pop(key3, None)
        if channel is not None:
            channel.close()
            return
        session = self._get_session(workspace_id, vlob_id)
        if session is None:
            return
        channel = session.connections.get(client_ctx.participant_uuid)
        if channel is None:
            return
        self._leave_session(workspace_id, vlob_id, client_ctx, session, channel)

    def _leave_session(
        self,
        workspace_id: VlobID,
        vlob_id: VlobID,
        client_ctx: EditicsClientContext,
        session: EditicsSession,
        channel: EditicsSseChannel,
    ) -> None:
        index_user: int | None = channel.index_user
        # Remove the connection and participant.
        session.connections.pop(client_ctx.participant_uuid, None)
        channel.close()
        if index_user is not None:
            session.participants.pop(index_user, None)
            # Release the save lock if held (no event to the leaver).
            if session.save_lock_holder == index_user:
                session.save_lock_holder = None
                session.in_flight_save = None
            # Release the auth lock if held -> unblock parked participants.
            if session.auth_lock_holder == index_user:
                session.auth_lock_holder = None
                self._unblock_parked(session)
            # Release the leaver's region locks -> standalone releaseLock to
            # the remaining participants.
            released = self._release_user_region_locks(session, index_user)
            if released and session.connections:
                self._broadcast(session, ServerEventReleaseLock(locks=released))
            # Clear the one-round saveDone marker if it was the leaver.
            if session.last_save_lock_holder == index_user:
                session.last_save_lock_holder = None
            # Drop the leaver from the parked list if it was parked.
            session.parked = [(c, i) for (c, i) in session.parked if c is not channel]

        if session.participants:
            # Broadcast the updated participant set to the remaining ones.
            waitAuth = bool(session.parked)
            self._broadcast_connect_state(session, waitAuth=waitAuth)
        else:
            self._drop_session_if_empty(workspace_id, vlob_id)


class _SaveChangesRejected(Exception):
    """Raised when a `saveChanges` arrives without the save lock held; the ASGI
    route maps it to HTTP 400 (todo step_1 §6.5.2)."""


__all__ = [
    "BaseEditicsComponent",
    "ChatMessage",
    "ClientEvent",
    "ClientEventAuth",
    "ClientEventAuthChangesAck",
    "ClientEventClose",
    "ClientEventCursor",
    "ClientEventGetLock",
    "ClientEventIsSaveLock",
    "ClientEventMessage",
    "ClientEventSaveChanges",
    "ClientEventSaveDone",
    "ClientEventUnLockDocument",
    "ClientEventUnSaveLock",
    "EditicsClientContext",
    "EditicsSession",
    "EditicsSseChannel",
    "InFlightSave",
    "IndexUser",
    "ParticipantEntry",
    "RegionLock",
    "ServerEvent",
    "ServerEventAuth",
    "ServerEventAuthChanges",
    "ServerEventAuthRejected",
    "ServerEventConnectState",
    "ServerEventCursor",
    "ServerEventDrop",
    "ServerEventGetLock",
    "ServerEventMessage",
    "ServerEventReleaseLock",
    "ServerEventSaveChanges",
    "ServerEventSaveLock",
    "ServerEventSavePartChanges",
    "ServerEventUnSaveLock",
    "ServerEventWaitAuth",
    "ServerEventWarning",
    "StoredChange",
]
