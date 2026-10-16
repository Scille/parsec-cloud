# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

"""
The editics protocol schemas is defined here, basically:
- Editics protocol is based on the OnlyOffice protocol (typically removing unneeded
  field and adding encryption to sensitive ones).
- The main exposed classes are `EditicsClientEvent` and `EditicsServerEvent`

See [RFC 1030](../../docs/rfcs/1030-collaborative-editics.md).
"""

from __future__ import annotations

from typing import Annotated, Any, Literal
from uuid import UUID

from pydantic import AfterValidator, BaseModel, Field, TypeAdapter

from parsec._parsec import DeviceID
from parsec.types import Base64BytesField

# TODO: remove option fields (we want to explicitly set a field to `None` instead of omitting it)

# TODO: rename to `EditicsParticipantIndex` ?
type EditicsIndexUser = int
"""
`indexUser`: 1-based, monotonic per client session, assigned by order of join
and never re-used.

OnlyOffice client uses this index to generate IDs that are guaranteed to never
clash with other clients: everything (e.g. paragraph, table) created collaboratively
gets an ID like "3_42" (where 3 is the editor's indexUser).
"""


# Don't type this as UUID to avoid serialization mixing between `UUID.__str__` and `UUID.hex`
type EditicsParticipantID = Annotated[str, AfterValidator(lambda v: UUID(v).hex)]
"""
Random UUID controlled by the client and used to identify the client session.
By design, this survives transport disconnection and is used mostly to identify
who has authored a modification or holds a lock.
"""


class EditicsParticipantEntry(BaseModel):
    model_config = {"arbitrary_types_allowed": True}

    id: EditicsParticipantID
    indexUser: EditicsIndexUser
    deviceId: DeviceID
    view: bool


# --- Client -> server events -------------------------------------------------


class EditicsReconnect(BaseModel):
    participantId: EditicsParticipantID
    indexUser: EditicsIndexUser
    # TODO: document if it is in seconds or ms etc.
    participantTimeConnect: int
    timeIdle: int


class EditicsClientEventAuth(BaseModel):
    type: Literal["auth"] = "auth"
    # The vlob version the client has loaded locally (RFC §1.2). For a fresh
    # session this becomes the session's initial version.
    # (OnlyOffice has no such field; editics addition.)
    vlobVersion: int
    # Reconnect info (forward-compat; unused in step 0 but kept).
    reconnect: EditicsReconnect | None
    mode: Literal["view", "edit"]


class EditicsClientEventAuthChangesAck(BaseModel):
    """OnlyOffice `authChangesAck` (c->s). Name kept.

    Acknowledges one `authChanges` chunk (RFC §2.2). In step 1 the backlog is
    delivered in a single chunk (todo step_1 §6.3), so at most one ack is
    expected per join.
    """

    type: Literal["authChangesAck"] = "authChangesAck"


class EditicsClientEventGetMessages(BaseModel):
    type: Literal["getMessages"] = "getMessages"


class EditicsClientEventMessage(BaseModel):
    type: Literal["message"] = "message"
    # Format: (<Key index>, <base64(ciphertext)>)
    encryptedMessage: tuple[int, Base64BytesField]


class EditicsClientEventCursor(BaseModel):
    """OnlyOffice `cursor` (c->s). Name kept. The cursor is encrypted (§2.4);
    renamed to `encryptedCursor` per RFC §2.2 editics changes."""

    type: Literal["cursor"] = "cursor"
    # Format: (<Key index>, <base64(ciphertext)>)
    encryptedCursor: tuple[int, Base64BytesField]


class EditicsClientEventGetLock(BaseModel):
    """OnlyOffice `getLock` (c->s). Name kept. Kept as-is per RFC §2.2 (the
    `block` array is opaque to the server; keyed by JSON serialization)."""

    type: Literal["getLock"] = "getLock"
    # Opaque block descriptors (shape depends on editor type).
    block: list[Any]


class EditicsClientEventIsSaveLock(BaseModel):
    """OnlyOffice `isSaveLock` (c->s). Name kept. The client's current
    always-advancing sync point (§2.2). Used by the server to detect a
    desynchronized client (keep denying the lock until it catches up)."""

    type: Literal["isSaveLock"] = "isSaveLock"
    syncChangesIndex: int


