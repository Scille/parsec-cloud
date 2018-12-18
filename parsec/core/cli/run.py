import trio
import click

from parsec.cli_utils import cli_exception_handler
from parsec.core import logged_core_factory
from parsec.core.cli.utils import core_config_and_device_options


async def _run(config, device, mountpoint):
    async with logged_core_factory(config, device) as core:
        await core.mountpoint_manager.start(mountpoint)
        await trio.sleep_forever()


@click.command(short_help="run parsec mountpoint")
@core_config_and_device_options
@click.option("--mountpoint", "-m", type=click.Path(exists=False), required=True)
def run(config, device, mountpoint, **kwargs):
    """
    Expose device's parsec drive on the given mountpoint.
    """
    with cli_exception_handler(config.debug):
        trio.run(_run, config, device, mountpoint)
