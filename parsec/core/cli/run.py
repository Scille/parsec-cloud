import trio
import click

from parsec.cli_utils import cli_exception_handler, generate_not_available_cmd
from parsec.core import logged_core_factory
from parsec.core.cli.utils import core_config_and_device_options, core_config_options

try:
    from parsec.core.gui import run_gui as _run_gui

except ImportError as exc:
    run_gui = generate_not_available_cmd(exc)

else:

    @click.command(short_help="run parsec GUI")
    @core_config_options
    def run_gui(config, **kwargs):
        """
        Run parsec GUI
        """
        _run_gui(config)


async def _run_mountpoint(config, device, mountpoint):
    async with logged_core_factory(config, device) as core:
        await core.mountpoint_manager.start(mountpoint)
        await trio.sleep_forever()


@click.command(short_help="run parsec mountpoint")
@core_config_and_device_options
@click.option("--mountpoint", "-m", type=click.Path(exists=False), required=True)
def run_mountpoint(config, device, mountpoint, **kwargs):
    """
    Expose device's parsec drive on the given mountpoint.
    """
    with cli_exception_handler(config.debug):
        trio.run(_run_mountpoint, config, device, mountpoint)
