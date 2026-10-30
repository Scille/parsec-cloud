# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

"""
The editics protocol schemas is defined here, basically:
- Editics protocol is based on the OnlyOffice protocol (typically removing unneeded
  field and adding encryption to sensitive ones).
- The main exposed classes are `EditicsClientEvent` and `EditicsServerEvent`.
- Commands with the `parsec` prefix are new ones specific for our needs (e.g.
  `parsecInitialContentUploaded`).

See [RFC 1030](../../docs/rfcs/1030-collaborative-editics.md).
"""

from __future__ import annotations

from typing import Annotated, Any, Literal
from uuid import UUID

from pydantic import (
    AfterValidator,
    BaseModel,
    Field,
    NonNegativeInt,
    PositiveInt,
    Strict,
    TypeAdapter,
)

from parsec._parsec import ApiVersion, DeviceID
from parsec.types import Base64BytesField

# Editics protocol API is independent from the RPC API, and hence has its own
# versioning.
#
# Changelog:
# - v1.0 (Parsec 3.12+): initial version
EDITICS_API_VERSION = ApiVersion(version=1, revision=0)

# TODO: remove option fields (we want to explicitly set a field to `None` instead of omitting it)

type EditicsParticipantIndex = Annotated[NonNegativeInt, Strict()]
"""
Equivalent to the `indexUser` in OnlyOffice protocol: 1-based, monotonic per
edition session, assigned by order of join and never re-used.

OnlyOffice client uses this index to generate IDs that are guaranteed to never
clash with other clients: everything (e.g. paragraph, table) created collaboratively
gets an ID like "3_42" (where 3 is the editor's `indexUser`).
"""


# Don't type this as UUID to avoid serialization mixing between `UUID.__str__` and `UUID.hex`
type EditicsParticipantSessionID = Annotated[str, AfterValidator(lambda v: UUID(v).hex)]
"""
Random UUID controlled by the client identifying its *participant session*
(it must be known before the SSE connection is created, and survives transport
disconnections so the connection can be restored).

This is a purely transport-level concept (the equivalent of the socket.io
`sessionId` used by the real OnlyOffice server), the identity of the participants
in the event payloads is carried by `EditicsParticipantIdentity` instead.
"""


type EditicsParticipantIdentity = tuple[DeviceID, EditicsParticipantIndex]
"""
OnlyOffice client builds its own identity as composite ID `<user id><indexUser>`
and compares it against the `user` fields of the locks/changes/cursors/messages
it receives.

So we have to follow a similar pattern here (instead of just using
`EditicsParticipantSessionID` for everything :/ ) by providing device ID + index
that the client combines to rebuild the composite ID.
"""


type EditicsTimestampMs = Annotated[NonNegativeInt, Strict()]
"""
In editics, timestamps use the number of milliseconds since epoch.
"""

type EditicsBase64Ciphertext = Base64BytesField
type EditicsKeyIndex = Annotated[NonNegativeInt, Strict()]
type EditicsEncryptedContent = tuple[EditicsKeyIndex, EditicsBase64Ciphertext]
"""
Format: (<Workspace key index>, <base64(ciphertext)>)
"""


class EditicsParticipantEntry(BaseModel):
    model_config = {"arbitrary_types_allowed": True}

    indexUser: EditicsParticipantIndex
    deviceId: DeviceID
    view: bool


# --- Spreadsheet structural changes -----------------------------------------


class EditicsExcelRecalcIndexElement(BaseModel):
    # TODO: `serialize_by_alias=True` no longer needed once upgranded to Pydantic v3
    model_config = {"serialize_by_alias": True}

    # 1=insert, 2=remove
    recalc_type: Literal[1, 2] = Field(alias="_recalcType")
    # 0-based index of the first inserted/deleted column (or row).
    position: Annotated[NonNegativeInt, Strict()] = Field(alias="_position")
    # Consecutive positions changed (always 1 in practice, since the OnlyOffice
    # editor expands bigger changes into N adjacent elements).
    count: Annotated[PositiveInt, Strict()] = Field(alias="_count")
    # True for the save-index markers a client re-emits to acknowledge the
    # structural changes it just applied (i.e. coming from other users' changes
    # it hasn't applied yet): not new geometry changes, the server skips them.
    is_save_index: bool = Field(alias="m_bIsSaveIndex")


