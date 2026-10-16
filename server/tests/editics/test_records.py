# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

import itertools
import json
import pprint
import re
import textwrap
from collections import defaultdict
from contextlib import AsyncExitStack
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Literal
from uuid import NAMESPACE_URL, uuid5

import anyio

from parsec._parsec import RealmRole, VlobID
from parsec.editics_protocol import EditicsParticipantID
from tests.common import (
    AuthenticatedRpcClient,
    Backend,
    CoolorgRpcClients,
    EditicsJSClient,
    EditicsJSRuntime,
    wksp1_alice_gives_role,
)

RECORDS_DIR = Path(__file__).resolve().parent / "records"

DEVICE_ID_TO_PARTICIPANT = {
    "de10a11cec0010000000000000000000": "Alice",  # alice@dev1
    "de10808c001000000000000000000000": "Bob",  # bob@dev1
    "de103a11031c00100000000000000000": "Mallory",  # mallory@dev1
}
PARTICIPANT_NAMES = set(DEVICE_ID_TO_PARTICIPANT.values())
DOCUMENT_ID = VlobID.from_hex("00000000000000000000000000000d0c")

# In OnlyOffice protocol there is three concepts:
# - `connectionId`: transport-level connection (i.e. websocket connection ID)
# - `sessionId`: logical session surviving websocket reconnects
#   (initially = first socket id; reused on restore).
# - `participantId`: server's primary per-participant key, its lifespan
#   is equal to the logical session. It is a composite ID made of `<idOriginal><indexUser>`
#   (e.g. `de10a11cec00100000000000000000001` for Alice with `indexUser` 1).
#
# In editics protocol however:
# - Session and participant are merged together (hence `sessionId` == `participantId`).
# - Participant ID is chosen client-side (needed since it must be known before creating
#   the SSE connection).


# Since the participant ID is chosen client-side, the harness must decide which ID to
# use for each editor stint (i.e. each transport-level connection lifetime delimited
# by the record's `ws-open`/`ws-close` rows): we derive them from the record's own
# OnlyOffice session IDs, so that a restore (whose `auth` carries the previous
# `sessionId`) naturally reuses the restored participant's ID, while a rejoin with a
# fresh editor (whose `auth` carries a `null` `sessionId`) gets its own new ID.
def _participant_id_from_session_id(session_id: str) -> EditicsParticipantID:
    return uuid5(NAMESPACE_URL, session_id).hex


# In OnlyOffice protocol, participant IDs are composites `<deviceId><indexUser>`
# (e.g. `de10a11cec00100000000000000000001` for Alice with `indexUser` 1): the
# record parser patches them back to the editics participant ID of the editor
# stint that was assigned this `indexUser` (learnt from the record's `auth` replies).
_COMPOSITE_PARTICIPANT_ID_RE = re.compile(
    "^(" + "|".join(DEVICE_ID_TO_PARTICIPANT.keys()) + r")(\d+)$"
)

# Matches a record event heading, e.g.:
#   ### 08:58:10.563   ->  Alice      auth
#   ### 08:58:10.739   <-  Alice      auth
#   ### 08:58:09.979       Alice      [Transport WebSocket] ws-open
# Note the direction arrows (`->` client-to-server, `<-` sever-to-client), entries
# without them are transport-level events (their type is `ws-open`/`ws-close` for
# the websocket transport, `open` for the Engine.IO transport which we skip).
_HEADING_RE = re.compile(
    r"^###\s+(?P<timestamp>\S+)\s+(?P<direction><-|->)?\s*(?P<participant>.+?)(\s+(?P<transport>\[Transport \S+\]))?\s+(?P<type>\S+)\s*$",
    re.MULTILINE,
)


class TypePlaceholder:
    """
    Placeholder replacing a field's value in an expected event to only check its
    type (useful for non-deterministic values such as timestamps).
    """

    __slots__ = ("_type",)

    def __init__(self, from_value: Any) -> None:
        self._type = type(from_value)

    def __eq__(self, other: object) -> bool:
        return isinstance(other, self._type)

    def __repr__(self) -> str:
        return f"<{self._type.__name__}>"


