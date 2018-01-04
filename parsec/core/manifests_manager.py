import attr
import json
from nacl.public import Box
from nacl.secret import SecretBox
from nacl.exceptions import BadSignatureError, CryptoError

from parsec.core.schemas import TypedManifestSchema
from parsec.utils import ParsecError


class ManifestDecryptionError(ParsecError):
    status = 'invalid_signature'


class ManifestsManager:
    def __init__(self, user, local_storage, backend_storage):
        self.user = user
        self._local_storage = local_storage
        self._backend_storage = backend_storage

    def _encrypt_manifest(self, key, manifest):
        raw = json.dumps(manifest).encode()
        box = SecretBox(key)
        # signed = self.user.signkey.sign(raw)
        # return box.encrypt(signed)
        return box.encrypt(raw)

    def _decrypt_manifest(self, key, blob):
        box = SecretBox(key)
        try:
            raw = box.decrypt(blob)
            # signed = box.decrypt(blob)
            # raw = self.user.verifykey.verify(signed)
        except (BadSignatureError, CryptoError, ValueError):
            raise ManifestDecryptionError()
        return json.loads(raw.decode())

    def _encrypt_user_manifest(self, manifest):
        raw = json.dumps(manifest).encode()
        box = Box(self.user.privkey, self.user.pubkey)
        return box.encrypt(raw)

    def _decrypt_user_manifest(self, blob):
        box = Box(self.user.privkey, self.user.pubkey)
        try:
            raw = box.decrypt(blob)
        except (BadSignatureError, CryptoError, ValueError):
            raise ManifestDecryptionError()
        return json.loads(raw.decode())

    async def fetch_user_manifest_from_backend(self, version=None):
        blob = await self._backend_storage.fetch_user_manifest(version=version)
        if blob:
            decrypted = self._decrypt_user_manifest(blob)
            data, _ = TypedManifestSchema(strict=True).load(decrypted)
            return data

    async def fetch_user_manifest_from_local(self):
        blob = self._local_storage.fetch_user_manifest()
        if blob:
            decrypted = self._decrypt_user_manifest(blob)
            data, _ = TypedManifestSchema(strict=True).load(decrypted)
            return data

    async def flush_user_manifest_on_local(self, manifest):
        data, _ = TypedManifestSchema(strict=True).dump(manifest)
        blob = self._encrypt_user_manifest(data)
        self._local_storage.flush_user_manifest(blob)

    async def sync_user_manifest_with_backend(self, manifest):
        data, _ = TypedManifestSchema(strict=True).dump(manifest)
        blob = self._encrypt_user_manifest(data)
        self._backend_storage.sync_user_manifest(manifest['version'], blob)

    async def fetch_from_local(self, id, key):
        blob = self._local_storage.fetch_manifest(id)
        if blob:
            decrypted = self._decrypt_manifest(key, blob)
            data, _ = TypedManifestSchema(strict=True).load(decrypted)
            return data

    async def fetch_from_backend(self, id, rts, key, version=None):
        blob = await self._backend_storage.fetch_manifest(id, rts, version=version)
        if blob:
            # TODO: store cache in local ?
            decrypted = self._decrypt_manifest(key, blob)
            data, _ = TypedManifestSchema(strict=True).load(decrypted)
            return data

    async def flush_on_local(self, id, key, manifest):
        data, _ = TypedManifestSchema(strict=True).dump(manifest)
        blob = self._encrypt_manifest(key, data)
        self._local_storage.flush_manifest(id, blob)

    async def sync_new_entry_with_backend(self, key, manifest):
        data, _ = TypedManifestSchema(strict=True).dump(manifest)
        blob = self._encrypt_manifest(key, data)
        return await self._backend_storage.sync_new_manifest(blob)

    async def sync_with_backend(self, id, wts, key, manifest):
        version = manifest['version']
        data, _ = TypedManifestSchema(strict=True).dump(manifest)
        blob = self._encrypt_manifest(key, data)
        await self._backend_storage.sync_manifest(id, wts, version, blob)
