# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import asyncio
from typing import Any

import click

from parsec._parsec import (
    HumanHandle,
    OrganizationID,
    UserID,
    VlobID,
)
from parsec.cli.options import db_server_options, debug_config_options, logging_config_options
from parsec.cli.testbed import if_testbed_available
from parsec.cli.utils import cli_exception_handler, start_backend
from parsec.components.realm import RealmGrantedRole
from parsec.components.user import UserDump
from parsec.config import BaseDatabaseConfig, DisabledBlockStoreConfig, LogLevel


class DevOption(click.Option):
    def handle_parse_result(
        self, ctx: click.Context, opts: Any, args: list[str]
    ) -> tuple[Any, list[str]]:
        value, args = super().handle_parse_result(ctx, opts, args)
        if value:
            for key, value in (
                ("debug", True),
                ("db", "MOCKED"),
                ("with_testbed", "coolorg"),
                ("organization", "CoolorgOrgTemplate"),
            ):
                if key not in opts:
                    opts[key] = value

        return value, args


@click.command(short_help="Get information about user&realm accesses")
@click.option("--organization", type=OrganizationID, required=True)
@click.option("--filter", type=str, default="", help="Filter by human handle or user ID")
@db_server_options
# Add --log-level/--log-format/--log-file
@logging_config_options(default_log_level="INFO")
# Add --debug & --version
@debug_config_options
@if_testbed_available(
    click.option("--with-testbed", help="Start by populating with a testbed template")
)
@if_testbed_available(
    click.option(
        "--dev",
        cls=DevOption,
        is_flag=True,
        is_eager=True,
        help=(
            "Equivalent to `--debug --db=MOCKED --with-testbed=coolorg --organization CoolorgOrgTemplate`"
        ),
    )
)
def human_accesses(
    filter: str,
    organization: OrganizationID,
    db: BaseDatabaseConfig,
    db_max_connections: int,
    db_min_connections: int,
    log_level: LogLevel,
    log_format: str,
    log_file: str | None,
    debug: bool,
    with_testbed: str | None = None,
    dev: bool = False,
) -> None:
    with cli_exception_handler(debug):
        asyncio.run(
            _human_accesses(
                debug=debug,
                db_config=db,
                organization_id=organization,
                with_testbed=with_testbed,
                user_filter=filter,
            )
        )


async def _human_accesses(
    db_config: BaseDatabaseConfig,
    debug: bool,
    with_testbed: str | None,
    organization_id: OrganizationID,
    user_filter: str,
) -> None:
    # Can use a dummy blockstore config since we are not going to query it
    blockstore_config = DisabledBlockStoreConfig()

    async with start_backend(
        db_config=db_config,
        blockstore_config=blockstore_config,
        debug=debug,
        populate_with_template=with_testbed,
    ) as backend:
        dump = await backend.user.test_dump_current_users(organization_id=organization_id)
        users = list(dump.values())
        if user_filter:
            # Now is a good time to filter out
            filter_split = user_filter.split()
            filtered_users = []
            for user in users:
                # Note user ID is present twice to handle both compact and dash separated formats
                # (i.e. `a11cec00-1000-0000-0000-000000000000` vs `a11cec00100000000000000000000000`).
                txt = f"{user.human_handle.str if user.human_handle else ''} {user.user_id.hex} {user.user_id}".lower()
                if len([True for sq in filter_split if sq in txt]) == len(filter_split):
                    filtered_users.append(user)
            users = filtered_users

        realms_granted_roles = await backend.realm.dump_realms_granted_roles(
            organization_id=organization_id
        )
        assert isinstance(realms_granted_roles, list)
        per_user_granted_roles: dict[UserID, list[RealmGrantedRole]] = {}
        for granted_role in realms_granted_roles:
            user_granted_roles = per_user_granted_roles.setdefault(granted_role.user_id, [])
            user_granted_roles.append(granted_role)

        humans: dict[HumanHandle, list[tuple[UserDump, dict[VlobID, list[RealmGrantedRole]]]]] = {}
        for user in users:
            human_users = humans.setdefault(user.human_handle, [])
            per_user_per_realm_granted_role: dict[VlobID, list[RealmGrantedRole]] = {}
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
        #   User 02e0486752d34d6ab3bf8e0befef1935 (REVOKED)
        #     2000-01-01T00:00:00Z: Created with profile STANDARD
        #     2000-01-02T00:00:00Z: Updated to profile CONTRIBUTOR
        #     2000-12-31T00:00:00Z: Revoked
        #
        #   User 9e082a43b51e44ab9858628bac4a61d9 (ADMIN)
        #     2001-01-01T00:00:00Z: Created with profile ADMIN
        #
        #     Realm 8006a491f0704040ae9a197ca7501f71
        #       2001-02-01T00:00:00Z: Access OWNER granted
        #       2001-02-02T00:00:00Z: Access removed
        #       2001-02-03T00:00:00Z: Access READER granted
        #
        #     Realm 109c48b7c931435c913945f08d23432d
        #       2001-02-01T00:00:00Z: Access OWNER granted
        #
        # Human Jane Doe <jane.doe@example.com>
        #
        #   User baf59386baf740bba93151cdde1beac8 (OUTSIDER)
        #     2000-01-01T00:00:00Z: Created with profile OUTSIDER
        #
        #     Realm 8006a491f0704040ae9a197ca7501f71
        #       2001-02-01T00:00:00Z: Access READER granted

        def _display_user(
            user: UserDump,
            per_realm_granted_role: dict[VlobID, list[RealmGrantedRole]],
            indent: int,
        ) -> None:
            base_indent = "\t" * indent
            display_user = click.style(user.user_id, fg="green")
            if not user.revoked_on:
                user_info = f"{user.current_profile}"
            else:
                user_info = "REVOKED"
            print(base_indent + f"User {display_user} ({user_info})")
            print(base_indent + f"\t{user.created_on}: Created with profile {user.initial_profile}")

            for profile_update in user.subsequent_profile_updates:
                print(
                    base_indent + f"\t{profile_update[0]}: Updated to profile {profile_update[1]}"
                )

            if user.revoked_on:
                print(base_indent + f"\t{user.revoked_on}: Revoked")

            print()

            for realm_id, granted_roles in per_realm_granted_role.items():
                display_realm = click.style(realm_id.hex, fg="yellow")
                print(base_indent + f"\tRealm {display_realm}")
                for granted_role in granted_roles:
                    if granted_role.role is None:
                        display_role = "Access removed"
                    else:
                        display_role = f"Access {granted_role.role.str} granted"
                    print(base_indent + f"\t\t{granted_role.granted_on}: {display_role}")

        print(f"Found {len(humans)} result(s)")

        for human_handle, human_users in humans.items():
            display_human = click.style(human_handle, fg="green")
            print(f"Human {display_human}")
            for user, per_realm_granted_roles in human_users:
                _display_user(user, per_realm_granted_roles, indent=1)
            print()
