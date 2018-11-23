import re
import attr
import itertools
from collections import defaultdict

from parsec.utils import decode_urlsafe_root_verify_key


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


def config_factory(db_url="MOCKED", blockstore_type="MOCKED", debug=False, environ={}):
    raw_conf = {**environ, "DB_URL": db_url, "BLOCKSTORE_TYPE": blockstore_type, "DEBUG": debug}

    config = BackendConfig(debug=raw_conf.get("DEBUG", False))

    if "DB_URL" not in raw_conf:
        raise ValueError("Missing mandatory config `DB_URL`")
    config.__dict__["db_url"] = raw_conf["DB_URL"]
    if config.db_url.startswith("postgresql://"):
        config.__dict__["db_type"] = "POSTGRESQL"
    elif config.db_url == "MOCKED":
        config.__dict__["db_type"] = "MOCKED"
    else:
        raise ValueError("DB_URL must be `MOCKED` or `postgresql://...`")

    if "BLOCKSTORE_TYPE" not in raw_conf:
        raise ValueError("Missing mandatory config `BLOCKSTORE_TYPE`")
    config.__dict__["blockstore_config"] = _extract_blockstore_config(
        raw_conf["BLOCKSTORE_TYPE"], raw_conf
    )

    if "ROOT_VERIFY_KEY" not in raw_conf:
        raise ValueError("Missing mandatory environ variable `ROOT_VERIFY_KEY`")
    try:
        config.__dict__["root_verify_key"] = decode_urlsafe_root_verify_key(
            raw_conf.get("ROOT_VERIFY_KEY")
        )
    except Exception as exc:
        raise ValueError("Invalid `ROOT_VERIFY_KEY` environ variable") from exc

    config.__dict__["sentry_url"] = raw_conf.get("SENTRY_URL") or None

    return config


@attr.s(frozen=True)
class RAIDBlockstoreConfig:
    type = attr.ib()

    blockstores = attr.ib(default=None)


@attr.s(frozen=True)
class S3BlockstoreConfig:
    type = "S3"

    s3_region = attr.ib(default=None)
    s3_bucket = attr.ib(default=None)
    s3_key = attr.ib(default=None)
    s3_secret = attr.ib(default=None)


@attr.s(frozen=True)
class SWIFTBlockstoreConfig:
    type = "SWIFT"

    swift_authurl = attr.ib(default=None)
    swift_tenant = attr.ib(default=None)
    swift_container = attr.ib(default=None)
    swift_user = attr.ib(default=None)
    swift_password = attr.ib(default=None)


@attr.s(frozen=True)
class PostgreSQLBlockstoreConfig:
    type = "POSTGRESQL"


@attr.s(frozen=True)
class MockedBlockstoreConfig:
    type = "MOCKED"


@attr.s(frozen=True)
class BackendConfig:

    db_url = attr.ib(default=None)
    db_type = attr.ib(default=None)

    root_verify_key = attr.ib(default=None)

    blockstore_config = attr.ib(default=None)

    sentry_url = attr.ib(default=None)

    debug = attr.ib(default=False)

    handshake_challenge_size = attr.ib(default=48)
