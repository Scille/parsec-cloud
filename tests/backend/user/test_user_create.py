import pytest
import pendulum

from parsec.trustchain import certify_user, certify_device
from parsec.backend.user import INVITATION_VALIDITY
from parsec.api.protocole import user_create_serializer

from tests.common import freeze_time


@pytest.mark.trio
async def test_user_create_ok(backend, backend_sock_factory, alice_backend_sock, alice, mallory):
    now = pendulum.now()
    certified_user = certify_user(
        alice.device_id, alice.device_signkey, mallory.user_id, mallory.user_pubkey, now=now
    )
    certified_device = certify_device(
        alice.device_id, alice.device_signkey, mallory.device_id, mallory.device_verifykey, now=now
    )

    with backend.event_bus.listen() as spy:
        await alice_backend_sock.send(
            user_create_serializer.req_dump(
                {
                    "cmd": "user_create",
                    "certified_user": certified_user,
                    "certified_device": certified_device,
                }
            )
        )
        raw_rep = await alice_backend_sock.recv()

    spy.assert_event_occured("user.created", kwargs={"user_id": mallory.user_id})
    rep = user_create_serializer.rep_load(raw_rep)
    assert rep == {"status": "ok"}

    # Make sure mallory can connect now
    async with backend_sock_factory(backend, mallory) as sock:
        await sock.send({"cmd": "ping", "ping": "Hello world !"})
        rep = await sock.recv()
        assert rep == {"status": "ok", "pong": "Hello world !"}


@pytest.mark.trio
async def test_user_create_invalid_certified(alice_backend_sock, alice, bob, mallory):
    now = pendulum.now()
    good_certified_user = certify_user(
        alice.device_id, alice.device_signkey, mallory.user_id, mallory.user_pubkey, now=now
    )
    good_certified_device = certify_device(
        alice.device_id, alice.device_signkey, mallory.device_id, mallory.device_verifykey, now=now
    )
    bad_certified_user = certify_user(
        bob.device_id, bob.device_signkey, mallory.user_id, mallory.user_pubkey, now=now
    )
    bad_certified_device = certify_device(
        bob.device_id, bob.device_signkey, mallory.device_id, mallory.device_verifykey, now=now
    )

    for cu, cd in [
        (good_certified_user, bad_certified_device),
        (bad_certified_user, good_certified_device),
        (bad_certified_user, bad_certified_device),
    ]:
        await alice_backend_sock.send(
            user_create_serializer.req_dump(
                {"cmd": "user_create", "certified_user": cu, "certified_device": cd}
            )
        )
        raw_rep = await alice_backend_sock.recv()

        rep = user_create_serializer.rep_load(raw_rep)
        assert rep == {
            "status": "invalid_certification",
            "reason": "Certifier is not the authenticated device.",
        }


@pytest.mark.trio
async def test_user_create_not_matching_user_device(alice_backend_sock, alice, mallory):
    now = pendulum.now()
    certified_user = certify_user(
        alice.device_id, alice.device_signkey, mallory.user_id, mallory.user_pubkey, now=now
    )
    certified_device = certify_device(
        alice.device_id, alice.device_signkey, "zack@foo", mallory.device_verifykey, now=now
    )

    await alice_backend_sock.send(
        user_create_serializer.req_dump(
            {
                "cmd": "user_create",
                "certified_user": certified_user,
                "certified_device": certified_device,
            }
        )
    )
    raw_rep = await alice_backend_sock.recv()

    rep = user_create_serializer.rep_load(raw_rep)
    assert rep == {
        "status": "invalid_data",
        "reason": "Device and User must have the same user ID.",
    }


@pytest.mark.trio
async def test_user_create_already_exists(alice_backend_sock, alice, bob):
    now = pendulum.now()
    certified_user = certify_user(
        alice.device_id, alice.device_signkey, bob.user_id, bob.user_pubkey, now=now
    )
    certified_device = certify_device(
        alice.device_id, alice.device_signkey, bob.device_id, bob.device_verifykey, now=now
    )

    await alice_backend_sock.send(
        user_create_serializer.req_dump(
            {
                "cmd": "user_create",
                "certified_user": certified_user,
                "certified_device": certified_device,
            }
        )
    )
    raw_rep = await alice_backend_sock.recv()

    rep = user_create_serializer.rep_load(raw_rep)
    assert rep == {"status": "already_exists", "reason": "User `bob` already exists"}


@pytest.mark.trio
async def test_device_create_certify_too_old(alice_backend_sock, alice, mallory):
    too_old = pendulum.Pendulum(2000, 1, 1)
    now = too_old.add(seconds=INVITATION_VALIDITY + 1)
    good_certified_user = certify_user(
        alice.device_id, alice.device_signkey, mallory.user_id, mallory.user_pubkey, now=now
    )
    good_certified_device = certify_device(
        alice.device_id, alice.device_signkey, mallory.device_id, mallory.device_verifykey, now=now
    )
    bad_certified_user = certify_user(
        alice.device_id, alice.device_signkey, mallory.user_id, mallory.user_pubkey, now=too_old
    )
    bad_certified_device = certify_device(
        alice.device_id,
        alice.device_signkey,
        mallory.device_id,
        mallory.device_verifykey,
        now=too_old,
    )

    with freeze_time(now):
        for cu, cd in [
            (good_certified_user, bad_certified_device),
            (bad_certified_user, good_certified_device),
            (bad_certified_user, bad_certified_device),
        ]:
            await alice_backend_sock.send(
                user_create_serializer.req_dump(
                    {"cmd": "user_create", "certified_user": cu, "certified_device": cd}
                )
            )
            raw_rep = await alice_backend_sock.recv()
            rep = user_create_serializer.rep_load(raw_rep)
            assert rep == {
                "status": "invalid_certification",
                "reason": "Invalid certification data (Timestamp is too old.).",
            }
