# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from secrets import token_hex
import pendulum

from parsec.types import DeviceID, UserID, DeviceName, BackendOrganizationAddr
from parsec.crypto import (
    CryptoError,
    PublicKey,
    PrivateKey,
    SigningKey,
    VerifyKey,
    generate_secret_key,
    encrypt_raw_for,
    decrypt_raw_for,
    build_device_certificate,
    build_user_certificate,
)

from parsec.serde import Serializer, UnknownCheckedSchema, fields
from parsec.core.types import LocalDevice, Access
from parsec.core.types.access import ManifestAccessSchema
from parsec.core.backend_connection import (
    BackendConnectionError,
    BackendNotAvailable,
    backend_cmds_pool_factory,
    backend_anonymous_cmds_factory,
)
from parsec.core.local_device import generate_new_device
from parsec.core.remote_devices_manager import (
    RemoteDevicesManagerError,
    RemoteDevicesManagerBackendOfflineError,
    get_user_invitation_creator,
    get_device_invitation_creator,
)


# TODO: wrap exceptions from lower layers
# TODO: handles backend_anonymous_cmds_factory for caller (just pass backend addr)


class InviteClaimError(Exception):
    pass


class InviteClaimValidationError(InviteClaimError):
    pass


class InviteClaimPackingError(InviteClaimError):
    pass


class InviteClaimCryptoError(InviteClaimError):
    pass


class InviteClaimBackendOfflineError(InviteClaimError):
    pass


class InviteClaimInvalidTokenError(InviteClaimError):
    pass


###  User claim  ###


class UserClaimSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("user_claim", required=True)
    token = fields.String(required=True)
    # Note claiming user also imply creating a first device
    device_id = fields.DeviceID(required=True)
    public_key = fields.PublicKey(required=True)
    verify_key = fields.VerifyKey(required=True)


user_claim_serializer = Serializer(
    UserClaimSchema, validation_exc=InviteClaimValidationError, packing_exc=InviteClaimPackingError
)


def generate_user_encrypted_claim(
    creator_public_key: PublicKey,
    token: str,
    device_id: DeviceID,
    public_key: PublicKey,
    verify_key: VerifyKey,
) -> bytes:
    """
    Raises:
        InviteClaimValidationError
        InviteClaimPackingError
        InviteClaimCryptoError
    """
    payload = {
        "type": "user_claim",
        "device_id": device_id,
        "token": token,
        "public_key": public_key,
        "verify_key": verify_key,
    }
    raw = user_claim_serializer.dumps(payload)
    try:
        return encrypt_raw_for(creator_public_key, raw)

    except CryptoError as exc:
        raise InviteClaimCryptoError(str(exc)) from exc


def extract_user_encrypted_claim(creator_private_key: PrivateKey, encrypted_claim: bytes) -> dict:
    """
    Raises:
        InviteClaimValidationError
        InviteClaimPackingError
        InviteClaimCryptoError
    """
    try:
        raw = decrypt_raw_for(creator_private_key, encrypted_claim)

    except CryptoError as exc:
        raise InviteClaimCryptoError(str(exc)) from exc

    return user_claim_serializer.loads(raw)


###  Device claim  ###


class DeviceClaimSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("device_claim", required=True)
    token = fields.String(required=True)
    device_id = fields.DeviceID(required=True)
    verify_key = fields.VerifyKey(required=True)
    answer_public_key = fields.PublicKey(required=True)


device_claim_serializer = Serializer(
    DeviceClaimSchema,
    validation_exc=InviteClaimValidationError,
    packing_exc=InviteClaimPackingError,
)


def generate_device_encrypted_claim(
    creator_public_key: PublicKey,
    token: str,
    device_id: DeviceID,
    verify_key: VerifyKey,
    answer_public_key: PublicKey,
) -> bytes:
    """
    Raises:
        InviteClaimValidationError
        InviteClaimPackingError
        InviteClaimCryptoError
    """
    payload = {
        "type": "device_claim",
        "token": token,
        "device_id": device_id,
        "verify_key": verify_key,
        "answer_public_key": answer_public_key,
    }
    raw = device_claim_serializer.dumps(payload)
    try:
        return encrypt_raw_for(creator_public_key, raw)

    except CryptoError as exc:
        raise InviteClaimCryptoError(str(exc)) from exc


