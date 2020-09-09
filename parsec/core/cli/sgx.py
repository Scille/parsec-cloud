# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import click

from parsec.utils import trio_run
from parsec.logging import configure_logging
from parsec.cli_utils import cli_exception_handler
from parsec.core.types import BackendAddr
from parsec.core.backend_connection import apiv1_backend_administration_cmds_factory


async def _sgx_hello_world(backend_addr, administration_token):
    print("Trying to execute hello world from SGX enclave...")
    async with apiv1_backend_administration_cmds_factory(
        backend_addr, administration_token
    ) as cmds:
        await cmds.sgx_hello_world()


@click.command(short_help="Hello World from the enclave")
@click.option("--addr", "-B", required=True, type=BackendAddr.from_url)
@click.option("--administration-token", "-T", required=True)
def sgx_hello_world(addr, administration_token):
    debug = "DEBUG" in os.environ
    configure_logging(log_level="DEBUG" if debug else "WARNING")

    with cli_exception_handler(debug):
        trio_run(_sgx_hello_world, addr, administration_token)
