from os import environ
import attr


def config_factory(raw_conf):
    raw_conf = {k.upper(): v for k, v in raw_conf.items() if v not in (None, "")}

    for mandatory in ("BLOCKSTORE_TYPE", "DB_URL"):
        if not raw_conf.get(mandatory):
            raise ValueError(f"Missing mandatory config {mandatory}")

    if "SENTRY_URL" not in raw_conf:
        raw_conf["SENTRY_URL"] = environ.get("SENTRY_URL") or None

    return BackendConfig(**{k.lower(): v for k, v in raw_conf.items()})


@attr.s(slots=True, frozen=True)
class BackendConfig:

    blockstore_type = attr.ib()

    @blockstore_type.validator
    def _validate_blockstore_type(self, field, val):
        if val == "MOCKED":
            return val
        elif val == "POSTGRESQL":
            return val
        elif val == "AMAZON_S3" and set(
            [val + "_" + item for item in ["REGION", "BUCKET", "KEY", "SECRET"]]
        ).issubset(environ):
            return val
        elif val == "OPENSTACK_SWIFT" and set(
            [val + "_" + item for item in ["AUTH_URL", "TENANT", "CONTAINER", "USER", "PASSWORD"]]
        ).issubset(environ):
            return val
        raise ValueError(
            "BLOCKSTORE_TYPE must be `MOCKED`, `POSTGRESQL`, `AMAZON_S3`, or `OPENSTACK_SWIFT` and environment variables must be set accordingly"
        )

    db_url = attr.ib()

    @db_url.validator
    def _validate_db_url(self, field, val):
        if val == "MOCKED":
            return val
        elif val.startswith("postgresql://"):
            return val
        raise ValueError("DB_URL must be `MOCKED` or `postgresql://...`")

    sentry_url = attr.ib(default=None)

    debug = attr.ib(default=False)
    handshake_challenge_size = attr.ib(default=48)