def extract_device_encrypted_claim(creator_private_key: PrivateKey, encrypted_claim: bytes) -> dict:
    """
    Raises:
        InviteClaimValidationError
        InviteClaimPackingError
        InviteClaimCryptoError
    """
    try:
        raw = decrypt_raw_for(creator_private_key, encrypted_claim)

    except CryptoError as exc:
        raise InviteClaimCryptoError(str(exc)) from exc

    return device_claim_serializer.loads(raw)


###  Device claim answer  ###


class DeviceClaimAnswerSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("device_claim_answer", required=True)
    private_key = fields.PrivateKey(required=True)
    user_manifest_access = fields.Nested(ManifestAccessSchema, required=True)


device_claim_answer_serializer = Serializer(DeviceClaimAnswerSchema)


def generate_device_encrypted_answer(
    creator_public_key: PublicKey, private_key: PrivateKey, user_manifest_access: Access
) -> bytes:
    """
    Raises:
        InviteClaimValidationError
        InviteClaimPackingError
        InviteClaimCryptoError
    """
    payload = {
        "type": "device_claim_answer",
        "private_key": private_key,
        "user_manifest_access": user_manifest_access,
    }
    raw = device_claim_answer_serializer.dumps(payload)
    try:
        return encrypt_raw_for(creator_public_key, raw)

    except CryptoError as exc:
        raise InviteClaimCryptoError(str(exc)) from exc


def extract_device_encrypted_answer(
    creator_private_key: PrivateKey, encrypted_claim: bytes
) -> dict:
    """
    Raises:
        InviteClaimValidationError
        InviteClaimPackingError
        InviteClaimCryptoError
    """
    try:
        raw = decrypt_raw_for(creator_private_key, encrypted_claim)

    except CryptoError as exc:
        raise InviteClaimCryptoError(str(exc)) from exc

    return device_claim_answer_serializer.loads(raw)


### Helpers ###


def generate_invitation_token():
    return token_hex(8)


async def claim_user(
    backend_addr: BackendOrganizationAddr, new_device_id: DeviceID, token: str
) -> LocalDevice:
    """
    Raises:
        InviteClaimError
        InviteClaimBackendOfflineError
        InviteClaimValidationError
        InviteClaimPackingError
        InviteClaimCryptoError
    """
    new_device = generate_new_device(new_device_id, backend_addr)

    try:
        async with backend_anonymous_cmds_factory(backend_addr) as cmds:
            # 1) Retrieve invitation creator
            try:
                invitation_creator = await get_user_invitation_creator(
                    cmds, new_device.root_verify_key, new_device.user_id
                )

            except RemoteDevicesManagerBackendOfflineError as exc:
                raise InviteClaimBackendOfflineError(str(exc)) from exc

            except RemoteDevicesManagerError as exc:
                raise InviteClaimError(f"Cannot retrieve invitation creator: {exc}") from exc

            # 2) Generate claim info for invitation creator
            encrypted_claim = generate_user_encrypted_claim(
                invitation_creator.public_key,
                token,
                new_device_id,
                new_device.public_key,
                new_device.verify_key,
            )

            # 3) Send claim
            try:
                await cmds.user_claim(new_device_id.user_id, encrypted_claim)

            except BackendNotAvailable as exc:
                raise InviteClaimBackendOfflineError(str(exc)) from exc

            except BackendConnectionError as exc:
                raise InviteClaimError(f"Cannot claim user: {exc}") from exc

    except BackendNotAvailable as exc:
        raise InviteClaimBackendOfflineError(str(exc)) from exc

    return new_device


async def claim_device(
    backend_addr: BackendOrganizationAddr, new_device_id: DeviceID, token: str
) -> LocalDevice:
    """
    Raises:
        InviteClaimError
        InviteClaimBackendOfflineError
        InviteClaimValidationError
        InviteClaimPackingError
        InviteClaimCryptoError
    """
    device_signing_key = SigningKey.generate()
    answer_private_key = PrivateKey.generate()

    try:
        async with backend_anonymous_cmds_factory(backend_addr) as cmds:
            # 1) Retrieve invitation creator
            try:
                invitation_creator = await get_device_invitation_creator(
                    cmds, backend_addr.root_verify_key, new_device_id
                )

            except RemoteDevicesManagerBackendOfflineError as exc:
                raise InviteClaimBackendOfflineError(str(exc)) from exc

            except RemoteDevicesManagerError as exc:
                raise InviteClaimError(f"Cannot retrieve invitation creator: {exc}") from exc

            # 2) Generate claim info for invitation creator
            encrypted_claim = generate_device_encrypted_claim(
                creator_public_key=invitation_creator.public_key,
                token=token,
                device_id=new_device_id,
                verify_key=device_signing_key.verify_key,
                answer_public_key=answer_private_key.public_key,
            )

            # 3) Send claim
            try:
                encrypted_answer = await cmds.device_claim(new_device_id, encrypted_claim)

            except BackendNotAvailable as exc:
                raise InviteClaimBackendOfflineError(str(exc)) from exc

            except BackendConnectionError as exc:
                raise InviteClaimError(f"Cannot claim device: {exc}") from exc

            answer = extract_device_encrypted_answer(answer_private_key, encrypted_answer)

    except BackendNotAvailable as exc:
        raise InviteClaimBackendOfflineError(str(exc)) from exc

    return LocalDevice(
        organization_addr=backend_addr,
        device_id=new_device_id,
        signing_key=device_signing_key,
        private_key=answer["private_key"],
        user_manifest_access=answer["user_manifest_access"],
        local_symkey=generate_secret_key(),
    )


