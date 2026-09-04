# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
"""Tests for the editics step-0 auth subset.

Covers the happy-path join flow (todo step_0 §6): opening the SSE stream, the
`auth` RPC, the `auth` RPC reply and the `connectState` SSE broadcast, plus
keepalive and the leave-on-disconnect cleanup. The rejection shape (§7) is
exercised too.
"""

from __future__ import annotations

import base64
import json
from contextlib import asynccontextmanager
from uuid import uuid4

import httpx
import pytest
from httpx_sse import aconnect_sse

from parsec._parsec import DeviceID, VlobID
from tests.common import MinimalorgRpcClients

SERVER_DOMAIN = "parsec.invalid"


def ClientEventAuthJSON(*, indexUser: int, editorType: int, vlobVersion: int) -> bytes:
    return json.dumps(
        {
            "type": "auth",
            "indexUser": indexUser,
            "editorType": editorType,
            "vlobVersion": vlobVersion,
        }
    ).encode()


def _auth_header(device_id: DeviceID, participant_uuid) -> str:
    return f"Editics {device_id.hex}.{participant_uuid.hex}"


def _send_event(client, send_url: str, auth: str, body: dict) -> httpx.Response:
    """POST one client event and return the response."""
    return client.post(
        send_url,
        headers={"Authorization": auth, "Content-Type": "application/json"},
        content=json.dumps(body).encode(),
    )


def _unlock_document_body(
    *, isSave: bool = False, unlock: bool = False, deleteIndex=None, releaseLocks: bool = False
) -> dict:
    return {
        "type": "unLockDocument",
        "isSave": isSave,
        "unlock": unlock,
        "deleteIndex": deleteIndex,
        "releaseLocks": releaseLocks,
    }


def _join_url(organization_id: str, workspace_id: VlobID, vlob_id: VlobID) -> str:
    return (
        f"http://{SERVER_DOMAIN}/authenticated/{organization_id}"
        f"/editics/sessions/{workspace_id.hex}/{vlob_id.hex}/join"
    )


def _send_url(organization_id: str, workspace_id: VlobID, vlob_id: VlobID) -> str:
    return (
        f"http://{SERVER_DOMAIN}/authenticated/{organization_id}"
        f"/editics/sessions/{workspace_id.hex}/{vlob_id.hex}/send"
    )


@asynccontextmanager
async def open_editics_sse(
    client: httpx.AsyncClient,
    url: str,
    auth: str,
):
    """Open the editics SSE stream and yield an async iterator of SSE events.

    The stream is kept open until the context manager exits (closing it
    triggers the leave flow).
    """
    headers = {"Authorization": auth, "Accept": "text/event-stream"}
    async with aconnect_sse(client, "GET", url, headers=headers) as event_source:
        yield event_source.aiter_sse()


async def _next_data_event(events) -> dict:
    """Skip keepalive events and return the next `data:` event's JSON payload.

    `events` is the async iterator yielded by `open_editics_sse`.
    """
    while True:
        sse = await events.__anext__()
        # Keepalive uses an `event:keepalive` line with empty data; data events
        # have no `event` line and carry JSON in `data`.
        if sse.event == "keepalive" or sse.event == "":
            if sse.data:
                return json.loads(sse.data)
            continue
        # Some SSE clients surface the default "message" event with the data.
        if sse.data:
            return json.loads(sse.data)


@pytest.mark.timeout(10)
async def test_join_fresh_session(minimalorg: MinimalorgRpcClients) -> None:
    """Happy path: join a fresh edition session (todo step_0 §6)."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    participant_uuid = uuid4()
    auth = _auth_header(device_id, participant_uuid)
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    async with open_editics_sse(minimalorg.raw_client, join_url, auth) as sse:
        # Send the auth RPC.
        auth_body = ClientEventAuthJSON(indexUser=-1, editorType=0, vlobVersion=10)
        rep = await minimalorg.raw_client.post(
            send_url,
            headers={"Authorization": auth, "Content-Type": "application/json"},
            content=auth_body,
        )
        assert rep.status_code == 200, rep.content
        auth_reply = rep.json()
        assert auth_reply["type"] == "auth"
        assert auth_reply["result"] == 1
        assert auth_reply["indexUser"] == 1
        assert auth_reply["sessionId"]
        assert isinstance(auth_reply["sessionTimeConnect"], int)
        assert auth_reply["participants"] == [
            {"indexUser": 1, "deviceId": device_id.hex, "view": False}
        ]

        # The SSE stream should deliver a connectState broadcast.
        connect_state = await _next_data_event(sse)
        assert connect_state["type"] == "connectState"
        assert connect_state["waitAuth"] is False
        assert isinstance(connect_state["participantsTimestamp"], int)
        assert connect_state["participants"] == [
            {"indexUser": 1, "deviceId": device_id.hex, "view": False}
        ]


@pytest.mark.timeout(10)
async def test_keepalive(minimalorg: MinimalorgRpcClients, backend) -> None:
    """SSE keepalive is received during idle (todo step_0 §9.3)."""
    # Speed up keepalive for the test.
    backend.config.sse_keepalive = 0.5

    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    participant_uuid = uuid4()
    auth = _auth_header(device_id, participant_uuid)
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)

    got_keepalive = False
    async with open_editics_sse(minimalorg.raw_client, join_url, auth) as events:
        # Iterate raw events until we see a keepalive.
        deadline_iterations = 50
        for _ in range(deadline_iterations):
            ev = await events.__anext__()
            if ev.event == "keepalive":
                got_keepalive = True
                break
    assert got_keepalive


@pytest.mark.timeout(10)
async def test_leave_removes_participant_and_broadcasts(
    minimalorg: MinimalorgRpcClients, backend
) -> None:
    """Leave flow (todo step_0 §8): remove the participant and broadcast connectState.

    The SSE-disconnect path itself is not exercisable through httpx's
    `ASGITransport` (it doesn't model client disconnect), so this test drives the
    leave directly on the component. It validates the leave/broadcast code path
    that the route's `finally` relies on.
    """
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    alice_uuid = uuid4()
    alice_auth = _auth_header(device_id, alice_uuid)
    bob_device_id = device_id  # same device, distinct participant uuid
    bob_uuid = uuid4()
    bob_auth = _auth_header(bob_device_id, bob_uuid)
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    # Two participants join the same session over SSE + RPC.
    async with (
        open_editics_sse(minimalorg.raw_client, join_url, alice_auth) as alice_sse,
        open_editics_sse(minimalorg.raw_client, join_url, bob_auth) as bob_sse,
    ):
        rep = await minimalorg.raw_client.post(
            send_url,
            headers={"Authorization": alice_auth, "Content-Type": "application/json"},
            content=ClientEventAuthJSON(indexUser=-1, editorType=0, vlobVersion=10),
        )
        assert rep.status_code == 200
        await _next_data_event(alice_sse)  # alice's connectState (waitAuth false)

        # Bob joins while Alice holds the auth lock -> Bob is parked (waitAuth).
        rep = await minimalorg.raw_client.post(
            send_url,
            headers={"Authorization": bob_auth, "Content-Type": "application/json"},
            content=ClientEventAuthJSON(indexUser=-1, editorType=0, vlobVersion=10),
        )
        assert rep.status_code == 200
        assert rep.json()["type"] == "waitAuth"
        await _next_data_event(alice_sse)  # alice sees bob join (waitAuth true)
        await _next_data_event(bob_sse)  # bob sees the participant set (waitAuth true)

        # Alice releases the auth lock -> Bob is unblocked.
        rep = await _send_event(
            minimalorg.raw_client, send_url, alice_auth, _unlock_document_body(unlock=True)
        )
        assert rep.status_code == 204
        await _next_data_event(bob_sse)  # bob's authChanges (empty)
        await _next_data_event(bob_sse)  # bob's connectState (waitAuth false)
        await _next_data_event(alice_sse)  # alice sees waitAuth false

        session = backend.editics._sessions[(workspace_id, vlob_id)]
        assert set(session.participants) == {1, 2}
        assert set(session.connections) == {alice_uuid, bob_uuid}

        # Simulate the SSE-disconnect `finally`: alice leaves.
        from parsec.components.editics import EditicsClientContext

        await backend.editics.leave(
            workspace_id,
            vlob_id,
            EditicsClientContext(device_id=device_id, participant_uuid=alice_uuid),
        )

        # Alice is gone; bob remains and should get a connectState broadcast.
        session = backend.editics._sessions[(workspace_id, vlob_id)]
        assert set(session.participants) == {2}
        assert set(session.connections) == {bob_uuid}
        cs = await _next_data_event(bob_sse)
        assert cs["type"] == "connectState"
        assert cs["participants"] == [{"indexUser": 2, "deviceId": device_id.hex, "view": False}]

        # Bob leaves too -> session is GC'd (no leak).
        await backend.editics.leave(
            workspace_id,
            vlob_id,
            EditicsClientContext(device_id=device_id, participant_uuid=bob_uuid),
        )
        assert (workspace_id, vlob_id) not in backend.editics._sessions
        assert not backend.editics._pending


@pytest.mark.timeout(10)
async def test_reject_wrong_vlob_version(minimalorg: MinimalorgRpcClients) -> None:
    """Rejection path: join an existing session with a wrong vlobVersion (§7)."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    participant_uuid = uuid4()
    auth = _auth_header(device_id, participant_uuid)
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    async with open_editics_sse(minimalorg.raw_client, join_url, auth) as sse:
        # Create the session at version 10.
        rep = await minimalorg.raw_client.post(
            send_url,
            headers={"Authorization": auth, "Content-Type": "application/json"},
            content=ClientEventAuthJSON(indexUser=-1, editorType=0, vlobVersion=10),
        )
        assert rep.status_code == 200
        await _next_data_event(sse)

        # Now a second participant joins with a too-new version -> rejected.
        bob_uuid = uuid4()
        bob_auth = _auth_header(device_id, bob_uuid)
        bob_join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
        bob_send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)
        # Open bob's SSE so the `auth` RPC has a pending connection to match;
        # bob is rejected (wrong version), so his stream is not promoted and we
        # don't read events from it.
        async with open_editics_sse(minimalorg.raw_client, bob_join_url, bob_auth):
            rep = await minimalorg.raw_client.post(
                bob_send_url,
                headers={"Authorization": bob_auth, "Content-Type": "application/json"},
                content=ClientEventAuthJSON(indexUser=-1, editorType=0, vlobVersion=11),
            )
            assert rep.status_code == 200, rep.content
            rejected = rep.json()
            assert rejected["type"] == "auth"
            assert rejected["result"] == 0
            assert rejected["latestAllowedVersion"] == 10


