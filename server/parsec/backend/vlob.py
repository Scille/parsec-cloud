# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

import json
import urllib.error
from typing import Dict, List, Tuple

from structlog import get_logger

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    RealmID,
    ReencryptionBatchEntry,
    SequesterServiceID,
    VlobID,
    authenticated_cmds,
)
from parsec.backend.client_context import AuthenticatedClientContext
from parsec.backend.http_utils import http_request
from parsec.backend.sequester import (
    BaseSequesterService,
    StorageSequesterService,
    WebhookSequesterService,
)
from parsec.backend.utils import api
from parsec.utils import (
    BALLPARK_CLIENT_EARLY_OFFSET,
    BALLPARK_CLIENT_LATE_OFFSET,
    timestamps_in_the_ballpark,
)


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
    def strictly_greater_than(self) -> DateTime:
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

    # Proceed webhook service before storage (guarantee data are not stored if they are rejected)
    for service in webhook_services:
        sequester_data = sequester_blob[service.service_id]
        try:
            await http_request(
                url=f"{service.webhook_url}",
                url_params={
                    "organization_id": organization_id.str,
                    "service_id": service.service_id.hex,
                },
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
                        service_id=service.service_id.hex,
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
                    service_id=service.service_id.hex,
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
                service_id=service.service_id.hex,
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
    @api
    async def apiv3_vlob_create(
        self, client_ctx: AuthenticatedClientContext, req: authenticated_cmds.v3.vlob_create.Req
    ) -> authenticated_cmds.v3.vlob_create.Rep:
        # `vlob_create` command is similar between APIv3 and v4+ from the server
        # point of view.
        # (from client point of view, server may return `bad_timestamp` response
        # with some fields missing)
        # TODO: proper api req/rep conversion
        return await self.api_vlob_create(client_ctx, req)  # type: ignore[return-value, arg-type]

    @api
    async def api_vlob_create(
        self, client_ctx: AuthenticatedClientContext, req: authenticated_cmds.latest.vlob_create.Req
    ) -> authenticated_cmds.latest.vlob_create.Rep:
        """
        This API call, when successful, performs the writing of a new vlob version to the database.
        Before adding new entries, extra care should be taken in order to guarantee the consistency in
        the ordering of the different timestamps stored in the database.

        See the `api_vlob_update` docstring for more information about the checks performed and the
        error returned in case those checks failed.
        """
        now = DateTime.now()
        if not timestamps_in_the_ballpark(req.timestamp, now):
            return authenticated_cmds.latest.vlob_create.RepBadTimestamp(
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
            return authenticated_cmds.latest.vlob_create.RepAlreadyExists(None)

        except (VlobAccessError, VlobRealmNotFoundError):
            return authenticated_cmds.latest.vlob_create.RepNotAllowed()

        except VlobRequireGreaterTimestampError as exc:
            return authenticated_cmds.latest.vlob_create.RepRequireGreaterTimestamp(
                exc.strictly_greater_than
            )

        except VlobEncryptionRevisionError:
            return authenticated_cmds.latest.vlob_create.RepBadEncryptionRevision()

        except VlobInMaintenanceError:
            return authenticated_cmds.latest.vlob_create.RepInMaintenance()

        except VlobSequesterDisabledError:
            return authenticated_cmds.latest.vlob_create.RepNotASequesteredOrganization()

        except VlobSequesterServiceInconsistencyError as exc:
            return authenticated_cmds.latest.vlob_create.RepSequesterInconsistency(
                sequester_authority_certificate=exc.sequester_authority_certificate,
                sequester_services_certificates=exc.sequester_services_certificates,
            )

        except VlobSequesterWebhookRejectionError as exc:
            return authenticated_cmds.latest.vlob_create.RepRejectedBySequesterService(
                service_id=exc.service_id, service_label=exc.service_label, reason=exc.reason
            )

        except VlobSequesterWebhookUnavailableError:
            return authenticated_cmds.latest.vlob_create.RepTimeout()

        return authenticated_cmds.latest.vlob_create.RepOk()

    @api
    async def apiv3_vlob_read(
        self, client_ctx: AuthenticatedClientContext, req: authenticated_cmds.v3.vlob_read.Req
    ) -> authenticated_cmds.v3.vlob_read.Rep:
        try:
            (version, blob, author, created_on, author_last_role_granted_on, _) = await self.read(
                client_ctx.organization_id,
                client_ctx.device_id,
                encryption_revision=req.encryption_revision,
                vlob_id=req.vlob_id,
                version=req.version,
                timestamp=req.timestamp,
            )

        except VlobNotFoundError:
            return authenticated_cmds.v3.vlob_read.RepNotFound(None)

        except VlobAccessError:
            return authenticated_cmds.v3.vlob_read.RepNotAllowed()

        except VlobVersionError:
            return authenticated_cmds.v3.vlob_read.RepBadVersion()

        except VlobEncryptionRevisionError:
            return authenticated_cmds.v3.vlob_read.RepBadEncryptionRevision()

        except VlobInMaintenanceError:
            return authenticated_cmds.v3.vlob_read.RepInMaintenance()

        return authenticated_cmds.v3.vlob_read.RepOk(
            version,
            blob,
            author,
            created_on,
            author_last_role_granted_on,
        )

    @api
    async def api_vlob_read(
        self, client_ctx: AuthenticatedClientContext, req: authenticated_cmds.latest.vlob_read.Req
    ) -> authenticated_cmds.latest.vlob_read.Rep:
        try:
            (version, blob, author, created_on, _, certificate_index) = await self.read(
                client_ctx.organization_id,
                client_ctx.device_id,
                encryption_revision=req.encryption_revision,
                vlob_id=req.vlob_id,
                version=req.version,
                timestamp=req.timestamp,
            )

        except VlobNotFoundError:
            return authenticated_cmds.latest.vlob_read.RepNotFound(None)

        except VlobAccessError:
            return authenticated_cmds.latest.vlob_read.RepNotAllowed()

        except VlobVersionError:
            return authenticated_cmds.latest.vlob_read.RepBadVersion()

        except VlobEncryptionRevisionError:
            return authenticated_cmds.latest.vlob_read.RepBadEncryptionRevision()

        except VlobInMaintenanceError:
            return authenticated_cmds.latest.vlob_read.RepInMaintenance()

        return authenticated_cmds.latest.vlob_read.RepOk(
            version,
            blob,
            author,
            created_on,
            certificate_index,
        )

    @api
    async def apiv3_vlob_update(
        self, client_ctx: AuthenticatedClientContext, req: authenticated_cmds.v3.vlob_update.Req
    ) -> authenticated_cmds.v3.vlob_update.Rep:
        # `vlob_update` command is similar between APIv3 and v4+ from the server
        # point of view.
        # (from client point of view, server may return `bad_timestamp` response
        # with some fields missing)
        # TODO: proper api req/rep conversion
        return await self.api_vlob_update(client_ctx, req)  # type: ignore[return-value, arg-type]

    @api
    async def api_vlob_update(
        self, client_ctx: AuthenticatedClientContext, req: authenticated_cmds.latest.vlob_update.Req
    ) -> authenticated_cmds.latest.vlob_update.Rep:
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
            return authenticated_cmds.latest.vlob_update.RepBadTimestamp(
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
            return authenticated_cmds.latest.vlob_update.RepNotFound(None)

        except VlobAccessError:
            return authenticated_cmds.latest.vlob_update.RepNotAllowed()

        except VlobRequireGreaterTimestampError as exc:
            return authenticated_cmds.latest.vlob_update.RepRequireGreaterTimestamp(
                exc.strictly_greater_than
            )

        except VlobVersionError:
            return authenticated_cmds.latest.vlob_update.RepBadVersion()

        except VlobEncryptionRevisionError:
            return authenticated_cmds.latest.vlob_update.RepBadEncryptionRevision()

        except VlobInMaintenanceError:
            return authenticated_cmds.latest.vlob_update.RepInMaintenance()

        except VlobSequesterDisabledError:
            return authenticated_cmds.latest.vlob_update.RepNotASequesteredOrganization()

        except VlobSequesterServiceInconsistencyError as exc:
            return authenticated_cmds.latest.vlob_update.RepSequesterInconsistency(
                sequester_authority_certificate=exc.sequester_authority_certificate,
                sequester_services_certificates=exc.sequester_services_certificates,
            )

        except VlobSequesterWebhookRejectionError as exc:
            return authenticated_cmds.latest.vlob_update.RepRejectedBySequesterService(
                service_id=exc.service_id, service_label=exc.service_label, reason=exc.reason
            )

        except VlobSequesterWebhookUnavailableError:
            return authenticated_cmds.latest.vlob_update.RepTimeout()

        return authenticated_cmds.latest.vlob_update.RepOk()

    @api
    async def api_vlob_poll_changes(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.vlob_poll_changes.Req,
    ) -> authenticated_cmds.latest.vlob_poll_changes.Rep:
        # TODO: raise error if too many events since offset ?
        try:
            checkpoint, changes = await self.poll_changes(
                client_ctx.organization_id,
                client_ctx.device_id,
                realm_id=req.realm_id,
                checkpoint=req.last_checkpoint,
            )

        except VlobAccessError:
            return authenticated_cmds.latest.vlob_poll_changes.RepNotAllowed()

        except VlobRealmNotFoundError:
            return authenticated_cmds.latest.vlob_poll_changes.RepNotFound(None)

        except VlobInMaintenanceError:
            return authenticated_cmds.latest.vlob_poll_changes.RepInMaintenance()

        return authenticated_cmds.latest.vlob_poll_changes.RepOk(changes, checkpoint)

    @api
    async def api_vlob_list_versions(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.vlob_list_versions.Req,
    ) -> authenticated_cmds.latest.vlob_list_versions.Rep:
        try:
            versions_dict = await self.list_versions(
                client_ctx.organization_id, client_ctx.device_id, vlob_id=req.vlob_id
            )

        except VlobAccessError:
            return authenticated_cmds.latest.vlob_list_versions.RepNotAllowed()

        except VlobNotFoundError:
            return authenticated_cmds.latest.vlob_list_versions.RepNotFound(None)

        except VlobInMaintenanceError:
            return authenticated_cmds.latest.vlob_list_versions.RepInMaintenance()

        return authenticated_cmds.latest.vlob_list_versions.RepOk(versions_dict)

    @api
    async def api_vlob_maintenance_get_reencryption_batch(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.vlob_maintenance_get_reencryption_batch.Req,
    ) -> authenticated_cmds.latest.vlob_maintenance_get_reencryption_batch.Rep:
        try:
            batch = await self.maintenance_get_reencryption_batch(
                client_ctx.organization_id,
                client_ctx.device_id,
                realm_id=req.realm_id,
                encryption_revision=req.encryption_revision,
                size=req.size,
            )

        except VlobAccessError:
            return authenticated_cmds.latest.vlob_maintenance_get_reencryption_batch.RepNotAllowed()

        except VlobRealmNotFoundError:
            return authenticated_cmds.latest.vlob_maintenance_get_reencryption_batch.RepNotFound(
                None
            )

        except VlobNotInMaintenanceError:
            return authenticated_cmds.latest.vlob_maintenance_get_reencryption_batch.RepNotInMaintenance(
                None
            )

        except VlobEncryptionRevisionError:
            return (
                authenticated_cmds.latest.vlob_maintenance_get_reencryption_batch.RepBadEncryptionRevision()
            )

        except VlobMaintenanceError:
            return authenticated_cmds.latest.vlob_maintenance_get_reencryption_batch.RepMaintenanceError(
                None
            )

        return authenticated_cmds.latest.vlob_maintenance_get_reencryption_batch.RepOk(
            [ReencryptionBatchEntry(vlob_id, version, blob) for vlob_id, version, blob in batch]
        )

    @api
    async def api_vlob_maintenance_save_reencryption_batch(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.vlob_maintenance_save_reencryption_batch.Req,
    ) -> authenticated_cmds.latest.vlob_maintenance_save_reencryption_batch.Rep:
        try:
            total, done = await self.maintenance_save_reencryption_batch(
                client_ctx.organization_id,
                client_ctx.device_id,
                realm_id=req.realm_id,
                encryption_revision=req.encryption_revision,
                batch=[(x.vlob_id, x.version, x.blob) for x in req.batch],
            )

        except VlobAccessError:
            return (
                authenticated_cmds.latest.vlob_maintenance_save_reencryption_batch.RepNotAllowed()
            )

        # No need to catch VlobNotFoundError given unknown vlob/version in batch are ignored
        except VlobRealmNotFoundError:
            return authenticated_cmds.latest.vlob_maintenance_save_reencryption_batch.RepNotFound(
                None
            )

        except VlobNotInMaintenanceError:
            return authenticated_cmds.latest.vlob_maintenance_save_reencryption_batch.RepNotInMaintenance(
                None
            )

        except VlobEncryptionRevisionError:
            return (
                authenticated_cmds.latest.vlob_maintenance_save_reencryption_batch.RepBadEncryptionRevision()
            )

        except VlobMaintenanceError:
            return authenticated_cmds.latest.vlob_maintenance_save_reencryption_batch.RepMaintenanceError(
                None
            )

        return authenticated_cmds.latest.vlob_maintenance_save_reencryption_batch.RepOk(total, done)

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
        sequester_blob: Dict[SequesterServiceID, bytes] | None = None,
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
        version: int | None = None,
        timestamp: DateTime | None = None,
    ) -> Tuple[int, bytes, DeviceID, DateTime, DateTime, int]:
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
        sequester_blob: Dict[SequesterServiceID, bytes] | None = None,
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
