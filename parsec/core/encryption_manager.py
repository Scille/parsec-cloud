import attr
import logbook
import json
from nacl.public import SealedBox, PublicKey
from nacl.secret import SecretBox
from nacl.signing import VerifyKey
from nacl.exceptions import CryptoError

from parsec.schema import UnknownCheckedSchema, fields
from parsec.core.base import BaseAsyncComponent
from parsec.core.devices_manager import Device, is_valid_user_id, is_valid_device_name
from parsec.core.backend_connection import BackendNotAvailable


logger = logbook.Logger("parsec.core.encryption_manager")


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


class BackendUserGetRepDevicesSchema(UnknownCheckedSchema):
    created_on = fields.DateTime(required=True)
    revocated_on = fields.DateTime(allow_none=True)
    verify_key = fields.Base64Bytes(required=True)


class BackendUserGetRepSchema(UnknownCheckedSchema):
    status = fields.CheckedConstant("ok", required=True)
    user_id = fields.String(required=True)
    created_on = fields.DateTime(required=True)
    created_by = fields.String(required=True)
    broadcast_key = fields.Base64Bytes(required=True)
    devices = fields.Map(
        fields.String(), fields.Nested(BackendUserGetRepDevicesSchema), required=True
    )


backend_user_get_rep_schema = BackendUserGetRepSchema()


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
    user_id = attr.ib()
    device_name = attr.ib()
    device_verifykey = attr.ib()

    def __init__(self, user_id: str, device_name: str, device_verifykey: bytes):
        assert is_valid_user_id(user_id)
        assert is_valid_device_name(device_name)
        self.user_id = user_id
        self.device_name = device_name
        self.device_verifykey = VerifyKey(device_verifykey)

    @property
    def id(self):
        return "%s@%s" % (self.user.user_id, self.name)


def encrypt_for_local(key: bytes, msg: dict) -> bytes:
    """
    Raises:
        MessageFormatError: if msg is not JSON-encodable.
        MessageEncryptionError: if key is invalid.
    """
    try:
        encoded_msg = json.dumps(msg).encode()
    except TypeError as exc:
        raise MessageFormatError("Cannot encode message as JSON") from exc

    try:
        box = SecretBox(key)
        return box.encrypt(encoded_msg)
    except CryptoError as exc:
        raise MessageEncryptionError() from exc


def decrypt_for_local(key: bytes, ciphered_msg: bytes) -> dict:
    """
    Raises:
        MessageFormatError: if deciphered message is not JSON-decodable.
        MessageEncryptionError: if key is invalid.
    """
    try:
        box = SecretBox(key)
        encoded_msg = box.decrypt(ciphered_msg)
    except CryptoError as exc:
        raise MessageEncryptionError() from exc

    try:
        return json.loads(encoded_msg)
    except json.JSONDecodeError as exc:
        raise MessageFormatError("Message is not valid json data") from exc


def sign_and_add_meta(device: Device, raw_msg: bytes) -> bytes:
    """
    Raises:
        MessageSignatureError: if the signature operation fails.
    """
    try:
        signed = device.device_signkey.sign(raw_msg)
    except CryptoError as exc:
        raise MessageSignatureError() from exc

    return device.id.encode("utf-8") + b"@" + signed


def extract_meta_from_signature(signed_with_meta: bytes) -> tuple:
    """
    Raises:
        MessageFormatError: if the metadata cannot be extracted
    """
    try:
        user_id, device_name, signed = signed_with_meta.split(b"@", 2)
        return user_id.decode("utf-8"), device_name.decode("utf-8"), signed
    except (ValueError, UnicodeDecodeError) as exc:
        raise MessageFormatError(
            "Message doesn't contain author metadata along with signed message"
        ) from exc


def encrypt_for_self(author: Device, msg: dict) -> bytes:
    return encrypt_for(author, author, msg)


