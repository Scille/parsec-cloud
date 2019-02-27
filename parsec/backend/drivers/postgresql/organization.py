# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from triopg import UniqueViolationError

from parsec.types import OrganizationID
from parsec.crypto import VerifyKey
from parsec.backend.user import BaseUserComponent, UserError, User
from parsec.backend.organization import (
    BaseOrganizationComponent,
    Organization,
    OrganizationError,
    OrganizationAlreadyExistsError,
    OrganizationInvalidBootstrapTokenError,
    OrganizationAlreadyBootstrappedError,
    OrganizationNotFoundError,
    OrganizationFirstUserCreationError,
)
from parsec.backend.drivers.postgresql.handler import PGHandler


class PGOrganizationComponent(BaseOrganizationComponent):
    def __init__(self, dbh: PGHandler, user_component: BaseUserComponent, **kwargs):
        super().__init__(**kwargs)
        self.dbh = dbh
        self.user_component = user_component

    async def create(self, id: OrganizationID, bootstrap_token: str) -> None:
        async with self.dbh.pool.acquire() as conn:
            try:
                result = await conn.execute(
                    """
INSERT INTO organization (
    organization_id, bootstrap_token
) VALUES ($1, $2)
""",
                    id,
                    bootstrap_token,
                )
            except UniqueViolationError:
                raise OrganizationAlreadyExistsError()

            if result != "INSERT 0 1":
                raise OrganizationError(f"Insertion error: {result}")

    async def get(self, id: OrganizationID) -> Organization:
        async with self.dbh.pool.acquire() as conn:
            return await self._get(conn, id)

    @staticmethod
    async def _get(conn, id: OrganizationID) -> Organization:
        data = await conn.fetchrow(
            """
SELECT bootstrap_token, root_verify_key
FROM organization
WHERE organization_id = $1
""",
            id,
        )
        if not data:
            raise OrganizationNotFoundError()

        rvk = VerifyKey(data[1]) if data[1] else None
        return Organization(organization_id=id, bootstrap_token=data[0], root_verify_key=rvk)

    async def bootstrap(
        self,
        organization_id: OrganizationID,
        user: User,
        bootstrap_token: str,
        root_verify_key: VerifyKey,
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                organization = await self._get(conn, organization_id)

                if organization.is_bootstrapped():
                    raise OrganizationAlreadyBootstrappedError()

                if organization.bootstrap_token != bootstrap_token:
                    raise OrganizationInvalidBootstrapTokenError()

                try:
                    await self.user_component._create_user(conn, organization_id, user)
                except UserError as exc:
                    raise OrganizationFirstUserCreationError(exc) from exc

                result = await conn.execute(
                    """
UPDATE organization
SET root_verify_key = $3
WHERE
    organization_id = $1
    AND bootstrap_token = $2
    AND root_verify_key IS NULL;
""",
                    organization_id,
                    bootstrap_token,
                    root_verify_key.encode(),
                )

                if result != "UPDATE 1":
                    raise OrganizationError(f"Update error: {result}")
