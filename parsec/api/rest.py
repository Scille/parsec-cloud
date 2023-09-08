# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from parsec.api.protocol.types import (
    DeviceIDField,
    DeviceLabelField,
    OrganizationIDField,
    UserProfileField,
)
from parsec.serde import BaseSchema, JSONSerializer, fields

### Webhooks ###


class OrganizationBootstrapWebhookSchema(BaseSchema):
    organization_id = OrganizationIDField(required=True)
    device_id = DeviceIDField(required=True)
    device_label = DeviceLabelField(allow_none=True, required=True)
    human_email = fields.String(allow_none=True, required=True)
    human_label = fields.String(allow_none=True, required=True)


organization_bootstrap_webhook_serializer = JSONSerializer(OrganizationBootstrapWebhookSchema)


### REST administration API ###


# POST /administration/organizations


class OrganizationCreateReqSchema(BaseSchema):
    organization_id = OrganizationIDField(required=True)
    # /!\ Missing field and field set to `None` does not mean the same thing:
    # - missing field: ask the backend to use it default value for this field
    # - field set to `None`: `None` is a valid value to use for this field
    user_profile_outsider_allowed = fields.Boolean(required=False, allow_none=False)
    # `None` stands for "no limit" here
    active_users_limit = fields.Integer(required=False, allow_none=True, validate=lambda x: x >= 0)
    # Introduced in API v2.10/v3.4 (Parsec 2.16+)
    minimum_archiving_period = fields.Integer(required=False, allow_none=False)


class OrganizationCreateRepSchema(BaseSchema):
    bootstrap_token = fields.String(required=True)


organization_create_req_serializer = JSONSerializer(OrganizationCreateReqSchema)
organization_create_rep_serializer = JSONSerializer(OrganizationCreateRepSchema)


# GET /administration/organizations/<organization_id>


class OrganizationConfigReqSchema(BaseSchema):
    pass


class OrganizationConfigRepSchema(BaseSchema):
    is_bootstrapped = fields.Boolean(required=True)
    is_expired = fields.Boolean(required=True)
    user_profile_outsider_allowed = fields.Boolean(required=True)
    # `None` stands for "no limit" here
    active_users_limit = fields.Integer(required=True, allow_none=True)
    # Introduced in API v2.10/v3.4 (Parsec 2.16+)
    minimum_archiving_period = fields.Integer(required=False)


organization_config_req_serializer = JSONSerializer(OrganizationConfigReqSchema)
organization_config_rep_serializer = JSONSerializer(OrganizationConfigRepSchema)


# GET /administration/organizations/<organization_id>/stats


class UsersPerProfileDetailItemSchema(BaseSchema):
    profile = UserProfileField(required=True)
    active = fields.Integer(required=True)
    revoked = fields.Integer(required=True)


class OrganizationStatsReqSchema(BaseSchema):
    pass


class OrganizationStatsRepSchema(BaseSchema):
    data_size = fields.Integer(required=True)
    metadata_size = fields.Integer(required=True)
    realms = fields.Integer(required=True)
    users = fields.Integer(required=True)
    active_users = fields.Integer(required=True)
    users_per_profile_detail = fields.List(
        fields.Nested(UsersPerProfileDetailItemSchema), required=True
    )


organization_stats_req_serializer = JSONSerializer(OrganizationStatsReqSchema)
organization_stats_rep_serializer = JSONSerializer(OrganizationStatsRepSchema)

# GET /administration/stats?format=<format>&at=<at>


class ServerStatsItem(BaseSchema):
    organization_id = fields.String(required=True)
    data_size = fields.Integer(required=True)
    metadata_size = fields.Integer(required=True)
    realms = fields.Integer(required=True)
    active_users = fields.Integer(required=True)
    users_per_profile_detail = fields.List(
        fields.Nested(UsersPerProfileDetailItemSchema), required=True
    )


class ServerStatsRepSchema(BaseSchema):
    stats = fields.List(fields.Nested(ServerStatsItem), required=True)


server_stats_rep_serializer = JSONSerializer(ServerStatsRepSchema)

# PATCH /administration/organizations/<organization_id>


class OrganizationUpdateReqSchema(BaseSchema):
    # /!\ Missing field and field set to `None` does not mean the same thing:
    # - missing field: don't modify this field
    # - field set to `None`: `None` is a valid value to use for this field
    is_expired = fields.Boolean(required=False)
    user_profile_outsider_allowed = fields.Boolean(required=False)
    # `None` stands for "no limit" here
    active_users_limit = fields.Integer(required=False, allow_none=True, validate=lambda x: x >= 0)
    # Introduced in API v2.10/v3.4 (Parsec 2.16+)
    minimum_archiving_period = fields.Integer(required=False)


class OrganizationUpdateRepSchema(BaseSchema):
    pass


organization_update_req_serializer = JSONSerializer(OrganizationUpdateReqSchema)
organization_update_rep_serializer = JSONSerializer(OrganizationUpdateRepSchema)
