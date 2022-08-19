# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import base64
from typing import List, Tuple, Dict, Optional
from pendulum import DateTime, now as pendulum_now
import urllib
from parsec.backend.http_utils import http_request
from parsec.backend.sequester import SequesterService, SequesterServiceType

from parsec.utils import timestamps_in_the_ballpark
from parsec.api.protocol import (
    RealmID,
    VlobID,
    DeviceID,
    OrganizationID,
    SequesterServiceID,
    vlob_create_serializer,
    vlob_read_serializer,
    vlob_update_serializer,
    vlob_poll_changes_serializer,
    vlob_list_versions_serializer,
    vlob_maintenance_get_reencryption_batch_serializer,
    vlob_maintenance_save_reencryption_batch_serializer,
)
from parsec.backend.utils import catch_protocol_errors, api


class VlobError(Exception):
    pass


class VlobAccessError(VlobError):
    pass


class VlobVersionError(VlobError):
    pass


class VlobNotFoundError(VlobError):
    pass


class VlobRealmNotFoundError(VlobError):
    pass


class VlobAlreadyExistsError(VlobError):
    pass


class VlobEncryptionRevisionError(VlobError):
    pass


class VlobInMaintenanceError(VlobError):
    pass


class VlobNotInMaintenanceError(VlobError):
    pass


class VlobMaintenanceError(VlobError):
    pass


class VlobRequireGreaterTimestampError(VlobError):
    @property
    def strictly_greater_than(self):
        return self.args[0]


class VlobSequesterDisabledError(VlobError):
    pass


class VlobSequesterWebhookServiceRejectedError(VlobError):
    pass


class VlobSequesterServiceMissingWebhookError(VlobError):
    pass


class VlobSequesterServiceWebhookUrlError(VlobError):
    pass


class VlobSequesterServiceInconsistencyError(VlobError):
    def __init__(
        self, sequester_authority_certificate: bytes, sequester_services_certificates: List[bytes]
    ):
        self.sequester_authority_certificate = sequester_authority_certificate
        self.sequester_services_certificates = sequester_services_certificates


async def extract_sequestered_data_and_proceed_webhook(
    services: Dict[SequesterServiceID, SequesterService],
    organization_id: OrganizationID,
    author: DeviceID,
    encryption_revision: int,
    vlob_id: VlobID,
    timestamp: DateTime,
    sequester_blob: Dict[SequesterServiceID, bytes],
):
    # Split storage services and webhook services
    storage_service_ids = [
        service.service_id
        for service in services.values()
        if service.service_type == SequesterServiceType.STORAGE
    ]
    webhook_service_ids = [
        service.service_id
        for service in services.values()
        if service.service_type == SequesterServiceType.WEBHOOK
    ]
    for webhook_service_id in webhook_service_ids:
        service = services[webhook_service_id]
        sequester_data = sequester_blob[webhook_service_id]
        if not service.webhook_url:
            raise VlobSequesterServiceMissingWebhookError()

        to_webhook_data = {
            "sequester_blob": base64.urlsafe_b64encode(sequester_data),
            "author": author,
            "encryption_revision": encryption_revision,
            "vlob_id": vlob_id,
            "timestamp": timestamp,
            "organization_id": organization_id,
        }
        to_webhook_data = urllib.parse.urlencode(to_webhook_data).encode()
        try:
            await http_request(url=service.webhook_url, method="POST", data=to_webhook_data)
        except urllib.error.HTTPError as exc:
            raise VlobSequesterWebhookServiceRejectedError from exc
        except urllib.error.URLError as exc:
            raise VlobSequesterServiceWebhookUrlError from exc

    # Get sequester data for storage
    sequestered_data = {
        service_id: sequester_blob[service_id] for service_id in storage_service_ids
    }

    return sequestered_data


