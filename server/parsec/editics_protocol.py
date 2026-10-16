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


class EditicsClientEventAuth(BaseModel):
    type: Literal["auth"] = "auth"
    # The vlob version the client has loaded locally (RFC §1.2). For a fresh
    # session this becomes the session's initial version.
    # (OnlyOffice has no such field; editics addition.)
    vlobVersion: int
    # The participant ID of the logical session this auth restores, or `null`
    # for a fresh join.
    # This is the same value the transport already carries (the participant ID
    # in the listen/send routes URL), but the transport *identifies* the connection
    # while this field is the client's *statement of intent*: the server only
    # restores a participant's session when the client explicitly says so, and
    # echoing the ID lets it validate the client's bookkeeping.
    reconnect: EditicsParticipantID | None
    mode: Literal["view", "edit"]


class EditicsClientEventAuthChangesAck(BaseModel):
    type: Literal["authChangesAck"] = "authChangesAck"


class EditicsClientEventGetMessages(BaseModel):
    type: Literal["getMessages"] = "getMessages"


class EditicsClientEventMessage(BaseModel):
    type: Literal["message"] = "message"
    # Format: (<Key index>, <base64(ciphertext)>)
    encryptedMessage: tuple[int, Base64BytesField]


class EditicsClientEventCursor(BaseModel):
    type: Literal["cursor"] = "cursor"
    # Format: (<Key index>, <base64(ciphertext)>)
    encryptedCursor: tuple[int, Base64BytesField]


class EditicsClientEventGetLock(BaseModel):
    type: Literal["getLock"] = "getLock"
    # Opaque block descriptors (shape depends on editor type).
    block: list[Any]


class EditicsClientEventIsSaveLock(BaseModel):
    type: Literal["isSaveLock"] = "isSaveLock"
    syncChangesIndex: int


class EditicsClientEventSaveChanges(BaseModel):
    type: Literal["saveChanges"] = "saveChanges"
    # Format: [(<Key index>, <base64(ciphertext)>), ...]
    encryptedChanges: list[tuple[int, Base64BytesField]]
    startSaveChanges: bool
    endSaveChanges: bool
    # Undo support (Only honored when `startSaveChanges=true`):
    # - `None` or `-1` = no truncate
    # - <int> = absolute index, the server deletes all stored changes with index >= deleteIndex.
    deleteIndex: int | None = None
    # In OnlyOffice protocol, the `excelAdditionalInfo` (despite its name) is
    # carried by all the editor types.
    # - The spreadsheet editor only sends it when there is something to carry:
    #   the `indexCols`/`indexRows` lock-shift info when columns/rows were
    #   inserted (the receivers use it to recalculate their locked ranges),
    #   and the `CursorInfo` cursor keys only when it has a document position.
    #   So a save can carry lock-shift info without any cursor part, and vice versa.
    # - The word/presentation editors always carry the cursor keys (and never the
    #   lock-shift info), with `CursorInfo` null when the editor is not in
    #   fast co-editing mode or has no document position.
    #
    # So here we split this into two fields:
    # - `excelAdditionalInfo` that contains the actual spreadsheet-specific infos (
    #   Shape: `{ indexCols: ..., indexRows: ... }`).
    # - `encryptedCursorAdditionalInfo` that contains the cursor info use by all editors.
    excelAdditionalInfo: dict[str, Any] | None = None
    # Format: (<Key index>, <base64(ciphertext)>)
    encryptedCursorAdditionalInfo: tuple[int, Base64BytesField] | None = None
    # Release this user's region locks after the save (OnlyOffice name kept).
    releaseLocks: bool = False


class EditicsClientEventUnSaveLock(BaseModel):
    type: Literal["unSaveLock"] = "unSaveLock"


class EditicsClientEventUnLockDocument(BaseModel):
    type: Literal["unLockDocument"] = "unLockDocument"
    isSave: bool = False
    unlock: bool = False
    deleteIndex: int | None = None
    releaseLocks: bool = False


class EditicsClientEventClose(BaseModel):
    type: Literal["close"] = "close"


class EditicsClientEventSaveDone(BaseModel):
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
    model_config = {"arbitrary_types_allowed": True}

    time: int
    holder: tuple[EditicsParticipantID, DeviceID]
    # Opaque block descriptor (shape depends on editor type; re-broadcast as-is).
    block: Any


class MessageRecord(BaseModel):
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
    type: Literal["connectState"] = "connectState"
    # Monotonic ms timestamp of this participant-set update (OnlyOffice name).
    participantsTimestamp: int
    participants: list[EditicsParticipantEntry]
    # True only in the broadcast announcing a non-viewer newcomer parked behind
    # the auth lock (i.e. that received a `waitAuth` event from the server).
    # The OnlyOffice client only uses this flag for error logging: the actual
    # auth lock release with `unLockDocument{unlock:true}` is triggered based on
    # the `participants` field (i.e. switches from single to multiple editors).
    waitAuth: bool


class EditicsServerEventAuthChanges(BaseModel):
    type: Literal["authChanges"] = "authChanges"
    changes: list[AuthChangeRecord] = Field(default_factory=list)


