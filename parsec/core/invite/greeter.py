# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Any, List, Tuple, Type

import attr

from parsec._parsec import AuthenticatedCmds as RsBackendAuthenticatedCmds
from parsec._parsec import (
    DeviceCreateRepOk,
    HashDigest,
    InvitationDeletedReason,
    Invite1GreeterWaitPeerRepOk,
    Invite2aGreeterGetHashedNonceRepOk,
    Invite2bGreeterSendNonceRepOk,
    Invite3aGreeterWaitPeerTrustRepOk,
    Invite3bGreeterSignifyTrustRepOk,
    Invite4GreeterCommunicateRepOk,
    PrivateKey,
    PublicKey,
    SecretKey,
    ShamirRecoveryCommunicatedData,
    ShamirRecoveryOthersListRepOk,
    ShamirRecoveryOthersListRepUnknownStatus,
    ShamirRecoveryShareCertificate,
    ShamirRecoveryShareData,
    UserCreateRepOk,
    UserID,
    VerifyKey,
    generate_nonce,
)
from parsec.api.data import (
    DataError,
    DeviceCertificate,
    InviteDeviceConfirmation,
    InviteDeviceData,
    InviteUserConfirmation,
    InviteUserData,
    SASCode,
    UserCertificate,
    generate_sas_code_candidates,
    generate_sas_codes,
)
from parsec.api.protocol import (
    DeviceID,
    DeviceLabel,
    DeviceName,
    HumanHandle,
    InvitationToken,
    UserProfile,
)
from parsec.core.backend_connection import BackendAuthenticatedCmds
from parsec.core.invite import (
    InviteActiveUsersLimitReachedError,
    InviteAlreadyUsedError,
    InviteError,
    InviteNotFoundError,
    InvitePeerResetError,
)
from parsec.core.invite.claimer import (
    ACTIVE_USERS_LIMIT_REACHED_TYPES,
    ALREADY_DELETED_TYPES,
    INVALID_STATE_TYPES,
    NOT_FOUND_TYPES,
    T_OK_TYPES,
)
from parsec.core.remote_devices_manager import RemoteDevicesManager
from parsec.core.types import LocalDevice

# Helper


async def get_shamir_recovery_share_data(
    cmds: BackendAuthenticatedCmds | RsBackendAuthenticatedCmds,
    device: LocalDevice,
    claimer_user_id: UserID,
    remote_devices_manager: RemoteDevicesManager | None = None,
) -> ShamirRecoveryShareData:
    # Create remote devices manager if necessary
    if remote_devices_manager is None:
        remote_devices_manager = RemoteDevicesManager(
            cmds, device.root_verify_key, device.time_provider
        )

    # Get shamir recovery certificates
    rep = await cmds.shamir_recovery_others_list()
    if isinstance(rep, ShamirRecoveryOthersListRepUnknownStatus):
        raise InviteError(f"Unknown status: {rep.status}")
    assert isinstance(rep, ShamirRecoveryOthersListRepOk)

    # Look for the right certificate
    for raw in rep.share_certificates:
        unsecure_share_certificate = ShamirRecoveryShareCertificate.unsecure_load(raw)
        if unsecure_share_certificate.author.user_id != claimer_user_id:
            continue

        # Retrieve the share
        author_certificate = await remote_devices_manager.get_device(
            unsecure_share_certificate.author
        )
        share_certificate = ShamirRecoveryShareCertificate.verify_and_load(
            raw, author_certificate.verify_key, unsecure_share_certificate.author
        )
        share_data = ShamirRecoveryShareData.decrypt_verify_and_load_for(
            share_certificate.ciphered_share, device.private_key, author_certificate.verify_key
        )
        return share_data

    raise InviteError("Shared recovery device not found")


def _check_rep(rep: Any, step_name: str, ok_type: Type[T_OK_TYPES]) -> T_OK_TYPES:
    if isinstance(rep, NOT_FOUND_TYPES):
        raise InviteNotFoundError
    elif isinstance(rep, ALREADY_DELETED_TYPES):
        raise InviteAlreadyUsedError
    elif isinstance(rep, INVALID_STATE_TYPES):
        raise InvitePeerResetError
    elif isinstance(rep, ACTIVE_USERS_LIMIT_REACHED_TYPES):
        raise InviteActiveUsersLimitReachedError
    elif not isinstance(rep, ok_type):
        raise InviteError(f"Backend error during {step_name}: {rep}")
    return rep


