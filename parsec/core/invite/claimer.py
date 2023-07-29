# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Any, List, Tuple, Type, TypeVar, Union

import attr

from parsec._parsec import (
    BackendActionAddr,
    BackendAddr,
    DeviceCreateRepOk,
    HashDigest,
    InvitationType,
    Invite1ClaimerWaitPeerRepInvalidState,
    Invite1ClaimerWaitPeerRepNotFound,
    Invite1ClaimerWaitPeerRepOk,
    Invite1GreeterWaitPeerRepAlreadyDeleted,
    Invite1GreeterWaitPeerRepInvalidState,
    Invite1GreeterWaitPeerRepNotFound,
    Invite1GreeterWaitPeerRepOk,
    Invite2aClaimerSendHashedNonceRepAlreadyDeleted,
    Invite2aClaimerSendHashedNonceRepInvalidState,
    Invite2aClaimerSendHashedNonceRepNotFound,
    Invite2aClaimerSendHashedNonceRepOk,
    Invite2aGreeterGetHashedNonceRepAlreadyDeleted,
    Invite2aGreeterGetHashedNonceRepInvalidState,
    Invite2aGreeterGetHashedNonceRepNotFound,
    Invite2aGreeterGetHashedNonceRepOk,
    Invite2bClaimerSendNonceRepInvalidState,
    Invite2bClaimerSendNonceRepNotFound,
    Invite2bClaimerSendNonceRepOk,
    Invite2bGreeterSendNonceRepAlreadyDeleted,
    Invite2bGreeterSendNonceRepInvalidState,
    Invite2bGreeterSendNonceRepNotFound,
    Invite2bGreeterSendNonceRepOk,
    Invite3aClaimerSignifyTrustRepInvalidState,
    Invite3aClaimerSignifyTrustRepNotFound,
    Invite3aClaimerSignifyTrustRepOk,
    Invite3aGreeterWaitPeerTrustRepAlreadyDeleted,
    Invite3aGreeterWaitPeerTrustRepInvalidState,
    Invite3aGreeterWaitPeerTrustRepNotFound,
    Invite3aGreeterWaitPeerTrustRepOk,
    Invite3bClaimerWaitPeerTrustRepInvalidState,
    Invite3bClaimerWaitPeerTrustRepNotFound,
    Invite3bClaimerWaitPeerTrustRepOk,
    Invite3bGreeterSignifyTrustRepAlreadyDeleted,
    Invite3bGreeterSignifyTrustRepInvalidState,
    Invite3bGreeterSignifyTrustRepNotFound,
    Invite3bGreeterSignifyTrustRepOk,
    Invite4ClaimerCommunicateRepInvalidState,
    Invite4ClaimerCommunicateRepNotFound,
    Invite4ClaimerCommunicateRepOk,
    Invite4GreeterCommunicateRepAlreadyDeleted,
    Invite4GreeterCommunicateRepInvalidState,
    Invite4GreeterCommunicateRepNotFound,
    Invite4GreeterCommunicateRepOk,
    InviteDeleteRepAlreadyDeleted,
    InviteDeleteRepNotFound,
    InviteDeleteRepOk,
    InviteInfoRepOk,
    InviteListRepOk,
    InviteNewRepOk,
    InviteShamirRecoveryRevealRepOk,
    PrivateKey,
    SecretKey,
    ShamirRecoveryCommunicatedData,
    ShamirRecoveryRecipient,
    ShamirRecoverySecret,
    ShamirShare,
    SigningKey,
    UserCreateRepActiveUsersLimitReached,
    UserCreateRepOk,
    generate_nonce,
    shamir_recover_secret,
)
from parsec._parsec import InvitedCmds as RsBackendInvitedCmds
from parsec.api.data import (
    DataError,
    InviteDeviceConfirmation,
    InviteDeviceData,
    InviteUserConfirmation,
    InviteUserData,
    SASCode,
    generate_sas_code_candidates,
    generate_sas_codes,
)
from parsec.api.protocol import DeviceLabel, HumanHandle, UserID
from parsec.core.backend_connection import BackendInvitedCmds
from parsec.core.invite import (
    InviteAlreadyUsedError,
    InviteError,
    InviteNotFoundError,
    InvitePeerResetError,
)
from parsec.core.types import BackendOrganizationAddr, LocalDevice