class AuthChangeRecord(BaseModel):
    model_config = {"arbitrary_types_allowed": True}

    # 1-based and monotonic.
    index: int
    author: tuple[EditicsParticipantID, DeviceID]
    # Server timestamp (ms) of the change.
    time: int
    # Format: (<Key index>, <base64(ciphertext)>)
    encryptedChange: tuple[int, Base64BytesField]


class EditicsServerEventWaitAuth(BaseModel):
    type: Literal["waitAuth"] = "waitAuth"
    authLockedBy: EditicsParticipantEntry


class EditicsServerEventMessage(BaseModel):
    type: Literal["message"] = "message"
    messages: list[MessageRecord]


class CursorRecord(BaseModel):
    model_config = {"arbitrary_types_allowed": True}
    time: int
    author: tuple[EditicsParticipantID, DeviceID]
    # Format: (<Key index>, <base64(ciphertext)>)
    encryptedCursor: tuple[int, Base64BytesField]


class EditicsServerEventCursor(BaseModel):
    type: Literal["cursor"] = "cursor"
    messages: list[CursorRecord]


class EditicsServerEventGetLock(BaseModel):
    type: Literal["getLock"] = "getLock"
    locks: dict[str, LockRecord]


class ReleaseLockRecord(BaseModel):
    model_config = {"arbitrary_types_allowed": True}
    block: Any  # opaque block descriptor (re-broadcast as-is)
    holder: tuple[EditicsParticipantID, DeviceID]
    time: int
    # Always null here (OnlyOffice shape; present for consistency with
    # `saveChanges`'s `locks` field).
    changes: None = None


class EditicsServerEventReleaseLock(BaseModel):
    type: Literal["releaseLock"] = "releaseLock"
    locks: list[ReleaseLockRecord]


class EditicsServerEventSaveLock(BaseModel):
    type: Literal["saveLock"] = "saveLock"
    # This field is named `saveLock` in OnlyOffice protocol, but this is confusing:
    # - `false` means the save lock is granted (i.e. we may now send our `saveChanges`)
    # - `true` means it is held by somebody else.
    lockedBySomebodyElse: bool


class SaveChangeRecord(BaseModel):
    model_config = {"arbitrary_types_allowed": True}
    time: int
    author: tuple[EditicsParticipantID, DeviceID]
    # Opaque encrypted blob (base64 over JSON, §2.4); server never inspects.
    # Format: (<Key index>, <base64(ciphertext)>)
    encryptedChange: tuple[int, Base64BytesField]


class EditicsServerEventSaveChanges(BaseModel):
    type: Literal["saveChanges"] = "saveChanges"
    # One record per change fragment across all chunks of this save.
    changes: list[SaveChangeRecord]
    changesIndex: int  # new save point after this save (§2.2)
    syncChangesIndex: int  # always-advancing total (§2.2)
    endSaveChanges: bool  # mirrors the originator's flag
    # Locks released by the originator in this save (only when its
    # `releaseLocks` was true). Same shape as `releaseLock` records.
    locks: list[ReleaseLockRecord] = Field(default_factory=list)
    # In OnlyOffice protocol, the `excelAdditionalInfo` (despite its name) is
    # carried by all the editor types.
    # - The spreadsheet editor only sends it when there is something to carry:
    #   the `indexCols`/`indexRows` lock-shift info when columns/rows were
    #   inserted (the receivers use it to recalculate their locked ranges),
    #   and the `CursorInfo` cursor keys only when it has a document position.
    #   So a save can carry lock-shift info without any cursor part, and vice versa.
    # - The word/presentation editors always carry the cursor keys (and never the
    #   lock-shift info), with `CursorInfo` null when the editor is not in
    #   fast co-editing mode or has no document position.
    #
    # So here we split this into two fields:
    # - `excelAdditionalInfo` that contains the actual spreadsheet-specific infos (
    #   Shape: `{ indexCols: ..., indexRows: ... }`).
    # - `encryptedCursorAdditionalInfo` that contains the cursor info use by all editors.
    excelAdditionalInfo: dict[str, Any] | None = None
    # Format: (<Key index>, <base64(ciphertext)>)
    encryptedCursorAdditionalInfo: tuple[int, Base64BytesField] | None = None


class EditicsServerEventSavePartChanges(BaseModel):
    type: Literal["savePartChanges"] = "savePartChanges"
    changesIndex: int  # -1 except first non-truncating chunk
    syncChangesIndex: int  # always-advancing total


class EditicsServerEventUnSaveLock(BaseModel):
    type: Literal["unSaveLock"] = "unSaveLock"
    index: int  # save point, or -1 on cancel
    time: int  # last change time, or -1 on cancel
    syncChangesIndex: int  # new total, or -1 on cancel


class EditicsServerEventDrop(BaseModel):
    type: Literal["drop"] = "drop"
    code: int = 4007  # OnlyOffice DROP_CODE constant
    description: str = ""


class EditicsServerEventWarning(BaseModel):
    type: Literal["warning"] = "warning"
    code: int
    message: str


class EditicsServerEventError(BaseModel):
    type: Literal["error"] = "error"
    code: int
    description: str


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
    | EditicsServerEventWarning
    | EditicsServerEventError,
    Field(discriminator="type"),
]
EditicsServerEventAdapter = TypeAdapter(EditicsServerEvent)
