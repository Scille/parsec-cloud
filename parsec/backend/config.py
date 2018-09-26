from os import environ
import attr


def config_factory(raw_conf):
    raw_conf = {k.upper(): v for k, v in raw_conf.items() if v not in (None, "")}

    for mandatory in ("BLOCKSTORE_URL", "DB_URL"):
        if not raw_conf.get(mandatory):
            raise ValueError(f"Missing mandatory config {mandatory}")

    if "SENTRY_URL" not in raw_conf:
        raw_conf["SENTRY_URL"] = environ.get("SENTRY_URL") or None

    return BackendConfig(**{k.lower(): v for k, v in raw_conf.items()})


@attr.s(slots=True, frozen=True)
class BackendConfig:

    blockstore_url = attr.ib()

    @blockstore_url.validator
    def _validate_blockstore_url(self, field, val):
        if val == "MOCKED":
            return val
        elif val == "POSTGRESQL":
            return val
        elif val.startswith("s3:") and len(val.split(":")) == 5:
            return val
        elif val.startswith("openstack:") and len(val.split(":")) == 6:
            return val
        raise ValueError(
            "BLOCKSTORE_URL must be `MOCKED`, `POSTGRESQL`, `s3:<region>:<bucket>:<key>:<secret>`,"
            " or `openstack:<auth_url>:<tenant>:<container>:<user>:<password>`"
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
