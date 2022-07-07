# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

from uuid import uuid4
from async_generator import asynccontextmanager
import attr
import click
import pendulum
from pathlib import Path
import oscrypto.asymmetric
from parsec import event_bus
from parsec.core.cli.utils import F

from parsec.event_bus import EventBus
from parsec.utils import open_service_nursery, trio_run
from parsec.sequester_crypto import sequester_authority_sign
from parsec.sequester_crypto import SequesterEncryptionKeyDer
from parsec.api.data.certif import SequesterServiceCertificate
from parsec.api.protocol.sequester import SequesterServiceID
from parsec.api.protocol.types import OrganizationID, UserID
from parsec.backend.postgresql.handler import PGHandler
from parsec.backend.postgresql.factory import components_factory
from parsec.backend.postgresql.sequester import PGPSequesterComponent
from parsec.backend.sequester import SequesterService
from parsec.backend.cli.utils import db_backend_options


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


async def _human_accesses(config: BackendDbConfig, organization: OrganizationID, query: str):
    user_component: PGUserComponent
    realm_component: PGRealmComponent
    sequester_component: PGPSequesterComponent

    accesses = user_component.dump_accesses(organization=organization)
    no_human_accesses = accesses.pop(None)

    humans_per_user = await user_component.find_humans(
        organization_id=organization,
        query=query,
        per_page=-1,
    )

    user_cache = {}
    async def _get_user(user_id: UserID) -> User:
        try:
            return user_cache[user_id]
        except KeyError:
            pass
        user = await user_component.get_user(organization=organization, user_id=user_id)
        user_cache[user_id] = user
        return user

    humans = {}
    for human in humans_per_user:
        human_str = str(human.human_handle or human.user_id)
        users = humans.setdefault(human_str, [])
        users.append(human.user_id)

    # Typicall output to dislay:
    # 
    # Found 2 results:
    # Human John Doe <john.doe@example.com>
    # 
    #   User 9e082a43b51e44ab9858628bac4a61d9 (created on: 2000-01-02T00:00:00Z)
    # 
    #     Realm 8006a491f0704040ae9a197ca7501f71
    #       2000-01-04T00:00:00Z: Got MANAGER access
    #       2000-01-03T00:00:00Z: Access removed
    #       2000-01-02T00:00:00Z: Got READER access
    # 
    #     Realm 109c48b7c931435c913945f08d23432d
    #       2000-01-01T00:00:00Z: Got OWNER access
    # 
    #   User 02e0486752d34d6ab3bf8e0befef1935 (created on: 2000-01-01T00:00:00Z, revoked on: 2000-01-02T00:00:00Z)
    # 
    # Human Jane Doe <jane.doe@example.com>
    # 
    #   User baf59386baf740bba93151cdde1beac8 (created on: 2000-01-01T00:00:00Z):
    # 
    #     Realm 8006a491f0704040ae9a197ca7501f71
    #       2000-01-01T00:00:00Z: Got OWNER access


    print(f"Found {len(humans)} result(s)")
    for human_str, user_ids in humans.items():
        display_human = click.style(human_str, fg="green")
        print(f"Human {display_human}")

        users = []
        for user_id in user_ids:
            user = await _get_user(user_id)
            users.append(user)
        user = sorted(user, key=lambda u: u.created_on, reverse=True)

        for user in users:
            print(f"User {display_user}")


        per_realm = {}
        for (granted_on, realm_id, granted_by, role) in await realm_component.get_realms_history_for_user(organization_id=organization, user_id=human.user_id):
            realm_items = per_realm.setdefault(realm_id, [])
            realm_items.append((granted_on, granted_by, role))

        for realm_id, realm_items in per_realm.items():
            display_realm = click.style(str(realm_id), fg="yellow")
            print(f"\tRealm {display_realm}")
            for (granted_on, granted_by, role) in sorted(realm_items):
                if role is None:
                    display_role = "Access removed"
                else:
                    display_role = f"Got {role} access"
                print(f"\t\t{granted_on}: {display_role}")


    res = await sequester_component.human_accesses(organization, query)


@click.command()
@click.argument("query", type=str)
@click.option("--organization", type=OrganizationID, help="Organization ID")
@db_backend_options
def human_accesses(organization: str, db: str, db_max_connections: int, db_min_connections: int):
    db_config = _get_config(db, db_min_connections, db_max_connections)
    trio_run(_human_accesses, db_config, organization, query, use_asyncio=True)
