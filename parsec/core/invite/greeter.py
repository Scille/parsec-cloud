# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import attr
from typing import Optional, List, Tuple

from parsec.crypto import (
    generate_shared_secret_key,
    generate_nonce,
    SecretKey,
    PrivateKey,
    HashDigest,
    PublicKey,
    VerifyKey,
)
from parsec.api.data import (
    DataError,
    SASCode,
    generate_sas_codes,
    generate_sas_code_candidates,
    InviteUserData,
    InviteUserConfirmation,
    InviteDeviceData,
    InviteDeviceConfirmation,
    DeviceCertificate,
    UserCertificate,
)
from parsec.api.protocol import (
    DeviceName,
    DeviceID,
    HumanHandle,
    InvitationToken,
    InvitationDeletedReason,
    DeviceLabel,
    UserProfile,
)
from parsec.core.backend_connection import BackendAuthenticatedCmds
from parsec.core.types import LocalDevice
from parsec.core.invite.exceptions import (
    InviteError,
    InvitePeerResetError,
    InviteNotFoundError,
    InviteAlreadyUsedError,
    InviteActiveUsersLimitReachedError,
)


def _check_rep(rep, step_name):
    from parsec.core.invite.claimer import (
        NOT_FOUND_TYPES,
        ALREADY_DELETED_TYPES,
        INVALID_STATE_TYPES,
        OK_TYPES,
    )

    if type(rep) in NOT_FOUND_TYPES:
        raise InviteNotFoundError
    elif type(rep) in ALREADY_DELETED_TYPES:
        raise InviteAlreadyUsedError
    elif type(rep) in INVALID_STATE_TYPES:
        raise InvitePeerResetError
    elif type(rep) == dict and rep["status"] == "active_users_limit_reached":
        raise InviteActiveUsersLimitReachedError
    elif type(rep) not in OK_TYPES and rep != {"status": "ok"}:
        raise InviteError(f"Backend error during {step_name}: {rep}")