@dataclass(slots=True)
class RawRecordEvent:
    """One event straight out of the record, before any patching."""

    timestamp: str  # Don't parse timestamp as it is only used in `__repr__`
    direction: Literal["client-to-server"] | Literal["server-to-client"] | Literal["transport"]
    participant: str
    type: str
    payload: Any  # JSON value (dict for protocol events, str for transport rows)


type OOEvent = dict[str, Any]
"JSON-based OnlyOffice protocol event"


@dataclass(slots=True)
class RecordEvent:
    timestamp: str  # Don't parse timestamp as it is only used in `__repr__`
    direction: Literal["client-to-server"] | Literal["server-to-client"] | Literal["transport"]
    participant: str
    # Special events types:
    # - `ws-open` (direction `transport`): open the transport-level connection,
    #   its payload carries the participant ID to use (`participant_id` field)
    # - `ws-close` (direction `transport`): close the transport-level connection
    # - `license`: in our OnlyOffice fork, the client generates its own license
    #   event instead of waiting for the server to send it
    # - `documentOpen`: in our OnlyOffice fork, this event is never sent by the server
    #   and instead is injected by the client when it receives the server `auth` event.
    type: Literal["license", "documentOpen", "ws-open", "ws-close"] | str
    payload: Any

    def __repr__(self) -> str:
        direction = {
            "client-to-server": "->s",
            "server-to-client": "c<-",
            "transport": "tr-",
        }[self.direction]
        return f"{self.timestamp}\t{direction}\t{self.participant}\t{self.type}"


def _patch_composite_participant_ids(
    value: Any, index_to_participant_id: dict[tuple[str, int], EditicsParticipantID]
) -> Any:
    """Replace the OnlyOffice composite participant IDs (`<deviceId><indexUser>`)
    with the editics participant ID of the editor stint the record assigned this
    `indexUser` to (see `_parse_record`)."""

    def _recursive_patch(value: Any) -> Any:
        match value:
            case dict():
                return {k: _recursive_patch(v) for k, v in value.items()}
            case list():
                return [_recursive_patch(v) for v in value]
            case str() if m := _COMPOSITE_PARTICIPANT_ID_RE.match(value):
                participant_name = DEVICE_ID_TO_PARTICIPANT[m.group(1)]
                index_user = int(m.group(2))
                participant_id = index_to_participant_id.get((participant_name, index_user))
                assert participant_id is not None, (
                    f"Composite participant ID `{value}` references {participant_name} with"
                    f" `indexUser` {index_user}, but no `auth` reply in the record assigned"
                    " this index (unexpected record ?)"
                )
                return participant_id
            case _:
                return value

    return _recursive_patch(value)


