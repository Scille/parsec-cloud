# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import sys
from collections import defaultdict
from collections.abc import Callable, Iterable, Iterator
from contextlib import contextmanager
from functools import wraps
from itertools import count
from typing import (
    Any,
    Concatenate,
    TextIO,
    cast,
)

import click

from parsec._version import __version__
from parsec.config import (
    BaseBlockStoreConfig,
    BaseDatabaseConfig,
    LogLevel,
    MockedBlockStoreConfig,
    MockedDatabaseConfig,
    PostgreSQLBlockStoreConfig,
    PostgreSQLDatabaseConfig,
    RAID0BlockStoreConfig,
    RAID1BlockStoreConfig,
    RAID5BlockStoreConfig,
    S3BlockStoreConfig,
    SWIFTBlockStoreConfig,
)
from parsec.logging import LogFormat, configure_logging, enable_sentry_logging


def logging_config_options(
    default_log_level: str,
) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
    LOG_LEVELS = ("DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL")
    assert default_log_level in LOG_LEVELS

    def _logging_config_options[**P, R](
        fn: Callable[P, R],
    ) -> Callable[Concatenate[str, str, str, P], R]:
        @click.option(
            "--log-level",
            "-l",
            type=click.Choice(LOG_LEVELS, case_sensitive=False),
            default=default_log_level,
            show_default=True,
            envvar="PARSEC_LOG_LEVEL",
            show_envvar=True,
        )
        @click.option(
            "--log-format",
            "-f",
            type=click.Choice([x.name for x in LogFormat], case_sensitive=False),
            default=LogFormat.CONSOLE.name,
            show_default=True,
            envvar="PARSEC_LOG_FORMAT",
            show_envvar=True,
        )
        @click.option(
            "--log-file",
            "-o",
            default=None,
            envvar="PARSEC_LOG_FILE",
            show_envvar=True,
            help="[default: stderr]",
        )
        @wraps(fn)
        def wrapper(
            log_level: str,
            log_format: str,
            log_file: str | None,
            *args: P.args,
            **kwargs: P.kwargs,
        ) -> R:
            # `click.open_file` considers "-" to be stdout
            if log_file in (None, "-"):

                @contextmanager
                def open_log_file() -> Iterator[TextIO]:
                    yield sys.stderr

            else:

                @contextmanager
                def open_log_file() -> Iterator[TextIO]:
                    assert log_file is not None
                    yield cast(TextIO, click.open_file(filename=log_file, mode="w"))

            parsed_log_level = LogLevel[log_level.upper()]
            parsed_log_format = LogFormat[log_format.upper()]
            kwargs["log_level"] = parsed_log_level
            kwargs["log_format"] = parsed_log_format
            kwargs["log_file"] = log_file

            with open_log_file() as fd:
                configure_logging(
                    log_level=parsed_log_level, log_format=parsed_log_format, log_stream=fd
                )

                return fn(*args, **kwargs)

        return wrapper

    return _logging_config_options


def sentry_config_options(
    configure_sentry: bool,
) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
    def _sentry_config_options[**P, R](fn: Callable[P, R]) -> Callable[Concatenate[str, str, P], R]:
        # Sentry SKD uses 3 environ variables during it configuration phase:
        # - `SENTRY_DSN`
        # - `SENTRY_ENVIRONMENT`
        # - `SENTRY_RELEASE`
        # Those variables are only used if the corresponding parameter is not
        # explicitly provided while calling `sentry_init(**config)`.
        # Hence we make sure we provide the three parameters (note the release
        # is determined from Parsec's version) so those `PARSEC_*` env vars
        # are never read and don't clash with the `PARSEC_SENTRY_*` ones.
        @click.option(
            "--sentry-dsn",
            metavar="URL",
            envvar="PARSEC_SENTRY_DSN",
            show_envvar=True,
            help="Sentry Data Source Name for telemetry report",
        )
        @click.option(
            "--sentry-environment",
            metavar="NAME",
            envvar="PARSEC_SENTRY_ENVIRONMENT",
            show_envvar=True,
            default="production",
            show_default=True,
            help="Sentry environment for telemetry report",
        )
        @wraps(fn)
        def wrapper(
            sentry_dsn: str | None, sentry_environment: str, *args: P.args, **kwargs: P.kwargs
        ) -> R:
            if configure_sentry and sentry_dsn:
                enable_sentry_logging(dsn=sentry_dsn, environment=sentry_environment)

            kwargs["sentry_dsn"] = sentry_dsn
            kwargs["sentry_environment"] = sentry_environment

            return fn(*args, **kwargs)

        return wrapper

    return _sentry_config_options