NOT_FOUND_TYPES = (
    Invite1ClaimerWaitPeerRepNotFound,
    Invite1GreeterWaitPeerRepNotFound,
    Invite2aClaimerSendHashedNonceRepNotFound,
    Invite2aGreeterGetHashedNonceRepNotFound,
    Invite2bClaimerSendNonceRepNotFound,
    Invite2bGreeterSendNonceRepNotFound,
    Invite3aClaimerSignifyTrustRepNotFound,
    Invite3aGreeterWaitPeerTrustRepNotFound,
    Invite3bClaimerWaitPeerTrustRepNotFound,
    Invite3bGreeterSignifyTrustRepNotFound,
    Invite4ClaimerCommunicateRepNotFound,
    Invite4GreeterCommunicateRepNotFound,
    InviteDeleteRepNotFound,
)

ALREADY_DELETED_TYPES = (
    Invite1GreeterWaitPeerRepAlreadyDeleted,
    Invite2aClaimerSendHashedNonceRepAlreadyDeleted,
    Invite2aGreeterGetHashedNonceRepAlreadyDeleted,
    Invite2bGreeterSendNonceRepAlreadyDeleted,
    Invite3aGreeterWaitPeerTrustRepAlreadyDeleted,
    Invite3bGreeterSignifyTrustRepAlreadyDeleted,
    Invite4GreeterCommunicateRepAlreadyDeleted,
    InviteDeleteRepAlreadyDeleted,
)

INVALID_STATE_TYPES = (
    Invite1GreeterWaitPeerRepInvalidState,
    Invite2aClaimerSendHashedNonceRepInvalidState,
    Invite2aGreeterGetHashedNonceRepInvalidState,
    Invite2bClaimerSendNonceRepInvalidState,
    Invite2bGreeterSendNonceRepInvalidState,
    Invite3aClaimerSignifyTrustRepInvalidState,
    Invite3aGreeterWaitPeerTrustRepInvalidState,
    Invite3bClaimerWaitPeerTrustRepInvalidState,
    Invite3bGreeterSignifyTrustRepInvalidState,
    Invite4ClaimerCommunicateRepInvalidState,
    Invite4GreeterCommunicateRepInvalidState,
    Invite1ClaimerWaitPeerRepInvalidState,
)

ACTIVE_USERS_LIMIT_REACHED_TYPES = (UserCreateRepActiveUsersLimitReached,)

T_OK_TYPES = TypeVar(
    "T_OK_TYPES",
    DeviceCreateRepOk,
    Invite1ClaimerWaitPeerRepOk,
    Invite1GreeterWaitPeerRepOk,
    Invite2aClaimerSendHashedNonceRepOk,
    Invite2aGreeterGetHashedNonceRepOk,
    Invite2bClaimerSendNonceRepOk,
    Invite2bGreeterSendNonceRepOk,
    Invite3aClaimerSignifyTrustRepOk,
    Invite3aGreeterWaitPeerTrustRepOk,
    Invite3bClaimerWaitPeerTrustRepOk,
    Invite3bGreeterSignifyTrustRepOk,
    Invite4ClaimerCommunicateRepOk,
    Invite4GreeterCommunicateRepOk,
    InviteDeleteRepOk,
    InviteInfoRepOk,
    InviteListRepOk,
    InviteNewRepOk,
    UserCreateRepOk,
)


def _check_rep(rep: Any, step_name: str, ok_type: Type[T_OK_TYPES]) -> T_OK_TYPES:
    if isinstance(rep, NOT_FOUND_TYPES):
        raise InviteNotFoundError
    elif isinstance(rep, ALREADY_DELETED_TYPES):
        raise InviteAlreadyUsedError
    elif isinstance(rep, INVALID_STATE_TYPES):
        raise InvitePeerResetError
    elif not isinstance(rep, ok_type):
        raise InviteError(f"Backend error during {step_name}: {rep}")
    return rep


