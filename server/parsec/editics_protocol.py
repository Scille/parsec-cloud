# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

"""
The editics protocol schemas is defined here, basically:
- Editics protocol is based on the OnlyOffice protocol (typically removing unneeded
  field and adding encryption to sensitive ones).
- The main exposed classes are `EditicsProtocolEditicsProtocolClientEvent` and `EditicsProtocolEditicsProtocolServerEvent`

See [RFC 1030](../../docs/rfcs/1030-collaborative-editics.md).
"""

from __future__ import annotations

from typing import Annotated, Any, Literal

from pydantic import BaseModel, Field, TypeAdapter

from parsec.types import DeviceIDField

# `indexUser`: 1-based, monotonic per session, assigned by order of join.
# OnlyOffice client code does arithmetic with this index (RFC §1.4), so it
# must stay a plain int.
EditicsProtocolIndexUser = int


class EditicsProtocolParticipantEntry(BaseModel):
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
    indexUser: EditicsProtocolIndexUser
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


class EditicsProtocolClientEventAuth(BaseModel):
    type: Literal["auth"] = "auth"
    # -1 on first open; server-assigned index on reconnect
    indexUser: int = -1
    # 0=Word, 1=Spreadsheet, 2=Presentation, 3=Visio.
    # Informational; server stores, ignores. (Kept for now; may be dropped later.)
    editorType: int
    # The vlob version the client has loaded locally (RFC §1.2). For a fresh
    # session this becomes the session's initial version.
    # (OnlyOffice has no such field; editics addition.)
    vlobVersion: int


class EditicsProtocolClientEventAuthChangesAck(BaseModel):
    """OnlyOffice `authChangesAck` (c->s). Name kept.

    Acknowledges one `authChanges` chunk (RFC §2.2). In step 1 the backlog is
    delivered in a single chunk (todo step_1 §6.3), so at most one ack is
    expected per join.
    """

    type: Literal["authChangesAck"] = "authChangesAck"


class EditicsProtocolClientEventMessage(BaseModel):
    """OnlyOffice `message` (c->s). Name kept. The chat message is encrypted
    (§2.4); renamed to `encryptedMessage` per RFC §2.2 editics changes."""

    type: Literal["message"] = "message"
    # base64 over JSON (§2.4). Opaque; the server never inspects the content.
    encryptedMessage: bytes


class EditicsProtocolClientEventCursor(BaseModel):
    """OnlyOffice `cursor` (c->s). Name kept. The cursor is encrypted (§2.4);
    renamed to `encryptedCursor` per RFC §2.2 editics changes."""

    type: Literal["cursor"] = "cursor"
    # base64 over JSON (§2.4). Opaque; the server never inspects the content.
    encryptedCursor: bytes


class EditicsProtocolClientEventGetLock(BaseModel):
    """OnlyOffice `getLock` (c->s). Name kept. Kept as-is per RFC §2.2 (the
    `block` array is opaque to the server; keyed by JSON serialization)."""

    type: Literal["getLock"] = "getLock"
    # Opaque block descriptors (shape depends on editor type).
    block: list[Any]


class EditicsProtocolClientEventIsSaveLock(BaseModel):
    """OnlyOffice `isSaveLock` (c->s). Name kept. The client's current
    always-advancing sync point (§2.2). Used by the server to detect a
    desynchronized client (keep denying the lock until it catches up)."""

    type: Literal["isSaveLock"] = "isSaveLock"
    syncChangesIndex: int


class EditicsProtocolClientEventSaveChanges(BaseModel):
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


class EditicsProtocolClientEventUnSaveLock(BaseModel):
    """OnlyOffice `unSaveLock` (c->s). Name kept. Cancel an in-progress save and
    release the save lock without saving (RFC §2.2). Kept as-is."""

    type: Literal["unSaveLock"] = "unSaveLock"


class EditicsProtocolClientEventUnLockDocument(BaseModel):
    """OnlyOffice `unLockDocument` (c->s). Name kept. Fire-and-forget cleanup
    combining up to three independent actions (RFC §2.2). Kept as-is."""

    type: Literal["unLockDocument"] = "unLockDocument"
    isSave: bool = False
    unlock: bool = False
    deleteIndex: int | None = None
    releaseLocks: bool = False


class EditicsProtocolClientEventClose(BaseModel):
    """OnlyOffice `close` (c->s). Name kept. Voluntary leave + close (unlike
    `unLockDocument` which keeps the connection alive). Kept as-is."""

    type: Literal["close"] = "close"


class EditicsProtocolClientEventSaveDone(BaseModel):
    """Editics addition (no OnlyOffice equivalent). Sent by the client after it
    has uploaded a new vlob version, to bump the session's
    `latest_allowed_version` (RFC §1.2 step 4.3). Must be sent only by the
    participant that just released the save lock (todo step_1 §6.9)."""

    type: Literal["saveDone"] = "saveDone"
    savedUpToIndex: int  # all changes up to this index are in the new version
    newVersion: int  # the new vlob version just uploaded


