# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Any, override

from parsec._parsec import DateTime, DeviceID, InvitationToken, OrganizationID
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
from parsec.components.memory.datamodel import MemoryDatamodel, MemoryOrganization
from parsec.components.organization import TermsOfService


class MemoryAuthComponent(BaseAuthComponent):
    def __init__(self, data: MemoryDatamodel, *args: Any, **kwargs: Any) -> None:
        super().__init__(*args, **kwargs)
        self._data = data

    @override
    async def anonymous_auth(
        self, now: DateTime, organization_id: OrganizationID, spontaneous_bootstrap: bool
    ) -> AnonymousAuthInfo | AuthAnonymousAuthBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            if spontaneous_bootstrap:
                org = MemoryOrganization(
                    organization_id=organization_id,
                    bootstrap_token=None,
                    user_profile_outsider_allowed=self._config.organization_initial_user_profile_outsider_allowed,
                    active_users_limit=self._config.organization_initial_active_users_limit,
                    minimum_archiving_period=self._config.organization_initial_minimum_archiving_period,
                    tos=None
                    if self._config.organization_initial_tos is None
                    else TermsOfService(
                        updated_on=now, per_locale_urls=self._config.organization_initial_tos
                    ),
                    allowed_client_agent=self._config.organization_initial_allowed_client_agent,
                    account_vault_strategy=self._config.organization_initial_account_vault_strategy,
                    created_on=now,
                )
                self._data.organizations[organization_id] = org

            else:
                return AuthAnonymousAuthBadOutcome.ORGANIZATION_NOT_FOUND

        if org.is_expired:
            return AuthAnonymousAuthBadOutcome.ORGANIZATION_EXPIRED

        return AnonymousAuthInfo(
            organization_id=organization_id,
            organization_internal_id=0,  # Only used by PostgreSQL implementation
            organization_allowed_client_agent=org.allowed_client_agent,
        )

    @override
    async def invited_auth(
        self, now: DateTime, organization_id: OrganizationID, token: InvitationToken
    ) -> InvitedAuthInfo | AuthInvitedAuthBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return AuthInvitedAuthBadOutcome.ORGANIZATION_NOT_FOUND

        if org.is_expired:
            return AuthInvitedAuthBadOutcome.ORGANIZATION_EXPIRED

        try:
            invitation = org.invitations[token]
        except KeyError:
            return AuthInvitedAuthBadOutcome.INVITATION_NOT_FOUND

        if invitation.deleted_on:
            return AuthInvitedAuthBadOutcome.INVITATION_ALREADY_USED

        return InvitedAuthInfo(
            organization_id=organization_id,
            token=token,
            type=invitation.type,
            organization_internal_id=0,  # Only used by PostgreSQL implementation
            invitation_internal_id=0,  # Only used by PostgreSQL implementation
            organization_allowed_client_agent=org.allowed_client_agent,
        )

    @override
    async def _get_authenticated_info(
        self, organization_id: OrganizationID, device_id: DeviceID, tos_acceptance_required: bool
    ) -> AuthenticatedAuthInfo | AuthAuthenticatedAuthBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return AuthAuthenticatedAuthBadOutcome.ORGANIZATION_NOT_FOUND

        if org.is_expired:
            return AuthAuthenticatedAuthBadOutcome.ORGANIZATION_EXPIRED

        try:
            device = org.devices[device_id]
        except KeyError:
            return AuthAuthenticatedAuthBadOutcome.DEVICE_NOT_FOUND
        user_id = device.cooked.user_id
        user = org.users[user_id]
        if user.is_revoked:
            return AuthAuthenticatedAuthBadOutcome.USER_REVOKED
        if user.is_frozen:
            return AuthAuthenticatedAuthBadOutcome.USER_FROZEN
        if tos_acceptance_required and org.tos is not None:
            if user.tos_accepted_on is None or user.tos_accepted_on < org.tos.updated_on:
                return AuthAuthenticatedAuthBadOutcome.USER_MUST_ACCEPT_TOS

        return AuthenticatedAuthInfo(
            organization_id=organization_id,
            user_id=user_id,
            device_id=device_id,
            device_verify_key=device.cooked.verify_key,
            organization_internal_id=0,  # Only used by PostgreSQL implementation
            device_internal_id=0,  # Only used by PostgreSQL implementation
            organization_allowed_client_agent=org.allowed_client_agent,
        )

    @override
    async def authenticated_account_auth(
        self,
        now: DateTime,
        token: AccountAuthenticationToken,
    ) -> AuthenticatedAccountAuthInfo | AuthAuthenticatedAccountAuthBadOutcome:
        match self._data.get_account_from_active_auth_method(auth_method_id=token.auth_method_id):
            case (account, auth_method):
                pass
            case None:
                return AuthAuthenticatedAccountAuthBadOutcome.ACCOUNT_NOT_FOUND

        if not token.verify(auth_method.mac_key):
            return AuthAuthenticatedAccountAuthBadOutcome.INVALID_TOKEN

        if timestamps_in_the_ballpark(token.timestamp, now) is not None:
            return AuthAuthenticatedAccountAuthBadOutcome.TOKEN_OUT_OF_BALLPARK

        return AuthenticatedAccountAuthInfo(
            account_email=account.account_email,
            auth_method_id=auth_method.id,
            account_internal_id=0,  # Only used by PostgreSQL implementation
            auth_method_internal_id=0,  # Only used by PostgreSQL implementation
        )
