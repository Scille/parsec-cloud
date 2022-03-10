# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from parsec.serde import fields, BaseSchema, JSONSerializer
from parsec.api.protocol.base import BaseReqSchema, BaseRepSchema, CmdSerializer
from parsec.api.protocol.types import (
    OrganizationIDField,
    DeviceIDField,
    UserProfileField,
    DeviceLabelField,
)


class APIV1_OrganizationBootstrapReqSchema(BaseReqSchema):
    bootstrap_token = fields.String(required=True)
    root_verify_key = fields.VerifyKey(required=True)
    user_certificate = fields.Bytes(required=True)
    device_certificate = fields.Bytes(required=True)
    # Same certificates than above, but expurged of human_handle/device_label
    # Backward compatibility prevent those field to be required, however
    # they should be considered so by recent version of Parsec (hence the
    # `allow_none=False`).
    # Hence only old version of Parsec will provide a payload with missing
    # redacted fields. In such case we consider the non-redacted can also
    # be used as redacted given the to-be-redacted fields have been introduce
    # in later version of Parsec.
    redacted_user_certificate = fields.Bytes(allow_none=False)
    redacted_device_certificate = fields.Bytes(allow_none=False)


class APIV1_OrganizationBootstrapRepSchema(BaseRepSchema):
    pass


# TODO: remove me once API v1 is deprecated
apiv1_organization_bootstrap_serializer = CmdSerializer(
    APIV1_OrganizationBootstrapReqSchema, APIV1_OrganizationBootstrapRepSchema
)


class OrganizationBootstrapWebhookSchema(BaseSchema):
    organization_id = OrganizationIDField(required=True)
    device_id = DeviceIDField(required=True)
    device_label = DeviceLabelField(allow_none=True, required=True)
    human_email = fields.String(allow_none=True, required=True)
    human_label = fields.String(allow_none=True, required=True)


organization_bootstrap_webhook_serializer = JSONSerializer(OrganizationBootstrapWebhookSchema)


class UsersPerProfileDetailItemSchema(BaseSchema):
    profile = UserProfileField(required=True)
    active = fields.Integer(required=True)
    revoked = fields.Integer(required=True)


class OrganizationStatsReqSchema(BaseReqSchema):
    pass


class OrganizationStatsRepSchema(BaseRepSchema):
    data_size = fields.Integer(required=True)
    metadata_size = fields.Integer(required=True)
    realms = fields.Integer(required=True)
    users = fields.Integer(required=True)
    active_users = fields.Integer(required=True)
    users_per_profile_detail = fields.List(
        fields.Nested(UsersPerProfileDetailItemSchema), required=True
    )


organization_stats_serializer = CmdSerializer(
    OrganizationStatsReqSchema, OrganizationStatsRepSchema
)


class OrganizationConfigReqSchema(BaseReqSchema):
    pass


class OrganizationConfigRepSchema(BaseRepSchema):
    user_profile_outsider_allowed = fields.Boolean(required=True)
    # `None` stands for "no limit" here
    active_users_limit = fields.Integer(allow_none=True, required=True)


organization_config_serializer = CmdSerializer(
    OrganizationConfigReqSchema, OrganizationConfigRepSchema
)