EditicsProtocolClientEvent = Annotated[
    EditicsProtocolClientEventAuth
    | EditicsProtocolClientEventAuthChangesAck
    | EditicsProtocolClientEventMessage
    | EditicsProtocolClientEventCursor
    | EditicsProtocolClientEventGetLock
    | EditicsProtocolClientEventIsSaveLock
    | EditicsProtocolClientEventSaveChanges
    | EditicsProtocolClientEventUnSaveLock
    | EditicsProtocolClientEventUnLockDocument
    | EditicsProtocolClientEventClose
    | EditicsProtocolClientEventSaveDone,
    Field(discriminator="type"),
]
EditicsProtocolClientEventAdapter = TypeAdapter(EditicsProtocolClientEvent)


# --- Server -> client events -------------------------------------------------


class EditicsProtocolServerEventAuth(BaseModel):
    """OnlyOffice server `auth` reply, trimmed. Name kept.

    See RFC §2.2 / todo step_0 §4.4 for the fields dropped from the OnlyOffice
    `auth` (server->client): jwt, messages, locks, hasForgotten,
    g_cAscSpellCheckUrl, buildVersion, buildNumber, licenseType, settings,
    openedAt, docid (carried by the URL path). The change backlog is NOT folded
    in: it is delivered as a separate `authChanges` SSE event (RFC §2.2).
    """

    type: Literal["auth"] = "auth"
    result: int = 1  # 1 = success (OnlyOffice convention)
    participants: list[EditicsProtocolParticipantEntry]  # current participant map
    indexUser: EditicsProtocolIndexUser  # this connection's assigned index
    # Reconnect info (forward-compat; unused in step 0 but kept).
    sessionId: str
    sessionTimeConnect: int  # server timestamp (ms) at connect


class EditicsProtocolServerEventAuthRejected(BaseModel):
    """`auth` reply shape reused for rejection (RFC §1.2).

    On rejection the RPC returns this instead of `EditicsProtocolServerEventAuth`, with a
    non-success `result` and the allowed version. OnlyOffice uses `result`
    codes; we reuse the field (bad name documented at the definition site).
    """

    type: Literal["auth_rejected"] = "auth_rejected"  # TODO: dummy type not to clash with auth
    result: int = 0  # 0 = rejected (OnlyOffice: non-1 = failure)
    # RFC §1.2: the version the client should reload to before retrying.
    latestAllowedVersion: int


class EditicsProtocolServerEventConnectState(BaseModel):
    """OnlyOffice `connectState`, trimmed. Name kept.

    See RFC §2.2 / todo step_0 §4.5 for the fields dropped from the OnlyOffice
    `connectState`: the rich per-participant objects are replaced by
    `ParticipantEntry { indexUser, deviceId, view }` (the client resolves names
    via libparsec; the server isn't trusted for them).
    """

    type: Literal["connectState"] = "connectState"
    # Monotonic ms timestamp of this participant-set update (OnlyOffice name).
    participantsTimestamp: int
    participants: list[EditicsProtocolParticipantEntry]
    # True while the document auth lock is held (todo step_1 §6.2): tells the
    # established editor it must send `unLockDocument{unlock:true}` to release
    # the lock and let newcomers proceed.
    waitAuth: bool = False


class EditicsProtocolServerEventAuthChanges(BaseModel):
    """OnlyOffice `authChanges` (s->c). Name kept.

    Delivered to a joining client (after the auth lock is released, if any)
    as the backlog of changes that occurred since the session was created
    (RFC §1.2 step 3.4). In step 1 the backlog is delivered in a single chunk.
    """

    type: Literal["authChanges"] = "authChanges"
    # Each entry: (change index, encrypted change blob). The index is 1-based
    # and monotonic; the blob is base64 over JSON (§2.4), opaque to the server.
    changes: list[tuple[int, bytes]] = Field(default_factory=list)


class EditicsProtocolServerEventWaitAuth(BaseModel):
    """OnlyOffice `waitAuth` (s->c). Name kept. Per RFC §2.2 editics changes,
    `lockDocument` is replaced by `authLockedBy` (the indexUser holding the
    auth lock). Sent to a joining non-view participant when the auth lock is
    held (§6.2), as the RPC reply (the newcomer is "parked")."""

    type: Literal["waitAuth"] = "waitAuth"
    authLockedBy: EditicsProtocolIndexUser


class MessageRecord(BaseModel):
    """One record in a server `message` event's `messages` array (todo §4.4)."""

    model_config = {"arbitrary_types_allowed": True}
    time: int  # server timestamp (ms)
    authorIndexUser: EditicsProtocolIndexUser
    # base64 over JSON (§2.4). Opaque; server never inspects.
    encryptedMessage: bytes