class EditicsExcelSheetRecalcIndex(BaseModel):
    # TODO: `serialize_by_alias=True` no longer needed once upgranded to Pydantic v3
    model_config = {"serialize_by_alias": True}

    elements: list[EditicsExcelRecalcIndexElement] = Field(alias="_arrElements")


# Sheet id (OnlyOffice carries it as a JSON object key, e.g. `"6"`) -> its recalc records.
type EditicsExcelRecalcIndex = dict[str, EditicsExcelSheetRecalcIndex]


# --- Client -> server events -------------------------------------------------


class EditicsClientEventAuthReconnect(BaseModel):
    participantSessionId: EditicsParticipantSessionID
    # Last save (i.e. document modification or undo) timestamp the client knows
    # about. The server rejects reconnection if this is not up to date.
    # Set to `None` if the client has not received any save yet.
    lastOtherSaveTime: EditicsTimestampMs | None


class EditicsClientEventAuth(BaseModel):
    type: Literal["auth"] = "auth"
    # The vlob version the client has loaded locally (RFC §1.2). For a fresh
    # session this becomes the session's initial version.
    # (OnlyOffice has no such field; editics addition.)
    vlobVersion: Annotated[NonNegativeInt, Strict()]
    # The client already has a participant ID and want to restore its session, or
    # `null` for a fresh join.
    # This is the same value the transport already carries (the participant ID
    # in the listen/send routes URL), but the transport *identifies* the connection
    # while this field is the client's *statement of intent*: the server only
    # restores a participant's session when the client explicitly says so, and
    # echoing the ID lets it validate the client's bookkeeping.
    reconnect: EditicsClientEventAuthReconnect | None
    mode: Literal["view", "edit"]


class EditicsClientEventAuthChangesAck(BaseModel):
    type: Literal["authChangesAck"] = "authChangesAck"


class EditicsClientEventGetMessages(BaseModel):
    type: Literal["getMessages"] = "getMessages"


class EditicsClientEventMessage(BaseModel):
    type: Literal["message"] = "message"
    encryptedMessage: EditicsEncryptedContent


class EditicsClientEventCursor(BaseModel):
    type: Literal["cursor"] = "cursor"
    encryptedCursor: EditicsEncryptedContent


class EditicsClientEventGetLock(BaseModel):
    type: Literal["getLock"] = "getLock"
    # Opaque block descriptors (shape depends on editor type).
    block: list[Any]


class EditicsClientEventIsSaveLock(BaseModel):
    type: Literal["isSaveLock"] = "isSaveLock"
    syncChangesIndex: int


class EditicsClientEventSaveChanges(BaseModel):
    type: Literal["saveChanges"] = "saveChanges"
    encryptedChanges: list[EditicsEncryptedContent]
    startSaveChanges: bool
    endSaveChanges: bool
    # Undo support (Only honored when `startSaveChanges=true`):
    # - `None` = no truncate
    # - <int> = absolute index, the server deletes all stored changes with index >= deleteIndex.
    deleteIndex: Annotated[NonNegativeInt, Strict()] | None
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
    # So here we split this into three fields:
    # - `excelAdditionalInfoIndex[Cols|Rows]` that contains the spreadsheet-specific infos.
    # - `encryptedCursorAdditionalInfo` that contains the cursor info use by all editors.
    excelAdditionalInfoIndexCols: EditicsExcelRecalcIndex | None
    excelAdditionalInfoIndexRows: EditicsExcelRecalcIndex | None
    encryptedCursorAdditionalInfo: EditicsEncryptedContent | None
    # Release this user's region locks after the save (OnlyOffice name kept).
    releaseLocks: bool = False


class EditicsClientEventUnSaveLock(BaseModel):
    type: Literal["unSaveLock"] = "unSaveLock"


class EditicsClientEventUnLockDocument(BaseModel):
    type: Literal["unLockDocument"] = "unLockDocument"
    isSave: bool = False
    unlock: bool = False
    deleteIndex: Annotated[NonNegativeInt, Strict()] | None
    releaseLocks: bool = False


