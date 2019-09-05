# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.api.protocol import OrganizationID
from parsec.crypto import VerifyKey
from parsec.backend.user import BaseUserComponent, UserError, User, Device
from parsec.backend.memory.vlob import MemoryVlobComponent
from parsec.backend.memory.block import MemoryBlockComponent

from parsec.backend.organization import (
    BaseOrganizationComponent,
    Organization,
    OrganizationStats,
    OrganizationAlreadyExistsError,
    OrganizationInvalidBootstrapTokenError,
    OrganizationAlreadyBootstrappedError,
    OrganizationNotFoundError,
    OrganizationFirstUserCreationError,
)


class MemoryOrganizationComponent(BaseOrganizationComponent):
    def __init__(
        self,
        user_component: BaseUserComponent,
        vlob_component: MemoryVlobComponent,
        block_component: MemoryBlockComponent,
        **kwargs
    ):
        super().__init__(**kwargs)
        self.user_component = user_component
        self.vlob_component = vlob_component
        self.block_component = block_component
        self._organizations = {}
        self._organizations_invitations = {}

    async def create(self, id: OrganizationID, bootstrap_token: str) -> None:
        if id in self._organizations:
            raise OrganizationAlreadyExistsError()

        self._organizations[id] = Organization(organization_id=id, bootstrap_token=bootstrap_token)

    async def get(self, id: OrganizationID) -> Organization:
        if id not in self._organizations:
            raise OrganizationNotFoundError()
        return self._organizations[id]

    async def bootstrap(
        self,
        id: OrganizationID,
        user: User,
        first_device: Device,
        bootstrap_token: str,
        root_verify_key: VerifyKey,
    ) -> None:
        organization = await self.get(id)
        if organization.is_bootstrapped():
            raise OrganizationAlreadyBootstrappedError()

        if organization.bootstrap_token != bootstrap_token:
            raise OrganizationInvalidBootstrapTokenError()

        try:
            await self.user_component.create_user(id, user, first_device)

        except UserError as exc:
            raise OrganizationFirstUserCreationError(exc) from exc

        self._organizations[organization.organization_id] = organization.evolve(
            root_verify_key=root_verify_key
        )

    async def stats(self, id: OrganizationID) -> OrganizationStats:
        await self.get(id)

        metadata_size = 0
        for (vlob_organization_id, _), vlob in self.vlob_component._vlobs.items():
            if vlob_organization_id == id:
                metadata_size += sum(len(blob) for (blob, _, _) in vlob.data)

        data_size = 0
        for (vlob_organization_id, _), blockmeta in self.block_component._blockmetas.items():
            if vlob_organization_id == id:
                data_size += blockmeta.size

        users = len(self.user_component._organizations[id]._users)

        return OrganizationStats(users=users, data_size=data_size, metadata_size=metadata_size)
