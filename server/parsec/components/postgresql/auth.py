# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import assert_never, override

import asyncpg

from parsec._parsec import DateTime, DeviceCertificate, DeviceID, InvitationToken, OrganizationID
from parsec.components.auth import (
    AnonymousAuthInfo,
    AuthAnonymousAuthBadOutcome,
    AuthAuthenticatedAuthBadOutcome,
    AuthenticatedAuthInfo,
    AuthInvitedAuthBadOutcome,
    BaseAuthComponent,
    InvitedAuthInfo,
)
from parsec.components.invite import DeviceInvitation, UserInvitation
from parsec.components.organization import Organization, OrganizationGetBadOutcome
from parsec.components.postgresql.invite import InviteAsInvitedInfoBadOutcome, PGInviteComponent
from parsec.components.postgresql.organization import PGOrganizationComponent
from parsec.components.postgresql.user_queries.get import query_check_user_for_authentication
from parsec.components.postgresql.utils import transaction
from parsec.components.user import CheckUserWithDeviceBadOutcome
from parsec.config import BackendConfig


class PGAuthComponent(BaseAuthComponent):
    def __init__(self, pool: asyncpg.Pool, config: BackendConfig) -> None:
        super().__init__(config)
        self.pool = pool
        self._organization: PGOrganizationComponent
        self._invite: PGInviteComponent

    def register_components(
        self, organization: PGOrganizationComponent, invite: PGInviteComponent, **kwargs
    ) -> None:
        self._organization = organization
        self._invite = invite

    @override
    @transaction
    async def anonymous_auth(
        self,
        conn: asyncpg.Connection,
        now: DateTime,
        organization_id: OrganizationID,
        spontaneous_bootstrap: bool,
    ) -> AnonymousAuthInfo | AuthAnonymousAuthBadOutcome:
        match await self._organization._get(conn, organization_id):
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                if spontaneous_bootstrap:
                    raise NotImplementedError
                return AuthAnonymousAuthBadOutcome.ORGANIZATION_NOT_FOUND
            case Organization() as organization:
                pass
            case unknown:
                assert_never(unknown)

        if organization.is_expired:
            return AuthAnonymousAuthBadOutcome.ORGANIZATION_EXPIRED

        return AnonymousAuthInfo(
            organization_id=organization_id,
            organization_internal_id=0,  # Unused at the moment
        )

    @override
    @transaction
    async def invited_auth(
        self,
        conn: asyncpg.Connection,
        now: DateTime,
        organization_id: OrganizationID,
        token: InvitationToken,
    ) -> InvitedAuthInfo | AuthInvitedAuthBadOutcome:
        match await self._invite._info_as_invited(conn, organization_id, token):
            case InviteAsInvitedInfoBadOutcome.ORGANIZATION_NOT_FOUND:
                return AuthInvitedAuthBadOutcome.ORGANIZATION_NOT_FOUND
            case InviteAsInvitedInfoBadOutcome.ORGANIZATION_EXPIRED:
                return AuthInvitedAuthBadOutcome.ORGANIZATION_EXPIRED
            case InviteAsInvitedInfoBadOutcome.INVITATION_NOT_FOUND:
                return AuthInvitedAuthBadOutcome.INVITATION_NOT_FOUND
            case InviteAsInvitedInfoBadOutcome.INVITATION_DELETED:
                return AuthInvitedAuthBadOutcome.INVITATION_ALREADY_USED
            case DeviceInvitation() | UserInvitation() as invitation:
                pass
            case unknown:
                assert_never(unknown)
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
        conn: asyncpg.Connection,
        organization_id: OrganizationID,
        device_id: DeviceID,
    ) -> AuthenticatedAuthInfo | AuthAuthenticatedAuthBadOutcome:
        match await self._organization._get(conn, organization_id):
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return AuthAuthenticatedAuthBadOutcome.ORGANIZATION_NOT_FOUND
            case Organization() as organization:
                pass
            case unknown:
                assert_never(unknown)

        if organization.is_expired:
            return AuthAuthenticatedAuthBadOutcome.ORGANIZATION_EXPIRED

        match await query_check_user_for_authentication(conn, organization_id, device_id):
            case CheckUserWithDeviceBadOutcome.DEVICE_NOT_FOUND:
                return AuthAuthenticatedAuthBadOutcome.DEVICE_NOT_FOUND
            case CheckUserWithDeviceBadOutcome.USER_NOT_FOUND:
                return AuthAuthenticatedAuthBadOutcome.DEVICE_NOT_FOUND
            case CheckUserWithDeviceBadOutcome.USER_REVOKED:
                return AuthAuthenticatedAuthBadOutcome.USER_REVOKED
            case DeviceCertificate() as certificate:
                pass
            case unknown:
                assert_never(unknown)

        return AuthenticatedAuthInfo(
            organization_id=organization_id,
            device_id=device_id,
            device_verify_key=certificate.verify_key,
            organization_internal_id=0,  # Only used by PostgreSQL implementation
            device_internal_id=0,  # Only used by PostgreSQL implementation
        )
