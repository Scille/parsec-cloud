# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from collections import defaultdict
from copy import deepcopy
from typing import TYPE_CHECKING, Any, Callable, Coroutine, Union

import trio

from parsec._parsec import (
    ActiveUsersLimit,
    DateTime,
    OrganizationStats,
    UsersPerProfileDetailItem,
    VerifyKey,
)
from parsec.api.protocol import OrganizationID, UserProfile
from parsec.backend.events import BackendEvent
from parsec.backend.organization import (
    BaseOrganizationComponent,
    Organization,
    OrganizationAlreadyBootstrappedError,
    OrganizationAlreadyExistsError,
    OrganizationFirstUserCreationError,
    OrganizationInvalidBootstrapTokenError,
    OrganizationNotFoundError,
    SequesterAuthority,
)
from parsec.backend.user import Device, User, UserError
from parsec.backend.utils import Unset, UnsetType

if TYPE_CHECKING:
    from parsec.backend.memory.block import MemoryBlockComponent
    from parsec.backend.memory.realm import MemoryRealmComponent
    from parsec.backend.memory.user import MemoryUserComponent
    from parsec.backend.memory.vlob import MemoryVlobComponent


class MemoryOrganizationComponent(BaseOrganizationComponent):
    def __init__(
        self, send_event: Callable[..., Coroutine[Any, Any, None]], *args: Any, **kwargs: Any
    ) -> None:
        super().__init__(*args, **kwargs)
        self._user_component: MemoryUserComponent | None = None
        self._vlob_component: MemoryVlobComponent | None = None
        self._block_component: MemoryBlockComponent | None = None
        self._realm_component: MemoryRealmComponent | None = None
        self._organizations: dict[OrganizationID, Organization] = {}
        self._send_event = send_event
        self._organization_bootstrap_lock: dict[OrganizationID, trio.Lock] = defaultdict(trio.Lock)

    def register_components(
        self,
        user: MemoryUserComponent,
        vlob: MemoryVlobComponent,
        block: MemoryBlockComponent,
        realm: MemoryRealmComponent,
        **other_components: Any,
    ) -> None:
        self._user_component = user
        self._vlob_component = vlob
        self._block_component = block
        self._realm_component = realm

    async def create(
        self,
        id: OrganizationID,
        bootstrap_token: str,
        active_users_limit: Union[UnsetType, ActiveUsersLimit] = Unset,
        user_profile_outsider_allowed: Union[UnsetType, bool] = Unset,
        created_on: DateTime | None = None,
    ) -> None:
        created_on = created_on or DateTime.now()
        org = self._organizations.get(id)
        # Allow overwriting of not-yet-bootstrapped organization
        if org and org.root_verify_key:
            raise OrganizationAlreadyExistsError()
        if active_users_limit is Unset:
            active_users_limit = self._config.organization_initial_active_users_limit
        if user_profile_outsider_allowed is Unset:
            user_profile_outsider_allowed = (
                self._config.organization_initial_user_profile_outsider_allowed
            )

        assert isinstance(active_users_limit, ActiveUsersLimit)

        self._organizations[id] = Organization(
            organization_id=id,
            bootstrap_token=bootstrap_token,
            is_expired=False,
            created_on=created_on,
            bootstrapped_on=None,
            root_verify_key=None,
            active_users_limit=active_users_limit,
            user_profile_outsider_allowed=user_profile_outsider_allowed,
            sequester_authority=None,
            sequester_services_certificates=None,
        )

    async def get(self, id: OrganizationID) -> Organization:
        if id not in self._organizations:
            raise OrganizationNotFoundError()
        return self._organizations[id]

    async def bootstrap(
        self,
        id: OrganizationID,
        user: User,
        first_device: Device,
        bootstrap_token: str,
        root_verify_key: VerifyKey,
        bootstrapped_on: DateTime | None = None,
        sequester_authority: SequesterAuthority | None = None,
    ) -> None:
        assert self._user_component is not None

        # Organization bootstrap involves multiple modifications (in user,
        # device and organization) and is not atomic (given await is used),
        # so we protect it from concurrency with a big old lock
        async with self._organization_bootstrap_lock[id]:

            organization = await self.get(id)

            if organization.is_bootstrapped():
                raise OrganizationAlreadyBootstrappedError()

            if organization.bootstrap_token != bootstrap_token:
                raise OrganizationInvalidBootstrapTokenError()

            try:
                await self._user_component.create_user(id, user, first_device)

            except UserError as exc:
                raise OrganizationFirstUserCreationError(exc) from exc

            self._organizations[organization.organization_id] = organization.evolve(
                bootstrapped_on=bootstrapped_on,
                root_verify_key=root_verify_key,
            )
            if sequester_authority:
                self._organizations[organization.organization_id] = self._organizations[
                    organization.organization_id
                ].evolve(
                    sequester_authority=sequester_authority, sequester_services_certificates=()
                )

    async def stats(
        self,
        id: OrganizationID,
        at: DateTime | None = None,
    ) -> OrganizationStats:
        assert self._vlob_component is not None
        assert self._block_component is not None
        assert self._user_component is not None
        assert self._realm_component is not None
        at = at or DateTime.now()

        org = await self.get(id)
        if org.created_on > at:
            raise OrganizationNotFoundError

        metadata_size = 0
        for (vlob_organization_id, _), vlob in self._vlob_component._vlobs.items():
            if vlob_organization_id == id:
                metadata_size += sum(len(blob) for (blob, _, ts) in vlob.data if ts <= at)

        data_size = 0
        for (vlob_organization_id, _), blockmeta in self._block_component._blockmetas.items():
            if vlob_organization_id == id and (blockmeta.created_on <= at):
                data_size += blockmeta.size

        users = 0
        active_users = 0
        users_per_profile_detail = {p: {"active": 0, "revoked": 0} for p in UserProfile.VALUES}
        for user in self._user_component._organizations[id].users.values():
            if user.created_on <= at:
                users += 1
                if user.revoked_on and user.revoked_on <= at:
                    users_per_profile_detail[user.profile]["revoked"] += 1
                else:
                    users_per_profile_detail[user.profile]["active"] += 1
                    active_users += 1

        realms = 0
        for (organization_id, _), realm in self._realm_component._realms.items():
            if organization_id == id and realm.created_on <= at:
                realms += 1

        users_per_profile_detail = tuple(
            UsersPerProfileDetailItem(profile=profile, **data)
            for profile, data in users_per_profile_detail.items()
        )

        return OrganizationStats(
            data_size=data_size,
            metadata_size=metadata_size,
            realms=realms,
            users=users,
            active_users=active_users,
            users_per_profile_detail=users_per_profile_detail,
        )

    async def server_stats(
        self, at: DateTime | None = None
    ) -> dict[OrganizationID, OrganizationStats]:
        at = at or DateTime.now()
        result = {}
        for org_id in self._organizations.keys():
            try:
                result[org_id] = await self.stats(org_id, at)
            except OrganizationNotFoundError:
                # Organization didn't not exist at the considered time, just ignore it
                pass

        return result

    async def update(
        self,
        id: OrganizationID,
        is_expired: Union[UnsetType, bool] = Unset,
        active_users_limit: Union[UnsetType, ActiveUsersLimit] = Unset,
        user_profile_outsider_allowed: Union[UnsetType, bool] = Unset,
    ) -> None:
        """
        Raises:
            OrganizationNotFoundError
        """
        if id not in self._organizations:
            raise OrganizationNotFoundError()

        organization = self._organizations[id]

        if is_expired is not Unset:
            organization = organization.evolve(is_expired=is_expired)
        if active_users_limit is not Unset:
            organization = organization.evolve(active_users_limit=active_users_limit)
        if user_profile_outsider_allowed is not Unset:
            organization = organization.evolve(
                user_profile_outsider_allowed=user_profile_outsider_allowed
            )

        assert isinstance(organization.active_users_limit, ActiveUsersLimit)
        self._organizations[id] = organization

        if self._organizations[id].is_expired:
            await self._send_event(BackendEvent.ORGANIZATION_EXPIRED, organization_id=id)

    def test_duplicate_organization(self, id: OrganizationID, new_id: OrganizationID) -> None:
        self._organizations[new_id] = deepcopy(self._organizations[id])

    def test_drop_organization(self, id: OrganizationID) -> None:
        self._organizations.pop(id, None)
