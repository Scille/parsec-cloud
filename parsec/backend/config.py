from decouple import config
import attr


@attr.s(slots=True, frozen=True)
class Config:
    debug = attr.ib(
        default=config('DEBUG', cast=bool, default=False)
    )
    blockstore_url = attr.ib(
        default=config('PARSEC_BLOCKSTORE_URL', default=None)
    )
    host = attr.ib(
        default=config('PARSEC_BACKEND_HOST', default=None)
    )
    port = attr.ib(
        default=config('PARSEC_BACKEND_PORT', cast=int, default=6777)
    )
    handshake_challenge_size = attr.ib(
        default=config('CONFIG_HANDSHAKE_CHALLENGE_SIZE', cast=int, default=48)
    )
