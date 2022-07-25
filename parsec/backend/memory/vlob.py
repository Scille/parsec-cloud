# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

from dataclasses import dataclass, field as dataclass_field
from pendulum import DateTime
from typing import TYPE_CHECKING, List, AbstractSet, Tuple, Dict, Optional
from collections import defaultdict

from parsec.api.protocol import (
    DeviceID,
    OrganizationID,
    UserID,
    RealmID,
    RealmRole,
    VlobID,
    SequesterServiceID,
)
from parsec.backend.utils import OperationKind
from parsec.backend.backend_events import BackendEvent
from parsec.backend.realm import RealmNotFoundError
from parsec.backend.vlob import (
    BaseVlobComponent,
    VlobAccessError,
    VlobVersionError,
    VlobNotFoundError,
    VlobRealmNotFoundError,
    VlobAlreadyExistsError,
    VlobEncryptionRevisionError,
    VlobInMaintenanceError,
    VlobNotInMaintenanceError,
    VlobRequireGreaterTimestampError,
    VlobSequesterDisabledError,
    VlobSequesterServiceInconsistencyError,
)

if TYPE_CHECKING:
    from parsec.backend.memory.realm import Realm, MemoryRealmComponent
    from parsec.backend.memory.organization import MemoryOrganizationComponent
    from parsec.backend.memory.sequester import MemorySequesterComponent


VlobData = List[Tuple[bytes, DeviceID, DateTime]]
SequesteredVlobData = List[Dict[SequesterServiceID, bytes]]


@dataclass
class Vlob:
    realm_id: RealmID
    data: VlobData
    sequestered_data: Optional[SequesteredVlobData]

    @property
    def current_version(self):
        return len(self.data)


class Reencryption:
    def __init__(self, realm_id: RealmID, vlobs: Dict[VlobID, Vlob]):
        self.realm_id = realm_id
        self._original_vlobs = vlobs
        self._todo: Dict[Tuple[VlobID, int], bytes] = {}
        self._done: Dict[Tuple[VlobID, int], bytes] = {}
        for vlob_id, vlob in vlobs.items():
            for index, (data, _, _) in enumerate(vlob.data):
                version = index + 1
                self._todo[(vlob_id, version)] = data
        self._total = len(self._todo)

    def get_reencrypted_vlobs(self) -> Dict[VlobID, Vlob]:
        assert self.is_finished()
        vlobs = {}
        for (vlob_id, version), data in sorted(self._done.items()):
            try:
                (_, author, timestamp) = self._original_vlobs[vlob_id].data[version - 1]

            except KeyError:
                raise VlobNotFoundError()

            if vlob_id not in vlobs:
                # Force `sequestered_data` field to `None` as it is not used here
                vlobs[vlob_id] = Vlob(self.realm_id, [(data, author, timestamp)], None)
            else:
                vlobs[vlob_id].data.append((data, author, timestamp))
            assert len(vlobs[vlob_id].data) == version

        return vlobs

    def is_finished(self) -> bool:
        return not self._todo

    def get_batch(self, size: int) -> List[Tuple[VlobID, int, bytes]]:
        batch = []
        for (vlob_id, version), data in self._todo.items():
            if (vlob_id, version) in self._done:
                continue
            batch.append((vlob_id, version, data))
        return batch[:size]

    def save_batch(self, batch: List[Tuple[VlobID, int, bytes]]) -> Tuple[int, int]:
        for vlob_id, version, data in batch:
            key = (vlob_id, version)
            if key in self._done:
                continue
            try:
                del self._todo[key]
            except KeyError:
                # Unknown item, just ignore it just like we do in PostgreSQL
                continue
            self._done[key] = data

        return self._total, len(self._done)


@dataclass
class Changes:
    checkpoint: int = dataclass_field(default=0)
    changes: Dict[VlobID, Tuple[DeviceID, int, int]] = dataclass_field(default_factory=dict)
    reencryption: Optional[Reencryption] = dataclass_field(default=None)
    last_vlob_update_per_user: Dict[UserID, DateTime] = dataclass_field(default_factory=dict)