class EditicsClientEventSaveChanges(BaseModel):
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
    # Format: [(<Key index>, <base64(ciphertext)>), ...]
    encryptedChanges: list[tuple[int, Base64BytesField]]
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
    # Format: (<Key index>, <base64(ciphertext)>)
    encryptedCursor: tuple[int, Base64BytesField] | None = None
    # Release this user's region locks after the save (OnlyOffice name kept).
    releaseLocks: bool = False


class EditicsClientEventUnSaveLock(BaseModel):
    """OnlyOffice `unSaveLock` (c->s). Name kept. Cancel an in-progress save and
    release the save lock without saving (RFC §2.2). Kept as-is."""

    type: Literal["unSaveLock"] = "unSaveLock"


class EditicsClientEventUnLockDocument(BaseModel):
    """OnlyOffice `unLockDocument` (c->s). Name kept. Fire-and-forget cleanup
    combining up to three independent actions (RFC §2.2). Kept as-is."""

    type: Literal["unLockDocument"] = "unLockDocument"
    isSave: bool = False
    unlock: bool = False
    deleteIndex: int | None = None
    releaseLocks: bool = False


class EditicsClientEventClose(BaseModel):
    """OnlyOffice `close` (c->s). Name kept. Voluntary leave + close (unlike
    `unLockDocument` which keeps the connection alive). Kept as-is."""

    type: Literal["close"] = "close"


class EditicsClientEventSaveDone(BaseModel):
    """Editics addition (no OnlyOffice equivalent). Sent by the client after it
    has uploaded a new vlob version, to bump the session's
    `latest_allowed_version` (RFC §1.2 step 4.3). Must be sent only by the
    participant that just released the save lock (todo step_1 §6.9)."""

    type: Literal["saveDone"] = "saveDone"
    savedUpToIndex: int  # all changes up to this index are in the new version
    newVersion: int  # the new vlob version just uploaded


EditicsClientEvent = Annotated[
    EditicsClientEventAuth
    | EditicsClientEventAuthChangesAck
    | EditicsClientEventMessage
    | EditicsClientEventGetMessages
    | EditicsClientEventCursor
    | EditicsClientEventGetLock
    | EditicsClientEventIsSaveLock
    | EditicsClientEventSaveChanges
    | EditicsClientEventUnSaveLock
    | EditicsClientEventUnLockDocument
    | EditicsClientEventClose
    | EditicsClientEventSaveDone,
    Field(discriminator="type"),
]
EditicsClientEventAdapter = TypeAdapter(EditicsClientEvent)


# --- Server -> client events -------------------------------------------------


class LockRecord(BaseModel):
    """One record in a `getLock` event's `locks` object / an `auth` event's
    `locks` array (todo §4.4).

    The holder's identity is carried in the record (the OnlyOffice protocol
    instead uses a `user` composite id, resolved through the participants list):
    a lock can be broadcast after its holder left the session (e.g. released
    on disconnect), so its identity cannot be resolved through the current
    participants list."""

    model_config = {"arbitrary_types_allowed": True}

    time: int
    holder: tuple[EditicsParticipantID, DeviceID]
    # Opaque block descriptor (shape depends on editor type; re-broadcast as-is).
    block: Any


class MessageRecord(BaseModel):
    """One record in a server `message` event's / the `auth` reply's `messages`
    array (todo §4.4)."""

    model_config = {"arbitrary_types_allowed": True}
    time: int  # server timestamp (ms)
    author: tuple[EditicsParticipantID, DeviceID]
    # Format: (<Key index>, <base64(ciphertext)>)
    encryptedMessage: tuple[int, Base64BytesField]


class EditicsServerEventAuth(BaseModel):
    type: Literal["auth"] = "auth"
    participants: list[EditicsParticipantEntry]  # current participant map
    participantId: EditicsParticipantID
    indexUser: EditicsIndexUser  # this connection's assigned index
    # Reconnect info (forward-compat; unused in step 0 but kept).
    participantTimeConnect: int  # server timestamp (ms) at connect
    # When the file edition session has been started
    openedAt: int  # ms
    # Region locks currently held, so the newcomer knows which regions are
    # taken (OnlyOffice carries the lock table in its `auth` event).
    locks: list[LockRecord] = Field(default_factory=list)
    # Chat history, so the newcomer gets the past conversation (the OnlyOffice
    # server carries it in its `auth` event; the client only renders the field
    # when non-empty, like for the `message` event).
    messages: list[MessageRecord] = Field(default_factory=list)


