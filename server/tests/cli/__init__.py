# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import click

from parsec.cli import cli


# Some PAAS services (e.g. Scalingo) has a 64 character limit on the environ variable name
def test_environ_variable_name_size():
    def _recursive(item: click.Command):
        if isinstance(item, click.Group):
            for cmd in item.commands.values():
                _recursive(cmd)
        else:
            for param in item.params:
                if param.envvar is None:
                    continue
                assert len(param.envvar) < 64, (
                    f"Command `{item.name}`: environ variable name too long `{param.envvar}`"
                )

    _recursive(cli)
