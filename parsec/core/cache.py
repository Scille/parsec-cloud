import sys
from datetime import datetime

from cachetools import LRUCache

from parsec.tools import get_arg


cache = LRUCache(maxsize=sys.maxsize)


def cached_block(method):

    def inner(*args, **kwargs):
        method_name = method.__name__
        response = None
        if method_name == 'create':
            content = get_arg(method, 'content', args, kwargs)
            response = method(*args, **kwargs)
            date = datetime.utcnow().isoformat()
            cached_content = {'status': 'ok', 'creation_date': date}
            cache['stat:' + response] = cached_content
            cached_content['content'] = content
            cache['read:' + response] = cached_content
        elif method_name in ['read', 'stat']:
            id = get_arg(method, 'id', args, kwargs)
            response = cache.get((method_name, id))
            if not response:
                response = method(*args, **kwargs)
                cache[method_name + ':' + id] = response
        else:
            response = method(*args, **kwargs)
        return response

    return inner


def cached_vlob(method):

    def inner(*args, **kwargs):
        method_name = method.__name__
        response = None
        if method_name == 'vlob_create':
            blob = get_arg(method, 'blob', args, kwargs)
            response = method(*args, **kwargs)
            cached_content = {'status': 'ok', 'id': response['id'], 'blob': blob, 'version': 1}
            cache[response['id'] + ':1'] = cached_content
        elif method_name == 'user_vlob_read':
            version = get_arg(method, 'version', args, kwargs)
            if version:
                response = cache['USER:' + str(version)]
            if not response:
                response = method(*args, **kwargs)
                cache['USER:' + str(response['version'])] = response
        elif method_name == 'vlob_read':
            id = get_arg(method, 'id', args, kwargs)
            version = get_arg(method, 'version', args, kwargs)
            if version:
                response = cache[id + ':' + str(version)]
            if not response:
                response = method(*args, **kwargs)
                cache[id + ':' + str(response['version'])] = response
        elif method_name == 'user_vlob_update':
            blob = get_arg(method, 'blob', args, kwargs)
            version = get_arg(method, 'version', args, kwargs)
            response = method(*args, **kwargs)
            cached_content = {'status': 'ok', 'blob': blob, 'version': version}
            cache['USER:' + str(version)] = cached_content
        elif method_name == 'vlob_update':
            id = get_arg(method, 'id', args, kwargs)
            blob = get_arg(method, 'blob', args, kwargs)
            version = get_arg(method, 'version', args, kwargs)
            response = method(*args, **kwargs)
            cached_content = {'status': 'ok', 'id': id, 'blob': blob, 'version': version}
            cache[id + ':' + str(version)] = cached_content
        else:
            response = method(*args, **kwargs)
        return response

    return inner