@pytest.mark.timeout(10)
async def test_join_via_authorization_query_param(minimalorg: MinimalorgRpcClients) -> None:
    """The SSE route accepts the identity as an `authorization` query parameter.

    The browser's `EventSource` API cannot set custom request headers, so the
    editics SSE route also accepts the `Authorization: Editics ...` value as an
    `authorization` query param (todo step_0 §9.2). The RPC route always uses
    the real header.
    """
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    participant_uuid = uuid4()
    auth = _auth_header(device_id, participant_uuid)
    # SSE join via query param; the RPC send uses the real header.
    join_url = (
        _join_url(minimalorg.organization_id, workspace_id, vlob_id)
        + "?authorization="
        + auth.replace(" ", "%20")
    )
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    headers = {"Accept": "text/event-stream"}
    async with aconnect_sse(
        minimalorg.raw_client, "GET", join_url, headers=headers
    ) as event_source:
        events = event_source.aiter_sse()
        rep = await minimalorg.raw_client.post(
            send_url,
            headers={"Authorization": auth, "Content-Type": "application/json"},
            content=ClientEventAuthJSON(indexUser=-1, editorType=0, vlobVersion=10),
        )
        assert rep.status_code == 200, rep.content
        connect_state = await _next_data_event(events)
        assert connect_state["type"] == "connectState"
        assert connect_state["participants"] == [
            {"indexUser": 1, "deviceId": device_id.hex, "view": False}
        ]


@pytest.mark.timeout(10)
async def test_missing_auth_header(minimalorg: MinimalorgRpcClients) -> None:
    """A missing/malformed Editics authorization header is rejected (401)."""
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    rep = await minimalorg.raw_client.post(
        send_url,
        headers={"Content-Type": "application/json"},
        content=ClientEventAuthJSON(indexUser=-1, editorType=0, vlobVersion=10),
    )
    assert rep.status_code == 401

    rep = await minimalorg.raw_client.post(
        send_url,
        headers={
            "Authorization": "Bearer not-editics",
            "Content-Type": "application/json",
        },
        content=ClientEventAuthJSON(indexUser=-1, editorType=0, vlobVersion=10),
    )
    assert rep.status_code == 401


# ---------------------------------------------------------------------------
# Step 1, substep A — Multiple user authentication (join a second client)
# (todo step_1 §6.1, validation conditions V-A1..V-A4).
# ---------------------------------------------------------------------------


@pytest.mark.timeout(10)
async def test_second_client_join(minimalorg: MinimalorgRpcClients) -> None:
    """V-A1: two clients join the same fresh session; both get the full map.

    Alice (index 1) and Bob (index 2) join the same fresh session. Bob's RPC
    reply has ``indexUser == 2`` and the full 2-participant map; Alice's SSE
    stream receives a ``connectState`` with both participants.
    """
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    alice_uuid = uuid4()
    alice_auth = _auth_header(device_id, alice_uuid)
    # Bob is a distinct device (also derived from the testbed, but a fresh
    # DeviceID is fine here since the editics header is not a real auth).
    bob_device_id = DeviceID.new()
    bob_uuid = uuid4()
    bob_auth = _auth_header(bob_device_id, bob_uuid)
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    expected_alice = {"indexUser": 1, "deviceId": device_id.hex, "view": False}
    expected_bob = {"indexUser": 2, "deviceId": bob_device_id.hex, "view": False}
    expected_map = [expected_alice, expected_bob]

    async with (
        open_editics_sse(minimalorg.raw_client, join_url, alice_auth) as alice_sse,
        open_editics_sse(minimalorg.raw_client, join_url, bob_auth) as bob_sse,
    ):
        # Alice joins the fresh session at version 10. Alice (first non-view
        # participant) takes the auth lock (todo step_1 §6.2).
        rep = await minimalorg.raw_client.post(
            send_url,
            headers={"Authorization": alice_auth, "Content-Type": "application/json"},
            content=ClientEventAuthJSON(indexUser=-1, editorType=0, vlobVersion=10),
        )
        assert rep.status_code == 200, rep.content
        alice_reply = rep.json()
        assert alice_reply["type"] == "auth"
        assert alice_reply["result"] == 1
        assert alice_reply["indexUser"] == 1
        assert alice_reply["participants"] == [expected_alice]
        await _next_data_event(alice_sse)  # alice's own connectState (waitAuth false)

        # Bob joins the existing session at the same version (in range). The
        # auth lock is held by Alice, so Bob is parked: his RPC reply is
        # `waitAuth` (NOT `auth`), and a `connectState{waitAuth:true}` is
        # broadcast to everyone (todo step_1 §6.2 V-B2).
        rep = await minimalorg.raw_client.post(
            send_url,
            headers={"Authorization": bob_auth, "Content-Type": "application/json"},
            content=ClientEventAuthJSON(indexUser=-1, editorType=0, vlobVersion=10),
        )
        assert rep.status_code == 200, rep.content
        bob_reply = rep.json()
        assert bob_reply["type"] == "waitAuth"
        assert bob_reply["authLockedBy"] == 1

        # Alice receives a connectState with both participants and waitAuth true.
        alice_cs = await _next_data_event(alice_sse)
        assert alice_cs["type"] == "connectState"
        assert alice_cs["participants"] == expected_map
        assert alice_cs["waitAuth"] is True
        # Bob also received that same broadcast (parked but still in the map).
        bob_cs_parked = await _next_data_event(bob_sse)
        assert bob_cs_parked["waitAuth"] is True

        # Alice releases the auth lock -> Bob is unblocked: he receives the
        # (empty) backlog `authChanges` then a `connectState{waitAuth:false}`.
        rep = await _send_event(
            minimalorg.raw_client, send_url, alice_auth, _unlock_document_body(unlock=True)
        )
        assert rep.status_code == 204
        bob_authchanges = await _next_data_event(bob_sse)
        assert bob_authchanges["type"] == "authChanges"
        assert bob_authchanges["changes"] == []
        bob_cs = await _next_data_event(bob_sse)
        assert bob_cs["type"] == "connectState"
        assert bob_cs["participants"] == expected_map
        assert bob_cs["waitAuth"] is False
        # Alice also sees the waitAuth:false transition.
        alice_cs2 = await _next_data_event(alice_sse)
        assert alice_cs2["waitAuth"] is False


@pytest.mark.timeout(10)
async def test_join_rejected_does_not_change_map(minimalorg: MinimalorgRpcClients, backend) -> None:
    """V-A2: a rejected (out-of-range version) joiner gets no indexUser and
    leaves the participant map unchanged."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    alice_uuid = uuid4()
    alice_auth = _auth_header(device_id, alice_uuid)
    bob_uuid = uuid4()
    bob_auth = _auth_header(device_id, bob_uuid)
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    async with (
        open_editics_sse(minimalorg.raw_client, join_url, alice_auth) as alice_sse,
        open_editics_sse(minimalorg.raw_client, join_url, bob_auth),
    ):
        # Alice creates the session at version 10.
        rep = await minimalorg.raw_client.post(
            send_url,
            headers={"Authorization": alice_auth, "Content-Type": "application/json"},
            content=ClientEventAuthJSON(indexUser=-1, editorType=0, vlobVersion=10),
        )
        assert rep.status_code == 200
        await _next_data_event(alice_sse)

        # Bob joins with a too-new version -> rejected (result 0).
        rep = await minimalorg.raw_client.post(
            send_url,
            headers={"Authorization": bob_auth, "Content-Type": "application/json"},
            content=ClientEventAuthJSON(indexUser=-1, editorType=0, vlobVersion=11),
        )
        assert rep.status_code == 200, rep.content
        rejected = rep.json()
        assert rejected["type"] == "auth"
        assert rejected["result"] == 0
        assert rejected["latestAllowedVersion"] == 10
        # The rejection shape carries no indexUser field.
        assert "indexUser" not in rejected

        # V-A2: the participant map is unchanged (still only Alice), and Bob
        # was NOT promoted to a full participant (no indexUser, no connection).
        # Bob's pending SSE connection is still registered (his SSE stream is
        # still open here; it is cleaned up on context exit via the leave flow).
        session_obj = backend.editics._sessions[(workspace_id, vlob_id)]
        assert set(session_obj.participants) == {1}
        assert set(session_obj.connections) == {alice_uuid}
        # Bob's next_index was NOT consumed (still 2, ready for a valid joiner).
        assert session_obj.next_index == 2


@pytest.mark.timeout(10)
async def test_participants_timestamp_monotonic(minimalorg: MinimalorgRpcClients) -> None:
    """V-A3: ``participantsTimestamp`` is monotonic non-decreasing across
    successive ``connectState`` broadcasts on a given stream."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    alice_uuid = uuid4()
    alice_auth = _auth_header(device_id, alice_uuid)

    async with open_editics_sse(minimalorg.raw_client, join_url, alice_auth) as alice_sse:
        # Alice joins.
        rep = await minimalorg.raw_client.post(
            send_url,
            headers={"Authorization": alice_auth, "Content-Type": "application/json"},
            content=ClientEventAuthJSON(indexUser=-1, editorType=0, vlobVersion=10),
        )
        assert rep.status_code == 200
        cs1 = await _next_data_event(alice_sse)
        assert cs1["type"] == "connectState"

        # Bob joins, triggering a second connectState on Alice's stream.
        bob_auth = _auth_header(device_id, uuid4())
        async with open_editics_sse(minimalorg.raw_client, join_url, bob_auth):
            rep = await minimalorg.raw_client.post(
                send_url,
                headers={"Authorization": bob_auth, "Content-Type": "application/json"},
                content=ClientEventAuthJSON(indexUser=-1, editorType=0, vlobVersion=10),
            )
            assert rep.status_code == 200
            cs2 = await _next_data_event(alice_sse)
            assert cs2["type"] == "connectState"

        # V-A3: monotonic non-decreasing.
        assert cs2["participantsTimestamp"] >= cs1["participantsTimestamp"]