async def claimer_retrieve_info(
    cmds: BackendInvitedCmds | RsBackendInvitedCmds,
) -> Union["UserClaimInitialCtx", "DeviceClaimInitialCtx", "ShamirRecoveryClaimPreludeCtx"]:
    rep = await cmds.invite_info()
    rep_ok = _check_rep(rep, step_name="invitation retrieval", ok_type=InviteInfoRepOk)
    if rep_ok.type == InvitationType.USER:
        return UserClaimInitialCtx(
            claimer_email=rep_ok.claimer_email,
            greeter_user_id=rep_ok.greeter_user_id,
            greeter_human_handle=rep_ok.greeter_human_handle,
            cmds=cmds,
        )
    if rep_ok.type == InvitationType.DEVICE:
        return DeviceClaimInitialCtx(
            greeter_user_id=rep_ok.greeter_user_id,
            greeter_human_handle=rep_ok.greeter_human_handle,
            cmds=cmds,
        )
    return ShamirRecoveryClaimPreludeCtx(
        recipients=list(rep_ok.recipients),
        threshold=rep_ok.threshold,
        recovered={},
        cmds=cmds,
    )


@attr.s(slots=True, auto_attribs=True)
class ShamirRecoveryClaimPreludeCtx:
    recipients: list[ShamirRecoveryRecipient]
    threshold: int
    recovered: dict[UserID, list[ShamirShare]]
    _cmds: BackendInvitedCmds | RsBackendInvitedCmds
    recovered_device: LocalDevice | None = attr.field(default=None)

    @property
    def shares(self) -> list[ShamirShare]:
        return [share for shares in self.recovered.values() for share in shares]

    @property
    def remaining_recipients(self) -> list[ShamirRecoveryRecipient]:
        return [
            recipient for recipient in self.recipients if recipient.user_id not in self.recovered
        ]

    def has_enough_shares(self) -> bool:
        return len(self.shares) >= self.threshold

    def get_initial_ctx(self, recipient: ShamirRecoveryRecipient) -> ShamirRecoveryClaimInitialCtx:
        return ShamirRecoveryClaimInitialCtx(
            recipient=recipient,
            greeter_user_id=recipient.user_id,
            greeter_human_handle=recipient.human_handle,
            cmds=self._cmds,
            prelude=self,
        )

    def add_shares(self, recipient_user_id: UserID, shares: list[ShamirShare]) -> bool:
        self.recovered[recipient_user_id] = shares
        return self.has_enough_shares()

    async def retrieve_recovery_device(self) -> LocalDevice:
        recovered_secret = ShamirRecoverySecret.load(
            shamir_recover_secret(self.threshold, self.shares)
        )
        rep = await self._cmds.invite_shamir_recovery_reveal(recovered_secret.reveal_token)
        assert isinstance(rep, InviteShamirRecoveryRevealRepOk)
        return LocalDevice.load(recovered_secret.data_key.decrypt(rep.ciphered_data))


@attr.s(slots=True, frozen=True, auto_attribs=True)
class BaseClaimInitialCtx:
    greeter_user_id: UserID
    greeter_human_handle: HumanHandle | None

    _cmds: BackendInvitedCmds | RsBackendInvitedCmds

    async def _do_wait_peer(self) -> Tuple[SASCode, SASCode, SecretKey]:
        claimer_private_key = PrivateKey.generate()
        rep = await self._cmds.invite_1_claimer_wait_peer(
            claimer_public_key=claimer_private_key.public_key,
            greeter_user_id=self.greeter_user_id,
        )
        rep_ok = _check_rep(rep, step_name="step 1", ok_type=Invite1ClaimerWaitPeerRepOk)

        shared_secret_key = claimer_private_key.generate_shared_secret_key(
            peer_public_key=rep_ok.greeter_public_key
        )
        claimer_nonce = generate_nonce()

        rep = await self._cmds.invite_2a_claimer_send_hashed_nonce(
            greeter_user_id=self.greeter_user_id,
            claimer_hashed_nonce=HashDigest.from_data(claimer_nonce),
        )
        rep_ok = _check_rep(rep, step_name="step 2a", ok_type=Invite2aClaimerSendHashedNonceRepOk)

        claimer_sas, greeter_sas = generate_sas_codes(
            claimer_nonce=claimer_nonce,
            greeter_nonce=rep_ok.greeter_nonce,
            shared_secret_key=shared_secret_key,
        )

        rep = await self._cmds.invite_2b_claimer_send_nonce(
            greeter_user_id=self.greeter_user_id, claimer_nonce=claimer_nonce
        )
        _check_rep(rep, step_name="step 2b", ok_type=Invite2bClaimerSendNonceRepOk)

        return claimer_sas, greeter_sas, shared_secret_key


