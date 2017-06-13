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


def cached_block(method):

    async def inner(*args, **kwargs):
        method_name = method.__name__
        response = None
        if method_name == 'create':
            content = get_arg(method, 'content', args, kwargs)
            response = await method(*args, **kwargs)
            date = datetime.utcnow().isoformat()
            cached_content = {'status': 'ok', 'creation_date': date}
            await cache.set('stat:' + response, cached_content)
            cached_content['content'] = content
            await cache.set('read:' + response, cached_content)
        elif method_name in ['read', 'stat']:
            id = get_arg(method, 'id', args, kwargs)
            response = await cache.get((method_name, id))
            if not response:
                response = await method(*args, **kwargs)
                await cache.set(method_name + ':' + id, response)
        else:
            response = await method(*args, **kwargs)
        return response

    return inner


def cached_vlob(method):

    async def inner(*args, **kwargs):
        method_name = method.__name__
        response = None
        if method_name == 'vlob_create':
            blob = get_arg(method, 'blob', args, kwargs)
            response = await method(*args, **kwargs)
            cached_content = {'status': 'ok', 'id': response['id'], 'blob': blob, 'version': 1}
            await cache.set(response['id'] + ':1', cached_content)
        elif method_name == 'user_vlob_read':
            version = get_arg(method, 'version', args, kwargs)
            if version:
                response = await cache.get('USER:' + str(version))
            if not response:
                response = await method(*args, **kwargs)
                await cache.set('USER:' + str(response['version']), response)
        elif method_name == 'vlob_read':
            id = get_arg(method, 'id', args, kwargs)
            version = get_arg(method, 'version', args, kwargs)
            if version:
                response = await cache.get(id + ':' + str(version))
            if not response:
                response = await method(*args, **kwargs)
                await cache.set(id + ':' + str(response['version']), response)
        elif method_name == 'user_vlob_update':
            blob = get_arg(method, 'blob', args, kwargs)
            version = get_arg(method, 'version', args, kwargs)
            response = await method(*args, **kwargs)
            cached_content = {'status': 'ok', 'blob': blob, 'version': version}
            await cache.set('USER:' + str(version), cached_content)
        elif method_name == 'vlob_update':
            id = get_arg(method, 'id', args, kwargs)
            blob = get_arg(method, 'blob', args, kwargs)
            version = get_arg(method, 'version', args, kwargs)
            response = await method(*args, **kwargs)
            cached_content = {'status': 'ok', 'id': id, 'blob': blob, 'version': version}
            await cache.set(id + ':' + str(version), cached_content)
        else:
            response = await method(*args, **kwargs)
        return response

    return inner
