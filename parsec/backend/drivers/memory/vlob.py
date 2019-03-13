# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
import pendulum
from uuid import UUID
from typing import List, Tuple, Dict
from collections import defaultdict

from parsec.types import DeviceID, UserID, OrganizationID
from parsec.event_bus import EventBus
from parsec.backend.drivers.memory.user import MemoryUserComponent, UserNotFoundError
from parsec.backend.vlob import (
    BaseVlobComponent,
    VlobAccessError,
    VlobVersionError,
    VlobNotFoundError,
    VlobAlreadyExistsError,
)


@attr.s
class Vlob:
    group = attr.ib()
    data = attr.ib(factory=list)

    @property
    def current_version(self):
        return len(self.data)


@attr.s
class VlobGroup:
    checkpoint = attr.ib(default=0)
    rights = attr.ib(factory=dict)
    changes = attr.ib(factory=dict)


class MemoryVlobComponent(BaseVlobComponent):
    def __init__(self, event_bus: EventBus, user_component: MemoryUserComponent):
        self.event_bus = event_bus
        self._user_component = user_component
        self._organizations = defaultdict(lambda: ({}, {}))

    def _get_vlob(self, organization, id):
        vlobs, _ = self._organizations[organization]

        try:
            return vlobs[id]

        except KeyError:
            raise VlobNotFoundError(f"Vlob `{id}` doesn't exist")

    def _get_group(self, organization, group_id):
        _, groups = self._organizations[organization]

        try:
            return groups[group_id]

        except KeyError:
            raise VlobNotFoundError(f"Group `{group_id}` doesn't exist")

    def _can_read(self, organization, user, group_id):
        try:
            group = self._get_group(organization, group_id)
            return group.rights[user][1]

        except (VlobNotFoundError, KeyError):
            return False

    def _can_write(self, organization, user, group_id):
        try:
            group = self._get_group(organization, group_id)
            return group.rights[user][2]

        except (VlobNotFoundError, KeyError):
            return False

    def _create_group_if_needed(self, organization, id, author):
        _, groups = self._organizations[organization]
        if id in groups:
            return
        groups[id] = VlobGroup(rights={author: (True, True, True)})

    def _update_group(self, organization_id, author, id, src_id, src_version=1):
        group = self._get_group(organization_id, id)
        group.checkpoint += 1
        group.changes[src_id] = (author, group.checkpoint, src_version)
        self.event_bus.send(
            "vlob_group.updated",
            organization_id=organization_id,
            author=author,
            id=id,
            checkpoint=group.checkpoint,
            src_id=src_id,
            src_version=src_version,
        )

    async def group_check(
        self, organization_id: OrganizationID, author: UserID, to_check: List[dict]
    ) -> List[dict]:
        changed = []
        for item in to_check:
            id = item["id"]
            version = item["version"]
            if version == 0:
                changed.append({"id": id, "version": version})
            else:
                try:
                    vlob = self._get_vlob(organization_id, id)
                except VlobNotFoundError:
                    continue

                if not self._can_read(organization_id, author, vlob.group):
                    continue

                if vlob.current_version != version:
                    changed.append({"id": id, "version": vlob.current_version})

        return changed

    async def create(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        id: UUID,
        group: UUID,
        timestamp: pendulum.Pendulum,
        blob: bytes,
    ) -> None:
        self._create_group_if_needed(organization_id, group, author.user_id)
        if not self._can_write(organization_id, author.user_id, group):
            raise VlobAccessError()

        vlobs, _ = self._organizations[organization_id]

        if id in vlobs:
            raise VlobAlreadyExistsError()

        vlobs[id] = Vlob(group, [(blob, author, timestamp)])

        self._update_group(organization_id, author, group, id)

    async def read(
        self, organization_id: OrganizationID, author: UserID, id: UUID, version: int = None
    ) -> Tuple[int, bytes, DeviceID, pendulum.Pendulum]:
        vlob = self._get_vlob(organization_id, id)

        if not self._can_read(organization_id, author, vlob.group):
            raise VlobAccessError()

        if version is None:
            version = vlob.current_version
        try:
            return (version, *vlob.data[version - 1])

        except IndexError:
            raise VlobVersionError()

    async def update(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        id: UUID,
        version: int,
        timestamp: pendulum.Pendulum,
        blob: bytes,
    ) -> None:
        vlob = self._get_vlob(organization_id, id)

        if not self._can_write(organization_id, author.user_id, vlob.group):
            raise VlobAccessError()

        if version - 1 == vlob.current_version:
            vlob.data.append((blob, author, timestamp))
        else:
            raise VlobVersionError()

        self._update_group(organization_id, author, vlob.group, id, version)

    async def get_group_rights(
        self, organization_id: OrganizationID, author: UserID, id: UUID
    ) -> Dict[DeviceID, Tuple[bool, bool, bool]]:
        group = self._get_group(organization_id, id)
        if not group.rights.get(author, (False, False, False))[1]:
            raise VlobAccessError()
        return group.rights.copy()

    async def update_group_rights(
        self,
        organization_id: OrganizationID,
        author: UserID,
        id: UUID,
        user: UserID,
        admin_right: bool,
        read_right: bool,
        write_right: bool,
    ) -> None:
        try:
            self._user_component._get_user(organization_id, user)
        except UserNotFoundError:
            raise VlobNotFoundError(f"User `{user}` doesn't exist")
        group = self._get_group(organization_id, id)
        if not group.rights.get(author, (False, False, False))[0]:
            raise VlobAccessError()
        if not read_right and not write_right:
            group.rights.pop(user, None)
        else:
            group.rights[user] = (admin_right, read_right, write_right)

    async def poll_group(
        self, organization_id: OrganizationID, author: UserID, id: UUID, checkpoint: int
    ) -> Tuple[int, Dict[UUID, int]]:
        group = self._get_group(organization_id, id)
        if not group.rights.get(author, (False, False, False))[1]:
            raise VlobAccessError()
        changes_since_checkpoint = {
            src_id: src_version
            for src_id, (_, change_checkpoint, src_version) in group.changes.items()
            if change_checkpoint > checkpoint
        }
        return (group.checkpoint, changes_since_checkpoint)
