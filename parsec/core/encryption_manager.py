import attr
import logbook
import json
from nacl.public import SealedBox, PublicKey
from nacl.secret import SecretBox
from nacl.signing import VerifyKey

from parsec.schema import UnknownCheckedSchema, fields
from parsec.core.base import BaseAsyncComponent
from parsec.core.devices_manager import Device, is_valid_user_id, is_valid_device_name
from parsec.core.backend_connection import BackendNotAvailable


logger = logbook.Logger("parsec.core.encryption_manager")


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
    devices = fields.Map(fields.String(), fields.Nested(BackendUserGetRepDevicesSchema), required=True)


backend_user_get_rep_schema = BackendUserGetRepSchema()


class BackendGetUserError(Exception):
    pass


class BackendUserNotFound(Exception):
    pass


class InvalidMessageError(Exception):
    pass


@attr.s(init=False, slots=True)
class RemoteUser:
    user_id = attr.ib()
    user_pubkey = attr.ib()
    devices = attr.ib(default=attr.Factory(dict))

    def __init__(self, id: str, user_pubkey: bytes, devices: dict = {}):
        assert is_valid_user_id(id)
        self.user_id = id
        self.user_pubkey = PublicKey(user_pubkey)
        self.devices = {dname: RemoteDevice(self, dname, dkey) for dname, dkey in devices.items()}


@attr.s(init=False, slots=True)
class RemoteDevice:
    user = attr.ib()
    device_name = attr.ib()
    device_verifykey = attr.ib()

    def __init__(self, user: RemoteUser, device_name: str, device_verifykey: bytes):
        assert is_valid_device_name(device_name)
        self.user = user
        self.device_name = device_name
        self.device_verifykey = VerifyKey(device_verifykey)

    @property
    def id(self):
        return "%s@%s" % (self.user.user_id, self.name)

    @property
    def user_id(self):
        return self.user.user_id


def sign_and_add_meta(device: Device, raw_msg: bytes) -> bytes:
    signed = device.device_signkey.sign(raw_msg)
    return device.id.encode("utf-8") + b"@" + signed


def extract_meta_from_signature(signed_with_meta: bytes) -> tuple:
    try:
        user_id, device_name, signed = signed_with_meta.split(b"@", 2)
        return user_id.decode("utf-8"), device_name.decode("utf-8"), signed
    except (ValueError, UnicodeDecodeError) as exc:
        raise InvalidMessageError(
            "Message doesn't contain author metadata along with signed message"
        ) from exc


def encrypt_for_self(author: Device, msg: dict) -> bytes:
    return encrypt_for(author, author, msg)


def encrypt_for(author: Device, recipient: RemoteUser, msg: dict) -> bytes:
    """
    Sign and encrypt a message.

    Raises:
        InvalidMessageError: if the message is not JSON-serializable.
        nacl.exceptions.CryptoError: if encryption or signature fails.
    """
    try:
        encoded_msg = json.dumps(msg).encode()
    except TypeError as exc:
        raise InvalidMessageError("Cannot encode message as JSON") from exc
    signed_msg_with_meta = sign_and_add_meta(author, encoded_msg)
    box = SealedBox(recipient.user_pubkey)
    return box.encrypt(signed_msg_with_meta)


def decrypt_for(recipient: Device, ciphered_msg: bytes) -> tuple:
    """
    Decrypt a message.

    Raises:
        InvalidMessageError: if the message doesn't contain author metadata.
        nacl.exceptions.BadSignatureError: if signature was forged or otherwise corrupt.

    Returns: a tuple of (<user_id>, <device_name>, <signed_message>)

    Note: Once decrypted, the masseg should be passed to
    :func:`verify_signature_from` to be finally converted to plain text.
    """
    box = SealedBox(recipient.user_privkey)
    signed_msg_with_meta = box.decrypt(ciphered_msg)
    return extract_meta_from_signature(signed_msg_with_meta)


def verify_signature_from(author: RemoteDevice, signed_text: bytes) -> dict:
    """
    Verify signature and decode message.

    Returns: The plain text message as a dict.

    Raises:
        InvalidMessageError: if the message is not valid JSON.
        nacl.exceptions.BadSignatureError: if signature was forged or otherwise corrupt.
    """
    encoded_msg = author.device_verifykey.verify(signed_text)
    try:
        return json.loads(encoded_msg)
    except json.JSONDecodeError as exc:
        raise InvalidMessageError("Message is not valid json data") from exc


