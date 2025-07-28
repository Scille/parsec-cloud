# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import override

from parsec._parsec import (
    AccountAuthMethodID,
    DateTime,
    DeviceID,
    EmailAddress,
    InvitationToken,
    InvitationType,
    OrganizationID,
    SecretKey,
    UserID,
    VerifyKey,
)
from parsec.ballpark import timestamps_in_the_ballpark
from parsec.components.auth import (
    AccountAuthenticationToken,
    AnonymousAuthInfo,
    AuthAnonymousAuthBadOutcome,
    AuthAuthenticatedAccountAuthBadOutcome,
    AuthAuthenticatedAuthBadOutcome,
    AuthenticatedAccountAuthInfo,
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
from parsec.config import AllowedClientAgent, BackendConfig
from parsec.logging import get_logger

logger = get_logger()


_q_anonymous_get_info = Q(
    """
SELECT
    _id AS organization_internal_id,
    is_expired AS organization_is_expired,
    allowed_client_agent AS organization_allowed_client_agent
FROM organization
WHERE organization_id = $organization_id
-- Note we don't filter out non-bootstrapped organization here, this is because
-- `organization_bootstrap` is precisely done trough anonymous auth !
"""
)


_q_invited_get_info = Q(
    """
SELECT
    _id AS organization_internal_id,
    is_expired AS organization_is_expired,
    allowed_client_agent AS organization_allowed_client_agent,
    (
        SELECT invitation._id FROM invitation
        WHERE invitation.organization = organization._id AND invitation.token = $token LIMIT 1
    ) AS invitation_internal_id,
    (
        SELECT invitation.type FROM invitation
        WHERE invitation.organization = organization._id AND invitation.token = $token LIMIT 1
    ) AS invitation_type,
    (
        SELECT invitation.deleted_on FROM invitation
        WHERE invitation.organization = organization._id AND invitation.token = $token LIMIT 1
    ) AS invitation_deleted_on
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
        is_expired,
        tos_updated_on,
        allowed_client_agent
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
),

my_user AS (
    SELECT
        user_.user_id,
        user_.revoked_on,
        user_.frozen,
        (
            CASE
                WHEN user_.tos_accepted_on IS NULL
                    THEN
                        -- The user hasn't accepted the TOS, this is acceptable if there
                        -- is no TOS at all for this organization.
                        COALESCE(
                            (
                                SELECT TRUE FROM my_organization
                                WHERE my_organization.tos_updated_on IS NOT NULL
                            ),
                            FALSE
                        )
                ELSE
                    -- The user has accepted an TOS, we must make sure it corresponds to
                    -- the current one in the organization.
                    -- If the organization no longer has a TOS, then what the user has
                    -- accepted in the past is irrelevant.
                    COALESCE(
                        (user_.tos_accepted_on < (SELECT my_organization.tos_updated_on FROM my_organization)),
                        FALSE
                    )
            END
        ) AS user_must_accept_tos
    FROM user_
    INNER JOIN my_device ON user_._id = my_device.user_
    LIMIT 1
)

SELECT
    (SELECT _id FROM my_organization) AS organization_internal_id,
    (SELECT is_expired FROM my_organization) AS organization_is_expired,
    (SELECT allowed_client_agent FROM my_organization) AS organization_allowed_client_agent,
    (SELECT _id FROM my_device) AS device_internal_id,
    (SELECT verify_key FROM my_device) AS device_verify_key,
    (SELECT user_id FROM my_user) AS user_id,
    (SELECT revoked_on FROM my_user) AS user_revoked_on,
    (SELECT frozen FROM my_user) AS user_is_frozen,
    (SELECT user_must_accept_tos FROM my_user) AS user_must_accept_tos
"""
)


_q_authenticated_account_get_info = Q(
    """
SELECT
    account._id AS account_internal_id,
    account.email AS account_email,
    vault_authentication_method._id AS auth_method_internal_id,
    vault_authentication_method.auth_method_id,
    vault_authentication_method.mac_key AS auth_method_mac_key
FROM account
INNER JOIN vault ON account._id = vault.account
INNER JOIN vault_authentication_method ON vault._id = vault_authentication_method.vault
WHERE
    vault_authentication_method.auth_method_id = $auth_method_id
    AND vault_authentication_method.disabled_on IS NULL
    -- Extra safety check since a deleted account should not have any active authentication method
    AND account.deleted_on IS NULL
LIMIT 1
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
                    tos_per_locale_urls=self._config.organization_initial_tos,
                    allowed_client_agent=self._config.organization_initial_allowed_client_agent,
                    account_vault_strategy=self._config.organization_initial_account_vault_strategy,
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
            case _:
                assert False, row

        match row["organization_is_expired"]:
            case False:
                pass
            case True:
                return AuthAnonymousAuthBadOutcome.ORGANIZATION_EXPIRED
            case _:
                assert False, row

        match row["organization_allowed_client_agent"]:
            case str() as raw_allowed_client_agent:
                organization_allowed_client_agent = AllowedClientAgent(raw_allowed_client_agent)
            case _:
                assert False, row

        return AnonymousAuthInfo(
            organization_id=organization_id,
            organization_internal_id=organization_internal_id,
            organization_allowed_client_agent=organization_allowed_client_agent,
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
            case _:
                assert False, row

        match row["organization_is_expired"]:
            case False:
                pass
            case True:
                return AuthInvitedAuthBadOutcome.ORGANIZATION_EXPIRED
            case _:
                assert False, row

        match row["invitation_internal_id"]:
            case int() as invitation_internal_id:
                pass
            case None:
                return AuthInvitedAuthBadOutcome.INVITATION_NOT_FOUND
            case _:
                assert False, row

        match row["invitation_type"]:
            case str() as raw_invitation_type:
                invitation_type = InvitationType.from_str(raw_invitation_type)
            case _:
                assert False, row

        match row["invitation_deleted_on"]:
            case None:
                pass
            case DateTime():
                return AuthInvitedAuthBadOutcome.INVITATION_ALREADY_USED
            case _:
                assert False, row

        match row["organization_allowed_client_agent"]:
            case str() as raw_allowed_client_agent:
                organization_allowed_client_agent = AllowedClientAgent(raw_allowed_client_agent)
            case _:
                assert False, row

        return InvitedAuthInfo(
            organization_id=organization_id,
            token=token,
            type=invitation_type,
            organization_internal_id=organization_internal_id,
            invitation_internal_id=invitation_internal_id,
            organization_allowed_client_agent=organization_allowed_client_agent,
        )

    @override
    @no_transaction
    async def _get_authenticated_info(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        device_id: DeviceID,
        tos_acceptance_required: bool,
    ) -> AuthenticatedAuthInfo | AuthAuthenticatedAuthBadOutcome:
        row = await conn.fetchrow(
            *_q_authenticated_get_info(organization_id=organization_id.str, device_id=device_id)
        )
        assert row is not None

        match row["organization_internal_id"]:
            case int() as organization_internal_id:
                pass
            case _:
                return AuthAuthenticatedAuthBadOutcome.ORGANIZATION_NOT_FOUND

        match row["organization_is_expired"]:
            case False:
                pass
            case True:
                return AuthAuthenticatedAuthBadOutcome.ORGANIZATION_EXPIRED
            case _:
                assert False, row

        match row["device_internal_id"]:
            case int() as device_internal_id:
                pass
            case None:
                return AuthAuthenticatedAuthBadOutcome.DEVICE_NOT_FOUND
            case _:
                assert False, row

        match row["device_verify_key"]:
            case bytes() as raw_device_verify_key:
                device_verify_key = VerifyKey(raw_device_verify_key)
            case _:
                assert False, row

        match row["user_id"]:
            case str() as raw_user_id:
                user_id = UserID.from_hex(raw_user_id)
            case _:
                assert False, row

        match row["user_revoked_on"]:
            case None:
                pass
            case DateTime():
                return AuthAuthenticatedAuthBadOutcome.USER_REVOKED
            case _:
                assert False, row

        match row["user_is_frozen"]:
            case False:
                pass
            case True:
                return AuthAuthenticatedAuthBadOutcome.USER_FROZEN
            case _:
                assert False, row

        if tos_acceptance_required:
            match row["user_must_accept_tos"]:
                case False:
                    pass
                case True:
                    return AuthAuthenticatedAuthBadOutcome.USER_MUST_ACCEPT_TOS
                case _:
                    assert False, row

        match row["organization_allowed_client_agent"]:
            case str() as raw_allowed_client_agent:
                organization_allowed_client_agent = AllowedClientAgent(raw_allowed_client_agent)
            case _:
                assert False, row

        return AuthenticatedAuthInfo(
            organization_id=organization_id,
            user_id=user_id,
            device_id=device_id,
            device_verify_key=device_verify_key,
            organization_internal_id=organization_internal_id,
            device_internal_id=device_internal_id,
            organization_allowed_client_agent=organization_allowed_client_agent,
        )

    @override
    @no_transaction
    async def authenticated_account_auth(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        token: AccountAuthenticationToken,
    ) -> AuthenticatedAccountAuthInfo | AuthAuthenticatedAccountAuthBadOutcome:
        row = await conn.fetchrow(
            *_q_authenticated_account_get_info(auth_method_id=token.auth_method_id)
        )
        if not row:
            return AuthAuthenticatedAccountAuthBadOutcome.ACCOUNT_NOT_FOUND

        match row["account_internal_id"]:
            case int() as account_internal_id:
                pass
            case _:
                assert False, row

        match row["account_email"]:
            case str() as raw_account_email:
                account_email = EmailAddress(raw_account_email)
            case _:
                assert False, row

        match row["auth_method_internal_id"]:
            case int() as auth_method_internal_id:
                pass
            case _:
                assert False, row

        match row["auth_method_id"]:
            case str() as raw_auth_method_id:
                auth_method_id = AccountAuthMethodID.from_hex(raw_auth_method_id)
            case _:
                assert False, row

        match row["auth_method_mac_key"]:
            case bytes() as raw_auth_method_mac_key:
                auth_method_mac_key = SecretKey(raw_auth_method_mac_key)
            case _:
                assert False, row

        # Verify the token with the MAC key
        if not token.verify(auth_method_mac_key):
            return AuthAuthenticatedAccountAuthBadOutcome.INVALID_TOKEN

        # Check token timestamp is within ballpark
        if timestamps_in_the_ballpark(token.timestamp, now) is not None:
            return AuthAuthenticatedAccountAuthBadOutcome.TOKEN_OUT_OF_BALLPARK

        return AuthenticatedAccountAuthInfo(
            account_email=account_email,
            auth_method_id=auth_method_id,
            account_internal_id=account_internal_id,
            auth_method_internal_id=auth_method_internal_id,
        )
