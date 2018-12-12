# This crappy script is just meant to be used to test the GUI. It creates a lot of
# workspaces (in what I'm sure is the best way possible :p) to see how the GUI handles workspace
# display. It just kinda works and I didn't want to waste my time finding a better solution.

import random
import string
import os
import argparse
import queue
import threading
import trio
from uuid import UUID
from urllib.parse import urlparse

from parsec.core import Core, CoreConfig

from parsec.core.gui.core_call import core_call, init_core_call


JOHN_DOE_DEVICE_ID = "johndoe@test"
JOHN_DOE_PRIVATE_KEY = (
    b"]x\xd3\xa9$S\xa92\x9ex\x91\xa7\xee\x04SY\xbe\xe6"
    b"\x03\xf0\x1d\xe2\xcc7\x8a\xd7L\x137\x9e\xa7\xc6"
)
JOHN_DOE_DEVICE_SIGNING_KEY = (
    b"w\xac\xd8\xb4\x88B:i\xd6G\xb9\xd6\xc5\x0f\xf6\x99"
    b"\xccH\xfa\xaeY\x00:\xdeP\x84\t@\xfe\xf8\x8a\xa5"
)
JOHN_DOE_USER_MANIFEST_ACCESS = {
    "id": UUID("230165e6acd441f4a0b4f2c8c0dc91f0"),
    "rts": "c7121459551b40e78e35f49115097594",
    "wts": "3c7d3cb553854ffea524092487674a0b",
    "key": (
        b"\x8d\xa3k\xb8\xd8'a6?\xf8\xc7\xf2p\xba\xc8=\xb9\r\x9a"
        b"\x0e\xea\xb1\xb8\x93\xae\xc2\xc2\x8c\x16\x8e\xa4\xc3"
    ),
}


def start():
    socket = "tcp://127.0.0.1:6776"
    backend_addr = "tcp://127.0.0.1:6777"

    async def _login_and_run(portal_queue, user=None):
        portal = trio.BlockingTrioPortal()
        portal_queue.put(portal)
        async with trio.open_nursery() as nursery:
            await core.init(nursery)
            try:
                if user:
                    await core.login(user)
                    print("Logged as %s" % user.id)

                with trio.open_cancel_scope() as cancel_scope:
                    portal_queue.put(cancel_scope)
                    parsed = urlparse(socket)
                    await trio.serve_tcp(core.handle_client, parsed.port, host=parsed.hostname)
            finally:
                await core.teardown()

    def _trio_run(funct, portal_queue, user=None):
        trio.run(funct, portal_queue, user)

    config = CoreConfig(debug=False, addr=socket, backend_addr=backend_addr, backend_watchdog=None)
    core = Core(config)
    try:
        portal_queue = queue.Queue(1)
        args = (_login_and_run, portal_queue)
        try:
            core.local_devices_manager.register_new_device(
                JOHN_DOE_DEVICE_ID,
                JOHN_DOE_PRIVATE_KEY,
                JOHN_DOE_DEVICE_SIGNING_KEY,
                JOHN_DOE_USER_MANIFEST_ACCESS,
            )
        except:
            pass
        device = core.local_devices_manager.load_device(JOHN_DOE_DEVICE_ID)
        args = args + (device,)
        trio_thread = threading.Thread(target=_trio_run, args=args)
        trio_thread.start()
        portal = portal_queue.get()
        cancel_scope = portal_queue.get()
        init_core_call(parsec_core=core, trio_portal=portal, cancel_scope=cancel_scope)
        trio_thread.join()
    except KeyboardInterrupt:
        print("bye ;-)")


def add_workspace(prefix):
    try:
        workspace = prefix + "".join(random.choice(string.ascii_lowercase) for _ in range(8))
        core_call().create_workspace(os.path.join("/", workspace))
    except:
        print("Failed to add workspace")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Add N workspaces")
    parser.add_argument("-n", type=int, default=20, help="Number of workspaces to add")
    parser.add_argument(
        "--prefix", type=str, default="workspace_", help="Prefix for generated workspace names"
    )
    args = parser.parse_args()

    start()

    for i in range(args.n):
        add_workspace(args.prefix)

    core_call().logout()
    core_call().stop()