def encrypt_for(author: Device, recipient: RemoteUser, msg: dict) -> bytes:
    """
    Sign and encrypt a message.

    Raises:
        MessageFormatError: if the message is not JSON-serializable.
        MessageEncryptionError: if encryption fails.
        MessageSignatureError: if signature fails.
    """
    try:
        encoded_msg = json.dumps(msg).encode()
    except TypeError as exc:
        raise MessageFormatError("Cannot encode message as JSON") from exc

    try:
        signed_msg_with_meta = sign_and_add_meta(author, encoded_msg)
    except CryptoError as exc:
        raise MessageSignatureError() from exc

    try:
        box = SealedBox(recipient.user_pubkey)
        return box.encrypt(signed_msg_with_meta)
    except CryptoError as exc:
        raise MessageEncryptionError() from exc


def decrypt_for(recipient: Device, ciphered_msg: bytes) -> tuple:
    """
    Decrypt a message and return it signed data and author metadata.

    Raises:
        MessageFormatError: if the author metadata cannot be extracted.
        MessageEncryptionError: if key is invalid.

    Returns: a tuple of (<user_id>, <device_name>, <signed_message>)

    Note: Once decrypted, the masseg should be passed to
    :func:`verify_signature_from` to be finally converted to plain text.
    """
    try:
        box = SealedBox(recipient.user_privkey)
        signed_msg_with_meta = box.decrypt(ciphered_msg)
    except CryptoError as exc:
        raise MessageEncryptionError() from exc

    return extract_meta_from_signature(signed_msg_with_meta)


def verify_signature_from(author: RemoteDevice, signed_text: bytes) -> dict:
    """
    Verify signature and decode message.

    Returns: The plain text message as a dict.

    Raises:
        MessageFormatError: if the message is not valid JSON.
        MessageSignatureError: if signature was forged or otherwise corrupt.
    """
    try:
        encoded_msg = author.device_verifykey.verify(signed_text)
    except CryptoError as exc:
        raise MessageSignatureError() from exc

    try:
        return json.loads(encoded_msg)

    except json.JSONDecodeError as exc:
        raise MessageFormatError("Message is not valid json data") from exc


def encrypt_with_secret_key(author: Device, key: bytes, msg: dict) -> bytes:
    """
    Sign and encrypt a message with a symetric key.

    Raises:
        MessageFormatError: if the message is not JSON-serializable.
        MessageSignatureError: if the signature operation fails
        MessageEncryptionError: if encryption operation fails.
    """
    try:
        encoded_msg = json.dumps(msg).encode()
    except TypeError as exc:
        raise MessageFormatError("Cannot encode message as JSON") from exc

    signed_msg_with_meta = sign_and_add_meta(author, encoded_msg)

    try:
        box = SecretBox(key)
        return box.encrypt(signed_msg_with_meta)
    except CryptoError as exc:
        raise MessageEncryptionError() from exc


def decrypt_with_secret_key(key: bytes, ciphered_msg: dict) -> tuple:
    """
    Decrypt a message with a symetric key.

    Raises:
        MessageFormatError: if the author metadata cannot be extracted.
        MessageEncryptionError: if key is invalid.

    Returns: a tuple of (<user_id>, <device_name>, <signed_message>)

    Note: Once decrypted, the masseg should be passed to
    :func:`verify_signature_from` to be finally converted to plain text.
    """
    try:
        box = SecretBox(key)
        signed_msg_with_meta = box.decrypt(ciphered_msg)
    except CryptoError as exc:
        raise MessageEncryptionError() from exc

    return extract_meta_from_signature(signed_msg_with_meta)


