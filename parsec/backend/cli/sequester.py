# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

import textwrap
from base64 import b64decode, b64encode
from pathlib import Path
from typing import AsyncGenerator, Dict, List, Tuple

import attr
import click
from async_generator import asynccontextmanager

from parsec._parsec import (
    CryptoError,
    DateTime,
    SequesterPrivateKeyDer,
    SequesterPublicKeyDer,
    SequesterSigningKeyDer,
    SequesterVerifyKeyDer,
)
from parsec.api.data import DataError, SequesterServiceCertificate
from parsec.api.protocol import HumanHandle, OrganizationID, RealmID, SequesterServiceID, UserID
from parsec.backend.blockstore import blockstore_factory
from parsec.backend.cli.utils import blockstore_backend_options, db_backend_options
from parsec.backend.config import BaseBlockStoreConfig
from parsec.backend.postgresql.handler import PGHandler
from parsec.backend.postgresql.organization import PGOrganizationComponent
from parsec.backend.postgresql.realm import PGRealmComponent
from parsec.backend.postgresql.sequester import PGPSequesterComponent
from parsec.backend.postgresql.sequester_export import RealmExporter
from parsec.backend.postgresql.user import PGUserComponent
from parsec.backend.realm import RealmGrantedRole
from parsec.backend.sequester import (
    BaseSequesterService,
    SequesterServiceType,
    StorageSequesterService,
    WebhookSequesterService,
)
from parsec.backend.user import User
from parsec.cli_utils import cli_exception_handler, debug_config_options, operation
from parsec.event_bus import EventBus
from parsec.sequester_export_reader import RealmExportProgress, extract_workspace
from parsec.utils import open_service_nursery, trio_run

SEQUESTER_SERVICE_CERTIFICATE_PEM_HEADER = "-----BEGIN PARSEC SEQUESTER SERVICE CERTIFICATE-----"
SEQUESTER_SERVICE_CERTIFICATE_PEM_FOOTER = "-----END PARSEC SEQUESTER SERVICE CERTIFICATE-----"


def dump_sequester_service_certificate_pem(
    certificate_data: SequesterServiceCertificate,
    authority_signing_key: SequesterSigningKeyDer,
) -> str:
    certificate = authority_signing_key.sign(certificate_data.dump())
    return "\n".join(
        (
            SEQUESTER_SERVICE_CERTIFICATE_PEM_HEADER,
            *textwrap.wrap(b64encode(certificate).decode(), width=64),
            SEQUESTER_SERVICE_CERTIFICATE_PEM_FOOTER,
            "",
        )
    )


def load_sequester_service_certificate_pem(
    data: str, authority_verify_key: SequesterVerifyKeyDer
) -> Tuple[SequesterServiceCertificate, bytes]:
    err_msg = "Not a valid Parsec sequester service certificate PEM file"
    try:
        header, *content, footer = data.strip().splitlines()
    except ValueError as exc:
        raise ValueError(err_msg) from exc

    if header != SEQUESTER_SERVICE_CERTIFICATE_PEM_HEADER:
        raise ValueError(f"{err_msg}: missing `{SEQUESTER_SERVICE_CERTIFICATE_PEM_HEADER}` header")
    if footer != SEQUESTER_SERVICE_CERTIFICATE_PEM_FOOTER:
        raise ValueError(f"{err_msg}: missing `{SEQUESTER_SERVICE_CERTIFICATE_PEM_FOOTER}` footer")

    try:
        certificate = b64decode("".join(content))
        return (
            SequesterServiceCertificate.load(authority_verify_key.verify(certificate)),
            certificate,
        )
    except (ValueError, DataError, CryptoError) as exc:
        raise ValueError(f"{err_msg}: invalid body ({exc})") from exc


class SequesterBackendCliError(Exception):
    pass


SERVICE_TYPE_CHOICES: Dict[str, SequesterServiceType] = {
    service.value: service for service in SequesterServiceType
}


@attr.s(slots=True, frozen=True, auto_attribs=True)
class BackendDbConfig:
    db_url: str
    db_min_connections: int
    db_max_connections: int


