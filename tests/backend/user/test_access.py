import pytest
import attr
from unittest.mock import ANY
from pendulum import Pendulum

from parsec.types import DeviceID
from parsec.crypto import SigningKey, PrivateKey
from parsec.trustchain import certify_user, certify_device
from parsec.api.protocole import user_get_serializer, user_find_serializer
from parsec.backend.user import (
    User as BackendUser,
    Device as BackendDevice,
    DevicesMapping as BackendDevicesMapping,
)

from tests.common import freeze_time


async def user_get(sock, user_id):
    await sock.send(user_get_serializer.req_dump({"cmd": "user_get", "user_id": user_id}))
    raw_rep = await sock.recv()
    return user_get_serializer.rep_load(raw_rep)


async def user_find(sock, **kwargs):
    await sock.send(user_find_serializer.req_dump({"cmd": "user_find", **kwargs}))
    raw_rep = await sock.recv()
    return user_find_serializer.rep_load(raw_rep)


@pytest.mark.trio
async def test_api_user_get_ok(backend, alice_backend_sock, bob):
    rep = await user_get(alice_backend_sock, bob.user_id)
    assert rep == {
        "status": "ok",
        "user_id": bob.user_id,
        "certified_user": ANY,
        "user_certifier": None,
        "created_on": Pendulum(2000, 1, 1),
        "devices": {
            bob.device_name: {
                "device_id": bob.device_id,
                "created_on": Pendulum(2000, 1, 1),
                "revocated_on": None,
                "certified_revocation": None,
                "revocation_certifier": None,
                "certified_device": ANY,
                "device_certifier": None,
            }
        },
        "trustchain": {},
    }


@attr.s
class Device:
    device_id = attr.ib()
    signing_key = attr.ib()


def user_factory(creator, device_id):
    device_id = DeviceID(device_id)
    private_key = PrivateKey.generate()
    certified_user = certify_user(
        creator.device_id, creator.signing_key, device_id.user_id, private_key.public_key
    )
    local_device, backend_device = device_factory(creator, device_id)
    backend_user = BackendUser(
        device_id.user_id,
        certified_user=certified_user,
        user_certifier=creator.device_id,
        devices=BackendDevicesMapping(backend_device),
    )
    return local_device, backend_user


def device_factory(creator, device_id):
    device_id = DeviceID(device_id)
    signing_key = SigningKey.generate()
    certified_device = certify_device(
        creator.device_id, creator.signing_key, device_id, signing_key.verify_key
    )
    backend_device = BackendDevice(
        device_id=device_id, certified_device=certified_device, device_certifier=creator.device_id
    )
    local_device = Device(device_id=device_id, signing_key=signing_key)
    return local_device, backend_device


async def create_user(backend, creator, device_id):
    local_device, user = user_factory(creator, device_id)
    await backend.user.create_user(user)
    return local_device


async def create_device(backend, creator, device_id):
    local_device, device = device_factory(creator, device_id)
    await backend.user.create_device(device)
    return local_device


