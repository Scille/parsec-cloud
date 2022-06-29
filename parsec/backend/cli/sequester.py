# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

from uuid import uuid4
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


async def run_pg_sequester_component(
    config: BackendDbConfig,
    organization_str: OrganizationID,
    register_service_req: SequesterService,
):
    event_bus = EventBus()
    dbh = PGHandler(config.db_url, config.db_min_connections, config.db_max_connections, event_bus)
    sequester_component = PGPSequesterComponent(dbh)

    async with open_service_nursery() as nursery:
        await dbh.init(nursery)
        try:
            await sequester_component.create_service(
                OrganizationID(organization_str), register_service_req
            )
        finally:
            await dbh.teardown()


@click.command()
@click.option(
    "--service_public_key",
    type=click.Path(
        exists=True, file_okay=True, dir_okay=False, path_type=Path  # type: ignore[type-var]
    ),
)
@click.option(
    "--authority_private_key",
    type=click.Path(
        exists=True, file_okay=True, dir_okay=False, path_type=Path  # type: ignore[type-var]
    ),
)
@click.option("--service_label", type=str, required=True, help="New service name")
@click.option("--organization", type=str)
@db_backend_options
def register_service(
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
    print("---")
    print(certif_data)
    print(authority_key)
    certificate = sequester_authority_sign(signing_key=authority_key, data=certif_data.dump())
    print(certificate)
    print("--")
    sequester_service = SequesterService(
        service_id=service_id,
        service_label=service_label,
        service_certificate=certificate,
        created_on=now,
    )

    if db.upper() == "MOCKED":
        raise SequesterBackendCliError("MOCKED DB can not be used with sequester services")

    db_config = BackendDbConfig(
        db_url=db, db_min_connections=db_min_connections, db_max_connections=db_max_connections
    )

    trio_run(
        run_pg_sequester_component, db_config, organization, sequester_service, use_asyncio=True
    )
