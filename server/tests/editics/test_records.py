# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

import itertools
import json
import re
from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path
from typing import Literal

import anyio
from anyio.abc import TaskStatus

from parsec._parsec import VlobID
from tests.common import CoolorgRpcClients, EditicsJSRuntime
from tests.common.editics2 import EditicsJSClient

RECORDS_DIR = Path(__file__).resolve().parent / "records"


# Matches a record event heading, e.g.:
#   ### 08:58:10.563   ->  John Smith      auth
#   ### 08:58:10.739   <-  John Smith      auth
#   ### 08:58:09.979       John Smith      [Transport WebSocket] ws-open
# The direction arrows are optional; entries without them are transport-level
# events that we skip.
_HEADING_RE = re.compile(
    r"^###\s+(?P<timestamp>\S+)\s+(?P<direction><-|->)?\s*(?P<participant>.+?)(\s+(?P<transport>\[Transport \S+\]))?\s+(?P<type>\S+)\s*$",
    re.MULTILINE,
)


def _parse_record(record: str) -> list[RecordEvent]:
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

        events.append(
            RecordEvent(
                timestamp=e1.group("timestamp"),
                direction="client-to-server"
                if e1.group("direction") == "->"
                else "server-to-client",
                participant=e1.group("participant"),
                type=e1.group("type"),
                payload=payload,
            )
        )

    return events


# TODO
@dataclass(slots=True)
class RecordPart:
    """
    A group of OnlyOffice events that has to be entirely processed before going
    to for the next one.

    This is used to indicate the events within this case can be somewhat re-ordered
    (typically it's okay if Kate waiting for a event).
    """

    name: str
    events: list[RecordEvent]


@dataclass(slots=True)
class RecordEvent:
    timestamp: str  # Don't parse timestamp as it is only used in `__repr__`
    direction: Literal["client-to-server"] | Literal["server-to-client"]
    participant: str
    type: (
        Literal["license"] | str
    )  # `license` event identifies the fact a new participant connects to the server
    payload: dict

    def __repr__(self) -> str:
        direction = "->" if self.direction == "client-to-server" else "<-"
        return f"{self.timestamp}\t{direction}\t{self.participant}\t{self.type}"


def _compare_server_event(event_from_record: dict, actual_event: dict) -> bool:
    breakpoint()
    return True
    # match event_from_record["type"]:
    #     case "auth":
    #     case _:
    #         return event_from_record == actual_event


async def _do_test_record(
    record_path: Path,
    coolorg: CoolorgRpcClients,
    editics_js_runtime: EditicsJSRuntime,
) -> None:
    """Run a captured OnlyOffice record against the real server.

    Client events (`->`) are injected through the editics JS translator (which
    turns them into editics client events sent to the server), and server
    events (`<-`) are expected to reach the client as OnlyOffice events
    produced by the translator from the server's SSE broadcasts.
    """
    events = _parse_record(
        record_path.read_text()  # noqa: ASYNC240
    )

    async def _start_editics_js_client(task_status: TaskStatus):
        async with editics_js_runtime.new_client(
            # All participants connect to the server as Alice for simplicity
            who=coolorg.alice,
            realm_id=coolorg.wksp1_id,
            document_id=VlobID.new(),
        ) as editics_js_client:
            task_status.started(editics_js_client)
            await anyio.sleep_forever()

    async with anyio.create_task_group() as tg:
        running_js_clients: dict[str, EditicsJSClient] = {}
        # Parsec server can provide server event in the response to a RPC request
        # sending a client event.
        # Hence we store those responses here until the record actually mention
        # them (which should be the case very soon, but is not guaranteed to actually
        # be the next event).

        # Handling server events is hard: the order in which they have been received
        # in the record might be arbitrary (e.g. multiple unrelated event sent
        # by concurrent operations).
        per_client_unacknowlegde_server_events: dict[str, list[dict]] = defaultdict(list)

        for event in events:
            print(f"Record event: {event!s}")
            participant = event.participant

            # Initial event signifies the need to start an editics JavaScript client
            if event.type == "license":
                assert participant not in running_js_clients
                running_js_clients[participant] = await tg.start(
                    _start_editics_js_client, name=f"{participant} editics"
                )

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
                    # Look for the event in the unacknowledged ones...
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
                            with anyio.fail_after(delay=3):
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
                            exc.add_note(
                                f"Participant {participant} was waiting for event: {event.payload}"
                            )

                            display_unacknowledge_events = ""
                            for (
                                participant,
                                events,
                            ) in per_client_unacknowlegde_server_events.items():
                                if not events:
                                    continue
                                display_unacknowledge_events += f"\t{participant}:"
                                for event in events:
                                    display_unacknowledge_events += f"\t\t{event}:"
                            if display_unacknowledge_events:
                                exc.add_note(
                                    f"Participant {participant} has some unacknowledge events: {display_unacknowledge_events}"
                                )

                            raise


# Generate one test per record file (e.g. `records/0_auth.md` -> `test_0_auth`)
for _record_path in sorted(RECORDS_DIR.glob("*.md")):
    _test_name = f"test_{_record_path.stem}"

    def _make_test(record_path: Path, test_name: str):
        async def _test(
            coolorg: CoolorgRpcClients,
            editics_js_runtime: EditicsJSRuntime,
        ) -> None:
            await _do_test_record(
                record_path=record_path,
                coolorg=coolorg,
                editics_js_runtime=editics_js_runtime,
            )

        _test.__name__ = test_name
        _test.__qualname__ = test_name
        return _test

    globals()[_test_name] = _make_test(_record_path, _test_name)

del _record_path