def version_option[**P, R](fn: Callable[P, R]) -> Callable[P, R]:
    return click.version_option(version=__version__, prog_name="parsec")(fn)


def debug_config_options[**P, R](fn: Callable[P, R]) -> Callable[Concatenate[bool, P], R]:
    for decorator in (
        click.option(
            "--debug",
            is_flag=True,
            # Don't prefix with `PARSEC_` given devs are lazy
            envvar="DEBUG",
        ),
        version_option,
    ):
        fn = decorator(fn)

    return cast(Callable[Concatenate[bool, P], R], fn)


DEFAULT_DB_MIN_CONNECTIONS = 1
DEFAULT_DB_MAX_CONNECTIONS = 7


def _parse_db_param(value: str) -> BaseDatabaseConfig:
    if value.upper() == "MOCKED":
        return MockedDatabaseConfig()
    elif value.startswith("postgresql://") or value.startswith("postgres://"):
        return PostgreSQLDatabaseConfig(url=value, min_connections=5, max_connections=7)
    else:
        raise click.BadParameter(f"Invalid db type `{value}`")


def _get_db_min_max_connections_from_raw(opts: Any) -> tuple[int, int] | None:
    raw_db_min_connections = opts.get("db_min_connections", DEFAULT_DB_MIN_CONNECTIONS)
    try:
        db_min_connections = int(raw_db_min_connections)
    except (TypeError, ValueError):
        return None

    raw_db_max_connections = opts.get("db_max_connections", DEFAULT_DB_MAX_CONNECTIONS)
    try:
        db_max_connections = int(raw_db_max_connections)
    except (TypeError, ValueError):
        return None

    return db_min_connections, db_max_connections


class DbConfigOption(click.Option):
    """
    DB connection configuration also take into account the min / max connections
    constraints that are passed as separated options.
    """

    def handle_parse_result(
        self, ctx: click.Context, opts: Any, args: list[str]
    ) -> tuple[Any, list[str]]:
        value, args = super().handle_parse_result(ctx, opts, args)
        assert isinstance(value, BaseDatabaseConfig)

        # Given we hook into Click's option parsing, we cannot access the final0
        # parsed value for other options. So instead we have to parse them ourselves !
        match _get_db_min_max_connections_from_raw(opts):
            case (db_min_connections, db_max_connections):
                value.set_min_max_connections(db_min_connections, db_max_connections)
            case None:
                # Provided DB min/max connections options are invalid, we don't
                # need to do anything then: Click is going to terminate the process
                # as soon as it parse the faulty value.
                pass

        return value, args


class DBMaxConnectionsOption(click.Option):
    """
    DB max connection must be superior to min connection, this is was is
    enforced here.
    """

    def handle_parse_result(
        self, ctx: click.Context, opts: Any, args: list[str]
    ) -> tuple[Any, list[str]]:
        value, args = super().handle_parse_result(ctx, opts, args)
        assert isinstance(value, int)

        # Given we hook into Click's option parsing, we cannot access the final0
        # parsed value for other options. So instead we have to parse them ourselves !
        match _get_db_min_max_connections_from_raw(opts):
            case (db_min_connections, _):
                if db_min_connections > value:
                    raise click.BadParameter(
                        "'--db-max-connections' must be greater than '--db-min-connections'"
                    )

            case None:
                # Provided DB min/max connections options are invalid, we don't
                # need to do anything then: Click is going to terminate the process
                # as soon as it parse the faulty value.
                pass

        return value, args


