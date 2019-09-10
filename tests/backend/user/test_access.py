# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from pendulum import Pendulum

from parsec.api.protocol import packb, user_get_serializer, user_find_serializer

from tests.common import freeze_time


async def user_get(sock, user_id):
    await sock.send(user_get_serializer.req_dumps({"cmd": "user_get", "user_id": user_id}))
    raw_rep = await sock.recv()
    return user_get_serializer.rep_loads(raw_rep)


async def user_find(sock, **kwargs):
    await sock.send(user_find_serializer.req_dumps({"cmd": "user_find", **kwargs}))
    raw_rep = await sock.recv()
    return user_find_serializer.rep_loads(raw_rep)


@pytest.fixture
async def access_testbed(
    backend_factory,
    backend_data_binder_factory,
    backend_sock_factory,
    organization_factory,
    local_device_factory,
):
    async with backend_factory(populated=False) as backend:
        binder = backend_data_binder_factory(backend)
        org = organization_factory("IFD")
        device = local_device_factory("Godfrey@dev1", org)
        with freeze_time("2000-01-01"):
            await binder.bind_organization(org, device)

        async with backend_sock_factory(backend, device) as sock:
            yield binder, org, device, sock


@pytest.mark.trio
async def test_api_user_get_ok(access_testbed):
    binder, org, device, sock = access_testbed

    rep = await user_get(sock, device.user_id)
    assert rep == {
        "status": "ok",
        "user_certificate": binder.certificates_store.get_user(device),
        "revoked_user_certificate": None,
        "device_certificates": [binder.certificates_store.get_device(device)],
        "trustchain": {"devices": [], "revoked_users": [], "users": []},
    }


@pytest.mark.trio
async def test_api_user_get_ok_deep_trustchain(
    access_testbed, organization_factory, local_device_factory
):
    binder, org, godfrey1, sock = access_testbed
    certificates_store = binder.certificates_store
    d1 = Pendulum(2000, 1, 1)
    d2 = Pendulum(2000, 1, 2)

    roger1 = local_device_factory("roger@dev1", org)
    mike1 = local_device_factory("mike@dev1", org)
    mike2 = local_device_factory("mike@dev2", org)
    ph1 = local_device_factory("philippe@dev1", org)
    ph2 = local_device_factory("philippe@dev2", org)

    # <root> --> godfrey@dev1 --> roger@dev1 --> mike@dev1 --> mike@dev2
    #                         --> philippe@dev1 --> philippe@dev2
    with freeze_time(d1):
        await binder.bind_device(roger1, certifier=godfrey1)
        await binder.bind_device(mike1, certifier=roger1)
        await binder.bind_device(ph1, certifier=godfrey1)
        await binder.bind_device(ph2, certifier=ph1)
        await binder.bind_device(mike2, certifier=mike1)

    with freeze_time(d2):
        await binder.bind_revocation(roger1.user_id, certifier=ph1)
        await binder.bind_revocation(mike1.user_id, certifier=ph2)

    rep = await user_get(sock, mike2.device_id.user_id)
    cooked_rep = {
        **rep,
        "user_certificate": certificates_store.translate_certif(rep["user_certificate"]),
        "device_certificates": certificates_store.translate_certifs(rep["device_certificates"]),
        "revoked_user_certificate": certificates_store.translate_certif(
            rep["revoked_user_certificate"]
        ),
        "trustchain": {
            "devices": sorted(certificates_store.translate_certifs(rep["trustchain"]["devices"])),
            "users": sorted(certificates_store.translate_certifs(rep["trustchain"]["users"])),
            "revoked_users": sorted(
                certificates_store.translate_certifs(rep["trustchain"]["revoked_users"])
            ),
        },
    }

    assert cooked_rep == {
        "status": "ok",
        "user_certificate": "<mike user certif>",
        "device_certificates": ["<mike@dev1 device certif>", "<mike@dev2 device certif>"],
        "revoked_user_certificate": "<mike revoked user certif>",
        "trustchain": {
            "devices": sorted(
                [
                    "<philippe@dev2 device certif>",
                    "<philippe@dev1 device certif>",
                    "<Godfrey@dev1 device certif>",
                    "<mike@dev1 device certif>",
                    "<roger@dev1 device certif>",
                ]
            ),
            "users": sorted(
                [
                    "<roger user certif>",
                    "<philippe user certif>",
                    "<mike user certif>",
                    "<Godfrey user certif>",
                ]
            ),
            "revoked_users": sorted(["<roger revoked user certif>", "<mike revoked user certif>"]),
        },
    }


