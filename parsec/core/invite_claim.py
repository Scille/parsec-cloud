from secrets import token_hex
import pendulum

from parsec.types import DeviceID, UserID, DeviceName
from parsec.crypto import (
    CryptoError,
    PublicKey,
    PrivateKey,
    SigningKey,
    VerifyKey,
    generate_secret_key,
    encrypt_raw_for,
    decrypt_raw_for,
)
from parsec.trustchain import certify_device, certify_user

from parsec.serde import Serializer, SerdeError, UnknownCheckedSchema, fields
from parsec.core.types import LocalDevice, Access
from parsec.core.types.access import ManifestAccessSchema
from parsec.core.backend_connection import BackendCmdsPool
from parsec.core.devices_manager import generate_new_device


class InviteClaimError(Exception):
    pass


class InviteClaimInvalidToken(InviteClaimError):
    pass


###  User claim  ###


class UserClaimSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("user_claim", required=True)
    token = fields.String(required=True)
    # Note claiming user also imply creating a first device
    device_id = fields.DeviceID(required=True)
    public_key = fields.PublicKey(required=True)
    verify_key = fields.VerifyKey(required=True)


user_claim_serializer = Serializer(UserClaimSchema)


def generate_user_encrypted_claim(
    creator_public_key: PublicKey,
    token: str,
    device_id: DeviceID,
    public_key: PublicKey,
    verify_key: VerifyKey,
) -> bytes:
    payload = {
        "type": "user_claim",
        "device_id": device_id,
        "token": token,
        "public_key": public_key,
        "verify_key": verify_key,
    }
    try:
        raw = user_claim_serializer.dumps(payload)
        return encrypt_raw_for(creator_public_key, raw)

    except (CryptoError, SerdeError) as exc:
        raise InviteClaimError(str(exc)) from exc


def extract_user_encrypted_claim(creator_private_key: PrivateKey, encrypted_claim: bytes) -> dict:
    try:
        raw = decrypt_raw_for(creator_private_key, encrypted_claim)
        return user_claim_serializer.loads(raw)

    except (CryptoError, SerdeError) as exc:
        raise InviteClaimError(str(exc)) from exc


###  Device claim  ###


class DeviceClaimSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("device_claim", required=True)
    token = fields.String(required=True)
    device_id = fields.DeviceID(required=True)
    verify_key = fields.VerifyKey(required=True)
    answer_public_key = fields.PublicKey(required=True)


device_claim_serializer = Serializer(DeviceClaimSchema)


def generate_device_encrypted_claim(
    creator_public_key: PublicKey,
    token: str,
    device_id: DeviceID,
    verify_key: VerifyKey,
    answer_public_key: PublicKey,
) -> bytes:
    payload = {
        "type": "device_claim",
        "token": token,
        "device_id": device_id,
        "verify_key": verify_key,
        "answer_public_key": answer_public_key,
    }
    try:
        raw = device_claim_serializer.dumps(payload)
        return encrypt_raw_for(creator_public_key, raw)

    except (CryptoError, SerdeError) as exc:
        raise InviteClaimError(str(exc)) from exc


def extract_device_encrypted_claim(creator_private_key: PrivateKey, encrypted_claim: bytes) -> dict:
    try:
        raw = decrypt_raw_for(creator_private_key, encrypted_claim)
        return device_claim_serializer.loads(raw)

    except (CryptoError, SerdeError) as exc:
        raise InviteClaimError(str(exc)) from exc


###  Device claim answer  ###


class DeviceClaimAnswerSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("device_claim_answer", required=True)
    private_key = fields.PrivateKey(required=True)
    user_manifest_access = fields.Nested(ManifestAccessSchema, required=True)


device_claim_answer_serializer = Serializer(DeviceClaimAnswerSchema)


def generate_device_encrypted_answer(
    creator_public_key: PublicKey, private_key: PrivateKey, user_manifest_access: Access
) -> bytes:
    payload = {
        "type": "device_claim_answer",
        "private_key": private_key,
        "user_manifest_access": user_manifest_access,
    }
    try:
        raw = device_claim_answer_serializer.dumps(payload)
        return encrypt_raw_for(creator_public_key, raw)

    except (CryptoError, SerdeError) as exc:
        raise InviteClaimError(str(exc)) from exc


