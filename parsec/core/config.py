import attr
from decouple import config
from os.path import expandvars


# TODO: add required=True option


def _cast_int(v):
    return int(v) if v is not None else None


@attr.s(slots=True, frozen=True)
class CoreConfig:
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
    local_storage_dir = attr.ib(
        default=config('PARSEC_LOCAL_STORAGE_DIR', default='')
    )
    base_settings_path = attr.ib(
        default=config('BASE_SETTINGS_PATH', default=expandvars('$HOME/.config/parsec'))
    )
