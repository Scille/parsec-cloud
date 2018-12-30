from triopg import UniqueViolationError

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

    async def create(self, name: str, bootstrap_token: str) -> None:
        async with self.dbh.pool.acquire() as conn:
            try:
                result = await conn.execute(
                    """
                    INSERT INTO organizations (
                        name, bootstrap_token
                    ) VALUES ($1, $2)
                    """,
                    name,
                    bootstrap_token,
                )
            except UniqueViolationError:
                raise OrganizationAlreadyExistsError()

            if result != "INSERT 0 1":
                raise OrganizationError(f"Insertion error: {result}")

    async def get(self, name: str) -> Organization:
        async with self.dbh.pool.acquire() as conn:
            return self._get(conn, name)

    @staticmethod
    async def _get(conn, name: str) -> Organization:
        data = await conn.fetchrow(
            """
                SELECT name, bootstrap_token, root_verify_key
                FROM organizations WHERE name = $1
                """,
            name,
        )
        if not data:
            raise OrganizationNotFoundError()

        rvk = VerifyKey(data[2]) if data[2] else None
        return Organization(name=data[0], bootstrap_token=data[1], root_verify_key=rvk)

    async def bootstrap(
        self, name: str, bootstrap_token: str, root_verify_key: VerifyKey, user: User
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                organization = await self._get(conn, name)

                if organization.is_bootstrapped():
                    raise OrganizationAlreadyBootstrappedError()

                if organization.bootstrap_token != bootstrap_token:
                    raise OrganizationInvalidBootstrapTokenError()

                try:
                    await self.user_component._create_user(conn, user)
                except UserError as exc:
                    raise OrganizationFirstUserCreationError() from exc

                result = await conn.execute(
                    """
                    UPDATE organizations SET root_verify_key = $3 WHERE name = $1 AND bootstrap_token = $2 AND root_verify_key IS NULL;
                    """,
                    name,
                    bootstrap_token,
                    root_verify_key.encode(),
                )

                if result != "UPDATE 1":
                    raise OrganizationError(f"Update error: {result}")