class BaseVlobComponent:
    @api("vlob_create")
    @catch_protocol_errors
    async def api_vlob_create(self, client_ctx, msg):
        """
        This API call, when successful, performs the writing of a new vlob version to the database.
        Before adding new entries, extra care should be taken in order to guarantee the consistency in
        the ordering of the different timestamps stored in the database.

        See the `api_vlob_update` docstring for more information about the checks performed and the
        error returned in case those checks failed.
        """
        msg = vlob_create_serializer.req_load(msg)

        now = pendulum_now()
        if not timestamps_in_the_ballpark(msg["timestamp"], now):
            return vlob_create_serializer.timestamp_out_of_ballpark_rep_dump(
                backend_timestamp=now, client_timestamp=msg["timestamp"]
            )

        try:
            await self.create(client_ctx.organization_id, client_ctx.device_id, **msg)

        except VlobAlreadyExistsError as exc:
            return vlob_create_serializer.rep_dump({"status": "already_exists", "reason": str(exc)})

        except (VlobAccessError, VlobRealmNotFoundError):
            return vlob_create_serializer.rep_dump({"status": "not_allowed"})

        except VlobRequireGreaterTimestampError as exc:
            return vlob_create_serializer.require_greater_timestamp_rep_dump(
                exc.strictly_greater_than
            )

        except VlobEncryptionRevisionError:
            return vlob_create_serializer.rep_dump({"status": "bad_encryption_revision"})

        except VlobInMaintenanceError:
            return vlob_create_serializer.rep_dump({"status": "in_maintenance"})

        except VlobSequesterDisabledError:
            return vlob_create_serializer.rep_dump({"status": "not_a_sequestered_organization"})

        except VlobSequesterServiceInconsistencyError as exc:
            return vlob_create_serializer.rep_dump(
                {
                    "status": "sequester_inconsistency",
                    "sequester_authority_certificate": exc.sequester_authority_certificate,
                    "sequester_services_certificates": exc.sequester_services_certificates,
                }
            )

        except VlobSequesterWebhookServiceRejectedError:
            return vlob_create_serializer.rep_dump(
                {
                    "status": "sequester_rejected",
                    "service_id": "",
                    "service_label": "",
                    "service_error": "",
                }
            )

        except (VlobSequesterServiceMissingWebhookError, VlobSequesterServiceWebhookUrlError):
            return vlob_create_serializer.rep_dump(
                {
                    "status": "sequester_webhook_failed",
                    "service_id": "",
                    "service_label": "",
                    "service_error": "",
                }
            )

        return vlob_create_serializer.rep_dump({"status": "ok"})

    @api("vlob_read")
    @catch_protocol_errors
    async def api_vlob_read(self, client_ctx, msg):
        msg = vlob_read_serializer.req_load(msg)

        try:
            version, blob, author, created_on, author_last_role_granted_on = await self.read(
                client_ctx.organization_id, client_ctx.device_id, **msg
            )

        except VlobNotFoundError as exc:
            return vlob_read_serializer.rep_dump({"status": "not_found", "reason": str(exc)})

        except VlobAccessError:
            return vlob_read_serializer.rep_dump({"status": "not_allowed"})

        except VlobVersionError:
            return vlob_read_serializer.rep_dump({"status": "bad_version"})

        except VlobEncryptionRevisionError:
            return vlob_create_serializer.rep_dump({"status": "bad_encryption_revision"})

        except VlobInMaintenanceError:
            return vlob_read_serializer.rep_dump({"status": "in_maintenance"})

        return vlob_read_serializer.rep_dump(
            {
                "status": "ok",
                "blob": blob,
                "version": version,
                "author": author,
                "timestamp": created_on,
                "author_last_role_granted_on": author_last_role_granted_on,
            }
        )

    @api("vlob_update")
    @catch_protocol_errors
    async def api_vlob_update(self, client_ctx, msg):
        """
        This API call, when successful, performs the writing of a new vlob version to the database.
        Before adding new entries, extra care should be taken in order to guarantee the consistency in
        the ordering of the different timestamps stored in the database.

        In particular, the backend server performs the following checks:
        - The vlob version must have a timestamp greater or equal than the timestamp of the previous
          version of the same vlob.
        - The vlob version must have a timestamp strictly greater than the timestamp of the last role
          certificate for the corresponding user in the corresponding realm.

        If one of those constraints is not satisfied, an error is returned with the status
        `require_greater_timestamp` indicating to the client that it should craft a new certificate
        with a timestamp strictly greater than the timestamp provided with the error.

        The `api_realm_update_roles` and `api_vlob_create` calls also perform similar checks.
        """
        msg = vlob_update_serializer.req_load(msg)

        now = pendulum_now()
        if not timestamps_in_the_ballpark(msg["timestamp"], now):
            return vlob_update_serializer.timestamp_out_of_ballpark_rep_dump(
                backend_timestamp=now, client_timestamp=msg["timestamp"]
            )

        try:
            await self.update(client_ctx.organization_id, client_ctx.device_id, **msg)

        except VlobNotFoundError as exc:
            return vlob_update_serializer.rep_dump({"status": "not_found", "reason": str(exc)})

        except VlobAccessError:
            return vlob_update_serializer.rep_dump({"status": "not_allowed"})

        except VlobRequireGreaterTimestampError as exc:
            return vlob_update_serializer.require_greater_timestamp_rep_dump(
                exc.strictly_greater_than
            )

        except VlobVersionError:
            return vlob_update_serializer.rep_dump({"status": "bad_version"})

        except VlobEncryptionRevisionError:
            return vlob_create_serializer.rep_dump({"status": "bad_encryption_revision"})

        except VlobInMaintenanceError:
            return vlob_update_serializer.rep_dump({"status": "in_maintenance"})

        except VlobSequesterDisabledError:
            return vlob_create_serializer.rep_dump({"status": "not_a_sequestered_organization"})

        except VlobSequesterServiceInconsistencyError as exc:
            return vlob_create_serializer.rep_dump(
                {
                    "status": "sequester_inconsistency",
                    "sequester_authority_certificate": exc.sequester_authority_certificate,
                    "sequester_services_certificates": exc.sequester_services_certificates,
                }
            )

        except VlobSequesterWebhookServiceRejectedError:
            return vlob_create_serializer.rep_dump(
                {
                    "status": "sequester_rejected",
                    "service_id": "",
                    "service_label": "",
                    "service_error": "",
                }
            )

        except (VlobSequesterServiceMissingWebhookError, VlobSequesterServiceWebhookUrlError):
            return vlob_create_serializer.rep_dump(
                {
                    "status": "sequester_webhook_failed",
                    "service_id": "",
                    "service_label": "",
                    "service_error": "",
                }
            )

        return vlob_update_serializer.rep_dump({"status": "ok"})

    @api("vlob_poll_changes")
    @catch_protocol_errors
    async def api_vlob_poll_changes(self, client_ctx, msg):
        msg = vlob_poll_changes_serializer.req_load(msg)

        # TODO: raise error if too many events since offset ?
        try:
            checkpoint, changes = await self.poll_changes(
                client_ctx.organization_id,
                client_ctx.device_id,
                msg["realm_id"],
                msg["last_checkpoint"],
            )

        except VlobAccessError:
            return vlob_poll_changes_serializer.rep_dump({"status": "not_allowed"})

        except VlobRealmNotFoundError as exc:
            return vlob_poll_changes_serializer.rep_dump(
                {"status": "not_found", "reason": str(exc)}
            )

        except VlobInMaintenanceError:
            return vlob_poll_changes_serializer.rep_dump({"status": "in_maintenance"})

        return vlob_poll_changes_serializer.rep_dump(
            {"status": "ok", "current_checkpoint": checkpoint, "changes": changes}
        )

    @api("vlob_list_versions")
    @catch_protocol_errors
    async def api_vlob_list_versions(self, client_ctx, msg):
        msg = vlob_list_versions_serializer.req_load(msg)

        try:
            versions_dict = await self.list_versions(
                client_ctx.organization_id, client_ctx.device_id, msg["vlob_id"]
            )

        except VlobAccessError:
            return vlob_list_versions_serializer.rep_dump({"status": "not_allowed"})

        except VlobNotFoundError as exc:
            return vlob_list_versions_serializer.rep_dump(
                {"status": "not_found", "reason": str(exc)}
            )

        except VlobInMaintenanceError:
            return vlob_list_versions_serializer.rep_dump({"status": "in_maintenance"})

        return vlob_list_versions_serializer.rep_dump({"status": "ok", "versions": versions_dict})

    @api("vlob_maintenance_get_reencryption_batch")
    @catch_protocol_errors
    async def api_vlob_maintenance_get_reencryption_batch(self, client_ctx, msg):
        msg = vlob_maintenance_get_reencryption_batch_serializer.req_load(msg)

        try:
            batch = await self.maintenance_get_reencryption_batch(
                client_ctx.organization_id, client_ctx.device_id, **msg
            )

        except VlobAccessError:
            return vlob_maintenance_get_reencryption_batch_serializer.rep_dump(
                {"status": "not_allowed"}
            )

        except VlobRealmNotFoundError as exc:
            return vlob_maintenance_get_reencryption_batch_serializer.rep_dump(
                {"status": "not_found", "reason": str(exc)}
            )

        except VlobNotInMaintenanceError as exc:
            return vlob_maintenance_get_reencryption_batch_serializer.rep_dump(
                {"status": "not_in_maintenance", "reason": str(exc)}
            )

        except VlobEncryptionRevisionError:
            return vlob_create_serializer.rep_dump({"status": "bad_encryption_revision"})

        except VlobMaintenanceError as exc:
            return vlob_maintenance_get_reencryption_batch_serializer.rep_dump(
                {"status": "maintenance_error", "reason": str(exc)}
            )

        return vlob_maintenance_get_reencryption_batch_serializer.rep_dump(
            {
                "status": "ok",
                "batch": [
                    {"vlob_id": vlob_id, "version": version, "blob": blob}
                    for vlob_id, version, blob in batch
                ],
            }
        )

    @api("vlob_maintenance_save_reencryption_batch")
    @catch_protocol_errors
    async def api_vlob_maintenance_save_reencryption_batch(self, client_ctx, msg):
        msg = vlob_maintenance_save_reencryption_batch_serializer.req_load(msg)

        try:
            total, done = await self.maintenance_save_reencryption_batch(
                client_ctx.organization_id,
                client_ctx.device_id,
                realm_id=msg["realm_id"],
                encryption_revision=msg["encryption_revision"],
                batch=[(x["vlob_id"], x["version"], x["blob"]) for x in msg["batch"]],
            )

        except VlobAccessError:
            return vlob_maintenance_save_reencryption_batch_serializer.rep_dump(
                {"status": "not_allowed"}
            )

        # No need to catch VlobNotFoundError given unknown vlob/version in batch are ignored
        except VlobRealmNotFoundError as exc:
            return vlob_maintenance_save_reencryption_batch_serializer.rep_dump(
                {"status": "not_found", "reason": str(exc)}
            )

        except VlobNotInMaintenanceError as exc:
            return vlob_maintenance_get_reencryption_batch_serializer.rep_dump(
                {"status": "not_in_maintenance", "reason": str(exc)}
            )

        except VlobEncryptionRevisionError:
            return vlob_create_serializer.rep_dump({"status": "bad_encryption_revision"})

        except VlobMaintenanceError as exc:
            return vlob_maintenance_save_reencryption_batch_serializer.rep_dump(
                {"status": "maintenance_error", "reason": str(exc)}
            )

        return vlob_maintenance_save_reencryption_batch_serializer.rep_dump(
            {"status": "ok", "total": total, "done": done}
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
        # Sequester is a special case, so give it a default version to simplify tests
        sequester_blob: Optional[Dict[SequesterServiceID, bytes]] = None,
    ) -> None:
        """
        Raises:
            VlobAlreadyExistsError
            VlobEncryptionRevisionError: if encryption_revision mismatch
            VlobInMaintenanceError
            VlobSequesterDisabledError
            VlobSequesterServiceInconsistencyError
        """
        raise NotImplementedError()

    async def read(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        encryption_revision: int,
        vlob_id: VlobID,
        version: Optional[int] = None,
        timestamp: Optional[DateTime] = None,
    ) -> Tuple[int, bytes, DeviceID, DateTime, DateTime]:
        """
        Raises:
            VlobAccessError
            VlobVersionError
            VlobNotFoundError
            VlobEncryptionRevisionError: if encryption_revision mismatch
            VlobInMaintenanceError
        """
        raise NotImplementedError()

    async def update(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        encryption_revision: int,
        vlob_id: VlobID,
        version: int,
        timestamp: DateTime,
        blob: bytes,
        # Sequester is a special case, so give it a default version to simplify tests
        sequester_blob: Optional[Dict[SequesterServiceID, bytes]] = None,
    ) -> None:
        """
        Raises:
            VlobAccessError
            VlobVersionError
            VlobNotFoundError
            VlobEncryptionRevisionError: if encryption_revision mismatch
            VlobInMaintenanceError
            VlobSequesterDisabledError
            VlobSequesterServiceInconsistencyError
        """
        raise NotImplementedError()

    async def poll_changes(
        self, organization_id: OrganizationID, author: DeviceID, realm_id: RealmID, checkpoint: int
    ) -> Tuple[int, Dict[VlobID, int]]:
        """
        Raises:
            VlobInMaintenanceError
            VlobNotFoundError
            VlobAccessError
        """
        raise NotImplementedError()

    async def list_versions(
        self, organization_id: OrganizationID, author: DeviceID, vlob_id: VlobID
    ) -> Dict[int, Tuple[DateTime, DeviceID]]:
        """
        Raises:
            VlobInMaintenanceError
            VlobNotFoundError
            VlobAccessError
        """
        raise NotImplementedError()

    async def maintenance_get_reencryption_batch(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: RealmID,
        encryption_revision: int,
        size: int,
    ) -> List[Tuple[VlobID, int, bytes]]:
        """
        Raises:
            VlobNotFoundError
            VlobAccessError
            VlobEncryptionRevisionError
            VlobMaintenanceError: not in maintenance
        """
        raise NotImplementedError()

    async def maintenance_save_reencryption_batch(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: RealmID,
        encryption_revision: int,
        batch: List[Tuple[VlobID, int, bytes]],
    ) -> Tuple[int, int]:
        """
        Raises:
            VlobNotFoundError
            VlobAccessError
            VlobEncryptionRevisionError
            VlobMaintenanceError: not in maintenance
        """
        raise NotImplementedError()
