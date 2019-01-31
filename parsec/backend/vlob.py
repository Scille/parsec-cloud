# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import List, Tuple
from uuid import UUID

from parsec.types import DeviceID, UserID, OrganizationID
from parsec.api.protocole import (
    vlob_group_check_serializer,
    vlob_create_serializer,
    vlob_read_serializer,
    vlob_update_serializer,
)
from parsec.backend.utils import catch_protocole_errors


class VlobError(Exception):
    pass


class VlobAccessError(VlobError):
    pass


class VlobVersionError(VlobError):
    pass


class VlobNotFoundError(VlobError):
    pass


class VlobAlreadyExistsError(VlobError):
    pass


class BaseVlobComponent:
    @catch_protocole_errors
    async def api_vlob_group_check(self, client_ctx, msg):
        msg = vlob_group_check_serializer.req_load(msg)
        changed = await self.group_check(
            client_ctx.organization_id, client_ctx.user_id, msg["to_check"]
        )
        return vlob_group_check_serializer.rep_dump({"status": "ok", "changed": changed})

    @catch_protocole_errors
    async def api_vlob_create(self, client_ctx, msg):
        msg = vlob_create_serializer.req_load(msg)
        try:
            await self.create(client_ctx.organization_id, client_ctx.device_id, **msg)

        except VlobAlreadyExistsError as exc:
            return vlob_create_serializer.rep_dump({"status": "already_exists", "reason": str(exc)})
        except VlobAccessError as exc:
            return vlob_create_serializer.rep_dump({"status": "not_allowed", "reason": str(exc)})

        return vlob_create_serializer.rep_dump({"status": "ok"})

    @catch_protocole_errors
    async def api_vlob_read(self, client_ctx, msg):
        msg = vlob_read_serializer.req_load(msg)

        try:
            version, blob = await self.read(client_ctx.organization_id, client_ctx.user_id, **msg)

        except VlobNotFoundError:
            return vlob_create_serializer.rep_dump({"status": "not_found"})

        except VlobAccessError:
            return vlob_create_serializer.rep_dump({"status": "not_allowed"})

        except VlobVersionError:
            return vlob_create_serializer.rep_dump({"status": "bad_version"})

        return vlob_read_serializer.rep_dump({"status": "ok", "blob": blob, "version": version})

    @catch_protocole_errors
    async def api_vlob_update(self, client_ctx, msg):
        msg = vlob_update_serializer.req_load(msg)

        try:
            await self.update(client_ctx.organization_id, client_ctx.device_id, **msg)

        except VlobNotFoundError:
            return vlob_create_serializer.rep_dump({"status": "not_found"})

        except VlobAccessError:
            return vlob_create_serializer.rep_dump({"status": "not_allowed"})

        except VlobVersionError:
            return vlob_create_serializer.rep_dump({"status": "bad_version"})

        return vlob_update_serializer.rep_dump({"status": "ok"})

    async def group_check(
        self, organization_id: OrganizationID, user_id: UserID, to_check: List[dict]
    ) -> List[dict]:
        """
        Raises:
            Nothing !
        """
        raise NotImplementedError()

    async def create(
        self, organization_id: OrganizationID, author: DeviceID, beacon: UUID, id: UUID, blob: bytes
    ) -> None:
        """
        Raises:
            VlobAlreadyExistsError
        """
        raise NotImplementedError()

    async def read(
        self, organization_id: OrganizationID, user_id: UserID, id: UUID, version: int = None
    ) -> Tuple[int, bytes]:
        """
        Raises:
            VlobAccessError
            VlobVersionError
            VlobNotFoundError
        """
        raise NotImplementedError()

    async def update(
        self, organization_id: OrganizationID, author: DeviceID, id: UUID, version: int, blob: bytes
    ) -> None:
        """
        Raises:
            VlobAccessError
            VlobVersionError
            VlobNotFoundError
        """
        raise NotImplementedError()