class EditicsClientEventClose(BaseModel):
    type: Literal["close"] = "close"


class EditicsClientEventSaveDone(BaseModel):
    type: Literal["saveDone"] = "saveDone"
    savedUpToIndex: Annotated[
        NonNegativeInt, Strict()
    ]  # all changes up to this index are in the new version
    newVersion: Annotated[NonNegativeInt, Strict()]  # the new vlob version just uploaded


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

    time: EditicsTimestampMs
    holder: EditicsParticipantIdentity
    # Opaque block descriptor (shape depends on editor type; re-broadcast as-is).
    block: Any


class MessageRecord(BaseModel):
    model_config = {"arbitrary_types_allowed": True}
    time: EditicsTimestampMs  # Server timestamp
    author: EditicsParticipantIdentity
    encryptedMessage: EditicsEncryptedContent


class EditicsServerEventAuth(BaseModel):
    type: Literal["auth"] = "auth"
    participants: list[EditicsParticipantEntry]
    participantSessionId: EditicsParticipantSessionID
    participantIndexUser: EditicsParticipantIndex
    participantSessionTimeConnect: EditicsTimestampMs
    # When the document edition session has been started
    openedAt: EditicsTimestampMs
    # Region locks currently held, so the newcomer knows which regions are
    # taken (OnlyOffice carries the lock table in its `auth` event).
    locks: list[LockRecord] = Field(default_factory=list)
    # Chat history, so the newcomer gets the past conversation (the OnlyOffice
    # server carries it in its `auth` event; the client only renders the field
    # when non-empty, like for the `message` event).
    messages: list[MessageRecord] = Field(default_factory=list)
    # Whether this connection already has an original content uploaded by an
    # editor: `false` means the participant must run x2t locally, (and upload
    # the result if it is an editor).
    initialContentUploaded: bool


class EditicsServerEventConnectState(BaseModel):
    type: Literal["connectState"] = "connectState"
    # Monotonic ms timestamp of this participant-set update (OnlyOffice name).
    participantsTimestamp: EditicsTimestampMs
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
    index: Annotated[PositiveInt, Strict()]
    author: EditicsParticipantIdentity
    # Server timestamp of the change.
    time: EditicsTimestampMs
    encryptedChange: EditicsEncryptedContent


class EditicsServerEventWaitAuth(BaseModel):
    type: Literal["waitAuth"] = "waitAuth"
    authLockedBy: EditicsParticipantEntry


class EditicsServerEventMessage(BaseModel):
    type: Literal["message"] = "message"
    messages: list[MessageRecord]


class CursorRecord(BaseModel):
    model_config = {"arbitrary_types_allowed": True}
    time: EditicsTimestampMs
    author: EditicsParticipantIdentity
    encryptedCursor: EditicsEncryptedContent


class EditicsServerEventCursor(BaseModel):
    type: Literal["cursor"] = "cursor"
    messages: list[CursorRecord]


class EditicsServerEventGetLock(BaseModel):
    type: Literal["getLock"] = "getLock"
    locks: dict[str, LockRecord]


class ReleaseLockRecord(BaseModel):
    model_config = {"arbitrary_types_allowed": True}
    block: Any  # opaque block descriptor (re-broadcast as-is)
    holder: EditicsParticipantIdentity
    time: EditicsTimestampMs
    # Always null here (OnlyOffice shape; present for consistency with
    # `saveChanges`'s `locks` field).
    changes: None


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
    time: EditicsTimestampMs
    author: EditicsParticipantIdentity
    # Opaque encrypted blob (base64 over JSON, §2.4); server never inspects.
    encryptedChange: EditicsEncryptedContent


