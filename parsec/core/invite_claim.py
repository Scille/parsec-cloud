# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from secrets import token_hex
import pendulum

from parsec.crypto import SecretKey, PrivateKey, SigningKey
from parsec.api.data import (
    DataError,
    UserCertificateContent,
    DeviceCertificateContent,
    UserClaimContent,
    DeviceClaimContent,
    DeviceClaimAnswerContent,
)
from parsec.api.protocol import UserID, DeviceName, DeviceID
from parsec.core.types import LocalDevice, BackendOrganizationAddr
from parsec.core.backend_connection import (
    BackendCmdsTimeout,
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


class InviteClaimTimeoutError(InviteClaimError):
    pass


class InviteClaimInvalidTokenError(InviteClaimError):
    pass


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
                invitation_creator_device, invitation_creator_user = await get_user_invitation_creator(
                    cmds, new_device.root_verify_key, new_device.user_id
                )

            except RemoteDevicesManagerBackendOfflineError as exc:
                raise InviteClaimBackendOfflineError(str(exc)) from exc

            except RemoteDevicesManagerError as exc:
                raise InviteClaimError(f"Cannot retrieve invitation creator: {exc}") from exc

            # 2) Generate claim info for invitation creator
            try:
                encrypted_claim = UserClaimContent(
                    device_id=new_device_id,
                    token=token,
                    public_key=new_device.public_key,
                    verify_key=new_device.verify_key,
                ).dump_and_encrypt_for(recipient_pubkey=invitation_creator_user.public_key)

            except DataError as exc:
                raise InviteClaimError(f"Cannot generate user claim message: {exc}") from exc

            # 3) Send claim
            try:
                user_certificate, device_certificate = await cmds.user_claim(
                    new_device_id.user_id, encrypted_claim
                )

            except BackendNotAvailable as exc:
                raise InviteClaimBackendOfflineError(str(exc)) from exc

            except BackendConnectionError as exc:
                raise InviteClaimError(f"Cannot claim user: {exc}") from exc

            # 4) Verify user&device certificates and check admin status
            try:
                user = UserCertificateContent.verify_and_load(
                    user_certificate,
                    author_verify_key=invitation_creator_device.verify_key,
                    expected_author=invitation_creator_device.device_id,
                    expected_user=new_device_id.user_id,
                )

                DeviceCertificateContent.verify_and_load(
                    device_certificate,
                    author_verify_key=invitation_creator_device.verify_key,
                    expected_author=invitation_creator_device.device_id,
                    expected_device=new_device_id,
                )

                new_device = new_device.evolve(is_admin=user.is_admin)

            except DataError as exc:
                raise InviteClaimCryptoError(str(exc)) from exc

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
                invitation_creator_device, invitation_creator_user = await get_device_invitation_creator(
                    cmds, backend_addr.root_verify_key, new_device_id
                )

            except RemoteDevicesManagerBackendOfflineError as exc:
                raise InviteClaimBackendOfflineError(str(exc)) from exc

            except RemoteDevicesManagerError as exc:
                raise InviteClaimError(f"Cannot retrieve invitation creator: {exc}") from exc

            # 2) Generate claim info for invitation creator
            try:
                encrypted_claim = DeviceClaimContent(
                    token=token,
                    device_id=new_device_id,
                    verify_key=device_signing_key.verify_key,
                    answer_public_key=answer_private_key.public_key,
                ).dump_and_encrypt_for(recipient_pubkey=invitation_creator_user.public_key)

            except DataError as exc:
                raise InviteClaimError(f"Cannot generate device claim message: {exc}") from exc

            # 3) Send claim
            try:
                device_certificate, encrypted_answer = await cmds.device_claim(
                    new_device_id, encrypted_claim
                )

            except BackendNotAvailable as exc:
                raise InviteClaimBackendOfflineError(str(exc)) from exc

            except BackendConnectionError as exc:
                raise InviteClaimError(f"Cannot claim device: {exc}") from exc

            # 4) Verify device certificate
            try:
                DeviceCertificateContent.verify_and_load(
                    device_certificate,
                    author_verify_key=invitation_creator_device.verify_key,
                    expected_author=invitation_creator_device.device_id,
                    expected_device=new_device_id,
                )

            except DataError as exc:
                raise InviteClaimCryptoError(str(exc)) from exc

            try:
                answer = DeviceClaimAnswerContent.decrypt_and_load_for(
                    encrypted_answer, recipient_privkey=answer_private_key
                )

            except DataError as exc:
                raise InviteClaimCryptoError(f"Cannot decrypt device claim answer: {exc}") from exc

    except BackendNotAvailable as exc:
        raise InviteClaimBackendOfflineError(str(exc)) from exc

    return LocalDevice(
        organization_addr=backend_addr,
        device_id=new_device_id,
        signing_key=device_signing_key,
        private_key=answer.private_key,
        is_admin=invitation_creator_user.is_admin,
        user_manifest_id=answer.user_manifest_id,
        user_manifest_key=answer.user_manifest_key,
        local_symkey=SecretKey.generate(),
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
        device.organization_addr, device.device_id, device.signing_key, max_pool=1
    ) as cmds:
        try:
            encrypted_claim = await cmds.device_invite(new_device_name)

        except BackendNotAvailable as exc:
            raise InviteClaimBackendOfflineError(str(exc)) from exc

        except BackendConnectionError as exc:
            raise InviteClaimError(f"Cannot invite device: {exc}") from exc

        try:
            claim = DeviceClaimContent.decrypt_and_load_for(
                encrypted_claim, recipient_privkey=device.private_key
            )

        except DataError as exc:
            raise InviteClaimCryptoError(f"Cannot decrypt device claim info: {exc}") from exc

        if claim.token != token:
            raise InviteClaimInvalidTokenError(
                f"Invalid claim token provided by peer: `{claim['token']}`"
                f" (was expecting `{token}`)"
            )

        try:
            now = pendulum.now()
            device_certificate = DeviceCertificateContent(
                author=device.device_id,
                timestamp=now,
                device_id=claim.device_id,
                verify_key=claim.verify_key,
            ).dump_and_sign(device.signing_key)

        except DataError as exc:
            raise InviteClaimError(f"Cannot generate device certificate: {exc}") from exc

        try:
            encrypted_answer = DeviceClaimAnswerContent(
                private_key=device.private_key,
                user_manifest_id=device.user_manifest_id,
                user_manifest_key=device.user_manifest_key,
            ).dump_and_encrypt_for(recipient_pubkey=claim.answer_public_key)

        except DataError as exc:
            raise InviteClaimError(f"Cannot generate user claim answer message: {exc}") from exc

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
        device.organization_addr, device.device_id, device.signing_key, max_pool=1
    ) as cmds:
        try:
            encrypted_claim = await cmds.user_invite(user_id)

        except BackendCmdsTimeout as exc:
            raise InviteClaimTimeoutError(str(exc)) from exc

        except BackendNotAvailable as exc:
            raise InviteClaimBackendOfflineError(str(exc)) from exc

        except BackendConnectionError as exc:
            raise InviteClaimError(f"Cannot invite user: {exc}") from exc

        try:
            claim = UserClaimContent.decrypt_and_load_for(
                encrypted_claim, recipient_privkey=device.private_key
            )

        except DataError as exc:
            raise InviteClaimCryptoError(f"Cannot decrypt user claim info: {exc}") from exc

        if claim.token != token:
            raise InviteClaimInvalidTokenError(
                f"Invalid claim token provided by peer: `{claim.token}`"
                f" (was expecting `{token}`)"
            )

        device_id = claim.device_id
        now = pendulum.now()
        try:

            user_certificate = UserCertificateContent(
                author=device.device_id,
                timestamp=now,
                user_id=device_id.user_id,
                public_key=claim.public_key,
                is_admin=is_admin,
            ).dump_and_sign(device.signing_key)
            device_certificate = DeviceCertificateContent(
                author=device.device_id,
                timestamp=now,
                device_id=device_id,
                verify_key=claim.verify_key,
            ).dump_and_sign(device.signing_key)

        except DataError as exc:
            raise InviteClaimError(
                f"Cannot generate user&first device certificates: {exc}"
            ) from exc

        try:
            await cmds.user_create(user_certificate, device_certificate)

        except BackendNotAvailable as exc:
            raise InviteClaimBackendOfflineError(str(exc)) from exc

        except BackendConnectionError as exc:
            raise InviteClaimError(f"Cannot create user: {exc}") from exc

    return device_id
