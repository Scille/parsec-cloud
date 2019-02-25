# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.types import OrganizationID
from parsec.crypto import VerifyKey
from parsec.backend.user import BaseUserComponent, UserError, User
from parsec.backend.organization import (
    BaseOrganizationComponent,
    Organization,
    OrganizationAlreadyExistsError,
    OrganizationInvalidBootstrapTokenError,
    OrganizationAlreadyBootstrappedError,
    OrganizationNotFoundError,
    OrganizationFirstUserCreationError,
)


class MemoryOrganizationComponent(BaseOrganizationComponent):
    def __init__(self, user_component: BaseUserComponent, **kwargs):
        super().__init__(**kwargs)
        self.user_component = user_component
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
        self, id: OrganizationID, user: User, bootstrap_token: str, root_verify_key: VerifyKey
    ) -> None:
        organization = await self.get(id)
        if organization.is_bootstrapped():
            raise OrganizationAlreadyBootstrappedError()

        if organization.bootstrap_token != bootstrap_token:
            raise OrganizationInvalidBootstrapTokenError()

        try:
            await self.user_component.create_user(id, user)

        except UserError as exc:
            raise OrganizationFirstUserCreationError(exc) from exc

        self._organizations[organization.organization_id] = organization.evolve(
            root_verify_key=root_verify_key
        )
