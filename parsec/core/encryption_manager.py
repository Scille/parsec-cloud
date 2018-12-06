import attr
import pickle
import hashlib
from typing import Tuple

from parsec.types import DeviceID, UserID
from parsec.crypto import (
    encrypt_for,
    encrypt_for_self,
    decrypt_for,
    encrypt_with_secret_key,
    verify_signature_from,
    decrypt_with_secret_key,
    PublicKey,
    VerifyKey,
)
from parsec.utils import from_jsonb64, to_jsonb64
from parsec.core.local_db import LocalDBMissingEntry
from parsec.core.base import BaseAsyncComponent
from parsec.core.devices_manager import is_valid_user_id
from parsec.api.protocole import user_get_serializer


class EncryptionManagerError(Exception):
    pass


class BackendGetUserError(EncryptionManagerError):
    pass


class UnknownUserError(EncryptionManagerError):
    pass


class UnknownDeviceError(EncryptionManagerError):
    pass


class MessageFormatError(EncryptionManagerError):
    pass


class MessageEncryptionError(EncryptionManagerError):
    pass


class MessageSignatureError(EncryptionManagerError):
    pass


@attr.s(init=False, slots=True)
class RemoteUser:
    user_id = attr.ib()
    user_pubkey = attr.ib()

    def __init__(self, id: str, user_pubkey: bytes):
        assert is_valid_user_id(id)
        self.user_id = id
        self.user_pubkey = PublicKey(user_pubkey)


@attr.s(init=False, slots=True)
class RemoteDevice:
    device_id = attr.ib()
    device_verifykey = attr.ib()

    def __init__(self, device_id: DeviceID, device_verifykey: bytes):
        self.device_id = device_id
        self.device_verifykey = VerifyKey(device_verifykey)

    @property
    def id(self):
        return "%s@%s" % (self.user.user_id, self.name)


