# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    OrganizationStatsReq,
    OrganizationStatsRep,
    OrganizationConfigReq,
    OrganizationConfigRep,
)
from parsec.serde import fields, BaseSchema, JSONSerializer
from parsec.api.protocol.base import (
    ApiCommandSerializer,
    BaseReqSchema,
    BaseRepSchema,
    CmdSerializer,
)
from parsec.api.protocol.types import (
    OrganizationIDField,
    DeviceIDField,
    DeviceLabelField,
)


class APIV1_OrganizationBootstrapReqSchema(BaseReqSchema):
    bootstrap_token = fields.String(required=True)
    root_verify_key = fields.VerifyKey(required=True)
    user_certificate = fields.Bytes(required=True)
    device_certificate = fields.Bytes(required=True)
    # Added Parsec 1.14.0, but we forgot to bump API V1 revision :'(
    # Same certificates than above, but expurged of human_handle/device_label
    # Backward compatibility prevent those field to be required, however
    # they should be considered so by recent version of Parsec (hence the
    # `allow_none=False`).
    # Hence only old version of Parsec will provide a payload with missing
    # redacted fields. In such case we consider the non-redacted can also
    # be used as redacted given the to-be-redacted fields have been introduce
    # in later version of Parsec.
    redacted_user_certificate = fields.Bytes(required=False, allow_none=False)
    redacted_device_certificate = fields.Bytes(required=False, allow_none=False)


class APIV1_OrganizationBootstrapRepSchema(BaseRepSchema):
    pass


# TODO: remove me once API v1 is deprecated
apiv1_organization_bootstrap_serializer = CmdSerializer(
    APIV1_OrganizationBootstrapReqSchema, APIV1_OrganizationBootstrapRepSchema
)


class OrganizationBootstrapReqSchema(BaseReqSchema):
    bootstrap_token = fields.String(required=True)
    root_verify_key = fields.VerifyKey(required=True)
    user_certificate = fields.Bytes(required=True)
    device_certificate = fields.Bytes(required=True)
    redacted_user_certificate = fields.Bytes(required=True)
    redacted_device_certificate = fields.Bytes(required=True)
    # Added in API version 2.8/3.2 (Parsec 2.11.0)
    # Set to `None` for sequester disabled
    # Note there is absolutely no way to change this later as this certif must
    # be signed by the root key which has been destroyed after bootstrap
    sequester_authority_certificate = fields.Bytes(required=False, allow_none=True, missing=None)


class OrganizationBootstrapRepSchema(BaseRepSchema):
    pass


organization_bootstrap_serializer = CmdSerializer(
    OrganizationBootstrapReqSchema, OrganizationBootstrapRepSchema
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
