# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

from uuid import uuid4
from async_generator import asynccontextmanager
import attr
import click
import pendulum
from typing import Optional, Dict, List, Tuple
from pathlib import Path
import oscrypto.asymmetric
from parsec.api.protocol.types import HumanHandle

from parsec.event_bus import EventBus
from parsec.utils import open_service_nursery, trio_run
from parsec.sequester_crypto import sequester_authority_sign
from parsec.sequester_crypto import SequesterEncryptionKeyDer
from parsec.api.data.certif import SequesterServiceCertificate
from parsec.api.protocol import OrganizationID, UserID, RealmID, SequesterServiceID
from parsec.backend.cli.utils import db_backend_options
from parsec.backend.realm import RealmGrantedRole
from parsec.backend.user import User
from parsec.backend.sequester import SequesterService
from parsec.backend.postgresql.handler import PGHandler
from parsec.backend.postgresql.sequester import PGPSequesterComponent
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
    config: BackendDbConfig, organization_str: str, register_service_req: SequesterService
):
    async with run_pg_db_handler(config) as dbh:
        sequester_component = PGPSequesterComponent(dbh)
        await sequester_component.create_service(
            OrganizationID(organization_str), register_service_req
        )


def display_service(service: SequesterService):
    click.echo(f"Service ID :: {service.service_id}")
    click.echo(f"Service label :: {service.service_label}")
    click.echo(f"Service creation date :: {service.created_on}")
    if service.deleted_on:
        click.echo(f"Service is disabled since {service.deleted_on}")
    click.echo("")


async def _list_services(config, organization_str: str):
    async with run_pg_db_handler(config) as dbh:
        sequester_component = PGPSequesterComponent(dbh)
        services = await sequester_component.get_organization_services(
            OrganizationID(organization_str)
        )
    if services:
        click.echo("=== Avaliable Services ===")
        for service in services:
            if not service.deleted_on:
                display_service(service)

        click.echo("=== Disabled Services ===")
        for service in services:
            if service.deleted_on:
                display_service(service)
    else:
        click.echo("No service configured")


async def _delete_service(config, organizaton_str: str, service_id: str):
    async with run_pg_db_handler(config) as dbh:
        sequester_component = PGPSequesterComponent(dbh)
        await sequester_component.delete_service(
            OrganizationID(organizaton_str), SequesterServiceID.from_hex(service_id)
        )


async def _enable_service(config, organizaton_str: str, service_id: str):
    async with run_pg_db_handler(config) as dbh:
        sequester_component = PGPSequesterComponent(dbh)
        await sequester_component.enable_service(
            OrganizationID(organizaton_str), SequesterServiceID.from_hex(service_id)
        )


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
)
@click.option(
    "--authority-private-key",
    help="The private authority key use. Used to sign the encryption key.",
    type=click.Path(exists=True, file_okay=True, dir_okay=False, path_type=Path),
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


@click.command(short_help="List availlable sequester services")
@click.option("--organization", type=str, help="Organization ID")
@db_backend_options
def list_services(organization: str, db: str, db_max_connections: int, db_min_connections: int):
    db_config = _get_config(db, db_min_connections, db_max_connections)
    trio_run(_list_services, db_config, organization, use_asyncio=True)


@click.command(short_help="Enable/disable service")
@click.option("--organization", type=str, help="Organization ID")
@click.option("--service-id", type=str, help="Service ID")
@click.option("--enable", is_flag=True, help="Enable service")
@click.option("--disable", is_flag=True, help="Disable service")
@db_backend_options
def update_service(
    organization: str,
    service_id: str,
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
        trio_run(_delete_service, db_config, organization, service_id, use_asyncio=True)
    if enable:
        trio_run(_enable_service, db_config, organization, service_id, use_asyncio=True)


async def _human_accesses(config: BackendDbConfig, organization: OrganizationID, user_filter: str):
    async with run_pg_db_handler(config) as dbh:
        user_component = PGUserComponent(event_bus=dbh.event_bus, dbh=dbh)
        realm_component = PGRealmComponent(dbh)

        users, _ = await user_component.dump_users(organization_id=organization)
        if user_filter:
            # Now is a good time to filter out
            filter_split = user_filter.split()
            filtered_users = []
            for user in users:
                txt = f"{user.human_handle if user.human_handle else ''} {user.user_id}".lower()
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
                        display_role = f"Access {granted_role.role} granted"
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


@click.command()
@click.argument("organization", type=OrganizationID)
@click.option("--filter", type=str, default="")
@db_backend_options
def human_accesses(
    filter: str, organization: str, db: str, db_max_connections: int, db_min_connections: int
):
    db_config = _get_config(db, db_min_connections, db_max_connections)
    trio_run(_human_accesses, db_config, organization, filter, use_asyncio=True)