def encrypt_with_secret_key(author: Device, key: bytes, msg: dict) -> bytes:
    """
    Sign and encrypt a message with a symetric key.

    Raises:
        InvalidMessageError: if the message is not JSON-serializable.
        nacl.exceptions.CryptoError: if encryption or signature fails.
    """
    try:
        encoded_msg = json.dumps(msg).encode()
    except TypeError as exc:
        raise InvalidMessageError("Cannot encode message as JSON") from exc
    signed_msg_with_meta = sign_and_add_meta(author, encoded_msg)
    box = SecretBox(key)
    return box.encrypt(signed_msg_with_meta)


def decrypt_with_secret_key(key: bytes, ciphered_msg: dict) -> tuple:
    """
    Decrypt a message with a symetric key.

    Raises:
        nacl.exceptions.TypeError: if key is not made of bytes.
        nacl.exceptions.ValueError: if the key is invalid.

    Returns: a tuple of (<user_id>, <device_name>, <signed_message>)
    """
    box = SecretBox(key)
    signed_msg_with_meta = box.decrypt(ciphered_msg)
    return extract_meta_from_signature(signed_msg_with_meta)


class EncryptionManager(BaseAsyncComponent):

    def __init__(self, device, backend_connection):
        super().__init__()
        self.device = device
        self._backend_connection = backend_connection

    async def _init(self, nursery):
        pass

    async def _teardown(self):
        pass

    async def fetch_remote_user(self, user_id: str) -> RemoteUser:
        """
        Retrieve a user from the backend.

        Raises:
            BackendNotAvailable: if the backend is offline.
            BackendGetUserError: if the reponse returned by the backend is invalid.
            BackendUserNotFound: if the backend couldn't find the user.
        """
        try:
            raw_rep = await self._backend_connection.send({"cmd": "user_get", "user_id": user_id})
        except BackendNotAvailable:
            # TODO: in case backend is not available we should store remote
            # users informations in the local storage.
            # In the meantime, this is a hack to keep offline tests working
            if user_id == self.device.user_id:
                return RemoteUser(
                    self.device.user_id, self.device.user_pubkey.encode(),
                    {self.device.device_name: self.device.device_verifykey.encode()})
        rep, errors = backend_user_get_rep_schema.load(raw_rep)
        if errors:
            if raw_rep.get("status") == "not_found":
                raise BackendUserNotFound("Cannot retreive user %s" % user_id)
            else:
                raise BackendGetUserError(
                    "Cannot retreive user %s: %r (errors: %r)" % (user_id, rep, errors)
                )
        user = RemoteUser(
            user_id,
            rep["broadcast_key"],
            {dname: d["verify_key"] for dname, d in rep["devices"].items()},
        )
        return user

    async def encrypt(self, recipient: str, msg: dict) -> bytes:
        user = await self.fetch_remote_user(recipient)
        return encrypt_for(self.device, user, msg)

    async def encrypt_for_self(self, msg: dict) -> bytes:
        return encrypt_for_self(self.device, msg)

    async def decrypt(self, ciphered_msg: bytes) -> dict:
        user_id, device_name, signed_msg = decrypt_for(self.device, ciphered_msg)
        user = await self.fetch_remote_user(user_id)
        try:
            author_device = user.devices[device_name]
        except KeyError:
            raise BackendUserNotFound(
                "Message is signed by %s@%s, but this user doesn't have device with this name."
                % (user_id, device_name)
            )
        return verify_signature_from(author_device, signed_msg)

    async def encrypt_with_secret_key(self, key: bytes, msg: dict) -> bytes:
        return encrypt_with_secret_key(self.device, key, msg)

    async def decrypt_with_secret_key(self, key: bytes, msg: dict) -> dict:
        user_id, device_name, signed_msg = decrypt_with_secret_key(key, msg)
        user = await self.fetch_remote_user(user_id)
        try:
            author_device = user.devices[device_name]
        except KeyError:
            raise BackendUserNotFound(
                "Message is signed by %s@%s, but this user doesn't have device with this name."
                % (user_id, device_name)
            )
        return verify_signature_from(author_device, signed_msg)
