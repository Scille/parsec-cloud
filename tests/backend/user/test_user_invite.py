import pytest

from parsec.trustchain import certify_user_invitation
from parsec.utils import to_jsonb64


@pytest.mark.trio
async def test_user_invite(alice_backend_sock, alice, mallory):
    certified_invitation = certify_user_invitation(
        alice.device_id, alice.device_signkey, mallory.user_id
    )
    await alice_backend_sock.send(
        {"cmd": "user_invite", "certified_invitation": to_jsonb64(certified_invitation)}
    )
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "ok"}


@pytest.mark.trio
async def test_user_invite_bad_certified(alice_backend_sock):
    await alice_backend_sock.send(
        {"cmd": "user_invite", "certified_invitation": to_jsonb64(b"foo")}
    )
    rep = await alice_backend_sock.recv()
    assert rep == {
        "status": "invalid_certification",
        "reason": (
            "Invalid certification data (Message doesn't contain"
            " author metadata along with signed message)."
        ),
    }


@pytest.mark.trio
async def test_user_invite_bad_signature(alice_backend_sock, alice):
    certified = (
        "{"
        f'"device_id": "{alice.device_id}", '
        '"timestamp": "2000-01-01T00:00:00Z", '
        '"content": ""'
        "}"
    ).encode()
    await alice_backend_sock.send(
        {"cmd": "user_invite", "certified_invitation": to_jsonb64(certified)}
    )
    rep = await alice_backend_sock.recv()
    assert rep == {
        "status": "invalid_certification",
        "reason": "Invalid certification data (Signature was forged or corrupt).",
    }


@pytest.mark.trio
async def test_user_invite_bad_certifier(alice_backend_sock, bob, mallory):
    certified_invitation = certify_user_invitation(
        bob.device_id, bob.device_signkey, mallory.user_id
    )
    await alice_backend_sock.send(
        {"cmd": "user_invite", "certified_invitation": to_jsonb64(certified_invitation)}
    )
    rep = await alice_backend_sock.recv()
    assert rep == {
        "status": "invalid_certification",
        "reason": "Certifier is not the authenticated device.",
    }
