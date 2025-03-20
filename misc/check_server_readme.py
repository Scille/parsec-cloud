#!/usr/bin/env python3
# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

import argparse
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable, Literal

BASEDIR = Path(__file__).parent.parent.resolve()
README = BASEDIR / "server/README.md"
COMMAND_SCHEMAS_DIR = BASEDIR / "libparsec/crates/protocol/schema/"
START_TAG = "<!-- start-table-what-is-lock-by-who -->"
END_TAG = "<!-- end-table-what-is-lock-by-who -->"


Family = (
    Literal["tos"]
    | Literal["invited"]
    | Literal["anonymous"]
    | Literal["authenticated"]
    | Literal["CLI sequester"]
)
TopicLock = Literal["read"] | Literal["write"]
ManualLock = Literal["read"] | Literal["write"]
UserRevokeSafe = Literal["forbidden"] | Literal["sequential"]
LastVlobTimestampCheck = Literal["all user's realms"] | Literal["target realm"]


HEADERS = (
    "API Family",
    "command",
    "common",
    "realm",
    "sequester",
    "shamir",
    "invitation_creation",
    "user_revoke_safe",
    "last_vlob_timestamp",
)
FAMILY_VALUES = ("tos", "invited", "anonymous", "authenticated", "CLI sequester")
LOCK_VALUES = ("read", "write", None)
USER_REVOKE_SAFE_VALUES = ("forbidden", "sequential", None)
LAST_VLOB_TIMESTAMP_CHECK_VALUES = ("all user's realms", "target realm", None)


def display_row(row: str) -> str:
    headers_line = ""
    separator_line = ""
    values_line = ""
    for header, value in zip(HEADERS, row.split("|")[:-1]):
        width = max(len(header), len(value))
        headers_line += f" {header.ljust(width)} |"
        separator_line += "-" + "-" * width + "-|"
        values_line += f" {value.ljust(width)} |"
    return f"\n{headers_line}\n{separator_line}\n{values_line}"


@dataclass(slots=True, repr=False)
class Command:
    raw_row: str
    family: Family
    command: str
    topic_common: TopicLock | None
    topic_realm: TopicLock | None
    topic_sequester: TopicLock | None
    topic_shamir: TopicLock | None
    manual_invitation_creation: ManualLock | None
    special_user_revoke_safe: UserRevokeSafe | None
    special_last_vlob_timestamp: LastVlobTimestampCheck | None

    def __repr__(self):
        return display_row(self.raw_row)

    @classmethod
    def from_raw_row(
        cls,
        row: str,
    ):
        assert row[-1] == "|", "Table rows must end with a trailing `|`"

        columns = [c if (c := column.strip()) else None for column in row[:-1].split("|")]
        (
            raw_family,
            raw_command,
            raw_common,
            raw_realm,
            raw_sequester,
            raw_shamir,
            raw_invitation_creation,
            raw_user_revoke_safe,
            raw_last_vlob_timestamp,
        ) = columns

        assert (
            raw_family in FAMILY_VALUES
        ), f"Bad family (allowed: {FAMILY_VALUES})\nIn row:{display_row(row)}"
        assert raw_command, f"Empty command column\nIn row:{display_row(row)}"

        assert (
            raw_common in LOCK_VALUES
        ), f"Bad `common` topic (allowed: {LOCK_VALUES})\nIn row:{display_row(row)}"
        assert (
            raw_realm in LOCK_VALUES
        ), f"Bad `realm` topic (allowed: {LOCK_VALUES})\nIn row:{display_row(row)}"
        assert (
            raw_sequester in LOCK_VALUES
        ), f"Bad `sequester` topic (allowed: {LOCK_VALUES})\nIn row:{display_row(row)}"
        assert (
            raw_shamir in LOCK_VALUES
        ), f"Bad `shamir` topic (allowed: {LOCK_VALUES})\nIn row:{display_row(row)}"

        assert (
            raw_invitation_creation in LOCK_VALUES
        ), f"Bad `invitation_creation` lock (allowed: {LOCK_VALUES})\nIn row:{display_row(row)}"

        assert (
            raw_user_revoke_safe in USER_REVOKE_SAFE_VALUES
        ), f"Bad `user_revoke_safe` column (allowed: {USER_REVOKE_SAFE_VALUES})\nIn row:{display_row(row)}"
        assert (
            raw_last_vlob_timestamp in LAST_VLOB_TIMESTAMP_CHECK_VALUES
        ), f"Bad `user_revoke_safe` column (allowed: {LAST_VLOB_TIMESTAMP_CHECK_VALUES})\nIn row:{display_row(row)}"

        return cls(
            raw_row=row,
            family=raw_family,
            command=raw_command,
            topic_common=raw_common,
            topic_realm=raw_realm,
            topic_sequester=raw_sequester,
            topic_shamir=raw_shamir,
            manual_invitation_creation=raw_invitation_creation,
            special_user_revoke_safe=raw_user_revoke_safe,
            special_last_vlob_timestamp=raw_last_vlob_timestamp,
        )