class EncryptionManager(BaseAsyncComponent):
    def __init__(self, device, backend_cmds_sender):
        super().__init__()
        self.device = device
        self.backend_cmds_sender = backend_cmds_sender
        self._local_db = device.local_db
        self._mem_cache = {}

    async def _init(self, nursery):
        pass

    async def _teardown(self):
        pass

    async def _populate_remote_user_cache(self, user_id: UserID):
        async with self.backend_cmds_pool.acquire() as cmds:
            user = cmds.user_get(user_id)
            # TODO: handle exception...
        # raw_rep = await self.backend_cmds_sender.send({"cmd": "user_get", "user_id": user_id})
        # try:
        #     rep, errors = user_get_serializer.rep_load(raw_rep)
        # except CmdRepError as exc:
        #     if exc.rep["status"] == "not_found":
        #         # User doesn't exit, nothing to populate then
        #         return
        #     raise

        user_data = {
            "user_id": rep["user_id"],
            "broadcast_key": to_jsonb64(rep["broadcast_key"]),
            "devices": {k: to_jsonb64(v["verify_key"]) for k, v in rep["devices"].items()},
        }
        # TODO: use schema here
        raw_user_data = pickle.dumps(user_data)
        self._local_db.set(self._build_remote_user_local_access(user_id), raw_user_data)

    def _build_remote_user_local_access(self, user_id: UserID):
        return {
            "id": hashlib.sha256(user_id.encode("utf8")).hexdigest(),
            "key": self.device.local_symkey,
        }

    def _fetch_remote_user_from_local(self, user_id: UserID):
        try:
            raw_user_data = self._local_db.get(self._build_remote_user_local_access(user_id))
            user_data = pickle.loads(raw_user_data)
            return RemoteUser(user_id, from_jsonb64(user_data["broadcast_key"]))

        except LocalDBMissingEntry as exc:
            return None

    def _fetch_remote_device_from_local(self, device_id: DeviceID):
        try:
            raw_user_data = self._local_db.get(
                self._build_remote_user_local_access(device_id.user_id)
            )
            user_data = pickle.loads(raw_user_data)
            try:
                device_b64_pubkey = user_data["devices"][device_id.device_name]
            except KeyError:
                return None
            return RemoteDevice(device_id, from_jsonb64(device_b64_pubkey))
            user_data["devices"] = {k: from_jsonb64(v) for k, v in user_data["devices"].items()}
            return RemoteUser(device_id.user_id, from_jsonb64(user_data["broadcast_key"]))

        except LocalDBMissingEntry as exc:
            return None

    async def fetch_remote_device(self, device_id: DeviceID) -> RemoteDevice:
        """
        Retrieve a device from the backend.

        Returns: The device or None if it couldn't be found.

        Raises:
            parsec.core.backend_connection.BackendNotAvailable: if the backend is offline.
            BackendGetUserError: if the response returned by the backend is invalid.
            parsec.crypto.CryptoError: if the device returnded by the backend is corrupted or invalid.
        """
        # First, try the quick win with the memory cache
        try:
            return self._mem_cache[device_id]
        except KeyError:
            pass

        if device_id == self.device.id:
            remote_device = RemoteDevice(device_id, self.device.device_verifykey.encode())
        else:
            # First try to retrieve from the local cache
            remote_device = self._fetch_remote_device_from_local(device_id)
            if not remote_device:
                # Cache miss ! Fetch data from the backend and retry
                await self._populate_remote_user_cache(device_id.user_id)
                remote_device = self._fetch_remote_device_from_local(device_id)
                if not remote_device:
                    # Still nothing found, the device doesn't exist
                    return None

        self._mem_cache[device_id] = remote_device
        return remote_device

    async def fetch_remote_user(self, user_id: UserID) -> RemoteUser:
        """
        Retrieve a user from the backend.

        Returns: The user or None if it couldn't be found.

        Raises:
            parsec.core.backend_connection.BackendNotAvailable: if the backend is offline.
            BackendGetUserError: if the response returned by the backend is invalid.
            parsec.crypto.CryptoError: if the user returnded by the backend is corrupted or invalid.
        """
        # First, try the quick win with the memory cache
        try:
            return self._mem_cache[user_id]
        except KeyError:
            pass

        # Now try to retrieve from the local cache
        if user_id == self.device.user_id:
            remote_user = RemoteUser(self.device.user_id, self.device.user_pubkey.encode())
        else:
            remote_user = self._fetch_remote_user_from_local(user_id)
            if not remote_user:
                # Cache miss ! Fetch data from the backend and retry
                await self._populate_remote_user_cache(user_id)
                remote_user = self._fetch_remote_user_from_local(user_id)
                if not remote_user:
                    # Still nothing found, the device doesn't exist
                    return None

        self._mem_cache[user_id] = remote_user
        return remote_user

    async def encrypt_for(self, recipient: str, data: bytes) -> bytes:
        """
        Raises:
                parsec.core.backend_connection.BackendNotAvailable: if the backend is offline.
                BackendGetUserError: if the response returned by the backend is invalid.
                parsec.crypto.CryptoError: if the user returnded by the backend is corrupted or invalid.
        """
        user = await self.fetch_remote_user(recipient)
        if not user:
            raise MessageEncryptionError("Unknown recipient `%s`" % recipient)
        return encrypt_for(self.device.id, self.device.device_signkey, user.user_pubkey, data)

    async def encrypt_for_self(self, data: bytes) -> bytes:
        return encrypt_for_self(
            self.device.id, self.device.device_signkey, self.device.user_pubkey, data
        )

    async def decrypt_for_self(self, ciphered: bytes) -> Tuple[DeviceID, bytes]:
        device_id, signed_data = decrypt_for(self.device.user_privkey, ciphered)
        author_device = await self.fetch_remote_device(device_id)
        if not author_device:
            raise MessageSignatureError(
                f"Message is said to be signed by `{device_id}`, but this device cannot be found on the backend."
            )
        return (device_id, verify_signature_from(author_device.device_verifykey, signed_data))

    def encrypt_with_secret_key(self, key: bytes, data: bytes) -> bytes:
        return encrypt_with_secret_key(self.device.id, self.device.device_signkey, key, data)

    async def decrypt_with_secret_key(self, key: bytes, ciphered: bytes) -> dict:
        device_id, signed = decrypt_with_secret_key(key, ciphered)
        author_device = await self.fetch_remote_device(device_id)
        if not author_device:
            raise MessageSignatureError(
                f"Message is said to be signed by `{device_id}`, but this device cannot be found on the backend."
            )
        return verify_signature_from(author_device.device_verifykey, signed)
