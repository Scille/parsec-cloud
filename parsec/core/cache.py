from datetime import datetime

from aiocache import caches

from parsec.tools import get_arg

caches.set_config({
    'default': {
        'cache': "aiocache.SimpleMemoryCache",
        'serializer': {
            'class': "aiocache.serializers.PickleSerializer"
        }
    },
})

cache = caches.get('default')
