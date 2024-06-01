# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from dataclasses import dataclass
from enum import auto
from typing import TypeAlias

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    RealmKeyRotationCertificate,
    RealmNameCertificate,
    RealmRole,
    RealmRoleCertificate,
    UserID,
    VerifyKey,
    VlobID,
    authenticated_cmds,
)
from parsec.api import api
from parsec.ballpark import (
    RequireGreaterTimestamp,
    TimestampOutOfBallpark,
    timestamps_in_the_ballpark,
)
from parsec.client_context import AuthenticatedClientContext
from parsec.types import BadOutcome, BadOutcomeEnum

KeyIndex: TypeAlias = int


@dataclass(slots=True)
class BadKeyIndex(BadOutcome):
    last_realm_certificate_timestamp: DateTime


@dataclass(slots=True)
class CertificateBasedActionIdempotentOutcome(BadOutcome):
    certificate_timestamp: DateTime


@dataclass(slots=True)
class RealmStats:
    blocks_size: int
    vlobs_size: int


@dataclass(slots=True, repr=False)
class RealmGrantedRole:
    def __repr__(self) -> str:
        return f"{self.__class__.__name__}({self.user_id.hex} {self.role})"

    certificate: bytes
    realm_id: VlobID
    user_id: UserID
    role: RealmRole | None
    granted_by: DeviceID | None
    granted_on: DateTime


@dataclass(slots=True)
class KeysBundle:
    key_index: int
    keys_bundle_access: bytes
    keys_bundle: bytes


class RealmCreateValidateBadOutcome(BadOutcomeEnum):
    INVALID_CERTIFICATE = auto()
    TIMESTAMP_MISMATCH = auto()
    INVALID_ROLE = auto()
    USER_ID_MISMATCH = auto()


def realm_create_validate(
    now: DateTime,
    expected_author_user_id: UserID,
    expected_author_device_id: DeviceID,
    author_verify_key: VerifyKey,
    realm_role_certificate: bytes,
) -> RealmRoleCertificate | TimestampOutOfBallpark | RealmCreateValidateBadOutcome:
    try:
        data = RealmRoleCertificate.verify_and_load(
            realm_role_certificate,
            author_verify_key=author_verify_key,
            expected_author=expected_author_device_id,
        )

    except ValueError:
        return RealmCreateValidateBadOutcome.INVALID_CERTIFICATE

    match timestamps_in_the_ballpark(data.timestamp, now):
        case TimestampOutOfBallpark() as error:
            return error
        case _:
            pass

    if expected_author_user_id != data.user_id:
        return RealmCreateValidateBadOutcome.USER_ID_MISMATCH

    if data.role != RealmRole.OWNER:
        return RealmCreateValidateBadOutcome.INVALID_ROLE

    return data


class RealmShareValidateBadOutcome(BadOutcomeEnum):
    INVALID_CERTIFICATE = auto()
    TIMESTAMP_MISMATCH = auto()
    INVALID_ROLE = auto()
    CANNOT_SELF_SHARE = auto()


def realm_share_validate(
    now: DateTime,
    expected_author_user_id: UserID,
    expected_author_device_id: DeviceID,
    author_verify_key: VerifyKey,
    realm_role_certificate: bytes,
) -> RealmRoleCertificate | TimestampOutOfBallpark | RealmShareValidateBadOutcome:
    try:
        data = RealmRoleCertificate.verify_and_load(
            realm_role_certificate,
            author_verify_key=author_verify_key,
            expected_author=expected_author_device_id,
        )

    except ValueError:
        return RealmShareValidateBadOutcome.INVALID_CERTIFICATE

    match timestamps_in_the_ballpark(data.timestamp, now):
        case TimestampOutOfBallpark() as error:
            return error
        case _:
            pass

    if expected_author_user_id == data.user_id:
        return RealmShareValidateBadOutcome.CANNOT_SELF_SHARE

    if data.role is None:
        return RealmShareValidateBadOutcome.INVALID_ROLE

    return data