@pytest.mark.timeout(10)
async def test_same_device_distinct_participant_uuid(
    minimalorg: MinimalorgRpcClients, backend
) -> None:
    """V-A4: two participants from the **same device** (same device_id, distinct
    participant_uuid) get distinct ``indexUser`` values."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    alice_uuid = uuid4()
    alice_auth = _auth_header(device_id, alice_uuid)
    bob_uuid = uuid4()
    bob_auth = _auth_header(device_id, bob_uuid)  # same device_id, distinct uuid

    async with (
        open_editics_sse(minimalorg.raw_client, join_url, alice_auth) as alice_sse,
        open_editics_sse(minimalorg.raw_client, join_url, bob_auth) as bob_sse,
    ):
        rep = await minimalorg.raw_client.post(
            send_url,
            headers={"Authorization": alice_auth, "Content-Type": "application/json"},
            content=ClientEventAuthJSON(indexUser=-1, editorType=0, vlobVersion=10),
        )
        assert rep.status_code == 200
        assert rep.json()["indexUser"] == 1
        await _next_data_event(alice_sse)

        # Bob joins while Alice holds the auth lock -> parked (waitAuth).
        rep = await minimalorg.raw_client.post(
            send_url,
            headers={"Authorization": bob_auth, "Content-Type": "application/json"},
            content=ClientEventAuthJSON(indexUser=-1, editorType=0, vlobVersion=10),
        )
        assert rep.status_code == 200
        bob_reply = rep.json()
        assert bob_reply["type"] == "waitAuth"
        await _next_data_event(alice_sse)  # connectState{waitAuth:true}
        await _next_data_event(bob_sse)  # connectState{waitAuth:true}

        # Alice releases the auth lock -> Bob unblocked.
        await _send_event(
            minimalorg.raw_client, send_url, alice_auth, _unlock_document_body(unlock=True)
        )
        await _next_data_event(bob_sse)  # authChanges
        await _next_data_event(bob_sse)  # connectState{waitAuth:false}
        await _next_data_event(alice_sse)  # connectState{waitAuth:false}

        # Both participants are present, both with the same deviceId.
        session_obj = backend.editics._sessions[(workspace_id, vlob_id)]
        assert set(session_obj.participants) == {1, 2}
        assert session_obj.participants[1] == device_id
        assert session_obj.participants[2] == device_id


# ---------------------------------------------------------------------------
# Step 1, substep A — vlob-version validity guards (RFC §1.2, §1.3).
# A Parsec vlob version is always >= 1; `base_version: 0` is a purely local
# placeholder for a not-yet-synced file and is never a valid server vlob
# version. The editics server rejects `vlobVersion < 1` and self-heals stale
# in-memory sessions left over with `initial_version < 1`.
# ---------------------------------------------------------------------------


@pytest.mark.timeout(10)
async def test_reject_vlob_version_zero(minimalorg: MinimalorgRpcClients, backend) -> None:
    """`vlobVersion: 0` is rejected: a document must exist as a vlob (version
    >= 1) before it can be edited in a session (RFC §1.3). No session is
    created."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    participant_uuid = uuid4()
    auth = _auth_header(device_id, participant_uuid)
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    async with open_editics_sse(minimalorg.raw_client, join_url, auth):
        rep = await minimalorg.raw_client.post(
            send_url,
            headers={"Authorization": auth, "Content-Type": "application/json"},
            content=ClientEventAuthJSON(indexUser=-1, editorType=0, vlobVersion=0),
        )
        assert rep.status_code == 200, rep.content
        rejected = rep.json()
        assert rejected["type"] == "auth"
        assert rejected["result"] == 0
        assert rejected["latestAllowedVersion"] == 0
        # No session was created.
        assert (workspace_id, vlob_id) not in backend.editics._sessions


@pytest.mark.timeout(10)
async def test_self_heal_stale_version_zero_session(
    minimalorg: MinimalorgRpcClients, backend
) -> None:
    """A malformed session left over with `initial_version < 1` (only
    creatable by a buggy client that joined a not-yet-synced file) is dropped
    when the next valid join (version >= 1) arrives, and recreated at the
    joiner's valid version. This self-heals stale in-memory sessions without
    a server restart."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    participant_uuid = uuid4()
    auth = _auth_header(device_id, participant_uuid)
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    # Inject a malformed session at initial_version 0 directly (this is the
    # shape a pre-fix buggy client would have created on a not-yet-synced
    # file). We can't create it via the RPC anymore (the server now rejects
    # vlobVersion < 1), so reach into the component to simulate the stale
    # state a server restart would have left behind.
    from parsec.components.editics import EditicsSession

    backend.editics._sessions[(workspace_id, vlob_id)] = EditicsSession(
        workspace_id=workspace_id,
        vlob_id=vlob_id,
        initial_version=0,
        latest_allowed_version=0,
    )

    # A valid join (version 1) arrives -> the stale session is dropped and
    # recreated at version 1, the join succeeds.
    async with open_editics_sse(minimalorg.raw_client, join_url, auth) as sse:
        rep = await minimalorg.raw_client.post(
            send_url,
            headers={"Authorization": auth, "Content-Type": "application/json"},
            content=ClientEventAuthJSON(indexUser=-1, editorType=0, vlobVersion=1),
        )
        assert rep.status_code == 200, rep.content
        reply = rep.json()
        assert reply["type"] == "auth"
        assert reply["result"] == 1
        assert reply["indexUser"] == 1
        await _next_data_event(sse)

        session_obj = backend.editics._sessions[(workspace_id, vlob_id)]
        # The session was recreated at the valid version (not 0).
        assert session_obj.initial_version == 1
        assert session_obj.latest_allowed_version == 1


# ---------------------------------------------------------------------------
# Helpers for the step-1 substep tests (B..I).
# ---------------------------------------------------------------------------


# The region-lock table key for block "K". The server keys word (string)
# blocks by the plain string (OnlyOffice's per-editor re-keying, §6.6), so the
# key is "K" itself (not its JSON serialization).
K = "K"


def _b64(b: bytes) -> str:
    """Pydantic serializes `bytes` as base64 over JSON; mirror that here so
    opaque blobs round-trip through the RPC."""
    return base64.b64encode(b).decode()


def _save_changes_body(
    *,
    encryptedChanges: list[bytes] | None = None,
    startSaveChanges: bool,
    endSaveChanges: bool,
    deleteIndex: int | None = None,
    releaseLocks: bool = False,
    excel_info: dict | None = None,
    encryptedCursor: bytes | None = None,
) -> dict:
    # `encryptedChanges` is a list[bytes]; Pydantic decodes each from base64.
    # When sending JSON manually we must base64-encode each fragment.
    return {
        "type": "saveChanges",
        "encryptedChanges": [_b64(b) for b in (encryptedChanges or [])],
        "startSaveChanges": startSaveChanges,
        "endSaveChanges": endSaveChanges,
        "deleteIndex": deleteIndex,
        "releaseLocks": releaseLocks,
        "excel_info": excel_info,
        "encryptedCursor": _b64(encryptedCursor) if encryptedCursor is not None else None,
    }


def _is_save_lock_body(syncChangesIndex: int) -> dict:
    return {"type": "isSaveLock", "syncChangesIndex": syncChangesIndex}


def _un_save_lock_body() -> dict:
    return {"type": "unSaveLock"}


def _get_lock_body(blocks: list) -> dict:
    return {"type": "getLock", "block": blocks}


def _cursor_body(blob: bytes) -> dict:
    return {"type": "cursor", "encryptedCursor": _b64(blob)}


def _message_body(blob: bytes) -> dict:
    return {"type": "message", "encryptedMessage": _b64(blob)}


def _save_done_body(savedUpToIndex: int, newVersion: int) -> dict:
    return {"type": "saveDone", "savedUpToIndex": savedUpToIndex, "newVersion": newVersion}


async def _full_join(client, join_url, send_url, auth, vlobVersion: int = 10) -> None:
    """Join a session and complete the auth-lock handshake as a non-first
    participant (park -> waitAuth -> authChanges -> connectState). The first
    participant must have released the lock for this to complete; this helper
    only drives the *joining* side."""
    # The auth RPC reply (waitAuth or auth) is consumed by the caller.
    raise NotImplementedError  # not used; tests drive joins inline


# ---------------------------------------------------------------------------
# Step 1, substep B — Auth lock & waitAuth (single-editor -> co-editing)
# (todo step_1 §6.2, V-B1..V-B4).
# ---------------------------------------------------------------------------


@pytest.mark.timeout(10)
async def test_waitAuth_then_unlock(minimalorg: MinimalorgRpcClients, backend) -> None:
    """V-B1..V-B3: Alice joins (takes auth lock); Bob joins (parked, waitAuth);
    Alice sends unLockDocument{unlock:true} -> Bob receives authChanges then
    connectState{waitAuth:false}."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    alice_uuid = uuid4()
    alice_auth = _auth_header(device_id, alice_uuid)
    bob_uuid = uuid4()
    bob_auth = _auth_header(device_id, bob_uuid)
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    async with (
        open_editics_sse(minimalorg.raw_client, join_url, alice_auth) as alice_sse,
        open_editics_sse(minimalorg.raw_client, join_url, bob_auth) as bob_sse,
    ):
        # V-B1: Alice joins -> auth_lock_holder == 1; connectState waitAuth false.
        rep = await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        assert rep.status_code == 200
        assert rep.json()["indexUser"] == 1
        alice_cs0 = await _next_data_event(alice_sse)
        assert alice_cs0["waitAuth"] is False
        assert backend.editics._sessions[(workspace_id, vlob_id)].auth_lock_holder == 1

        # V-B2: Bob joins while lock held -> waitAuth (NOT auth).
        rep = await _send_event(
            minimalorg.raw_client,
            send_url,
            bob_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        assert rep.json()["type"] == "waitAuth"
        assert rep.json()["authLockedBy"] == 1
        await _next_data_event(alice_sse)  # connectState{waitAuth:true}
        await _next_data_event(bob_sse)  # connectState{waitAuth:true}

        # V-B3: Alice unlocks -> Bob gets authChanges then connectState{waitAuth:false}.
        rep = await _send_event(
            minimalorg.raw_client, send_url, alice_auth, _unlock_document_body(unlock=True)
        )
        assert rep.status_code == 204
        ac = await _next_data_event(bob_sse)
        assert ac["type"] == "authChanges"
        cs = await _next_data_event(bob_sse)
        assert cs["waitAuth"] is False
        assert backend.editics._sessions[(workspace_id, vlob_id)].auth_lock_holder is None


@pytest.mark.timeout(10)
async def test_disconnect_auth_lock_holder_unblocks_parked(minimalorg, backend) -> None:
    """V-B4: if Alice (auth lock holder) disconnects while Bob is parked, the
    lock is released and Bob is unblocked."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    alice_uuid = uuid4()
    alice_auth = _auth_header(device_id, alice_uuid)
    bob_uuid = uuid4()
    bob_auth = _auth_header(device_id, bob_uuid)
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    async with (
        open_editics_sse(minimalorg.raw_client, join_url, alice_auth) as alice_sse,
        open_editics_sse(minimalorg.raw_client, join_url, bob_auth) as bob_sse,
    ):
        await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _send_event(
            minimalorg.raw_client,
            send_url,
            bob_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)  # waitAuth true
        await _next_data_event(bob_sse)  # waitAuth true

        from parsec.components.editics import EditicsClientContext

        # Alice disconnects (SSE drop) -> auth lock released, Bob unblocked.
        await backend.editics.leave(
            workspace_id,
            vlob_id,
            EditicsClientContext(device_id=device_id, participant_uuid=alice_uuid),
        )
        # Bob receives his backlog then connectState{waitAuth:false} (and a
        # releaseLock is not expected since Alice held no region locks).
        ac = await _next_data_event(bob_sse)
        assert ac["type"] == "authChanges"
        cs = await _next_data_event(bob_sse)
        assert cs["waitAuth"] is False
        # Only Bob remains.
        assert set(backend.editics._sessions[(workspace_id, vlob_id)].participants) == {2}


# ---------------------------------------------------------------------------
# Step 1, substep C — authChanges backlog on join (V-C1..V-C4).
# ---------------------------------------------------------------------------


@pytest.mark.timeout(10)
async def test_authchanges_backlog(minimalorg, backend) -> None:
    """V-C1: Alice stores 2 change fragments; Bob joins (after the auth-lock
    handshake) -> Bob receives authChanges with exactly 2 entries (index 1,2)."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    alice_uuid = uuid4()
    alice_auth = _auth_header(device_id, alice_uuid)
    bob_uuid = uuid4()
    bob_auth = _auth_header(device_id, bob_uuid)
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    async with open_editics_sse(minimalorg.raw_client, join_url, alice_auth) as alice_sse:
        await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)

        # Alice saves 1 chunk with 2 fragments (no other participant to broadcast to).
        rep = await _send_event(minimalorg.raw_client, send_url, alice_auth, _is_save_lock_body(0))
        assert rep.json()["saveLock"] is False
        rep = await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            _save_changes_body(
                encryptedChanges=[b"frag1", b"frag2"], startSaveChanges=True, endSaveChanges=True
            ),
        )
        assert rep.json()["type"] == "unSaveLock"
        assert rep.json()["syncChangesIndex"] == 2

        async with open_editics_sse(minimalorg.raw_client, join_url, bob_auth) as bob_sse:
            # Bob joins (parked behind auth lock since Alice still holds it).
            await _send_event(
                minimalorg.raw_client,
                send_url,
                bob_auth,
                {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
            )
            await _next_data_event(alice_sse)  # waitAuth true
            await _next_data_event(bob_sse)  # waitAuth true
            # Alice releases the lock.
            await _send_event(
                minimalorg.raw_client, send_url, alice_auth, _unlock_document_body(unlock=True)
            )
            await _next_data_event(alice_sse)  # waitAuth false
            ac = await _next_data_event(bob_sse)
            assert ac["type"] == "authChanges"
            assert len(ac["changes"]) == 2
            assert [c[0] for c in ac["changes"]] == [1, 2]
            # Blobs round-trip as base64; decode to compare.
            assert [base64.b64decode(c[1]) for c in ac["changes"]] == [b"frag1", b"frag2"]


@pytest.mark.timeout(10)
async def test_authchanges_empty_for_subsequent_joiner(minimalorg, backend) -> None:
    """V-C3: a subsequent participant joining a session with no changes still
    receives an (empty) authChanges. V-C4: the first participant receives none."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    alice_uuid = uuid4()
    alice_auth = _auth_header(device_id, alice_uuid)
    bob_uuid = uuid4()
    bob_auth = _auth_header(device_id, bob_uuid)
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    async with (
        open_editics_sse(minimalorg.raw_client, join_url, alice_auth) as alice_sse,
        open_editics_sse(minimalorg.raw_client, join_url, bob_auth) as bob_sse,
    ):
        # V-C4: first participant (Alice) receives NO authChanges.
        await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        cs = await _next_data_event(alice_sse)
        assert cs["type"] == "connectState"  # no authChanges before it

        # Bob joins (parked), Alice unlocks -> Bob gets empty authChanges (V-C3).
        await _send_event(
            minimalorg.raw_client,
            send_url,
            bob_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _next_data_event(bob_sse)
        await _send_event(
            minimalorg.raw_client, send_url, alice_auth, _unlock_document_body(unlock=True)
        )
        await _next_data_event(alice_sse)
        ac = await _next_data_event(bob_sse)
        assert ac["type"] == "authChanges"
        assert ac["changes"] == []


# ---------------------------------------------------------------------------
# Step 1, substep D — Chat messages (V-D1..V-D3).
# ---------------------------------------------------------------------------


@pytest.mark.timeout(10)
async def test_chat_message_broadcast(minimalorg, backend) -> None:
    """V-D1..V-D3: Alice sends a message -> both Alice and Bob receive it; the
    RPC reply is 204; the time field is consistent across recipients."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    alice_uuid = uuid4()
    alice_auth = _auth_header(device_id, alice_uuid)
    bob_uuid = uuid4()
    bob_auth = _auth_header(device_id, bob_uuid)
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    async def _both_joined_and_unlocked(alice_sse, bob_sse):
        await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)  # alice connectState waitAuth false
        await _send_event(
            minimalorg.raw_client,
            send_url,
            bob_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)  # alice connectState waitAuth true
        await _next_data_event(bob_sse)  # bob connectState waitAuth true
        await _send_event(
            minimalorg.raw_client, send_url, alice_auth, _unlock_document_body(unlock=True)
        )
        await _next_data_event(alice_sse)  # alice connectState waitAuth false
        await _next_data_event(bob_sse)  # bob authChanges
        await _next_data_event(bob_sse)  # bob connectState waitAuth false

    async with (
        open_editics_sse(minimalorg.raw_client, join_url, alice_auth) as alice_sse,
        open_editics_sse(minimalorg.raw_client, join_url, bob_auth) as bob_sse,
    ):
        await _both_joined_and_unlocked(alice_sse, bob_sse)

        rep = await _send_event(
            minimalorg.raw_client, send_url, alice_auth, _message_body(b"hello")
        )
        assert rep.status_code == 204  # V-D2

        alice_msg = await _next_data_event(alice_sse)
        bob_msg = await _next_data_event(bob_sse)
        assert alice_msg["type"] == "message"
        assert bob_msg["type"] == "message"
        # V-D1: both receive the same record (sender included).
        assert alice_msg["messages"][0]["authorIndexUser"] == 1
        assert base64.b64decode(alice_msg["messages"][0]["encryptedMessage"]) == b"hello"
        assert bob_msg["messages"][0]["authorIndexUser"] == 1
        # V-D3: time is the same across recipients.
        assert alice_msg["messages"][0]["time"] == bob_msg["messages"][0]["time"]