@asynccontextmanager
async def run_pg_db_handler(config: BackendDbConfig) -> AsyncGenerator[PGHandler, None]:
    event_bus = EventBus()
    dbh = PGHandler(config.db_url, config.db_min_connections, config.db_max_connections, event_bus)

    async with open_service_nursery() as nursery:
        await dbh.init(nursery)
        try:
            yield dbh
        finally:
            await dbh.teardown()


async def _create_service(
    config: BackendDbConfig,
    organization_id: OrganizationID,
    register_service_req: BaseSequesterService,
) -> None:
    async with run_pg_db_handler(config) as dbh:
        sequester_component = PGPSequesterComponent(dbh)
        await sequester_component.create_service(organization_id, register_service_req)


def _display_service(service: BaseSequesterService) -> None:
    display_service_id = click.style(service.service_id.hex, fg="yellow")
    display_service_label = click.style(service.service_label, fg="green")
    click.echo(f"Service {display_service_label} (id: {display_service_id})")
    click.echo(f"\tCreated on: {service.created_on}")
    click.echo(f"\tService type: {service.service_type}")
    if isinstance(service, WebhookSequesterService):
        click.echo(f"\tWebhook endpoint URL {service.webhook_url}")
    if not service.is_enabled:
        display_disable = click.style("Disabled", fg="red")
        click.echo(f"\t{display_disable} on: {service.disabled_on}")


async def _list_services(config: BackendDbConfig, organization_id: OrganizationID) -> None:
    async with run_pg_db_handler(config) as dbh:
        sequester_component = PGPSequesterComponent(dbh)
        services = await sequester_component.get_organization_services(organization_id)

    display_services_count = click.style(len(services), fg="green")
    click.echo(f"Found {display_services_count} sequester service(s)")

    # Display enabled services first
    for service in services:
        if service.is_enabled:
            _display_service(service)
    for service in services:
        if not service.is_enabled:
            _display_service(service)


async def _disable_service(
    config: BackendDbConfig, organization_id: OrganizationID, service_id: SequesterServiceID
) -> None:
    async with run_pg_db_handler(config) as dbh:
        sequester_component = PGPSequesterComponent(dbh)
        await sequester_component.disable_service(organization_id, service_id)


async def _enable_service(
    config: BackendDbConfig, organization_id: OrganizationID, service_id: SequesterServiceID
) -> None:
    async with run_pg_db_handler(config) as dbh:
        sequester_component = PGPSequesterComponent(dbh)
        await sequester_component.enable_service(organization_id, service_id)


def _get_config(db: str, db_min_connections: int, db_max_connections: int) -> BackendDbConfig:
    if db.upper() == "MOCKED":
        raise SequesterBackendCliError("MOCKED DB can not be used with sequester services")

    return BackendDbConfig(
        db_url=db, db_min_connections=db_min_connections, db_max_connections=db_max_connections
    )


@click.command(short_help="Generate a certificate for a new sequester service")
@click.option("--service-label", type=str, help="New service name", required=True)
@click.option(
    "--service-public-key",
    help="The service encryption public key used to encrypt data to the sequester service",
    type=click.Path(exists=True, file_okay=True, dir_okay=False, path_type=Path),
    required=True,
)
@click.option(
    "--authority-private-key",
    help="The private authority key use. Used to sign the encryption key.",
    type=click.Path(exists=True, file_okay=True, dir_okay=False, path_type=Path),
    required=True,
)
@click.option(
    "--output",
    "-o",
    help="File to write the sequester service certificate into.",
    type=click.File("w", encoding="utf8"),
    required=True,
    metavar="CERTIFICATE.pem",
)
# Add --debug
@debug_config_options
def generate_service_certificate(
    service_label: str,
    service_public_key: Path,
    authority_private_key: Path,
    output: click.utils.LazyFile,
    debug: bool,
) -> None:
    with cli_exception_handler(debug):
        # Load key files
        service_key = SequesterPublicKeyDer.load_pem(service_public_key.read_text())
        authority_key = SequesterSigningKeyDer.load_pem(authority_private_key.read_text())

        # Generate data schema
        service_id = SequesterServiceID.new()
        now = DateTime.now()
        certificate_data = SequesterServiceCertificate(
            timestamp=now,
            service_id=service_id,
            service_label=service_label,
            encryption_key_der=service_key,
        )

        # Write the output file
        pem_content = dump_sequester_service_certificate_pem(
            certificate_data=certificate_data,
            authority_signing_key=authority_key,
        )
        output.write(pem_content)

        display_service = f"{click.style(service_label, fg='yellow')} (id: {click.style(service_id.hex, fg='yellow')})"
        display_file = click.style(output.name, fg="green")
        click.echo(f"Sequester service certificate {display_service} exported in {display_file}")
        click.echo(
            f"Use {click.style('import_service_certificate', fg='yellow')} command to add it to an organization"
        )