@attr.s(slots=True, frozen=True, auto_attribs=True)
class BaseGreetInitialCtx:
    token: InvitationToken
    _cmds: BackendAuthenticatedCmds | RsBackendAuthenticatedCmds

    async def _do_wait_peer(self) -> Tuple[SASCode, SASCode, SecretKey]:
        greeter_private_key = PrivateKey.generate()
        rep = await self._cmds.invite_1_greeter_wait_peer(
            token=self.token, greeter_public_key=greeter_private_key.public_key
        )
        rep_ok = _check_rep(rep, step_name="step 1", ok_type=Invite1GreeterWaitPeerRepOk)

        shared_secret_key = greeter_private_key.generate_shared_secret_key(
            peer_public_key=rep_ok.claimer_public_key
        )
        greeter_nonce = generate_nonce()

        rep = await self._cmds.invite_2a_greeter_get_hashed_nonce(token=self.token)
        rep_ok = _check_rep(rep, step_name="step 2a", ok_type=Invite2aGreeterGetHashedNonceRepOk)

        claimer_hashed_nonce = rep_ok.claimer_hashed_nonce

        rep = await self._cmds.invite_2b_greeter_send_nonce(
            token=self.token, greeter_nonce=greeter_nonce
        )
        rep2_ok = _check_rep(rep, step_name="step 2b", ok_type=Invite2bGreeterSendNonceRepOk)

        if HashDigest.from_data(rep2_ok.claimer_nonce) != claimer_hashed_nonce:
            raise InviteError("Invitee nonce and hashed nonce doesn't match")

        claimer_sas, greeter_sas = generate_sas_codes(
            claimer_nonce=rep2_ok.claimer_nonce,
            greeter_nonce=greeter_nonce,
            shared_secret_key=shared_secret_key,
        )

        return claimer_sas, greeter_sas, shared_secret_key


@attr.s(slots=True, frozen=True, auto_attribs=True)
class UserGreetInitialCtx(BaseGreetInitialCtx):
    async def do_wait_peer(self) -> "UserGreetInProgress1Ctx":
        claimer_sas, greeter_sas, shared_secret_key = await self._do_wait_peer()

        return UserGreetInProgress1Ctx(
            token=self.token,
            greeter_sas=greeter_sas,
            claimer_sas=claimer_sas,
            shared_secret_key=shared_secret_key,
            cmds=self._cmds,
        )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class DeviceGreetInitialCtx(BaseGreetInitialCtx):
    async def do_wait_peer(self) -> "DeviceGreetInProgress1Ctx":
        claimer_sas, greeter_sas, shared_secret_key = await self._do_wait_peer()

        return DeviceGreetInProgress1Ctx(
            token=self.token,
            greeter_sas=greeter_sas,
            claimer_sas=claimer_sas,
            shared_secret_key=shared_secret_key,
            cmds=self._cmds,
        )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class ShamirRecoveryGreetInitialCtx(BaseGreetInitialCtx):
    async def do_wait_peer(self) -> "ShamirRecoveryGreetInProgress1Ctx":
        claimer_sas, greeter_sas, shared_secret_key = await self._do_wait_peer()

        return ShamirRecoveryGreetInProgress1Ctx(
            token=self.token,
            greeter_sas=greeter_sas,
            claimer_sas=claimer_sas,
            shared_secret_key=shared_secret_key,
            cmds=self._cmds,
        )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class BaseGreetInProgress1Ctx:
    token: InvitationToken
    greeter_sas: SASCode

    _claimer_sas: SASCode
    _shared_secret_key: SecretKey
    _cmds: BackendAuthenticatedCmds | RsBackendAuthenticatedCmds

    async def _do_wait_peer_trust(self) -> None:
        rep = await self._cmds.invite_3a_greeter_wait_peer_trust(token=self.token)
        _check_rep(rep, step_name="step 3a", ok_type=Invite3aGreeterWaitPeerTrustRepOk)


