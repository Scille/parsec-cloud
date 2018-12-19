from json import JSONDecodeError
from secrets import token_hex

from parsec.types import DeviceID
from parsec.crypto import (
    CryptoError,
    PublicKey,
    PrivateKey,
    VerifyKey,
    encrypt_raw_for,
    decrypt_raw_for,
)
from parsec.schema import ValidationError, UnknownCheckedSchema, fields
from parsec.core.fs.types import Access
from parsec.core.schemas import ManifestAccessSchema


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


user_claim_schema = UserClaimSchema(strict=True)


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
        raw = user_claim_schema.dumps(payload).data.encode("utf8")
        return encrypt_raw_for(creator_public_key, raw)

    except (CryptoError, ValidationError, JSONDecodeError, ValueError) as exc:
        raise InviteClaimError(str(exc)) from exc


def extract_user_encrypted_claim(creator_private_key: PrivateKey, encrypted_claim: bytes) -> dict:
    try:
        raw = decrypt_raw_for(creator_private_key, encrypted_claim)
        return user_claim_schema.loads(raw.decode("utf8")).data

    except (CryptoError, ValidationError, JSONDecodeError, ValueError) as exc:
        raise InviteClaimError(str(exc)) from exc


###  Device claim  ###


class DeviceClaimSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("device_claim", required=True)
    token = fields.String(required=True)
    device_id = fields.DeviceID(required=True)
    verify_key = fields.VerifyKey(required=True)
    answer_public_key = fields.PublicKey(required=True)


device_claim_schema = DeviceClaimSchema(strict=True)


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
        raw = device_claim_schema.dumps(payload).data.encode("utf8")
        return encrypt_raw_for(creator_public_key, raw)

    except (CryptoError, ValidationError, JSONDecodeError, ValueError) as exc:
        raise InviteClaimError(str(exc)) from exc


def extract_device_encrypted_claim(creator_private_key: PrivateKey, encrypted_claim: bytes) -> dict:
    try:
        raw = decrypt_raw_for(creator_private_key, encrypted_claim)
        return device_claim_schema.loads(raw.decode("utf8")).data

    except (CryptoError, ValidationError, JSONDecodeError, ValueError) as exc:
        raise InviteClaimError(str(exc)) from exc


###  Device claim answer  ###


class DeviceClaimAnswerSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("device_claim_answer", required=True)
    private_key = fields.PrivateKey(required=True)
    user_manifest_access = fields.Nested(ManifestAccessSchema, required=True)


device_claim_answer_schema = DeviceClaimAnswerSchema(strict=True)


def generate_device_encrypted_answer(
    creator_public_key: PublicKey, private_key: PrivateKey, user_manifest_access: Access
) -> bytes:
    payload = {
        "type": "device_claim_answer",
        "private_key": private_key,
        "user_manifest_access": user_manifest_access,
    }
    try:
        raw = device_claim_answer_schema.dumps(payload).data.encode("utf8")
        return encrypt_raw_for(creator_public_key, raw)

    except (CryptoError, ValidationError, JSONDecodeError, ValueError) as exc:
        raise InviteClaimError(str(exc)) from exc


def extract_device_encrypted_answer(
    creator_private_key: PrivateKey, encrypted_claim: bytes
) -> dict:
    try:
        raw = decrypt_raw_for(creator_private_key, encrypted_claim)
        return device_claim_answer_schema.loads(raw.decode("utf8")).data

    except (CryptoError, ValidationError, JSONDecodeError, ValueError) as exc:
        raise InviteClaimError(str(exc)) from exc


### Helpers ###

# TODO: use crypto.generate_token instead
def generate_invitation_token(token_size):
    return token_hex(token_size)


import pendulum
from parsec.types import UserID
from parsec.trustchain import certify_device, certify_user
from parsec.core.types import LocalDevice


async def bootstrap_domain(backend_addr: str, token: str) -> LocalDevice:
    root_signing_key = SigningKey.generate()
    root_id = DeviceID("root@root")
    certify_device
    device = generate_new_device(device, backend_addr, root_signing_key.verify_key)
    certify_user()
    async with backend_anonymous_cmds_factory(backend_addr) as cmds:
        cmds.domain_bootstrap()


async def invite_and_create_user(core, user_id: UserID, token: str) -> DeviceID:
    """
    Raises:
        InviteClaimError
        core.backend_connection.BackendConnectionError
        core.trustchain.TrustChainError
    """
    encrypted_claim = await core.backend_cmds.user_invite(user_id)

    claim = extract_user_encrypted_claim(core.device.private_key, encrypted_claim)
    if claim["token"] != token:
        raise InviteClaimInvalidToken(claim["token"])

    device_id = claim["device_id"]
    now = pendulum.now()
    certified_user = certify_user(
        core.device.device_id,
        core.device.signing_key,
        device_id.user_id,
        claim["public_key"],
        now=now,
    )
    certified_device = certify_device(
        core.device.device_id, core.device.signing_key, device_id, claim["verify_key"], now=now
    )

    await core.backend_cmds.user_create(certified_user, certified_device)
    return device_id


async def claim_and_save_user(backend_addr, root_verify_key, device_id, token: str) -> LocalDevice:
    device = generate_new_device(device_id, backend_addr, root_verify_key)

    async with backend_anonymous_cmds_factory(backend_addr) as cmds:
        invitation_creator = await cmds.user_get_invitation_creator(device_id.user_id)

        encrypted_claim = generate_user_encrypted_claim(
            invitation_creator.public_key, token, device_id, device.public_key, device.verify_key
        )
        await cmds.user_claim(device_id.user_id, encrypted_claim)

        if pkcs11:
            save_device_with_password(config_dir, device_id, token_id, key_id, pin)
        else:
            save_device_with_password(config_dir, device_id, password)

    async with backend_anonymous_cmds_factory(backend_addr) as cmds:
        invitation_creator = await cmds.user_get_invitation_creator(mallory.user_id)
        assert isinstance(invitation_creator, RemoteUser)

        encrypted_claim = generate_user_encrypted_claim(
            invitation_creator.public_key, mallory.device_id, mallory.public_key, mallory.verify_key
        )
        await cmds.user_claim(mallory.user_id, encrypted_claim)