@click.command(short_help="Register a new sequester service from it existing certificate")
@click.option(
    "--service-certificate",
    help="File containing the sequester service certificate (previously generated by `generate_service_certificate` command).",
    type=click.File("r", encoding="utf8"),
    required=True,
    metavar="CERTIFICATE.pem",
)
@click.option(
    "--organization",
    type=OrganizationID,
    help="Organization ID where to register the service",
    required=True,
)
@click.option(
    "--service-type",
    type=click.Choice(list(SERVICE_TYPE_CHOICES.keys()), case_sensitive=False),
    default=SequesterServiceType.STORAGE.value,
    help="Service type",
)
@click.option(
    "--webhook-url",
    type=str,
    default=None,
    help="[Service Type webhook only] webhook url used to send encrypted service data",
)
@db_backend_options
# Add --debug
@debug_config_options
def import_service_certificate(
    service_certificate: click.utils.LazyFile,
    organization: OrganizationID,
    db: str,
    db_max_connections: int,
    db_min_connections: int,
    service_type: str,
    webhook_url: str | None,
    debug: bool,
) -> None:
    with cli_exception_handler(debug):
        cooked_service_type: SequesterServiceType = SERVICE_TYPE_CHOICES[service_type]
        # Check service type
        if webhook_url is not None and cooked_service_type != SequesterServiceType.WEBHOOK:
            raise SequesterBackendCliError(
                f"Incompatible service type {cooked_service_type} with webhook_url option\nwebhook_url can only be used with {SequesterServiceType.WEBHOOK}."
            )
        if cooked_service_type == SequesterServiceType.WEBHOOK and not webhook_url:
            raise SequesterBackendCliError(
                "Webhook sequester service requires webhook_url argument"
            )

        db_config = _get_config(db, db_min_connections, db_max_connections)
        service_certificate_pem = service_certificate.read()

        trio_run(
            _import_service_certificate,
            db_config,
            organization,
            service_certificate_pem,
            cooked_service_type,
            webhook_url,
            use_asyncio=True,
        )
        click.echo(click.style("Service created", fg="green"))


async def _import_service_certificate(
    db_config: BackendDbConfig,
    organization_id: OrganizationID,
    service_certificate_pem: str,
    service_type: SequesterServiceType,
    webhook_url: None | None,
) -> None:
    async with run_pg_db_handler(db_config) as dbh:
        # 1) Retrieve the sequester authority verify key and check organization is compatible

        async with dbh.pool.acquire() as conn:
            organization = await PGOrganizationComponent._get(conn, id=organization_id)

        if not organization.is_bootstrapped():
            raise RuntimeError("Organization is not bootstrapped, aborting.")

        if organization.sequester_authority is None:
            raise RuntimeError("Organization doesn't support sequester, aborting.")

        # 2) Validate the certificate

        (
            service_certificate_data,
            service_certificate_content,
        ) = load_sequester_service_certificate_pem(
            data=service_certificate_pem,
            authority_verify_key=organization.sequester_authority.verify_key_der,
        )

        # 3) Insert the certificate

        service: BaseSequesterService
        if service_type == SequesterServiceType.STORAGE:
            assert webhook_url is None
            service = StorageSequesterService(
                service_id=service_certificate_data.service_id,
                service_label=service_certificate_data.service_label,
                service_certificate=service_certificate_content,
                created_on=service_certificate_data.timestamp,
            )
        else:
            assert service_type == SequesterServiceType.WEBHOOK
            assert webhook_url
            # Removing the extra slash if present to avoid a useless redirection
            service = WebhookSequesterService(
                service_id=service_certificate_data.service_id,
                service_label=service_certificate_data.service_label,
                service_certificate=service_certificate_content,
                created_on=service_certificate_data.timestamp,
                webhook_url=webhook_url,
            )

        sequester_component = PGPSequesterComponent(dbh)
        await sequester_component.create_service(organization_id, service)


