from base64 import decodebytes, encodebytes

from parsec.service import BaseService, cmd
from parsec.exceptions import ParsecError


class CryptoError(ParsecError):
    pass


class SymCryptoError(ParsecError):
    status = 'sym_crypto_error'


class AsymCryptoError(ParsecError):
    status = 'asym_crypto_error'


class BaseCryptoService(BaseService):

    @staticmethod
    def _get_field(msg, field, type_=str, default=None):
        value = msg.get(field)
        if value is None:
            if default is not None:
                return default
            raise CryptoError('bad_params', 'Param `%s` is required.' % field)
        if type_ is bytes:
            try:
                value = decodebytes(value.encode())
            except (TypeError, AttributeError):
                raise CryptoError('bad_params', 'Param `%s` is not valid base64 data.' % field)
        if not isinstance(value, type_):
            raise CryptoError('bad_params', 'Param `%s` must be of type `%s`.' % (field, type_))
        return value

    @cmd('encrypt')
    async def cmd_ENCRYPT(self, msg):
        content = self._get_field(msg, 'content', bytes)
        digest = await self.encrypt(content)
        return {
            'status': 'ok',
            'key': digest['key'],
            'key_signature': digest['key_signature'],
            'content': encodebytes(digest['content']).decode(),
            'signature': digest['signature']
        }

    @cmd('decrypt')
    async def cmd_DECRYPT(self, msg):
        key = self._get_field(msg, 'key')
        key_signature = self._get_field(msg, 'key_signature')
        content = self._get_field(msg, 'content', bytes)
        signature = self._get_field(msg, 'signature')
        dec = await self.decrypt(key, key_signature, content, signature)
        return {'status': 'ok', 'content': encodebytes(dec).decode()}

    @cmd('load_key')
    async def cmd_LOAD_KEY(self, msg):
        key = self._get_field(msg, 'key')
        passphrase = self._get_field(msg, 'passphrase', default='')
        await self.load_key(key, passphrase)
        return {'status': 'ok'}

    @cmd('gen_key')
    async def cmd_GEN_KEY(self, msg):
        key_size = self._get_field(msg, 'key_size', int)
        passphrase = self._get_field(msg, 'passphrase')
        key = await self.gen_key(key_size, passphrase)
        return {'status': 'ok', 'key': key}