@pytest.mark.trio
async def test_api_user_get_ok_deep_trustchain(backend, alice_backend_sock, alice):
    # <root> --> alice@dev1 --> philippe@dev1 --> mike@dev1 --> mike@dev2
    with freeze_time("2000-01-01"):
        ph1 = await create_user(backend, alice, "philippe@dev1")
        mike1 = await create_user(backend, ph1, "mike@dev1")
        mike2 = await create_device(backend, mike1, "mike@dev2")

    rep = await user_get(alice_backend_sock, mike2.device_id.user_id)
    assert rep == {
        "status": "ok",
        "user_id": mike2.device_id.user_id,
        "certified_user": ANY,
        "user_certifier": ph1.device_id,
        "created_on": Pendulum(2000, 1, 1),
        "devices": {
            mike1.device_id.device_name: {
                "device_id": mike1.device_id,
                "created_on": Pendulum(2000, 1, 1),
                "revocated_on": None,
                "certified_revocation": None,
                "revocation_certifier": None,
                "certified_device": ANY,
                "device_certifier": ph1.device_id,
            },
            mike2.device_id.device_name: {
                "device_id": mike2.device_id,
                "created_on": Pendulum(2000, 1, 1),
                "revocated_on": None,
                "certified_revocation": None,
                "revocation_certifier": None,
                "certified_device": ANY,
                "device_certifier": mike1.device_id,
            },
        },
        "trustchain": {
            alice.device_id: {
                "device_id": alice.device_id,
                "created_on": Pendulum(2000, 1, 1),
                "revocated_on": None,
                "certified_revocation": None,
                "revocation_certifier": None,
                "certified_device": ANY,
                "device_certifier": None,
            },
            ph1.device_id: {
                "device_id": ph1.device_id,
                "created_on": Pendulum(2000, 1, 1),
                "revocated_on": None,
                "certified_revocation": None,
                "revocation_certifier": None,
                "certified_device": ANY,
                "device_certifier": alice.device_id,
            },
        },
    }


# TODO: test user_get with revocation filled


@pytest.mark.parametrize(
    "bad_msg", [{"user_id": 42}, {"user_id": None}, {"user_id": "alice", "unknown": "field"}, {}]
)
@pytest.mark.trio
async def test_api_user_get_bad_msg(alice_backend_sock, bad_msg):
    await alice_backend_sock.send({"cmd": "user_get", **bad_msg})
    raw_rep = await alice_backend_sock.recv()
    rep = user_get_serializer.rep_load(raw_rep)
    assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_api_user_get_not_found(alice_backend_sock):
    rep = await user_get(alice_backend_sock, "dummy")
    assert rep == {"status": "not_found"}


@pytest.mark.trio
async def test_api_user_find(alice, backend, alice_backend_sock, root_key_certifier):
    # We won't use those keys anyway
    dpk = alice.public_key
    dvk = alice.verify_key
    # Populate with cool guys
    for cool_guy, devices_names in [
        ("Philippe", ["p1", "p2"]),
        ("Mike", ["m1"]),
        ("Blacky", []),
        ("Philip_J_Fry", ["pe1"]),
    ]:
        user = root_key_certifier.user_factory(
            cool_guy, dpk, devices=[(d, dvk) for d in devices_names]
        )
        await backend.user.create_user(user)

    # Test exact match
    rep = await user_find(alice_backend_sock, query="Mike")
    assert rep == {"status": "ok", "results": ["Mike"], "per_page": 100, "page": 1, "total": 1}

    # Test partial search
    rep = await user_find(alice_backend_sock, query="Phil")
    assert rep == {
        "status": "ok",
        "results": ["Philip_J_Fry", "Philippe"],
        "per_page": 100,
        "page": 1,
        "total": 2,
    }

    # Test pagination
    rep = await user_find(alice_backend_sock, query="Phil", page=1, per_page=1)
    assert rep == {
        "status": "ok",
        "results": ["Philip_J_Fry"],
        "per_page": 1,
        "page": 1,
        "total": 2,
    }

    # Test out of pagination
    rep = await user_find(alice_backend_sock, query="Phil", page=2, per_page=5)
    assert rep == {"status": "ok", "results": [], "per_page": 5, "page": 2, "total": 2}

    # Test no params
    rep = await user_find(alice_backend_sock)
    assert rep == {
        "status": "ok",
        "results": ["alice", "Blacky", "bob", "Mike", "Philip_J_Fry", "Philippe"],
        "per_page": 100,
        "page": 1,
        "total": 6,
    }

    # Test bad params
    for bad in [{"dummy": 42}, {"query": 42}, {"page": 0}, {"per_page": 0}, {"per_page": 101}]:
        await alice_backend_sock.send({"cmd": "user_find", **bad})
        raw_rep = await alice_backend_sock.recv()
        rep = user_find_serializer.rep_load(raw_rep)
        assert rep["status"] == "bad_message"