@click.command(short_help="Register a new sequester service")
@click.option(
    "--service-public-key",
    help="The service encryption public key used to encrypt data to the sequester service",
    type=click.Path(exists=True, file_okay=True, dir_okay=False, path_type=Path),
    required=True,
)
@click.option(
    "--authority-private-key",
    help="The private authority key use. Used to sign the encryption key.",
    type=click.Path(exists=True, file_okay=True, dir_okay=False, path_type=Path),
    required=True,
)
@click.option("--service-label", type=str, help="New service name", required=True)
@click.option(
    "--organization",
    type=OrganizationID,
    help="Organization ID where to register the service",
    required=True,
)
@click.option(
    "--service-type",
    type=click.Choice(list(SERVICE_TYPE_CHOICES.keys()), case_sensitive=False),
    default=SequesterServiceType.STORAGE.value,
    help="Service type",
)
@click.option(
    "--webhook-url",
    type=str,
    default=None,
    help="[Service Type webhook only] webhook url used to send encrypted service data",
)
@db_backend_options
# Add --debug
@debug_config_options
def create_service(
    service_public_key: Path,
    authority_private_key: Path,
    service_label: str,
    organization: OrganizationID,
    db: str,
    db_max_connections: int,
    db_min_connections: int,
    service_type: str,
    webhook_url: str | None,
    debug: bool,
) -> None:
    with cli_exception_handler(debug):
        cooked_service_type: SequesterServiceType = SERVICE_TYPE_CHOICES[service_type]
        # Check service type
        if webhook_url is not None and cooked_service_type != SequesterServiceType.WEBHOOK:
            raise SequesterBackendCliError(
                f"Incompatible service type {cooked_service_type} with webhook_url option\nwebhook_url can only be used with {SequesterServiceType.WEBHOOK}."
            )
        if cooked_service_type == SequesterServiceType.WEBHOOK and not webhook_url:
            raise SequesterBackendCliError(
                "Webhook sequester service requires webhook_url argument"
            )
        # Load key files
        service_key = SequesterPublicKeyDer.load_pem(service_public_key.read_text())
        authority_key = SequesterSigningKeyDer.load_pem(authority_private_key.read_text())
        # Generate data schema
        service_id = SequesterServiceID.new()
        now = DateTime.now()
        certif_data = SequesterServiceCertificate(
            timestamp=now,
            service_id=service_id,
            service_label=service_label,
            encryption_key_der=service_key,
        )
        certificate = authority_key.sign(certif_data.dump())

        sequester_service: BaseSequesterService
        if cooked_service_type == SequesterServiceType.STORAGE:
            assert webhook_url is None
            sequester_service = StorageSequesterService(
                service_id=service_id,
                service_label=service_label,
                service_certificate=certificate,
                created_on=now,
            )
        else:
            assert cooked_service_type == SequesterServiceType.WEBHOOK
            assert webhook_url
            # Removing the extra slash if present to avoid a useless redirection
            sequester_service = WebhookSequesterService(
                service_id=service_id,
                service_label=service_label,
                service_certificate=certificate,
                created_on=now,
                webhook_url=webhook_url,
            )

        db_config = _get_config(db, db_min_connections, db_max_connections)

        trio_run(_create_service, db_config, organization, sequester_service, use_asyncio=True)
        click.echo(click.style("Service created", fg="green"))


@click.command(short_help="List available sequester services")
@click.option("--organization", type=OrganizationID, help="Organization ID", required=True)
@db_backend_options
def list_services(
    organization: OrganizationID, db: str, db_max_connections: int, db_min_connections: int
) -> None:
    db_config = _get_config(db, db_min_connections, db_max_connections)
    trio_run(_list_services, db_config, organization, use_asyncio=True)


