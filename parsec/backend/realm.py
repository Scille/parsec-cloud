# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from typing import Any, Dict, List

import attr

from parsec._parsec import (
    DateTime,
    RealmArchivingCertificate,
    RealmArchivingConfiguration,
    RealmCreateRep,
    RealmCreateRepAlreadyExists,
    RealmCreateRepBadTimestamp,
    RealmCreateRepInvalidCertification,
    RealmCreateRepInvalidData,
    RealmCreateRepNotFound,
    RealmCreateRepOk,
    RealmCreateReq,
    RealmFinishReencryptionMaintenanceRep,
    RealmFinishReencryptionMaintenanceRepBadEncryptionRevision,
    RealmFinishReencryptionMaintenanceRepMaintenanceError,
    RealmFinishReencryptionMaintenanceRepNotAllowed,
    RealmFinishReencryptionMaintenanceRepNotFound,
    RealmFinishReencryptionMaintenanceRepNotInMaintenance,
    RealmFinishReencryptionMaintenanceRepOk,
    RealmFinishReencryptionMaintenanceRepRealmDeleted,
    RealmFinishReencryptionMaintenanceReq,
    RealmGetRoleCertificatesRep,
    RealmGetRoleCertificatesRepNotAllowed,
    RealmGetRoleCertificatesRepNotFound,
    RealmGetRoleCertificatesRepOk,
    RealmGetRoleCertificatesReq,
    RealmStartReencryptionMaintenanceRep,
    RealmStartReencryptionMaintenanceRepBadEncryptionRevision,
    RealmStartReencryptionMaintenanceRepBadTimestamp,
    RealmStartReencryptionMaintenanceRepInMaintenance,
    RealmStartReencryptionMaintenanceRepMaintenanceError,
    RealmStartReencryptionMaintenanceRepNotAllowed,
    RealmStartReencryptionMaintenanceRepNotFound,
    RealmStartReencryptionMaintenanceRepOk,
    RealmStartReencryptionMaintenanceRepParticipantMismatch,
    RealmStartReencryptionMaintenanceRepRealmDeleted,
    RealmStartReencryptionMaintenanceReq,
    RealmStatsRep,
    RealmStatsRepNotAllowed,
    RealmStatsRepNotFound,
    RealmStatsRepOk,
    RealmStatsRepRealmDeleted,
    RealmStatsReq,
    RealmStatusRep,
    RealmStatusRepNotAllowed,
    RealmStatusRepNotFound,
    RealmStatusRepOk,
    RealmStatusRepRealmDeleted,
    RealmStatusReq,
    RealmUpdateArchivingRep,
    RealmUpdateArchivingRepArchivingPeriodTooShort,
    RealmUpdateArchivingRepBadTimestamp,
    RealmUpdateArchivingRepInvalidCertification,
    RealmUpdateArchivingRepNotAllowed,
    RealmUpdateArchivingRepNotFound,
    RealmUpdateArchivingRepOk,
    RealmUpdateArchivingRepRealmDeleted,
    RealmUpdateArchivingRepRequireGreaterTimestamp,
    RealmUpdateArchivingReq,
    RealmUpdateRolesRep,
    RealmUpdateRolesRepAlreadyGranted,
    RealmUpdateRolesRepBadTimestamp,
    RealmUpdateRolesRepIncompatibleProfile,
    RealmUpdateRolesRepInMaintenance,
    RealmUpdateRolesRepInvalidCertification,
    RealmUpdateRolesRepInvalidData,
    RealmUpdateRolesRepNotAllowed,
    RealmUpdateRolesRepNotFound,
    RealmUpdateRolesRepOk,
    RealmUpdateRolesRepRealmDeleted,
    RealmUpdateRolesRepRequireGreaterTimestamp,
    RealmUpdateRolesRepUserRevoked,
    RealmUpdateRolesReq,
)
from parsec.api.data import DataError, RealmRoleCertificate
from parsec.api.protocol import (
    DeviceID,
    MaintenanceType,
    OrganizationID,
    RealmID,
    RealmRole,
    UserID,
    UserProfile,
)
from parsec.backend.client_context import AuthenticatedClientContext
from parsec.backend.user import UserAlreadyRevokedError
from parsec.backend.utils import api, api_typed_msg_adapter, catch_protocol_errors
from parsec.utils import (
    BALLPARK_CLIENT_EARLY_OFFSET,
    BALLPARK_CLIENT_LATE_OFFSET,
    timestamps_in_the_ballpark,
)


