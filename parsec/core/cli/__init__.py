import click

# from parsec.core.cli.claim_user import claim_user
from parsec.core.cli.list_devices import list_devices
from parsec.core.cli.invite_user import invite_user
from parsec.core.cli.create_organization import create_organization
from parsec.core.cli.bootstrap_organization import bootstrap_organization
from parsec.core.cli.run import run_gui, run_mountpoint


__all__ = ("core_cmd",)


@click.group()
def core_cmd():
    pass


# core_cmd.add_command(core_gui, 'gui')
core_cmd.add_command(run_gui, "gui")
core_cmd.add_command(run_mountpoint, "run")
core_cmd.add_command(list_devices, "list_devices")
core_cmd.add_command(invite_user, "invite_user")
core_cmd.add_command(create_organization, "create_organization")
core_cmd.add_command(bootstrap_organization, "bootstrap_organization")
