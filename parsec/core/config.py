from os import environ


CONFIG = {
    'SERVER_PUBLIC': '',
    'ADDR': environ.get('ADDR', 'tcp://127.0.0.1:6777'),
    'BACKEND_ADDR': environ.get('BACKEND_ADDR', ''),
    'ANONYMOUS_PUBKEY': 'y4scJ4mV09t5FJXtjwTctrpFg+xctuCyh+e4EoyuDFA=',
    'ANONYMOUS_PRIVKEY': 'ua1CbOtQ0dUrWG0+Satf2SeFpQsyYugJTcEB4DNIu/c=',
    'LOCAL_STORAGE_DIR': ''
}
