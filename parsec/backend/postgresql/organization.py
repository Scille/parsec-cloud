# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from triopg import UniqueViolationError
from pypika import Parameter

from parsec.types import OrganizationID
from parsec.crypto import VerifyKey
from parsec.backend.user import BaseUserComponent, UserError, User, Device
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
from parsec.backend.postgresql.handler import PGHandler
from parsec.backend.postgresql.utils import Query
from parsec.backend.postgresql.tables import t_organization, q_organization
from parsec.backend.postgresql.user_queries.create import _create_user


_q_insert_organization = (
    Query.into(t_organization)
    .columns("organization_id", "bootstrap_token")
    .insert(Parameter("$1"), Parameter("$2"))
    .get_sql()
)


_q_get_organization = (
    q_organization(Parameter("$1")).select("bootstrap_token", "root_verify_key").get_sql()
)


_q_bootstrap_organization = (
    Query.update(t_organization)
    .where(
        (t_organization.organization_id == Parameter("$1"))
        & (t_organization.bootstrap_token == Parameter("$2"))
        & (t_organization.root_verify_key.isnull())
    )
    .set(t_organization.root_verify_key, Parameter("$3"))
    .get_sql()
)


class PGOrganizationComponent(BaseOrganizationComponent):
    def __init__(self, dbh: PGHandler, user_component: BaseUserComponent, **kwargs):
        super().__init__(**kwargs)
        self.dbh = dbh
        self.user_component = user_component

    async def create(self, id: OrganizationID, bootstrap_token: str) -> None:
        async with self.dbh.pool.acquire() as conn:
            try:
                result = await conn.execute(_q_insert_organization, id, bootstrap_token)
            except UniqueViolationError:
                raise OrganizationAlreadyExistsError()

            if result != "INSERT 0 1":
                raise OrganizationError(f"Insertion error: {result}")

    async def get(self, id: OrganizationID) -> Organization:
        async with self.dbh.pool.acquire() as conn:
            return await self._get(conn, id)

    @staticmethod
    async def _get(conn, id: OrganizationID) -> Organization:
        data = await conn.fetchrow(_q_get_organization, id)
        if not data:
            raise OrganizationNotFoundError()

        rvk = VerifyKey(data[1]) if data[1] else None
        return Organization(organization_id=id, bootstrap_token=data[0], root_verify_key=rvk)

    async def bootstrap(
        self,
        organization_id: OrganizationID,
        user: User,
        first_device: Device,
        bootstrap_token: str,
        root_verify_key: VerifyKey,
    ) -> None:
        async with self.dbh.pool.acquire() as conn, conn.transaction():
            organization = await self._get(conn, organization_id)

            if organization.is_bootstrapped():
                raise OrganizationAlreadyBootstrappedError()

            if organization.bootstrap_token != bootstrap_token:
                raise OrganizationInvalidBootstrapTokenError()

            try:
                await _create_user(conn, organization_id, user, first_device)
            except UserError as exc:
                raise OrganizationFirstUserCreationError(exc) from exc

            result = await conn.execute(
                _q_bootstrap_organization,
                organization_id,
                bootstrap_token,
                root_verify_key.encode(),
            )

            if result != "UPDATE 1":
                raise OrganizationError(f"Update error: {result}")
