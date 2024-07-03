# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import override

from parsec._parsec import (
    DateTime,
    DeviceID,
    InvitationToken,
    OrganizationID,
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
from parsec.components.invite import DeviceInvitation, UserInvitation
from parsec.components.organization import Organization, OrganizationGetBadOutcome
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.invite import InviteAsInvitedInfoBadOutcome, PGInviteComponent
from parsec.components.postgresql.organization import PGOrganizationComponent
from parsec.components.postgresql.user import PGUserComponent
from parsec.components.postgresql.utils import transaction
from parsec.components.user import CheckUserForAuthenticationBadOutcome
from parsec.config import BackendConfig


class PGAuthComponent(BaseAuthComponent):
    def __init__(self, pool: AsyncpgPool, event_bus: EventBus, config: BackendConfig) -> None:
        super().__init__(event_bus, config)
        self.pool = pool
        self.organization: PGOrganizationComponent
        self.invite: PGInviteComponent
        self.user: PGUserComponent

    def register_components(
        self,
        organization: PGOrganizationComponent,
        invite: PGInviteComponent,
        user: PGUserComponent,
        **kwargs,
    ) -> None:
        self.organization = organization
        self.invite = invite
        self.user = user

    @override
    @transaction
    async def anonymous_auth(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        spontaneous_bootstrap: bool,
    ) -> AnonymousAuthInfo | AuthAnonymousAuthBadOutcome:
        match await self.organization._get(conn, organization_id):
            case Organization() as organization:
                is_expired = organization.is_expired
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                if not spontaneous_bootstrap:
                    return AuthAnonymousAuthBadOutcome.ORGANIZATION_NOT_FOUND
                await self.organization.spontaneous_create(conn, organization_id, now=now)
                is_expired = False

        if is_expired:
            return AuthAnonymousAuthBadOutcome.ORGANIZATION_EXPIRED

        return AnonymousAuthInfo(
            organization_id=organization_id,
            organization_internal_id=0,  # Unused at the moment
        )

    @override
    @transaction
    async def invited_auth(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        token: InvitationToken,
    ) -> InvitedAuthInfo | AuthInvitedAuthBadOutcome:
        match await self.invite._info_as_invited(conn, organization_id, token):
            case DeviceInvitation() | UserInvitation() as invitation:
                pass
            case InviteAsInvitedInfoBadOutcome.ORGANIZATION_NOT_FOUND:
                return AuthInvitedAuthBadOutcome.ORGANIZATION_NOT_FOUND
            case InviteAsInvitedInfoBadOutcome.ORGANIZATION_EXPIRED:
                return AuthInvitedAuthBadOutcome.ORGANIZATION_EXPIRED
            case InviteAsInvitedInfoBadOutcome.INVITATION_NOT_FOUND:
                return AuthInvitedAuthBadOutcome.INVITATION_NOT_FOUND
            case InviteAsInvitedInfoBadOutcome.INVITATION_DELETED:
                return AuthInvitedAuthBadOutcome.INVITATION_ALREADY_USED

        return InvitedAuthInfo(
            organization_id=organization_id,
            token=token,
            type=invitation.TYPE,
            organization_internal_id=0,  # Only used by PostgreSQL implementation
            invitation_internal_id=0,  # Only used by PostgreSQL implementation
        )

    @override
    @transaction
    async def _get_authenticated_info(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        device_id: DeviceID,
    ) -> AuthenticatedAuthInfo | AuthAuthenticatedAuthBadOutcome:
        match await self.organization._get(conn, organization_id):
            case Organization() as organization:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return AuthAuthenticatedAuthBadOutcome.ORGANIZATION_NOT_FOUND

        if organization.is_expired:
            return AuthAuthenticatedAuthBadOutcome.ORGANIZATION_EXPIRED

        match await self.user._check_user_for_authentication(conn, organization_id, device_id):
            case (user_id, verify_key):
                pass
            case CheckUserForAuthenticationBadOutcome.DEVICE_NOT_FOUND:
                return AuthAuthenticatedAuthBadOutcome.DEVICE_NOT_FOUND
            case CheckUserForAuthenticationBadOutcome.USER_NOT_FOUND:
                return AuthAuthenticatedAuthBadOutcome.DEVICE_NOT_FOUND
            case CheckUserForAuthenticationBadOutcome.USER_REVOKED:
                return AuthAuthenticatedAuthBadOutcome.USER_REVOKED
            case CheckUserForAuthenticationBadOutcome.USER_FROZEN:
                return AuthAuthenticatedAuthBadOutcome.USER_FROZEN

        return AuthenticatedAuthInfo(
            organization_id=organization_id,
            user_id=user_id,
            device_id=device_id,
            device_verify_key=verify_key,
            organization_internal_id=0,  # Only used by PostgreSQL implementation
            device_internal_id=0,  # Only used by PostgreSQL implementation
        )
