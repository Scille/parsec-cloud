# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from typing import Dict, List, Optional
from parsec._parsec import DateTime
import attr
from parsec.backend.user import UserAlreadyRevokedError

from parsec.utils import timestamps_in_the_ballpark
from parsec.api.data import DataError, RealmRoleCertificate
from parsec.api.protocol import (
    OrganizationID,
    UserID,
    DeviceID,
    RealmID,
    RealmRole,
    MaintenanceType,
    UserProfile,
    realm_status_serializer,
    realm_stats_serializer,
    realm_create_serializer,
    realm_get_role_certificates_serializer,
    realm_update_roles_serializer,
    realm_start_reencryption_maintenance_serializer,
    realm_finish_reencryption_maintenance_serializer,
)
from parsec.backend.utils import catch_protocol_errors, api


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


class RealmRoleAlreadyGranted(RealmError):
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


class RealmRoleRequireGreaterTimestampError(RealmError):
    @property
    def strictly_greater_than(self):
        return self.args[0]


@attr.s(slots=True, frozen=True, auto_attribs=True)
class RealmStatus:
    maintenance_type: Optional[MaintenanceType]
    maintenance_started_on: Optional[DateTime]
    maintenance_started_by: Optional[DeviceID]
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
    def __repr__(self):
        return f"{self.__class__.__name__}({self.user_id.str} {self.role})"

    def evolve(self, **kwargs) -> "RealmGrantedRole":
        return attr.evolve(self, **kwargs)

    certificate: bytes
    realm_id: RealmID
    user_id: UserID
    role: Optional[RealmRole]
    granted_by: Optional[DeviceID]
    granted_on: DateTime


