# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
import click
import cProfile
from pathlib import Path

from parsec.core.types import FsPath
from parsec.utils import trio_run
from parsec.cli_utils import cli_exception_handler
from parsec.core import logged_core_factory
from parsec.core.cli.utils import core_config_and_device_options


async def _run_benchmark(stats_out, core, wid, w_mount, type):
    with core.event_bus.waiter_on("backend.connection.ready") as event:
        await event.wait()

    total_size = 10 * 2 ** 20  # 10mo file
    total_buff = bytearray(total_size)
    chunk_size = 4 * 1024
    chunk = bytearray(chunk_size)
    pr = cProfile.Profile()
    pr.enable()

    print("********** starting bench ***********")

    if type == "mountpoint":
        foo = trio.Path(w_mount / "foo.txt")
        await foo.touch()
        await foo.write_bytes(total_buff)

    elif type.startswith("api"):
        w = core.user_fs.get_workspace(wid)
        path = FsPath("/foo.txt")
        await w.touch(path)

        if type == "api_4k_chunks":
            _, fd = await w.entry_transactions.file_open(path, "w")
            try:
                for i in range(total_size // chunk_size):
                    await w.file_transactions.fd_write(fd, chunk, i * chunk_size)
            finally:
                await w.file_transactions.fd_close(fd)
        else:
            await w.write_bytes(path, total_buff)

    print("********** bench done ***********")

    # Finally store the stats
    pr.disable()
    pr.create_stats()
    if not stats_out:
        pr.print_stats()
    else:
        pr.dump_stats(stats_out)


async def _run_mountpoint(config, device, stats_out, type):
    config = config.evolve(mountpoint_enabled=True)
    async with logged_core_factory(config, device) as core:

        wid = await core.user_fs.workspace_create("w1")

        w_mount = await core.mountpoint_manager.mount_workspace(wid)
        display_device = click.style(device.device_id, fg="yellow")
        mountpoint_display = click.style(str(config.mountpoint_base_dir.absolute()), fg="yellow")
        click.echo(f"{display_device}'s drive mounted at {mountpoint_display}")

        await _run_benchmark(stats_out, core, wid, w_mount, type)


@click.command(short_help="Run benchmark")
@core_config_and_device_options
@click.option("--mountpoint", "-m", type=click.Path(exists=False))
@click.option("--stats-out", "-o")
@click.option(
    "--type",
    "-t",
    default="api_4k_chunks",
    type=click.Choice(("mountpoint", "api", "api_4k_chunks")),
)
def benchmark(config, device, mountpoint, stats_out, type, **kwargs):
    config = config.evolve(mountpoint_enabled=True)
    if mountpoint:
        config = config.evolve(mountpoint_base_dir=Path(mountpoint))
    with cli_exception_handler(config.debug):
        trio_run(_run_mountpoint, config, device, stats_out, type)