@click.command(short_help="Disable/re-enable a sequester service")
@click.option("--organization", type=OrganizationID, help="Organization ID", required=True)
@click.option(
    "--service",
    type=SequesterServiceID.from_hex,
    help="ID of the sequester service to update",
    required=True,
)
@click.option("--enable", is_flag=True, help="Enable service")
@click.option("--disable", is_flag=True, help="Disable service")
@db_backend_options
# Add --debug
@debug_config_options
def update_service(
    organization: OrganizationID,
    service: SequesterServiceID,
    db: str,
    db_max_connections: int,
    db_min_connections: int,
    enable: bool,
    disable: bool,
    debug: bool,
) -> None:
    with cli_exception_handler(debug):
        if enable and disable:
            raise click.BadParameter("Enable and disable flags are both set")
        if not enable and not disable:
            raise click.BadParameter("Required: enable or disable flag")
        db_config = _get_config(db, db_min_connections, db_max_connections)
        if disable:
            trio_run(_disable_service, db_config, organization, service, use_asyncio=True)
        if enable:
            trio_run(_enable_service, db_config, organization, service, use_asyncio=True)
        click.echo(click.style("Service updated", fg="green"))


async def _human_accesses(
    config: BackendDbConfig, organization: OrganizationID, user_filter: str
) -> None:
    async with run_pg_db_handler(config) as dbh:
        user_component = PGUserComponent(event_bus=dbh.event_bus, dbh=dbh)
        realm_component = PGRealmComponent(dbh)

        users, _ = await user_component.dump_users(organization_id=organization)
        if user_filter:
            # Now is a good time to filter out
            filter_split = user_filter.split()
            filtered_users = []
            for user in users:
                txt = f"{user.human_handle.str if user.human_handle else ''} {user.user_id.str}".lower()
                if len([True for sq in filter_split if sq in txt]) == len(filter_split):
                    filtered_users.append(user)
            users = filtered_users

        realms_granted_roles = await realm_component.dump_realms_granted_roles(
            organization_id=organization
        )
        per_user_granted_roles: Dict[UserID, List[RealmGrantedRole]] = {}
        for granted_role in realms_granted_roles:
            user_granted_roles = per_user_granted_roles.setdefault(granted_role.user_id, [])
            user_granted_roles.append(granted_role)

        humans: Dict[
            HumanHandle | None, List[Tuple[User, Dict[RealmID, List[RealmGrantedRole]]]]
        ] = {
            None: []
        }  # Non-human are all stored in `None` key
        for user in users:
            human_users = humans.setdefault(user.human_handle, [])
            per_user_per_realm_granted_role: Dict[RealmID, List[RealmGrantedRole]] = {}
            for granted_role in per_user_granted_roles.get(user.user_id, []):
                realm_granted_roles = per_user_per_realm_granted_role.setdefault(
                    granted_role.realm_id, []
                )
                realm_granted_roles.append(granted_role)

            for realm_granted_roles in per_user_per_realm_granted_role.values():
                realm_granted_roles.sort(key=lambda x: x.granted_on)

            human_users.append((user, per_user_per_realm_granted_role))

        # Typical output to display:
        #
        # Found 2 results:
        # Human John Doe <john.doe@example.com>
        #
        #   User 9e082a43b51e44ab9858628bac4a61d9 (ADMIN, created on 2000-01-02T00:00:00Z)
        #
        #     Realm 8006a491f0704040ae9a197ca7501f71
        #       2000-01-04T00:00:00Z: Access OWNER granted
        #       2000-01-03T00:00:00Z: Access removed
        #       2000-01-02T00:00:00Z: Access READER granted
        #
        #     Realm 109c48b7c931435c913945f08d23432d
        #       2000-01-01T00:00:00Z: Access OWNER granted
        #
        #   User 02e0486752d34d6ab3bf8e0befef1935 (STANDARD, created on 2000-01-01T00:00:00Z, revoked on 2000-01-02T00:00:00Z)
        #
        # Human Jane Doe <jane.doe@example.com>
        #
        #   User baf59386baf740bba93151cdde1beac8 (OUTSIDER, created on 2000-01-01T00:00:00Z)
        #
        #     Realm 8006a491f0704040ae9a197ca7501f71
        #       2000-01-01T00:00:00Z: Access READER granted

        def _display_user(
            user: User, per_realm_granted_role: dict[RealmID, list[RealmGrantedRole]], indent: int
        ) -> None:
            base_indent = "\t" * indent
            display_user = click.style(user.user_id, fg="green")
            user_info = f"{user.profile}, created on {user.created_on}"
            if user.revoked_on:
                user_info += f", revoked on {user.revoked_on}"
            print(base_indent + f"User {display_user} ({user_info})")
            for realm_id, granted_roles in per_realm_granted_role.items():
                display_realm = click.style(realm_id.hex, fg="yellow")
                print(base_indent + f"\tRealm {display_realm}")
                for granted_role in granted_roles:
                    if granted_role.role is None:
                        display_role = "Access removed"
                    else:
                        display_role = f"Access {granted_role.role.str} granted"
                    print(base_indent + f"\t\t{granted_role.granted_on}: {display_role}")

        non_human_users = humans.pop(None)
        print(f"Found {len(humans)} result(s)")

        for human_handle, human_users in humans.items():
            display_human = click.style(human_handle, fg="green")
            print(f"Human {display_human}")
            for user, per_realm_granted_roles in human_users:
                _display_user(user, per_realm_granted_roles, indent=1)
            print()

        for user, per_realm_granted_roles in non_human_users:
            _display_user(user, per_realm_granted_roles, indent=0)