async def invite_and_create_device(
    device: LocalDevice, new_device_name: DeviceName, token: str
) -> None:
    """
    Raises:
        InviteClaimError
        InviteClaimBackendOfflineError
        InviteClaimValidationError
        InviteClaimPackingError
        InviteClaimCryptoError
        InviteClaimInvalidTokenError
    """
    async with backend_cmds_pool_factory(
        device.organization_addr, device.device_id, device.signing_key, max=1
    ) as cmds:
        try:
            encrypted_claim = await cmds.device_invite(new_device_name)

        except BackendNotAvailable as exc:
            raise InviteClaimBackendOfflineError(str(exc)) from exc

        except BackendConnectionError as exc:
            raise InviteClaimError(f"Cannot invite device: {exc}") from exc

        claim = extract_device_encrypted_claim(device.private_key, encrypted_claim)
        if claim["token"] != token:
            raise InviteClaimInvalidTokenError(
                f"Invalid claim token provided by peer: `{claim['token']}`"
                f" (was expecting `{token}`)"
            )

        try:
            now = pendulum.now()
            device_certificate = build_device_certificate(
                device.device_id, device.signing_key, claim["device_id"], claim["verify_key"], now
            )

        except CryptoError as exc:
            raise InviteClaimError(f"Cannot generate device certificate: {exc}") from exc

        encrypted_answer = generate_device_encrypted_answer(
            claim["answer_public_key"], device.private_key, device.user_manifest_access
        )

        try:
            await cmds.device_create(device_certificate, encrypted_answer)

        except BackendNotAvailable as exc:
            raise InviteClaimBackendOfflineError(str(exc)) from exc

        except BackendConnectionError as exc:
            raise InviteClaimError(f"Cannot create device: {exc}") from exc


async def invite_and_create_user(
    device: LocalDevice, user_id: UserID, token: str, is_admin: bool
) -> DeviceID:
    """
    Raises:
        InviteClaimError
        InviteClaimBackendOfflineError
        InviteClaimValidationError
        InviteClaimPackingError
        InviteClaimCryptoError
        InviteClaimInvalidTokenError
    """
    async with backend_cmds_pool_factory(
        device.organization_addr, device.device_id, device.signing_key, max=1
    ) as cmds:
        try:
            encrypted_claim = await cmds.user_invite(user_id)

        except BackendNotAvailable as exc:
            raise InviteClaimBackendOfflineError(str(exc)) from exc

        except BackendConnectionError as exc:
            raise InviteClaimError(f"Cannot invite user: {exc}") from exc

        claim = extract_user_encrypted_claim(device.private_key, encrypted_claim)
        if claim["token"] != token:
            raise InviteClaimInvalidTokenError(
                f"Invalid claim token provided by peer: `{claim['token']}`"
                f" (was expecting `{token}`)"
            )

        device_id = claim["device_id"]
        now = pendulum.now()
        try:
            user_certificate = build_user_certificate(
                device.device_id, device.signing_key, device_id.user_id, claim["public_key"], now
            )
            device_certificate = build_device_certificate(
                device.device_id, device.signing_key, device_id, claim["verify_key"], now
            )

        except CryptoError as exc:
            raise InviteClaimError(
                f"Cannot generate user&first device certificates: {exc}"
            ) from exc

        try:
            await cmds.user_create(user_certificate, device_certificate, is_admin)

        except BackendNotAvailable as exc:
            raise InviteClaimBackendOfflineError(str(exc)) from exc

        except BackendConnectionError as exc:
            raise InviteClaimError(f"Cannot create user: {exc}") from exc

    return device_id
