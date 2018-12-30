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