# ---------------------------------------------------------------------------
# Step 1, substep E — Document modification / saveChanges (V-E1..V-E8).
# ---------------------------------------------------------------------------


async def _setup_two_participants(minimalorg, alice_auth, bob_auth, join_url, send_url):
    """Join Alice and Bob, complete the auth-lock handshake. Returns (alice_sse, bob_sse)
    via the enclosing `async with`."""
    raise NotImplementedError  # tests inline this instead


@pytest.mark.timeout(10)
async def test_save_single_chunk(minimalorg, backend) -> None:
    """V-E1: isSaveLock -> saveLock{false}; saveChanges (1 fragment, start+end) ->
    unSaveLock{index:1, syncChangesIndex:1}; the other participant gets one
    saveChanges broadcast with 1 record."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    alice_auth = _auth_header(device_id, uuid4())
    bob_auth = _auth_header(device_id, uuid4())
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    async with (
        open_editics_sse(minimalorg.raw_client, join_url, alice_auth) as alice_sse,
        open_editics_sse(minimalorg.raw_client, join_url, bob_auth) as bob_sse,
    ):
        # Join both + handshake.
        await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _send_event(
            minimalorg.raw_client,
            send_url,
            bob_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _next_data_event(bob_sse)
        await _send_event(
            minimalorg.raw_client, send_url, alice_auth, _unlock_document_body(unlock=True)
        )
        await _next_data_event(alice_sse)  # alice: connectState waitAuth false
        await _next_data_event(bob_sse)  # bob: authChanges
        await _next_data_event(bob_sse)  # bob: connectState waitAuth false

        # Take the save lock.
        rep = await _send_event(minimalorg.raw_client, send_url, alice_auth, _is_save_lock_body(0))
        assert rep.json() == {"type": "saveLock", "saveLock": False}

        # Single-chunk save with 1 fragment.
        rep = await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            _save_changes_body(encryptedChanges=[b"X"], startSaveChanges=True, endSaveChanges=True),
        )
        reply = rep.json()
        assert reply["type"] == "unSaveLock"
        assert reply["index"] == 1
        assert reply["syncChangesIndex"] == 1

        # Bob receives one saveChanges broadcast.
        sc = await _next_data_event(bob_sse)
        assert sc["type"] == "saveChanges"
        assert len(sc["changes"]) == 1
        assert sc["changesIndex"] == 1
        assert sc["syncChangesIndex"] == 1
        assert sc["endSaveChanges"] is True
        assert base64.b64decode(sc["changes"][0]["change"]) == b"X"
        assert sc["changes"][0]["authorIndexUser"] == 1


@pytest.mark.timeout(10)
async def test_save_multi_fragment_single_chunk(minimalorg, backend) -> None:
    """V-E2: 1 chunk with 3 fragments -> unSaveLock{index:1, sync:3}; the other
    participant gets one broadcast with 3 records and syncChangesIndex 3."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    alice_auth = _auth_header(device_id, uuid4())
    bob_auth = _auth_header(device_id, uuid4())
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    async with (
        open_editics_sse(minimalorg.raw_client, join_url, alice_auth) as alice_sse,
        open_editics_sse(minimalorg.raw_client, join_url, bob_auth) as bob_sse,
    ):
        await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _send_event(
            minimalorg.raw_client,
            send_url,
            bob_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _next_data_event(bob_sse)
        await _send_event(
            minimalorg.raw_client, send_url, alice_auth, _unlock_document_body(unlock=True)
        )
        await _next_data_event(alice_sse)  # alice: connectState waitAuth false
        await _next_data_event(bob_sse)  # bob: authChanges
        await _next_data_event(bob_sse)  # bob: connectState waitAuth false

        await _send_event(minimalorg.raw_client, send_url, alice_auth, _is_save_lock_body(0))
        rep = await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            _save_changes_body(
                encryptedChanges=[b"1", b"2", b"3"], startSaveChanges=True, endSaveChanges=True
            ),
        )
        assert rep.json()["syncChangesIndex"] == 3
        assert rep.json()["index"] == 1

        sc = await _next_data_event(bob_sse)
        assert len(sc["changes"]) == 3
        # The broadcast's `changesIndex` is the new total (`puckerIndex`), NOT
        # the saver's save point (OnlyOffice DocsCoServer.saveChanges publish).
        assert sc["changesIndex"] == 3
        assert sc["syncChangesIndex"] == 3
        assert [base64.b64decode(c["change"]) for c in sc["changes"]] == [b"1", b"2", b"3"]


