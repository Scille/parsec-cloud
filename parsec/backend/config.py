import attr
from dynaconf import settings


@attr.s(slots=True, frozen=True)
class BackendConfig:

    blockstore_db_url = attr.ib(factory=lambda: settings.BLOCKSTORE_DB_URL)

    @blockstore_db_url.validator
    def _validate_blockstore_db_url(self, field, val):
        if val == "MOCKED":
            return val
        elif val == "POSTGRESQL":
            return val
        elif val.startswith("s3:") and len(val.split(":")) == 5:
            return val
        elif val.startswith("openstack:") and len(val.split(":")) == 5:
            return val
        raise ValueError(
            "BLOCKSTORE_DB_URL must be `MOCKED`, `POSTGRESQL`, `s3:<region>:<bucket>:<key>:<secret>`,"
            " or `openstack:<auth_url>:<container>:<user>:<tenant>:<password>:"
        )

    metadata_db_url = attr.ib(factory=lambda: settings.METADATA_DB_URL)

    @metadata_db_url.validator
    def _validate_metadata_db_url(self, field, val):
        if val == "MOCKED":
            return val
        elif val.startswith("postgresql://"):
            return val
        raise ValueError("METADATA_DB_URL must be `MOCKED` or `postgresql://...`")

    sentry_url = attr.ib(default=settings.get("SENTRY_URL"))

    debug = attr.ib(default=settings.as_bool("DEBUG"))
    handshake_challenge_size = attr.ib(
        settings.get("CONFIG_HANDSHAKE_CHALLENGE_SIZE", cast="@int", default=48)
    )
