import trio
import click

from parsec.core import logged_core_factory
from parsec.core.gui import run_gui


async def _gui(config, device):
    async with logged_core_factory(config, device) as core:
        await core.mountpoint_manager.start(mountpoint)
        await trio.sleep_forever()


@click.command(short_help="run parsec GUI")
@core_config_options
def gui(config, **kwargs):
    """
    Run parsec GUI
    """
    run_gui(config)


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


# @click.command()
# @click.option("--device", "-D", type=DeviceID, required=True)
# @click.option("--pkcs11", is_flag=True)
# @click.option("--backend-watchdog", "-W", type=click.INT, default=None)
# @click.option("--debug", "-d", is_flag=True)
# @click.option(
#     "--log-level", "-l", default="WARNING", type=click.Choice(("DEBUG", "INFO", "WARNING", "ERROR"))
# )
# @click.option("--log-format", "-f", default="CONSOLE", type=click.Choice(("CONSOLE", "JSON")))
# @click.option("--log-file", "-o")
# @click.option("--log-filter", default=None)
# def core_cli(**kwargs):
#     configure_logging(kwargs['log_level'], kwargs['log_format'], kwargs['log_file'], kwargs['log_filter'])
#     config = config_factory(
#         debug=kwargs['debug'],
#         backend_watchdog=kwargs['backend_watchdog'],
#         environ=os.environ
#     )

#     if config.sentry_url:
#         configure_sentry_logging(config.sentry_url)

#     device_id = kwargs['device']
#     if device_id not in list_available_devices(config.config_dir):
#         print(list_available_devices(config.config_dir))
#         raise SystemExit(f"Device {device_id} doesn't exist.")

#     try:
#         if kwargs['pkcs11']:
#             token_id = click.prompt('PCKS11 token id', type=int)
#             key_id = click.prompt('PCKS11 key id', type=int)
#             pin = click.prompt('PCKS11 pin', hide_input=True, confirmation_prompt=True)
#             device = load_device_with_pkcs11(config.config_dir, device_id, token_id, key_id, pin)

#         else:
#             password = click.prompt('password', hide_input=True, confirmation_prompt=True)
#             device = load_device_with_password(config.config_dir, device_id, password)

#     except DeviceManagerError as exc:
#         raise SystemExit(f"Cannot load device {device_id}: {exc}")

#     async def _run_core():
#         async with logged_core_factory(config, device) as core:
#             try:
#                 mountpoint = get_default_mountpoint(device.device_id)
#                 await core.mountpoint_manager.start(mountpoint)
#                 await trio.sleep_forever()

#             except KeyboardInterrupt:
#                 print("bye ;-)")

#     trio.run(_run_core)


# @click.command()
# @click.option("--backend-addr", "-A", default="tcp://127.0.0.1:6777")
# @click.option("--backend-watchdog", "-W", type=click.INT, default=None)
# @click.option("--debug", "-d", is_flag=True)
# @click.option(
#     "--log-level", "-l", default="WARNING", type=click.Choice(("DEBUG", "INFO", "WARNING", "ERROR"))
# )
# @click.option("--log-format", "-f", default="CONSOLE", type=click.Choice(("CONSOLE", "JSON")))
# @click.option("--log-file", "-o")
# @click.option("--log-filter", default=None)
# @click.option("--no-ui", help="Disable the GUI", is_flag=True)
# def core_cmd(log_level, log_format, log_file, log_filter, **kwargs):
#     configure_logging(log_level, log_format, log_file, log_filter)
#     return _core(**kwargs)


# def _core(socket, backend_addr, backend_watchdog, debug, i_am_john, no_ui):
#     async def _login_and_run(portal_queue, user=None):
#         portal = trio.BlockingTrioPortal()
#         portal_queue.put(portal)
#         async with trio.open_nursery() as nursery:
#             await core.init(nursery)
#             try:
#                 if user:
#                     await core.login(user)
#                     print("Logged as %s" % user.id)

#                 with trio.open_cancel_scope() as cancel_scope:
#                     portal_queue.put(cancel_scope)
#                     if socket.startswith("unix://"):
#                         await trio.serve_unix(core.handle_client, socket[len("unix://") :])
#                     elif socket.startswith("tcp://"):
#                         parsed = urlparse(socket)
#                         await trio.serve_tcp(core.handle_client, parsed.port, host=parsed.hostname)
#                     else:
#                         raise SystemExit(f"Error: Invalid --socket value `{socket}`")
#             finally:
#                 await core.teardown()

#     def _trio_run(funct, portal_queue, user=None):
#         trio.run(funct, portal_queue, user)

#     config = CoreConfig(
#         debug=debug, addr=socket, backend_addr=backend_addr, backend_watchdog=backend_watchdog
#     )

#     if config.sentry_url:
#         configure_sentry_logging(config.sentry_url)

#     core = Core(config)

#     print(f"Starting Parsec Core on {socket} (with backend on {config.backend_addr})")

#     try:
#         portal_queue = queue.Queue(1)
#         args = (_login_and_run, portal_queue)
#         if i_am_john:
#             try:
#                 core.local_devices_manager.register_new_device(
#                     JOHN_DOE_DEVICE_ID,
#                     JOHN_DOE_PRIVATE_KEY,
#                     JOHN_DOE_DEVICE_SIGNING_KEY,
#                     JOHN_DOE_USER_MANIFEST_ACCESS,
#                 )
#             except DeviceSavingAlreadyExists:
#                 pass
#             device = core.local_devices_manager.load_device(JOHN_DOE_DEVICE_ID)

#             print(f"Hello Mr. Doe, your conf dir is `{device.local_db.path}`")
#             args = args + (device,)
#         if no_ui:
#             print("UI is disabled")
#             _trio_run(*args)
#         else:
#             trio_thread = threading.Thread(target=_trio_run, args=args)
#             trio_thread.start()
#             portal = portal_queue.get()
#             cancel_scope = portal_queue.get()
#             run_gui(core, portal, cancel_scope)
#             trio_thread.join()
#     except KeyboardInterrupt:
#         print("bye ;-)")
