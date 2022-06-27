# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

from typing import TYPE_CHECKING, Optional, Union, Dict
import trio
from collections import defaultdict

from parsec.api.protocol import OrganizationID
from parsec.api.data.certif import UserProfile
from parsec.crypto import VerifyKey
from parsec.backend.user import UserError, User, Device
from parsec.backend.organization import (
    BaseOrganizationComponent,
    Organization,
    SequesterAuthority,
    OrganizationStats,
    OrganizationAlreadyExistsError,
    OrganizationInvalidBootstrapTokenError,
    OrganizationAlreadyBootstrappedError,
    OrganizationNotFoundError,
    OrganizationFirstUserCreationError,
    UsersPerProfileDetailItem,
)
from parsec.backend.utils import Unset, UnsetType
from parsec.backend.events import BackendEvent

if TYPE_CHECKING:
    from parsec.backend.memory.user import MemoryUserComponent
    from parsec.backend.memory.vlob import MemoryVlobComponent
    from parsec.backend.memory.block import MemoryBlockComponent
    from parsec.backend.memory.realm import MemoryRealmComponent


class MemoryOrganizationComponent(BaseOrganizationComponent):
    def __init__(self, send_event, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self._user_component: "MemoryUserComponent" = None
        self._vlob_component: "MemoryVlobComponent" = None
        self._block_component: "MemoryBlockComponent" = None
        self._realm_component: "MemoryRealmComponent" = None
        self._organizations: Dict[OrganizationID, Organization] = {}
        self._send_event = send_event
        self._organization_bootstrap_lock = defaultdict(trio.Lock)

    def register_components(
        self,
        user: "MemoryUserComponent",
        vlob: "MemoryVlobComponent",
        block: "MemoryBlockComponent",
        realm: "MemoryRealmComponent",
        **other_components
    ):
        self._user_component = user
        self._vlob_component = vlob
        self._block_component = block
        self._realm_component = realm

    async def create(
        self,
        id: OrganizationID,
        bootstrap_token: str,
        active_users_limit: Union[UnsetType, Optional[int]] = Unset,
        user_profile_outsider_allowed: Union[UnsetType, bool] = Unset,
    ) -> None:
        org = self._organizations.get(id)
        # Allow overwritting of not-yet-bootstrapped organization
        if org and org.root_verify_key:
            raise OrganizationAlreadyExistsError()
        if active_users_limit is Unset:
            active_users_limit = self._config.organization_initial_active_users_limit
        if user_profile_outsider_allowed is Unset:
            user_profile_outsider_allowed = (
                self._config.organization_initial_user_profile_outsider_allowed
            )

        self._organizations[id] = Organization(
            organization_id=id,
            bootstrap_token=bootstrap_token,
            is_expired=False,
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
        sequester_authority: Optional[SequesterAuthority] = None,
    ) -> None:
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
                root_verify_key=root_verify_key
            )
            if sequester_authority:
                self._organizations[organization.organization_id] = self._organizations[
                    organization.organization_id
                ].evolve(
                    sequester_authority=sequester_authority, sequester_services_certificates=()
                )

    async def stats(self, id: OrganizationID) -> OrganizationStats:
        await self.get(id)

        metadata_size = 0
        for (vlob_organization_id, _), vlob in self._vlob_component._vlobs.items():
            if vlob_organization_id == id:
                metadata_size += sum(len(blob) for (blob, _, _) in vlob.data)

        data_size = 0
        for (vlob_organization_id, _), blockmeta in self._block_component._blockmetas.items():
            if vlob_organization_id == id:
                data_size += blockmeta.size

        users = 0
        active_users = 0
        users_per_profile_detail = {p: {"active": 0, "revoked": 0} for p in UserProfile}
        for user in self._user_component._organizations[id].users.values():
            users += 1
            if user.revoked_on:
                users_per_profile_detail[user.profile]["revoked"] += 1
            else:
                users_per_profile_detail[user.profile]["active"] += 1
                active_users += 1

        realms = len(
            [
                realm_id
                for organization_id, realm_id in self._realm_component._realms.keys()
                if organization_id == id
            ]
        )
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

    async def update(
        self,
        id: OrganizationID,
        is_expired: Union[UnsetType, bool] = Unset,
        active_users_limit: Union[UnsetType, Optional[int]] = Unset,
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

        self._organizations[id] = organization

        if self._organizations[id].is_expired:
            await self._send_event(BackendEvent.ORGANIZATION_EXPIRED, organization_id=id)
