# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import asyncio
import json
from pathlib import Path
from typing import Any

import anyio
import click

from parsec._parsec import (
    DateTime,
    OrganizationID,
    VlobID,
)
from parsec.cli.options import (
    db_server_options,
    debug_config_options,
    logging_config_options,
)
from parsec.cli.testbed import if_testbed_available
from parsec.cli.utils import cli_exception_handler, start_backend
from parsec.components.realm import (
    RealmDelete1GetBlocksBatchBadOutcome,
    RealmDelete2DoDeleteMetadataBadOutcome,
    RealmDeleteGetBlocksBatch,
    RealmDeletionCandidateOrphaned,
    RealmDeletionCandidatePlanned,
    RealmListDeletionCandidatesBadOutcome,
)
from parsec.config import (
    BaseDatabaseConfig,
    DisabledBlockStoreConfig,
    LogLevel,
    MockedBlockStoreConfig,
)

# This batch refers to the export of all block IDs from the PostgreSQL
# database (see `delete_1_get_blocks_batch()`).
# PostgreSQL returns only a SERIAL (4 bytes) + UUID (16 bytes) for each
# row, so we can go with a large batch.
BLOCK_BATCH_SIZE = 10_000


class ListDeletableRealmsDevOption(click.Option):
    def handle_parse_result(
        self, ctx: click.Context, opts: Any, args: list[str]
    ) -> tuple[Any, list[str]]:
        value, args = super().handle_parse_result(ctx, opts, args)
        if value:
            for key, value in (
                ("debug", True),
                ("db", "MOCKED"),
                ("with_testbed", "workspace_archived"),
                ("organization", "WorkspaceArchivedOrgTemplate"),
            ):
                if key not in opts:
                    opts[key] = value

        return value, args


@click.command(short_help="List realms eligible for deletion")
@click.option("--organization", type=OrganizationID, required=True)
@click.option("--json", "output_json", is_flag=True, default=False, help="Output as JSON")
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
        cls=ListDeletableRealmsDevOption,
        is_flag=True,
        is_eager=True,
        help=(
            "Equivalent to `--debug --db=MOCKED --with-testbed=workspace_archived --organization WorkspaceArchivedTemplate`"
        ),
    )
)
def list_deletable_realms(
    organization: OrganizationID,
    output_json: bool,
    db: BaseDatabaseConfig,
    db_min_connections: int,
    db_max_connections: int,
    log_level: LogLevel,
    log_format: str,
    log_file: str | None,
    debug: bool,
    with_testbed: str | None = None,
    dev: bool = False,
) -> None:
    with cli_exception_handler(debug):
        asyncio.run(
            _list_deletable_realms(
                debug=debug,
                db_config=db,
                organization_id=organization,
                with_testbed=with_testbed,
                output_json=output_json,
            )
        )


async def _list_deletable_realms(
    db_config: BaseDatabaseConfig,
    debug: bool,
    with_testbed: str | None,
    organization_id: OrganizationID,
    output_json: bool = False,
) -> None:
    # Can use a dummy blockstore config since we are not going to query it
    if with_testbed is None:
        blockstore_config = DisabledBlockStoreConfig()
    else:
        blockstore_config = MockedBlockStoreConfig()

    async with start_backend(
        db_config=db_config,
        blockstore_config=blockstore_config,
        debug=debug,
        populate_with_template=with_testbed,
    ) as backend:
        now = DateTime.now()
        outcome = await backend.realm.list_deletion_candidates(
            organization_id=organization_id,
            now=now,
        )
        match outcome:
            case RealmListDeletionCandidatesBadOutcome.ORGANIZATION_NOT_FOUND:
                raise RuntimeError(f"Organization `{organization_id.str}` not found")
            case list() as candidates:
                pass

        if output_json:
            items = []
            for candidate in candidates:
                match candidate:
                    case RealmDeletionCandidatePlanned(deletion_date=deletion_date):
                        items.append(
                            {
                                "realm_id": candidate.realm_id.hex,
                                "kind": "deletion_planned",
                                "deletion_date": str(deletion_date),
                            }
                        )
                    case RealmDeletionCandidateOrphaned(orphaned_since=orphaned_since):
                        items.append(
                            {
                                "realm_id": candidate.realm_id.hex,
                                "kind": "orphaned",
                                "orphaned_since": str(orphaned_since),
                            }
                        )
                    case unknown:
                        assert False, unknown
            click.echo(json.dumps(items, indent=2))
            return

        if not candidates:
            click.echo("No realms eligible for deletion")
            return

        click.echo(f"Found {len(candidates)} deletion candidate(s):\n")
        for candidate in candidates:
            realm_display = click.style(candidate.realm_id.hex, fg="yellow")
            match candidate:
                case RealmDeletionCandidatePlanned(deletion_date=deletion_date):
                    kind = click.style("deletion_planned", fg="red")
                    click.echo(
                        f"  Realm {realm_display}  kind={kind}  deletion_date={deletion_date}"
                    )
                case RealmDeletionCandidateOrphaned(orphaned_since=orphaned_since):
                    kind = click.style("orphaned        ", fg="cyan")
                    click.echo(
                        f"  Realm {realm_display}  kind={kind}  orphaned_since={orphaned_since}"
                    )
                case unknown:
                    assert False, unknown


class DeleteRealmDevOption(click.Option):
    def handle_parse_result(
        self, ctx: click.Context, opts: Any, args: list[str]
    ) -> tuple[Any, list[str]]:
        value, args = super().handle_parse_result(ctx, opts, args)
        if value:
            for key, value in (
                ("debug", True),
                ("db", "MOCKED"),
                ("with_testbed", "workspace_archived"),
                ("organization", "WorkspaceArchivedOrgTemplate"),
            ):
                if key not in opts:
                    opts[key] = value

        return value, args