class MemoryVlobComponent(BaseVlobComponent):
    def __init__(self, send_event):
        self._send_event = send_event
        self._organization_component: "MemoryOrganizationComponent" = None
        self._realm_component: "MemoryRealmComponent" = None
        self._sequester_component: "MemorySequesterComponent" = None
        self._vlobs: Dict[Tuple[OrganizationID, VlobID], Vlob] = {}
        self._per_realm_changes: Dict[Tuple[OrganizationID, RealmID], Changes] = defaultdict(
            Changes
        )

    def register_components(
        self,
        organization: "MemoryOrganizationComponent",
        realm: "MemoryRealmComponent",
        sequester: "MemorySequesterComponent",
        **other_components,
    ) -> None:
        self._organization_component = organization
        self._realm_component = realm
        self._sequester_component = sequester

    def _maintenance_reencryption_start_hook(
        self, organization_id: OrganizationID, realm_id: RealmID, encryption_revision: int
    ) -> None:
        changes = self._per_realm_changes[(organization_id, realm_id)]
        assert not changes.reencryption
        realm_vlobs = {
            vlob_id: vlob
            for (orgid, vlob_id), vlob in self._vlobs.items()
            if orgid == organization_id and vlob.realm_id == realm_id
        }
        changes.reencryption = Reencryption(realm_id, realm_vlobs)

    def _maintenance_reencryption_is_finished_hook(
        self, organization_id: OrganizationID, realm_id: RealmID, encryption_revision: int
    ) -> bool:
        changes = self._per_realm_changes[(organization_id, realm_id)]
        assert changes.reencryption
        if not changes.reencryption.is_finished():
            return False

        realm_vlobs = changes.reencryption.get_reencrypted_vlobs()
        for vlob_id, vlob in realm_vlobs.items():
            self._vlobs[(organization_id, vlob_id)] = vlob
        changes.reencryption = None
        return True

    def _get_vlob(self, organization_id: OrganizationID, vlob_id: VlobID) -> Vlob:
        try:
            return self._vlobs[(organization_id, vlob_id)]

        except KeyError:
            raise VlobNotFoundError(f"Vlob `{vlob_id}` doesn't exist")

    def _check_sequestered_organization(
        self,
        organization_id: OrganizationID,
        expect_sequestered_organization: bool,
        expect_active_sequester_services: AbstractSet[SequesterServiceID] = set(),
    ):
        try:
            org = self._organization_component._organizations[organization_id]
        except KeyError:
            raise VlobNotFoundError()

        if not org.sequester_authority:
            # Regular organization
            if expect_sequestered_organization:
                raise VlobSequesterDisabledError()

        else:
            # Sequestered organization
            services = self._sequester_component._enabled_services(organization_id)
            if (
                not expect_sequestered_organization
                or {s.service_id for s in services} != expect_active_sequester_services
            ):
                raise VlobSequesterServiceInconsistencyError(
                    sequester_authority_certificate=org.sequester_authority.certificate,
                    sequester_services_certificates=[s.service_certificate for s in services],
                )

    def _check_realm_read_access(
        self,
        organization_id: OrganizationID,
        realm_id: RealmID,
        user_id: UserID,
        encryption_revision: Optional[int],
        timestamp: Optional[DateTime],
    ) -> "Realm":
        return self._check_realm_access(
            organization_id,
            realm_id,
            user_id,
            encryption_revision,
            timestamp,
            OperationKind.DATA_READ,
        )

    def _check_realm_write_access(
        self,
        organization_id: OrganizationID,
        realm_id: RealmID,
        user_id: UserID,
        encryption_revision: Optional[int],
        timestamp: Optional[DateTime],
    ) -> "Realm":
        return self._check_realm_access(
            organization_id,
            realm_id,
            user_id,
            encryption_revision,
            timestamp,
            OperationKind.DATA_WRITE,
        )

    def _check_realm_access(
        self,
        organization_id: OrganizationID,
        realm_id: RealmID,
        user_id: UserID,
        encryption_revision: Optional[int],
        timestamp: Optional[DateTime],
        operation_kind: OperationKind,
    ) -> "Realm":
        try:
            realm = self._realm_component._get_realm(organization_id, realm_id)
        except RealmNotFoundError:
            raise VlobRealmNotFoundError(f"Realm `{realm_id}` doesn't exist")

        allowed_roles: Tuple[RealmRole, ...]
        # Only an owner can perform maintenance operation
        if operation_kind == OperationKind.MAINTENANCE:
            allowed_roles = (RealmRole.OWNER,)
        # All roles can do read-only operation
        elif operation_kind == OperationKind.DATA_READ:
            allowed_roles = (
                RealmRole.OWNER,
                RealmRole.MANAGER,
                RealmRole.CONTRIBUTOR,
                RealmRole.READER,
            )
        # All roles except reader can do write operation
        elif operation_kind == OperationKind.DATA_WRITE:
            allowed_roles = (RealmRole.OWNER, RealmRole.MANAGER, RealmRole.CONTRIBUTOR)
        else:
            assert False, f"Operation kind {operation_kind} not supported"

        # Check the role
        last_role = realm.get_last_role(user_id)
        if last_role is None or last_role.role not in allowed_roles:
            raise VlobAccessError()

        # Extra check for write operations
        if operation_kind == OperationKind.DATA_WRITE:

            # Write operations should always occurs strictly after the last change of role for this user
            assert last_role is not None
            assert timestamp is not None
            if last_role.granted_on >= timestamp:
                raise VlobRequireGreaterTimestampError(last_role.granted_on)

        # Special case of reading while in reencryption
        if operation_kind == OperationKind.DATA_READ and realm.status.in_reencryption:
            # Starting a reencryption maintenance bumps the encryption revision.
            # Hence if we are currently in reencryption maintenance, last encryption revision is not ready
            # to be used (it will be once the reencryption is over !).
            # So during this intermediary state, we allow read access to the previous encryption revision instead.

            # Note that `encryption_revision` might also be `None` in the case of `poll_changes` and `list_versions`
            # requests, which should also be allowed during a reencryption

            # The vlob is not available yet for the current revision
            if (
                encryption_revision is not None
                and encryption_revision == realm.status.encryption_revision
            ):
                raise VlobInMaintenanceError(f"Realm `{realm_id}` is currently under maintenance")

            # The vlob is only available at the previous revision
            if (
                encryption_revision is not None
                and encryption_revision != realm.status.encryption_revision - 1
            ):
                raise VlobEncryptionRevisionError()

        # In all other cases
        else:
            # Writing during maintenance is forbidden
            if operation_kind != OperationKind.MAINTENANCE and realm.status.in_maintenance:
                raise VlobInMaintenanceError(f"Realm `{realm_id}` is currently under maintenance")

            # A maintenance state was expected
            if operation_kind == OperationKind.MAINTENANCE and not realm.status.in_maintenance:
                raise VlobNotInMaintenanceError(f"Realm `{realm_id}` not under maintenance")

            # Otherwise, simply check that the revisions match
            if (
                encryption_revision is not None
                and encryption_revision != realm.status.encryption_revision
            ):
                raise VlobEncryptionRevisionError()

        # Return the real for later use
        return realm

    def _check_realm_in_maintenance_access(
        self,
        organization_id: OrganizationID,
        realm_id: RealmID,
        user_id: UserID,
        encryption_revision: int,
    ) -> "Realm":
        return self._check_realm_access(
            organization_id, realm_id, user_id, encryption_revision, None, OperationKind.MAINTENANCE
        )

    def _get_last_vlob_update(
        self, organization_id: OrganizationID, realm_id: RealmID, user_id: UserID
    ) -> Optional[DateTime]:
        changes = self._per_realm_changes[(organization_id, realm_id)]
        return changes.last_vlob_update_per_user.get(user_id)

    async def _update_changes(
        self,
        organization_id: OrganizationID,
        author,
        realm_id: RealmID,
        src_id: VlobID,
        timestamp: DateTime,
        src_version: int = 1,
    ) -> None:
        changes = self._per_realm_changes[(organization_id, realm_id)]
        changes.checkpoint += 1
        changes.changes[src_id] = (author, changes.checkpoint, src_version)

        current_value = changes.last_vlob_update_per_user.get(author.user_id)
        changes.last_vlob_update_per_user[author.user_id] = (
            timestamp if current_value is None else max(current_value, timestamp)
        )
        await self._send_event(
            BackendEvent.REALM_VLOBS_UPDATED,
            organization_id=organization_id,
            author=author,
            realm_id=realm_id,
            checkpoint=changes.checkpoint,
            src_id=src_id,
            src_version=src_version,
        )

    async def create(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: RealmID,
        encryption_revision: int,
        vlob_id: VlobID,
        timestamp: DateTime,
        blob: bytes,
        sequester_blob: Optional[Dict[SequesterServiceID, bytes]] = None,
    ) -> None:
        self._check_realm_write_access(
            organization_id, realm_id, author.user_id, encryption_revision, timestamp
        )
        if sequester_blob is None:
            self._check_sequestered_organization(
                organization_id=organization_id, expect_sequestered_organization=False
            )
            sequestered_data = None
        else:
            self._check_sequestered_organization(
                organization_id=organization_id,
                expect_sequestered_organization=True,
                expect_active_sequester_services=sequester_blob.keys(),
            )
            sequestered_data = [sequester_blob]

        key = (organization_id, vlob_id)
        if key in self._vlobs:
            raise VlobAlreadyExistsError()

        self._vlobs[key] = Vlob(realm_id, [(blob, author, timestamp)], sequestered_data)

        await self._update_changes(organization_id, author, realm_id, vlob_id, timestamp)

    async def read(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        encryption_revision: int,
        vlob_id: VlobID,
        version: Optional[int] = None,
        timestamp: Optional[DateTime] = None,
    ) -> Tuple[int, bytes, DeviceID, DateTime, DateTime]:
        vlob = self._get_vlob(organization_id, vlob_id)

        realm = self._check_realm_read_access(
            organization_id, vlob.realm_id, author.user_id, encryption_revision, timestamp
        )

        if version is None:
            if timestamp is None:
                version = vlob.current_version
            else:
                for i in range(vlob.current_version, 0, -1):
                    if vlob.data[i - 1][2] <= timestamp:
                        version = i
                        break
                else:
                    raise VlobVersionError()
        try:
            vlob_data, vlob_device_id, vlob_timestamp = vlob.data[version - 1]
            last_role = realm.get_last_role(vlob_device_id.user_id)
            # Given the vlob exists, the author must have had a role
            assert last_role is not None
            return (version, vlob_data, vlob_device_id, vlob_timestamp, last_role.granted_on)

        except IndexError:
            raise VlobVersionError()

    async def update(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        encryption_revision: int,
        vlob_id: VlobID,
        version: int,
        timestamp: DateTime,
        blob: bytes,
        sequester_blob: Optional[Dict[SequesterServiceID, bytes]] = None,
    ) -> None:
        vlob = self._get_vlob(organization_id, vlob_id)

        self._check_realm_write_access(
            organization_id, vlob.realm_id, author.user_id, encryption_revision, timestamp
        )
        if sequester_blob is None:
            self._check_sequestered_organization(
                organization_id=organization_id, expect_sequestered_organization=False
            )
        else:
            self._check_sequestered_organization(
                organization_id=organization_id,
                expect_sequestered_organization=True,
                expect_active_sequester_services=sequester_blob.keys(),
            )

        if version - 1 != vlob.current_version:
            raise VlobVersionError()
        if timestamp < vlob.data[vlob.current_version - 1][2]:
            raise VlobRequireGreaterTimestampError(vlob.data[vlob.current_version - 1][2])
        vlob.data.append((blob, author, timestamp))
        if sequester_blob is not None:  # /!\ We want to accept empty dicts !
            assert vlob.sequestered_data is not None
            vlob.sequestered_data.append(sequester_blob)

        await self._update_changes(
            organization_id, author, vlob.realm_id, vlob_id, timestamp, version
        )

    async def poll_changes(
        self, organization_id: OrganizationID, author: DeviceID, realm_id: RealmID, checkpoint: int
    ) -> Tuple[int, Dict[VlobID, int]]:
        self._check_realm_read_access(organization_id, realm_id, author.user_id, None, None)

        changes = self._per_realm_changes[(organization_id, realm_id)]
        changes_since_checkpoint = {
            src_id: src_version
            for src_id, (_, change_checkpoint, src_version) in changes.changes.items()
            if change_checkpoint > checkpoint
        }
        return (changes.checkpoint, changes_since_checkpoint)

    async def list_versions(
        self, organization_id: OrganizationID, author: DeviceID, vlob_id: VlobID
    ) -> Dict[int, Tuple[DateTime, DeviceID]]:
        vlobs = self._get_vlob(organization_id, vlob_id)

        self._check_realm_read_access(organization_id, vlobs.realm_id, author.user_id, None, None)
        return {k: (v[2], v[1]) for (k, v) in enumerate(vlobs.data, 1)}

    async def maintenance_get_reencryption_batch(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: RealmID,
        encryption_revision: int,
        size: int,
    ) -> List[Tuple[VlobID, int, bytes]]:
        self._check_realm_in_maintenance_access(
            organization_id, realm_id, author.user_id, encryption_revision
        )

        changes = self._per_realm_changes[(organization_id, realm_id)]
        assert changes.reencryption

        return changes.reencryption.get_batch(size)

    async def maintenance_save_reencryption_batch(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: RealmID,
        encryption_revision: int,
        batch: List[Tuple[VlobID, int, bytes]],
    ) -> Tuple[int, int]:
        self._check_realm_in_maintenance_access(
            organization_id, realm_id, author.user_id, encryption_revision
        )

        changes = self._per_realm_changes[(organization_id, realm_id)]
        assert changes.reencryption

        total, done = changes.reencryption.save_batch(batch)

        return total, done
