# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pickle
import hashlib
from pendulum import Pendulum
from typing import Tuple

from parsec.api.protocole.user import UserSchema
from parsec.trustchain import cascade_validate_devices
from parsec.types import DeviceID, UserID
from parsec.crypto import (
    encrypt_for,
    encrypt_for_self,
    decrypt_for,
    encrypt_with_secret_key,
    verify_signature_from,
    decrypt_with_secret_key,
)
from parsec.core.base import BaseAsyncComponent
from parsec.core.local_db import LocalDBMissingEntry
from parsec.core.types import RemoteDevice, RemoteDevicesMapping, RemoteUser, ManifestAccess
from parsec.core.backend_connection import BackendCmdsBadResponse
from parsec.serde import Serializer


user_schema_serializer = Serializer(UserSchema)


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


class EncryptionManager(BaseAsyncComponent):
    def __init__(self, device, local_db, backend_cmds):
        super().__init__()
        self.device = device
        self.backend_cmds = backend_cmds
        self.local_db = local_db
        self._mem_cache = {}

    async def _init(self, nursery):
        pass

    async def _teardown(self):
        pass

    async def _populate_remote_user_cache(self, user_id: UserID):
        try:
            user, trustchain = await self.backend_cmds.user_get(user_id)
        except BackendCmdsBadResponse as exc:
            if exc.status == "not_found":
                # User doesn't exit, nothing to populate then
                return
            else:
                raise

        new_devices = cascade_validate_devices(
            user, trustchain, self.device.organization_id, self.device.root_verify_key
        )

        new_user = RemoteUser(
            user_id=user.user_id,
            certified_user=user.certified_user,
            user_certifier=user.user_certifier,
            devices=RemoteDevicesMapping(*new_devices),
            created_on=user.created_on,
        )

        # raw = user_schema_serializer.dumps({
        #     'user_id': new_user.user_id,
        #     'is_admin': True,
        #     'created_on': new_user.created_on,
        #     'certified_user': new_user.certified_user,
        #     'user_certifier': new_user.user_certifier,
        #     'devices': new_user.devices
        # })

        # TODO: use schema here
        raw = pickle.dumps(new_user)
        self.local_db.set(self._build_remote_user_local_access(user_id), raw)

    def _build_remote_user_local_access(self, user_id: UserID) -> ManifestAccess:
        return ManifestAccess(
            id=hashlib.sha256(user_id.encode("utf8")).hexdigest(), key=self.device.local_symkey
        )

    def _fetch_remote_user_from_local(self, user_id: UserID):
        try:
            raw_user_data = self.local_db.get(self._build_remote_user_local_access(user_id))
            return pickle.loads(raw_user_data)

        except LocalDBMissingEntry:
            return None

    def _fetch_remote_device_from_local(self, device_id: DeviceID):
        try:
            raw_user_data = self.local_db.get(
                self._build_remote_user_local_access(device_id.user_id)
            )
            user_data = pickle.loads(raw_user_data)
            try:
                return user_data.devices[device_id.device_name]
            except KeyError:
                return None

        except LocalDBMissingEntry:
            return None

    async def fetch_remote_device(self, device_id: DeviceID) -> RemoteDevice:
        """
        Retrieve a device from the backend.

        Returns: The device or None if it couldn't be found.

        Raises:
            parsec.core.backend_connection.BackendNotAvailable: if the backend is offline.
            BackendGetUserError: if the response returned by the backend is invalid.
            parsec.crypto.CryptoError: if the device returnded by the backend
                is corrupted or invalid.
        """
        # First, try the quick win with the memory cache
        try:
            return self._mem_cache[device_id]
        except KeyError:
            pass

        if device_id == self.device.device_id:
            return self.device
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
            return self.device
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
                parsec.crypto.CryptoError: if the user returnded by the backend
                    is corrupted or invalid.
        """
        user = await self.fetch_remote_user(recipient)
        if not user:
            raise MessageEncryptionError("Unknown recipient `%s`" % recipient)
        return encrypt_for(self.device.device_id, self.device.signing_key, user.public_key, data)

    async def encrypt_for_self(self, data: bytes) -> bytes:
        return encrypt_for_self(
            self.device.device_id, self.device.signing_key, self.device.public_key, data
        )

    async def decrypt_for_self(self, ciphered: bytes) -> Tuple[DeviceID, bytes]:
        device_id, signed_data = decrypt_for(self.device.private_key, ciphered)
        author_device = await self.fetch_remote_device(device_id)
        if not author_device:
            raise MessageSignatureError(
                f"Message is said to be signed by `{device_id}`, "
                "but this device cannot be found on the backend."
            )
        return (device_id, verify_signature_from(author_device.verify_key, signed_data))

    def encrypt_with_secret_key(self, key: bytes, data: bytes) -> bytes:
        return encrypt_with_secret_key(self.device.device_id, self.device.signing_key, key, data)

    async def decrypt_with_secret_key(self, key: bytes, ciphered: bytes) -> dict:
        device_id, signed = decrypt_with_secret_key(key, ciphered)
        author_device = await self.fetch_remote_device(device_id)
        if not author_device:
            raise MessageSignatureError(
                f"Message is said to be signed by `{device_id}`, "
                "but this device cannot be found on the backend."
            )
        return verify_signature_from(author_device.verify_key, signed)
