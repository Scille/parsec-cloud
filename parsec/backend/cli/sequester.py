# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

from uuid import uuid4
from async_generator import asynccontextmanager
import attr
import click
import pendulum
from parsec.api.data.certif import SequesterServiceCertificate
from parsec.event_bus import EventBus

from parsec.api.protocol.sequester import SequesterServiceID
from parsec.api.protocol.types import OrganizationID
from parsec.backend.postgresql.handler import PGHandler
from parsec.backend.postgresql.sequester import PGPSequesterComponent
from parsec.backend.sequester import SequesterService
from parsec.sequester_crypto import sequester_authority_sign
from pathlib import Path
from parsec.sequester_crypto import SequesterEncryptionKeyDer

from parsec.utils import open_service_nursery, trio_run
from parsec.backend.cli.utils import db_backend_options

import oscrypto.asymmetric


class SequesterBackendCliError(Exception):
    pass


@attr.s(slots=True, frozen=True, auto_attribs=True)
class BackendDbConfig:
    db_url: str
    db_min_connections: int
    db_max_connections: int


@asynccontextmanager
async def run_pg_sequester_component(config: BackendDbConfig):
    event_bus = EventBus()
    dbh = PGHandler(config.db_url, config.db_min_connections, config.db_max_connections, event_bus)
    sequester_component = PGPSequesterComponent(dbh)

    async with open_service_nursery() as nursery:
        await dbh.init(nursery)
        try:
            yield sequester_component
        finally:
            await dbh.teardown()


async def _create_service(
    config: BackendDbConfig, organization_str: str, register_service_req: SequesterService
):
    async with run_pg_sequester_component(config) as sequester_component:
        await sequester_component.create_service(
            OrganizationID(organization_str), register_service_req
        )


def display_service(service: SequesterService):
    click.echo(f"Service ID :: {service.service_id}")
    click.echo(f"Service label :: {service.service_label}")
    click.echo(f"Service creation date :: {service.created_on}")
    if service.deleted_on:
        click.echo(f"Service is deleted since {service.deleted_on}")
    click.echo("")


async def _list_services(config, organization_str: str):
    async with run_pg_sequester_component(config) as sequester_component:
        services = await sequester_component.get_organization_services(
            OrganizationID(organization_str)
        )
    if services:
        click.echo("=== Services ===")
        for service in services:
            display_service(service)
    else:
        click.echo("No service configured")


def _get_config(db: str, db_min_connections: int, db_max_connections: int) -> BackendDbConfig:
    if db.upper() == "MOCKED":
        raise SequesterBackendCliError("MOCKED DB can not be used with sequester services")

    return BackendDbConfig(
        db_url=db, db_min_connections=db_min_connections, db_max_connections=db_max_connections
    )


@click.command(short_help="Register a new sequester service")
@click.option(
    "--service-public-key",
    help="The service encryption public key used to encrypt data to the sequester service",
    type=click.Path(
        exists=True, file_okay=True, dir_okay=False, path_type=Path  # type: ignore[type-var]
    ),
)
@click.option(
    "--authority-private-key",
    help="The private authority key use. Used to sign the encryption key.",
    type=click.Path(
        exists=True, file_okay=True, dir_okay=False, path_type=Path  # type: ignore[type-var]
    ),
)
@click.option("--service-label", type=str, required=True, help="New service name")
@click.option("--organization", type=str, help="Organization ID where to register the service")
@db_backend_options
def create_service(
    service_public_key: Path,
    authority_private_key: Path,
    service_label: str,
    organization: str,
    db: str,
    db_max_connections: int,
    db_min_connections: int,
):
    # Load key files
    service_key = SequesterEncryptionKeyDer(service_public_key.read_bytes())
    authority_key = oscrypto.asymmetric.load_private_key(authority_private_key.read_bytes())
    # Generate data schema
    service_id = SequesterServiceID(uuid4())
    now = pendulum.now()
    certif_data = SequesterServiceCertificate(
        timestamp=now,
        service_id=service_id,
        service_label=service_label,
        encryption_key_der=service_key,
    )
    certificate = sequester_authority_sign(signing_key=authority_key, data=certif_data.dump())
    sequester_service = SequesterService(
        service_id=service_id,
        service_label=service_label,
        service_certificate=certificate,
        created_on=now,
    )
    db_config = _get_config(db, db_min_connections, db_max_connections)

    trio_run(_create_service, db_config, organization, sequester_service, use_asyncio=True)


@click.command()
@click.option("--organization", type=str, help="Organization ID")
@db_backend_options
def list_services(organization: str, db: str, db_max_connections: int, db_min_connections: int):
    db_config = _get_config(db, db_min_connections, db_max_connections)
    trio_run(_list_services, db_config, organization, use_asyncio=True)