@click.command(short_help="Get information about user&realm accesses")
@click.option("--organization", type=OrganizationID, required=True)
@click.option("--filter", type=str, default="")
@db_backend_options
def human_accesses(
    filter: str,
    organization: OrganizationID,
    db: str,
    db_max_connections: int,
    db_min_connections: int,
) -> None:
    db_config = _get_config(db, db_min_connections, db_max_connections)
    trio_run(_human_accesses, db_config, organization, filter, use_asyncio=True)


async def _export_realm(
    db_config: BackendDbConfig,
    blockstore_config: BaseBlockStoreConfig,
    organization_id: OrganizationID,
    realm_id: RealmID,
    service_id: SequesterServiceID,
    output: Path,
) -> None:
    if output.is_dir():
        # Output is pointing to a directory, use a default name for the database extract
        output_db_path = output / f"parsec-sequester-export-realm-{realm_id.hex}.sqlite"
    else:
        output_db_path = output

    output_db_display = click.style(str(output_db_path), fg="green")
    if output.exists():
        click.echo(
            f"File {output_db_display} already exists, continue the extract from where it was left"
        )
    else:
        click.echo(f"Creating {output_db_display}")

    click.echo(
        f"Use { click.style('^C', fg='yellow') } to stop the export,"
        " progress won't be lost when restarting the command"
    )

    async with run_pg_db_handler(db_config) as dbh:
        blockstore_component = blockstore_factory(config=blockstore_config, postgresql_dbh=dbh)

        async with RealmExporter.run(
            organization_id=organization_id,
            realm_id=realm_id,
            service_id=service_id,
            output_db_path=output_db_path,
            input_dbh=dbh,
            input_blockstore=blockstore_component,
        ) as exporter:
            # 1) Export vlobs

            with operation("Computing vlobs (i.e. file/folder metadata) to export"):
                (
                    vlob_total_count,
                    vlob_batch_offset_marker,
                ) = await exporter.compute_vlobs_export_status()

            if not vlob_total_count:
                click.echo("No more vlobs to export !")
            else:
                vlob_total_count_display = click.style(str(vlob_total_count), fg="green")
                click.echo(f"About {vlob_total_count_display} vlobs need to be exported")
                with click.progressbar(length=vlob_total_count, label="Exporting vlobs") as bar:
                    vlobs_exported_count = 0
                    vlob_batch_size = 1000
                    while True:
                        new_vlob_batch_offset_marker = await exporter.export_vlobs(
                            batch_size=vlob_batch_size, batch_offset_marker=vlob_batch_offset_marker
                        )
                        if new_vlob_batch_offset_marker <= vlob_batch_offset_marker:
                            break
                        vlob_batch_offset_marker = new_vlob_batch_offset_marker
                        # Note we might end up with vlobs_exported_count > vlob_total_count
                        # in case additional vlobs are created during the export, this is no
                        # big deal though (progress bar will stay at 100%)
                        bar.update(vlobs_exported_count)

            # Export blocks

            with operation("Computing blocks (i.e. files data) to export"):
                (
                    block_total_count,
                    block_batch_offset_marker,
                ) = await exporter.compute_blocks_export_status()

            if not block_total_count:
                click.echo("No more blocks to export !")
            else:
                block_total_count_display = click.style(str(block_total_count), fg="green")

                click.echo(f"About {block_total_count_display} blocks need to be exported")
                with click.progressbar(length=block_total_count, label="Exporting blocks") as bar:
                    blocks_exported_count = 0
                    block_batch_size = 100
                    while True:
                        new_block_batch_offset_marker = await exporter.export_blocks(
                            batch_size=block_batch_size,
                            batch_offset_marker=block_batch_offset_marker,
                        )
                        if new_block_batch_offset_marker <= block_batch_offset_marker:
                            break
                        block_batch_offset_marker = new_block_batch_offset_marker
                        # Note we might end up with blocks_exported_count > block_total_count
                        # in case additional blocks are created during the export, this is no
                        # big deal though (progress bar will stay at 100%)
                        bar.update(blocks_exported_count)