@click.command(short_help="Delete a realm's metadata from the database")
@click.option("--organization", type=OrganizationID, required=True)
@click.option("--realm", type=VlobID.from_hex, required=True)
@click.option(
    "--dump-realm-blocks",
    type=click.Path(dir_okay=False, writable=True, path_type=Path),
    required=True,
    help="Output file for the list of block slugs to remove from the object storage",
)
@click.option(
    "--dry-run",
    is_flag=True,
    default=False,
    help="Pretend to do the deletion (export block slugs but do not delete metadata)",
)
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
        cls=DeleteRealmDevOption,
        is_flag=True,
        is_eager=True,
        help=(
            "Equivalent to `--debug --db=MOCKED --with-testbed=workspace_archived --organization WorkspaceArchivedTemplate`"
        ),
    )
)
def delete_realm(
    organization: OrganizationID,
    realm: VlobID,
    dump_realm_blocks: Path,
    dry_run: bool,
    db: BaseDatabaseConfig,
    db_min_connections: int,
    db_max_connections: int,
    log_level: LogLevel,
    log_format: str,
    log_file: str | None,
    debug: bool,
    with_testbed: str | None = None,
    dev: bool = False,
) -> None:
    with cli_exception_handler(debug):
        asyncio.run(
            _delete_realm(
                debug=debug,
                db_config=db,
                organization_id=organization,
                realm_id=realm,
                dump_realm_blocks=dump_realm_blocks,
                dry_run=dry_run,
                with_testbed=with_testbed,
            )
        )


async def _delete_realm(
    db_config: BaseDatabaseConfig,
    debug: bool,
    with_testbed: str | None,
    organization_id: OrganizationID,
    realm_id: VlobID,
    dump_realm_blocks: Path,
    dry_run: bool,
) -> None:
    # Can use a dummy blockstore config since we are not going to query it
    if with_testbed is None:
        blockstore_config = DisabledBlockStoreConfig()
    else:
        blockstore_config = MockedBlockStoreConfig()

    async with start_backend(
        db_config=db_config,
        blockstore_config=blockstore_config,
        debug=debug,
        populate_with_template=with_testbed,
    ) as backend:
        realm_display = click.style(realm_id.hex, fg="yellow")

        # Step 1: Export block slugs

        batch_offset_marker = 0
        total_blocks = 0
        async with await anyio.open_file(dump_realm_blocks, "w") as f:
            with click.progressbar(
                length=0,
                label="Exporting a list of all the realm's block that can be removed",
                show_pos=True,
                update_min_steps=0,
            ) as bar:
                assert bar.length is not None

                while True:
                    outcome = await backend.realm.delete_1_get_blocks_batch(
                        organization_id=organization_id,
                        realm_id=realm_id,
                        batch_offset_marker=batch_offset_marker,
                        batch_size=BLOCK_BATCH_SIZE,
                    )

                    match outcome:
                        case RealmDeleteGetBlocksBatch() as batch:
                            pass
                        case RealmDelete1GetBlocksBatchBadOutcome.ORGANIZATION_NOT_FOUND:
                            raise RuntimeError(f"Organization `{organization_id.str}` not found")
                        case RealmDelete1GetBlocksBatchBadOutcome.REALM_NOT_FOUND:
                            raise RuntimeError(f"Realm `{realm_id.hex}` not found")

                    for block_id in batch.blocks:
                        slug = f"{organization_id.str}/{block_id.hyphenated}"
                        await f.write(slug + "\n")

                    total_blocks += len(batch.blocks)
                    bar.length += len(batch.blocks)
                    bar.update(len(batch.blocks))

                    if len(batch.blocks) < BLOCK_BATCH_SIZE:
                        break

                    batch_offset_marker = batch.batch_offset_marker

        blocks_file_display = click.style(str(dump_realm_blocks), fg="green")
        click.echo(f"Exported {total_blocks} block slug(s) to {blocks_file_display}")

        if dry_run:
            click.echo(click.style("Dry run: skipping metadata deletion", fg="yellow"))
            return

        # Step 2: Delete metadata

        click.echo(f"Deleting metadata for realm {realm_display}... ", nl=False)

        now = DateTime.now()
        outcome = await backend.realm.delete_2_do_delete_metadata(
            organization_id=organization_id,
            realm_id=realm_id,
            now=now,
        )

        match outcome:
            case None:
                pass
            case RealmDelete2DoDeleteMetadataBadOutcome.ORGANIZATION_NOT_FOUND:
                raise RuntimeError(f"Organization `{organization_id.str}` not found")
            case RealmDelete2DoDeleteMetadataBadOutcome.REALM_NOT_FOUND:
                raise RuntimeError(f"Realm `{realm_id.hex}` not found")
            case RealmDelete2DoDeleteMetadataBadOutcome.REALM_ALREADY_DELETED:
                raise RuntimeError(f"Realm `{realm_id.hex}` has already been deleted")
            case RealmDelete2DoDeleteMetadataBadOutcome.REALM_NOT_ORPHANED_NOR_DELETION_PLANNED:
                raise RuntimeError(
                    f"Realm `{realm_id.hex}` is neither orphaned nor planned for deletion"
                )
            case RealmDelete2DoDeleteMetadataBadOutcome.REALM_DELETION_DATE_NOT_REACHED:
                raise RuntimeError(
                    f"Realm `{realm_id.hex}` has a planned deletion date that has not been reached yet"
                )

        click.echo(click.style("✔", fg="green"))
        click.echo(
            "⚠️ The realm has been deleted, however its blocks are still present in the object storage"
        )
        click.echo(
            f"⚠️ You should now manually clean the object storage by removing all the path listed in {blocks_file_display}"
        )
