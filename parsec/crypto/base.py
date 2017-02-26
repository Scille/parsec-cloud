from base64 import decodebytes, encodebytes

from parsec.base import BaseService, cmd, ParsecError


class CryptoError(ParsecError):
    pass


class SymCryptoError(ParsecError):
    status = 'sym_crypto_error'


class AsymCryptoError(ParsecError):
    status = 'asym_crypto_error'


class BaseCryptoService(BaseService):

    @staticmethod
    def _get_field(msg, field, type_=str):
        value = msg.get(field)
        if value is None:
            raise CryptoError('bad_params', 'Param `%s` is required' % field)
        if type_ is bytes:
            try:
                value = decodebytes(value.encode())
            except TypeError:
                raise CryptoError('bad_params', 'Param `%s` is not valid base64 data' % field)
        if not isinstance(value, type_):
            raise CryptoError('bad_params', 'Param `%s` must be of type `%s`' % (field, type_))
        return value

    @cmd('encrypt')
    async def cmd_ENCRYPT(self, msg):
        content = self._get_field(msg, 'content', bytes)
        digest = await self.encrypt(content)
        return {'status': 'ok', **digest}

    @cmd('decrypt')
    async def cmd_DECRYPT(self, msg):
        key = self._get_field(msg, 'key')
        key_signature = self._get_field(msg, 'key_signature')
        content = self._get_field(msg, 'content', bytes)
        signature = self._get_field(msg, 'signature')
        dec = await self.decrypt(key, key_signature, content, signature)
        return {'status': 'ok', 'dec': dec}

    @cmd('load_key')
    async def cmd_LOAD_KEY(self, msg):
        key = self._get_field(msg, 'key')
        passphrase = self._get_field(msg, 'passphrase')
        await self.load_key(key, passphrase)
        return {'status': 'ok'}

    @cmd('gen_key')
    async def cmd_GEN_KEY(self, msg):
        key_size = self._get_field(msg, 'key_size', int)
        passphrase = self._get_field(msg, 'passphrase')
        key = await self.gen_key(key_size, passphrase)
        return {'status': 'ok', 'key': key}

    async def encrypt(self, content: bytes):
        aes_key, enc = self._sym.encrypt(content)
        signature = self._asym.sign(enc)
        encrypted_key = self._asym.encrypt(aes_key)
        key_sig = self._asym.sign(encrypted_key)
        return {
            'key': encrypted_key,
            'content': enc,
            'signature': signature,
            'key_signature': key_sig
        }
