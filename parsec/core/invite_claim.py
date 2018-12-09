from json import JSONDecodeError

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


###  User claim  ###


class UserClaimSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("user_claim", required=True)
    # Note claiming user also imply creating a first device
    device_id = fields.DeviceID(required=True)
    public_key = fields.PublicKey(required=True)
    verify_key = fields.VerifyKey(required=True)


user_claim_schema = UserClaimSchema(strict=True)


def generate_user_encrypted_claim(
    creator_public_key: PublicKey, device_id: DeviceID, public_key: PublicKey, verify_key=VerifyKey
) -> bytes:
    payload = {
        "type": "user_claim",
        "device_id": device_id,
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
    device_id = fields.DeviceID(required=True)
    verify_key = fields.VerifyKey(required=True)
    answer_public_key = fields.PublicKey(required=True)


device_claim_schema = DeviceClaimSchema(strict=True)


def generate_device_encrypted_claim(
    creator_public_key: PublicKey,
    device_id: DeviceID,
    verify_key: VerifyKey,
    answer_public_key: PublicKey,
) -> bytes:
    payload = {
        "type": "device_claim",
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
