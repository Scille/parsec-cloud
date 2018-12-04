import pytest
import pendulum

from parsec.trustchain import certify_user_revocation
from parsec.handshake import HandshakeRevokedDevice


@pytest.fixture
def bob_revocation(alice, bob):
    now = pendulum.now()
    return certify_user_revocation(alice.device_id, alice.device_signkey, bob.user_id, now=now)


@pytest.mark.trio
async def test_user_revoke_ok(
    backend, backend_sock_factory, alice_backend_sock, bob, bob_revocation
):
    await alice_backend_sock.send({"cmd": "user_revoke", "certified_revocation": bob_revocation})
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "ok"}

    # Make sure bob cannot connect from now on
    with pytest.raises(HandshakeRevokedDevice):
        async with backend_sock_factory(backend, bob):
            pass


@pytest.mark.trio
async def test_user_revoke_unknown(alice_backend_sock, alice):
    certified = certify_user_revocation(
        alice.device_id, alice.device_signkey, "zack", now=pendulum.now()
    )

    await alice_backend_sock.send({"cmd": "user_revoke", "certified_revocation": certified})
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "not_found"}


@pytest.mark.trio
async def test_user_revoke_already_revoked(alice_backend_sock, bob_revocation):
    await alice_backend_sock.send({"cmd": "user_revoke", "certified_revocation": bob_revocation})
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "ok"}

    await alice_backend_sock.send({"cmd": "user_revoke", "certified_revocation": bob_revocation})
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "already_revoked", "reason": "User `bob` already revoked"}