@attr.s(slots=True, frozen=True, auto_attribs=True)
class UserGreetInProgress1Ctx(BaseGreetInProgress1Ctx):
    async def do_wait_peer_trust(self) -> "UserGreetInProgress2Ctx":
        await self._do_wait_peer_trust()

        return UserGreetInProgress2Ctx(
            token=self.token,
            claimer_sas=self._claimer_sas,
            shared_secret_key=self._shared_secret_key,
            cmds=self._cmds,
        )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class DeviceGreetInProgress1Ctx(BaseGreetInProgress1Ctx):
    async def do_wait_peer_trust(self) -> "DeviceGreetInProgress2Ctx":
        await self._do_wait_peer_trust()

        return DeviceGreetInProgress2Ctx(
            token=self.token,
            claimer_sas=self._claimer_sas,
            shared_secret_key=self._shared_secret_key,
            cmds=self._cmds,
        )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class ShamirRecoveryGreetInProgress1Ctx(BaseGreetInProgress1Ctx):
    async def do_wait_peer_trust(self) -> "ShamirRecoveryGreetInProgress2Ctx":
        await self._do_wait_peer_trust()

        return ShamirRecoveryGreetInProgress2Ctx(
            token=self.token,
            claimer_sas=self._claimer_sas,
            shared_secret_key=self._shared_secret_key,
            cmds=self._cmds,
        )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class BaseGreetInProgress2Ctx:
    token: InvitationToken
    claimer_sas: SASCode

    _shared_secret_key: SecretKey
    _cmds: BackendAuthenticatedCmds | RsBackendAuthenticatedCmds

    def generate_claimer_sas_choices(self, size: int = 3) -> List[SASCode]:
        return generate_sas_code_candidates(self.claimer_sas, size=size)

    async def _do_signify_trust(self) -> None:
        rep = await self._cmds.invite_3b_greeter_signify_trust(token=self.token)
        _check_rep(rep, step_name="step 3b", ok_type=Invite3bGreeterSignifyTrustRepOk)


@attr.s(slots=True, frozen=True, auto_attribs=True)
class UserGreetInProgress2Ctx(BaseGreetInProgress2Ctx):
    async def do_signify_trust(self) -> "UserGreetInProgress3Ctx":
        await self._do_signify_trust()

        return UserGreetInProgress3Ctx(
            token=self.token, shared_secret_key=self._shared_secret_key, cmds=self._cmds
        )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class DeviceGreetInProgress2Ctx(BaseGreetInProgress2Ctx):
    async def do_signify_trust(self) -> "DeviceGreetInProgress3Ctx":
        await self._do_signify_trust()

        return DeviceGreetInProgress3Ctx(
            token=self.token, shared_secret_key=self._shared_secret_key, cmds=self._cmds
        )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class ShamirRecoveryGreetInProgress2Ctx(BaseGreetInProgress2Ctx):
    async def do_signify_trust(self) -> "ShamirRecoveryGreetInProgress3Ctx":
        await self._do_signify_trust()

        return ShamirRecoveryGreetInProgress3Ctx(
            token=self.token, shared_secret_key=self._shared_secret_key, cmds=self._cmds
        )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class UserGreetInProgress3Ctx:
    token: InvitationToken

    _shared_secret_key: SecretKey
    _cmds: BackendAuthenticatedCmds | RsBackendAuthenticatedCmds

    async def do_get_claim_requests(self) -> "UserGreetInProgress4Ctx":
        rep = await self._cmds.invite_4_greeter_communicate(token=self.token, payload=b"")
        rep_ok = _check_rep(
            rep, step_name="step 4 (data exchange)", ok_type=Invite4GreeterCommunicateRepOk
        )

        if rep_ok.payload is None:
            raise InviteError("Missing InviteUserData payload")

        try:
            data = InviteUserData.decrypt_and_load(rep_ok.payload, key=self._shared_secret_key)
        except DataError as exc:
            raise InviteError("Invalid InviteUserData payload provided by peer") from exc

        return UserGreetInProgress4Ctx(
            token=self.token,
            requested_device_label=data.requested_device_label,
            requested_human_handle=data.requested_human_handle,
            public_key=data.public_key,
            verify_key=data.verify_key,
            shared_secret_key=self._shared_secret_key,
            cmds=self._cmds,
        )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class DeviceGreetInProgress3Ctx:
    token: InvitationToken

    _shared_secret_key: SecretKey
    _cmds: BackendAuthenticatedCmds | RsBackendAuthenticatedCmds

    async def do_get_claim_requests(self) -> "DeviceGreetInProgress4Ctx":
        rep = await self._cmds.invite_4_greeter_communicate(token=self.token, payload=b"")
        rep_ok = _check_rep(
            rep, step_name="step 4 (data exchange)", ok_type=Invite4GreeterCommunicateRepOk
        )

        if rep_ok.payload is None:
            raise InviteError("Missing InviteDeviceData payload")

        try:
            data = InviteDeviceData.decrypt_and_load(rep_ok.payload, key=self._shared_secret_key)
        except DataError as exc:
            raise InviteError("Invalid InviteDeviceData payload provided by peer") from exc

        return DeviceGreetInProgress4Ctx(
            token=self.token,
            requested_device_label=data.requested_device_label,
            verify_key=data.verify_key,
            shared_secret_key=self._shared_secret_key,
            cmds=self._cmds,
        )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class ShamirRecoveryGreetInProgress3Ctx:
    token: InvitationToken

    _shared_secret_key: SecretKey
    _cmds: BackendAuthenticatedCmds | RsBackendAuthenticatedCmds

    async def send_share_data(self, share_data: ShamirRecoveryShareData) -> None:
        to_communicate = ShamirRecoveryCommunicatedData(share_data.weighted_share)
        rep = await self._cmds.invite_4_greeter_communicate(
            token=self.token, payload=to_communicate.dump()
        )
        _check_rep(rep, step_name="step 4 (data exchange)", ok_type=Invite4GreeterCommunicateRepOk)


