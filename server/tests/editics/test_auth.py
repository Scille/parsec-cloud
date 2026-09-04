# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
"""Editics client-side auth test (todo step_2 §7.1).

A single test drives two clients connecting to the same editics session one
after the other, end-to-end through the protocol translator (`client/editics/
protocol.js`) running in PyMiniRacer against the real Parsec server (in-process
over httpx `ASGITransport`). It mirrors the first part of the captured
OnlyOffice session (`docs/rfcs/1030-collaborative-editics/oo_example_session.md`):

1. Alice joins a fresh session (John's first `auth`).
2. Bob joins the existing session while Alice holds the auth lock (Kate's
   `auth`, server `waitAuth`, John's `connectState{waitAuth:true}`).
3. Alice releases the auth lock (John's `unLockDocument{unlock:true}`); Bob
   receives `authChanges` then `connectState{waitAuth:false}`.

The assertions compare the OnlyOffice events the translator produces against
the captured session **structurally** (same `type`, same field presence and
nesting — todo §6.4 `assert_oo_shape`), not by exact value, since timestamps,
ids, JWTs and opaque blobs differ. They also assert the **editics** events
round-trip through the real server (the server's `waitAuth` and
`connectState` replies are what the translator consumes), proving the
translator and the server agree on the protocol.
"""

from __future__ import annotations

import asyncio
from pathlib import Path
from typing import Any

import pytest

from parsec._parsec import VlobID
from tests.common import CoolorgRpcClients
from tests.common.editics import (
    assert_oo_shape,
    load_captured_session,
)

# Path to the captured OnlyOffice session used as the structural oracle.
CAPTURED_SESSION = (
    Path(__file__).resolve().parents[3]
    / "docs"
    / "rfcs"
    / "1030-collaborative-editics"
    / "oo_example_session.md"
)


def _captured_for(user: str, etype: str, direction: str | None = None) -> dict[str, Any]:
    """Return the editor-facing payload of the first captured event matching
    `user` + `type` (and optional direction).

    The captured markdown blocks are the full OnlyOffice wire event
    `{type, payload}`; the editor consumes the inner `payload` (the shape the
    translator produces). Return that inner `payload`.
    """
    for ev in load_captured_session(CAPTURED_SESSION):
        if (
            ev.user == user
            and ev.type == etype
            and (direction is None or ev.direction == direction)
        ):
            return ev.payload.get("payload", ev.payload)
    raise AssertionError(f"no captured event for {user!r} {etype!r} {direction!r}")


