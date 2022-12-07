# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    OrganizationBootstrapRep,
    OrganizationBootstrapReq,
    OrganizationConfigRep,
    OrganizationConfigReq,
    OrganizationStatsRep,
    OrganizationStatsReq,
)
from parsec.api.protocol.base import ApiCommandSerializer
from parsec.api.protocol.types import DeviceIDField, DeviceLabelField, OrganizationIDField
from parsec.serde import BaseSchema, JSONSerializer, fields

organization_bootstrap_serializer = ApiCommandSerializer(
    OrganizationBootstrapReq, OrganizationBootstrapRep
)


class OrganizationBootstrapWebhookSchema(BaseSchema):
    organization_id = OrganizationIDField(required=True)
    device_id = DeviceIDField(required=True)
    device_label = DeviceLabelField(allow_none=True, required=True)
    human_email = fields.String(allow_none=True, required=True)
    human_label = fields.String(allow_none=True, required=True)


organization_bootstrap_webhook_serializer = JSONSerializer(OrganizationBootstrapWebhookSchema)


organization_stats_serializer = ApiCommandSerializer(OrganizationStatsReq, OrganizationStatsRep)

organization_config_serializer = ApiCommandSerializer(OrganizationConfigReq, OrganizationConfigRep)
