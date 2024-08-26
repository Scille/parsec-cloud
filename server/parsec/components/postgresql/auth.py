# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import override

from parsec._parsec import (
    DateTime,
    DeviceID,
    InvitationToken,
    InvitationType,
    OrganizationID,
    UserID,
    VerifyKey,
)
from parsec.components.auth import (
    AnonymousAuthInfo,
    AuthAnonymousAuthBadOutcome,
    AuthAuthenticatedAuthBadOutcome,
    AuthenticatedAuthInfo,
    AuthInvitedAuthBadOutcome,
    BaseAuthComponent,
    InvitedAuthInfo,
)
from parsec.components.events import EventBus
from parsec.components.organization import OrganizationCreateBadOutcome
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.organization_create import organization_create
from parsec.components.postgresql.utils import Q, no_transaction, transaction
from parsec.config import BackendConfig
from parsec.logging import get_logger

logger = get_logger()


_q_anonymous_get_info = Q(
    """
SELECT
    _id as organization_internal_id,
    is_expired AS organization_is_expired
FROM organization
WHERE organization_id = $organization_id
-- Note we don't filter out non-bootstrapped organization here, this is because
-- `organization_bootstrap` is precisely done trough anonymous auth !
"""
)


_q_invited_get_info = Q(
    """
SELECT
    _id as organization_internal_id,
    is_expired as organization_is_expired,
    (
        SELECT _id FROM invitation WHERE organization = organization._id AND token = $token LIMIT 1
    ) as invitation_internal_id,
    (
        SELECT type FROM invitation WHERE organization = organization._id AND token = $token LIMIT 1
    ) as invitation_type,
    (
        SELECT deleted_on FROM invitation WHERE organization = organization._id AND token = $token LIMIT
    1) as invitation_deleted_on
FROM organization
WHERE
    organization_id = $organization_id
    -- Only consider bootstrapped organizations
    AND root_verify_key IS NOT NULL
"""
)


_q_authenticated_get_info = Q(
    """
WITH my_organization AS (
    SELECT
        _id,
        is_expired
    FROM organization
    WHERE
        organization_id = $organization_id
        -- Only consider bootstrapped organizations
        AND root_verify_key IS NOT NULL
    LIMIT 1
),

my_device AS (
    SELECT
        device._id,
        device.verify_key,
        device.user_
    FROM device
    INNER JOIN my_organization ON device.organization = my_organization._id
    WHERE
        device.device_id = $device_id
    LIMIT 1
)

SELECT
    (SELECT _id FROM my_organization) as organization_internal_id,
    (SELECT is_expired FROM my_organization) as organization_is_expired,
    (SELECT _id FROM my_device) as device_internal_id,
    (SELECT verify_key FROM my_device) as device_verify_key,
    (
        SELECT user_id
        FROM user_
        INNER JOIN my_device ON user_._id = my_device.user_
    ) as user_id,
    (
        SELECT revoked_on
        FROM user_
        INNER JOIN my_device ON user_._id = my_device.user_
    ) as user_revoked_on,
    (
        SELECT frozen
        FROM user_
        INNER JOIN my_device ON user_._id = my_device.user_
    ) as user_is_frozen
"""
)