@attr.s(slots=True, frozen=True, auto_attribs=True)
class BaseGreetInitialCtx:
    token: InvitationToken
    _cmds: BackendAuthenticatedCmds

    async def _do_wait_peer(self) -> Tuple[SASCode, SASCode, SecretKey]:
        greeter_private_key = PrivateKey.generate()
        rep = await self._cmds.invite_1_greeter_wait_peer(
            token=self.token, greeter_public_key=greeter_private_key.public_key
        )
        from parsec.core.invite.claimer import (
            NOT_FOUND_TYPES,
            ALREADY_DELETED_TYPES,
            INVALID_STATE_TYPES,
            OK_TYPES,
        )

        if type(rep) in NOT_FOUND_TYPES:
            raise InviteNotFoundError
        elif type(rep) in ALREADY_DELETED_TYPES:
            raise InviteAlreadyUsedError
        elif type(rep) in INVALID_STATE_TYPES:
            raise InvitePeerResetError
        elif type(rep) not in OK_TYPES:
            raise InviteError(f"Backend error during step 1: {rep}")

        shared_secret_key = generate_shared_secret_key(
            our_private_key=greeter_private_key, peer_public_key=rep.claimer_public_key
        )
        greeter_nonce = generate_nonce()

        rep = await self._cmds.invite_2a_greeter_get_hashed_nonce(token=self.token)
        _check_rep(rep, step_name="step 2a")

        claimer_hashed_nonce = rep.claimer_hashed_nonce

        rep = await self._cmds.invite_2b_greeter_send_nonce(
            token=self.token, greeter_nonce=greeter_nonce
        )
        _check_rep(rep, step_name="step 2b")

        if HashDigest.from_data(rep.claimer_nonce) != claimer_hashed_nonce:
            raise InviteError("Invitee nonce and hashed nonce doesn't match")

        claimer_sas, greeter_sas = generate_sas_codes(
            claimer_nonce=rep.claimer_nonce,
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
class BaseGreetInProgress1Ctx:
    token: InvitationToken
    greeter_sas: SASCode

    _claimer_sas: SASCode
    _shared_secret_key: SecretKey
    _cmds: BackendAuthenticatedCmds

    async def _do_wait_peer_trust(self) -> None:
        rep = await self._cmds.invite_3a_greeter_wait_peer_trust(token=self.token)
        _check_rep(rep, step_name="step 3b")


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
class BaseGreetInProgress2Ctx:
    token: InvitationToken
    claimer_sas: SASCode

    _shared_secret_key: SecretKey
    _cmds: BackendAuthenticatedCmds

    def generate_claimer_sas_choices(self, size: int = 3) -> List[SASCode]:
        return generate_sas_code_candidates(self.claimer_sas, size=size)

    async def _do_signify_trust(self) -> None:
        rep = await self._cmds.invite_3b_greeter_signify_trust(token=self.token)
        _check_rep(rep, step_name="step 3a")


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
class UserGreetInProgress3Ctx:
    token: InvitationToken

    _shared_secret_key: SecretKey
    _cmds: BackendAuthenticatedCmds

    async def do_get_claim_requests(self) -> "UserGreetInProgress4Ctx":
        rep = await self._cmds.invite_4_greeter_communicate(token=self.token, payload=b"")
        _check_rep(rep, step_name="step 4 (data exchange)")

        if rep.payload is None:
            raise InviteError("Missing InviteUserData payload")

        try:
            data = InviteUserData.decrypt_and_load(rep.payload, key=self._shared_secret_key)
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
    _cmds: BackendAuthenticatedCmds

    async def do_get_claim_requests(self) -> "DeviceGreetInProgress4Ctx":
        rep = await self._cmds.invite_4_greeter_communicate(token=self.token, payload=b"")
        _check_rep(rep, step_name="step 4 (data exchange)")

        if rep.payload is None:
            raise InviteError("Missing InviteDeviceData payload")

        try:
            data = InviteDeviceData.decrypt_and_load(rep.payload, key=self._shared_secret_key)
        except DataError as exc:
            raise InviteError("Invalid InviteDeviceData payload provided by peer") from exc

        return DeviceGreetInProgress4Ctx(
            token=self.token,
            requested_device_label=data.requested_device_label,
            verify_key=data.verify_key,
            shared_secret_key=self._shared_secret_key,
            cmds=self._cmds,
        )


def _create_new_user_certificates(
    author: LocalDevice,
    device_label: Optional[DeviceLabel],
    human_handle: Optional[HumanHandle],
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

        user_certificate = user_certificate.dump_and_sign(author.signing_key)
        redacted_user_certificate = redacted_user_certificate.dump_and_sign(author.signing_key)
        device_certificate = device_certificate.dump_and_sign(author.signing_key)
        redacted_device_certificate = redacted_device_certificate.dump_and_sign(author.signing_key)

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
        user_certificate,
        redacted_user_certificate,
        device_certificate,
        redacted_device_certificate,
        invite_user_confirmation,
    )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class UserGreetInProgress4Ctx:
    token: InvitationToken
    requested_device_label: Optional[DeviceLabel]
    requested_human_handle: Optional[HumanHandle]

    _public_key: PublicKey
    _verify_key: VerifyKey
    _shared_secret_key: SecretKey
    _cmds: BackendAuthenticatedCmds

    async def do_create_new_user(
        self,
        author: LocalDevice,
        device_label: Optional[DeviceLabel],
        human_handle: Optional[HumanHandle],
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
        _check_rep(rep, step_name="step 4 (user certificates upload)")

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
        _check_rep(rep, step_name="step 4 (confirmation exchange)")

        # Invitation deletion is not strictly necessary (enrollment has succeeded
        # anyway) so it's no big deal if something goes wrong before it can be
        # done (and it can be manually deleted from invitation list).
        await self._cmds.invite_delete(token=self.token, reason=InvitationDeletedReason.FINISHED)


@attr.s(slots=True, frozen=True, auto_attribs=True)
class DeviceGreetInProgress4Ctx:
    token: InvitationToken
    requested_device_label: Optional[DeviceLabel]

    _verify_key: VerifyKey
    _shared_secret_key: SecretKey
    _cmds: BackendAuthenticatedCmds

    async def do_create_new_device(
        self, author: LocalDevice, device_label: Optional[DeviceLabel]
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

            device_certificate = device_certificate.dump_and_sign(author.signing_key)
            redacted_device_certificate = redacted_device_certificate.dump_and_sign(
                author.signing_key
            )

        except DataError as exc:
            raise InviteError(f"Cannot generate device certificate: {exc}") from exc

        rep = await self._cmds.device_create(
            device_certificate=device_certificate,
            redacted_device_certificate=redacted_device_certificate,
        )
        _check_rep(rep, step_name="step 4 (device certificates upload)")

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
        _check_rep(rep, step_name="step 4 (confirmation exchange)")

        # Invitation deletion is not strictly necessary (enrollment has succeeded
        # anyway) so it's no big deal if something goes wrong before it can be
        # done (and it can be manually deleted from invitation list).
        await self._cmds.invite_delete(token=self.token, reason=InvitationDeletedReason.FINISHED)