class EncryptionManager(BaseAsyncComponent):
    def __init__(self, device, backend_connection, local_storage):
        super().__init__()
        self.device = device
        self._backend_connection = backend_connection
        self._local_storage = local_storage

    async def _init(self, nursery):
        pass

    async def _teardown(self):
        pass

    async def _populate_remote_user_cache_in_local_storage(self, user_id: str):
        raw_rep = await self._backend_connection.send({"cmd": "user_get", "user_id": user_id})
        rep, errors = backend_user_get_rep_schema.load(raw_rep)
        if errors:
            if raw_rep.get("status") == "not_found":
                # User doesn't exit, nothing to populate then
                return
            else:
                raise BackendGetUserError(
                    "Cannot retreive user `%s`: %r (errors: %r)" % (user_id, raw_rep, errors)
                )

        pubkey = rep["broadcast_key"]
        self._local_storage.flush_user_pubkey(user_id, pubkey)
        for device_name, device in rep["devices"].items():
            self._local_storage.flush_device_verifykey(user_id, device_name, device["verify_key"])

    async def fetch_remote_device(self, user_id: str, device_name: str) -> RemoteDevice:
        """
        Retrieve a device from the backend.

        Returns: The device or None if it couldn't be found.

        Raises:
            BackendNotAvailable: if the backend is offline.
            BackendGetUserError: if the reponse returned by the backend is invalid.
        """
        if user_id == self.device.user_id and device_name == self.device.device_name:
            return RemoteDevice(
                self.device.user_id, self.device.device_name, self.device.device_verifykey.encode()
            )

        # First try to retreive from the local cache
        verifykey = self._local_storage.fetch_device_verifykey(user_id, device_name)
        if not verifykey:
            # Cache miss ! Fetch data from the backend and retry
            await self._populate_remote_user_cache_in_local_storage(user_id)
            verifykey = self._local_storage.fetch_device_verifykey(user_id, device_name)
            if not verifykey:
                # Still nothing found, the device doesn't exist
                return None

        return RemoteDevice(user_id, device_name, verifykey)

    async def fetch_remote_user(self, user_id: str) -> RemoteUser:
        """
        Retrieve a user from the backend.

        Returns: The user or None if it couldn't be found.

        Raises:
            BackendNotAvailable: if the backend is offline.
            BackendGetUserError: if the reponse returned by the backend is invalid.
        """
        if user_id == self.device.user_id:
            return RemoteUser(self.device.user_id, self.device.user_pubkey.encode())

        # First try to retreive from the local cache
        pubkey = self._local_storage.fetch_user_pubkey(user_id)
        if not pubkey:
            # Cache miss ! Fetch data from the backend and retry
            await self._populate_remote_user_cache_in_local_storage(user_id)
            pubkey = self._local_storage.fetch_user_pubkey(user_id)
            if not pubkey:
                # Still nothing found, the user doesn't exist
                return None

        return RemoteUser(user_id, pubkey)

    async def encrypt(self, recipient: str, msg: dict) -> bytes:
        user = await self.fetch_remote_user(recipient)
        if not user:
            raise MessageEncryptionError("Unknown recipient `%s`" % recipient)
        return encrypt_for(self.device, user, msg)

    async def encrypt_for_self(self, msg: dict) -> bytes:
        return encrypt_for_self(self.device, msg)

    async def decrypt(self, ciphered_msg: bytes) -> dict:
        user_id, device_name, signed_msg = decrypt_for(self.device, ciphered_msg)
        author_device = await self.fetch_remote_device(user_id, device_name)
        if not author_device:
            raise MessageSignatureError(
                "Message is signed by `%s@%s`, but this device cannot be found on the backend."
                % (user_id, device_name)
            )
        return verify_signature_from(author_device, signed_msg)

    async def encrypt_with_secret_key(self, key: bytes, msg: dict) -> bytes:
        return encrypt_with_secret_key(self.device, key, msg)

    async def decrypt_with_secret_key(self, key: bytes, msg: dict) -> dict:
        user_id, device_name, signed_msg = decrypt_with_secret_key(key, msg)
        author_device = await self.fetch_remote_device(user_id, device_name)
        if not author_device:
            raise MessageSignatureError(
                "Message is signed by `%s@%s`, but this device cannot be found on the backend."
                % (user_id, device_name)
            )
        return verify_signature_from(author_device, signed_msg)
