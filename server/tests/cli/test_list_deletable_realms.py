# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import json

from click.testing import CliRunner

from parsec.cli import cli

from .test_delete_realm import COMMON_ARGS, ORG, workspace_archived_template

# Prevent this fixture from being seen as unused
_ = workspace_archived_template


def test_ok_with_candidates(
    workspace_archived_template,
) -> None:
    runner = CliRunner()
    t = workspace_archived_template

    result = runner.invoke(
        cli,
        f"list_deletable_realms {COMMON_ARGS} --organization {ORG}",
    )
    assert result.exit_code == 0, result.output
    assert "deletion candidate(s)" in result.output
    assert "deletion_planned" in result.output
    assert "orphaned" in result.output
    assert t["wksp_ready_to_delete_id"] in result.output
    assert t["wksp_orphaned_id"] in result.output
    assert t["wksp_orphaned_and_ready_to_delete_id"] in result.output


def test_ok_json(
    workspace_archived_template,
) -> None:
    runner = CliRunner()
    t = workspace_archived_template

    result = runner.invoke(
        cli,
        f"list_deletable_realms {COMMON_ARGS} --organization {ORG} --json --log-level=error",
    )
    assert result.exit_code == 0, result.output
    items = json.loads(result.output)
    assert isinstance(items, list)
    assert len(items) == 4

    realm_ids = {item["realm_id"] for item in items}
    assert t["wksp_ready_to_delete_id"] in realm_ids
    assert t["wksp_orphaned_id"] in realm_ids
    assert t["wksp_orphaned_and_ready_to_delete_id"] in realm_ids

    kinds = {item["kind"] for item in items}
    assert kinds == {"deletion_planned", "orphaned"}

    for item in items:
        if item["kind"] == "deletion_planned":
            assert "deletion_date" in item
        else:
            assert "orphaned_since" in item


def test_ok_json_empty() -> None:
    runner = CliRunner()

    result = runner.invoke(
        cli,
        "list_deletable_realms --db MOCKED --with-testbed coolorg"
        " --organization CoolorgOrgTemplate --json  --log-level=error",
    )
    assert result.exit_code == 0, result.output
    items = json.loads(result.output)
    assert items == []


def test_organization_not_found() -> None:
    runner = CliRunner()

    result = runner.invoke(
        cli,
        f"list_deletable_realms {COMMON_ARGS} --organization NonExistent",
    )
    assert result.exit_code == 1
    assert "not found" in result.output
