# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import click
from typing import Optional, List
from itertools import count
from collections import defaultdict

from parsec.backend.config import (
    BaseBlockStoreConfig,
    MockedBlockStoreConfig,
    PostgreSQLBlockStoreConfig,
    S3BlockStoreConfig,
    SWIFTBlockStoreConfig,
    RAID0BlockStoreConfig,
    RAID1BlockStoreConfig,
    RAID5BlockStoreConfig,
)


def db_backend_options(fn):
    decorators = [
        click.option(
            "--db",
            required=True,
            envvar="PARSEC_DB",
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
            default=5,
            show_default=True,
            envvar="PARSEC_DB_MIN_CONNECTIONS",
            help="Minimal number of connections to the database if using PostgreSQL",
        ),
        click.option(
            "--db-max-connections",
            default=7,
            show_default=True,
            envvar="PARSEC_DB_MAX_CONNECTIONS",
            help="Maximum number of connections to the database if using PostgreSQL",
        ),
    ]
    for decorator in decorators:
        fn = decorator(fn)
    return fn


# Blockstore option


def _split_with_escaping(txt: str) -> List[str]:
    """
    Simple colon (i.e. `:`) escaping with backslash
    Rules (using slash instead of backslash to avoid weird encoding error):
    `<bs>:` -> `:`
    `<bs><bs>` -> `<bs>`
    `<bs>whatever` -> `<bs>whatever`
    """
    parts = []
    current_part = ""
    escaping = False
    for c in txt:
        if escaping:
            if c not in ("\\", ":"):
                current_part += "\\"
            current_part += c
        elif c == "\\":
            escaping = True
            continue
        elif c == ":":
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
        parts = _split_with_escaping(value)
        if parts[0].upper() == "S3":
            try:
                endpoint_url, region, bucket, key, secret = parts[1:]
            except ValueError:
                raise click.BadParameter(
                    "Invalid S3 config, must be `s3:[<endpoint_url>]:<region>:<bucket>:<key>:<secret>`"
                )
            # Provide https by default to avoid anoying escaping for most cases
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
            # Provide https by default to avoid anoying escaping for most cases
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


def _parse_blockstore_params(raw_params: str) -> BaseBlockStoreConfig:
    raid_configs = defaultdict(list)
    for raw_param in raw_params:
        raid_mode: Optional[str]
        raid_node: Optional[int]
        raw_param_parts = raw_param.split(":", 2)
        if raw_param_parts[0].upper() in ("RAID0", "RAID1", "RAID5") and len(raw_param_parts) == 3:
            raid_mode, raw_raid_node, node_param = raw_param_parts
            try:
                raid_node = int(raw_raid_node)
            except ValueError:
                raise click.BadParameter(f"Invalid node index `{raid_node}` (must be integer)")
        else:
            raid_mode = raid_node = None
            node_param = raw_param
        raid_configs[raid_mode].append((raid_node, node_param))

    if len(raid_configs) != 1:
        config_types = [k if k else v[0][1] for k, v in raid_configs.items()]
        raise click.BadParameter(
            f"Multiple blockstore config with different types: {'/'.join(config_types)}"
        )

    raid_mode, raid_params = list(raid_configs.items())[0]
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


def blockstore_backend_options(fn):
    decorators = [
        click.option(
            "--blockstore",
            "-b",
            required=True,
            multiple=True,
            callback=lambda ctx, param, value: _parse_blockstore_params(value),
            envvar="PARSEC_BLOCKSTORE",
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

\b
""",
        )
    ]
    for decorator in decorators:
        fn = decorator(fn)
    return fn
