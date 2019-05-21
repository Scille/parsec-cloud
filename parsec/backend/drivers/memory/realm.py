# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
import pendulum
from uuid import UUID
from typing import Dict, Optional

from parsec.api.protocole import RealmRole
from parsec.types import DeviceID, UserID, OrganizationID
from parsec.event_bus import EventBus
from parsec.backend.realm import (
    MaintenanceType,
    BaseRealmComponent,
    RealmStatus,
    RealmAccessError,
    RealmAlreadyExistsError,
    RealmNotFoundError,
    RealmMaintenanceError,
    RealmInMaintenanceError,
)
from parsec.backend.drivers.memory.user import MemoryUserComponent, UserNotFoundError
from parsec.backend.drivers.memory.message import MemoryMessageComponent


@attr.s
class Realm:
    status: RealmStatus = attr.ib(factory=lambda: RealmStatus(False, None, None, None, 1))
    checkpoint: int = attr.ib(default=0)
    roles: Dict[UserID, RealmRole] = attr.ib(factory=dict)


class MemoryRealmComponent(BaseRealmComponent):
    def __init__(
        self,
        event_bus: EventBus,
        user_component: MemoryUserComponent,
        message_component: MemoryMessageComponent,
    ):
        self.event_bus = event_bus
        self._user_component = user_component
        self._message_component = message_component
        self._realms = {}
        self._maintenance_reencryption_start_hook = None
        self._maintenance_reencryption_is_finished_hook = None

    # Semi-private methods to avoid recursive dependencies with vlob component
    def _register_maintenance_reencryption_hooks(self, start, is_finished):
        self._maintenance_reencryption_start_hook = start
        self._maintenance_reencryption_is_finished_hook = is_finished

    # Semi-private method used by memory vlob component
    def _create_realm(self, organization_id: OrganizationID, realm_id: UUID, author: DeviceID):
        key = (organization_id, realm_id)
        if key not in self._realms:
            self._realms[key] = Realm(roles={author.user_id: RealmRole.OWNER})

        else:
            raise RealmAlreadyExistsError()

    # Semi-private method used by memory vlob&block components
    def _check_access_and_maintenance(
        self,
        organization_id,
        realm_id,
        user_id,
        allowed_roles,
        not_found_exc=None,
        access_error_exc=None,
        in_maintenance_exc=None,
    ):
        try:
            realm = self._get_realm(organization_id, realm_id)
        except RealmNotFoundError:
            if not_found_exc:
                raise not_found_exc(f"Realm {realm_id} doesn't exist")
            else:
                return False

        if realm.roles.get(user_id) not in allowed_roles:
            if access_error_exc:
                raise access_error_exc()
            else:
                return False

        if realm.status.in_maintenance:
            if in_maintenance_exc:
                raise in_maintenance_exc(f"Realm {realm_id} is currently under maintenance")
            else:
                return False

        return True

    # Semi-private method used by memory vlob&block components
    def _check_read_access_and_maintenance(self, organization_id, realm_id, user_id, **kwargs):
        can_read_roles = (
            RealmRole.OWNER,
            RealmRole.MANAGER,
            RealmRole.CONTRIBUTOR,
            RealmRole.READER,
        )
        return self._check_access_and_maintenance(
            organization_id, realm_id, user_id, can_read_roles, **kwargs
        )

    # Semi-private method used by memory vlob&block components
    def _check_write_access_and_maintenance(self, organization_id, realm_id, user_id, **kwargs):
        can_write_roles = (RealmRole.OWNER, RealmRole.MANAGER, RealmRole.CONTRIBUTOR)
        return self._check_access_and_maintenance(
            organization_id, realm_id, user_id, can_write_roles, **kwargs
        )

    # Semi-private method used by memory vlob&block components
    def _can_read(self, organization_id: OrganizationID, realm_id: UUID, user_id: UserID):
        can_read_roles = (
            RealmRole.OWNER,
            RealmRole.MANAGER,
            RealmRole.CONTRIBUTOR,
            RealmRole.READER,
        )
        return self._has_role(organization_id, realm_id, user_id, can_read_roles)

    # Semi-private method used by memory vlob&block components
    def _can_write(self, organization_id, realm_id, user_id):
        can_write_roles = (RealmRole.OWNER, RealmRole.MANAGER, RealmRole.CONTRIBUTOR)
        return self._has_role(organization_id, realm_id, user_id, can_write_roles)

    def _has_role(self, organization_id, realm_id, user_id, roles):
        try:
            realm = self._get_realm(organization_id, realm_id)
            return realm.roles[user_id] in roles

        except (RealmNotFoundError, KeyError):
            return False

    def _get_realm(self, organization_id, realm_id):
        try:
            return self._realms[(organization_id, realm_id)]
        except KeyError:
            raise RealmNotFoundError(f"Realm `{realm_id}` doesn't exist")

    async def get_status(
        self, organization_id: OrganizationID, author: DeviceID, realm_id: UUID
    ) -> RealmStatus:
        realm = self._get_realm(organization_id, realm_id)
        if author.user_id not in realm.roles:
            raise RealmAccessError()
        return realm.status

    async def get_roles(
        self, organization_id: OrganizationID, author: DeviceID, realm_id: UUID
    ) -> Dict[DeviceID, RealmRole]:
        realm = self._get_realm(organization_id, realm_id)
        if author.user_id not in realm.roles:
            raise RealmAccessError()
        return realm.roles.copy()

    async def update_roles(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: UUID,
        user: UserID,
        role: Optional[RealmRole],
    ) -> None:
        if author.user_id == user:
            raise RealmAccessError("Cannot modify our own role")

        try:
            self._user_component._get_user(organization_id, user)
        except UserNotFoundError:
            raise RealmNotFoundError(f"User `{user}` doesn't exist")

        realm = self._get_realm(organization_id, realm_id)

        if realm.status.in_maintenance:
            raise RealmInMaintenanceError("Data realm is currently under maintenance")

        existing_user_role = realm.roles.get(user)
        if existing_user_role in (RealmRole.MANAGER, RealmRole.OWNER):
            needed_roles = (RealmRole.OWNER,)
        else:
            needed_roles = (RealmRole.MANAGER, RealmRole.OWNER)

        author_role = realm.roles.get(author.user_id)
        if author_role not in needed_roles:
            raise RealmAccessError()

        if not role:
            realm.roles.pop(user, None)
        else:
            realm.roles[user] = role

    async def start_reencryption_maintenance(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: UUID,
        encryption_revision: int,
        per_participant_message: Dict[UserID, bytes],
    ) -> None:
        realm = self._get_realm(organization_id, realm_id)
        if realm.roles.get(author.user_id) != RealmRole.OWNER:
            raise RealmAccessError()
        if realm.status.in_maintenance:
            raise RealmInMaintenanceError("Realm already in maintenance")
        if encryption_revision != realm.status.encryption_revision + 1:
            raise RealmMaintenanceError("Invalid encryption revision")
        if per_participant_message.keys() ^ realm.roles.keys():
            raise RealmMaintenanceError("Realm participants and message recipients mismatch")

        timestamp = pendulum.now()
        realm.status = RealmStatus(
            in_maintenance=True,
            maintenance_type=MaintenanceType.REENCRYPTION,
            maintenance_started_on=timestamp,
            maintenance_started_by=author,
            encryption_revision=encryption_revision,
        )
        if self._maintenance_reencryption_start_hook:
            self._maintenance_reencryption_start_hook(
                organization_id, realm_id, encryption_revision
            )
        for recipient, msg in per_participant_message.items():
            await self._message_component.send(organization_id, author, recipient, timestamp, msg)

    async def finish_reencryption_maintenance(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: UUID,
        encryption_revision: int,
    ) -> None:
        realm = self._get_realm(organization_id, realm_id)
        if realm.roles.get(author.user_id) != RealmRole.OWNER:
            raise RealmAccessError()
        if not realm.status.in_maintenance:
            raise RealmMaintenanceError("Realm not currently in maintenance")
        if encryption_revision != realm.status.encryption_revision:
            raise RealmMaintenanceError("Invalid encryption revision")
        if (
            self._maintenance_reencryption_is_finished_hook
            and not self._maintenance_reencryption_is_finished_hook(
                organization_id, realm_id, encryption_revision
            )
        ):
            raise RealmMaintenanceError("Reencryption operations are not over")

        realm.status = RealmStatus(
            in_maintenance=False,
            maintenance_type=None,
            maintenance_started_on=None,
            maintenance_started_by=None,
            encryption_revision=encryption_revision,
        )