def db_server_options[**Q, T](fn: Callable[Q, T]) -> Callable[Q, T]:
    decorators = [
        click.option(
            "--db",
            cls=DbConfigOption,
            required=True,
            envvar="PARSEC_DB",
            show_envvar=True,
            type=_parse_db_param,
            metavar="URL",
            help="""Database configuration.
Allowed values:
\b
-`MOCKED`: Mocked in memory
-`postgresql://<...>`: Use PostgreSQL database
\b
""",
        ),
        click.option(
            "--db-min-connections",
            default=DEFAULT_DB_MIN_CONNECTIONS,
            show_default=True,
            envvar="PARSEC_DB_MIN_CONNECTIONS",
            show_envvar=True,
            type=int,
            help="Minimal number of connections to the database if using PostgreSQL",
        ),
        click.option(
            "--db-max-connections",
            cls=DBMaxConnectionsOption,
            default=DEFAULT_DB_MAX_CONNECTIONS,
            show_default=True,
            envvar="PARSEC_DB_MAX_CONNECTIONS",
            show_envvar=True,
            # 1 connection always used for listening to PostgreSQL events,
            # and at least 1 required to be used in the requests.
            type=click.IntRange(min=2),
            help="Maximum number of connections to the database if using PostgreSQL",
        ),
    ]
    for decorator in decorators:
        fn = decorator(fn)
    return fn


# Blockstore option


def _split_with_escaping(txt: str, delimiter: str) -> list[str]:
    """
    Simple delimiter-separated list with backslash escaping.
    Rules (considering `:` as delimiter and `<bs>` to be the backslash character):
    `<bs>:` -> `:`
    `<bs><bs>` -> `<bs>`
    `<bs>whatever` -> `<bs>whatever`
    """
    parts = []
    current_part = ""
    escaping = False
    for c in txt:
        if escaping:
            if c not in ("\\", delimiter):
                current_part += "\\"
            current_part += c
        elif c == "\\":
            escaping = True
            continue
        elif c == delimiter:
            parts.append(current_part)
            current_part = ""
        else:
            current_part += c
        escaping = False
    if escaping:
        current_part += "\\"
    parts.append(current_part)
    return parts


def _parse_blockstore_param(value: str) -> BaseBlockStoreConfig:
    if value.upper() == "MOCKED":
        return MockedBlockStoreConfig()
    elif value.upper() == "POSTGRESQL":
        return PostgreSQLBlockStoreConfig()
    else:
        parts = _split_with_escaping(value, delimiter=":")
        if parts[0].upper() == "S3":
            try:
                endpoint_url, region, bucket, key, secret = parts[1:]
            except ValueError:
                raise click.BadParameter(
                    "Invalid S3 config, must be `s3:[<endpoint_url>]:<region>:<bucket>:<key>:<secret>`"
                )
            # Provide https by default to avoid annoying escaping for most cases
            if (
                endpoint_url
                and not endpoint_url.startswith("http://")
                and not endpoint_url.startswith("https://")
            ):
                endpoint_url = f"https://{endpoint_url}"
            return S3BlockStoreConfig(
                s3_endpoint_url=endpoint_url or None,
                s3_region=region,
                s3_bucket=bucket,
                s3_key=key,
                s3_secret=secret,
            )

        elif parts[0].upper() == "SWIFT":
            try:
                auth_url, tenant, container, user, password = parts[1:]
            except ValueError:
                raise click.BadParameter(
                    "Invalid SWIFT config, must be `swift:<auth_url>:<tenant>:<container>:<user>:<password>`"
                )
            # Provide https by default to avoid annoying escaping for most cases
            if (
                auth_url
                and not auth_url.startswith("http://")
                and not auth_url.startswith("https://")
            ):
                auth_url = f"https://{auth_url}"
            return SWIFTBlockStoreConfig(
                swift_authurl=auth_url,
                swift_tenant=tenant,
                swift_container=container,
                swift_user=user,
                swift_password=password,
            )
        else:
            raise click.BadParameter(f"Invalid blockstore type `{parts[0]}`")