@click.command(short_help="Export a realm to consult it with a sequester service key")
@click.option("--organization", type=OrganizationID, required=True)
@click.option("--realm", type=RealmID.from_hex, required=True)
@click.option(
    "--service",
    type=SequesterServiceID.from_hex,
    help="ID of the sequester service to retrieve data from",
    required=True,
)
@click.option("--output", type=Path, required=True)
@db_backend_options
@blockstore_backend_options
# Add --debug
@debug_config_options
def export_realm(
    organization: OrganizationID,
    realm: RealmID,
    service: SequesterServiceID,
    output: Path,
    db: str,
    db_max_connections: int,
    db_min_connections: int,
    blockstore: BaseBlockStoreConfig,
    debug: bool,
) -> None:
    with cli_exception_handler(debug):
        db_config = _get_config(db, db_min_connections, db_max_connections)
        trio_run(
            _export_realm,
            db_config,
            blockstore,
            organization,
            realm,
            service,
            output,
            use_asyncio=True,
        )


@click.command(short_help="Open a realm export using the sequester service key and dump it content")
@click.option(
    "--service-decryption-key",
    help="Decryption key of the sequester service that have been use to create the realm export archive",
    type=click.Path(exists=True, file_okay=True, dir_okay=False, path_type=Path),
    required=True,
)
@click.option("--input", type=Path, required=True, help="Realm export archive")
@click.option(
    "--output", type=Path, required=True, help="Directory where to dump the content of the realm"
)
@click.option(
    "--filter-date",
    type=click.DateTime(formats=["%Y-%m-%d"]),
    required=False,
    help="Extract at a specific date; format year-month-day",
)
# Add --debug
@debug_config_options
def extract_realm_export(
    service_decryption_key: Path,
    input: Path,
    output: Path,
    filter_date: click.DateTime,
    debug: bool,
) -> int:
    with cli_exception_handler(debug):
        # Finally a command that is not async !
        # This is because here we do only a single thing at a time and sqlite3 provide
        # a synchronous api anyway
        decryption_key = SequesterPrivateKeyDer.load_pem(service_decryption_key.read_text())

        # Convert filter_date from click.Datetime to parsec.Datetime
        date: DateTime
        if filter_date:
            date = DateTime.from_timestamp(date.timestamp())
        else:
            date = DateTime.now()
        ret = 0
        for fs_path, event_type, event_msg in extract_workspace(
            output=output, export_db=input, decryption_key=decryption_key, filter_on_date=date
        ):
            if event_type == RealmExportProgress.EXTRACT_IN_PROGRESS:
                fs_path_display = click.style(str(fs_path), fg="yellow")
                click.echo(f"{ fs_path_display }: { event_msg }")
            else:
                # Error
                ret = 1
                fs_path_display = click.style(str(fs_path), fg="red")
                click.echo(f"{ fs_path_display }: { event_type.value } { event_msg }")

        return ret