def extract_device_encrypted_answer(
    creator_private_key: PrivateKey, encrypted_claim: bytes
) -> dict:
    try:
        raw = decrypt_raw_for(creator_private_key, encrypted_claim)
        return device_claim_answer_serializer.loads(raw)

    except (CryptoError, SerdeError) as exc:
        raise InviteClaimError(str(exc)) from exc


### Helpers ###


def generate_invitation_token():
    return token_hex(8)


async def claim_user(cmds: BackendCmdsPool, new_device_id: DeviceID, token: str) -> LocalDevice:
    """
    Raises:
        InviteClaimError
        core.backend_connection.BackendConnectionError
        core.trustchain.TrustChainError
    """
    device = generate_new_device(new_device_id, cmds.addr)

    invitation_creator = await cmds.user_get_invitation_creator(new_device_id.user_id)

    encrypted_claim = generate_user_encrypted_claim(
        invitation_creator.public_key, token, new_device_id, device.public_key, device.verify_key
    )
    await cmds.user_claim(new_device_id.user_id, encrypted_claim)

    return device


async def claim_device(cmds: BackendCmdsPool, new_device_id: DeviceID, token: str) -> LocalDevice:
    """
    Raises:
        InviteClaimError
        core.backend_connection.BackendConnectionError
        core.trustchain.TrustChainError
    """
    device_signing_key = SigningKey.generate()
    answer_private_key = PrivateKey.generate()

    invitation_creator = await cmds.device_get_invitation_creator(new_device_id)

    encrypted_claim = generate_device_encrypted_claim(
        creator_public_key=invitation_creator.public_key,
        token=token,
        device_id=new_device_id,
        verify_key=device_signing_key.verify_key,
        answer_public_key=answer_private_key.public_key,
    )
    encrypted_answer = await cmds.device_claim(new_device_id, encrypted_claim)

    answer = extract_device_encrypted_answer(answer_private_key, encrypted_answer)
    return LocalDevice(
        backend_addr=cmds.addr,
        device_id=new_device_id,
        signing_key=device_signing_key,
        private_key=answer["private_key"],
        user_manifest_access=answer["user_manifest_access"],
        local_symkey=generate_secret_key(),
    )


async def invite_and_create_device(
    device: LocalDevice, cmds: BackendCmdsPool, new_device_name: DeviceName, token: str
) -> None:
    """
    Raises:
        InviteClaimError
        core.backend_connection.BackendConnectionError
        core.trustchain.TrustChainError
    """
    encrypted_claim = await cmds.device_invite(new_device_name)

    claim = extract_device_encrypted_claim(device.private_key, encrypted_claim)
    if claim["token"] != token:
        raise InviteClaimInvalidToken(claim["token"])

    certified_device = certify_device(
        device.device_id, device.signing_key, claim["device_id"], claim["verify_key"]
    )
    encrypted_answer = generate_device_encrypted_answer(
        claim["answer_public_key"], device.private_key, device.user_manifest_access
    )

    await cmds.device_create(certified_device, encrypted_answer)


async def invite_and_create_user(
    device: LocalDevice, cmds: BackendCmdsPool, user_id: UserID, token: str, is_admin: bool
) -> DeviceID:
    """
    Raises:
        InviteClaimError
        core.backend_connection.BackendConnectionError
        core.trustchain.TrustChainError
    """
    encrypted_claim = await cmds.user_invite(user_id)

    claim = extract_user_encrypted_claim(device.private_key, encrypted_claim)
    if claim["token"] != token:
        raise InviteClaimInvalidToken(claim["token"])

    device_id = claim["device_id"]
    now = pendulum.now()
    certified_user = certify_user(
        device.device_id, device.signing_key, device_id.user_id, claim["public_key"], now=now
    )
    certified_device = certify_device(
        device.device_id, device.signing_key, device_id, claim["verify_key"], now=now
    )

    await cmds.user_create(certified_user, certified_device, is_admin)
    return device_id
