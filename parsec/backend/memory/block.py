# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

import attr
from typing import TYPE_CHECKING, Dict, Optional, Tuple

from parsec._parsec import DateTime
from parsec.api.protocol import OrganizationID, DeviceID, UserID, RealmID, RealmRole, BlockID
from parsec.backend.utils import OperationKind
from parsec.backend.realm import RealmNotFoundError
from parsec.backend.blockstore import BaseBlockStoreComponent
from parsec.backend.block import (
    BaseBlockComponent,
    BlockAlreadyExistsError,
    BlockAccessError,
    BlockNotFoundError,
    BlockInMaintenanceError,
    BlockStoreError,
)

if TYPE_CHECKING:
    from parsec.backend.memory.realm import MemoryRealmComponent


@attr.s(auto_attribs=True)
class BlockMeta:
    realm_id: RealmID
    size: int
    created_on: DateTime


class MemoryBlockComponent(BaseBlockComponent):
    def __init__(self):
        self._blockmetas = {}
        self._blockstore_component: "MemoryBlockStoreComponent" = None
        self._realm_component: "MemoryRealmComponent" = None

    def register_components(
        self,
        blockstore: "MemoryBlockStoreComponent",
        realm: "MemoryRealmComponent",
        **other_components,
    ) -> None:
        self._blockstore_component = blockstore
        self._realm_component = realm

    def _check_realm_read_access(
        self, organization_id: OrganizationID, realm_id: RealmID, user_id: UserID
    ) -> None:
        self._check_realm_access(organization_id, realm_id, user_id, OperationKind.DATA_READ)

    def _check_realm_write_access(
        self, organization_id: OrganizationID, realm_id: RealmID, user_id: UserID
    ) -> None:
        self._check_realm_access(organization_id, realm_id, user_id, OperationKind.DATA_WRITE)

    def _check_realm_access(
        self,
        organization_id: OrganizationID,
        realm_id: RealmID,
        user_id: UserID,
        operation_kind: OperationKind,
    ) -> None:
        try:
            realm = self._realm_component._get_realm(organization_id, realm_id)
        except RealmNotFoundError:
            raise BlockNotFoundError(f"Realm `{realm_id.str}` doesn't exist")

        allowed_roles: Tuple[RealmRole, ...]
        if operation_kind == operation_kind.DATA_READ:
            allowed_roles = (
                RealmRole.OWNER,
                RealmRole.MANAGER,
                RealmRole.CONTRIBUTOR,
                RealmRole.READER,
            )
        elif operation_kind == operation_kind.DATA_WRITE:
            allowed_roles = (RealmRole.OWNER, RealmRole.MANAGER, RealmRole.CONTRIBUTOR)
        elif operation_kind == operation_kind.MAINTENANCE:
            allowed_roles = (RealmRole.OWNER,)
        else:
            assert False, f"Operation kind {operation_kind} not supported"

        if realm.roles.get(user_id) not in allowed_roles:
            raise BlockAccessError()

        # Special case of reading while in reencryption is authorized
        if realm.status.in_reencryption and operation_kind == OperationKind.DATA_READ:
            pass
        # Access is not allowed while in maintenance
        elif realm.status.in_maintenance:
            raise BlockInMaintenanceError(f"Realm `{realm_id.str}` is currently under maintenance")

    async def read(
        self, organization_id: OrganizationID, author: DeviceID, block_id: BlockID
    ) -> bytes:
        try:
            blockmeta = self._blockmetas[(organization_id, block_id)]

        except KeyError:
            raise BlockNotFoundError()

        self._check_realm_read_access(organization_id, blockmeta.realm_id, author.user_id)

        return await self._blockstore_component.read(organization_id, block_id)

    async def create(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        block_id: BlockID,
        realm_id: RealmID,
        block: bytes,
        timestamp: Optional[DateTime] = None,
    ) -> None:
        timestamp = timestamp or DateTime.now()
        self._check_realm_write_access(organization_id, realm_id, author.user_id)
        if (organization_id, block_id) in self._blockmetas:
            raise BlockAlreadyExistsError()

        await self._blockstore_component.create(organization_id, block_id, block)

        self._blockmetas[(organization_id, block_id)] = BlockMeta(realm_id, len(block), timestamp)


class MemoryBlockStoreComponent(BaseBlockStoreComponent):
    def __init__(self):
        self._blocks: Dict[Tuple[OrganizationID, BlockID], bytes] = {}

    async def read(self, organization_id: OrganizationID, block_id: BlockID) -> bytes:
        try:
            return self._blocks[(organization_id, block_id)]

        except KeyError:
            raise BlockStoreError()

    async def create(
        self, organization_id: OrganizationID, block_id: BlockID, block: bytes
    ) -> None:
        key = (organization_id, block_id)
        self._blocks[key] = block
