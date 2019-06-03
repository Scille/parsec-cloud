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
    RealmEncryptionRevisionError,
    RealmMaintenanceError,
    RealmInMaintenanceError,
)
from parsec.backend.drivers.memory.user import MemoryUserComponent, UserNotFoundError
from parsec.backend.drivers.memory.message import MemoryMessageComponent


@attr.s
class Realm:
    status: RealmStatus = attr.ib(factory=lambda: RealmStatus(None, None, None, 1))
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
        timestamp: pendulum.Pendulum,
    ) -> None:
        realm = self._get_realm(organization_id, realm_id)
        if realm.roles.get(author.user_id) != RealmRole.OWNER:
            raise RealmAccessError()
        if realm.status.in_maintenance:
            raise RealmInMaintenanceError(f"Realm `{realm_id}` alrealy in maintenance")
        if encryption_revision != realm.status.encryption_revision + 1:
            raise RealmEncryptionRevisionError("Invalid encryption revision")
        if per_participant_message.keys() ^ realm.roles.keys():
            raise RealmMaintenanceError("Realm participants and message recipients mismatch")

        realm.status = RealmStatus(
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

        self.event_bus.send(
            "realm.maintenance_started",
            organization_id=organization_id,
            author=author,
            realm_id=realm_id,
            encryption_revision=encryption_revision,
        )

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
            raise RealmMaintenanceError(f"Realm `{realm_id}` not under maintenance")
        if encryption_revision != realm.status.encryption_revision:
            raise RealmEncryptionRevisionError("Invalid encryption revision")
        if (
            self._maintenance_reencryption_is_finished_hook
            and not self._maintenance_reencryption_is_finished_hook(
                organization_id, realm_id, encryption_revision
            )
        ):
            raise RealmMaintenanceError("Reencryption operations are not over")

        realm.status = RealmStatus(
            maintenance_type=None,
            maintenance_started_on=None,
            maintenance_started_by=None,
            encryption_revision=encryption_revision,
        )

        self.event_bus.send(
            "realm.maintenance_finished",
            organization_id=organization_id,
            author=author,
            realm_id=realm_id,
            encryption_revision=encryption_revision,
        )
