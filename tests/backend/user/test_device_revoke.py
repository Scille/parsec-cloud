import pytest
import pendulum

from parsec.trustchain import certify_device_revocation
from parsec.backend.user import INVITATION_VALIDITY
from parsec.api.protocole import device_revoke_serializer, HandshakeRevokedDevice

from tests.common import freeze_time


@pytest.fixture
def bob_revocation(alice, bob):
    now = pendulum.now()
    return certify_device_revocation(alice.device_id, alice.signing_key, bob.device_id, now=now)


async def device_revoke(sock, **kwargs):
    await sock.send(device_revoke_serializer.req_dumps({"cmd": "device_revoke", **kwargs}))
    raw_rep = await sock.recv()
    rep = device_revoke_serializer.rep_loads(raw_rep)
    return rep


@pytest.mark.trio
async def test_device_revoke_ok(
    backend, backend_sock_factory, alice_backend_sock, bob, bob_revocation
):
    rep = await device_revoke(alice_backend_sock, certified_revocation=bob_revocation)
    assert rep == {"status": "ok"}

    # Make sure bob cannot connect from now on
    with pytest.raises(HandshakeRevokedDevice):
        async with backend_sock_factory(backend, bob):
            pass


@pytest.mark.trio
async def test_device_revoke_unknown(alice_backend_sock, alice, mallory):
    certified_revocation = certify_device_revocation(
        alice.device_id, alice.signing_key, mallory.device_id, now=pendulum.now()
    )

    rep = await device_revoke(alice_backend_sock, certified_revocation=certified_revocation)
    assert rep == {"status": "not_found"}


@pytest.mark.trio
async def test_device_good_user_bad_device(alice_backend_sock, alice):
    certified_revocation = certify_device_revocation(
        alice.device_id, alice.signing_key, f"{alice.user_id}@foo", now=pendulum.now()
    )

    rep = await device_revoke(alice_backend_sock, certified_revocation=certified_revocation)
    assert rep == {"status": "not_found"}


@pytest.mark.trio
async def test_device_revoke_already_revoked(alice_backend_sock, bob, bob_revocation):
    rep = await device_revoke(alice_backend_sock, certified_revocation=bob_revocation)
    assert rep == {"status": "ok"}

    rep = await device_revoke(alice_backend_sock, certified_revocation=bob_revocation)
    assert rep == {
        "status": "already_revoked",
        "reason": f"Device `{bob.device_id}` already revoked",
    }


@pytest.mark.trio
async def test_device_revoke_invalid_certified(alice_backend_sock, alice2, bob):
    certified_revocation = certify_device_revocation(
        alice2.device_id, alice2.signing_key, bob.device_id, now=pendulum.now()
    )

    rep = await device_revoke(alice_backend_sock, certified_revocation=certified_revocation)
    assert rep == {
        "status": "invalid_certification",
        "reason": "Certifier is not the authenticated device.",
    }


@pytest.mark.trio
async def test_device_revoke_certify_too_old(alice_backend_sock, alice, bob):
    now = pendulum.Pendulum(2000, 1, 1)
    certified_revocation = certify_device_revocation(
        alice.device_id, alice.signing_key, bob.device_id, now=now
    )

    with freeze_time(now.add(seconds=INVITATION_VALIDITY + 1)):
        rep = await device_revoke(alice_backend_sock, certified_revocation=certified_revocation)
        assert rep == {
            "status": "invalid_certification",
            "reason": "Invalid certification data (Timestamp is too old.).",
        }
