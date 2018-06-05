import attr
from decouple import config


@attr.s(slots=True, frozen=True)
class BackendConfig:
    debug = attr.ib(default=config("DEBUG", cast=bool, default=False))
    blockstore_postgresql = attr.ib(
        default=config("BLOCKSTORE_POSTGRESQL", cast=bool, default=False)
    )
    blockstore_openstack = attr.ib(default=config("BLOCKSTORE_OPENSTACK", default=None))
    blockstore_s3 = attr.ib(default=config("BLOCKSTORE_S3", default=None))
    host = attr.ib(default=config("BACKEND_HOST", default=None))
    port = attr.ib(default=config("BACKEND_PORT", cast=int, default=6777))
    handshake_challenge_size = attr.ib(
        default=config("CONFIG_HANDSHAKE_CHALLENGE_SIZE", cast=int, default=48)
    )
    dburl = attr.ib(default=config("PARSEC_DB_URL", default=None))
    sentry_url = attr.ib(default=config("SENTRY_URL", default=""))