def _parse_blockstore_params(raw_params: Iterable[str]) -> BaseBlockStoreConfig:
    raid_configs = defaultdict(list)
    for raw_param in raw_params:
        raid_mode: str | None
        raid_node: int | None
        raw_param_parts = raw_param.split(":", 2)
        if raw_param_parts[0].upper() in ("RAID0", "RAID1", "RAID5") and len(raw_param_parts) == 3:
            raid_mode, raw_raid_node, node_param = raw_param_parts
            try:
                raid_node = int(raw_raid_node)
            except ValueError:
                raise click.BadParameter(f"Invalid node index `{raw_raid_node}` (must be integer)")
        else:
            raid_mode = raid_node = None
            node_param = raw_param
        raid_configs[raid_mode].append((raid_node, node_param))

    if len(raid_configs) != 1:
        config_types = [k if k else v[0][1] for k, v in raid_configs.items()]
        raise click.BadParameter(
            f"Multiple blockstore config with different types: {'/'.join(config_types)}"
        )

    raid_mode, raid_params = next(iter(raid_configs.items()))
    if not raid_mode:
        if len(raid_params) == 1:
            return _parse_blockstore_param(raid_params[0][1])
        else:
            raise click.BadParameter("Multiple blockstore configs only available for RAID mode")

    blockstores = []
    for x in count(0):
        if x == len(raid_params):
            break

        x_node_params = [node_param for raid_node, node_param in raid_params if raid_node == x]
        if len(x_node_params) == 0:
            raise click.BadParameter(f"Missing node index `{x}` in RAID config")
        elif len(x_node_params) > 1:
            raise click.BadParameter(f"Multiple configuration for node index `{x}` in RAID config")
        blockstores.append(_parse_blockstore_param(x_node_params[0]))

    if raid_mode.upper() == "RAID0":
        return RAID0BlockStoreConfig(blockstores=blockstores)
    elif raid_mode.upper() == "RAID1":
        return RAID1BlockStoreConfig(blockstores=blockstores)
    elif raid_mode.upper() == "RAID5":
        return RAID5BlockStoreConfig(blockstores=blockstores)
    else:
        raise click.BadParameter(f"Invalid multi blockstore mode `{raid_mode}`")


def blockstore_server_options[**P, T](fn: Callable[P, T]) -> Callable[P, T]:
    decorators = [
        click.option(
            "--blockstore",
            "-b",
            required=True,
            multiple=True,
            callback=lambda _ctx, _param, value: _parse_blockstore_params(value),
            envvar="PARSEC_BLOCKSTORE",
            show_envvar=True,
            metavar="CONFIG",
            help="""Blockstore configuration.
Allowed values:

\b
-`MOCKED`: Mocked in memory
-`POSTGRESQL`: Use the database specified in the `--db` param
-`s3:[<endpoint_url>]:<region>:<bucket>:<key>:<secret>`: Use S3 storage
-`swift:<auth_url>:<tenant>:<container>:<user>:<password>`: Use SWIFT storage

Note endpoint_url/auth_url are considered as https by default (e.g.
`s3:foo.com:[...]` -> https://foo.com).
Escaping must be used to provide a custom scheme (e.g. `s3:http\\://foo.com:[...]`).

On top of that, multiple blockstore configurations can be provided to form a
RAID0/1/5 cluster.

Each configuration must be provided with the form
`<raid_type>:<node>:<config>` with `<raid_type>` RAID0/RAID1/RAID5, `<node>` a
integer and `<config>` the MOCKED/POSTGRESQL/S3/SWIFT config.
""",
        )
    ]
    for decorator in decorators:
        fn = decorator(fn)
    return fn
