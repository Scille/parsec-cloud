from decouple import config
import attr


def _cast_int(v):
    return int(v) if v is not None else None


@attr.s(slots=True, frozen=True)
class Config:
    debug = attr.ib(
        default=config('DEBUG', cast=bool, default=False)
    )
    server_public = attr.ib(
        default=config('PARSEC_SERVER_PUBLIC', default='')
    )
    addr = attr.ib(
        default=config('PARSEC_ADDR', default='tcp://127.0.0.1:6777')
    )
    backend_addr = attr.ib(
        default=config('PARSEC_BACKEND_ADDR', default='')
    )
    backend_watchdog = attr.ib(
        default=config('PARSEC_BACKEND_WATCHDOG', cast=_cast_int, default=None)
    )
    anonymous_pubkey = attr.ib(
        default=config(
            'PARSEC_ANONYMOUS_PUBKEY',
            default='y4scJ4mV09t5FJXtjwTctrpFg+xctuCyh+e4EoyuDFA='
        )
    )
    anonymous_privkey = attr.ib(
        default=config(
            'PARSEC_ANONYMOUS_PRIVKEY',
            default='ua1CbOtQ0dUrWG0+Satf2SeFpQsyYugJTcEB4DNIu/c='
        )
    )
    local_storage_dir = attr.ib(
        default=config('PARSEC_LOCAL_STORAGE_DIR', default='')
    )
