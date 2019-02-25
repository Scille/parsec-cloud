# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from uuid import UUID
from typing import Dict, Tuple

from parsec.types import DeviceID, UserID, OrganizationID
from parsec.api.protocole import (
    beacon_get_rights_serializer,
    beacon_set_rights_serializer,
    beacon_poll_serializer,
)
from parsec.backend.utils import catch_protocole_errors


class BeaconError(Exception):
    pass


class BeaconAlreadyExists(BeaconError):
    pass


class BeaconNotFound(BeaconError):
    pass


class BeaconAccessError(BeaconError):
    pass


class BaseBeaconComponent:
    @catch_protocole_errors
    async def api_beacon_get_rights(self, client_ctx, msg):
        msg = beacon_get_rights_serializer.req_load(msg)

        try:
            per_users_rights = await self.get_rights(
                client_ctx.organization_id, client_ctx.user_id, msg["id"]
            )

        except BeaconAccessError:
            return beacon_poll_serializer.rep_dump({"status": "not_allowed"})

        except BeaconNotFound:
            return beacon_poll_serializer.rep_dump({"status": "not_found"})

        return beacon_get_rights_serializer.rep_dump(
            {
                "status": "ok",
                "users": {
                    u: {"admin_access": aa, "read_access": ra, "write_access": wa}
                    for u, (aa, ra, wa) in per_users_rights.items()
                },
            }
        )

    @catch_protocole_errors
    async def api_beacon_set_rights(self, client_ctx, msg):
        msg = beacon_set_rights_serializer.req_load(msg)

        try:
            await self.set_rights(client_ctx.organization_id, client_ctx.user_id, **msg)

        except BeaconAccessError:
            return beacon_poll_serializer.rep_dump({"status": "not_allowed"})

        except BeaconNotFound:
            return beacon_poll_serializer.rep_dump({"status": "not_found"})

        return beacon_set_rights_serializer.rep_dump({"status": "ok"})

    @catch_protocole_errors
    async def api_beacon_poll(self, client_ctx, msg):
        msg = beacon_poll_serializer.req_load(msg)

        # TODO: raise error if too many events since offset ?
        try:
            checkpoint, changes = await self.poll(
                client_ctx.organization_id, client_ctx.user_id, msg["id"], msg["last_checkpoint"]
            )

        except BeaconAccessError:
            return beacon_poll_serializer.rep_dump({"status": "not_allowed"})

        except BeaconNotFound:
            return beacon_poll_serializer.rep_dump({"status": "not_found"})

        return beacon_poll_serializer.rep_dump(
            {"status": "ok", "current_checkpoint": checkpoint, "changes": changes}
        )

    async def get_rights(
        self, organization_id: OrganizationID, author: UserID, id: UUID
    ) -> Dict[DeviceID, Tuple[bool, bool, bool]]:
        raise NotImplementedError()

    async def set_rights(
        self,
        organization_id: OrganizationID,
        author: UserID,
        id: UUID,
        user: UserID,
        admin_access: bool,
        read_access: bool,
        write_access: bool,
    ) -> None:
        raise NotImplementedError()

    async def poll(
        self, organization_id: OrganizationID, author: UserID, id: UUID, checkpoint: int
    ) -> Tuple[int, Dict[UUID, int]]:
        raise NotImplementedError()