def assert_all_commands_present(commands: Iterable[Command]) -> None:
    cooked_commands = set(
        f"{c.family} - {c.command}" for c in commands if c.family != "CLI sequester"
    )
    expected_commands = set()
    for command in COMMAND_SCHEMAS_DIR.glob("*/*.json5"):
        name = command.name.removesuffix(".json5")
        family = command.parent.name.removesuffix("_cmds")
        expected_commands.add(f"{family} - {name}")

    unknown_commands = cooked_commands - expected_commands
    missing_commands = expected_commands - cooked_commands

    assert not unknown_commands, "Unknown commands:\n" + "\n".join(sorted(unknown_commands))
    assert not missing_commands, "Missing commands:\n" + "\n".join(sorted(missing_commands))


def assert_last_vlob_timestamp_target_realm(commands: Iterable[Command]) -> None:
    """
    In `last_vlob_timestamp`, `target realm` value references a realm that must be
    specified in the command. This is something only done in `ream_*` commands.
    """
    for command in commands:
        match command.special_last_vlob_timestamp:
            case "all user's realms":
                assert not command.command.startswith(
                    "realm_"
                ), f"`realm_*` commands reference a specific realm, and hence must use `target realm` for `last_vlob_timestamp`.\nIn row:{command}"
            case "target realm":
                assert command.command.startswith(
                    "realm_"
                ), f"Non-`realm_*` commands must use `all user's realms` for `last_vlob_timestamp`.\nIn row:{command}"
            case None:
                pass


def assert_invitation_creation_only_for_invitation(commands: Iterable[Command]) -> None:
    for command in commands:
        if command.manual_invitation_creation:
            assert command.command.startswith(
                "invite_"
            ), f"As it name suggests, `invitation_creation` is only for invite commands !\nIn row:{command}"


def assert_common_topic_protects_user_revoke_safe(commands: Iterable[Command]) -> None:
    """
    See README's part "User revocation and concurrent operation involving users"
    """
    for command in commands:
        if command.command == "user_revoke":
            assert (
                command.special_user_revoke_safe == "sequential"
            ), f"Unexpected config for `user_revoke`.\nIn row:{command}"
            assert (
                command.topic_common == "write"
            ), f"Unexpected config for `user_revoke`.\nIn row:{command}"
        elif command.special_user_revoke_safe:
            assert (
                command.special_user_revoke_safe == "forbidden"
            ), f"Only `user_revoke` command should be marked `sequential` (as it is the only command to revoke user !).\nIn row:{command}"
            assert (
                command.topic_common in ("read", "write")
            ), f"To be forbidden to run concurrently with `user_revoke`, the command must lock the `common` topic.\nIn row:{command}"


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="""Check consistency of `server/README.md`'s "What is locked by who" table"""
    )
    args = parser.parse_args()

    # 1) Parse the Table

    lines = README.read_text().splitlines()
    lines = lines[lines.index(START_TAG) + 1 : lines.index(END_TAG)]
    (raw_headers, _, *raw_commands) = lines

    assert raw_headers[-1] == "|", "Table rows must end with a trailing `|`"
    headers = tuple(h.strip() for h in raw_headers[:-1].split("|"))
    assert (
        headers == HEADERS
    ), "Headers have changed, you most likely want to check this script is still doing relevant things !"

    commands = [Command.from_raw_row(row) for row in raw_commands]

    # 2) Do the consistency checks

    assert_all_commands_present(commands)
    assert_last_vlob_timestamp_target_realm(commands)
    assert_invitation_creation_only_for_invitation(commands)
    assert_common_topic_protects_user_revoke_safe(commands)
