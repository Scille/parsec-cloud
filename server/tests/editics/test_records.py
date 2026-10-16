# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

import itertools
import json
import pprint
import re
import textwrap
from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Literal

import anyio
from anyio.abc import TaskStatus

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

# Editics participant IDs: arbitrary UUIDs chosen client-side when joining an
# editics session (the session and participant concepts are merged in the
# editics protocol, see `editics_protocol.py`). The records' `auth` events
# carry the OnlyOffice session id on the wire (a transport-level concept),
# which `_parse_record` patches back to those values.
ALICE_PARTICIPANT_ID = "4a51295bf2d04b47b6269a8e33b120e0"
BOB_PARTICIPANT_ID = "26dbd085b4c548a48fed5577eecd9709"
MALLORY_PARTICIPANT_ID = "fdc46f2f2c584305be070cc626f4dccc"
PARTICIPANT_IDS = {
    "Alice": ALICE_PARTICIPANT_ID,
    "Bob": BOB_PARTICIPANT_ID,
    "Mallory": MALLORY_PARTICIPANT_ID,
}
DEVICE_ID_TO_PARTICIPANT = {
    "de10a11cec0010000000000000000000": "Alice",  # alice@dev1
    "de10808c001000000000000000000000": "Bob",  # bob@dev1
    "de103a11031c00100000000000000000": "Mallory",  # mallory@dev1
}
DOCUMENT_ID = VlobID.from_hex("00000000000000000000000000000d0c")

# In OnlyOffice protocol there is three concepts:
# - `connectionId`: transport-level connection (i.e. websocket connection ID)
# - `sessionId`: logical session surviving websocket reconnects
#   (initially = first socket id; reused on restore).
# - `participantId`: server's primary per-participant key, its lifespan
#   is equal to the logical session. It is a composite ID made of `<idOriginal><indexUser>`
#   (e.g. `de10a11cec00100000000000000000001` for Alice with `indexUser` 1).
# In editics protocol however:
# - Session and participant are merged together (hence `sessionId` == `participantId`).
# - Participant ID is chosen client-side (needed since it must be known before creating
#   the SSE connection).
_COMPOSITE_PARTICIPANT_ID_RE = re.compile(
    # In OnlyOffice protocol, participant ID is made of `<originalID><counter>`
    "^(" + "|".join(DEVICE_ID_TO_PARTICIPANT.keys()) + r")\d+$"
)


def _fix_session_and_participant_ids(participant: str, payload: Any) -> Any:
    def _recursive_fix_participant_ids(value: Any) -> Any:
        match value:
            case dict():
                return {k: _recursive_fix_participant_ids(v) for k, v in value.items()}
            case list():
                return [_recursive_fix_participant_ids(v) for v in value]
            case str() if m := _COMPOSITE_PARTICIPANT_ID_RE.match(value):
                return PARTICIPANT_IDS[DEVICE_ID_TO_PARTICIPANT[m.group(1)]]
            case _:
                return value

    patched_payload = _recursive_fix_participant_ids(payload)

    if "sessionId" in patched_payload:
        patched_payload["sessionId"] = PARTICIPANT_IDS[participant]

    return patched_payload


# Matches a record event heading, e.g.:
#   ### 08:58:10.563   ->  Alice      auth
#   ### 08:58:10.739   <-  Alice      auth
#   ### 08:58:09.979       Alice      [Transport WebSocket] ws-open
# Note the direction arrows (`->` client-to-server, `<-` sever-to-client), entries
# without them are client-side transport-level events that we skip.
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


