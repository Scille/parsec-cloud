# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS
from uuid import uuid4
import click
import pendulum
from parsec.event_bus import EventBus
from parsec.api.data.sequester import EncryptionKeyFormat, SequesterServiceEncryptionKey

from parsec.api.protocol.sequester import SequesterServiceType
from parsec.api.protocol.types import OrganizationID
from parsec.backend.app import backend_app_factory
from parsec.backend.cli.run import DEFAULT_EMAIL_SENDER, basic_admin_backend_options
from parsec.backend.config import (
    BackendConfig,
    BaseBlockStoreConfig,
    MockedBlockStoreConfig,
    MockedEmailConfig,
)
from parsec.backend.postgresql.handler import PGHandler
from parsec.backend.postgresql.sequester import PGPSequesterComponent
from parsec.backend.sequester import SequesterService
from parsec.sequester_crypto import (
    load_sequester_private_key,
    load_sequester_public_key,
    sign_sequester,
)
from pathlib import Path

from parsec.utils import open_service_nursery, trio_run

SERVICE_TYPE_CHOICES = {service.value: service for service in SequesterServiceType}


class SequesterCliError(Exception):
    pass


class SequesterCliWrongKeyFormatError(SequesterCliError):
    pass


def _create_encryption_key_certificate(
    raw_encryption_public_key: bytes, service_name: str
) -> bytes:
    now = pendulum.now()
    encryption_public_key = load_sequester_public_key(raw_encryption_public_key)
    try:
        key_format = EncryptionKeyFormat(
            encryption_public_key.algorithm.upper()  # type: ignore[attr-defined]
        )
    except ValueError:
        raise SequesterCliWrongKeyFormatError(
            f"Unsupported Key Format {encryption_public_key.algorithm}"  # type: ignore[attr-defined]
        )
    return SequesterServiceEncryptionKey(
        encryption_key=raw_encryption_public_key,
        encryption_key_format=key_format,
        timestamp=now,
        service_name=service_name,
    ).dump()


def _sign_encryption_key_certificate(certificate: bytes, raw_signing_key: bytes) -> bytes:
    signing_key = load_sequester_private_key(raw_signing_key)
    return sign_sequester(signing_key, certificate)


def _generate_service_request_schema(
    service_type, encryption_key_certificate, encryption_key_certificate_signature
) -> SequesterService:
    return SequesterService(
        service_type=service_type,
        service_id=uuid4(),
        sequester_encryption_certificate=encryption_key_certificate,
        sequester_encryption_certificate_signature=encryption_key_certificate_signature,
    )


async def run_memomry_sequest_component(
    config: BackendConfig, organization_str: str, register_service_req: SequesterService
):
    async with backend_app_factory(config=config) as backend:
        await backend.sequester.register_service(
            OrganizationID(organization_str), register_service_req
        )


async def run_pg_sequester_component(
    config: BackendConfig, organization_str: OrganizationID, register_service_req: SequesterService
):
    event_bus = EventBus()
    dbh = PGHandler(config.db_url, config.db_min_connections, config.db_max_connections, event_bus)
    sequester_component = PGPSequesterComponent(dbh)

    async with open_service_nursery() as nursery:
        await dbh.init(nursery)
        try:
            await sequester_component.register_service(
                OrganizationID(organization_str), register_service_req
            )
        finally:
            await dbh.teardown()


# TODO HELPERS + Update with option service_id
@click.command()
@click.option(
    "--encryption_public_key",
    type=click.Path(
        exists=True, file_okay=True, dir_okay=False, path_type=Path  # type: ignore[type-var]
    ),
)
@click.option(
    "--signing_private_key",
    type=click.Path(
        exists=True, file_okay=True, dir_okay=False, path_type=Path  # type: ignore[type-var]
    ),
)
@click.option(
    "--service_type",
    type=click.Choice(SERVICE_TYPE_CHOICES),
    required=True,
    help="New service type",
)
@click.option("--service_name", type=str, required=True, help="New service name")
@click.option("--organization", type=str)
@basic_admin_backend_options
def new_service(
    encryption_public_key: Path,
    signing_private_key: Path,
    service_type: SequesterServiceType,
    service_name: str,
    organization: str,
    db: str,
    db_max_connections: int,
    db_min_connections: int,
    administration_token: str,
):
    # Load key files
    raw_encryption_key = encryption_public_key.read_bytes()
    raw_signing_key = signing_private_key.read_bytes()
    # Generate data schema
    encryption_key_certificate = _create_encryption_key_certificate(
        raw_encryption_public_key=raw_encryption_key, service_name=service_name
    )
    # Sign data schema
    encryption_key_certificate_signature = _sign_encryption_key_certificate(
        encryption_key_certificate, raw_signing_key
    )
    # Generate register service request
    register_service_req = _generate_service_request_schema(
        service_type=service_type,
        encryption_key_certificate=encryption_key_certificate,
        encryption_key_certificate_signature=encryption_key_certificate_signature,
    )

    # Create backend basic config
    run_in_memory = db.upper() == "MOCKED"

    app_config = BackendConfig(
        administration_token=administration_token,
        db_url=db,
        db_min_connections=db_min_connections,
        db_max_connections=db_max_connections,
        blockstore_config=MockedBlockStoreConfig() if run_in_memory else BaseBlockStoreConfig(),
        ssl_context=False,
        debug=False,
        backend_addr=None,
        email_config=MockedEmailConfig(sender=DEFAULT_EMAIL_SENDER, tmpdir="/tmp"),
        forward_proto_enforce_https=None,
    )

    if run_in_memory:
        trio_run(
            run_memomry_sequest_component,
            app_config,
            organization,
            register_service_req,
            use_asyncio=True,
        )

    else:
        trio_run(
            run_pg_sequester_component,
            app_config,
            organization,
            register_service_req,
            use_asyncio=True,
        )


# TODO HELPERS + Update with option service_id
@click.command()
@click.option(
    "--encryption_public_key",
    type=click.Path(
        exists=True, file_okay=True, dir_okay=False, path_type=Path  # type: ignore[type-var]
    ),
)
@click.option(
    "--signing_private_key",
    type=click.Path(
        exists=True, file_okay=True, dir_okay=False, path_type=Path  # type: ignore[type-var]
    ),
)
@click.option(
    "--service_type",
    type=click.Choice(SERVICE_TYPE_CHOICES),
    required=True,
    help="New service type",
)
@click.option("--service_name", type=str, required=True, help="New service name")
@click.option("--organization", type=str)
@basic_admin_backend_options
def update_service():
    # TODO
    pass


@click.command()
def delete_service():
    pass
