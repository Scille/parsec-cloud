# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import pytest
from click.testing import CliRunner

from parsec._parsec import VlobID, testbed
from parsec.cli import cli


@pytest.fixture
def workspace_archived_template():
    """Get the workspace_archived testbed template and extract relevant IDs."""
    template = testbed.test_get_testbed_template("workspace_archived")
    assert template is not None

    realm_ids = [
        event.realm_id
        for event in template.events
        if isinstance(event, testbed.TestbedEventNewRealm)
    ]
    assert len(realm_ids) == 7

    return {
        "organization_id": "WorkspaceArchivedOrgTemplate",
        "wksp_archived_id": realm_ids[0].hex,
        "wksp_soon_to_delete_id": realm_ids[1].hex,
        "wksp_ready_to_delete_id": realm_ids[2].hex,
        "wksp_deleted_id": realm_ids[3].hex,
        "wksp_not_longer_to_delete_id": realm_ids[4].hex,
        "wksp_orphaned_id": realm_ids[5].hex,
        "wksp_orphaned_and_ready_to_delete_id": realm_ids[6].hex,
    }


ORG = "WorkspaceArchivedOrgTemplate"
COMMON_ARGS = "--db MOCKED --with-testbed workspace_archived"


def test_ok(
    tmp_path,
    workspace_archived_template,
) -> None:
    runner = CliRunner()
    blocks_file = tmp_path / "blocks.txt"
    realm_id = workspace_archived_template["wksp_ready_to_delete_id"]

    result = runner.invoke(
        cli,
        f"delete_realm {COMMON_ARGS}"
        f" --organization {ORG}"
        f" --realm {realm_id}"
        f" --dump-realm-blocks {blocks_file}",
    )
    assert result.exit_code == 0, result.output
    assert blocks_file.exists()
    slugs = [line for line in blocks_file.read_text().splitlines() if line.strip()]
    # wksp_ready_to_delete has 5 blocks
    assert len(slugs) == 5
    for slug in slugs:
        assert slug.startswith(f"{ORG}/")


def test_dry_run(
    tmp_path,
    workspace_archived_template,
) -> None:
    runner = CliRunner()
    blocks_file = tmp_path / "blocks.txt"
    realm_id = workspace_archived_template["wksp_ready_to_delete_id"]

    result = runner.invoke(
        cli,
        f"delete_realm {COMMON_ARGS}"
        f" --organization {ORG}"
        f" --realm {realm_id}"
        f" --dump-realm-blocks {blocks_file}"
        f" --dry-run",
    )
    assert result.exit_code == 0, result.output
    assert blocks_file.exists()
    slugs = [line for line in blocks_file.read_text().splitlines() if line.strip()]
    assert len(slugs) == 5
    assert "Dry run" in result.output


def test_realm_not_eligible(
    tmp_path,
    workspace_archived_template,
) -> None:
    runner = CliRunner()
    blocks_file = tmp_path / "blocks.txt"
    realm_id = workspace_archived_template["wksp_archived_id"]

    result = runner.invoke(
        cli,
        f"delete_realm {COMMON_ARGS}"
        f" --organization {ORG}"
        f" --realm {realm_id}"
        f" --dump-realm-blocks {blocks_file}",
    )
    assert result.exit_code == 1
    assert "neither orphaned nor planned for deletion" in result.output


def test_realm_already_deleted(
    tmp_path,
    workspace_archived_template,
) -> None:
    runner = CliRunner()
    blocks_file = tmp_path / "blocks.txt"
    realm_id = workspace_archived_template["wksp_deleted_id"]

    result = runner.invoke(
        cli,
        f"delete_realm {COMMON_ARGS}"
        f" --organization {ORG}"
        f" --realm {realm_id}"
        f" --dump-realm-blocks {blocks_file}",
    )
    assert result.exit_code == 1
    assert "already been deleted" in result.output


def test_realm_deletion_date_not_reached(
    tmp_path,
    workspace_archived_template,
) -> None:
    runner = CliRunner()
    blocks_file = tmp_path / "blocks.txt"
    realm_id = workspace_archived_template["wksp_soon_to_delete_id"]

    result = runner.invoke(
        cli,
        f"delete_realm {COMMON_ARGS}"
        f" --organization {ORG}"
        f" --realm {realm_id}"
        f" --dump-realm-blocks {blocks_file}",
    )
    assert result.exit_code == 1
    assert "not been reached yet" in result.output


def test_orphaned_realm(
    tmp_path,
    workspace_archived_template,
) -> None:
    runner = CliRunner()
    blocks_file = tmp_path / "blocks.txt"
    realm_id = workspace_archived_template["wksp_orphaned_id"]

    result = runner.invoke(
        cli,
        f"delete_realm {COMMON_ARGS}"
        f" --organization {ORG}"
        f" --realm {realm_id}"
        f" --dump-realm-blocks {blocks_file}",
    )
    assert result.exit_code == 0, result.output
    assert blocks_file.exists()


def test_unknown_realm(
    tmp_path,
) -> None:
    runner = CliRunner()
    blocks_file = tmp_path / "blocks.txt"
    realm_id = VlobID.new()

    result = runner.invoke(
        cli,
        f"delete_realm {COMMON_ARGS}"
        f" --organization {ORG}"
        f" --realm {realm_id}"
        f" --dump-realm-blocks {blocks_file}",
    )
    assert result.exit_code == 1, result.output
    assert f"Realm `{realm_id.hex}` not found" in result.output
