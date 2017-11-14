import attr
from decouple import config



@attr.s(frozen=True)
class Config:
    attr.ib()
CONFIG = {
    'SERVER_PUBLIC': '',
    'HOST': '127.0.0.1',
    'PORT': 9999,
    'CLIENTS_SOCKET_URL': environ.get('CLIENTS_SOCKET_URL', 'tcp://localhost:9090'),
    'BACKEND_URL': environ.get('BACKEND_URL', ''),
    'ANONYMOUS_PUBKEY': 'y4scJ4mV09t5FJXtjwTctrpFg+xctuCyh+e4EoyuDFA=',
    'ANONYMOUS_PRIVKEY': 'ua1CbOtQ0dUrWG0+Satf2SeFpQsyYugJTcEB4DNIu/c=',
    'LOCAL_STORAGE_DIR': ''
}


# BASE_DIR = Path(__file__).parent

DEBUG = config('DEBUG', default=False, cast=bool)
TEMPLATE_DEBUG = DEBUG

DATABASES = {
    'default': config(
        'DATABASE_URL',
        default='sqlite:///foo.sqlite3',
        # cast=db_url
    )
}

TIME_ZONE = 'America/Sao_Paulo'
USE_L10N = True
USE_TZ = True

SECRET_KEY = config('SECRET_KEY')

EMAIL_HOST = config('EMAIL_HOST', default='localhost')
EMAIL_PORT = config('EMAIL_PORT', default=25, cast=int)
EMAIL_HOST_PASSWORD = config('EMAIL_HOST_PASSWORD', default='')
EMAIL_HOST_USER = config('EMAIL_HOST_USER', default='')
EMAIL_USE_TLS = config('EMAIL_USE_TLS', default=False, cast=bool)