@attr.s(slots=True, frozen=True, auto_attribs=True)
class UserClaimInitialCtx(BaseClaimInitialCtx):
    claimer_email: str

    async def do_wait_peer(self) -> "UserClaimInProgress1Ctx":
        claimer_sas, greeter_sas, shared_secret_key = await self._do_wait_peer()
        return UserClaimInProgress1Ctx(
            greeter_user_id=self.greeter_user_id,
            greeter_sas=greeter_sas,
            claimer_sas=claimer_sas,
            shared_secret_key=shared_secret_key,
            cmds=self._cmds,
        )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class DeviceClaimInitialCtx(BaseClaimInitialCtx):
    async def do_wait_peer(self) -> "DeviceClaimInProgress1Ctx":
        claimer_sas, greeter_sas, shared_secret_key = await self._do_wait_peer()
        return DeviceClaimInProgress1Ctx(
            greeter_user_id=self.greeter_user_id,
            greeter_sas=greeter_sas,
            claimer_sas=claimer_sas,
            shared_secret_key=shared_secret_key,
            cmds=self._cmds,
        )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class ShamirRecoveryClaimInitialCtx(BaseClaimInitialCtx):
    recipient: ShamirRecoveryRecipient
    prelude: ShamirRecoveryClaimPreludeCtx

    async def do_wait_peer(self) -> "ShamirRecoveryClaimInProgress1Ctx":
        claimer_sas, greeter_sas, shared_secret_key = await self._do_wait_peer()
        return ShamirRecoveryClaimInProgress1Ctx(
            greeter_user_id=self.greeter_user_id,
            greeter_sas=greeter_sas,
            claimer_sas=claimer_sas,
            shared_secret_key=shared_secret_key,
            cmds=self._cmds,
            prelude=self.prelude,
        )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class BaseClaimInProgress1Ctx:
    greeter_user_id: UserID
    greeter_sas: SASCode

    _claimer_sas: SASCode
    _shared_secret_key: SecretKey
    _cmds: BackendInvitedCmds | RsBackendInvitedCmds

    def generate_greeter_sas_choices(self, size: int = 3) -> List[SASCode]:
        return generate_sas_code_candidates(self.greeter_sas, size=size)

    async def _do_signify_trust(self) -> None:
        rep = await self._cmds.invite_3a_claimer_signify_trust(self.greeter_user_id)
        _check_rep(rep, step_name="step 3a", ok_type=Invite3aClaimerSignifyTrustRepOk)


@attr.s(slots=True, frozen=True, auto_attribs=True)
class UserClaimInProgress1Ctx(BaseClaimInProgress1Ctx):
    async def do_signify_trust(self) -> "UserClaimInProgress2Ctx":
        await self._do_signify_trust()
        return UserClaimInProgress2Ctx(
            greeter_user_id=self.greeter_user_id,
            claimer_sas=self._claimer_sas,
            shared_secret_key=self._shared_secret_key,
            cmds=self._cmds,
        )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class DeviceClaimInProgress1Ctx(BaseClaimInProgress1Ctx):
    async def do_signify_trust(self) -> "DeviceClaimInProgress2Ctx":
        await self._do_signify_trust()
        return DeviceClaimInProgress2Ctx(
            greeter_user_id=self.greeter_user_id,
            claimer_sas=self._claimer_sas,
            shared_secret_key=self._shared_secret_key,
            cmds=self._cmds,
        )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class ShamirRecoveryClaimInProgress1Ctx(BaseClaimInProgress1Ctx):
    prelude: ShamirRecoveryClaimPreludeCtx

    async def do_signify_trust(self) -> "ShamirRecoveryClaimInProgress2Ctx":
        await self._do_signify_trust()
        return ShamirRecoveryClaimInProgress2Ctx(
            greeter_user_id=self.greeter_user_id,
            claimer_sas=self._claimer_sas,
            shared_secret_key=self._shared_secret_key,
            cmds=self._cmds,
            prelude=self.prelude,
        )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class BaseClaimInProgress2Ctx:
    greeter_user_id: UserID
    claimer_sas: SASCode

    _shared_secret_key: SecretKey
    _cmds: BackendInvitedCmds | RsBackendInvitedCmds

    async def _do_wait_peer_trust(self) -> None:
        rep = await self._cmds.invite_3b_claimer_wait_peer_trust(self.greeter_user_id)
        _check_rep(rep, step_name="step 3b", ok_type=Invite3bClaimerWaitPeerTrustRepOk)


