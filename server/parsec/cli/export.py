# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import asyncio
from pathlib import Path
from typing import Any

import click

from parsec._parsec import (
    DateTime,
    OrganizationID,
    VlobID,
)
from parsec.cli.options import (
    blockstore_server_options,
    db_server_options,
    debug_config_options,
    logging_config_options,
)
from parsec.cli.testbed import if_testbed_available
from parsec.cli.utils import cli_exception_handler, start_backend
from parsec.config import (
    BaseBlockStoreConfig,
    BaseDatabaseConfig,
    LogLevel,
)
from parsec.realm_export import (
    ExportProgressStep,
    default_realm_export_db_name,
    get_earliest_allowed_snapshot_timestamp,
)
from parsec.realm_export import export_realm as do_export_realm


class DevOption(click.Option):
    def handle_parse_result(
        self, ctx: click.Context, opts: Any, args: list[str]
    ) -> tuple[Any, list[str]]:
        value, args = super().handle_parse_result(ctx, opts, args)
        if value:
            for key, value in (
                ("debug", True),
                ("db", "MOCKED"),
                ("blockstore", ("MOCKED",)),
                ("with_testbed", "workspace_history"),
                ("organization", "WorkspaceHistoryOrgTemplate"),
                ("realm", "f0000000000000000000000000000008"),
            ):
                if key not in opts:
                    opts[key] = value

        return value, args


@click.command(
    short_help="Export the content of a realm in order to consult it with a sequester service key"
)
@click.argument("output", type=Path, required=False)
@click.option("--organization", type=OrganizationID, required=True)
@click.option("--realm", type=VlobID.from_hex, required=True)
@click.option("--snapshot-timestamp", type=DateTime.from_rfc3339)
@db_server_options
@blockstore_server_options
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
            "Equivalent to `--debug --db=MOCKED --blockstore=MOCKED"
            " --with-testbed=workspace_history --organization WorkspaceHistoryOrgTemplate --realm f0000000000000000000000000000008`"
        ),
    )
)
def export_realm(
    organization: OrganizationID,
    realm: VlobID,
    snapshot_timestamp: DateTime | None,
    output: Path,
    db: BaseDatabaseConfig,
    db_min_connections: int,
    db_max_connections: int,
    blockstore: BaseBlockStoreConfig,
    log_level: LogLevel,
    log_format: str,
    log_file: str | None,
    debug: bool,
    with_testbed: str | None = None,
    dev: bool = False,
) -> None:
    with cli_exception_handler(debug):
        asyncio.run(
            _export_realm(
                debug=debug,
                db_config=db,
                blockstore_config=blockstore,
                organization_id=organization,
                realm_id=realm,
                snapshot_timestamp=snapshot_timestamp,
                output=output,
                with_testbed=with_testbed,
            )
        )


async def _export_realm(
    db_config: BaseDatabaseConfig,
    blockstore_config: BaseBlockStoreConfig,
    debug: bool,
    with_testbed: str | None,
    organization_id: OrganizationID,
    realm_id: VlobID,
    snapshot_timestamp: DateTime | None,
    output: Path | None,
):
    snapshot_timestamp = snapshot_timestamp or get_earliest_allowed_snapshot_timestamp()
    output = output or Path.cwd()

    if output.is_dir():
        # Output is pointing to a directory, use a default name for the database extract
        output_db_path = output / default_realm_export_db_name(
            organization_id, realm_id, snapshot_timestamp
        )

    else:
        output_db_path = output

    output_db_display = click.style(str(output_db_path), fg="green")
    if output_db_path.exists():
        click.echo(
            f"File {output_db_display} already exists, continue the extract from where it was left"
        )
    else:
        click.echo(f"Creating {output_db_display}")

    click.echo(
        f"Use {click.style('^C', fg='yellow')} to stop the export,"
        " progress won't be lost when restarting the command"
    )

    async with start_backend(
        db_config=db_config,
        blockstore_config=blockstore_config,
        debug=debug,
        populate_with_template=with_testbed,
    ) as backend:
        with click.progressbar(
            length=0, label="Starting", show_pos=True, update_min_steps=0
        ) as bar:

            def _on_progress(step: ExportProgressStep):
                match step:
                    case "certificates_start":
                        bar.finished = False
                        bar.label = "1/4 Exporting certificates"
                        bar.length = 1
                        bar.update(0)
                    case "certificates_done":
                        bar.update(1)
                    case ("vlobs", exported, total):
                        bar.finished = False
                        bar.label = "2/4 Exporting vlobs"
                        bar.length = total
                        bar.pos = exported
                        bar.update(0)
                    case ("blocks_metadata", exported, total):
                        bar.finished = False
                        bar.label = "3/4 Exporting blocks metadata"
                        bar.length = total
                        bar.pos = exported
                        bar.update(0)
                    case ("blocks_data", exported, total):
                        bar.finished = False
                        bar.label = "4/4 Exporting blocks data"
                        bar.length = total
                        bar.pos = exported
                        bar.update(0)

            await do_export_realm(
                backend,
                organization_id,
                realm_id,
                snapshot_timestamp,
                output_db_path,
                _on_progress,
            )