class RealmUnshareValidateBadOutcome(BadOutcomeEnum):
    INVALID_CERTIFICATE = auto()
    TIMESTAMP_MISMATCH = auto()
    INVALID_ROLE = auto()
    CANNOT_SELF_UNSHARE = auto()


def realm_unshare_validate(
    now: DateTime,
    expected_author_user_id: UserID,
    expected_author_device_id: DeviceID,
    author_verify_key: VerifyKey,
    realm_role_certificate: bytes,
) -> RealmRoleCertificate | TimestampOutOfBallpark | RealmUnshareValidateBadOutcome:
    try:
        data = RealmRoleCertificate.verify_and_load(
            realm_role_certificate,
            author_verify_key=author_verify_key,
            expected_author=expected_author_device_id,
        )

    except ValueError:
        return RealmUnshareValidateBadOutcome.INVALID_CERTIFICATE

    match timestamps_in_the_ballpark(data.timestamp, now):
        case TimestampOutOfBallpark() as error:
            return error
        case _:
            pass

    if expected_author_user_id == data.user_id:
        return RealmUnshareValidateBadOutcome.CANNOT_SELF_UNSHARE

    if data.role is not None:
        return RealmUnshareValidateBadOutcome.INVALID_ROLE

    return data


class RealmRenameValidateBadOutcome(BadOutcomeEnum):
    INVALID_CERTIFICATE = auto()
    TIMESTAMP_MISMATCH = auto()


def realm_rename_validate(
    now: DateTime,
    expected_author: DeviceID,
    author_verify_key: VerifyKey,
    realm_name_certificate: bytes,
) -> RealmNameCertificate | TimestampOutOfBallpark | RealmRenameValidateBadOutcome:
    try:
        data = RealmNameCertificate.verify_and_load(
            realm_name_certificate,
            author_verify_key=author_verify_key,
            expected_author=expected_author,
        )

    except ValueError:
        return RealmRenameValidateBadOutcome.INVALID_CERTIFICATE

    match timestamps_in_the_ballpark(data.timestamp, now):
        case TimestampOutOfBallpark() as error:
            return error
        case _:
            pass

    return data


class RealmRotateKeyValidateBadOutcome(BadOutcomeEnum):
    INVALID_CERTIFICATE = auto()
    TIMESTAMP_MISMATCH = auto()


def realm_rotate_key_validate(
    now: DateTime,
    expected_author: DeviceID,
    author_verify_key: VerifyKey,
    realm_key_rotation_certificate: bytes,
) -> RealmKeyRotationCertificate | TimestampOutOfBallpark | RealmRotateKeyValidateBadOutcome:
    try:
        data = RealmKeyRotationCertificate.verify_and_load(
            realm_key_rotation_certificate,
            author_verify_key=author_verify_key,
            expected_author=expected_author,
        )

    except ValueError:
        return RealmRotateKeyValidateBadOutcome.INVALID_CERTIFICATE

    match timestamps_in_the_ballpark(data.timestamp, now):
        case TimestampOutOfBallpark() as error:
            return error
        case _:
            pass

    return data


class RealmCheckBadOutcome(BadOutcomeEnum):
    REALM_NOT_FOUND = auto()
    USER_NOT_IN_REALM = auto()


class RealmCreateStoreBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()


class RealmShareStoreBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    REALM_NOT_FOUND = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    AUTHOR_NOT_ALLOWED = auto()
    RECIPIENT_NOT_FOUND = auto()
    RECIPIENT_REVOKED = auto()
    ROLE_INCOMPATIBLE_WITH_OUTSIDER = auto()


class RealmUnshareStoreBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    REALM_NOT_FOUND = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    AUTHOR_NOT_ALLOWED = auto()
    RECIPIENT_NOT_FOUND = auto()


class RealmRenameStoreBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    REALM_NOT_FOUND = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    AUTHOR_NOT_ALLOWED = auto()


class RealmRotateKeyStoreBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    REALM_NOT_FOUND = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    AUTHOR_NOT_ALLOWED = auto()
    PARTICIPANT_MISMATCH = auto()


class RealmGetKeysBundleBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    REALM_NOT_FOUND = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    AUTHOR_NOT_ALLOWED = auto()
    ACCESS_NOT_AVAILABLE_FOR_AUTHOR = auto()
    BAD_KEY_INDEX = auto()


class RealmGetStatsAsUserBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    REALM_NOT_FOUND = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    AUTHOR_NOT_ALLOWED = auto()


class RealmGetStatsBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    REALM_NOT_FOUND = auto()


class RealmGetCurrentRealmsForUserBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    USER_NOT_FOUND = auto()


class RealmDumpRealmsGrantedRolesBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()


class BaseRealmComponent:
    #
    # Public methods
    #

    async def create(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        author_verify_key: VerifyKey,
        realm_role_certificate: bytes,
    ) -> (
        RealmRoleCertificate
        | CertificateBasedActionIdempotentOutcome
        | RealmCreateValidateBadOutcome
        | TimestampOutOfBallpark
        | RealmCreateStoreBadOutcome
        | RequireGreaterTimestamp
    ):
        raise NotImplementedError

    async def share(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        author_verify_key: VerifyKey,
        realm_role_certificate: bytes,
        recipient_keys_bundle_access: bytes,
        key_index: int,
    ) -> (
        RealmRoleCertificate
        | BadKeyIndex
        | CertificateBasedActionIdempotentOutcome
        | RealmShareValidateBadOutcome
        | TimestampOutOfBallpark
        | RealmShareStoreBadOutcome
        | RequireGreaterTimestamp
    ):
        raise NotImplementedError

    async def unshare(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        author_verify_key: VerifyKey,
        realm_role_certificate: bytes,
    ) -> (
        RealmRoleCertificate
        | CertificateBasedActionIdempotentOutcome
        | RealmUnshareValidateBadOutcome
        | TimestampOutOfBallpark
        | RealmUnshareStoreBadOutcome
        | RequireGreaterTimestamp
    ):
        raise NotImplementedError

    async def rename(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        author_verify_key: VerifyKey,
        realm_name_certificate: bytes,
        initial_name_or_fail: bool,
    ) -> (
        RealmNameCertificate
        | BadKeyIndex
        | CertificateBasedActionIdempotentOutcome
        | RealmRenameValidateBadOutcome
        | TimestampOutOfBallpark
        | RealmRenameStoreBadOutcome
        | RequireGreaterTimestamp
    ):
        raise NotImplementedError

    async def rotate_key(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        author_verify_key: VerifyKey,
        realm_key_rotation_certificate: bytes,
        per_participant_keys_bundle_access: dict[UserID, bytes],
        keys_bundle: bytes,
    ) -> (
        RealmKeyRotationCertificate
        | BadKeyIndex
        | RealmRotateKeyValidateBadOutcome
        | TimestampOutOfBallpark
        | RealmRotateKeyStoreBadOutcome
        | RequireGreaterTimestamp
    ):
        raise NotImplementedError

    async def get_keys_bundle(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: VlobID,
        key_index: int | None,
    ) -> KeysBundle | RealmGetKeysBundleBadOutcome:
        raise NotImplementedError

    async def get_stats(
        self, organization_id: OrganizationID, author: DeviceID, realm_id: VlobID
    ) -> RealmStats | RealmGetStatsAsUserBadOutcome:
        raise NotImplementedError

    async def get_current_realms_for_user(
        self, organization_id: OrganizationID, user: UserID
    ) -> dict[VlobID, RealmRole] | RealmGetCurrentRealmsForUserBadOutcome:
        raise NotImplementedError

    async def dump_realms_granted_roles(
        self, organization_id: OrganizationID
    ) -> list[RealmGrantedRole] | RealmDumpRealmsGrantedRolesBadOutcome:
        raise NotImplementedError

    #
    # API commands
    #

    @api
    async def api_realm_create(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.realm_create.Req,
    ) -> authenticated_cmds.latest.realm_create.Rep:
        outcome = await self.create(
            now=DateTime.now(),
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            author_verify_key=client_ctx.device_verify_key,
            realm_role_certificate=req.realm_role_certificate,
        )
        match outcome:
            case RealmRoleCertificate():
                return authenticated_cmds.latest.realm_create.RepOk()
            case RequireGreaterTimestamp() as error:
                return authenticated_cmds.latest.realm_create.RepRequireGreaterTimestamp(
                    strictly_greater_than=error.strictly_greater_than
                )
            case TimestampOutOfBallpark() as error:
                return authenticated_cmds.latest.realm_create.RepTimestampOutOfBallpark(
                    server_timestamp=error.server_timestamp,
                    client_timestamp=error.client_timestamp,
                    ballpark_client_early_offset=error.ballpark_client_early_offset,
                    ballpark_client_late_offset=error.ballpark_client_late_offset,
                )
            case RealmCreateValidateBadOutcome():
                return authenticated_cmds.latest.realm_create.RepInvalidCertificate()
            case CertificateBasedActionIdempotentOutcome() as error:
                return authenticated_cmds.latest.realm_create.RepRealmAlreadyExists(
                    last_realm_certificate_timestamp=error.certificate_timestamp
                )
            case RealmCreateStoreBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case RealmCreateStoreBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case RealmCreateStoreBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case RealmCreateStoreBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()

    @api
    async def api_realm_share(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.realm_share.Req,
    ) -> authenticated_cmds.latest.realm_share.Rep:
        outcome = await self.share(
            now=DateTime.now(),
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            author_verify_key=client_ctx.device_verify_key,
            realm_role_certificate=req.realm_role_certificate,
            recipient_keys_bundle_access=req.recipient_keys_bundle_access,
            key_index=req.key_index,
        )
        match outcome:
            case RealmRoleCertificate():
                return authenticated_cmds.latest.realm_share.RepOk()
            case RequireGreaterTimestamp() as error:
                return authenticated_cmds.latest.realm_share.RepRequireGreaterTimestamp(
                    strictly_greater_than=error.strictly_greater_than
                )
            case TimestampOutOfBallpark() as error:
                return authenticated_cmds.latest.realm_share.RepTimestampOutOfBallpark(
                    server_timestamp=error.server_timestamp,
                    client_timestamp=error.client_timestamp,
                    ballpark_client_early_offset=error.ballpark_client_early_offset,
                    ballpark_client_late_offset=error.ballpark_client_late_offset,
                )
            case RealmShareValidateBadOutcome():
                return authenticated_cmds.latest.realm_share.RepInvalidCertificate()
            case BadKeyIndex() as error:
                return authenticated_cmds.latest.realm_share.RepBadKeyIndex(
                    last_realm_certificate_timestamp=error.last_realm_certificate_timestamp,
                )
            case CertificateBasedActionIdempotentOutcome() as error:
                return authenticated_cmds.latest.realm_share.RepRoleAlreadyGranted(
                    last_realm_certificate_timestamp=error.certificate_timestamp,
                )
            case RealmShareStoreBadOutcome.REALM_NOT_FOUND:
                return authenticated_cmds.latest.realm_share.RepRealmNotFound()
            case RealmShareStoreBadOutcome.AUTHOR_NOT_ALLOWED:
                return authenticated_cmds.latest.realm_share.RepAuthorNotAllowed()
            case RealmShareStoreBadOutcome.RECIPIENT_NOT_FOUND:
                return authenticated_cmds.latest.realm_share.RepRecipientNotFound()
            case RealmShareStoreBadOutcome.RECIPIENT_REVOKED:
                return authenticated_cmds.latest.realm_share.RepRecipientRevoked()
            case RealmShareStoreBadOutcome.ROLE_INCOMPATIBLE_WITH_OUTSIDER:
                return authenticated_cmds.latest.realm_share.RepRoleIncompatibleWithOutsider()
            case RealmShareStoreBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case RealmShareStoreBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case RealmShareStoreBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case RealmShareStoreBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()

    @api
    async def api_realm_unshare(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.realm_unshare.Req,
    ) -> authenticated_cmds.latest.realm_unshare.Rep:
        outcome = await self.unshare(
            now=DateTime.now(),
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            author_verify_key=client_ctx.device_verify_key,
            realm_role_certificate=req.realm_role_certificate,
        )
        match outcome:
            case RealmRoleCertificate():
                return authenticated_cmds.latest.realm_unshare.RepOk()
            case RequireGreaterTimestamp() as error:
                return authenticated_cmds.latest.realm_unshare.RepRequireGreaterTimestamp(
                    strictly_greater_than=error.strictly_greater_than
                )
            case TimestampOutOfBallpark() as error:
                return authenticated_cmds.latest.realm_unshare.RepTimestampOutOfBallpark(
                    server_timestamp=error.server_timestamp,
                    client_timestamp=error.client_timestamp,
                    ballpark_client_early_offset=error.ballpark_client_early_offset,
                    ballpark_client_late_offset=error.ballpark_client_late_offset,
                )
            case RealmUnshareValidateBadOutcome():
                return authenticated_cmds.latest.realm_unshare.RepInvalidCertificate()
            case CertificateBasedActionIdempotentOutcome() as error:
                return authenticated_cmds.latest.realm_unshare.RepRecipientAlreadyUnshared(
                    last_realm_certificate_timestamp=error.certificate_timestamp,
                )
            case RealmUnshareStoreBadOutcome.REALM_NOT_FOUND:
                return authenticated_cmds.latest.realm_unshare.RepRealmNotFound()
            case RealmUnshareStoreBadOutcome.AUTHOR_NOT_ALLOWED:
                return authenticated_cmds.latest.realm_unshare.RepAuthorNotAllowed()
            case RealmUnshareStoreBadOutcome.RECIPIENT_NOT_FOUND:
                return authenticated_cmds.latest.realm_unshare.RepRecipientNotFound()
            case RealmUnshareStoreBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case RealmUnshareStoreBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case RealmUnshareStoreBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case RealmUnshareStoreBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()

    @api
    async def api_realm_rename(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.realm_rename.Req,
    ) -> authenticated_cmds.latest.realm_rename.Rep:
        outcome = await self.rename(
            now=DateTime.now(),
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            author_verify_key=client_ctx.device_verify_key,
            realm_name_certificate=req.realm_name_certificate,
            initial_name_or_fail=req.initial_name_or_fail,
        )
        match outcome:
            case RealmNameCertificate():
                return authenticated_cmds.latest.realm_rename.RepOk()
            case RequireGreaterTimestamp() as error:
                return authenticated_cmds.latest.realm_rename.RepRequireGreaterTimestamp(
                    strictly_greater_than=error.strictly_greater_than
                )
            case TimestampOutOfBallpark() as error:
                return authenticated_cmds.latest.realm_rename.RepTimestampOutOfBallpark(
                    server_timestamp=error.server_timestamp,
                    client_timestamp=error.client_timestamp,
                    ballpark_client_early_offset=error.ballpark_client_early_offset,
                    ballpark_client_late_offset=error.ballpark_client_late_offset,
                )
            case RealmRenameValidateBadOutcome():
                return authenticated_cmds.latest.realm_rename.RepInvalidCertificate()
            case BadKeyIndex() as error:
                return authenticated_cmds.latest.realm_rename.RepBadKeyIndex(
                    last_realm_certificate_timestamp=error.last_realm_certificate_timestamp,
                )
            case CertificateBasedActionIdempotentOutcome() as error:
                return authenticated_cmds.latest.realm_rename.RepInitialNameAlreadyExists(
                    last_realm_certificate_timestamp=error.certificate_timestamp
                )
            case RealmRenameStoreBadOutcome.REALM_NOT_FOUND:
                return authenticated_cmds.latest.realm_rename.RepRealmNotFound()
            case RealmRenameStoreBadOutcome.AUTHOR_NOT_ALLOWED:
                return authenticated_cmds.latest.realm_rename.RepAuthorNotAllowed()
            case RealmRenameStoreBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case RealmRenameStoreBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case RealmRenameStoreBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case RealmRenameStoreBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()

    @api
    async def api_realm_rotate_key(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.realm_rotate_key.Req,
    ) -> authenticated_cmds.latest.realm_rotate_key.Rep:
        outcome = await self.rotate_key(
            now=DateTime.now(),
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            author_verify_key=client_ctx.device_verify_key,
            realm_key_rotation_certificate=req.realm_key_rotation_certificate,
            per_participant_keys_bundle_access=req.per_participant_keys_bundle_access,
            keys_bundle=req.keys_bundle,
        )
        match outcome:
            case RealmKeyRotationCertificate():
                return authenticated_cmds.latest.realm_rotate_key.RepOk()
            case RequireGreaterTimestamp() as error:
                return authenticated_cmds.latest.realm_rotate_key.RepRequireGreaterTimestamp(
                    strictly_greater_than=error.strictly_greater_than
                )
            case TimestampOutOfBallpark() as error:
                return authenticated_cmds.latest.realm_rotate_key.RepTimestampOutOfBallpark(
                    server_timestamp=error.server_timestamp,
                    client_timestamp=error.client_timestamp,
                    ballpark_client_early_offset=error.ballpark_client_early_offset,
                    ballpark_client_late_offset=error.ballpark_client_late_offset,
                )
            case RealmRotateKeyValidateBadOutcome():
                return authenticated_cmds.latest.realm_rotate_key.RepInvalidCertificate()
            case BadKeyIndex() as error:
                return authenticated_cmds.latest.realm_rotate_key.RepBadKeyIndex(
                    last_realm_certificate_timestamp=error.last_realm_certificate_timestamp,
                )
            case RealmRotateKeyStoreBadOutcome.PARTICIPANT_MISMATCH:
                return authenticated_cmds.latest.realm_rotate_key.RepParticipantMismatch()
            case RealmRotateKeyStoreBadOutcome.REALM_NOT_FOUND:
                return authenticated_cmds.latest.realm_rotate_key.RepRealmNotFound()
            case RealmRotateKeyStoreBadOutcome.AUTHOR_NOT_ALLOWED:
                return authenticated_cmds.latest.realm_rotate_key.RepAuthorNotAllowed()
            case RealmRotateKeyStoreBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case RealmRotateKeyStoreBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case RealmRotateKeyStoreBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case RealmRotateKeyStoreBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()

    @api
    async def api_realm_get_keys_bundle(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.realm_get_keys_bundle.Req,
    ) -> authenticated_cmds.latest.realm_get_keys_bundle.Rep:
        outcome = await self.get_keys_bundle(
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            realm_id=req.realm_id,
            key_index=req.key_index,
        )
        match outcome:
            case KeysBundle() as keys_bundle:
                return authenticated_cmds.latest.realm_get_keys_bundle.RepOk(
                    keys_bundle_access=keys_bundle.keys_bundle_access,
                    keys_bundle=keys_bundle.keys_bundle,
                )
            case (
                RealmGetKeysBundleBadOutcome.AUTHOR_NOT_ALLOWED
                | RealmGetKeysBundleBadOutcome.REALM_NOT_FOUND
            ):
                return authenticated_cmds.latest.realm_get_keys_bundle.RepAuthorNotAllowed()
            case RealmGetKeysBundleBadOutcome.ACCESS_NOT_AVAILABLE_FOR_AUTHOR:
                return (
                    authenticated_cmds.latest.realm_get_keys_bundle.RepAccessNotAvailableForAuthor()
                )
            case RealmGetKeysBundleBadOutcome.BAD_KEY_INDEX:
                return authenticated_cmds.latest.realm_get_keys_bundle.RepBadKeyIndex()
            case RealmGetKeysBundleBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case RealmGetKeysBundleBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case RealmGetKeysBundleBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case RealmGetKeysBundleBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()