@pytest.mark.timeout(60)
async def test_two_clients_join_one_after_the_other(
    coolorg: CoolorgRpcClients,
    fake_editics_client_factory,
) -> None:
    realm_id = VlobID.new()
    doc_id = VlobID.new()
    vlob_version = 10

    # Resolve display names locally (the server is NOT trusted for names, RFC
    # §3.3). Map the test device id hex to the captured-session persona so the
    # structural comparison against the captured participants lines up.
    alice_device_hex = coolorg.alice.device_id.hex
    bob_device_hex = coolorg.bob.device_id.hex
    user_names = {
        alice_device_hex: "John Smith",
        bob_device_hex: "Kate Cage",
    }

    alice = await fake_editics_client_factory(
        coolorg.alice,
        realm_id,
        doc_id,
        vlob_version=vlob_version,
        user_names=user_names,
    )
    bob = await fake_editics_client_factory(
        coolorg.bob,
        realm_id,
        doc_id,
        vlob_version=vlob_version,
        user_names=user_names,
    )

    # Per-client buffers of OO server events, filled by background drainers so
    # the server's SSE broadcasts never backpressure while the test drives RPCs.
    alice_events: list[dict[str, Any]] = []
    bob_events: list[dict[str, Any]] = []

    async def drain(client, sink):
        try:
            async for oo in client.oo_server_events():
                sink.append(oo)
        except asyncio.CancelledError:
            raise

    alice_drain = asyncio.create_task(drain(alice, alice_events))
    bob_drain = asyncio.create_task(drain(bob, bob_events))

    def alice_cs(predicate):
        for oo in alice_events:
            if oo.get("type") == "connectState" and predicate(oo):
                return oo
        return None

    def bob_event(etype, predicate=lambda _oo: True):
        for oo in bob_events:
            if oo.get("type") == etype and predicate(oo):
                return oo
        return None

    try:
        # --- 1. Alice joins a fresh session --------------------------------

        # The OO `auth` client event, taken from John's captured `auth` (c->s):
        # the translator must strip everything except {type, indexUser, editorType,
        # vlobVersion} (RFC §2.2 editics changes).
        john_auth_c2s = _captured_for("John Smith", "auth", "->")
        cooked_auth = await alice.inject_oo_client_event(john_auth_c2s)
        assert cooked_auth == {
            "type": "auth",
            "indexUser": -1,
            "editorType": 0,
            "vlobVersion": vlob_version,
        }, cooked_auth

        # The server's `auth` RPC reply is a success (the translator turns it
        # into an OO `connectState`, not an OO `auth`).
        from parsec.components.editics import ServerEventAuth

        assert isinstance(alice._last_reply, ServerEventAuth)
        assert alice._last_reply.result == 1
        assert alice._last_reply.indexUser == 1

        # Alice's translator produces a `connectState` (from the SSE broadcast)
        # with one participant, waitAuth false.
        cs = await _wait_for(lambda: alice_cs(lambda oo: len(oo["participants"]) == 1), 3)
        assert cs is not None, "Alice received no single-participant connectState"
        assert cs["waitAuth"] is False
        assert len(cs["participants"]) == 1
        p = cs["participants"][0]
        assert p["indexUser"] == 1
        assert p["username"] == "John Smith"

        # Structural check: the participant entry has the OnlyOffice participant
        # fields the editor needs (the editics translator intentionally drops
        # `connectionId`/`isCloseCoAuthoring`/`isLiveViewer`/`encrypted` per RFC
        # §2.2 editics changes).
        john_auth_s2c = _captured_for("John Smith", "auth", "<-")
        captured_p = john_auth_s2c["participants"][0]
        for key in ("id", "idOriginal", "username", "indexUser", "view"):
            assert key in p, f"participant missing {key!r}"
            assert key in captured_p, f"captured participant missing {key!r}"
        assert_oo_shape(p["id"], captured_p["id"])
        assert_oo_shape(p["idOriginal"], captured_p["idOriginal"])
        assert_oo_shape(p["username"], captured_p["username"])

        # --- 2. Bob joins the existing session (auth lock held by Alice) ----

        kate_auth_c2s = _captured_for("Kate Cage", "auth", "->")
        bob_cooked_auth = await bob.inject_oo_client_event(kate_auth_c2s)
        assert bob_cooked_auth == {
            "type": "auth",
            "indexUser": -1,
            "editorType": 0,
            "vlobVersion": vlob_version,
        }

        # The server parks Bob: RPC reply is `waitAuth` (editics); the translator
        # turns it into the OO `waitAuth` event.
        from parsec.components.editics import ServerEventWaitAuth

        assert isinstance(bob._last_reply, ServerEventWaitAuth)
        assert bob._last_reply.authLockedBy == 1

        # Bob's translator produces an OO `waitAuth` whose `lockDocument` is
        # rebuilt from the participant table. Compare its shape to Kate's
        # captured `waitAuth`.
        bob_wait_auth = await bob.inject_editics_server_event(
            bob._last_reply.model_dump(mode="python")
        )
        assert bob_wait_auth is not None
        assert bob_wait_auth["type"] == "waitAuth"
        assert_oo_shape(bob_wait_auth, _captured_for("Kate Cage", "waitAuth", "<-"))
        assert bob_wait_auth["lockDocument"]["indexUser"] == 1

        # Alice should now see a `connectState` with both participants and
        # `waitAuth: true` (the nudge to release the lock).
        alice_wait_cs = await _wait_for(
            lambda: alice_cs(lambda oo: len(oo["participants"]) == 2 and oo["waitAuth"] is True),
            5,
        )
        assert alice_wait_cs is not None, (
            "Alice never got connectState{waitAuth:true, 2 participants}"
        )
        assert_oo_shape(alice_wait_cs, _captured_for("John Smith", "connectState", "<-"))

        # --- 3. Alice releases the auth lock -------------------------------

        # Feed John's captured `unLockDocument{unlock:true}` from Alice. The
        # translator forwards it as-is; the server unblocks Bob (sends
        # `authChanges` over SSE) then broadcasts `connectState{waitAuth:false}`.
        john_unlock = _captured_for("John Smith", "unLockDocument", "->")
        await alice.inject_oo_client_event(john_unlock)

        # Bob receives `authChanges` (empty backlog in this fresh session) then
        # a `connectState` with `waitAuth: false`.
        bob_auth_changes = await _wait_for(lambda: bob_event("authChanges"), 8)
        assert bob_auth_changes is not None, (
            "Bob received no authChanges after Alice released the lock"
        )
        assert_oo_shape(bob_auth_changes, _captured_for("Kate Cage", "authChanges", "<-"))
        # The captured backlog has John's two changes; in this test the session
        # is fresh (no changes), so the translator's `authChanges` is empty —
        # the shape (a `changes` list) is what we assert.
        assert bob_auth_changes["changes"] == []

        bob_cs_false = await _wait_for(
            lambda: bob_event("connectState", lambda oo: oo["waitAuth"] is False),
            8,
        )
        assert bob_cs_false is not None, "Bob never got connectState{waitAuth:false}"
        assert bob_cs_false["waitAuth"] is False
        assert len(bob_cs_false["participants"]) == 2
    finally:
        alice_drain.cancel()
        bob_drain.cancel()
        for t in (alice_drain, bob_drain):
            try:
                await t
            except (asyncio.CancelledError,):
                pass


async def _wait_for(predicate, deadline_s: float):
    """Poll `predicate()` until it returns truthy or `deadline_s` seconds elapse."""
    deadline = asyncio.get_event_loop().time() + deadline_s
    while asyncio.get_event_loop().time() < deadline:
        result = predicate()
        if result:
            return result
        await asyncio.sleep(0.05)
    return predicate()
