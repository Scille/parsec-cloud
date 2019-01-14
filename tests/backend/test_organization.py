import pytest
import pendulum
from unittest.mock import ANY

from parsec.types import UserID
from parsec.trustchain import certify_user, certify_device

from parsec.api.protocole import organization_create_serializer, organization_bootstrap_serializer

from tests.backend.test_events import ping
from tests.fixtures import local_device_to_backend_user


async def organization_create(sock, organization_id):
    raw_rep = await sock.send(
        organization_create_serializer.req_dumps(
            {"cmd": "organization_create", "organization_id": organization_id}
        )
    )
    raw_rep = await sock.recv()
    return organization_create_serializer.rep_loads(raw_rep)


async def organization_bootstrap(
    sock, bootstrap_token, certified_user, certified_device, root_verify_key
):
    raw_rep = await sock.send(
        organization_bootstrap_serializer.req_dumps(
            {
                "cmd": "organization_bootstrap",
                "bootstrap_token": bootstrap_token,
                "certified_user": certified_user,
                "certified_device": certified_device,
                "root_verify_key": root_verify_key,
            }
        )
    )
    raw_rep = await sock.recv()
    return organization_bootstrap_serializer.rep_loads(raw_rep)


@pytest.mark.trio
async def test_organization_create_already_exists(administrator_backend_sock, coolorg):
    rep = await organization_create(administrator_backend_sock, coolorg.organization_id)
    assert rep["status"] == "already_exists"


@pytest.mark.trio
async def test_organization_create_bad_name(administrator_backend_sock):
    rep = await organization_create(administrator_backend_sock, "a" * 33)
    assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_organization_create_and_bootstrap(
    backend,
    organization_factory,
    local_device_factory,
    alice,
    administrator_backend_sock,
    backend_sock_factory,
):
    neworg = organization_factory("NewOrg")

    # 1) Create organization, note this means `neworg.bootstrap_token`
    # will contain an invalid token

    rep = await organization_create(administrator_backend_sock, neworg.organization_id)
    assert rep == {"status": "ok", "bootstrap_token": ANY}
    bootstrap_token = rep["bootstrap_token"]

    # 2) Bootstrap organization

    # Use an existing user name to make sure they didn't mix together
    newalice = local_device_factory("alice@dev1", neworg)
    backend_newalice = local_device_to_backend_user(newalice, neworg)

    async with backend_sock_factory(backend, neworg.organization_id) as sock:
        rep = await organization_bootstrap(
            sock,
            bootstrap_token,
            backend_newalice.certified_user,
            backend_newalice.devices[newalice.device_name].certified_device,
            neworg.root_verify_key,
        )
    assert rep == {"status": "ok"}

    # 3) Now our new device can connect the backend

    async with backend_sock_factory(backend, newalice) as sock:
        await ping(sock)

    # 4) Make sure alice from the other organization is still working

    async with backend_sock_factory(backend, alice) as sock:
        await ping(sock)


@pytest.mark.trio
async def test_organization_bootstrap_bad_data(
    backend_data_binder,
    backend_sock_factory,
    organization_factory,
    local_device_factory,
    backend,
    coolorg,
    alice,
):
    neworg = organization_factory("NewOrg")
    newalice = local_device_factory("alice@dev1", neworg)
    await backend_data_binder.bind_organization(neworg)

    bad_organization_id = coolorg.organization_id
    good_organization_id = neworg.organization_id

    root_signing_key = neworg.root_signing_key
    bad_root_signing_key = coolorg.root_signing_key

    good_bootstrap_token = neworg.bootstrap_token
    bad_bootstrap_token = coolorg.bootstrap_token

    good_rvk = neworg.root_verify_key
    bad_rvk = coolorg.root_verify_key

    good_device_id = newalice.device_id

    good_user_id = newalice.user_id
    bad_user_id = UserID("dummy")

    public_key = newalice.public_key
    verify_key = newalice.verify_key

    now = pendulum.now()
    good_cu = certify_user(None, root_signing_key, good_user_id, public_key, now)
    good_cd = certify_device(None, root_signing_key, good_device_id, verify_key, now)

    bad_now = now - pendulum.interval(seconds=1)
    bad_now_cu = certify_user(None, root_signing_key, good_user_id, public_key, bad_now)
    bad_now_cd = certify_device(None, root_signing_key, good_device_id, verify_key, bad_now)
    bad_id_cu = certify_user(None, root_signing_key, bad_user_id, public_key, now)
    bad_key_cu = certify_user(None, bad_root_signing_key, good_user_id, public_key, now)
    bad_key_cd = certify_device(None, bad_root_signing_key, good_device_id, verify_key, now)

    for i, (status, organization_id, *params) in enumerate(
        [
            ("not_found", good_organization_id, bad_bootstrap_token, good_cu, good_cd, good_rvk),
            (
                "already_bootstrapped",
                bad_organization_id,
                bad_bootstrap_token,
                bad_key_cu,
                bad_key_cd,
                bad_rvk,
            ),
            (
                "invalid_certification",
                good_organization_id,
                good_bootstrap_token,
                good_cu,
                good_cd,
                bad_rvk,
            ),
            (
                "invalid_data",
                good_organization_id,
                good_bootstrap_token,
                bad_now_cu,
                good_cd,
                good_rvk,
            ),
            (
                "invalid_data",
                good_organization_id,
                good_bootstrap_token,
                bad_id_cu,
                good_cd,
                good_rvk,
            ),
            (
                "invalid_certification",
                good_organization_id,
                good_bootstrap_token,
                bad_key_cu,
                good_cd,
                good_rvk,
            ),
            (
                "invalid_data",
                good_organization_id,
                good_bootstrap_token,
                good_cu,
                bad_now_cd,
                good_rvk,
            ),
            (
                "invalid_certification",
                good_organization_id,
                good_bootstrap_token,
                good_cu,
                bad_key_cd,
                good_rvk,
            ),
        ]
    ):
        async with backend_sock_factory(backend, organization_id) as sock:
            rep = await organization_bootstrap(sock, *params)
        assert rep["status"] == status

    # Finally cheap test to make sure our "good" data were really good
    async with backend_sock_factory(backend, good_organization_id) as sock:
        rep = await organization_bootstrap(sock, good_bootstrap_token, good_cu, good_cd, good_rvk)
    assert rep["status"] == "ok"