class RealmError(Exception):
    pass


class RealmAccessError(RealmError):
    pass


class RealmIncompatibleProfileError(RealmError):
    pass


class RealmNotFoundError(RealmError):
    pass


class RealmAlreadyExistsError(RealmError):
    pass


class RealmRoleAlreadyGrantedError(RealmError):
    pass


class RealmEncryptionRevisionError(RealmError):
    pass


class RealmParticipantsMismatchError(RealmError):
    pass


class RealmInMaintenanceError(RealmError):
    pass


class RealmNotInMaintenanceError(RealmError):
    pass


class RealmMaintenanceError(RealmError):
    pass


class RealmArchivedError(RealmError):
    pass


class RealmDeletedError(RealmError):
    pass


class RealmArchivingPeriodTooShortError(RealmError):
    pass


class RealmRoleRequireGreaterTimestampError(RealmError):
    @property
    def strictly_greater_than(self) -> DateTime:
        return self.args[0]


@attr.s(slots=True, frozen=True, auto_attribs=True)
class RealmStatus:
    maintenance_type: MaintenanceType | None
    maintenance_started_on: DateTime | None
    maintenance_started_by: DeviceID | None
    encryption_revision: int

    @property
    def in_maintenance(self) -> bool:
        return self.maintenance_type is not None

    @property
    def in_reencryption(self) -> bool:
        return self.maintenance_type == MaintenanceType.REENCRYPTION

    @property
    def in_garbage_collection(self) -> bool:
        return self.maintenance_type == MaintenanceType.GARBAGE_COLLECTION


@attr.s(slots=True, frozen=True, auto_attribs=True)
class RealmStats:
    blocks_size: int
    vlobs_size: int


@attr.s(slots=True, frozen=True, auto_attribs=True)
class RealmGrantedRole:
    def __repr__(self) -> str:
        return f"{self.__class__.__name__}({self.user_id.str} {self.role})"

    def evolve(self, **kwargs: Any) -> RealmGrantedRole:
        return attr.evolve(self, **kwargs)

    certificate: bytes
    realm_id: RealmID
    user_id: UserID
    role: RealmRole | None
    granted_by: DeviceID | None
    granted_on: DateTime


@attr.s(slots=True, frozen=True, auto_attribs=True)
class RealmConfiguredArchiving:
    def __repr__(self) -> str:
        return f"{self.__class__.__name__}({self.realm_id} {self.configuration})"

    def evolve(self, **kwargs: Any) -> RealmConfiguredArchiving:
        return attr.evolve(self, **kwargs)

    def is_valid_archiving_configuration(self, minimum_archiving_period: int) -> bool:
        if not self.configuration.is_deletion_planned():
            return True
        minimum_deletion_date = self.configured_on.add(seconds=minimum_archiving_period)
        return self.configuration.deletion_date >= minimum_deletion_date

    certificate: bytes
    realm_id: RealmID
    configuration: RealmArchivingConfiguration
    configured_by: DeviceID
    configured_on: DateTime