def _create_new_user_certificates(
    author: LocalDevice,
    device_label: DeviceLabel | None,
    human_handle: HumanHandle | None,
    profile: UserProfile,
    public_key: PublicKey,
    verify_key: VerifyKey,
) -> Tuple[bytes, bytes, bytes, bytes, InviteUserConfirmation]:
    """Helper to prepare the creation of a new user."""
    device_id = DeviceID.new()
    try:
        timestamp = author.timestamp()

        user_certificate = UserCertificate(
            author=author.device_id,
            timestamp=timestamp,
            user_id=device_id.user_id,
            human_handle=human_handle,
            public_key=public_key,
            profile=profile,
        )
        redacted_user_certificate = user_certificate.evolve(human_handle=None)

        device_certificate = DeviceCertificate(
            author=author.device_id,
            timestamp=timestamp,
            device_id=device_id,
            device_label=device_label,
            verify_key=verify_key,
        )
        redacted_device_certificate = device_certificate.evolve(device_label=None)

        user_certificate_bytes = user_certificate.dump_and_sign(author.signing_key)
        redacted_user_certificate_bytes = redacted_user_certificate.dump_and_sign(
            author.signing_key
        )
        device_certificate_bytes = device_certificate.dump_and_sign(author.signing_key)
        redacted_device_certificate_bytes = redacted_device_certificate.dump_and_sign(
            author.signing_key
        )

    except DataError as exc:
        raise InviteError(f"Cannot generate device certificate: {exc}") from exc

    invite_user_confirmation = InviteUserConfirmation(
        device_id=device_id,
        device_label=device_label,
        human_handle=human_handle,
        profile=profile,
        root_verify_key=author.root_verify_key,
    )

    return (
        user_certificate_bytes,
        redacted_user_certificate_bytes,
        device_certificate_bytes,
        redacted_device_certificate_bytes,
        invite_user_confirmation,
    )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class UserGreetInProgress4Ctx:
    token: InvitationToken
    requested_device_label: DeviceLabel | None
    requested_human_handle: HumanHandle | None

    _public_key: PublicKey
    _verify_key: VerifyKey
    _shared_secret_key: SecretKey
    _cmds: BackendAuthenticatedCmds | RsBackendAuthenticatedCmds

    async def do_create_new_user(
        self,
        author: LocalDevice,
        device_label: DeviceLabel | None,
        human_handle: HumanHandle | None,
        profile: UserProfile,
    ) -> None:
        (
            user_certificate,
            redacted_user_certificate,
            device_certificate,
            redacted_device_certificate,
            invite_user_confirmation,
        ) = _create_new_user_certificates(
            author,
            device_label,
            human_handle,
            profile,
            self._public_key,
            self._verify_key,
        )

        rep = await self._cmds.user_create(
            user_certificate=user_certificate,
            device_certificate=device_certificate,
            redacted_user_certificate=redacted_user_certificate,
            redacted_device_certificate=redacted_device_certificate,
        )
        _check_rep(rep, step_name="step 4 (user certificates upload)", ok_type=UserCreateRepOk)

        # From now on the user has been created on the server, but greeter
        # is not aware of it yet. If something goes wrong, we can end up with
        # the greeter losing it private keys.
        # This is considered acceptable given 1) the error window is small and
        # 2) if this occurs the inviter can revoke the user and retry the
        # enrollment process to fix this

        try:
            payload = invite_user_confirmation.dump_and_encrypt(key=self._shared_secret_key)
        except DataError as exc:
            raise InviteError("Cannot generate InviteUserConfirmation payload") from exc

        rep = await self._cmds.invite_4_greeter_communicate(token=self.token, payload=payload)
        _check_rep(
            rep, step_name="step 4 (confirmation exchange)", ok_type=Invite4GreeterCommunicateRepOk
        )

        # Invitation deletion is not strictly necessary (enrollment has succeeded
        # anyway) so it's no big deal if something goes wrong before it can be
        # done (and it can be manually deleted from invitation list).
        await self._cmds.invite_delete(token=self.token, reason=InvitationDeletedReason.FINISHED)