@pytest.mark.parametrize("bad_msg", [{"user_id": 42}, {"user_id": None}, {}])
@pytest.mark.trio
async def test_api_user_get_bad_msg(alice_backend_sock, bad_msg):
    await alice_backend_sock.send(packb({"cmd": "user_get", **bad_msg}))
    raw_rep = await alice_backend_sock.recv()
    rep = user_get_serializer.rep_loads(raw_rep)
    assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_api_user_get_not_found(alice_backend_sock, coolorg):
    rep = await user_get(alice_backend_sock, "dummy")
    assert rep == {"status": "not_found"}


@pytest.mark.trio
async def test_api_user_get_other_organization(
    backend, alice, sock_from_other_organization_factory
):
    # Organizations should be isolated
    async with sock_from_other_organization_factory(backend) as sock:
        rep = await user_get(sock, alice.user_id)
        assert rep == {"status": "not_found"}


@pytest.mark.trio
async def test_api_user_find(access_testbed, organization_factory, local_device_factory):
    binder, org, godfrey1, sock = access_testbed

    # Populate with cool guys
    for name in ["Philippe@p1", "Philippe@p2", "Mike@p1", "Blacky@p1", "Philip_J_Fry@p1"]:
        device = local_device_factory(name, org)
        await binder.bind_device(device, certifier=godfrey1)

    await binder.bind_revocation("Philip_J_Fry", certifier=godfrey1)

    # Also create homonyme in different organization, just to be sure...
    other_org = organization_factory("FilmMark")
    other_device = local_device_factory("Philippe@p1", other_org)
    await binder.bind_organization(other_org, other_device)

    # # Test exact match
    # rep = await user_find(sock, query="Mike")
    # assert rep == {"status": "ok", "results": ["Mike"], "per_page": 100, "page": 1, "total": 1}

    # Test partial search
    rep = await user_find(sock, query="Phil")
    assert rep == {
        "status": "ok",
        "results": ["Philip_J_Fry", "Philippe"],
        "per_page": 100,
        "page": 1,
        "total": 2,
    }

    # Test case insensitivity
    rep = await user_find(sock, query="phil")
    assert rep == {
        "status": "ok",
        "results": ["Philip_J_Fry", "Philippe"],
        "per_page": 100,
        "page": 1,
        "total": 2,
    }

    # Test partial search while omitting revoked users
    rep = await user_find(sock, query="Phil", omit_revoked=True)
    assert rep == {"status": "ok", "results": ["Philippe"], "per_page": 100, "page": 1, "total": 1}

    # Test partial search with invalid query
    rep = await user_find(sock, query="p*", omit_revoked=True)
    assert rep == {"status": "ok", "results": [], "per_page": 100, "page": 1, "total": 0}

    # Test pagination
    rep = await user_find(sock, query="Phil", page=1, per_page=1)
    assert rep == {
        "status": "ok",
        "results": ["Philip_J_Fry"],
        "per_page": 1,
        "page": 1,
        "total": 2,
    }

    # Test out of pagination
    rep = await user_find(sock, query="Phil", page=2, per_page=5)
    assert rep == {"status": "ok", "results": [], "per_page": 5, "page": 2, "total": 2}

    # Test no params
    rep = await user_find(sock)
    assert rep == {
        "status": "ok",
        "results": ["Blacky", "Godfrey", "Mike", "Philip_J_Fry", "Philippe"],
        "per_page": 100,
        "page": 1,
        "total": 5,
    }

    # Test omit revoked users
    rep = await user_find(sock, omit_revoked=True)
    assert rep == {
        "status": "ok",
        "results": ["Blacky", "Godfrey", "Mike", "Philippe"],
        "per_page": 100,
        "page": 1,
        "total": 4,
    }

    # Test bad params
    for bad in [{"query": 42}, {"page": 0}, {"per_page": 0}, {"per_page": 101}]:
        await sock.send(packb({"cmd": "user_find", **bad}))
        raw_rep = await sock.recv()
        rep = user_find_serializer.rep_loads(raw_rep)
        assert rep["status"] == "bad_message"