class BaseRealmComponent:
    @api("realm_create")
    @catch_protocol_errors
    @api_typed_msg_adapter(RealmCreateReq, RealmCreateRep)
    async def api_realm_create(
        self, client_ctx: AuthenticatedClientContext, req: RealmCreateReq
    ) -> RealmCreateRep:
        try:
            data = RealmRoleCertificate.verify_and_load(
                req.role_certificate,
                author_verify_key=client_ctx.verify_key,
                expected_author=client_ctx.device_id,
            )

        except DataError:
            return RealmCreateRepInvalidCertification(None)

        now = DateTime.now()
        if not timestamps_in_the_ballpark(data.timestamp, now):
            return RealmCreateRepBadTimestamp(
                reason=None,
                ballpark_client_early_offset=BALLPARK_CLIENT_EARLY_OFFSET,
                ballpark_client_late_offset=BALLPARK_CLIENT_LATE_OFFSET,
                backend_timestamp=now,
                client_timestamp=data.timestamp,
            )

        granted_role = RealmGrantedRole(
            certificate=req.role_certificate,
            realm_id=data.realm_id,
            user_id=data.user_id,
            role=data.role,
            granted_by=data.author,
            granted_on=data.timestamp,
        )
        if (
            granted_role.granted_by
            and granted_role.granted_by.user_id != granted_role.user_id
            or granted_role.role != RealmRole.OWNER
        ):
            return RealmCreateRepInvalidData(None)

        try:
            await self.create(client_ctx.organization_id, granted_role)

        except RealmNotFoundError:
            return RealmCreateRepNotFound(None)

        except RealmAlreadyExistsError:
            return RealmCreateRepAlreadyExists()

        return RealmCreateRepOk()

    @api("realm_status")
    @catch_protocol_errors
    @api_typed_msg_adapter(RealmStatusReq, RealmStatusRep)
    async def api_realm_status(
        self, client_ctx: AuthenticatedClientContext, req: RealmStatusReq
    ) -> RealmStatusRep:
        now = DateTime.now()
        try:
            status = await self.get_status(
                client_ctx.organization_id, client_ctx.device_id, req.realm_id, now
            )

        except RealmAccessError:
            return RealmStatusRepNotAllowed()

        except RealmNotFoundError:
            return RealmStatusRepNotFound(None)

        except RealmDeletedError:
            return RealmStatusRepRealmDeleted()

        return RealmStatusRepOk(
            in_maintenance=status.in_maintenance,
            maintenance_type=status.maintenance_type,
            maintenance_started_on=status.maintenance_started_on,
            maintenance_started_by=status.maintenance_started_by,
            encryption_revision=status.encryption_revision,
        )

    @api("realm_stats")
    @catch_protocol_errors
    @api_typed_msg_adapter(RealmStatsReq, RealmStatsRep)
    async def api_realm_stats(
        self, client_ctx: AuthenticatedClientContext, req: RealmStatsReq
    ) -> RealmStatsRep:
        now = DateTime.now()
        try:
            stats = await self.get_stats(
                client_ctx.organization_id, client_ctx.device_id, req.realm_id, now
            )
        except RealmAccessError:
            return RealmStatsRepNotAllowed()
        except RealmNotFoundError:
            return RealmStatsRepNotFound(None)
        except RealmDeletedError:
            return RealmStatsRepRealmDeleted()
        return RealmStatsRepOk(blocks_size=stats.blocks_size, vlobs_size=stats.vlobs_size)

    @api("realm_get_role_certificates")
    @catch_protocol_errors
    @api_typed_msg_adapter(RealmGetRoleCertificatesReq, RealmGetRoleCertificatesRep)
    async def api_realm_get_role_certificates(
        self, client_ctx: AuthenticatedClientContext, req: RealmGetRoleCertificatesReq
    ) -> RealmGetRoleCertificatesRep:
        now = DateTime.now()
        try:
            certificates = await self.get_role_certificates(
                client_ctx.organization_id, client_ctx.device_id, req.realm_id, now
            )

        except RealmAccessError:
            return RealmGetRoleCertificatesRepNotAllowed()

        except RealmNotFoundError:
            return RealmGetRoleCertificatesRepNotFound(None)

        return RealmGetRoleCertificatesRepOk(certificates)

    @api("realm_update_roles")
    @catch_protocol_errors
    @api_typed_msg_adapter(RealmUpdateRolesReq, RealmUpdateRolesRep)
    async def api_realm_update_roles(
        self, client_ctx: AuthenticatedClientContext, req: RealmUpdateRolesReq
    ) -> RealmUpdateRolesRep:
        """
        This API call, when successful, performs the writing of a new role certificate to the database.
        Before adding new entries, extra care should be taken in order to guarantee the consistency in
        the ordering of the different timestamps stored in the database.

        In particular, the backend server performs the following checks:
        - The certificate must have a timestamp strictly greater than the last certificate for
          the same user in the same realm.
        - If the certificate corresponds to a role without write rights, its timestamp should
          be strictly greater than the timestamp of the last vlob update performed by the
          corresponding user in the corresponding realm.
        - If the certificate corresponds to a role without management rights, its timestamp should
          be strictly greater than the timestamp of the last role certificate uploaded by the
          corresponding user in the corresponding realm.
        - If the certificate revokes some owner rights, its timestamp should be strictly greater than
          the timestamp of the last archiving certificate uploaded by the corresponding user in the
          corresponding realm.

        If one of those constraints is not satisfied, an error is returned with the status
        `require_greater_timestamp` indicating to the client that it should craft a new certificate
        with a timestamp strictly greater than the timestamp provided with the error.

        The `api_vlob_create` and `api_vlob_update` calls also perform similar checks.
        """
        # An OUTSIDER is allowed to create a realm (given he needs to have one
        # to store it user manifest). However he cannot be MANAGER or OWNER in
        # a shared realm as well.
        # Hence the only way for him to be OWNER is to create a realm, and in
        # this case he cannot share this realm with anyone.
        # On top of that, we don't have to fetch the user profile from the
        # database before checking it given it cannot be updated.
        if client_ctx.profile == UserProfile.OUTSIDER:
            return RealmUpdateRolesRepNotAllowed(None)

        try:
            data = RealmRoleCertificate.verify_and_load(
                req.role_certificate,
                author_verify_key=client_ctx.verify_key,
                expected_author=client_ctx.device_id,
            )

        except DataError:
            return RealmUpdateRolesRepInvalidCertification(None)

        now = DateTime.now()
        if not timestamps_in_the_ballpark(data.timestamp, now):
            return RealmUpdateRolesRepBadTimestamp(
                reason=None,
                ballpark_client_early_offset=BALLPARK_CLIENT_EARLY_OFFSET,
                ballpark_client_late_offset=BALLPARK_CLIENT_LATE_OFFSET,
                backend_timestamp=now,
                client_timestamp=data.timestamp,
            )

        granted_role = RealmGrantedRole(
            certificate=req.role_certificate,
            realm_id=data.realm_id,
            user_id=data.user_id,
            role=data.role,
            granted_by=data.author,
            granted_on=data.timestamp,
        )
        if granted_role.granted_by and granted_role.granted_by.user_id == granted_role.user_id:
            return RealmUpdateRolesRepInvalidData(None)

        try:
            await self.update_roles(
                client_ctx.organization_id,
                granted_role,
                req.recipient_message,
                now,
            )

        except UserAlreadyRevokedError:
            return RealmUpdateRolesRepUserRevoked()

        except RealmRoleAlreadyGrantedError:
            return RealmUpdateRolesRepAlreadyGranted()

        except RealmAccessError:
            return RealmUpdateRolesRepNotAllowed(None)

        except RealmRoleRequireGreaterTimestampError as exc:
            return RealmUpdateRolesRepRequireGreaterTimestamp(exc.strictly_greater_than)

        except RealmIncompatibleProfileError:
            return RealmUpdateRolesRepIncompatibleProfile(None)

        except RealmNotFoundError:
            return RealmUpdateRolesRepNotFound(None)

        except RealmInMaintenanceError:
            return RealmUpdateRolesRepInMaintenance()

        except RealmDeletedError:
            return RealmUpdateRolesRepRealmDeleted()

        return RealmUpdateRolesRepOk()

    @api("realm_update_archiving")
    @catch_protocol_errors
    @api_typed_msg_adapter(RealmUpdateArchivingReq, RealmUpdateArchivingRep)
    async def api_realm_update_archiving(
        self, client_ctx: AuthenticatedClientContext, req: RealmUpdateArchivingReq
    ) -> RealmUpdateArchivingRep:
        """
        This API call, when successful, performs the writing of a new archiving configuration certificate
        to the database.

        The backend server performs the following checks:
        - The certificate must have a timestamp strictly greater than the last archiving certificate for
          this realm.
        - The certificate must have a timestamp strictly greater than the last role certificate in this
          realm for this author.

        If one of those constraints is not satisfied, an error is returned with the status
        `require_greater_timestamp` indicating to the client that it should craft a new certificate
        with a timestamp strictly greater than the timestamp provided with the error.

        /!\\ A note on `RealmArchivingConfiguration::DeletionPlanned { deletion_date }`:
        The deletion date provided when planning a deletion is **not** checked for causality. Instead:
        - For the backend: it means that it no longer has to accept requests after this date is met.
        - For the client: it means that it should not expect requests to succeed after this date is met.
        In particular, vlobs or role certificates with timestamp after the deletion date might be accepted
        by the backend if they happen to be stamped a little bit in the future (since the backend typically
        use its own current time to check for deletion).
        """
        try:
            data = RealmArchivingCertificate.verify_and_load(
                req.archiving_certificate,
                author_verify_key=client_ctx.verify_key,
                expected_author=client_ctx.device_id,
            )

        except DataError:
            return RealmUpdateArchivingRepInvalidCertification()

        now = DateTime.now()
        if not timestamps_in_the_ballpark(data.timestamp, now):
            return RealmUpdateArchivingRepBadTimestamp(
                ballpark_client_early_offset=BALLPARK_CLIENT_EARLY_OFFSET,
                ballpark_client_late_offset=BALLPARK_CLIENT_LATE_OFFSET,
                backend_timestamp=now,
                client_timestamp=data.timestamp,
            )

        archiving_configuration_request = RealmConfiguredArchiving(
            certificate=req.archiving_certificate,
            realm_id=data.realm_id,
            configuration=data.configuration,
            configured_by=data.author,
            configured_on=data.timestamp,
        )

        try:
            await self.update_archiving(
                client_ctx.organization_id, archiving_configuration_request, now
            )

        except RealmAccessError:
            return RealmUpdateArchivingRepNotAllowed()

        except RealmRoleRequireGreaterTimestampError as exc:
            return RealmUpdateArchivingRepRequireGreaterTimestamp(exc.strictly_greater_than)

        except RealmNotFoundError:
            return RealmUpdateArchivingRepNotFound()

        except RealmDeletedError:
            return RealmUpdateArchivingRepRealmDeleted()

        except RealmArchivingPeriodTooShortError:
            return RealmUpdateArchivingRepArchivingPeriodTooShort()

        return RealmUpdateArchivingRepOk()

    @api("realm_start_reencryption_maintenance")
    @catch_protocol_errors
    @api_typed_msg_adapter(
        RealmStartReencryptionMaintenanceReq, RealmStartReencryptionMaintenanceRep
    )
    async def api_realm_start_reencryption_maintenance(
        self, client_ctx: AuthenticatedClientContext, req: RealmStartReencryptionMaintenanceReq
    ) -> RealmStartReencryptionMaintenanceRep:
        now = DateTime.now()
        if not timestamps_in_the_ballpark(req.timestamp, now):
            return RealmStartReencryptionMaintenanceRepBadTimestamp(
                reason=None,
                ballpark_client_early_offset=BALLPARK_CLIENT_EARLY_OFFSET,
                ballpark_client_late_offset=BALLPARK_CLIENT_LATE_OFFSET,
                backend_timestamp=now,
                client_timestamp=req.timestamp,
            )

        try:
            await self.start_reencryption_maintenance(
                client_ctx.organization_id,
                client_ctx.device_id,
                realm_id=req.realm_id,
                timestamp=req.timestamp,
                encryption_revision=req.encryption_revision,
                per_participant_message=req.per_participant_message,
                now=now,
            )

        except RealmAccessError:
            return RealmStartReencryptionMaintenanceRepNotAllowed()

        except RealmNotFoundError:
            return RealmStartReencryptionMaintenanceRepNotFound(None)

        except RealmEncryptionRevisionError:
            return RealmStartReencryptionMaintenanceRepBadEncryptionRevision()

        except RealmParticipantsMismatchError:
            return RealmStartReencryptionMaintenanceRepParticipantMismatch(None)

        except RealmMaintenanceError:
            return RealmStartReencryptionMaintenanceRepMaintenanceError(None)

        except RealmInMaintenanceError:
            return RealmStartReencryptionMaintenanceRepInMaintenance()

        except RealmDeletedError:
            return RealmStartReencryptionMaintenanceRepRealmDeleted()

        return RealmStartReencryptionMaintenanceRepOk()

    @api("realm_finish_reencryption_maintenance")
    @catch_protocol_errors
    @api_typed_msg_adapter(
        RealmFinishReencryptionMaintenanceReq, RealmFinishReencryptionMaintenanceRep
    )
    async def api_realm_finish_reencryption_maintenance(
        self, client_ctx: AuthenticatedClientContext, req: RealmFinishReencryptionMaintenanceReq
    ) -> RealmFinishReencryptionMaintenanceRep:
        now = DateTime.now()
        try:
            await self.finish_reencryption_maintenance(
                client_ctx.organization_id,
                client_ctx.device_id,
                realm_id=req.realm_id,
                encryption_revision=req.encryption_revision,
                now=now,
            )

        except RealmAccessError:
            return RealmFinishReencryptionMaintenanceRepNotAllowed()

        except RealmNotFoundError:
            return RealmFinishReencryptionMaintenanceRepNotFound(None)

        except RealmEncryptionRevisionError:
            return RealmFinishReencryptionMaintenanceRepBadEncryptionRevision()

        except RealmNotInMaintenanceError:
            return RealmFinishReencryptionMaintenanceRepNotInMaintenance(None)

        except RealmMaintenanceError:
            return RealmFinishReencryptionMaintenanceRepMaintenanceError(None)

        except RealmDeletedError:
            return RealmFinishReencryptionMaintenanceRepRealmDeleted()

        return RealmFinishReencryptionMaintenanceRepOk()

    async def create(
        self,
        organization_id: OrganizationID,
        self_granted_role: RealmGrantedRole,
    ) -> None:
        """
        Raises:
            RealmNotFoundError
            RealmAccessError
            RealmAlreadyExistsError
        """
        raise NotImplementedError()

    async def get_status(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: RealmID,
        now: DateTime,
    ) -> RealmStatus:
        """
        Raises:
            RealmNotFoundError
            RealmAccessError
            RealmDeletedError
        """
        raise NotImplementedError()

    async def get_stats(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: RealmID,
        now: DateTime,
    ) -> RealmStats:
        """
        Raises:
            RealmNotFoundError
            RealmAccessError
            RealmDeletedError
        """
        raise NotImplementedError()

    async def get_current_roles(
        self,
        organization_id: OrganizationID,
        realm_id: RealmID,
        now: DateTime,
    ) -> Dict[UserID, RealmRole]:
        """
        Raises:
            RealmNotFoundError
        """
        raise NotImplementedError()

    async def get_role_certificates(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: RealmID,
        now: DateTime,
    ) -> List[bytes]:
        """
        Raises:
            RealmNotFoundError
            RealmAccessError
        """
        raise NotImplementedError()

    async def update_roles(
        self,
        organization_id: OrganizationID,
        new_role: RealmGrantedRole,
        recipient_message: bytes | None,
        now: DateTime,
    ) -> None:
        """
        Raises:
            UserAlreadyRevokedError
            RealmRoleAlreadyGrantedError
            RealmAccessError
            RealmRoleRequireGreaterTimestampError
            RealmIncompatibleProfileError
            RealmNotFoundError
            RealmInMaintenanceError
            RealmDeletedError
        """
        raise NotImplementedError()

    async def update_archiving(
        self,
        organization_id: OrganizationID,
        archiving_certificate: RealmConfiguredArchiving,
        now: DateTime,
    ) -> None:
        """
        Raises:
            RealmAccessError
            RealmRoleRequireGreaterTimestampError
            RealmNotFoundError
            RealmDeletedError
            RealmArchivingPeriodTooShortError
            RealmDeletedError
        """
        raise NotImplementedError()

    async def start_reencryption_maintenance(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: RealmID,
        encryption_revision: int,
        per_participant_message: Dict[UserID, bytes],
        timestamp: DateTime,
        now: DateTime,
    ) -> None:
        """
        Raises:
            RealmInMaintenanceError
            RealmMaintenanceError: bad encryption_revision or per_participant_message
            RealmNotFoundError
            RealmAccessError
            RealmDeletedError
        """
        raise NotImplementedError()

    async def finish_reencryption_maintenance(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: RealmID,
        encryption_revision: int,
        now: DateTime,
    ) -> None:
        """
        Raises:
            RealmNotFoundError
            RealmAccessError
            RealmMaintenanceError: not in maintenance or bad encryption_revision
            RealmDeletedError
        """
        raise NotImplementedError()

    async def get_realms_for_user(
        self,
        organization_id: OrganizationID,
        user: UserID,
    ) -> Dict[RealmID, RealmRole]:
        """
        Raises: Nothing !
        """
        raise NotImplementedError()

    async def dump_realms_granted_roles(
        self, organization_id: OrganizationID
    ) -> List[RealmGrantedRole]:
        """
        Raises: Nothing !
        """
        raise NotImplementedError