@attr.s(slots=True, frozen=True, auto_attribs=True)
class DeviceGreetInProgress4Ctx:
    token: InvitationToken
    requested_device_label: DeviceLabel | None

    _verify_key: VerifyKey
    _shared_secret_key: SecretKey
    _cmds: BackendAuthenticatedCmds | RsBackendAuthenticatedCmds

    async def do_create_new_device(
        self, author: LocalDevice, device_label: DeviceLabel | None
    ) -> None:
        device_id = author.user_id.to_device_id(DeviceName.new())
        try:
            timestamp = author.timestamp()

            device_certificate = DeviceCertificate(
                author=author.device_id,
                timestamp=timestamp,
                device_id=device_id,
                device_label=device_label,
                verify_key=self._verify_key,
            )
            redacted_device_certificate = device_certificate.evolve(device_label=None)

            device_certificate_bytes = device_certificate.dump_and_sign(author.signing_key)
            redacted_device_certificate_bytes = redacted_device_certificate.dump_and_sign(
                author.signing_key
            )

        except DataError as exc:
            raise InviteError(f"Cannot generate device certificate: {exc}") from exc

        rep = await self._cmds.device_create(
            device_certificate=device_certificate_bytes,
            redacted_device_certificate=redacted_device_certificate_bytes,
        )
        _check_rep(rep, step_name="step 4 (device certificates upload)", ok_type=DeviceCreateRepOk)

        # From now on the device has been created on the server, but greeter
        # is not aware of it yet. If something goes wrong, we can end up with
        # the greeter losing it private keys.
        # This is considered acceptable given 1) the error window is small and
        # 2) if this occurs the inviter can revoke the device and retry the
        # enrollment process to fix this

        try:
            payload = InviteDeviceConfirmation(
                device_id=device_id,
                device_label=device_label,
                human_handle=author.human_handle,
                profile=author.profile,
                private_key=author.private_key,
                user_manifest_id=author.user_manifest_id,
                user_manifest_key=author.user_manifest_key,
                root_verify_key=author.root_verify_key,
            ).dump_and_encrypt(key=self._shared_secret_key)
        except DataError as exc:
            raise InviteError("Cannot generate InviteUserConfirmation payload") from exc

        rep = await self._cmds.invite_4_greeter_communicate(token=self.token, payload=payload)
        _check_rep(
            rep, step_name="step 4 (confirmation exchange)", ok_type=Invite4GreeterCommunicateRepOk
        )

        # Invitation deletion is not strictly necessary (enrollment has succeeded
        # anyway) so it's no big deal if something goes wrong before it can be
        # done (and it can be manually deleted from invitation list).
        await self._cmds.invite_delete(token=self.token, reason=InvitationDeletedReason.FINISHED)