def _parse_record(
    record_path: Path,
) -> tuple[int, list[RecordEvent], dict[EditicsParticipantID, str]]:
    """
    Parse the record's events, returning the editor type (0=Word, 1=Spreadsheet,
    2=Presentation), the events, and a label per participant ID (for readable
    error messages, e.g. `Alice stint 2`).
    """
    record = record_path.read_text()

    # Pass 1: split the record into raw events.

    raw_events: list[RawRecordEvent] = []
    e1s, e2s = itertools.tee(_HEADING_RE.finditer(record), 2)
    next(e2s)
    for e1, e2 in itertools.zip_longest(e1s, e2s):
        if e1.group("transport") == "[Transport Engine.IO]":
            # socket.io artifact with no editics counterpart
            continue
        type_ = e1.group("type")
        if e2:
            event_content = record[e1.start() : e2.start()]
        else:  # Last event
            event_content = record[e1.start() :]

        participant = e1.group("participant")
        assert participant in PARTICIPANT_NAMES, (
            f"Unexpected participant name `{participant}`, tl;dr: capture your records using "
            "`cd docs/rfcs/1030-collaborative-editics/oo-protocol-monitor/ && npm run with-docker`"
        )

        if e1.group("transport"):
            direction: (
                Literal["client-to-server"] | Literal["server-to-client"] | Literal["transport"]
            ) = "transport"
        elif e1.group("direction") == "->":
            direction = "client-to-server"
        else:
            direction = "server-to-client"

        raw_events.append(
            RawRecordEvent(
                timestamp=e1.group("timestamp"),
                direction=direction,
                participant=participant,
                type=type_,
                payload=json.loads(event_content.split("```json")[1].split("```")[0].strip()),
            )
        )

    # Pass 2: split each participant's events into stints (the transport-level
    # connection lifetimes, delimited by the `ws-open`/`ws-close` rows), and
    # determine each stint's participant ID.

    stint_of_event: list[int] = []
    per_participant_stint_count: dict[str, int] = defaultdict(int)
    opened_stints: set[tuple[str, int]] = set()
    for raw_event in raw_events:
        if raw_event.direction == "transport" and raw_event.type == "ws-open":
            per_participant_stint_count[raw_event.participant] += 1
            opened_stints.add(
                (raw_event.participant, per_participant_stint_count[raw_event.participant])
            )
        stint_of_event.append(per_participant_stint_count[raw_event.participant])

    # Each stint's participant ID is derived from the OnlyOffice session ID
    # carried by its `auth` events (both the request of a restore and the reply
    # of a fresh join carry it, and a restore's request/reply agree on it).
    stint_participant_ids: dict[tuple[str, int], EditicsParticipantID] = {}
    for raw_event, stint in zip(raw_events, stint_of_event, strict=True):
        if raw_event.type != "auth":
            continue
        session_id = (
            raw_event.payload.get("sessionId") if isinstance(raw_event.payload, dict) else None
        )
        if session_id is not None:
            stint_participant_ids.setdefault(
                (raw_event.participant, stint), _participant_id_from_session_id(session_id)
            )
    for participant, stint in sorted(opened_stints):
        assert (participant, stint) in stint_participant_ids, (
            f"No `auth` event with a session ID found for {participant} stint {stint}"
        )

    # Resolve the composites: `<deviceId><indexUser>` -> the participant ID of the
    # stint that got this index assigned in its `auth` reply. The map is built
    # before any patching since a composite can be referenced *before* the
    # corresponding `auth` reply appears in the record (e.g. a `connectState`
    # broadcast mentioning a newcomer).
    index_to_participant_id: dict[tuple[str, int], EditicsParticipantID] = {}
    for raw_event, stint in zip(raw_events, stint_of_event, strict=True):
        if raw_event.type == "auth" and raw_event.direction == "server-to-client":
            index_to_participant_id[(raw_event.participant, raw_event.payload["indexUser"])] = (
                stint_participant_ids[(raw_event.participant, stint)]
            )

    # Reverse label map (participant ID -> human readable label), used to make the
    # test failure messages readable (the derived participant IDs are opaque).
    participant_id_labels: dict[EditicsParticipantID, str] = {}
    for (participant, stint), participant_id in stint_participant_ids.items():
        participant_id_labels.setdefault(participant_id, f"{participant} stint {stint}")

    # Pass 3: patch the payloads.

    events: list[RecordEvent] = []
    for raw_event, stint in zip(raw_events, stint_of_event, strict=True):
        if raw_event.direction == "transport":
            if raw_event.type == "ws-open":
                payload: Any = {
                    "participant_id": stint_participant_ids[(raw_event.participant, stint)]
                }
            else:  # ws-close (the payload is the close reason, e.g. `client close, code 1001`)
                payload = {"reason": raw_event.payload}
            events.append(
                RecordEvent(
                    timestamp=raw_event.timestamp,
                    direction="transport",
                    participant=raw_event.participant,
                    type=raw_event.type,
                    payload=payload,
                )
            )
            continue

        payload = _patch_composite_participant_ids(raw_event.payload, index_to_participant_id)
        # The OnlyOffice `sessionId` is the editics participant ID of the recipient's
        # stint (a restore's auth echoes the restored session's ID, which the editics
        # client turns into the `reconnect` info); a `null` `sessionId` (a fresh join)
        # is kept as-is, since this is what tells the server this is not a restore.
        if isinstance(payload, dict) and payload.get("sessionId") is not None:
            payload["sessionId"] = _participant_id_from_session_id(payload["sessionId"])

        # Patch the non-deterministic fields from the server events
        if raw_event.direction == "server-to-client":
            if raw_event.type == "auth":
                if "sessionTimeConnect" in payload:
                    payload["sessionTimeConnect"] = TypePlaceholder(payload["sessionTimeConnect"])
                # `openedAt` is optional in OnlyOffice protocol (`documentOpen` event
                # arrives after `auth` and always provide this field).
                # In editics protocol this field is always provided since the server
                # never sends a `documentOpen` event.
                payload["openedAt"] = TypePlaceholder(payload.get("openedAt", 0))
                for p in payload["participants"]:
                    p["connectionId"] = TypePlaceholder(p["connectionId"])

                # `locks` is a dict when opening a docx, but a list when opening
                # a pptx or a xlsx (and its entries contain server-generated
                # timestamps).
                locks = payload["locks"]
                for lock in locks.values() if isinstance(locks, dict) else locks:
                    lock["time"] = TypePlaceholder(lock["time"])

                # `messages` field is only present if some messages has been send
                for m in payload.get("messages", []):
                    m["docid"] = f"f0000000000000000000000000000004/{DOCUMENT_ID.hex}"
                    m["time"] = TypePlaceholder(m["time"])

                # Hardcoded values in `client/editics/collaborative_protocol.js`
                payload["hasForgotten"] = False
                payload["jwt"] = ""
                payload["g_cAscSpellCheckUrl"] = ""
                payload["buildVersion"] = ""
                payload["buildNumber"] = 0
                payload["licenseType"] = 0
                payload["settings"] = {
                    "reconnection": {"attempts": 50, "delay": 2000},
                    "binaryChanges": False,
                    "websocketMaxPayloadSize": 1572864,
                    "maxChangesSize": 157286400,
                    "limits_image_size": 26214400,
                    "limits_image_types_upload": "jpg;jpeg;jpe;png;gif;bmp;svg;tiff;tif;webp;heic;heif;avif",
                }

            elif raw_event.type == "waitAuth":
                payload["lockDocument"]["connectionId"] = TypePlaceholder(
                    payload["lockDocument"]["connectionId"]
                )
            elif raw_event.type == "connectState":
                payload["participantsTimestamp"] = TypePlaceholder(payload["participantsTimestamp"])
                for p in payload["participants"]:
                    p["connectionId"] = TypePlaceholder(p["connectionId"])
            elif raw_event.type == "cursor":
                for m in payload["messages"]:
                    m["time"] = TypePlaceholder(m["time"])
            elif raw_event.type == "message":
                for m in payload.get("messages", []):
                    m["docid"] = f"f0000000000000000000000000000004/{DOCUMENT_ID.hex}"
                    m["time"] = TypePlaceholder(m["time"])
            elif raw_event.type == "unSaveLock":
                # `time` is either the server timestamp of the last change, or
                # -1 if the save has been cancelled.
                payload["time"] = -1 if payload["time"] == -1 else TypePlaceholder(payload["time"])
            elif raw_event.type == "saveChanges":
                for c in payload["changes"] or []:
                    c["docid"] = f"f0000000000000000000000000000004/{DOCUMENT_ID.hex}"
                    c["time"] = TypePlaceholder(c["time"])
                for lock in payload.get("locks") or []:
                    lock["time"] = TypePlaceholder(lock["time"])
                # The client-side translation layer only carries `excelAdditionalInfo`
                # when present (the OnlyOffice protocol has it optional).
                if payload.get("excelAdditionalInfo") is None:
                    payload.pop("excelAdditionalInfo", None)
            elif raw_event.type == "authChanges":
                for c in payload["changes"]:
                    c["docid"] = f"f0000000000000000000000000000004/{DOCUMENT_ID.hex}"
                    c["time"] = TypePlaceholder(c["time"])
            elif raw_event.type == "getLock":
                for lock in payload["locks"].values():
                    lock["time"] = TypePlaceholder(lock["time"])
            elif raw_event.type == "releaseLock":
                for lock in payload["locks"]:
                    lock["time"] = TypePlaceholder(lock["time"])

        events.append(
            RecordEvent(
                timestamp=raw_event.timestamp,
                direction=raw_event.direction,
                participant=raw_event.participant,
                type=raw_event.type,
                payload=payload,
            )
        )

    editor_type = next(
        event.payload["editorType"]
        for event in events
        if event.type == "auth" and event.direction == "client-to-server"
    )

    return editor_type, events, participant_id_labels


