# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Any

from parsec._parsec import DateTime, DeviceID, InvitationToken, OrganizationID
from parsec.components.auth import (
    AnonymousAuthInfo,
    AuthAnonymousAuthBadOutcome,
    AuthAuthenticatedAuthBadOutcome,
    AuthenticatedAuthInfo,
    AuthInvitedAuthBadOutcome,
    BaseAuthComponent,
    InvitedAuthInfo,
)
from parsec.components.memory.datamodel import MemoryDatamodel, MemoryOrganization


class MemoryAuthComponent(BaseAuthComponent):
    def __init__(self, data: MemoryDatamodel, *args: Any, **kwargs: Any) -> None:
        super().__init__(*args, **kwargs)
        self._data = data

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
        )

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
        )

    async def _get_authenticated_info(
        self, organization_id: OrganizationID, device_id: DeviceID
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

        return AuthenticatedAuthInfo(
            organization_id=organization_id,
            user_id=user_id,
            device_id=device_id,
            device_verify_key=device.cooked.verify_key,
            organization_internal_id=0,  # Only used by PostgreSQL implementation
            device_internal_id=0,  # Only used by PostgreSQL implementation
        )
