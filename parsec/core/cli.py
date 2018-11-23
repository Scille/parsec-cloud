import queue
import threading
import trio
import click
from uuid import UUID
from urllib.parse import urlparse

from parsec.logging import configure_logging, configure_sentry_logging
from parsec.core import Core, CoreConfig
from parsec.core.devices_manager import DeviceSavingAlreadyExists
from parsec.core.gui import run_gui


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
DEFAULT_CORE_SOCKET = "tcp://127.0.0.1:6776"


@click.command()
@click.option(
    "--socket",
    "-s",
    default=DEFAULT_CORE_SOCKET,
    help=(
        "Socket path (`tcp://<domain:port>` or `unix://<path>`) "
        "exposing the core API (default: %s)." % DEFAULT_CORE_SOCKET
    ),
)
@click.option("--backend-addr", "-A", default="tcp://127.0.0.1:6777")
@click.option("--backend-watchdog", "-W", type=click.INT, default=None)
@click.option("--debug", "-d", is_flag=True)
@click.option(
    "--log-level", "-l", default="WARNING", type=click.Choice(("DEBUG", "INFO", "WARNING", "ERROR"))
)
@click.option("--log-format", "-f", default="CONSOLE", type=click.Choice(("CONSOLE", "JSON")))
@click.option("--log-file", "-o")
@click.option("--log-filter", default=None)
@click.option("--I-am-John", is_flag=True, help="Log as dummy John Doe user")
@click.option("--no-ui", help="Disable the GUI", is_flag=True)
def core_cmd(log_level, log_format, log_file, log_filter, **kwargs):
    configure_logging(log_level, log_format, log_file, log_filter)
    return _core(**kwargs)


def _core(socket, backend_addr, backend_watchdog, debug, i_am_john, no_ui):
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
                    if socket.startswith("unix://"):
                        await trio.serve_unix(core.handle_client, socket[len("unix://") :])
                    elif socket.startswith("tcp://"):
                        parsed = urlparse(socket)
                        await trio.serve_tcp(core.handle_client, parsed.port, host=parsed.hostname)
                    else:
                        raise SystemExit(f"Error: Invalid --socket value `{socket}`")
            finally:
                await core.teardown()

    def _trio_run(funct, portal_queue, user=None):
        trio.run(funct, portal_queue, user)

    config = CoreConfig(
        debug=debug, addr=socket, backend_addr=backend_addr, backend_watchdog=backend_watchdog
    )

    if config.sentry_url:
        configure_sentry_logging(config.sentry_url)

    core = Core(config)

    print(f"Starting Parsec Core on {socket} (with backend on {config.backend_addr})")

    try:
        portal_queue = queue.Queue(1)
        args = (_login_and_run, portal_queue)
        if i_am_john:
            try:
                core.local_devices_manager.register_new_device(
                    JOHN_DOE_DEVICE_ID,
                    JOHN_DOE_PRIVATE_KEY,
                    JOHN_DOE_DEVICE_SIGNING_KEY,
                    JOHN_DOE_USER_MANIFEST_ACCESS,
                )
            except DeviceSavingAlreadyExists:
                pass
            device = core.local_devices_manager.load_device(JOHN_DOE_DEVICE_ID)

            print(f"Hello Mr. Doe, your conf dir is `{device.local_db.path}`")
            args = args + (device,)
        if no_ui:
            print("UI is disabled")
            _trio_run(*args)
        else:
            trio_thread = threading.Thread(target=_trio_run, args=args)
            trio_thread.start()
            portal = portal_queue.get()
            cancel_scope = portal_queue.get()
            run_gui(core, portal, cancel_scope)
            trio_thread.join()
    except KeyboardInterrupt:
        print("bye ;-)")


if __name__ == "__main__":
    core_cmd()