@attr.s(slots=True, frozen=True, auto_attribs=True)
class UserClaimInProgress2Ctx(BaseClaimInProgress2Ctx):
    async def do_wait_peer_trust(self) -> "UserClaimInProgress3Ctx":
        await self._do_wait_peer_trust()
        return UserClaimInProgress3Ctx(
            greeter_user_id=self.greeter_user_id,
            shared_secret_key=self._shared_secret_key,
            cmds=self._cmds,
        )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class DeviceClaimInProgress2Ctx(BaseClaimInProgress2Ctx):
    async def do_wait_peer_trust(self) -> "DeviceClaimInProgress3Ctx":
        await self._do_wait_peer_trust()
        return DeviceClaimInProgress3Ctx(
            greeter_user_id=self.greeter_user_id,
            shared_secret_key=self._shared_secret_key,
            cmds=self._cmds,
        )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class ShamirRecoveryClaimInProgress2Ctx(BaseClaimInProgress2Ctx):
    prelude: ShamirRecoveryClaimPreludeCtx

    async def do_wait_peer_trust(self) -> "ShamirRecoveryClaimInProgress3Ctx":
        await self._do_wait_peer_trust()
        return ShamirRecoveryClaimInProgress3Ctx(
            greeter_user_id=self.greeter_user_id,
            shared_secret_key=self._shared_secret_key,
            cmds=self._cmds,
            prelude=self.prelude,
        )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class UserClaimInProgress3Ctx:
    greeter_user_id: UserID
    _shared_secret_key: SecretKey
    _cmds: BackendInvitedCmds | RsBackendInvitedCmds

    async def do_claim_user(
        self,
        requested_device_label: DeviceLabel | None,
        requested_human_handle: HumanHandle | None,
    ) -> LocalDevice:
        # User&device keys are generated here and kept in memory until the end of
        # the enrollment process. This mean we can lost it if something goes wrong.
        # This has no impact until step 4 (somewhere between data exchange and
        # confirmation exchange steps) where greeter upload our certificates in
        # the server.
        # This is considered acceptable given 1) the error window is small and
        # 2) if this occurs the inviter can revoke the user and retry the
        # enrollment process to fix this
        private_key = PrivateKey.generate()
        signing_key = SigningKey.generate()

        try:
            payload = InviteUserData(
                requested_device_label=requested_device_label,
                requested_human_handle=requested_human_handle,
                public_key=private_key.public_key,
                verify_key=signing_key.verify_key,
            ).dump_and_encrypt(key=self._shared_secret_key)
        except DataError as exc:
            raise InviteError("Cannot generate InviteUserData payload") from exc

        rep = await self._cmds.invite_4_claimer_communicate(
            greeter_user_id=self.greeter_user_id, payload=payload
        )
        _check_rep(rep, step_name="step 4 (data exchange)", ok_type=Invite4ClaimerCommunicateRepOk)

        rep = await self._cmds.invite_4_claimer_communicate(
            greeter_user_id=self.greeter_user_id, payload=b""
        )
        rep_ok = _check_rep(
            rep, step_name="step 4 (confirmation exchange)", ok_type=Invite4ClaimerCommunicateRepOk
        )

        try:
            confirmation = InviteUserConfirmation.decrypt_and_load(
                rep_ok.payload, key=self._shared_secret_key
            )
        except DataError as exc:
            raise InviteError("Invalid InviteUserConfirmation payload provided by peer") from exc

        addr = self._cmds.addr
        assert not isinstance(
            addr, (BackendAddr, BackendActionAddr)
        ), "BackendAddr/BackendActionAddr don't have `get_backend_addr` defined"

        organization_addr = BackendOrganizationAddr.build(
            backend_addr=addr.get_backend_addr(),
            organization_id=addr.organization_id,
            root_verify_key=confirmation.root_verify_key,
        )

        new_device = LocalDevice.generate_new_device(
            organization_addr=organization_addr,
            device_id=confirmation.device_id,
            device_label=confirmation.device_label,
            human_handle=confirmation.human_handle,
            profile=confirmation.profile,
            private_key=private_key,
            signing_key=signing_key,
        )

        return new_device


