# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    VlobCreateRep,
    VlobCreateReq,
    VlobID,
    VlobListVersionsRep,
    VlobListVersionsReq,
    VlobMaintenanceGetReencryptionBatchRep,
    VlobMaintenanceGetReencryptionBatchReq,
    VlobMaintenanceSaveReencryptionBatchRep,
    VlobMaintenanceSaveReencryptionBatchReq,
    VlobPollChangesRep,
    VlobPollChangesReq,
    VlobReadRep,
    VlobReadReq,
    VlobUpdateRep,
    VlobUpdateReq,
)
from parsec.api.protocol.base import ApiCommandSerializer
from parsec.serde import fields

__all__ = (
    "VlobID",
    "VlobIDField",
    "vlob_create_serializer",
    "vlob_read_serializer",
    "vlob_update_serializer",
    "vlob_poll_changes_serializer",
    "vlob_list_versions_serializer",
    "vlob_maintenance_get_reencryption_batch_serializer",
    "vlob_maintenance_save_reencryption_batch_serializer",
)


VlobIDField = fields.uuid_based_field_factory(VlobID)

vlob_create_serializer = ApiCommandSerializer(VlobCreateReq, VlobCreateRep)
vlob_read_serializer = ApiCommandSerializer(VlobReadReq, VlobReadRep)
vlob_update_serializer = ApiCommandSerializer(VlobUpdateReq, VlobUpdateRep)
vlob_poll_changes_serializer = ApiCommandSerializer(VlobPollChangesReq, VlobPollChangesRep)
vlob_list_versions_serializer = ApiCommandSerializer(VlobListVersionsReq, VlobListVersionsRep)

# Maintenance stuff
vlob_maintenance_get_reencryption_batch_serializer = ApiCommandSerializer(
    VlobMaintenanceGetReencryptionBatchReq, VlobMaintenanceGetReencryptionBatchRep
)
vlob_maintenance_save_reencryption_batch_serializer = ApiCommandSerializer(
    VlobMaintenanceSaveReencryptionBatchReq, VlobMaintenanceSaveReencryptionBatchRep
)
