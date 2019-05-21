# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import Dict, Optional
from uuid import UUID
import pendulum
import attr

from parsec.types import DeviceID, UserID, OrganizationID
from parsec.api.protocole import (
    RealmRole,
    MaintenanceType,
    realm_status_serializer,
    realm_get_roles_serializer,
    realm_update_roles_serializer,
    realm_start_reencryption_maintenance_serializer,
    realm_finish_reencryption_maintenance_serializer,
)
from parsec.backend.utils import catch_protocole_errors


class RealmError(Exception):
    pass


class RealmAccessError(RealmError):
    pass


class RealmNotFoundError(RealmError):
    pass


class RealmAlreadyExistsError(RealmError):
    pass


class RealmInMaintenanceError(RealmError):
    pass


class RealmMaintenanceError(RealmError):
    pass


@attr.s(slots=True, frozen=True, auto_attribs=True)
class RealmStatus:
    in_maintenance: bool
    maintenance_type: MaintenanceType
    maintenance_started_on: Optional[DeviceID]
    maintenance_started_by: Optional[pendulum.Pendulum]
    encryption_revision: int


class BaseRealmComponent:
    @catch_protocole_errors
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

    @catch_protocole_errors
    async def api_realm_get_roles(self, client_ctx, msg):
        msg = realm_get_roles_serializer.req_load(msg)

        try:
            roles = await self.get_roles(client_ctx.organization_id, client_ctx.device_id, **msg)

        except RealmAccessError:
            return realm_get_roles_serializer.rep_dump({"status": "not_allowed"})

        except RealmNotFoundError as exc:
            return realm_get_roles_serializer.rep_dump({"status": "not_found", "reason": str(exc)})

        return realm_get_roles_serializer.rep_dump({"status": "ok", "users": roles})

    @catch_protocole_errors
    async def api_realm_update_roles(self, client_ctx, msg):
        msg = realm_update_roles_serializer.req_load(msg)

        try:
            await self.update_roles(client_ctx.organization_id, client_ctx.device_id, **msg)

        except RealmAccessError:
            return realm_update_roles_serializer.rep_dump({"status": "not_allowed"})

        except RealmNotFoundError as exc:
            return realm_update_roles_serializer.rep_dump(
                {"status": "not_found", "reason": str(exc)}
            )

        except RealmInMaintenanceError:
            return realm_update_roles_serializer.rep_dump({"status": "in_maintenance"})

        return realm_update_roles_serializer.rep_dump({"status": "ok"})

    @catch_protocole_errors
    async def api_realm_start_reencryption_maintenance(self, client_ctx, msg):
        msg = realm_start_reencryption_maintenance_serializer.req_load(msg)

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

        except RealmMaintenanceError as exc:
            return realm_finish_reencryption_maintenance_serializer.rep_dump(
                {"status": "maintenance_error", "reason": str(exc)}
            )

        except RealmInMaintenanceError:
            return realm_update_roles_serializer.rep_dump({"status": "in_maintenance"})

        return realm_start_reencryption_maintenance_serializer.rep_dump({"status": "ok"})

    @catch_protocole_errors
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

        except RealmMaintenanceError as exc:
            return realm_finish_reencryption_maintenance_serializer.rep_dump(
                {"status": "maintenance_error", "reason": str(exc)}
            )

        return realm_finish_reencryption_maintenance_serializer.rep_dump({"status": "ok"})

    async def get_status(
        self, organization_id: OrganizationID, author: DeviceID, realm_id: UUID
    ) -> RealmStatus:
        """
        Raises:
            RealmNotFoundError
            RealmAccessError
        """
        raise NotImplementedError()

    async def get_roles(
        self, organization_id: OrganizationID, author: DeviceID, realm_id: UUID
    ) -> Dict[UserID, RealmRole]:
        """
        Raises:
            RealmNotFoundError
            RealmAccessError
        """
        raise NotImplementedError()

    async def update_roles(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: UUID,
        user: UserID,
        role: Optional[RealmRole],
    ) -> None:
        """
        Raises:
            RealmInMaintenanceError
            RealmNotFoundError
            RealmAccessError
        """
        raise NotImplementedError()

    async def start_reencryption_maintenance(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: UUID,
        encryption_revision: int,
        per_participant_message: Dict[UserID, bytes],
    ) -> None:
        """
        Raises:
            RealmInMaintenanceError
            RealmMaintenanceError: bad encryption_revision or per_participant_message
            RealmNotFoundError
            RealmAccessError
        """
        pass

    async def finish_reencryption_maintenance(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: UUID,
        encryption_revision: int,
    ) -> None:
        """
        Raises:
            RealmNotFoundError
            RealmAccessError
            RealmMaintenanceError: not in maintenance or bad encryption_revision
        """
        pass