@attr.s(slots=True, frozen=True, auto_attribs=True)
class DeviceClaimInProgress3Ctx:
    greeter_user_id: UserID
    _shared_secret_key: SecretKey
    _cmds: BackendInvitedCmds | RsBackendInvitedCmds

    async def do_claim_device(self, requested_device_label: DeviceLabel | None) -> LocalDevice:
        # Device key is generated here and kept in memory until the end of
        # the enrollment process. This mean we can lost it if something goes wrong.
        # This has no impact until step 4 (somewhere between data exchange and
        # confirmation exchange steps) where greeter upload our certificate in
        # the server.
        # This is considered acceptable given 1) the error window is small and
        # 2) if this occurs the inviter can revoke the device and retry the
        # enrollment process to fix this
        signing_key = SigningKey.generate()

        try:
            payload = InviteDeviceData(
                requested_device_label=requested_device_label,
                verify_key=signing_key.verify_key,
            ).dump_and_encrypt(key=self._shared_secret_key)
        except DataError as exc:
            raise InviteError("Cannot generate InviteDeviceData payload") from exc

        rep = await self._cmds.invite_4_claimer_communicate(
            greeter_user_id=self.greeter_user_id, payload=payload
        )
        _check_rep(rep, step_name="step 4 (data exchange)", ok_type=Invite4ClaimerCommunicateRepOk)

        rep = await self._cmds.invite_4_claimer_communicate(
            greeter_user_id=self.greeter_user_id, payload=b""
        )
        rep_ok = _check_rep(
            rep, step_name="step 4 (confirmation exchange)", ok_type=Invite4ClaimerCommunicateRepOk
        )

        try:
            confirmation = InviteDeviceConfirmation.decrypt_and_load(
                rep_ok.payload, key=self._shared_secret_key
            )
        except DataError as exc:
            raise InviteError("Invalid InviteDeviceConfirmation payload provided by peer") from exc

        addr = self._cmds.addr
        assert not isinstance(
            addr, (BackendAddr, BackendActionAddr)
        ), "BackendAddr/BackendActionAddr don't have `get_backend_addr` defined"

        organization_addr = BackendOrganizationAddr.build(
            backend_addr=addr.get_backend_addr(),
            organization_id=addr.organization_id,
            root_verify_key=confirmation.root_verify_key,
        )

        return LocalDevice(
            organization_addr=organization_addr,
            device_id=confirmation.device_id,
            device_label=confirmation.device_label,
            human_handle=confirmation.human_handle,
            profile=confirmation.profile,
            private_key=confirmation.private_key,
            signing_key=signing_key,
            user_manifest_id=confirmation.user_manifest_id,
            user_manifest_key=confirmation.user_manifest_key,
            local_symkey=SecretKey.generate(),
        )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class ShamirRecoveryClaimInProgress3Ctx:
    greeter_user_id: UserID
    _shared_secret_key: SecretKey
    _cmds: BackendInvitedCmds | RsBackendInvitedCmds
    prelude: ShamirRecoveryClaimPreludeCtx

    async def do_recover_share(
        self,
    ) -> bool:
        rep = await self._cmds.invite_4_claimer_communicate(
            greeter_user_id=self.greeter_user_id, payload=b""
        )
        rep = _check_rep(
            rep, step_name="step 4 (data exchange)", ok_type=Invite4ClaimerCommunicateRepOk
        )
        data = ShamirRecoveryCommunicatedData.load(rep.payload)

        return self.prelude.add_shares(self.greeter_user_id, data.weighted_share)
