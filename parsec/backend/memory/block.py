# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from typing import TYPE_CHECKING, Any, Dict, Tuple

import attr

from parsec._parsec import DateTime
from parsec.api.protocol import BlockID, DeviceID, OrganizationID, RealmID, RealmRole, UserID
from parsec.backend.block import (
    BaseBlockComponent,
    BlockAccessError,
    BlockAlreadyExistsError,
    BlockInMaintenanceError,
    BlockNotFoundError,
    BlockStoreError,
)
from parsec.backend.blockstore import BaseBlockStoreComponent
from parsec.backend.realm import RealmNotFoundError
from parsec.backend.utils import OperationKind

if TYPE_CHECKING:
    from parsec.backend.memory.realm import MemoryRealmComponent


@attr.s(auto_attribs=True)
class BlockMeta:
    realm_id: RealmID
    size: int
    created_on: DateTime


class MemoryBlockComponent(BaseBlockComponent):
    def __init__(self) -> None:
        self._blockmetas: dict[Tuple[OrganizationID, BlockID], BlockMeta] = {}
        self._blockstore_component: MemoryBlockStoreComponent | None = None
        self._realm_component: MemoryRealmComponent | None = None

    def register_components(
        self,
        blockstore: MemoryBlockStoreComponent,
        realm: MemoryRealmComponent,
        **other_components: Any,
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
        assert self._realm_component is not None

        try:
            realm = self._realm_component._get_realm(organization_id, realm_id)
        except RealmNotFoundError:
            raise BlockNotFoundError(f"Realm `{realm_id.hex}` doesn't exist")

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
            raise BlockInMaintenanceError(f"Realm `{realm_id.hex}` is currently under maintenance")

    async def read(
        self, organization_id: OrganizationID, author: DeviceID, block_id: BlockID
    ) -> bytes:
        assert self._blockstore_component is not None

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
        created_on: DateTime | None = None,
    ) -> None:
        assert self._blockstore_component is not None

        created_on = created_on or DateTime.now()
        self._check_realm_write_access(organization_id, realm_id, author.user_id)
        if (organization_id, block_id) in self._blockmetas:
            raise BlockAlreadyExistsError()

        await self._blockstore_component.create(organization_id, block_id, block)

        self._blockmetas[(organization_id, block_id)] = BlockMeta(realm_id, len(block), created_on)


class MemoryBlockStoreComponent(BaseBlockStoreComponent):
    def __init__(self) -> None:
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