class EditicsProtocolServerEventMessage(BaseModel):
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
    authorIndexUser: EditicsProtocolIndexUser
    # base64 over JSON (§2.4). Opaque; server never inspects.
    encryptedCursor: bytes


class EditicsProtocolServerEventCursor(BaseModel):
    """OnlyOffice `cursor` (s->c). Name kept. Per RFC §2.2 editics changes:
    `cursor` -> `encryptedCursor` (bytes); `user`/`useridoriginal` ->
    `authorIndexUser`. OnlyOffice wraps the payload in `messages: [...]`; kept
    for symmetry. Broadcast to other participants (§6.4)."""

    type: Literal["cursor"] = "cursor"
    messages: list[CursorRecord]


class EditicsProtocolServerEventGetLock(BaseModel):
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
    user: EditicsProtocolIndexUser
    time: int
    # Always null here (OnlyOffice shape; present for consistency with
    # `saveChanges`'s `locks` field).
    changes: None = None


class EditicsProtocolServerEventReleaseLock(BaseModel):
    """OnlyOffice `releaseLock` (s->c). Name kept. Broadcast to others when a
    user releases region locks outside of a `saveChanges` (i.e. from
    `unLockDocument{releaseLocks:true}` or disconnect cleanup). Per RFC §2.2 the
    `user` field is the holder's indexUser (bad name documented)."""

    type: Literal["releaseLock"] = "releaseLock"
    locks: list[ReleaseLockRecord]


class EditicsProtocolServerEventSaveLock(BaseModel):
    """OnlyOffice `saveLock` (s->c). Name kept. Reply to `isSaveLock` (c->s).
    `saveLock: true` means denied (someone holds it / client is desynced);
    `false` means granted."""

    type: Literal["saveLock"] = "saveLock"
    saveLock: bool


class SaveChangeRecord(BaseModel):
    """One record in a server `saveChanges` broadcast's `changes` array."""

    model_config = {"arbitrary_types_allowed": True}
    time: int
    authorIndexUser: EditicsProtocolIndexUser
    # Opaque encrypted blob (base64 over JSON, §2.4); server never inspects.
    change: bytes


class EditicsProtocolServerEventSaveChanges(BaseModel):
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


class EditicsProtocolServerEventSavePartChanges(BaseModel):
    """OnlyOffice `savePartChanges` (s->c, reply to the saver for intermediate
    chunks). Name kept. `changesIndex` is -1 except for the first chunk of a
    non-truncating save (§2.2). `syncChangesIndex` always advances."""

    type: Literal["savePartChanges"] = "savePartChanges"
    changesIndex: int  # -1 except first non-truncating chunk
    syncChangesIndex: int  # always-advancing total


class EditicsProtocolServerEventUnSaveLock(BaseModel):
    """OnlyOffice `unSaveLock` (s->c). Name kept. Two uses (RFC §2.2):
    1. Cancellation: reply to `unSaveLock` (c->s) -> index/time/sync = -1.
    2. Success: reply to a final `saveChanges` chunk -> real values."""

    type: Literal["unSaveLock"] = "unSaveLock"
    index: int  # save point, or -1 on cancel
    time: int  # last change time, or -1 on cancel
    syncChangesIndex: int  # new total, or -1 on cancel


class EditicsProtocolServerEventDrop(BaseModel):
    """OnlyOffice `drop` (s->c). Name kept. Kept as-is (RFC §2.2). Sent to a
    participant the server is force-removing (e.g. duplicate participant
    detection, or future permission revocation)."""

    type: Literal["drop"] = "drop"
    code: int = 4007  # OnlyOffice DROP_CODE constant
    description: str = ""


class EditicsProtocolServerEventWarning(BaseModel):
    """OnlyOffice `warning` (s->c). Name kept. Kept as-is (RFC §2.2). Shape only
    in step 1 (not actively triggered)."""

    type: Literal["warning"] = "warning"
    code: int
    message: str


EditicsProtocolServerEvent = Annotated[
    EditicsProtocolServerEventAuth
    | EditicsProtocolServerEventAuthRejected
    | EditicsProtocolServerEventConnectState
    | EditicsProtocolServerEventAuthChanges
    | EditicsProtocolServerEventWaitAuth
    | EditicsProtocolServerEventMessage
    | EditicsProtocolServerEventCursor
    | EditicsProtocolServerEventGetLock
    | EditicsProtocolServerEventReleaseLock
    | EditicsProtocolServerEventSaveLock
    | EditicsProtocolServerEventSaveChanges
    | EditicsProtocolServerEventSavePartChanges
    | EditicsProtocolServerEventUnSaveLock
    | EditicsProtocolServerEventDrop
    | EditicsProtocolServerEventWarning,
    Field(discriminator="type"),
]
EditicsProtocolServerEventAdapter = TypeAdapter(EditicsProtocolServerEvent)