class EditicsServerEventConnectState(BaseModel):
    """OnlyOffice `connectState`, trimmed. Name kept.

    See RFC §2.2 / todo step_0 §4.5 for the fields dropped from the OnlyOffice
    `connectState`: the rich per-participant objects are replaced by
    `ParticipantEntry { indexUser, deviceId, view }` (the client resolves names
    via libparsec; the server isn't trusted for them).
    """

    type: Literal["connectState"] = "connectState"
    # Monotonic ms timestamp of this participant-set update (OnlyOffice name).
    participantsTimestamp: int
    participants: list[EditicsParticipantEntry]
    # True while the document auth lock is held (todo step_1 §6.2): tells the
    # established editor it must send `unLockDocument{unlock:true}` to release
    # the lock and let newcomers proceed.
    waitAuth: bool


class EditicsServerEventAuthChanges(BaseModel):
    """OnlyOffice `authChanges` (s->c). Name kept.

    Delivered to a joining client (after the auth lock is released, if any)
    as the backlog of changes that occurred since the session was created
    (RFC §1.2 step 3.4). In step 1 the backlog is delivered in a single chunk.
    """

    type: Literal["authChanges"] = "authChanges"
    changes: list[AuthChangeRecord] = Field(default_factory=list)


class AuthChangeRecord(BaseModel):
    """One record in an `authChanges` event's `changes` array (todo §4.4)."""

    model_config = {"arbitrary_types_allowed": True}

    # 1-based and monotonic.
    index: int
    author: tuple[EditicsParticipantID, DeviceID]
    # Server timestamp (ms) of the change.
    time: int
    # Format: (<Key index>, <base64(ciphertext)>)
    encryptedChange: tuple[int, Base64BytesField]


class EditicsServerEventWaitAuth(BaseModel):
    """OnlyOffice `waitAuth` (s->c). Name kept. Per RFC §2.2 editics changes,
    `lockDocument` is replaced by `authLockedBy` (the indexUser holding the
    auth lock). Sent to a joining non-view participant when the auth lock is
    held (§6.2): the newcomer is "parked" until the holder releases the lock.
    Pushed on the newcomer's SSE channel, after the `connectState` broadcast
    that carried the holder's participant entry (which the client needs to
    translate this event into the OnlyOffice `lockDocument` field)."""

    type: Literal["waitAuth"] = "waitAuth"
    authLockedBy: EditicsParticipantEntry


class EditicsServerEventMessage(BaseModel):
    type: Literal["message"] = "message"
    messages: list[MessageRecord]


class CursorRecord(BaseModel):
    """One record in a server `cursor` event's `messages` array (todo §4.4)."""

    model_config = {"arbitrary_types_allowed": True}
    time: int
    author: tuple[EditicsParticipantID, DeviceID]
    # Format: (<Key index>, <base64(ciphertext)>)
    encryptedCursor: tuple[int, Base64BytesField]


class EditicsServerEventCursor(BaseModel):
    """OnlyOffice `cursor` (s->c). Name kept. Per RFC §2.2 editics changes:
    `cursor` -> `encryptedCursor` (bytes); `user`/`useridoriginal` ->
    the author identity carried in the record. OnlyOffice wraps the payload in
    `messages: [...]`; kept for symmetry. Broadcast to other participants (§6.4)."""

    type: Literal["cursor"] = "cursor"
    messages: list[CursorRecord]


class EditicsServerEventGetLock(BaseModel):
    """OnlyOffice `getLock` (s->c). Name kept. Kept as-is per RFC §2.2. The full
    lock table as it stands after the server attempted to acquire the requested
    blocks for the requester. Broadcast to *everyone* (including the sender).
    `locks` is an object keyed by the block key; each record carries the
    holder's identity (see `LockRecord`)."""

    type: Literal["getLock"] = "getLock"
    locks: dict[str, LockRecord]


class ReleaseLockRecord(BaseModel):
    """One record in a `releaseLock` event / the `locks` field of a
    `saveChanges` broadcast (todo §4.4)."""

    model_config = {"arbitrary_types_allowed": True}
    block: Any  # opaque block descriptor (re-broadcast as-is)
    holder: tuple[EditicsParticipantID, DeviceID]
    time: int
    # Always null here (OnlyOffice shape; present for consistency with
    # `saveChanges`'s `locks` field).
    changes: None = None


