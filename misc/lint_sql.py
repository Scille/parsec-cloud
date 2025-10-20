#!/usr/bin/env python
# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

import importlib
import sys
import traceback
from argparse import ArgumentParser
from collections.abc import Iterable, Iterator
from pathlib import Path
from types import ModuleType
from typing import Any, Literal, cast

import sqlfluff

PROJECT_DIR = Path(__file__).parent.parent
SERVER_SQL_DIR = PROJECT_DIR / "server/parsec/components/postgresql"
LIBPARSEC_SQL_DIR = PROJECT_DIR / "libparsec/crates/platform_storage/src/native"
POSTGRESQL_SQLFLUFF_CONFIG = str(
    PROJECT_DIR / "server/parsec/components/postgresql/migrations/.sqlfluff"
)
SQLITE_SQLFLUFF_CONFIG = str(
    PROJECT_DIR / "libparsec/crates/platform_storage/src/native/sql/.sqlfluff"
)

BOLD_RED = "\x1b[1;31m"
GREY = "\x1b[37m"
PINK = "\x1b[35m"
NO_COLOR = "\x1b[0;0m"


def get_files(suffix: str, paths: Iterable[Path]) -> Iterator[Path]:
    for path in paths:
        if path.is_dir():
            yield from path.glob(f"**/*{suffix}")
        elif path.is_file() and path.name.endswith(suffix):
            yield path
        elif not path.exists():
            raise SystemExit(f"Error: Path `{path}` doesn't exist !")


def lint_sql_file(file: Path, dialect: Literal["postgres"] | Literal["sqlite"], fix: bool) -> bool:
    match dialect:
        case "postgres":
            config_path = POSTGRESQL_SQLFLUFF_CONFIG
        case "sqlite":
            config_path = SQLITE_SQLFLUFF_CONFIG

    sql = file.read_text()
    errors = sqlfluff.lint(sql, dialect=dialect, config_path=config_path)
    if errors:
        display_sqlfluff_errors(file, sql, errors)

        if fix:
            fixed = sqlfluff.fix(sql, dialect=dialect, config_path=config_path)
            file.write_text(fixed)

        return False

    return True


def display_sqlfluff_errors(file: Path | str, sql: str, errors: Iterable[dict[str, Any]]):
    if isinstance(file, Path):
        file_display = file.relative_to(PROJECT_DIR)
    else:
        file_display = file

    print(f"{BOLD_RED}SQL lint error: {NO_COLOR}{file_display}")

    src_lines = sql.splitlines()
    for error in errors:
        print(src_lines[error["start_line_no"] - 1])
        print(
            " " * (error["start_line_pos"] - 1)
            + f"{GREY}^ {error['code']}: {error['description']}{NO_COLOR}"
        )


if __name__ == "__main__":
    parser = ArgumentParser("Lint SQL")
    parser.add_argument(
        "--what",
        choices=["all", "libparsec-sql", "server-sql", "server-py"],
        nargs="*",
        default=["all"],
    )
    parser.add_argument(
        "--fix",
        action="store_true",
        help="Fix SQL files (only for `.sql` files).",
    )
    parser.add_argument(
        "--non-sql-file-print-fix",
        action="store_true",
        help="Only .sql file can be fixed, so dump on stdout the fixed SQL query otherwise",
    )
    parser.add_argument(
        "files",
        nargs="*",
        help="Files or directories to work on",
        type=Path,
        default=[
            SERVER_SQL_DIR,
            LIBPARSEC_SQL_DIR,
        ],
    )
    args = parser.parse_args()
    args.what = cast(list[str], args.what)
    args.files = cast(list[Path], args.files)
    args.files = [f.absolute() for f in args.files]
    args.fix = cast(bool, args.fix)
    args.non_sql_file_print_fix = cast(bool, args.non_sql_file_print_fix)

    lint_server_sql = False
    lint_server_py = False
    lint_libparsec = False

    for what in args.what:
        match what:
            case "all":
                lint_server_sql = True
                lint_server_py = True
                lint_libparsec = True
            case "libparsec-sql":
                lint_libparsec = True
            case "server-sql":
                lint_server_sql = True
            case "server-py":
                lint_server_py = True
            case "sql-files":
                lint_server_sql = True
                lint_libparsec = True
            case unknown:
                assert False, unknown

    all_good = True
    linted_files = 0

    # 1) Lint `.sql` files in server
    if lint_server_sql:
        for file in get_files(".sql", args.files):
            if not file.is_relative_to(SERVER_SQL_DIR):
                continue
            all_good &= lint_sql_file(file, dialect="postgres", fix=args.fix)
            linted_files += 1

    # 2) Lint `.sql` files in `libparsec_platform_storage`
    if lint_libparsec:
        for file in get_files(".sql", args.files):
            if not file.is_relative_to(LIBPARSEC_SQL_DIR):
                continue
            all_good &= lint_sql_file(file, dialect="sqlite", fix=args.fix)
            linted_files += 1

    # 3) Lint SQL embedded in server's Python code
    if lint_server_py:
        # To know if SQL lint is enable, `parsec.components.postgresql.utils` will try to
        # load the `__parsec_lint_sql` module.
        #
        # Hence here we create such module, then import `parsec.components.postgresql`
        # that will in turn initialize all global `Q(<sql>)` query, allowed then to be linted.

        def lint_sql(sql: str, variables: dict[str, str]):
            global all_good
            global linted_files

            # Current frame is this callback, parent frame is `Q.__init__`, finally
            # it's the grand parent that is the actual frame calling `Q(<sql>)` tha
            # we care about !
            caller_frame = traceback.extract_stack()[-3]
            # Ignore queries that are not part of the file we care about
            if Path(caller_frame.filename).resolve().absolute() not in allowed_py_files:
                return
            file = f"{caller_frame.filename}:{caller_frame.lineno}"

            # Lint the sql string
            errors = sqlfluff.lint(
                sql,
                config_path=POSTGRESQL_SQLFLUFF_CONFIG,
                exclude_rules=[  # Note this overrides `exclude_rules` from config file
                    "LT13",  # Exclude rule LT13 (aka "Files must not begin with newlines or whitespace")
                    "AM09",  # Exclude rule AM09 temporarily (https://github.com/Scille/parsec-cloud/issues/11385)
                ],
            )

            linted_files += 1
            if errors:
                all_good = False

                display_sqlfluff_errors(file, sql, errors)

                if args.non_sql_file_print_fix:
                    # We cannot modify the Python file, so we just display the correct SQL
                    # and let the user manually integrate it ;-)
                    print(f"{PINK}=============== Fixed ===================={NO_COLOR}")

                    fix = sqlfluff.fix(
                        sql, dialect="postgres", config_path=POSTGRESQL_SQLFLUFF_CONFIG
                    )
                    # Convert back the parameters (e.g. `$1` -> `$foo`)
                    for variable, positional_parameter in sorted(variables.items(), reverse=True):
                        fix = fix.replace(positional_parameter, f"${variable}")

                    print(fix)
                    print(f"{PINK}============= End Fixed =================={NO_COLOR}")

        allowed_py_files = [
            file.resolve().absolute()
            for file in get_files(".py", args.files)
            if file.is_relative_to(SERVER_SQL_DIR)
        ]
        if allowed_py_files:
            lint_sql_mod = ModuleType("__parsec_lint_sql")
            lint_sql_mod.lint_sql = lint_sql  # type: ignore
            sys.modules["__parsec_lint_sql"] = lint_sql_mod
            importlib.import_module("parsec.components.postgresql")

    if not all_good:
        raise SystemExit(1)
    else:
        print(f"{linted_files} linted files, all good ;-)")