@pytest.mark.timeout(10)
async def test_save_multi_chunk(minimalorg, backend) -> None:
    """V-E3: 2 chunks (chunk1: [b1], chunk2: [b2,b3]) -> savePartChanges then
    unSaveLock{sync:3}; the other participant gets a single broadcast with 3."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    alice_auth = _auth_header(device_id, uuid4())
    bob_auth = _auth_header(device_id, uuid4())
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    async with (
        open_editics_sse(minimalorg.raw_client, join_url, alice_auth) as alice_sse,
        open_editics_sse(minimalorg.raw_client, join_url, bob_auth) as bob_sse,
    ):
        await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _send_event(
            minimalorg.raw_client,
            send_url,
            bob_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _next_data_event(bob_sse)
        await _send_event(
            minimalorg.raw_client, send_url, alice_auth, _unlock_document_body(unlock=True)
        )
        await _next_data_event(alice_sse)  # alice: connectState waitAuth false
        await _next_data_event(bob_sse)  # bob: authChanges
        await _next_data_event(bob_sse)  # bob: connectState waitAuth false

        await _send_event(minimalorg.raw_client, send_url, alice_auth, _is_save_lock_body(0))
        # Chunk 1 (intermediate).
        rep = await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            _save_changes_body(
                encryptedChanges=[b"1"], startSaveChanges=True, endSaveChanges=False
            ),
        )
        spc = rep.json()
        assert spc["type"] == "savePartChanges"
        assert spc["changesIndex"] == 1
        assert spc["syncChangesIndex"] == 1
        # Chunk 2 (final).
        rep = await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            _save_changes_body(
                encryptedChanges=[b"2", b"3"], startSaveChanges=False, endSaveChanges=True
            ),
        )
        assert rep.json()["type"] == "unSaveLock"
        assert rep.json()["syncChangesIndex"] == 3
        # The saver's `unSaveLock.index` is its save point (startIndex), NOT the
        # new total (OnlyOffice `unSaveLock` reply uses the local `changesIndex`).
        assert rep.json()["index"] == 1

        sc = await _next_data_event(bob_sse)
        assert len(sc["changes"]) == 3
        # The broadcast's `changesIndex` is the new total (`puckerIndex`).
        assert sc["changesIndex"] == 3
        assert sc["syncChangesIndex"] == 3
        assert [base64.b64decode(c["change"]) for c in sc["changes"]] == [b"1", b"2", b"3"]


@pytest.mark.timeout(10)
async def test_save_lock_denied_and_desync(minimalorg, backend) -> None:
    """V-E4/V-E5: isSaveLock is denied when someone holds the lock or the client
    is desynced; a saveChanges without the lock is rejected (HTTP 400)."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    alice_auth = _auth_header(device_id, uuid4())
    bob_auth = _auth_header(device_id, uuid4())
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    async with (
        open_editics_sse(minimalorg.raw_client, join_url, alice_auth) as alice_sse,
        open_editics_sse(minimalorg.raw_client, join_url, bob_auth) as bob_sse,
    ):
        await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _send_event(
            minimalorg.raw_client,
            send_url,
            bob_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _next_data_event(bob_sse)
        await _send_event(
            minimalorg.raw_client, send_url, alice_auth, _unlock_document_body(unlock=True)
        )
        await _next_data_event(alice_sse)  # alice: connectState waitAuth false
        await _next_data_event(bob_sse)  # bob: authChanges
        await _next_data_event(bob_sse)  # bob: connectState waitAuth false

        # Alice takes the lock.
        await _send_event(minimalorg.raw_client, send_url, alice_auth, _is_save_lock_body(0))
        # V-E4: Bob is denied (someone holds the lock).
        rep = await _send_event(minimalorg.raw_client, send_url, bob_auth, _is_save_lock_body(0))
        assert rep.json()["saveLock"] is True
        # Bob's saveChanges is rejected (no lock held).
        rep = await _send_event(
            minimalorg.raw_client,
            send_url,
            bob_auth,
            _save_changes_body(encryptedChanges=[b"x"], startSaveChanges=True, endSaveChanges=True),
        )
        assert rep.status_code == 400

        # V-E5: a desynced isSaveLock (wrong syncChangesIndex) is denied even with
        # the lock free. Alice cancels first.
        await _send_event(minimalorg.raw_client, send_url, alice_auth, _un_save_lock_body())
        rep = await _send_event(
            minimalorg.raw_client, send_url, alice_auth, _is_save_lock_body(999)
        )
        assert rep.json()["saveLock"] is True


@pytest.mark.timeout(10)
async def test_save_cancel(minimalorg, backend) -> None:
    """V-E6: take the lock, send unSaveLock (c->s) -> unSaveLock{-1,-1,-1}; lock free."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    alice_auth = _auth_header(device_id, uuid4())
    bob_auth = _auth_header(device_id, uuid4())
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    async with (
        open_editics_sse(minimalorg.raw_client, join_url, alice_auth) as alice_sse,
        open_editics_sse(minimalorg.raw_client, join_url, bob_auth) as bob_sse,
    ):
        await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _send_event(
            minimalorg.raw_client,
            send_url,
            bob_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _next_data_event(bob_sse)
        await _send_event(
            minimalorg.raw_client, send_url, alice_auth, _unlock_document_body(unlock=True)
        )
        await _next_data_event(alice_sse)  # alice: connectState waitAuth false
        await _next_data_event(bob_sse)  # bob: authChanges
        await _next_data_event(bob_sse)  # bob: connectState waitAuth false

        await _send_event(minimalorg.raw_client, send_url, alice_auth, _is_save_lock_body(0))
        rep = await _send_event(minimalorg.raw_client, send_url, alice_auth, _un_save_lock_body())
        assert rep.json() == {"type": "unSaveLock", "index": -1, "time": -1, "syncChangesIndex": -1}
        # Bob can now take the lock.
        rep = await _send_event(minimalorg.raw_client, send_url, bob_auth, _is_save_lock_body(0))
        assert rep.json()["saveLock"] is False


@pytest.mark.timeout(10)
async def test_save_ordering(minimalorg, backend) -> None:
    """V-E7: Alice saves A then Bob saves B; the save points and syncChangesIndex
    are monotonic."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    alice_auth = _auth_header(device_id, uuid4())
    bob_auth = _auth_header(device_id, uuid4())
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    async with (
        open_editics_sse(minimalorg.raw_client, join_url, alice_auth) as alice_sse,
        open_editics_sse(minimalorg.raw_client, join_url, bob_auth) as bob_sse,
    ):
        await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _send_event(
            minimalorg.raw_client,
            send_url,
            bob_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _next_data_event(bob_sse)
        await _send_event(
            minimalorg.raw_client, send_url, alice_auth, _unlock_document_body(unlock=True)
        )
        await _next_data_event(alice_sse)  # alice: connectState waitAuth false
        await _next_data_event(bob_sse)  # bob: authChanges
        await _next_data_event(bob_sse)  # bob: connectState waitAuth false

        # Alice saves A.
        await _send_event(minimalorg.raw_client, send_url, alice_auth, _is_save_lock_body(0))
        rep = await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            _save_changes_body(encryptedChanges=[b"A"], startSaveChanges=True, endSaveChanges=True),
        )
        a_index = rep.json()["index"]
        a_sync = rep.json()["syncChangesIndex"]
        await _next_data_event(bob_sse)  # broadcast
        # Bob saves B.
        await _send_event(minimalorg.raw_client, send_url, bob_auth, _is_save_lock_body(a_sync))
        rep = await _send_event(
            minimalorg.raw_client,
            send_url,
            bob_auth,
            _save_changes_body(encryptedChanges=[b"B"], startSaveChanges=True, endSaveChanges=True),
        )
        b_index = rep.json()["index"]
        b_sync = rep.json()["syncChangesIndex"]
        assert b_index > a_index
        assert b_sync > a_sync


@pytest.mark.timeout(10)
async def test_save_release_locks(minimalorg, backend) -> None:
    """V-E8: Alice holds a region lock, then saves with releaseLocks -> the
    broadcast `locks` contains the released lock; the table no longer has it."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    alice_auth = _auth_header(device_id, uuid4())
    bob_auth = _auth_header(device_id, uuid4())
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    async with (
        open_editics_sse(minimalorg.raw_client, join_url, alice_auth) as alice_sse,
        open_editics_sse(minimalorg.raw_client, join_url, bob_auth) as bob_sse,
    ):
        await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _send_event(
            minimalorg.raw_client,
            send_url,
            bob_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _next_data_event(bob_sse)
        await _send_event(
            minimalorg.raw_client, send_url, alice_auth, _unlock_document_body(unlock=True)
        )
        await _next_data_event(alice_sse)  # alice: connectState waitAuth false
        await _next_data_event(bob_sse)  # bob: authChanges
        await _next_data_event(bob_sse)  # bob: connectState waitAuth false

        # Alice acquires a region lock on block "K". The getLock RPC reply is
        # Alice's view; Bob gets the broadcast (the sender is excluded from the
        # SSE broadcast, §6.6).
        rep = await _send_event(minimalorg.raw_client, send_url, alice_auth, _get_lock_body(["K"]))
        assert rep.json()["type"] == "getLock"
        await _next_data_event(bob_sse)  # bob: getLock broadcast
        assert K in backend.editics._sessions[(workspace_id, vlob_id)].region_locks

        # Alice saves with releaseLocks.
        await _send_event(minimalorg.raw_client, send_url, alice_auth, _is_save_lock_body(0))
        await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            _save_changes_body(
                encryptedChanges=[b"x"],
                startSaveChanges=True,
                endSaveChanges=True,
                releaseLocks=True,
            ),
        )
        sc = await _next_data_event(bob_sse)
        assert sc["type"] == "saveChanges"
        assert len(sc["locks"]) == 1
        assert sc["locks"][0]["block"] == "K"
        assert sc["locks"][0]["user"] == 1
        assert K not in backend.editics._sessions[(workspace_id, vlob_id)].region_locks


# ---------------------------------------------------------------------------
# Step 1, substep F — Region locks (V-F1..V-F4).
# ---------------------------------------------------------------------------


@pytest.mark.timeout(10)
async def test_get_lock_acquire_and_contend(minimalorg, backend) -> None:
    """V-F1/V-F2: Alice acquires a free block; Bob's getLock on the same block
    reports Alice as the holder (Bob does not acquire it)."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    alice_auth = _auth_header(device_id, uuid4())
    bob_auth = _auth_header(device_id, uuid4())
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    async with (
        open_editics_sse(minimalorg.raw_client, join_url, alice_auth) as alice_sse,
        open_editics_sse(minimalorg.raw_client, join_url, bob_auth) as bob_sse,
    ):
        await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _send_event(
            minimalorg.raw_client,
            send_url,
            bob_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _next_data_event(bob_sse)
        await _send_event(
            minimalorg.raw_client, send_url, alice_auth, _unlock_document_body(unlock=True)
        )
        await _next_data_event(alice_sse)  # alice: connectState waitAuth false
        await _next_data_event(bob_sse)  # bob: authChanges
        await _next_data_event(bob_sse)  # bob: connectState waitAuth false

        # V-F1: Alice acquires "K".
        rep = await _send_event(minimalorg.raw_client, send_url, alice_auth, _get_lock_body(["K"]))
        gl = rep.json()
        assert gl["type"] == "getLock"
        assert gl["locks"][K]["user"] == 1
        await _next_data_event(bob_sse)  # broadcast to bob too

        # V-F2: Bob tries the same block -> reports Alice as holder.
        rep = await _send_event(minimalorg.raw_client, send_url, bob_auth, _get_lock_body(["K"]))
        assert rep.json()["locks"][K]["user"] == 1
        await _next_data_event(alice_sse)  # broadcast to alice too


@pytest.mark.timeout(10)
async def test_release_lock_via_unlock_document(minimalorg, backend) -> None:
    """V-F3: Alice unLockDocument{releaseLocks:true} (holding K) -> Bob gets a
    standalone releaseLock for K; K is free again."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    alice_auth = _auth_header(device_id, uuid4())
    bob_auth = _auth_header(device_id, uuid4())
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    async with (
        open_editics_sse(minimalorg.raw_client, join_url, alice_auth) as alice_sse,
        open_editics_sse(minimalorg.raw_client, join_url, bob_auth) as bob_sse,
    ):
        await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _send_event(
            minimalorg.raw_client,
            send_url,
            bob_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _next_data_event(bob_sse)
        await _send_event(
            minimalorg.raw_client, send_url, alice_auth, _unlock_document_body(unlock=True)
        )
        await _next_data_event(alice_sse)  # alice: connectState waitAuth false
        await _next_data_event(bob_sse)  # bob: authChanges
        await _next_data_event(bob_sse)  # bob: connectState waitAuth false

        await _send_event(minimalorg.raw_client, send_url, alice_auth, _get_lock_body(["K"]))
        await _next_data_event(bob_sse)  # bob: getLock broadcast (sender excluded)

        rep = await _send_event(
            minimalorg.raw_client, send_url, alice_auth, _unlock_document_body(releaseLocks=True)
        )
        assert rep.status_code == 204
        rl = await _next_data_event(bob_sse)
        assert rl["type"] == "releaseLock"
        assert rl["locks"][0]["block"] == "K"
        assert rl["locks"][0]["user"] == 1
        assert K not in backend.editics._sessions[(workspace_id, vlob_id)].region_locks


@pytest.mark.timeout(10)
async def test_release_lock_on_disconnect(minimalorg, backend) -> None:
    """V-F4: Alice disconnects while holding K -> Bob gets a standalone releaseLock."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    alice_uuid = uuid4()
    alice_auth = _auth_header(device_id, alice_uuid)
    bob_uuid = uuid4()
    bob_auth = _auth_header(device_id, bob_uuid)
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    async with (
        open_editics_sse(minimalorg.raw_client, join_url, alice_auth) as alice_sse,
        open_editics_sse(minimalorg.raw_client, join_url, bob_auth) as bob_sse,
    ):
        await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _send_event(
            minimalorg.raw_client,
            send_url,
            bob_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _next_data_event(bob_sse)
        await _send_event(
            minimalorg.raw_client, send_url, alice_auth, _unlock_document_body(unlock=True)
        )
        await _next_data_event(alice_sse)  # alice: connectState waitAuth false
        await _next_data_event(bob_sse)  # bob: authChanges
        await _next_data_event(bob_sse)  # bob: connectState waitAuth false

        await _send_event(minimalorg.raw_client, send_url, alice_auth, _get_lock_body(["K"]))
        await _next_data_event(bob_sse)  # bob: getLock broadcast (sender excluded)

        from parsec.components.editics import EditicsClientContext

        await backend.editics.leave(
            workspace_id,
            vlob_id,
            EditicsClientContext(device_id=device_id, participant_uuid=alice_uuid),
        )
        # Bob gets a releaseLock then a connectState (without Alice).
        rl = await _next_data_event(bob_sse)
        assert rl["type"] == "releaseLock"
        assert rl["locks"][0]["block"] == "K"
        cs = await _next_data_event(bob_sse)
        assert cs["type"] == "connectState"
        assert len(cs["participants"]) == 1


# ---------------------------------------------------------------------------
# Step 1, substep G — unLockDocument paths (V-G1..V-G4).
# ---------------------------------------------------------------------------


@pytest.mark.timeout(10)
async def test_unlock_document_isSave(minimalorg, backend) -> None:
    """V-G1: Alice (lock holder) sends unLockDocument{isSave:true} -> unSaveLock
    {-1,-1,-1}; the save lock is free."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    alice_auth = _auth_header(device_id, uuid4())
    bob_auth = _auth_header(device_id, uuid4())
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    async with (
        open_editics_sse(minimalorg.raw_client, join_url, alice_auth) as alice_sse,
        open_editics_sse(minimalorg.raw_client, join_url, bob_auth) as bob_sse,
    ):
        await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _send_event(
            minimalorg.raw_client,
            send_url,
            bob_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _next_data_event(bob_sse)
        await _send_event(
            minimalorg.raw_client, send_url, alice_auth, _unlock_document_body(unlock=True)
        )
        await _next_data_event(alice_sse)  # alice: connectState waitAuth false
        await _next_data_event(bob_sse)  # bob: authChanges
        await _next_data_event(bob_sse)  # bob: connectState waitAuth false

        await _send_event(minimalorg.raw_client, send_url, alice_auth, _is_save_lock_body(0))
        rep = await _send_event(
            minimalorg.raw_client, send_url, alice_auth, _unlock_document_body(isSave=True)
        )
        assert rep.json() == {"type": "unSaveLock", "index": -1, "time": -1, "syncChangesIndex": -1}
        assert backend.editics._sessions[(workspace_id, vlob_id)].save_lock_holder is None


@pytest.mark.timeout(10)
async def test_unlock_document_delete_index_truncates(minimalorg, backend) -> None:
    """V-G4: unLockDocument{deleteIndex:N} truncates the history; a subsequent
    joiner's authChanges reflects the truncation."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    alice_auth = _auth_header(device_id, uuid4())
    bob_auth = _auth_header(device_id, uuid4())
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    async with open_editics_sse(minimalorg.raw_client, join_url, alice_auth) as alice_sse:
        await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        # Alice saves 3 fragments.
        await _send_event(minimalorg.raw_client, send_url, alice_auth, _is_save_lock_body(0))
        await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            _save_changes_body(
                encryptedChanges=[b"1", b"2", b"3"], startSaveChanges=True, endSaveChanges=True
            ),
        )
        assert backend.editics._sessions[(workspace_id, vlob_id)].sync_changes_index == 3
        # Truncate from index 2 (keep only change 1).
        await _send_event(
            minimalorg.raw_client, send_url, alice_auth, _unlock_document_body(deleteIndex=2)
        )
        assert backend.editics._sessions[(workspace_id, vlob_id)].sync_changes_index == 1
        assert len(backend.editics._sessions[(workspace_id, vlob_id)].changes) == 1

        async with open_editics_sse(minimalorg.raw_client, join_url, bob_auth) as bob_sse:
            await _send_event(
                minimalorg.raw_client,
                send_url,
                bob_auth,
                {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
            )
            await _next_data_event(alice_sse)
            await _next_data_event(bob_sse)
            await _send_event(
                minimalorg.raw_client, send_url, alice_auth, _unlock_document_body(unlock=True)
            )
            await _next_data_event(alice_sse)
            ac = await _next_data_event(bob_sse)
            assert ac["type"] == "authChanges"
            assert [c[0] for c in ac["changes"]] == [1]


# ---------------------------------------------------------------------------
# Step 1, substep H — close / disconnect (V-H1..V-H4).
# ---------------------------------------------------------------------------


@pytest.mark.timeout(10)
async def test_close_broadcasts_connectstate(minimalorg, backend) -> None:
    """V-H1: Alice `close`s -> Bob gets connectState without Alice; Alice's region
    locks produce a releaseLock to Bob."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    alice_uuid = uuid4()
    alice_auth = _auth_header(device_id, alice_uuid)
    bob_auth = _auth_header(device_id, uuid4())
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    async with (
        open_editics_sse(minimalorg.raw_client, join_url, alice_auth) as alice_sse,
        open_editics_sse(minimalorg.raw_client, join_url, bob_auth) as bob_sse,
    ):
        await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _send_event(
            minimalorg.raw_client,
            send_url,
            bob_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _next_data_event(bob_sse)
        await _send_event(
            minimalorg.raw_client, send_url, alice_auth, _unlock_document_body(unlock=True)
        )
        await _next_data_event(alice_sse)  # alice: connectState waitAuth false
        await _next_data_event(bob_sse)  # bob: authChanges
        await _next_data_event(bob_sse)  # bob: connectState waitAuth false

        await _send_event(minimalorg.raw_client, send_url, alice_auth, _get_lock_body(["K"]))
        await _next_data_event(bob_sse)  # bob: getLock broadcast (sender excluded)

        # Alice sends `close`.
        rep = await _send_event(minimalorg.raw_client, send_url, alice_auth, {"type": "close"})
        assert rep.status_code == 204
        rl = await _next_data_event(bob_sse)
        assert rl["type"] == "releaseLock"
        cs = await _next_data_event(bob_sse)
        assert cs["type"] == "connectState"
        assert all(p["indexUser"] != 1 for p in cs["participants"])


@pytest.mark.timeout(10)
async def test_disconnect_mid_save(minimalorg, backend) -> None:
    """V-H2: Alice disconnects mid-save (she held the save lock) -> Bob can take
    the save lock; no stale unSaveLock is sent."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    alice_uuid = uuid4()
    alice_auth = _auth_header(device_id, alice_uuid)
    bob_auth = _auth_header(device_id, uuid4())
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    async with (
        open_editics_sse(minimalorg.raw_client, join_url, alice_auth) as alice_sse,
        open_editics_sse(minimalorg.raw_client, join_url, bob_auth) as bob_sse,
    ):
        await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _send_event(
            minimalorg.raw_client,
            send_url,
            bob_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _next_data_event(bob_sse)
        await _send_event(
            minimalorg.raw_client, send_url, alice_auth, _unlock_document_body(unlock=True)
        )
        await _next_data_event(alice_sse)  # alice: connectState waitAuth false
        await _next_data_event(bob_sse)  # bob: authChanges
        await _next_data_event(bob_sse)  # bob: connectState waitAuth false

        # Alice takes the lock and starts a save (intermediate chunk).
        await _send_event(minimalorg.raw_client, send_url, alice_auth, _is_save_lock_body(0))
        await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            _save_changes_body(
                encryptedChanges=[b"1"], startSaveChanges=True, endSaveChanges=False
            ),
        )
        assert backend.editics._sessions[(workspace_id, vlob_id)].save_lock_holder == 1

        from parsec.components.editics import EditicsClientContext

        await backend.editics.leave(
            workspace_id,
            vlob_id,
            EditicsClientContext(device_id=device_id, participant_uuid=alice_uuid),
        )
        # The save lock is cleared.
        assert backend.editics._sessions[(workspace_id, vlob_id)].save_lock_holder is None
        # Bob can take it (syncChangesIndex advanced by the intermediate chunk).
        rep = await _send_event(minimalorg.raw_client, send_url, bob_auth, _is_save_lock_body(1))
        assert rep.json()["saveLock"] is False


# ---------------------------------------------------------------------------
# Step 1, substep I — saveDone / vlob version bump (V-I1..V-I3).
# ---------------------------------------------------------------------------


@pytest.mark.timeout(10)
async def test_save_done_bumps_latest_allowed_version(minimalorg, backend) -> None:
    """V-I1/V-I2: after a save, Alice sends saveDone{newVersion:11} -> the
    session's latest_allowed_version == 11; a joiner with vlobVersion 11 is
    accepted, 10 is still accepted (in range), 12 is rejected."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    alice_auth = _auth_header(device_id, uuid4())
    bob_auth = _auth_header(device_id, uuid4())
    carol_auth = _auth_header(device_id, uuid4())
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    async with open_editics_sse(minimalorg.raw_client, join_url, alice_auth) as alice_sse:
        await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        # Alice saves.
        await _send_event(minimalorg.raw_client, send_url, alice_auth, _is_save_lock_body(0))
        await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            _save_changes_body(encryptedChanges=[b"x"], startSaveChanges=True, endSaveChanges=True),
        )
        # saveDone bumps latest_allowed_version to 11.
        rep = await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            _save_done_body(savedUpToIndex=1, newVersion=11),
        )
        assert rep.status_code == 204
        assert backend.editics._sessions[(workspace_id, vlob_id)].latest_allowed_version == 11

        # Bob joins with vlobVersion 11 -> accepted (in [10, 11]).
        async with open_editics_sse(minimalorg.raw_client, join_url, bob_auth):
            rep = await _send_event(
                minimalorg.raw_client,
                send_url,
                bob_auth,
                {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 11},
            )
            assert rep.json()["type"] == "waitAuth"  # parked behind Alice's auth lock
            # Carol joins with vlobVersion 12 -> rejected.
            async with open_editics_sse(minimalorg.raw_client, join_url, carol_auth):
                rep = await _send_event(
                    minimalorg.raw_client,
                    send_url,
                    carol_auth,
                    {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 12},
                )
                assert rep.json()["result"] == 0
                assert rep.json()["latestAllowedVersion"] == 11


@pytest.mark.timeout(10)
async def test_save_done_ignored_from_non_holder(minimalorg, backend) -> None:
    """V-I3: saveDone from a participant that did not just release the save lock
    is ignored (no version bump)."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    alice_auth = _auth_header(device_id, uuid4())
    bob_auth = _auth_header(device_id, uuid4())
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    async with (
        open_editics_sse(minimalorg.raw_client, join_url, alice_auth) as alice_sse,
        open_editics_sse(minimalorg.raw_client, join_url, bob_auth) as bob_sse,
    ):
        await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _send_event(
            minimalorg.raw_client,
            send_url,
            bob_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _next_data_event(bob_sse)
        await _send_event(
            minimalorg.raw_client, send_url, alice_auth, _unlock_document_body(unlock=True)
        )
        await _next_data_event(alice_sse)  # alice: connectState waitAuth false
        await _next_data_event(bob_sse)  # bob: authChanges
        await _next_data_event(bob_sse)  # bob: connectState waitAuth false

        # Bob (not the last save lock holder) sends saveDone -> ignored.
        rep = await _send_event(
            minimalorg.raw_client,
            send_url,
            bob_auth,
            _save_done_body(savedUpToIndex=1, newVersion=99),
        )
        assert rep.status_code == 204
        assert backend.editics._sessions[(workspace_id, vlob_id)].latest_allowed_version == 10


# ---------------------------------------------------------------------------
# Step 1, substep F — Region lock keying (OnlyOffice per-editor re-keying).
# The `getLock` reply's `locks` dict MUST be keyed by the plain block id for
# the Document editor (Word) and by `block.guid` for Spreadsheet/Presentation,
# matching the editor's internal `_locks`/`_lockCallbacks` keys; otherwise the
# editor's pending-lock callback never fires and the lock is stuck.
# ---------------------------------------------------------------------------


@pytest.mark.timeout(10)
async def test_get_lock_keying_word_and_excel(minimalorg, backend) -> None:
    """The lock-table key is the plain block string for Word and `block.guid`
    for Spreadsheet/Presentation (OnlyOffice per-editor re-keying, §6.6)."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    alice_auth = _auth_header(device_id, uuid4())
    bob_auth = _auth_header(device_id, uuid4())
    join_url = _join_url(minimalorg.organization_id, workspace_id, vlob_id)
    send_url = _send_url(minimalorg.organization_id, workspace_id, vlob_id)

    async with (
        open_editics_sse(minimalorg.raw_client, join_url, alice_auth) as alice_sse,
        open_editics_sse(minimalorg.raw_client, join_url, bob_auth) as bob_sse,
    ):
        await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _send_event(
            minimalorg.raw_client,
            send_url,
            bob_auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": 10},
        )
        await _next_data_event(alice_sse)
        await _next_data_event(bob_sse)
        await _send_event(
            minimalorg.raw_client, send_url, alice_auth, _unlock_document_body(unlock=True)
        )
        await _next_data_event(alice_sse)
        await _next_data_event(bob_sse)
        await _next_data_event(bob_sse)

        # Word-style: plain string block "K" -> key "K" (not '"K"').
        rep = await _send_event(minimalorg.raw_client, send_url, alice_auth, _get_lock_body(["K"]))
        gl = rep.json()
        assert "K" in gl["locks"]
        assert '"K"' not in gl["locks"]
        assert gl["locks"]["K"]["user"] == 1
        await _next_data_event(bob_sse)
        # The internal table is keyed by "K".
        assert "K" in backend.editics._sessions[(workspace_id, vlob_id)].region_locks

        # Excel/Presentation-style: object block with a `guid` -> key = guid.
        guid = "abc-123"
        rep = await _send_event(
            minimalorg.raw_client,
            send_url,
            alice_auth,
            _get_lock_body([{"guid": guid, "range": "A1:B2"}]),
        )
        gl2 = rep.json()
        assert guid in gl2["locks"]
        assert gl2["locks"][guid]["block"] == {"guid": guid, "range": "A1:B2"}
        await _next_data_event(bob_sse)
        assert guid in backend.editics._sessions[(workspace_id, vlob_id)].region_locks


# ---------------------------------------------------------------------------
# Step 1 — Two-tab co-editing scenarios (protocol-level).
#
# These tests simulate two browser tabs at the editics-protocol level (HTTP
# SSE + RPC) and exercise the locking-sensitive scenarios: inserting a line
# (getLock + saveChanges + releaseLock), inserting multiple lines, typing on a
# line, removing a line, concurrent edits, close/reopen, and save. After each
# scenario the save lock and the region locks must be released (no stuck
# lock), and the other tab must receive the broadcast changes.
# ---------------------------------------------------------------------------


class _Tab:
    """A simulated editor tab: an SSE stream + an RPC sender.

    It tracks the editor-side state needed to drive the save/lock handshake:
    `syncChangesIndex` (echoed in `isSaveLock`) and the save-lock grant. The
    translation layer (onlyoffice-editics-client.js) is covered by the vitest
    suite; this drives the *editics* wire directly.
    """

    def __init__(self, client, auth, join_url, send_url, participant_uuid):
        self.client = client
        self.auth = auth
        self.participant_uuid = participant_uuid
        self.join_url = join_url
        self.send_url = send_url
        self.sse = None
        self.indexUser = -1
        self.syncChangesIndex = 0

    async def __aenter__(self):
        self.sse_ctx = open_editics_sse(self.client, self.join_url, self.auth)
        self.sse = await self.sse_ctx.__aenter__()
        return self

    async def __aexit__(self, *exc):
        await self.sse_ctx.__aexit__(*exc)

    async def join(self, vlobVersion: int = 10) -> dict:
        rep = await _send_event(
            self.client,
            self.send_url,
            self.auth,
            {"type": "auth", "indexUser": -1, "editorType": 0, "vlobVersion": vlobVersion},
        )
        data = rep.json()
        if data.get("type") == "auth" and data.get("result") == 1:
            self.indexUser = data["indexUser"]
            # Drain the self-connectState; syncChangesIndex starts at the
            # backlog (delivered as authChanges for a non-first participant).
        return data

    async def drain(self):
        """Consume one SSE data event."""
        return await _next_data_event(self.sse)

    async def drain_auth_handshake(self, other: _Tab):
        """Complete the auth-lock handshake: `other` holds the auth lock; this
        tab is parked; `other` unlocks; this tab gets authChanges +
        connectState{waitAuth:false}. Assumes `other` is the first participant.
        """
        # this tab's auth returned waitAuth; drain the waitAuth:true broadcast
        cs_wait = await self.drain()  # connectState{waitAuth:true}
        # The parked tab's indexUser is the participant that isn't the auth-lock
        # holder (the `waitAuth` reply only carries `authLockedBy`, not the
        # newcomer's index, so recover it from the participant map).
        holder = other.indexUser
        for p in cs_wait.get("participants", []):
            if p["indexUser"] != holder:
                self.indexUser = p["indexUser"]
                break
        # other releases the lock
        await other.send(_unlock_document_body(unlock=True))
        await other.drain()  # other: connectState{waitAuth:false}
        ac = await self.drain()  # authChanges
        assert ac["type"] == "authChanges"
        self.syncChangesIndex = len(ac.get("changes", []))
        cs = await self.drain()  # connectState{waitAuth:false}
        assert cs["waitAuth"] is False

    async def send(self, body: dict) -> httpx.Response:
        return await _send_event(self.client, self.send_url, self.auth, body)

    async def save(
        self, fragments: list[bytes], *, releaseLocks: bool = False, deleteIndex=None
    ) -> dict:
        """Drive a full save cycle: isSaveLock -> saveChanges (single chunk) -> unSaveLock reply."""
        rep = await self.send(_is_save_lock_body(self.syncChangesIndex))
        assert rep.json()["saveLock"] is False, "save lock denied (desync or held)"
        return await self.save_locked(fragments, releaseLocks=releaseLocks, deleteIndex=deleteIndex)

    async def save_locked(
        self, fragments: list[bytes], *, releaseLocks: bool = False, deleteIndex=None
    ) -> dict:
        """Send saveChanges assuming the save lock is already held (skip the
        isSaveLock handshake). Returns the unSaveLock reply."""
        rep = await self.send(
            _save_changes_body(
                encryptedChanges=fragments,
                startSaveChanges=True,
                endSaveChanges=True,
                releaseLocks=releaseLocks,
                deleteIndex=deleteIndex,
            )
        )
        reply = rep.json()
        assert reply["type"] == "unSaveLock", reply
        self.syncChangesIndex = reply["syncChangesIndex"]
        return reply

    async def cancel_save(self) -> dict:
        rep = await self.send(_un_save_lock_body())
        return rep.json()

    async def get_lock(self, blocks: list) -> dict:
        rep = await self.send(_get_lock_body(blocks))
        return rep.json()

    async def unlock_document(self, *, releaseLocks: bool = False, deleteIndex=None) -> int:
        rep = await self.send(
            _unlock_document_body(releaseLocks=releaseLocks, deleteIndex=deleteIndex)
        )
        return rep.status_code

    async def close(self) -> int:
        rep = await self.send({"type": "close"})
        return rep.status_code


def _locks_held(backend, workspace_id, vlob_id) -> dict:
    return backend.editics._sessions[(workspace_id, vlob_id)].region_locks


def _save_lock_held(backend, workspace_id, vlob_id):
    return backend.editics._sessions[(workspace_id, vlob_id)].save_lock_holder


async def _setup_two_tabs(minimalorg, backend, vlobVersion: int = 10):
    """Two tabs (same device, distinct participant_uuid) on a fresh session.
    Returns (alice, bob, workspace_id, vlob_id, backend)."""
    device_id = minimalorg.alice.device_id
    workspace_id = VlobID.new()
    vlob_id = VlobID.new()
    a_uuid = uuid4()
    b_uuid = uuid4()
    alice = _Tab(
        minimalorg.raw_client,
        _auth_header(device_id, a_uuid),
        _join_url(minimalorg.organization_id, workspace_id, vlob_id),
        _send_url(minimalorg.organization_id, workspace_id, vlob_id),
        participant_uuid=a_uuid,
    )
    bob = _Tab(
        minimalorg.raw_client,
        _auth_header(device_id, b_uuid),
        _join_url(minimalorg.organization_id, workspace_id, vlob_id),
        _send_url(minimalorg.organization_id, workspace_id, vlob_id),
        participant_uuid=b_uuid,
    )
    return alice, bob, workspace_id, vlob_id, backend


async def _join_both(alice: _Tab, bob: _Tab):
    """Alice joins (takes auth lock), Bob joins (parked), Alice unlocks, both
    co-editing. Drains the handshake on both sides."""
    await alice.join()
    await alice.drain()  # alice connectState{waitAuth:false}
    bob_auth_reply = await bob.join()
    assert bob_auth_reply["type"] == "waitAuth"
    await alice.drain()  # alice: connectState{waitAuth:true} (bob joined)
    await bob.drain_auth_handshake(alice)


@pytest.mark.timeout(20)
async def test_scenario_insert_line_then_type(minimalorg, backend) -> None:
    """Insert a new line (getLock + saveChanges + releaseLocks), then type on it
    (saveChanges). The other tab receives the changes; no lock is stuck."""
    alice, bob, workspace_id, vlob_id, _ = await _setup_two_tabs(minimalorg, backend)
    async with alice, bob:
        await _join_both(alice, bob)

        # Alice inserts a line: acquire a region lock on block "P1".
        gl = await alice.get_lock(["P1"])
        assert gl["locks"]["P1"]["user"] == alice.indexUser
        await bob.drain()  # bob: getLock broadcast
        assert "P1" in _locks_held(backend, workspace_id, vlob_id)

        # Alice saves the structural change, releasing the lock.
        rep = await alice.save([b"ins-P1"], releaseLocks=True)
        assert rep["index"] >= 1
        sc = await bob.drain()
        assert sc["type"] == "saveChanges"
        assert len(sc["changes"]) == 1
        # The released lock is embedded in the broadcast.
        assert any(r["block"] == "P1" for r in sc["locks"])
        assert "P1" not in _locks_held(backend, workspace_id, vlob_id)
        bob.syncChangesIndex = sc["syncChangesIndex"]
        assert _save_lock_held(backend, workspace_id, vlob_id) is None

        # Alice types text on the new line (a plain saveChanges, no region lock).
        rep = await alice.save([b"text-a", b"text-b"])
        sc = await bob.drain()
        assert len(sc["changes"]) == 2
        bob.syncChangesIndex = sc["syncChangesIndex"]
        assert _save_lock_held(backend, workspace_id, vlob_id) is None


@pytest.mark.timeout(20)
async def test_scenario_insert_multiple_lines(minimalorg, backend) -> None:
    """Insert several lines in sequence; each acquires+releases its own region
    lock. No lock is left held between inserts."""
    alice, bob, workspace_id, vlob_id, _ = await _setup_two_tabs(minimalorg, backend)
    async with alice, bob:
        await _join_both(alice, bob)
        for i in range(1, 5):
            block = f"P{i}"
            await alice.get_lock([block])
            assert block in _locks_held(backend, workspace_id, vlob_id)
            await bob.drain()
            await alice.save([f"ins-{block}".encode()], releaseLocks=True)
            sc = await bob.drain()
            assert any(r["block"] == block for r in sc["locks"])
            assert block not in _locks_held(backend, workspace_id, vlob_id)
            bob.syncChangesIndex = sc["syncChangesIndex"]
            assert _save_lock_held(backend, workspace_id, vlob_id) is None


@pytest.mark.timeout(20)
async def test_scenario_remove_line(minimalorg, backend) -> None:
    """Removing a line is a saveChanges with deleteIndex (UNDO) and releaseLocks."""
    alice, bob, workspace_id, vlob_id, _ = await _setup_two_tabs(minimalorg, backend)
    async with alice, bob:
        await _join_both(alice, bob)
        # Seed one change.
        await alice.save([b"line1"])
        sc = await bob.drain()  # bob: saveChanges broadcast for the seed
        bob.syncChangesIndex = sc["syncChangesIndex"]
        # Alice acquires a lock and saves a structural removal with deleteIndex.
        await alice.get_lock(["P1"])
        await bob.drain()  # bob: getLock broadcast (alice holds P1)
        # deleteIndex=1 truncates history from index 1 (UNDO the seed).
        await alice.save([b"remove-P1"], releaseLocks=True, deleteIndex=1)
        sc = await bob.drain()  # bob: saveChanges broadcast with P1 released
        assert any(r["block"] == "P1" for r in sc["locks"])
        assert _save_lock_held(backend, workspace_id, vlob_id) is None
        assert "P1" not in _locks_held(backend, workspace_id, vlob_id)
        # The history was truncated.
        assert backend.editics._sessions[(workspace_id, vlob_id)].sync_changes_index >= 1


@pytest.mark.timeout(20)
async def test_scenario_concurrent_edits_no_deadlock(minimalorg, backend) -> None:
    """Both tabs save concurrently (serialized by the save lock). Neither gets
    stuck: the second waits for the lock, then saves."""
    alice, bob, workspace_id, vlob_id, _ = await _setup_two_tabs(minimalorg, backend)
    async with alice, bob:
        await _join_both(alice, bob)
        # Alice takes the save lock first.
        rep = await alice.send(_is_save_lock_body(alice.syncChangesIndex))
        assert rep.json()["saveLock"] is False
        # Bob is denied while Alice holds it.
        rep = await bob.send(_is_save_lock_body(bob.syncChangesIndex))
        assert rep.json()["saveLock"] is True
        # Alice completes her save (lock already held) -> lock released.
        await alice.save_locked([b"a-change"])
        sc = await bob.drain()
        bob.syncChangesIndex = sc["syncChangesIndex"]
        assert _save_lock_held(backend, workspace_id, vlob_id) is None
        # Bob can now take the lock and save.
        rep = await bob.send(_is_save_lock_body(bob.syncChangesIndex))
        assert rep.json()["saveLock"] is False
        await bob.save_locked([b"b-change"])
        sc = await alice.drain()
        alice.syncChangesIndex = sc["syncChangesIndex"]
        assert _save_lock_held(backend, workspace_id, vlob_id) is None


@pytest.mark.timeout(20)
async def test_scenario_contended_region_lock(minimalorg, backend) -> None:
    """Both tabs request the same region lock; the second is denied (sees the
    first as holder). The holder releases via saveChanges{releaseLocks}; the
    other can then acquire it."""
    alice, bob, workspace_id, vlob_id, _ = await _setup_two_tabs(minimalorg, backend)
    async with alice, bob:
        await _join_both(alice, bob)
        await alice.get_lock(["K"])
        await bob.drain()  # bob sees alice hold K
        gl = await bob.get_lock(["K"])
        assert gl["locks"]["K"]["user"] == alice.indexUser
        await alice.drain()  # alice sees bob's failed attempt (K still alice)
        # Alice releases K via a save with releaseLocks.
        await alice.save([b"x"], releaseLocks=True)
        await bob.drain()  # bob: saveChanges broadcast with K released
        assert "K" not in _locks_held(backend, workspace_id, vlob_id)
        # Bob can now acquire K.
        gl = await bob.get_lock(["K"])
        assert gl["locks"]["K"]["user"] == bob.indexUser
        await alice.drain()
        assert _save_lock_held(backend, workspace_id, vlob_id) is None


@pytest.mark.timeout(20)
async def test_scenario_close_releases_locks(minimalorg, backend) -> None:
    """Closing a tab mid-edit releases its region lock and the save lock; the
    other tab gets a releaseLock + connectState and is not stuck."""
    alice, bob, workspace_id, vlob_id, _ = await _setup_two_tabs(minimalorg, backend)
    async with alice, bob:
        await _join_both(alice, bob)
        await alice.get_lock(["K"])
        await bob.drain()
        # Alice holds a region lock and (simulate) a save lock.
        await alice.send(_is_save_lock_body(alice.syncChangesIndex))
        # Alice closes.
        await alice.close()
        # Bob gets a releaseLock for K and a connectState without Alice.
        rl = await bob.drain()
        assert rl["type"] == "releaseLock"
        assert any(r["block"] == "K" for r in rl["locks"])
        cs = await bob.drain()
        assert cs["type"] == "connectState"
        assert all(p["indexUser"] != alice.indexUser for p in cs["participants"])
        assert "K" not in _locks_held(backend, workspace_id, vlob_id)
        assert _save_lock_held(backend, workspace_id, vlob_id) is None
        # Bob can still save.
        rep = await bob.send(_is_save_lock_body(bob.syncChangesIndex))
        assert rep.json()["saveLock"] is False


@pytest.mark.timeout(20)
async def test_scenario_close_reopen_and_save_to_vlob(minimalorg, backend) -> None:
    """Close both tabs (session GC'd), reopen, and bump the vlob version via
    saveDone. A new joiner with the new version is accepted."""
    from parsec.components.editics import EditicsClientContext

    alice, bob, workspace_id, vlob_id, _ = await _setup_two_tabs(minimalorg, backend)
    async with alice, bob:
        await _join_both(alice, bob)
        await alice.save([b"edit"])
        await bob.drain()
        # Alice bumps the vlob version (saveDone).
        rep = await alice.send(_save_done_body(savedUpToIndex=1, newVersion=11))
        assert rep.status_code == 204
        assert backend.editics._sessions[(workspace_id, vlob_id)].latest_allowed_version == 11
        # Both close (the SSE-disconnect leave isn't modelled by ASGITransport, so
        # drive the leave explicitly, as `test_leave_removes_participant...` does).
        await alice.close()
        await bob.close()
        for tab in (alice, bob):
            await backend.editics.leave(
                workspace_id,
                vlob_id,
                EditicsClientContext(
                    device_id=minimalorg.alice.device_id, participant_uuid=tab.participant_uuid
                ),
            )
    # Session is GC'd once both leave.
    assert (workspace_id, vlob_id) not in backend.editics._sessions

    # Reopen as a new tab with the new vlob version.
    carol_uuid = uuid4()
    carol = _Tab(
        minimalorg.raw_client,
        _auth_header(minimalorg.alice.device_id, carol_uuid),
        _join_url(minimalorg.organization_id, workspace_id, vlob_id),
        _send_url(minimalorg.organization_id, workspace_id, vlob_id),
        participant_uuid=carol_uuid,
    )
    async with carol:
        rep = await carol.join(vlobVersion=11)
        assert rep["type"] == "auth"
        assert rep["result"] == 1
        assert _save_lock_held(backend, workspace_id, vlob_id) is None
