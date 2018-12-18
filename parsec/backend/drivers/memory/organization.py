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

    async def create(self, name: str, bootstrap_token: str) -> None:
        if name in self._organizations:
            raise OrganizationAlreadyExistsError()

        self._organizations[name] = Organization(name=name, bootstrap_token=bootstrap_token)

    async def get(self, name: str) -> Organization:
        if name not in self._organizations:
            raise OrganizationNotFoundError()
        return self._organizations[name]

    async def bootstrap(
        self, name: str, bootstrap_token: str, root_verify_key: VerifyKey, user: User
    ) -> None:
        organization = await self.get(name)
        if organization.is_bootstrapped():
            raise OrganizationAlreadyBootstrappedError()

        if organization.bootstrap_token != bootstrap_token:
            raise OrganizationInvalidBootstrapTokenError()

        try:
            await self.user_component.create_user(user)

        except UserError as exc:
            raise OrganizationFirstUserCreationError() from exc

        self._organizations[name] = organization.evolve(root_verify_key=root_verify_key)