class EditicsServerEventReleaseLock(BaseModel):
    """OnlyOffice `releaseLock` (s->c). Name kept. Broadcast to others when a
    user releases region locks outside of a `saveChanges` (i.e. from
    `unLockDocument{releaseLocks:true}` or disconnect cleanup). Each record
    carries the holder's identity (see `ReleaseLockRecord`)."""

    type: Literal["releaseLock"] = "releaseLock"
    locks: list[ReleaseLockRecord]


class EditicsServerEventSaveLock(BaseModel):
    """OnlyOffice `saveLock` (s->c). Name kept. Reply to `isSaveLock` (c->s).
    `saveLock: true` means denied (someone holds it / client is desynced);
    `false` means granted."""

    type: Literal["saveLock"] = "saveLock"
    # This field is named `saveLock` in OnlyOffice protocol, but this is confusing:
    # - `false` means the save lock is granted (i.e. we may now send our `saveChanges`)
    # - `true` means it is held by somebody else.
    lockedBySomebodyElse: bool


class SaveChangeRecord(BaseModel):
    """One record in a server `saveChanges` broadcast's `changes` array."""

    model_config = {"arbitrary_types_allowed": True}
    time: int
    author: tuple[EditicsParticipantID, DeviceID]
    # Opaque encrypted blob (base64 over JSON, §2.4); server never inspects.
    # Format: (<Key index>, <base64(ciphertext)>)
    encryptedChange: tuple[int, Base64BytesField]


class EditicsServerEventSaveChanges(BaseModel):
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
    # Format: (<Key index>, <base64(ciphertext)>)
    encryptedCursor: tuple[int, Base64BytesField] | None = None


class EditicsServerEventSavePartChanges(BaseModel):
    """OnlyOffice `savePartChanges` (s->c, reply to the saver for intermediate
    chunks). Name kept. `changesIndex` is -1 except for the first chunk of a
    non-truncating save (§2.2). `syncChangesIndex` always advances."""

    type: Literal["savePartChanges"] = "savePartChanges"
    changesIndex: int  # -1 except first non-truncating chunk
    syncChangesIndex: int  # always-advancing total


class EditicsServerEventUnSaveLock(BaseModel):
    """OnlyOffice `unSaveLock` (s->c). Name kept. Two uses (RFC §2.2):
    1. Cancellation: reply to `unSaveLock` (c->s) -> index/time/sync = -1.
    2. Success: reply to a final `saveChanges` chunk -> real values."""

    type: Literal["unSaveLock"] = "unSaveLock"
    index: int  # save point, or -1 on cancel
    time: int  # last change time, or -1 on cancel
    syncChangesIndex: int  # new total, or -1 on cancel


class EditicsServerEventDrop(BaseModel):
    """OnlyOffice `drop` (s->c). Name kept. Kept as-is (RFC §2.2). Sent to a
    participant the server is force-removing (e.g. duplicate participant
    detection, or future permission revocation)."""

    type: Literal["drop"] = "drop"
    code: int = 4007  # OnlyOffice DROP_CODE constant
    description: str = ""


class EditicsServerEventWarning(BaseModel):
    """OnlyOffice `warning` (s->c). Name kept. Kept as-is (RFC §2.2). Shape only
    in step 1 (not actively triggered)."""

    type: Literal["warning"] = "warning"
    code: int
    message: str


EditicsServerEvent = Annotated[
    EditicsServerEventAuth
    | EditicsServerEventConnectState
    | EditicsServerEventAuthChanges
    | EditicsServerEventWaitAuth
    | EditicsServerEventMessage
    | EditicsServerEventCursor
    | EditicsServerEventGetLock
    | EditicsServerEventReleaseLock
    | EditicsServerEventSaveLock
    | EditicsServerEventSaveChanges
    | EditicsServerEventSavePartChanges
    | EditicsServerEventUnSaveLock
    | EditicsServerEventDrop
    | EditicsServerEventWarning,
    Field(discriminator="type"),
]
EditicsServerEventAdapter = TypeAdapter(EditicsServerEvent)