def _parse_record(record_path: Path) -> tuple[int, list[RecordEvent]]:
    """
    Parse the record's events, returning the editor type (0=Word, 1=Spreadsheet,
    2=Presentation) along with the events.
    """
    record = record_path.read_text()
    events = []
    e1s, e2s = itertools.tee(_HEADING_RE.finditer(record), 2)
    next(e2s)
    for e1, e2 in itertools.zip_longest(e1s, e2s):
        if e1.group("transport"):
            continue
        if e2:
            event_content = record[e1.start() : e2.start()]
        else:  # Last event
            event_content = record[e1.start() :]
        payload = json.loads(event_content.split("```json")[1].split("```")[0].strip())

        direction = "client-to-server" if e1.group("direction") == "->" else "server-to-client"
        type_ = e1.group("type")

        participant = e1.group("participant")
        assert participant in PARTICIPANT_IDS, (
            f"Unexpected participant name `{participant}`, tl;dr: capture your records using "
            "`cd docs/rfcs/1030-collaborative-editics/oo-protocol-monitor/ && npm run with-docker`"
        )

        payload = _fix_session_and_participant_ids(participant, payload)

        # Patch the non-deterministic fields from the server events
        if direction == "server-to-client":
            if type_ == "auth":
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
                # a pptx or a xlsx...
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

            elif type_ == "waitAuth":
                payload["lockDocument"]["connectionId"] = TypePlaceholder(
                    payload["lockDocument"]["connectionId"]
                )
            elif type_ == "connectState":
                payload["participantsTimestamp"] = TypePlaceholder(payload["participantsTimestamp"])
                for p in payload["participants"]:
                    p["connectionId"] = TypePlaceholder(p["connectionId"])
            elif type_ == "cursor":
                for m in payload["messages"]:
                    m["time"] = TypePlaceholder(m["time"])
            elif type_ == "message":
                for m in payload.get("messages", []):
                    m["docid"] = f"f0000000000000000000000000000004/{DOCUMENT_ID.hex}"
                    m["time"] = TypePlaceholder(m["time"])
            elif type_ == "unSaveLock":
                # `time` is either the server timestamp of the last change, or
                # -1 if the save has been cancelled.
                payload["time"] = -1 if payload["time"] == -1 else TypePlaceholder(payload["time"])
            elif type_ == "saveChanges":
                for c in payload["changes"] or []:
                    c["docid"] = f"f0000000000000000000000000000004/{DOCUMENT_ID.hex}"
                    c["time"] = TypePlaceholder(c["time"])
                for lock in payload.get("locks") or []:
                    lock["time"] = TypePlaceholder(lock["time"])
                # The client-side translation layer only carries `excelAdditionalInfo`
                # when present (the OnlyOffice protocol has it optional).
                if payload.get("excelAdditionalInfo") is None:
                    payload.pop("excelAdditionalInfo", None)
            elif type_ == "authChanges":
                for c in payload["changes"]:
                    c["docid"] = f"f0000000000000000000000000000004/{DOCUMENT_ID.hex}"
                    c["time"] = TypePlaceholder(c["time"])
            elif type_ == "getLock":
                for lock in payload["locks"].values():
                    lock["time"] = TypePlaceholder(lock["time"])
            elif type_ == "releaseLock":
                for lock in payload["locks"]:
                    lock["time"] = TypePlaceholder(lock["time"])

        events.append(
            RecordEvent(
                timestamp=e1.group("timestamp"),
                direction=direction,
                participant=participant,
                type=type_,
                payload=payload,
            )
        )

    editor_type = next(
        event.payload["editorType"]
        for event in events
        if event.type == "auth" and event.direction == "client-to-server"
    )

    return editor_type, events


type OOEvent = dict[str, Any]
"JSON-based OnlyOffice protocol event"


