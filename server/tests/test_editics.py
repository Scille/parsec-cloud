# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
"""Tests for the editics step-0 auth subset.

Covers the happy-path join flow (todo step_0 §6): opening the SSE stream, the
`auth` RPC, the `auth` RPC reply and the `connectState` SSE broadcast, plus
keepalive and the leave-on-disconnect cleanup. The rejection shape (§7) is
exercised too.
"""

from __future__ import annotations

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
        assert auth_reply["participants"] == [{"indexUser": 1, "deviceId": device_id.hex}]

        # The SSE stream should deliver a connectState broadcast.
        connect_state = await _next_data_event(sse)
        assert connect_state["type"] == "connectState"
        assert connect_state["waitAuth"] is False
        assert isinstance(connect_state["participantsTimestamp"], int)
        assert connect_state["participants"] == [{"indexUser": 1, "deviceId": device_id.hex}]


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
        await _next_data_event(alice_sse)  # alice's connectState

        rep = await minimalorg.raw_client.post(
            send_url,
            headers={"Authorization": bob_auth, "Content-Type": "application/json"},
            content=ClientEventAuthJSON(indexUser=-1, editorType=0, vlobVersion=10),
        )
        assert rep.status_code == 200
        await _next_data_event(alice_sse)  # alice sees bob join
        await _next_data_event(bob_sse)  # bob sees the participant set

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
        assert cs["participants"] == [{"indexUser": 2, "deviceId": device_id.hex}]

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
        assert connect_state["participants"] == [{"indexUser": 1, "deviceId": device_id.hex}]


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
