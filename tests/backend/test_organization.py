import pytest
import pendulum

from parsec.types import DeviceID
from parsec.crypto import SigningKey
from parsec.trustchain import certify_user, certify_device
from parsec.core.devices_manager import generate_new_device

from parsec.api.protocole import organization_create_serializer, organization_bootstrap_serializer

from tests.backend.test_events import ping


async def organization_create(sock, name):
    raw_rep = await sock.send(
        organization_create_serializer.req_dumps({"cmd": "organization_create", "name": name})
    )
    raw_rep = await sock.recv()
    return organization_create_serializer.rep_loads(raw_rep)


async def organization_bootstrap(
    sock, name, bootstrap_token, certified_user, certified_device, root_verify_key
):
    raw_rep = await sock.send(
        organization_bootstrap_serializer.req_dumps(
            {
                "cmd": "organization_bootstrap",
                "name": name,
                "bootstrap_token": bootstrap_token,
                "certified_user": certified_user,
                "certified_device": certified_device,
                "root_verify_key": root_verify_key,
            }
        )
    )
    raw_rep = await sock.recv()
    return organization_bootstrap_serializer.rep_loads(raw_rep)


@pytest.fixture
async def fresh_organization(anonymous_backend_sock, name="cool_org_inc"):
    rep = await organization_create(anonymous_backend_sock, name)
    assert rep["status"] == "ok"
    return name, rep["bootstrap_token"]


@pytest.mark.trio
async def test_organization_create_and_bootstrap(
    backend, backend_addr, anonymous_backend_sock, backend_sock_factory
):
    # Create organization

    rep = await organization_create(anonymous_backend_sock, "cool_org_inc")
    assert rep["status"] == "ok"
    bootstrap_token = rep["bootstrap_token"]

    # Generate device & root key locally

    root_signing_key = SigningKey.generate()
    root_verify_key = root_signing_key.verify_key

    device = generate_new_device(DeviceID("Zack@pc1"), backend_addr, root_verify_key)

    now = pendulum.now()
    certified_user = certify_user(None, root_signing_key, device.user_id, device.public_key, now)
    certified_device = certify_device(
        None, root_signing_key, device.device_id, device.verify_key, now
    )

    # Bootstrap organization

    rep = await organization_bootstrap(
        anonymous_backend_sock,
        "cool_org_inc",
        bootstrap_token,
        certified_user,
        certified_device,
        root_verify_key,
    )
    assert rep == {"status": "ok"}

    # Now our new device can connect backend

    async with backend_sock_factory(backend, device) as sock:
        await ping(sock)


@pytest.mark.trio
async def test_organization_create_bad_name(anonymous_backend_sock):
    rep = await organization_create(anonymous_backend_sock, "a" * 33)
    assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_organization_bootstrap_bad_data(
    anonymous_backend_sock, mallory, bob, root_signing_key, root_verify_key, fresh_organization
):
    good_rvk = root_verify_key
    bad_rvk = bob.verify_key

    now = pendulum.now()
    good_cu = certify_user(None, root_signing_key, mallory.user_id, mallory.public_key, now)
    good_cd = certify_device(None, root_signing_key, mallory.device_id, mallory.verify_key, now)

    bad_now = now - pendulum.interval(seconds=1)
    bad_now_cu = certify_user(None, root_signing_key, mallory.user_id, mallory.public_key, bad_now)
    bad_now_cd = certify_device(
        None, root_signing_key, mallory.device_id, mallory.verify_key, bad_now
    )
    bad_id_cu = certify_user(None, root_signing_key, bob.user_id, mallory.public_key, now)
    bad_id_cd = certify_device(None, root_signing_key, bob.device_id, mallory.verify_key, now)
    bad_key_cu = certify_user(None, bob.signing_key, mallory.user_id, mallory.public_key, now)
    bad_key_cd = certify_device(None, bob.signing_key, mallory.device_id, mallory.verify_key, now)

    good_name, good_bootstrap_token = fresh_organization
    bad_name = "dummy-org"
    bad_bootstrap_token = "bad-123456"

    for i, (status, *params) in enumerate(
        [
            ("not_found", bad_name, good_bootstrap_token, good_cu, good_cd, good_rvk),
            ("not_found", good_name, bad_bootstrap_token, good_cu, good_cd, good_rvk),
            ("invalid_certification", good_name, good_bootstrap_token, good_cu, good_cd, bad_rvk),
            ("invalid_data", good_name, good_bootstrap_token, bad_now_cu, good_cd, good_rvk),
            ("invalid_data", good_name, good_bootstrap_token, bad_id_cu, good_cd, good_rvk),
            (
                "invalid_certification",
                good_name,
                good_bootstrap_token,
                bad_key_cu,
                good_cd,
                good_rvk,
            ),
            ("invalid_data", good_name, good_bootstrap_token, good_cu, bad_now_cd, good_rvk),
            ("invalid_data", good_name, good_bootstrap_token, good_cu, bad_id_cd, good_rvk),
            (
                "invalid_certification",
                good_name,
                good_bootstrap_token,
                good_cu,
                bad_key_cd,
                good_rvk,
            ),
        ]
    ):
        rep = await organization_bootstrap(anonymous_backend_sock, *params)
        assert rep["status"] == status

    # Finaly cheap test to make sure our "good" data were really good
    rep = await organization_bootstrap(
        anonymous_backend_sock, good_name, good_bootstrap_token, good_cu, good_cd, good_rvk
    )
    assert rep["status"] == "ok"
