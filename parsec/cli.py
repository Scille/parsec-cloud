import click
from pyupdater.client import Client, AppUpdate
import sys

from parsec.core.cli import core_cmd
from parsec.backend.cli import backend_cmd
from parsec.ui import shell
from parsec import __version__

try:
    from parsec.ui.fuse import cli as fuse_cli
except ImportError:

    @click.command()
    def fuse_cli():
        raise RuntimeError("No available, is fusepy installed ?")


except NameError:
    pass

try:
    from client_config import ClientConfig  # pylint: disable=import-error
except ImportError:
    sys.stderr.write(
        "client_config.py is missing.\n"
        "You need to run: pyupdater init\n"
        "See: http://www.pyupdater.org/usage-cli/\n"
    )
    sys.exit(1)


@click.group()
def cli():
    pass


cli.add_command(core_cmd, "core")
cli.add_command(backend_cmd, "backend")
try:
    cli.add_command(fuse_cli, "fuse")
except NameError:
    pass
cli.add_command(shell.cli, "shell")


def print_status_info(info):
    total = info.get(u"total")
    downloaded = info.get(u"downloaded")
    status = info.get(u"status")
    print(str(downloaded) + "/" + str(total), status)


def get_update():
    client = Client(ClientConfig(), refresh=True, progress_hooks=[print_status_info])
    app_update = client.update_check(ClientConfig.APP_NAME, __version__, channel="stable")
    if app_update:
        if app_update.download():
            if isinstance(app_update, AppUpdate):
                app_update.extract_restart()
                return app_update.latest
            else:
                app_update.extract()
                return app_update.latest


def main():
    print("Current version is", __version__)
    latest_version = get_update()
    if latest_version:
        print("There is a new update. Version", latest_version, "is now installed :)")
    else:
        print("No update available :(")
    cli()


if __name__ == "__main__":
    main()
