# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from typing import Optional, Dict, Union

from functools import lru_cache
from pendulum import DateTime
from triopg import UniqueViolationError

from parsec.api.protocol import OrganizationID
from parsec.crypto import VerifyKey
from parsec.backend.events import BackendEvent
from parsec.backend.user import UserError, User, Device
from parsec.backend.utils import unset_sentinel, Unset
from parsec.backend.organization import (
    BaseOrganizationComponent,
    OrganizationStats,
    Organization,
    OrganizationError,
    OrganizationAlreadyExistsError,
    OrganizationInvalidBootstrapTokenError,
    OrganizationAlreadyBootstrappedError,
    OrganizationNotFoundError,
    OrganizationFirstUserCreationError,
)
from parsec.backend.postgresql.handler import PGHandler
from parsec.backend.postgresql.user_queries.create import _create_user
from parsec.backend.postgresql.utils import Q, q_organization_internal_id
from parsec.backend.postgresql.handler import send_signal


_q_insert_organization = Q(
    """
INSERT INTO organization (organization_id, bootstrap_token, expiration_date, allow_outsider_profile)
VALUES ($organization_id, $bootstrap_token, $expiration_date, FALSE)
ON CONFLICT (organization_id) DO
    UPDATE SET
        bootstrap_token = EXCLUDED.bootstrap_token,
        expiration_date = EXCLUDED.expiration_date
    WHERE organization.root_verify_key is NULL
"""
)


_q_get_organization = Q(
    """
SELECT bootstrap_token, root_verify_key, expiration_date, allow_outsider_profile
FROM organization
WHERE organization_id = $organization_id
"""
)


_q_bootstrap_organization = Q(
    """
UPDATE organization
SET root_verify_key = $root_verify_key
WHERE
    organization_id = $organization_id
    AND bootstrap_token = $bootstrap_token
    AND root_verify_key IS NULL
"""
)


_q_get_stats = Q(
    f"""
SELECT
    (
        SELECT COUNT(*)
        FROM user_
        WHERE user_.organization = { q_organization_internal_id("$organization_id") }
    ) users,
    (
        SELECT COUNT(*)
        FROM realm
        WHERE realm.organization = { q_organization_internal_id("$organization_id") }
    ) workspaces,
    (
        SELECT COALESCE(SUM(size), 0)
        FROM vlob_atom
        WHERE
            organization = { q_organization_internal_id("$organization_id") }
    ) metadata_size,
    (
        SELECT COALESCE(SUM(size), 0)
        FROM block
        WHERE
            organization = { q_organization_internal_id("$organization_id") }
    ) data_size
"""
)


_q_update_organisation_expiration_date = Q(
    """
UPDATE organization
SET expiration_date = $expiration_date
WHERE organization_id = $organization_id
"""
)


@lru_cache()
def _q_update_factory(with_expiration_date: bool, with_allow_outsider_profile: bool):
    fields = []
    if with_expiration_date:
        fields.append("expiration_date = $expiration_date")
    if with_allow_outsider_profile:
        fields.append("allow_outsider_profile = $allow_outsider_profile")

    return Q(
        f"""
            UPDATE organization
            SET
            { ", ".join(fields) }
            WHERE organization_id = $organization_id
        """
    )


class PGOrganizationComponent(BaseOrganizationComponent):
    def __init__(self, dbh: PGHandler, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.dbh = dbh

    async def create(
        self, id: OrganizationID, bootstrap_token: str, expiration_date: Optional[DateTime] = None
    ) -> None:
        async with self.dbh.pool.acquire() as conn:
            try:
                result = await conn.execute(
                    *_q_insert_organization(
                        organization_id=id,
                        bootstrap_token=bootstrap_token,
                        expiration_date=expiration_date,
                    )
                )
            except UniqueViolationError:
                raise OrganizationAlreadyExistsError()

            if result != "INSERT 0 1":
                raise OrganizationAlreadyExistsError()

    async def get(self, id: OrganizationID) -> Organization:
        async with self.dbh.pool.acquire() as conn:
            return await self._get(conn, id)

    @staticmethod
    async def _get(conn, id: OrganizationID) -> Organization:
        data = await conn.fetchrow(*_q_get_organization(organization_id=id))
        if not data:
            raise OrganizationNotFoundError()

        rvk = VerifyKey(data[1]) if data[1] else None
        return Organization(
            organization_id=id,
            bootstrap_token=data[0],
            root_verify_key=rvk,
            expiration_date=data[2],
            allow_outsider_profile=data[3],
        )

    async def bootstrap(
        self,
        id: OrganizationID,
        user: User,
        first_device: Device,
        bootstrap_token: str,
        root_verify_key: VerifyKey,
    ) -> None:
        async with self.dbh.pool.acquire() as conn, conn.transaction():
            organization = await self._get(conn, id)

            if organization.is_bootstrapped():
                raise OrganizationAlreadyBootstrappedError()

            if organization.bootstrap_token != bootstrap_token:
                raise OrganizationInvalidBootstrapTokenError()

            try:
                await _create_user(conn, id, user, first_device)
            except UserError as exc:
                raise OrganizationFirstUserCreationError(exc) from exc

            result = await conn.execute(
                *_q_bootstrap_organization(
                    organization_id=id,
                    bootstrap_token=bootstrap_token,
                    root_verify_key=root_verify_key.encode(),
                )
            )

            if result != "UPDATE 1":
                raise OrganizationError(f"Update error: {result}")

    async def stats(self, id: OrganizationID) -> OrganizationStats:
        async with self.dbh.pool.acquire() as conn, conn.transaction():
            await self._get(conn, id)  # Check organization exists
            result = await conn.fetchrow(*_q_get_stats(organization_id=id))
        return OrganizationStats(
            users=result["users"],
            data_size=result["data_size"],
            metadata_size=result["metadata_size"],
            workspaces=result["workspaces"],
        )

    async def update(
        self,
        id: OrganizationID,
        expiration_date: Union[Unset, Optional[DateTime]] = unset_sentinel,
        allow_outsider_profile: Union[Unset, bool] = unset_sentinel,
    ) -> None:
        """
        Raises:
            OrganizationNotFoundError
            OrganizationError
        """
        fields: Dict[str, Union[Optional[DateTime], bool]] = {}
        with_expiration_date = False
        with_allow_outsider_profile: bool = False

        if not isinstance(expiration_date, Unset):
            with_expiration_date = True
            fields["expiration_date"] = expiration_date
        if not isinstance(allow_outsider_profile, Unset):
            with_allow_outsider_profile = True
            fields["allow_outsider_profile"] = allow_outsider_profile

        q = _q_update_factory(
            with_expiration_date=with_expiration_date,
            with_allow_outsider_profile=with_allow_outsider_profile,
        )
        async with self.dbh.pool.acquire() as conn, conn.transaction():
            result = await conn.execute(*q(organization_id=id, **fields))

            if result == "UPDATE 0":
                raise OrganizationNotFoundError

            if result != "UPDATE 1":
                raise OrganizationError(f"Update error: {result}")

            if (
                isinstance(fields.get("expiration_date"), DateTime)
                and fields.get("expiration_date") <= DateTime.now()  # type: ignore[operator]
            ):
                await send_signal(conn, BackendEvent.ORGANIZATION_EXPIRED, organization_id=id)