class PGAuthComponent(BaseAuthComponent):
    def __init__(self, pool: AsyncpgPool, event_bus: EventBus, config: BackendConfig) -> None:
        super().__init__(event_bus, config)
        self.pool = pool

    @override
    @transaction
    async def anonymous_auth(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        spontaneous_bootstrap: bool,
    ) -> AnonymousAuthInfo | AuthAnonymousAuthBadOutcome:
        while True:
            row = await conn.fetchrow(*_q_anonymous_get_info(organization_id=organization_id.str))
            if row is None:
                # Organization doesn't exist yet, may it's time to create it !

                if not spontaneous_bootstrap:
                    return AuthAnonymousAuthBadOutcome.ORGANIZATION_NOT_FOUND

                outcome = await organization_create(
                    conn,
                    now=now,
                    id=organization_id,
                    active_users_limit=self._config.organization_initial_active_users_limit,
                    user_profile_outsider_allowed=self._config.organization_initial_user_profile_outsider_allowed,
                    minimum_archiving_period=self._config.organization_initial_minimum_archiving_period,
                    bootstrap_token=None,
                )
                match outcome:
                    case int() as organization_internal_id:
                        log_outcome = "success"
                    case OrganizationCreateBadOutcome.ORGANIZATION_ALREADY_EXISTS:
                        log_outcome = "err_already_created"
                logger.info(
                    "spontaneous organization creation",
                    organization_id=organization_id,
                    outcome=log_outcome,
                )
                # The organization should exist now, time to retry
                continue

            break

        match row["organization_internal_id"]:
            case int() as organization_internal_id:
                pass
            case unknown:
                assert False, repr(unknown)

        match row["organization_is_expired"]:
            case False:
                pass
            case True:
                return AuthAnonymousAuthBadOutcome.ORGANIZATION_EXPIRED
            case unknown:
                assert False, repr(unknown)

        return AnonymousAuthInfo(
            organization_id=organization_id,
            organization_internal_id=organization_internal_id,
        )

    @override
    @no_transaction
    async def invited_auth(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        token: InvitationToken,
    ) -> InvitedAuthInfo | AuthInvitedAuthBadOutcome:
        row = await conn.fetchrow(
            *_q_invited_get_info(organization_id=organization_id.str, token=token.hex)
        )
        if not row:
            return AuthInvitedAuthBadOutcome.ORGANIZATION_NOT_FOUND

        match row["organization_internal_id"]:
            case int() as organization_internal_id:
                pass
            case unknown:
                assert False, repr(unknown)

        match row["organization_is_expired"]:
            case False:
                pass
            case True:
                return AuthInvitedAuthBadOutcome.ORGANIZATION_EXPIRED
            case unknown:
                assert False, repr(unknown)

        match row["invitation_internal_id"]:
            case int() as invitation_internal_id:
                pass
            case None:
                return AuthInvitedAuthBadOutcome.INVITATION_NOT_FOUND
            case unknown:
                assert False, repr(unknown)

        match row["invitation_type"]:
            case str() as raw_invitation_type:
                invitation_type = InvitationType.from_str(raw_invitation_type)
            case unknown:
                assert False, repr(unknown)

        match row["invitation_deleted_on"]:
            case None:
                pass
            case DateTime():
                return AuthInvitedAuthBadOutcome.INVITATION_ALREADY_USED
            case unknown:
                assert False, repr(unknown)

        return InvitedAuthInfo(
            organization_id=organization_id,
            token=token,
            type=invitation_type,
            organization_internal_id=organization_internal_id,
            invitation_internal_id=invitation_internal_id,
        )

    @override
    @no_transaction
    async def _get_authenticated_info(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        device_id: DeviceID,
    ) -> AuthenticatedAuthInfo | AuthAuthenticatedAuthBadOutcome:
        row = await conn.fetchrow(
            *_q_authenticated_get_info(organization_id=organization_id.str, device_id=device_id)
        )
        assert row is not None

        match row["organization_internal_id"]:
            case int() as organization_internal_id:
                pass
            case unknown:
                return AuthAuthenticatedAuthBadOutcome.ORGANIZATION_NOT_FOUND

        match row["organization_is_expired"]:
            case False:
                pass
            case True:
                return AuthAuthenticatedAuthBadOutcome.ORGANIZATION_EXPIRED
            case unknown:
                assert False, repr(unknown)

        match row["device_internal_id"]:
            case int() as device_internal_id:
                pass
            case None:
                return AuthAuthenticatedAuthBadOutcome.DEVICE_NOT_FOUND
            case unknown:
                assert False, repr(unknown)

        match row["device_verify_key"]:
            case bytes() as raw_device_verify_key:
                device_verify_key = VerifyKey(raw_device_verify_key)
            case unknown:
                assert False, repr(unknown)

        match row["user_id"]:
            case str() as raw_user_id:
                user_id = UserID.from_hex(raw_user_id)
            case unknown:
                assert False, repr(unknown)

        match row["user_revoked_on"]:
            case None:
                pass
            case DateTime():
                return AuthAuthenticatedAuthBadOutcome.USER_REVOKED
            case unknown:
                assert False, repr(unknown)

        match row["user_is_frozen"]:
            case False:
                pass
            case True:
                return AuthAuthenticatedAuthBadOutcome.USER_FROZEN
            case unknown:
                assert False, repr(unknown)

        return AuthenticatedAuthInfo(
            organization_id=organization_id,
            user_id=user_id,
            device_id=device_id,
            device_verify_key=device_verify_key,
            organization_internal_id=organization_internal_id,
            device_internal_id=device_internal_id,
        )
