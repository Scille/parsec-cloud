# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import json
from typing import List, Tuple, Dict, Optional
from structlog import get_logger
import urllib

from parsec.backend.http_utils import http_request
from parsec.backend.sequester import (
    BaseSequesterService,
    StorageSequesterService,
    WebhookSequesterService,
)
from parsec._parsec import (
    DateTime,
    VlobCreateReq,
    VlobCreateRep,
    VlobCreateRepOk,
    VlobCreateRepAlreadyExists,
    VlobCreateRepBadEncryptionRevision,
    VlobCreateRepInMaintenance,
    VlobCreateRepNotAllowed,
    VlobCreateRepRequireGreaterTimestamp,
    VlobCreateRepBadTimestamp,
    VlobCreateRepNotASequesteredOrganization,
    VlobCreateRepSequesterInconsistency,
    VlobCreateRepRejectedBySequesterService,
    VlobCreateRepTimeout,
    VlobReadReq,
    VlobReadRep,
    VlobReadRepOk,
    VlobReadRepNotFound,
    VlobReadRepNotAllowed,
    VlobReadRepBadVersion,
    VlobReadRepBadEncryptionRevision,
    VlobReadRepInMaintenance,
    VlobUpdateReq,
    VlobUpdateRep,
    VlobUpdateRepOk,
    VlobUpdateRepBadEncryptionRevision,
    VlobUpdateRepBadVersion,
    VlobUpdateRepInMaintenance,
    VlobUpdateRepNotFound,
    VlobUpdateRepNotAllowed,
    VlobUpdateRepRequireGreaterTimestamp,
    VlobUpdateRepBadTimestamp,
    VlobUpdateRepNotASequesteredOrganization,
    VlobUpdateRepSequesterInconsistency,
    VlobUpdateRepRejectedBySequesterService,
    VlobUpdateRepTimeout,
    VlobPollChangesReq,
    VlobPollChangesRep,
    VlobPollChangesRepOk,
    VlobPollChangesRepNotFound,
    VlobPollChangesRepInMaintenance,
    VlobPollChangesRepNotAllowed,
    VlobListVersionsReq,
    VlobListVersionsRep,
    VlobListVersionsRepOk,
    VlobListVersionsRepInMaintenance,
    VlobListVersionsRepNotAllowed,
    VlobListVersionsRepNotFound,
    VlobMaintenanceGetReencryptionBatchReq,
    VlobMaintenanceGetReencryptionBatchRep,
    VlobMaintenanceGetReencryptionBatchRepOk,
    VlobMaintenanceGetReencryptionBatchRepNotInMaintenance,
    VlobMaintenanceGetReencryptionBatchRepBadEncryptionRevision,
    VlobMaintenanceGetReencryptionBatchRepMaintenanceError,
    VlobMaintenanceGetReencryptionBatchRepNotAllowed,
    VlobMaintenanceGetReencryptionBatchRepNotFound,
    VlobMaintenanceSaveReencryptionBatchReq,
    VlobMaintenanceSaveReencryptionBatchRep,
    VlobMaintenanceSaveReencryptionBatchRepOk,
    VlobMaintenanceSaveReencryptionBatchRepNotInMaintenance,
    VlobMaintenanceSaveReencryptionBatchRepBadEncryptionRevision,
    VlobMaintenanceSaveReencryptionBatchRepMaintenanceError,
    VlobMaintenanceSaveReencryptionBatchRepNotAllowed,
    VlobMaintenanceSaveReencryptionBatchRepNotFound,
    ReencryptionBatchEntry,
)
from parsec.api.protocol.base import api_typed_msg_adapter
from parsec.utils import (
    BALLPARK_CLIENT_EARLY_OFFSET,
    BALLPARK_CLIENT_LATE_OFFSET,
    timestamps_in_the_ballpark,
)
from parsec.api.protocol import (
    RealmID,
    VlobID,
    DeviceID,
    OrganizationID,
    SequesterServiceID,
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


class VlobSequesterWebhookRejectionError(VlobError):
    def __init__(self, service_id: SequesterServiceID, service_label: str, reason: str):
        self.service_id = service_id
        self.service_label = service_label
        self.reason = reason


class VlobSequesterWebhookUnavailableError(VlobError):
    def __init__(self, service_id: SequesterServiceID, service_label: str):
        self.service_id = service_id
        self.service_label = service_label


class VlobSequesterServiceInconsistencyError(VlobError):
    def __init__(
        self,
        sequester_authority_certificate: bytes,
        sequester_services_certificates: List[bytes],
    ):
        self.sequester_authority_certificate = sequester_authority_certificate
        self.sequester_services_certificates = sequester_services_certificates


async def extract_sequestered_data_and_proceed_webhook(
    services: Dict[SequesterServiceID, BaseSequesterService],
    organization_id: OrganizationID,
    author: DeviceID,
    encryption_revision: int,
    vlob_id: VlobID,
    timestamp: DateTime,
    sequester_blob: Dict[SequesterServiceID, bytes],
) -> Dict[SequesterServiceID, bytes]:

    logger = get_logger()
    # Split storage services and webhook services
    storage_services = [
        service for service in services.values() if isinstance(service, StorageSequesterService)
    ]
    webhook_services = [
        service for service in services.values() if isinstance(service, WebhookSequesterService)
    ]

    # Proceed webhook service before storage (garantee data are not stored if they are rejected)
    for service in webhook_services:
        sequester_data = sequester_blob[service.service_id]
        try:
            await http_request(
                url=f"{service.webhook_url}/{organization_id.str}",
                method="POST",
                data=sequester_data,
            )
        except urllib.error.HTTPError as exc:
            if exc.code == 400:
                raw_body = exc.read()
                try:
                    body = json.loads(raw_body)
                    if not isinstance(body, dict) or not isinstance(body.get("reason"), str):
                        raise ValueError
                    reason = body["reason"]
                except (json.JSONDecodeError, ValueError):
                    logger.warning(
                        "Invalid rejection reason body returned by webhook",
                        service_id=service.service_id.str,
                        service_label=service.service_label,
                        body=raw_body,
                    )
                    reason = "File rejected (no reason)"

                raise VlobSequesterWebhookRejectionError(
                    service_id=service.service_id,
                    service_label=service.service_label,
                    reason=reason,
                )

            else:
                logger.warning(
                    "Invalid HTTP status returned by webhook",
                    service_id=service.service_id.str,
                    service_label=service.service_label,
                    status=exc.code,
                    exc_info=exc,
                )
                raise VlobSequesterWebhookUnavailableError(
                    service_id=service.service_id,
                    service_label=service.service_label,
                ) from exc

        except urllib.error.URLError as exc:
            logger.warning(
                "Cannot reach webhook server",
                service_id=service.service_id.str,
                service_label=service.service_label,
                exc_info=exc,
            )
            raise VlobSequesterWebhookUnavailableError(
                service_id=service.service_id, service_label=service.service_label
            ) from exc

    # Get sequester data for storage
    sequestered_data = {
        service.service_id: sequester_blob[service.service_id] for service in storage_services
    }

    return sequestered_data


class BaseVlobComponent:
    @api("vlob_create")
    @catch_protocol_errors
    @api_typed_msg_adapter(VlobCreateReq, VlobCreateRep)
    async def api_vlob_create(self, client_ctx, req: VlobCreateReq) -> VlobCreateRep:
        """
        This API call, when successful, performs the writing of a new vlob version to the database.
        Before adding new entries, extra care should be taken in order to guarantee the consistency in
        the ordering of the different timestamps stored in the database.

        See the `api_vlob_update` docstring for more information about the checks performed and the
        error returned in case those checks failed.
        """
        now = DateTime.now()
        if not timestamps_in_the_ballpark(req.timestamp, now):
            return VlobCreateRepBadTimestamp(
                reason=None,
                ballpark_client_early_offset=BALLPARK_CLIENT_EARLY_OFFSET,
                ballpark_client_late_offset=BALLPARK_CLIENT_LATE_OFFSET,
                backend_timestamp=now,
                client_timestamp=req.timestamp,
            )

        try:
            await self.create(
                client_ctx.organization_id,
                client_ctx.device_id,
                realm_id=req.realm_id,
                encryption_revision=req.encryption_revision,
                vlob_id=req.vlob_id,
                timestamp=req.timestamp,
                blob=req.blob,
                sequester_blob=req.sequester_blob,
            )

        except VlobAlreadyExistsError:
            return VlobCreateRepAlreadyExists(None)

        except (VlobAccessError, VlobRealmNotFoundError):
            return VlobCreateRepNotAllowed()

        except VlobRequireGreaterTimestampError as exc:
            return VlobCreateRepRequireGreaterTimestamp(exc.strictly_greater_than)

        except VlobEncryptionRevisionError:
            return VlobCreateRepBadEncryptionRevision()

        except VlobInMaintenanceError:
            return VlobCreateRepInMaintenance()

        except VlobSequesterDisabledError:
            return VlobCreateRepNotASequesteredOrganization()

        except VlobSequesterServiceInconsistencyError as exc:
            return VlobCreateRepSequesterInconsistency(
                sequester_authority_certificate=exc.sequester_authority_certificate,
                sequester_services_certificates=exc.sequester_services_certificates,
            )

        except VlobSequesterWebhookRejectionError as exc:
            return VlobCreateRepRejectedBySequesterService(
                service_id=exc.service_id, service_label=exc.service_label, reason=exc.reason
            )

        except VlobSequesterWebhookUnavailableError:
            return VlobCreateRepTimeout()

        return VlobCreateRepOk()

    @api("vlob_read")
    @catch_protocol_errors
    @api_typed_msg_adapter(VlobReadReq, VlobReadRep)
    async def api_vlob_read(self, client_ctx, req: VlobReadReq) -> VlobReadRep:
        try:
            (version, blob, author, created_on, author_last_role_granted_on,) = await self.read(
                client_ctx.organization_id,
                client_ctx.device_id,
                encryption_revision=req.encryption_revision,
                vlob_id=req.vlob_id,
                version=req.version,
                timestamp=req.timestamp,
            )

        except VlobNotFoundError:
            return VlobReadRepNotFound(None)

        except VlobAccessError:
            return VlobReadRepNotAllowed()

        except VlobVersionError:
            return VlobReadRepBadVersion()

        except VlobEncryptionRevisionError:
            return VlobReadRepBadEncryptionRevision()

        except VlobInMaintenanceError:
            return VlobReadRepInMaintenance()

        return VlobReadRepOk(
            version,
            blob,
            author,
            created_on,
            author_last_role_granted_on,
        )

    @api("vlob_update")
    @catch_protocol_errors
    @api_typed_msg_adapter(VlobUpdateReq, VlobUpdateRep)
    async def api_vlob_update(self, client_ctx, req: VlobUpdateReq) -> VlobUpdateRep:
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
        now = DateTime.now()
        if not timestamps_in_the_ballpark(req.timestamp, now):
            return VlobUpdateRepBadTimestamp(
                reason=None,
                ballpark_client_early_offset=BALLPARK_CLIENT_EARLY_OFFSET,
                ballpark_client_late_offset=BALLPARK_CLIENT_LATE_OFFSET,
                backend_timestamp=now,
                client_timestamp=req.timestamp,
            )

        try:
            await self.update(
                client_ctx.organization_id,
                client_ctx.device_id,
                encryption_revision=req.encryption_revision,
                vlob_id=req.vlob_id,
                version=req.version,
                timestamp=req.timestamp,
                blob=req.blob,
                sequester_blob=req.sequester_blob,
            )

        except VlobNotFoundError:
            return VlobUpdateRepNotFound(None)

        except VlobAccessError:
            return VlobUpdateRepNotAllowed()

        except VlobRequireGreaterTimestampError as exc:
            return VlobUpdateRepRequireGreaterTimestamp(exc.strictly_greater_than)

        except VlobVersionError:
            return VlobUpdateRepBadVersion()

        except VlobEncryptionRevisionError:
            return VlobUpdateRepBadEncryptionRevision()

        except VlobInMaintenanceError:
            return VlobUpdateRepInMaintenance()

        except VlobSequesterDisabledError:
            return VlobUpdateRepNotASequesteredOrganization()

        except VlobSequesterServiceInconsistencyError as exc:
            return VlobUpdateRepSequesterInconsistency(
                sequester_authority_certificate=exc.sequester_authority_certificate,
                sequester_services_certificates=exc.sequester_services_certificates,
            )

        except VlobSequesterWebhookRejectionError as exc:
            return VlobUpdateRepRejectedBySequesterService(
                service_id=exc.service_id, service_label=exc.service_label, reason=exc.reason
            )

        except VlobSequesterWebhookUnavailableError:
            return VlobUpdateRepTimeout()

        return VlobUpdateRepOk()

    @api("vlob_poll_changes")
    @catch_protocol_errors
    @api_typed_msg_adapter(VlobPollChangesReq, VlobPollChangesRep)
    async def api_vlob_poll_changes(
        self, client_ctx, req: VlobPollChangesReq
    ) -> VlobPollChangesRep:
        # TODO: raise error if too many events since offset ?
        try:
            checkpoint, changes = await self.poll_changes(
                client_ctx.organization_id,
                client_ctx.device_id,
                realm_id=req.realm_id,
                checkpoint=req.last_checkpoint,
            )

        except VlobAccessError:
            return VlobPollChangesRepNotAllowed()

        except VlobRealmNotFoundError:
            return VlobPollChangesRepNotFound(None)

        except VlobInMaintenanceError:
            return VlobPollChangesRepInMaintenance()

        return VlobPollChangesRepOk(changes, checkpoint)

    @api("vlob_list_versions")
    @catch_protocol_errors
    @api_typed_msg_adapter(VlobListVersionsReq, VlobListVersionsRep)
    async def api_vlob_list_versions(
        self, client_ctx, req: VlobListVersionsReq
    ) -> VlobListVersionsRep:
        try:
            versions_dict = await self.list_versions(
                client_ctx.organization_id, client_ctx.device_id, vlob_id=req.vlob_id
            )

        except VlobAccessError:
            return VlobListVersionsRepNotAllowed()

        except VlobNotFoundError:
            return VlobListVersionsRepNotFound(None)

        except VlobInMaintenanceError:
            return VlobListVersionsRepInMaintenance()

        return VlobListVersionsRepOk(versions_dict)

    @api("vlob_maintenance_get_reencryption_batch")
    @catch_protocol_errors
    @api_typed_msg_adapter(
        VlobMaintenanceGetReencryptionBatchReq, VlobMaintenanceGetReencryptionBatchRep
    )
    async def api_vlob_maintenance_get_reencryption_batch(
        self, client_ctx, req: VlobMaintenanceGetReencryptionBatchReq
    ) -> VlobMaintenanceGetReencryptionBatchRep:
        try:
            batch = await self.maintenance_get_reencryption_batch(
                client_ctx.organization_id,
                client_ctx.device_id,
                realm_id=req.realm_id,
                encryption_revision=req.encryption_revision,
                size=req.size,
            )

        except VlobAccessError:
            return VlobMaintenanceGetReencryptionBatchRepNotAllowed()

        except VlobRealmNotFoundError:
            return VlobMaintenanceGetReencryptionBatchRepNotFound(None)

        except VlobNotInMaintenanceError:
            return VlobMaintenanceGetReencryptionBatchRepNotInMaintenance(None)

        except VlobEncryptionRevisionError:
            return VlobMaintenanceGetReencryptionBatchRepBadEncryptionRevision()

        except VlobMaintenanceError:
            return VlobMaintenanceGetReencryptionBatchRepMaintenanceError(None)

        return VlobMaintenanceGetReencryptionBatchRepOk(
            [ReencryptionBatchEntry(vlob_id, version, blob) for vlob_id, version, blob in batch]
        )

    @api("vlob_maintenance_save_reencryption_batch")
    @catch_protocol_errors
    @api_typed_msg_adapter(
        VlobMaintenanceSaveReencryptionBatchReq, VlobMaintenanceSaveReencryptionBatchRep
    )
    async def api_vlob_maintenance_save_reencryption_batch(
        self, client_ctx, req: VlobMaintenanceSaveReencryptionBatchReq
    ) -> VlobMaintenanceSaveReencryptionBatchRep:
        try:
            total, done = await self.maintenance_save_reencryption_batch(
                client_ctx.organization_id,
                client_ctx.device_id,
                realm_id=req.realm_id,
                encryption_revision=req.encryption_revision,
                batch=[(x.vlob_id, x.version, x.blob) for x in req.batch],
            )

        except VlobAccessError:
            return VlobMaintenanceSaveReencryptionBatchRepNotAllowed()

        # No need to catch VlobNotFoundError given unknown vlob/version in batch are ignored
        except VlobRealmNotFoundError:
            return VlobMaintenanceSaveReencryptionBatchRepNotFound(None)

        except VlobNotInMaintenanceError:
            return VlobMaintenanceSaveReencryptionBatchRepNotInMaintenance(None)

        except VlobEncryptionRevisionError:
            return VlobMaintenanceSaveReencryptionBatchRepBadEncryptionRevision()

        except VlobMaintenanceError:
            return VlobMaintenanceSaveReencryptionBatchRepMaintenanceError(None)

        return VlobMaintenanceSaveReencryptionBatchRepOk(total, done)

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
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: RealmID,
        checkpoint: int,
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
