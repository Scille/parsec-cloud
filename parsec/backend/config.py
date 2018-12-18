import re
import attr
import itertools
from typing import List, Union
from collections import defaultdict


__all__ = ("config_factory", "BackendConfig", "BaseBlockstoreConfig")


blockstore_environ_vars = {
    "S3": ["S3_REGION", "S3_BUCKET", "S3_KEY", "S3_SECRET"],
    "SWIFT": ["SWIFT_AUTHURL", "SWIFT_TENANT", "SWIFT_CONTAINER", "SWIFT_USER", "SWIFT_PASSWORD"],
}


def _extract_s3_blockstore_config(environ):
    config = S3BlockstoreConfig()
    needed_vars = blockstore_environ_vars["S3"]
    for key in needed_vars:
        try:
            config.__dict__[key.lower()] = environ[key]
        except KeyError:
            raise ValueError(
                f"Blockstore `S3` requires environment variables: {', '.join(needed_vars)}"
            )
    return config


def _extract_swift_blockstore_config(environ):
    config = SWIFTBlockstoreConfig()
    needed_vars = blockstore_environ_vars["SWIFT"]
    for key in needed_vars:
        try:
            config.__dict__[key.lower()] = environ[key]
        except KeyError:
            raise ValueError(
                f"Blockstore `SWIFT` requires environment variables: {', '.join(needed_vars)}"
            )
    return config


def _extract_raid0_blockstore_config(environ):
    return _extract_raid_blockstore_config(0, environ)


def _extract_raid1_blockstore_config(environ):
    return _extract_raid_blockstore_config(1, environ)


def _extract_raid_blockstore_config(type, environ):
    assert type in (0, 1)

    blockstore_configs = []
    flat_blockstore_environ_vars = ["TYPE"] + [
        var for familly in blockstore_environ_vars.values() for var in familly
    ]
    raid1_regex = re.compile(f"RAID{type}_([0-9]+)_({'|'.join(flat_blockstore_environ_vars)})")

    config = defaultdict(dict)
    for key, value in environ.items():
        match = raid1_regex.match(key)
        if not match:
            continue
        count, var = match.groups()
        config[int(count)][var] = value

    if not config:
        raise ValueError(
            f"Missing `RAID{type}_{{index}}_{{suffix}}` environ vars\n"
            f"Possible suffixes: {', '.join(flat_blockstore_environ_vars)}"
        )

    for i in itertools.count():
        if i not in config:
            break
    after_hole_values = [f"RAID{type}_{k}_*" for k in config if k > i]
    if after_hole_values:
        raise ValueError(
            f"Invalid environ vars: {', '.join(after_hole_values)}\n"
            f"RAID{type} environ vars config index should starts at 0 and not contain holes"
        )

    for sub_index, sub_config in config.items():
        try:
            blockstore_configs.append(_extract_blockstore_config(sub_config["TYPE"], sub_config))
        except ValueError as exc:
            raise ValueError(f"Invalid config in `RAID_{sub_index}` config:\n{str(exc)}")

    return RAIDBlockstoreConfig(type=f"RAID{type}", blockstores=tuple(blockstore_configs))


def _extract_blockstore_config(blockstore_type, environ):
    if blockstore_type == "MOCKED":
        return MockedBlockstoreConfig()
    elif blockstore_type == "POSTGRESQL":
        return PostgreSQLBlockstoreConfig()
    elif blockstore_type == "S3":
        return _extract_s3_blockstore_config(environ)
    elif blockstore_type == "SWIFT":
        return _extract_swift_blockstore_config(environ)
    elif blockstore_type == "RAID0":
        return _extract_raid0_blockstore_config(environ)
    elif blockstore_type == "RAID1":
        return _extract_raid1_blockstore_config(environ)
    else:
        raise ValueError("BLOCKSTORE_TYPE must be `MOCKED`, `POSTGRESQL`, `S3`, `SWIFT` or `RAID1`")


class BaseBlockstoreConfig:
    pass


@attr.s(frozen=True)
class RAIDBlockstoreConfig(BaseBlockstoreConfig):
    type = attr.ib()

    blockstores: List[BaseBlockstoreConfig] = attr.ib(default=None)


@attr.s(frozen=True)
class S3BlockstoreConfig(BaseBlockstoreConfig):
    type = "S3"

    s3_region = attr.ib(default=None)
    s3_bucket = attr.ib(default=None)
    s3_key = attr.ib(default=None)
    s3_secret = attr.ib(default=None)


@attr.s(frozen=True)
class SWIFTBlockstoreConfig(BaseBlockstoreConfig):
    type = "SWIFT"

    swift_authurl = attr.ib(default=None)
    swift_tenant = attr.ib(default=None)
    swift_container = attr.ib(default=None)
    swift_user = attr.ib(default=None)
    swift_password = attr.ib(default=None)


@attr.s(frozen=True)
class PostgreSQLBlockstoreConfig(BaseBlockstoreConfig):
    type = "POSTGRESQL"


@attr.s(frozen=True)
class MockedBlockstoreConfig(BaseBlockstoreConfig):
    type = "MOCKED"


@attr.s(slots=True, frozen=True, auto_attribs=True)
class BackendConfig:

    db_url: str = None
    db_type: str = None

    blockstore_config: BaseBlockstoreConfig = None

    sentry_url: str = None

    debug: bool = False

    handshake_challenge_size: int = 48


def config_factory(
    db_url: str = "MOCKED", blockstore_type: str = "MOCKED", debug: bool = False, environ: dict = {}
) -> BackendConfig:
    config = {"debug": debug, "db_url": db_url}

    if db_url.startswith("postgresql://"):
        config["db_type"] = "POSTGRESQL"
    elif db_url == "MOCKED":
        config["db_type"] = "MOCKED"
    else:
        raise ValueError("DB_URL must be `MOCKED` or `postgresql://...`")

    config["blockstore_config"] = _extract_blockstore_config(blockstore_type, environ)

    config["sentry_url"] = environ.get("SENTRY_URL") or None

    return BackendConfig(**config)
