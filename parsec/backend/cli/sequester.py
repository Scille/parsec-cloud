# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from uuid import uuid4
from async_generator import asynccontextmanager
import attr
import click
from parsec._parsec import DateTime
from typing import Optional, Dict, List, Tuple
from pathlib import Path
import oscrypto.asymmetric

from parsec.event_bus import EventBus
from parsec.utils import open_service_nursery, trio_run
from parsec.cli_utils import operation
from parsec.sequester_crypto import sequester_authority_sign, SequesterEncryptionKeyDer
from parsec.sequester_export_reader import extract_workspace, RealmExportProgress
from parsec.api.data import SequesterServiceCertificate
from parsec.api.protocol import OrganizationID, UserID, RealmID, SequesterServiceID, HumanHandle
from parsec.backend.cli.utils import db_backend_options, blockstore_backend_options
from parsec.backend.config import BaseBlockStoreConfig
from parsec.backend.realm import RealmGrantedRole
from parsec.backend.user import User
from parsec.backend.sequester import SequesterService
from parsec.backend.blockstore import blockstore_factory
from parsec.backend.postgresql.handler import PGHandler
from parsec.backend.postgresql.sequester import PGPSequesterComponent
from parsec.backend.postgresql.sequester_export import RealmExporter
from parsec.backend.postgresql.user import PGUserComponent
from parsec.backend.postgresql.realm import PGRealmComponent


class SequesterBackendCliError(Exception):
    pass


@attr.s(slots=True, frozen=True, auto_attribs=True)
class BackendDbConfig:
    db_url: str
    db_min_connections: int
    db_max_connections: int


@asynccontextmanager
async def run_pg_db_handler(config: BackendDbConfig):
    event_bus = EventBus()
    dbh = PGHandler(config.db_url, config.db_min_connections, config.db_max_connections, event_bus)

    async with open_service_nursery() as nursery:
        await dbh.init(nursery)
        try:
            yield dbh
        finally:
            await dbh.teardown()


async def _create_service(
    config: BackendDbConfig, organization_id: OrganizationID, register_service_req: SequesterService
) -> None:
    async with run_pg_db_handler(config) as dbh:
        sequester_component = PGPSequesterComponent(dbh)
        await sequester_component.create_service(organization_id, register_service_req)


def _display_service(service: SequesterService) -> None:
    display_service_id = click.style(service.service_id, fg="yellow")
    display_service_label = click.style(service.service_label, fg="green")
    click.echo(f"Service {display_service_label} (id: {display_service_id})")
    click.echo(f"\tCreated on: {service.created_on}")
    if not service.is_enabled:
        click.echo(f"\tDisabled on: {service.disabled_on}")


async def _list_services(config, organization_id: OrganizationID) -> None:
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
    config, organizaton_id: OrganizationID, service_id: SequesterServiceID
) -> None:
    async with run_pg_db_handler(config) as dbh:
        sequester_component = PGPSequesterComponent(dbh)
        await sequester_component.disable_service(organizaton_id, service_id)


async def _enable_service(
    config, organizaton_id: OrganizationID, service_id: SequesterServiceID
) -> None:
    async with run_pg_db_handler(config) as dbh:
        sequester_component = PGPSequesterComponent(dbh)
        await sequester_component.enable_service(organizaton_id, service_id)


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
@db_backend_options
def create_service(
    service_public_key: Path,
    authority_private_key: Path,
    service_label: str,
    organization: OrganizationID,
    db: str,
    db_max_connections: int,
    db_min_connections: int,
):
    # Load key files
    service_key = SequesterEncryptionKeyDer(service_public_key.read_bytes())
    authority_key = oscrypto.asymmetric.load_private_key(authority_private_key.read_bytes())
    # Generate data schema
    service_id = SequesterServiceID(uuid4())
    now = DateTime.now()
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
    click.echo(click.style("Service created", fg="green"))


@click.command(short_help="List available sequester services")
@click.option("--organization", type=OrganizationID, help="Organization ID", required=True)
@db_backend_options
def list_services(
    organization: OrganizationID, db: str, db_max_connections: int, db_min_connections: int
):
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
def update_service(
    organization: OrganizationID,
    service: SequesterServiceID,
    db: str,
    db_max_connections: int,
    db_min_connections: int,
    enable: bool,
    disable: bool,
):
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
            Optional[HumanHandle], List[Tuple[User, Dict[RealmID, List[RealmGrantedRole]]]]
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

        # Typicall output to dislay:
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

        def _display_user(user, per_realm_granted_role, indent):
            base_indent = "\t" * indent
            display_user = click.style(user.user_id, fg="green")
            user_info = f"{user.profile}, created on {user.created_on}"
            if user.revoked_on:
                user_info += f", revoked on {user.revoked_on}"
            print(base_indent + f"User {display_user} ({user_info})")
            for realm_id, granted_roles in per_realm_granted_role.items():
                display_realm = click.style(str(realm_id), fg="yellow")
                print(base_indent + f"\tRealm {display_realm}")
                for granted_role in granted_roles:
                    if granted_role.role is None:
                        display_role = "Access removed"
                    else:
                        display_role = f"Access {granted_role.role.value} granted"
                    print(base_indent + f"\t\t{granted_role.granted_on}: {display_role}")

        non_human_users = humans.pop(None)
        print(f"Found {len(humans)} result(s)")

        for human_handle, human_users in humans.items():
            display_human = click.style(human_handle, fg="green")
            print(f"Human {display_human}")
            for (user, per_realm_granted_roles) in human_users:
                _display_user(user, per_realm_granted_roles, indent=1)
            print()

        for user, per_realm_granted_roles in non_human_users:
            _display_user(user.user_id, per_realm_granted_roles, indent=0)


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
):
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
        output_db_path = output / f"parsec-sequester-export-realm-{realm_id.str}.sqlite"
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
    help="ID of the sequester service to retreive data from",
    required=True,
)
@click.option("--output", type=Path, required=True)
@db_backend_options
@blockstore_backend_options
def export_realm(
    organization: OrganizationID,
    realm: RealmID,
    service: SequesterServiceID,
    output: Path,
    db: str,
    db_max_connections: int,
    db_min_connections: int,
    blockstore: BaseBlockStoreConfig,
):
    db_config = _get_config(db, db_min_connections, db_max_connections)
    trio_run(
        _export_realm, db_config, blockstore, organization, realm, service, output, use_asyncio=True
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
def extract_realm_export(service_decryption_key: Path, input: Path, output: Path):
    # Finally a command that is not async !
    # This is because here we do only a single thing at a time and sqlite3 provide
    # a synchronous api anyway
    decryption_key = oscrypto.asymmetric.load_private_key(service_decryption_key.read_bytes())

    ret = 0
    for fs_path, event_type, event_msg in extract_workspace(
        output=output, export_db=input, decryption_key=decryption_key
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
