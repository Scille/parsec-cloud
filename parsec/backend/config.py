import attr
from decouple import config


@attr.s(slots=True, frozen=True)
class Config:
    debug = attr.ib(
        default=config('DEBUG', cast=bool, default=False)
    )
    blockstore_url = attr.ib(
        default=config('BLOCKSTORE_URL', default=None)
    )
    host = attr.ib(
        default=config('BACKEND_HOST', default=None)
    )
    port = attr.ib(
        default=config('BACKEND_PORT', cast=int, default=6777)
    )
    handshake_challenge_size = attr.ib(
        default=config('CONFIG_HANDSHAKE_CHALLENGE_SIZE', cast=int, default=48)
    )
    dburl = attr.ib(
        default=config('PARSEC_DB_URL', default=None)
    )


# TODO: rename Config -> BackendConfig
BackendConfig = Config
