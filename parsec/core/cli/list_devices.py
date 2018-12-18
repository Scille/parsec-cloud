import os
import click

from parsec.core.config import get_default_config_dir
from parsec.core.devices_manager import list_available_devices


@click.command()
@click.option("--config-dir", type=click.Path(exists=True, file_okay=False))
def list_devices(config_dir):
    config_dir = config_dir or get_default_config_dir(os.environ)
    devices = list_available_devices(config_dir)
    num_devices_display = click.style(str(len(devices)), fg="green")
    click.echo(f"Found {num_devices_display} device(s):")
    for device, cipher in devices:
        click.echo(f"{device} (cipher: {cipher})")