class EditicsServerEventSaveChanges(BaseModel):
    type: Literal["saveChanges"] = "saveChanges"
    # One record per change fragment across all chunks of this save.
    changes: list[SaveChangeRecord]
    changesIndex: Annotated[NonNegativeInt, Strict()]  # new save point after this save (§2.2)
    syncChangesIndex: Annotated[NonNegativeInt, Strict()]  # always-advancing total (§2.2)
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
    # So here we split this into three fields:
    # - `excelAdditionalInfoIndex[Cols|Rows]` that contains the spreadsheet-specific infos.
    # - `encryptedCursorAdditionalInfo` that contains the cursor info use by all editors.
    excelAdditionalInfoIndexCols: EditicsExcelRecalcIndex | None
    excelAdditionalInfoIndexRows: EditicsExcelRecalcIndex | None
    # Format: (<Key index>, <base64(ciphertext)>)
    encryptedCursorAdditionalInfo: EditicsEncryptedContent | None


class EditicsServerEventSavePartChanges(BaseModel):
    type: Literal["savePartChanges"] = "savePartChanges"
    changesIndex: int  # -1 except first non-truncating chunk
    syncChangesIndex: int  # always-advancing total


class EditicsServerEventUnSaveLock(BaseModel):
    type: Literal["unSaveLock"] = "unSaveLock"
    index: Annotated[NonNegativeInt, Strict()] | None  # save point, or None on cancel
    time: EditicsTimestampMs | None  # last change time, or None on cancel
    syncChangesIndex: Annotated[NonNegativeInt, Strict()] | None  # new total, or None on cancel


class EditicsServerEventDisconnectReason(BaseModel):
    """
    The server kicks the participant out and closes its connection.
    """

    type: Literal["disconnectReason"] = "disconnectReason"
    # OnlyOffice ACCESS_DENIED_CODE constant.
    # 4001 shutdown, 4002 idle, 4003 absolute, 4004 access denied, 4006 jwt error.

    # OnlyOffice protocol codes:
    # - 4004: ACCESS_DENIED_CODE, reason='access deny'
    # - 4001: SHUTDOWN_CODE, reason='server shutdown'
    # - 4002: SESSION_IDLE_CODE, reason='idle session expires'
    # - 4003: SESSION_ABSOLUTE_CODE, reason='absolute session expires'
    # - 4005: JWT_EXPIRED_CODE, reason='token: <reason>'
    # - 4006: JWT_ERROR_CODE, reason='token: <reason>'
    #
    # We currently only use 4004 to reject viewer try to send an editor event.
    code: Literal[4004]
    description: str

    @classmethod
    def access_denied(cls, description: str | None = None) -> EditicsServerEventDisconnectReason:
        return cls(
            code=4004,
            description=description or "access deny",
        )


class EditicsServerEventDrop(BaseModel):
    """
    Inform the client its editing session has been terminated by an external action
    (typically the user's realm role has been revoked).

    Note `drop` and `disconnectReason` look similar but are not interchangeable:
    - `drop` is a command ("you're out, close your side"). The server rely on the client
      to acknowledge this and close the connection itself by sending a `close` event.
    - `disconnectReason` is a notification ("the transport is about to die server-side, here's why").
    """

    type: Literal["drop"] = "drop"
    description: str


class EditicsServerEventWarning(BaseModel):
    """
    Send a warning message to the client to be displayed to the end user.
    """

    type: Literal["warning"] = "warning"
    # e.g. `-200`: FORCED_VIEW_MODE, `-201`: FILE_NOT_ASSEMBLED
    code: int
    message: str


class EditicsServerEventError(BaseModel):
    type: Literal["error"] = "error"
    # e.g. code=4010, description="Restore error. Document modified."
    code: int
    description: str


class EditicsServerEventParsecInitialContentUploaded(BaseModel):
    """
    The session's original content just became available: participants
    that opened the document from their own local x2t conversion must
    reload the document with the session's content.

    Only ever broadcast to the connected viewers (an editor is either the
    uploader, or will get `alreadyUploaded: true` from its own upload
    attempt).

    This editics event has no OnlyOffice counterpart: upon receive, the host-page
    is expected to reload a fresh OnlyOffice editor page.
    """

    type: Literal["parsecInitialContentUploaded"] = "parsecInitialContentUploaded"


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
    | EditicsServerEventDisconnectReason
    | EditicsServerEventDrop
    | EditicsServerEventWarning
    | EditicsServerEventError
    | EditicsServerEventParsecInitialContentUploaded,
    Field(discriminator="type"),
]
EditicsServerEventAdapter = TypeAdapter(EditicsServerEvent)
