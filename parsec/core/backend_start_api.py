import attr
import aiohttp

from parsec.exceptions import (
    PrivKeyHashCollision, PrivKeyError, PrivKeyNotFound, BackendIdentityRegisterError)
from parsec.crypto import hash_id_password


def backend_to_start_api_url(url, prefix='start'):
    if url.startswith('ws://'):
        return '%s/%s' % (url.replace('ws://', 'http://'), prefix)
    elif url.startswith('wss://'):
        return '%s/%s' % (url.replace('wss://', 'https://'), prefix)
    else:
        raise RuntimeError('Invalid backend url `%s`, should start with `ws://` or `wss://`' % url)


@attr.s
class EBackendCipherKeyAdd:
    id = attr.ib()
    password = attr.ib()
    cipherkey = attr.ib()


@attr.s
class EBackendCipherKeyGet:
    id = attr.ib()
    password = attr.ib()


@attr.s
class EBackendIdentityRegister:
    id = attr.ib()
    pubkey = attr.ib()


@attr.s
class StartAPIComponent:
    url = attr.ib()

    async def perform_identity_register(self, intent):
        route = '%s/pubkey/%s' % (self.url, intent.id)
        async with aiohttp.ClientSession() as session:
            async with session.post(route, data=intent.pubkey) as resp:
                if resp.status != 200:
                    error_msg = await resp.text()
                    raise BackendIdentityRegisterError(error_msg)


    async def perform_cipherkey_add(self, intent):
        hash = hash_id_password(intent.id, intent.password)
        route = '%s/cipherkey/%s' % (self.url, hash)
        async with aiohttp.ClientSession() as session:
            async with session.post(route, data=intent.cipherkey) as resp:
                if resp.status != 200:
                    error_msg = await resp.text()
                    if resp.status == 409:
                        raise PrivKeyHashCollision(error_msg)
                    else:
                        raise PrivKeyError(error_msg)

    async def perform_cipherkey_get(self, intent):
        hash = hash_id_password(intent.id, intent.password)
        route = '%s/cipherkey/%s' % (self.url, hash)
        async with aiohttp.ClientSession() as session:
            async with session.get(route) as resp:
                if resp.status == 200:
                    cipherkey = await resp.read()
                    return cipherkey
                else:
                    error_msg = await resp.text()
                    if resp.status == 404:
                        raise PrivKeyNotFound('Bad id or password')
                    else:
                        raise PrivKeyError(error_msg)
