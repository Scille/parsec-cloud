#! /usr/bin/env python3
# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


import trio
import os

from parsec.crypto import PrivateKey
from parsec.core.types import BackendInvitationAddr, BackendAddr
from parsec.core.config import get_default_config_dir
from parsec.core.local_device import list_available_devices, load_device_with_password
from parsec.core.backend_connection import backend_authenticated_cmds_factory
from parsec.core.backend_connection.transport import connect_as_invited
from parsec.core.backend_connection.cmds import invite_1_claimer_wait_peer_serializer
from parsec.api.protocol import InvitationType, ping_serializer
from parsec.test_utils import initialize_test_organization
from run_testenv import DEFAULT_ADMINISTRATION_TOKEN, DEFAULT_DEVICE_PASSWORD
from wsproto.events import Ping

backend_url = "parsec://localhost:6888?no_ssl=true"
org_id = "Org"
alice_email = "alice@example.com"
test_config_dir = "dev_debug_websocket"


async def _api_ping(transport):
    while True:
        raw_req = ping_serializer.req_dumps({"cmd": "ping", "ping": ""})
        await transport.send(raw_req)
        print("\n _api_ping")
        try:
            await transport.recv()
        except Exception:
            print(
                "api pong error transport.stream.receive_some already used by _invite_1_claimer_wait_peer"
            )
        else:
            print("api pong")
        await trio.sleep(4)


async def _websocket_ping(transport):
    while True:
        print("\n websocket ping")
        await transport._net_send(Ping())
        await trio.sleep(4)


async def invite_new_user(cmds, device):
    print("\n invite new user")
    rep = await cmds.invite_new(
        type=InvitationType.USER, claimer_email="test@test.fr", send_email=False
    )
    print("invit new user response")
    return BackendInvitationAddr.build(
        backend_addr=device.organization_addr,
        organization_id=device.organization_id,
        invitation_type=InvitationType.USER,
        token=rep["token"],
    )


async def _invite_1_claimer_wait_peer(transport):
    await trio.sleep(5)
    pk = PrivateKey.generate()
    raw_req = invite_1_claimer_wait_peer_serializer.req_dumps(
        {"cmd": "invite_1_claimer_wait_peer", "claimer_public_key": pk.public_key}
    )
    print("\n _invite_1_claimer_wait_peer")
    await transport.send(raw_req)
    await transport.recv()
    print("_invite_1_claimer_wait_peer response")


async def invite_1_greeter_wait_peer(cmds, token):
    await trio.sleep(25)
    print("\n greeter wait")
    pk = PrivateKey.generate()
    await cmds.invite_1_greeter_wait_peer(token=token, greeter_public_key=pk.public_key)
    print("greeter wait response")


async def main():
    backend_addr = BackendAddr.from_url(backend_url)
    config_dir = get_default_config_dir(os.environ) / test_config_dir
    print("initialize test organization %s" % config_dir)
    try:
        alice_device, _ = await initialize_test_organization(
            config_dir=config_dir,
            backend_address=backend_addr,
            password=DEFAULT_DEVICE_PASSWORD,
            administration_token=DEFAULT_ADMINISTRATION_TOKEN,
            force=False,
            additional_users_number=True,
            additional_devices_number=True,
        )
    except AssertionError:
        devices = list_available_devices(config_dir)
        for d in devices:
            if d.organization_id == org_id and d.human_handle.email == alice_email:
                alice_device = load_device_with_password(d.key_file_path, DEFAULT_DEVICE_PASSWORD)
                break

    print(alice_device.organization_addr)
    async with backend_authenticated_cmds_factory(
        addr=alice_device.organization_addr,
        device_id=alice_device.device_id,
        signing_key=alice_device.signing_key,
    ) as alice_cmds:
        invitation_addr = await invite_new_user(alice_cmds, alice_device)
        transport = await connect_as_invited(invitation_addr)
        async with trio.open_nursery() as nursery:
            nursery.start_soon(_invite_1_claimer_wait_peer, transport)
            # nursery.start_soon(_api_ping, transport)
            nursery.start_soon(_websocket_ping, transport)
            nursery.start_soon(invite_1_greeter_wait_peer, alice_cmds, invitation_addr._token)


if __name__ == "__main__":
    trio.run(main)