class BaseRealmComponent:
    @api("realm_create")
    @catch_protocol_errors
    async def api_realm_create(self, client_ctx, msg):
        msg = realm_create_serializer.req_load(msg)

        try:
            data = RealmRoleCertificate.verify_and_load(
                msg["role_certificate"],
                author_verify_key=client_ctx.verify_key,
                expected_author=client_ctx.device_id,
            )

        except DataError as exc:
            return {
                "status": "invalid_certification",
                "reason": f"Invalid certification data ({exc}).",
            }

        now = DateTime.now()
        if not timestamps_in_the_ballpark(data.timestamp, now):
            return realm_create_serializer.timestamp_out_of_ballpark_rep_dump(
                backend_timestamp=now, client_timestamp=data.timestamp
            )

        granted_role = RealmGrantedRole(
            certificate=msg["role_certificate"],
            realm_id=data.realm_id,
            user_id=data.user_id,
            role=data.role,
            granted_by=data.author,
            granted_on=data.timestamp,
        )
        if granted_role.granted_by.user_id != granted_role.user_id:
            return {
                "status": "invalid_data",
                "reason": f"Initial realm role certificate must be self-signed.",
            }
        if granted_role.role != RealmRole.OWNER:
            return {
                "status": "invalid_data",
                "reason": f"Initial realm role certificate must set OWNER role.",
            }

        try:
            await self.create(client_ctx.organization_id, granted_role)

        except RealmNotFoundError as exc:
            return realm_create_serializer.rep_dump({"status": "not_found", "reason": str(exc)})

        except RealmAlreadyExistsError:
            return realm_create_serializer.rep_dump({"status": "already_exists"})

        return realm_create_serializer.rep_dump({"status": "ok"})

    @api("realm_status")
    @catch_protocol_errors
    async def api_realm_status(self, client_ctx, msg):
        msg = realm_status_serializer.req_load(msg)

        try:
            status = await self.get_status(
                client_ctx.organization_id, client_ctx.device_id, msg["realm_id"]
            )

        except RealmAccessError:
            return realm_status_serializer.rep_dump({"status": "not_allowed"})

        except RealmNotFoundError as exc:
            return realm_status_serializer.rep_dump({"status": "not_found", "reason": str(exc)})

        return realm_status_serializer.rep_dump(
            {
                "status": "ok",
                "in_maintenance": status.in_maintenance,
                "maintenance_type": status.maintenance_type,
                "maintenance_started_on": status.maintenance_started_on,
                "maintenance_started_by": status.maintenance_started_by,
                "encryption_revision": status.encryption_revision,
            }
        )

    @api("realm_stats")
    @catch_protocol_errors
    async def api_realm_stats(self, client_ctx, msg):
        msg = realm_stats_serializer.req_load(msg)
        try:
            stats = await self.get_stats(
                client_ctx.organization_id, client_ctx.device_id, msg["realm_id"]
            )
        except RealmAccessError:
            return realm_status_serializer.rep_dump({"status": "not_allowed"})
        except RealmNotFoundError as exc:
            return realm_status_serializer.rep_dump({"status": "not_found", "reason": str(exc)})
        return realm_stats_serializer.rep_dump(
            {"status": "ok", "blocks_size": stats.blocks_size, "vlobs_size": stats.vlobs_size}
        )

    @api("realm_get_role_certificates")
    @catch_protocol_errors
    async def api_realm_get_role_certificates(self, client_ctx, msg):
        msg = realm_get_role_certificates_serializer.req_load(msg)
        try:
            certificates = await self.get_role_certificates(
                client_ctx.organization_id, client_ctx.device_id, **msg
            )

        except RealmAccessError:
            return realm_get_role_certificates_serializer.rep_dump({"status": "not_allowed"})

        except RealmNotFoundError as exc:
            return realm_get_role_certificates_serializer.rep_dump(
                {"status": "not_found", "reason": str(exc)}
            )

        return realm_get_role_certificates_serializer.rep_dump(
            {"status": "ok", "certificates": certificates}
        )

    @api("realm_update_roles")
    @catch_protocol_errors
    async def api_realm_update_roles(self, client_ctx, msg):
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
            return {"status": "not_allowed", "reason": "Outsider user cannot share realm"}

        msg = realm_update_roles_serializer.req_load(msg)

        try:
            data = RealmRoleCertificate.verify_and_load(
                msg["role_certificate"],
                author_verify_key=client_ctx.verify_key,
                expected_author=client_ctx.device_id,
            )

        except DataError as exc:
            return {
                "status": "invalid_certification",
                "reason": f"Invalid certification data ({exc}).",
            }

        now = DateTime.now()
        if not timestamps_in_the_ballpark(data.timestamp, now):
            return realm_update_roles_serializer.timestamp_out_of_ballpark_rep_dump(
                backend_timestamp=now, client_timestamp=data.timestamp
            )

        granted_role = RealmGrantedRole(
            certificate=msg["role_certificate"],
            realm_id=data.realm_id,
            user_id=data.user_id,
            role=data.role,
            granted_by=data.author,
            granted_on=data.timestamp,
        )
        if granted_role.granted_by.user_id == granted_role.user_id:
            return {
                "status": "invalid_data",
                "reason": f"Realm role certificate cannot be self-signed.",
            }

        try:
            await self.update_roles(
                client_ctx.organization_id, granted_role, msg["recipient_message"]
            )

        except UserAlreadyRevokedError:
            return realm_update_roles_serializer.rep_dump({"status": "user_revoked"})

        except RealmRoleAlreadyGranted:
            return realm_update_roles_serializer.rep_dump({"status": "already_granted"})

        except RealmAccessError:
            return realm_update_roles_serializer.rep_dump({"status": "not_allowed"})

        except RealmRoleRequireGreaterTimestampError as exc:
            return realm_update_roles_serializer.require_greater_timestamp_rep_dump(
                exc.strictly_greater_than
            )

        except RealmIncompatibleProfileError as exc:
            return realm_update_roles_serializer.rep_dump(
                {"status": "incompatible_profile", "reason": str(exc)}
            )

        except RealmNotFoundError as exc:
            return realm_update_roles_serializer.rep_dump(
                {"status": "not_found", "reason": str(exc)}
            )

        except RealmInMaintenanceError:
            return realm_update_roles_serializer.rep_dump({"status": "in_maintenance"})

        return realm_update_roles_serializer.rep_dump({"status": "ok"})

    @api("realm_start_reencryption_maintenance")
    @catch_protocol_errors
    async def api_realm_start_reencryption_maintenance(self, client_ctx, msg):
        msg = realm_start_reencryption_maintenance_serializer.req_load(msg)

        now = DateTime.now()
        if not timestamps_in_the_ballpark(msg["timestamp"], now):
            return (
                realm_start_reencryption_maintenance_serializer.timestamp_out_of_ballpark_rep_dump(
                    backend_timestamp=now, client_timestamp=msg["timestamp"]
                )
            )

        try:
            await self.start_reencryption_maintenance(
                client_ctx.organization_id, client_ctx.device_id, **msg
            )

        except RealmAccessError:
            return realm_start_reencryption_maintenance_serializer.rep_dump(
                {"status": "not_allowed"}
            )

        except RealmNotFoundError as exc:
            return realm_start_reencryption_maintenance_serializer.rep_dump(
                {"status": "not_found", "reason": str(exc)}
            )

        except RealmEncryptionRevisionError:
            return realm_start_reencryption_maintenance_serializer.rep_dump(
                {"status": "bad_encryption_revision"}
            )

        except RealmParticipantsMismatchError as exc:
            return realm_start_reencryption_maintenance_serializer.rep_dump(
                {"status": "participants_mismatch", "reason": str(exc)}
            )

        except RealmMaintenanceError as exc:
            return realm_start_reencryption_maintenance_serializer.rep_dump(
                {"status": "maintenance_error", "reason": str(exc)}
            )

        except RealmInMaintenanceError:
            return realm_start_reencryption_maintenance_serializer.rep_dump(
                {"status": "in_maintenance"}
            )

        return realm_start_reencryption_maintenance_serializer.rep_dump({"status": "ok"})

    @api("realm_finish_reencryption_maintenance")
    @catch_protocol_errors
    async def api_realm_finish_reencryption_maintenance(self, client_ctx, msg):
        msg = realm_finish_reencryption_maintenance_serializer.req_load(msg)

        try:
            await self.finish_reencryption_maintenance(
                client_ctx.organization_id, client_ctx.device_id, **msg
            )

        except RealmAccessError:
            return realm_finish_reencryption_maintenance_serializer.rep_dump(
                {"status": "not_allowed"}
            )

        except RealmNotFoundError as exc:
            return realm_finish_reencryption_maintenance_serializer.rep_dump(
                {"status": "not_found", "reason": str(exc)}
            )

        except RealmEncryptionRevisionError:
            return realm_finish_reencryption_maintenance_serializer.rep_dump(
                {"status": "bad_encryption_revision"}
            )

        except RealmNotInMaintenanceError as exc:
            return realm_finish_reencryption_maintenance_serializer.rep_dump(
                {"status": "not_in_maintenance", "reason": str(exc)}
            )

        except RealmMaintenanceError as exc:
            return realm_finish_reencryption_maintenance_serializer.rep_dump(
                {"status": "maintenance_error", "reason": str(exc)}
            )

        return realm_finish_reencryption_maintenance_serializer.rep_dump({"status": "ok"})

    async def create(
        self, organization_id: OrganizationID, self_granted_role: RealmGrantedRole
    ) -> None:
        """
        Raises:
            RealmNotFoundError
            RealmAccessError
            RealmAlreadyExistsError
        """
        raise NotImplementedError()

    async def get_status(
        self, organization_id: OrganizationID, author: DeviceID, realm_id: RealmID
    ) -> RealmStatus:
        """
        Raises:
            RealmNotFoundError
            RealmAccessError
        """
        raise NotImplementedError()

    async def get_stats(
        self, organization_id: OrganizationID, author: DeviceID, realm_id: RealmID
    ) -> RealmStats:
        """
        Raises:
            RealmNotFoundError
            RealmAccessError
        """
        raise NotImplementedError()

    async def get_current_roles(
        self, organization_id: OrganizationID, realm_id: RealmID
    ) -> Dict[UserID, RealmRole]:
        """
        Raises:
            RealmNotFoundError
        """
        raise NotImplementedError()

    async def get_role_certificates(
        self, organization_id: OrganizationID, author: DeviceID, realm_id: RealmID
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
        recipient_message: Optional[bytes] = None,
    ) -> None:
        """
        Raises:
            RealmInMaintenanceError
            RealmNotFoundError
            RealmAccessError
            RealmIncompatibleProfileError
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
    ) -> None:
        """
        Raises:
            RealmInMaintenanceError
            RealmMaintenanceError: bad encryption_revision or per_participant_message
            RealmNotFoundError
            RealmAccessError
        """
        raise NotImplementedError()

    async def finish_reencryption_maintenance(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: RealmID,
        encryption_revision: int,
    ) -> None:
        """
        Raises:
            RealmNotFoundError
            RealmAccessError
            RealmMaintenanceError: not in maintenance or bad encryption_revision
        """
        raise NotImplementedError()

    async def get_realms_for_user(
        self, organization_id: OrganizationID, user: UserID
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