def _compare_server_event(event_from_record: OOEvent, actual_event: OOEvent) -> bool:
    # The expected event may contain `TypePlaceholder` placeholders (installed by the
    # cooking step in `_parse_record`) for fields whose exact value is
    # non-deterministic; those placeholders only check the value's type through
    # their `__eq__`, so a plain dict equality is enough here.
    return event_from_record == actual_event


async def _do_test_record(
    record_path: Path,
    coolorg: CoolorgRpcClients,
    backend: Backend,
    editics_js_runtime: EditicsJSRuntime,
) -> None:
    """
    Run a captured OnlyOffice record against the real server.

    Client events (`->`) are injected through the editics JS translator (which
    turns them into editics client events sent to the server), and server
    events (`<-`) are expected to reach the client as OnlyOffice events
    produced by the translator from the server's SSE broadcasts.

    The transport-level rows (`ws-open`/`ws-close`) open/close the SSE connection
    of the corresponding participant's JS client.
    """
    editor_type, events, participant_id_labels = _parse_record(record_path)

    # Originally, Bob is READER and Mallory is not part of the workspace. Correct
    # those roles if needed.
    for event in events:
        if event.type == "auth" and event.direction == "client-to-server":
            if event.participant == "Bob" and event.payload["mode"] == "edit":
                await wksp1_alice_gives_role(
                    coolorg, backend, coolorg.bob.user_id, RealmRole.CONTRIBUTOR
                )
            if event.participant == "Mallory":
                await wksp1_alice_gives_role(
                    coolorg, backend, coolorg.mallory.user_id, RealmRole.CONTRIBUTOR
                )

    per_participant_rpc_client: dict[str, AuthenticatedRpcClient] = {
        "Alice": coolorg.alice,
        "Bob": coolorg.bob,
        "Mallory": coolorg.mallory,
    }

    # Handling server events is hard: the order in which they have been received
    # in the record might be arbitrary (e.g. multiple unrelated event sent
    # by concurrent operations).
    # On top of that, Parsec server can provide server event in the response
    # to a RPC request sending a client event.
    # Hence we store the received server events here until the record actually
    # mention them (which should be the case very soon, but is not guaranteed
    # to actually be from the next event).
    per_client_unacknowlegde_server_events: dict[str, list[OOEvent]] = defaultdict(list)

    async with AsyncExitStack() as clients_stack:
        # Each participant gets its own JS client, that gets replaced for each stint
        js_clients: dict[str, EditicsJSClient] = {}

        for event in events:
            print(f"Record event: {event!s}")
            participant = event.participant
            js_client = js_clients.get(participant)

            # 1) Handle client connect/disconnect

            if event.direction == "transport" and event.type == "ws-open":
                participant_id = event.payload["participant_id"]

                if js_client is not None:
                    # Sanity check: we must have been disconnected if we reconnect reconnect
                    assert not js_client.connected
                    # Re-use the JS client if in case of a session re-join
                    if js_client.participant_id != participant_id:
                        js_client = None

                if js_client is None:
                    js_client = js_clients[participant] = await clients_stack.enter_async_context(
                        editics_js_runtime.new_client(
                            who=per_participant_rpc_client[participant],
                            realm_id=coolorg.wksp1_id,
                            document_id=DOCUMENT_ID,
                            participant_id=participant_id,
                            editor_type=editor_type,
                        )
                    )

                await js_client.server_connect()

                continue

            assert js_client is not None

            if event.direction == "transport" and event.type == "ws-close":
                assert js_client.connected  # Sanity check
                await js_client.server_disconnect()
                # Events still in the dying SSE stream are lost with
                # the connection (and so are the received-but-
                # unacknowledged ones).
                per_client_unacknowlegde_server_events[participant].clear()

                continue

            # 2) Skip ignored events:

            if event.type == "license":
                # In our end-to-end encrypted use of OnlyOffice, the client generates
                # its own license event instead of waiting for the server to send it.
                continue

            elif event.type == "documentOpen":
                # In our end-to-end encrypted use of OnlyOffice, the client reads the
                # content of the document by itself (using libparsec) instead of
                # waiting for the server to send a ``documentOpen` event.
                continue

            elif event.type == "refreshToken":
                # Parsec has its own authentication system that doesn't rely on this event
                continue

            # 3) Listen/inject event we actually care for

            match event.direction:
                case "client-to-server":
                    maybe_server_event = await js_client.inject_oo_client_event(event.payload)
                    if maybe_server_event:
                        per_client_unacknowlegde_server_events[participant].append(
                            maybe_server_event
                        )

                case "server-to-client":
                    # Look for the event in the unacknowledgedd ones...
                    unacknowlegde_server_events = per_client_unacknowlegde_server_events[
                        participant
                    ]
                    for i, server_event in enumerate(unacknowlegde_server_events):
                        if _compare_server_event(event.payload, server_event):
                            # Found our event
                            del unacknowlegde_server_events[i]
                            break
                    else:
                        # ...or actually poll it from the server.
                        # In theory the conditions leading to the emission of the event
                        # we are waiting for have already been met, so here the wait
                        # should be minimal (just for some already scheduled work to
                        # settle).
                        try:
                            with anyio.fail_after(delay=0.3):  # TODO: increase this delay for CI
                                while True:
                                    server_event = await js_client.listen_oo_server_event()
                                    if _compare_server_event(event.payload, server_event):
                                        break
                                    else:
                                        # This event is not the one we are waiting for, enqueue it for later
                                        unacknowlegde_server_events.append(server_event)
                        except TimeoutError as exc:
                            pink = "\x1b[35m"
                            blue = "\x1b[34m"
                            no_color = "\x1b[0;0m"
                            exc.add_note(
                                f"{pink}Participant {blue}{participant}{pink} was waiting for event {blue}{event.type}{pink}:{no_color}\n{textwrap.indent(pprint.pformat(event.payload), prefix='\t')}"
                            )
                            if participant_id_labels:
                                labels = ", ".join(
                                    f"{blue}{label}{no_color}={participant_id}"
                                    for participant_id, label in sorted(
                                        participant_id_labels.items(),
                                        key=lambda item: item[1],
                                    )
                                )
                                exc.add_note(f"{pink}Participant IDs:{no_color} {labels}")

                            display_unacknowledged_events = ""
                            for (
                                u_participant,
                                u_events,
                            ) in per_client_unacknowlegde_server_events.items():
                                if not u_events:
                                    continue
                                display_unacknowledged_events += (
                                    f"\t{blue}{u_participant}{no_color}:\n"
                                )
                                for u_event in u_events:
                                    display_unacknowledged_events += (
                                        f"\n\t\t{blue}{u_event['type']}{no_color}:\n"
                                    )
                                    display_unacknowledged_events += f"{textwrap.indent(pprint.pformat(u_event), prefix='\t\t')}\n"
                            if display_unacknowledged_events:
                                exc.add_note(
                                    f"\n{pink}Received but unacknowledged yet events:{no_color}\n{display_unacknowledged_events}"
                                )

                            if unacknowlegde_server_events:
                                from difflib import unified_diff
                                from pprint import pformat

                                exc.add_note(
                                    "\n".join(
                                        (
                                            f"{pink}Best candidate diff{no_color}:",
                                            *unified_diff(
                                                pformat(event.payload).splitlines(),
                                                pformat(
                                                    unacknowlegde_server_events[0]
                                                ).splitlines(),
                                                fromfile="expected",
                                                tofile="candidate",
                                                # color=True,  # TODO: enable in Python 3.15
                                            ),
                                        )
                                    )
                                )

                            raise

                case "transport":
                    pass


# Generate one test per record file (e.g. `records/0_auth.md` -> `test_0_auth`)
for _record_path in sorted(RECORDS_DIR.glob("*.md")):
    if _record_path.name == "README.md":
        continue

    _test_name = f"test_{_record_path.stem}"

    def _make_test(record_path: Path, test_name: str):
        async def _test(
            coolorg: CoolorgRpcClients,
            backend: Backend,
            editics_js_runtime: EditicsJSRuntime,
        ) -> None:
            await _do_test_record(
                record_path=record_path,
                coolorg=coolorg,
                backend=backend,
                editics_js_runtime=editics_js_runtime,
            )

        _test.__name__ = test_name
        _test.__qualname__ = test_name
        return _test

    globals()[_test_name] = _make_test(_record_path, _test_name)