@dataclass(slots=True)
class RecordEvent:
    timestamp: str  # Don't parse timestamp as it is only used in `__repr__`
    direction: Literal["client-to-server"] | Literal["server-to-client"]
    participant: EditicsParticipantID
    # Special events types:
    # - `license`: identifies the fact a new participant connects to the server
    # - `documentOpen`: in our OnlyOffice fork, this event is never sent by the server
    #   and instead is injected by the client when it receives the server `auth` event.
    type: Literal["license", "documentOpen"] | str
    payload: OOEvent

    def __repr__(self) -> str:
        direction = "->s" if self.direction == "client-to-server" else "c<-"
        return f"{self.timestamp}\t{direction}\t{self.participant}\t{self.type}"


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
    """Run a captured OnlyOffice record against the real server.

    Client events (`->`) are injected through the editics JS translator (which
    turns them into editics client events sent to the server), and server
    events (`<-`) are expected to reach the client as OnlyOffice events
    produced by the translator from the server's SSE broadcasts.
    """
    editor_type, events = _parse_record(record_path)

    # Originally, Bob is READER and Mallory is not part of the workspace:
    # both are granted a write role whenever they take part in a record.
    # This doesn't affect the records where they join in view mode
    # (`mode: "view"`): the client-requested mode is what makes them viewers
    # there (the realm role is only a floor, see `EditicsClientEventAuth.view`
    # and `records_wip/new_view/README.md`).
    if any(event.participant == "Bob" for event in events):
        await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.CONTRIBUTOR)
    if any(event.participant == "Mallory" for event in events):
        await wksp1_alice_gives_role(
            coolorg, backend, coolorg.mallory.user_id, RealmRole.CONTRIBUTOR
        )

    per_participant_id_and_client: dict[
        str, tuple[EditicsParticipantID, AuthenticatedRpcClient]
    ] = {
        "Alice": (ALICE_PARTICIPANT_ID, coolorg.alice),
        "Bob": (BOB_PARTICIPANT_ID, coolorg.bob),
        "Mallory": (MALLORY_PARTICIPANT_ID, coolorg.mallory),
    }
    async def _start_editics_js_client(
        who: AuthenticatedRpcClient,
        participant_id: EditicsParticipantID,
        *,
        task_status: TaskStatus[EditicsJSClient],
    ):
        async with editics_js_runtime.new_client(
            who=who,
            realm_id=coolorg.wksp1_id,
            document_id=DOCUMENT_ID,
            participant_id=participant_id,
            editor_type=editor_type,
        ) as editics_js_client:
            task_status.started(editics_js_client)
            await anyio.sleep_forever()

    async with anyio.create_task_group() as tg:
        running_js_clients: dict[EditicsParticipantID, EditicsJSClient] = {}
        # Handling server events is hard: the order in which they have been received
        # in the record might be arbitrary (e.g. multiple unrelated event sent
        # by concurrent operations).
        # On top of that, Parsec server can provide server event in the response
        # to a RPC request sending a client event.
        # Hence we store the received server events here until the record actually
        # mention them (which should be the case very soon, but is not guaranteed
        # to actually be from the next event).
        per_client_unacknowlegde_server_events: dict[EditicsParticipantID, list[OOEvent]] = (
            defaultdict(list)
        )

        for event in events:
            print(f"Record event: {event!s}")
            participant = event.participant

            # Initial event signifies the need to start an editics JavaScript client
            if event.type == "license":
                assert participant not in running_js_clients
                participant_id, who = per_participant_id_and_client[participant]
                running_js_clients[participant] = await tg.start(
                    _start_editics_js_client, who, participant_id, name=f"{participant} editics"
                )
                # In our end-to-end encrypted use of OnlyOffice, the client generates
                # its own license event instead of waiting for the server to send it.
                continue

            elif event.type == "documentOpen":
                # In our end-to-end encrypted use of OnlyOffice, the client reads the
                # content of the document by itself (using libparsec) instead of
                # waiting for the server to send a ``documentOpen` event.
                continue

            match event.direction:
                case "client-to-server":
                    maybe_server_event = await running_js_clients[
                        participant
                    ].inject_oo_client_event(event.payload)
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
                                    server_event = await running_js_clients[
                                        participant
                                    ].listen_oo_server_event()
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

                            display_unacknowledged_events = ""
                            for (
                                u_participant,
                                u_events,
                            ) in per_client_unacknowlegde_server_events.items():
                                if not events:
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
        tg.cancel()  # Test is done, stop the editics JavaScript clients


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
