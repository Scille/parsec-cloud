# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import List, Tuple, Dict
from uuid import UUID

from parsec.types import DeviceID, UserID, OrganizationID
from parsec.api.protocole import (
    vlob_group_check_serializer,
    vlob_create_serializer,
    vlob_read_serializer,
    vlob_update_serializer,
    vlob_group_update_rights_serializer,
    vlob_group_get_rights_serializer,
    vlob_group_poll_serializer,
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

        except VlobNotFoundError as exc:
            return vlob_create_serializer.rep_dump({"status": "not_found", "reason": str(exc)})

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

        except VlobNotFoundError as exc:
            return vlob_create_serializer.rep_dump({"status": "not_found", "reason": str(exc)})

        except VlobAccessError:
            return vlob_create_serializer.rep_dump({"status": "not_allowed"})

        except VlobVersionError:
            return vlob_create_serializer.rep_dump({"status": "bad_version"})

        return vlob_update_serializer.rep_dump({"status": "ok"})

    @catch_protocole_errors
    async def api_vlob_group_get_rights(self, client_ctx, msg):
        msg = vlob_group_get_rights_serializer.req_load(msg)

        try:
            per_users_rights = await self.get_group_rights(
                client_ctx.organization_id, client_ctx.user_id, msg["id"]
            )

        except VlobAccessError:
            return vlob_group_get_rights_serializer.rep_dump({"status": "not_allowed"})

        except VlobNotFoundError as exc:
            return vlob_group_get_rights_serializer.rep_dump(
                {"status": "not_found", "reason": str(exc)}
            )

        except VlobError as exc:
            return vlob_group_get_rights_serializer.rep_dump(
                {"status": "error", "reason": str(exc)}
            )

        return vlob_group_get_rights_serializer.rep_dump(
            {
                "status": "ok",
                "users": {
                    u: {"admin_right": aa, "read_right": ra, "write_right": wa}
                    for u, (aa, ra, wa) in per_users_rights.items()
                },
            }
        )

    @catch_protocole_errors
    async def api_vlob_group_update_rights(self, client_ctx, msg):
        msg = vlob_group_update_rights_serializer.req_load(msg)

        try:
            await self.update_group_rights(client_ctx.organization_id, client_ctx.user_id, **msg)

        except VlobAccessError:
            return vlob_group_update_rights_serializer.rep_dump({"status": "not_allowed"})

        except VlobNotFoundError as exc:
            return vlob_group_update_rights_serializer.rep_dump(
                {"status": "not_found", "reason": str(exc)}
            )

        except VlobError as exc:
            return vlob_group_update_rights_serializer.rep_dump(
                {"status": "error", "reason": str(exc)}
            )

        return vlob_group_update_rights_serializer.rep_dump({"status": "ok"})

    @catch_protocole_errors
    async def api_vlob_group_poll(self, client_ctx, msg):
        msg = vlob_group_poll_serializer.req_load(msg)

        # TODO: raise error if too many events since offset ?
        try:
            checkpoint, changes = await self.poll_group(
                client_ctx.organization_id, client_ctx.user_id, msg["id"], msg["last_checkpoint"]
            )

        except VlobAccessError:
            return vlob_group_poll_serializer.rep_dump({"status": "not_allowed"})

        except VlobNotFoundError as exc:
            return vlob_group_poll_serializer.rep_dump({"status": "not_found", "reason": str(exc)})

        except VlobError as exc:
            return vlob_group_update_rights_serializer.rep_dump(
                {"status": "error", "reason": str(exc)}
            )

        return vlob_group_poll_serializer.rep_dump(
            {"status": "ok", "current_checkpoint": checkpoint, "changes": changes}
        )

    async def get_group_rights(
        self, organization_id: OrganizationID, author: UserID, id: UUID
    ) -> Dict[DeviceID, Tuple[bool, bool, bool]]:
        raise NotImplementedError()

    async def update_group_rights(
        self,
        organization_id: OrganizationID,
        author: UserID,
        id: UUID,
        user: UserID,
        admin_right: bool,
        read_right: bool,
        write_right: bool,
    ) -> None:
        raise NotImplementedError()

    async def poll_group(
        self, organization_id: OrganizationID, author: UserID, id: UUID, checkpoint: int
    ) -> Tuple[int, Dict[UUID, int]]:
        raise NotImplementedError()

    async def group_check(
        self, organization_id: OrganizationID, author: UserID, to_check: List[dict]
    ) -> List[dict]:
        """
        Raises:
            Nothing !
        """
        raise NotImplementedError()

    async def create(
        self, organization_id: OrganizationID, author: DeviceID, group: UUID, id: UUID, blob: bytes
    ) -> None:
        """
        Raises:
            VlobAlreadyExistsError
        """
        raise NotImplementedError()

    async def read(
        self, organization_id: OrganizationID, author: UserID, id: UUID, version: int = None
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
