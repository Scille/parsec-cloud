# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.serde import fields
from parsec.api.protocol.base import BaseReqSchema, BaseRepSchema, CmdSerializer
from parsec.api.protocol.types import OrganizationIDField


__all__ = "organization_bootstrap_serializer"


class OrganizationCreateReqSchema(BaseReqSchema):
    organization_id = OrganizationIDField(required=True)
    expiration_date = fields.DateTime(allow_none=True, required=False)


class OrganizationCreateRepSchema(BaseRepSchema):
    bootstrap_token = fields.String(required=True)
    expiration_date = fields.DateTime(allow_none=True, required=False)


organization_create_serializer = CmdSerializer(
    OrganizationCreateReqSchema, OrganizationCreateRepSchema
)


class OrganizationBootstrapReqSchema(BaseReqSchema):
    bootstrap_token = fields.String(required=True)
    user_certificate = fields.Bytes(required=True)
    device_certificate = fields.Bytes(required=True)
    root_verify_key = fields.VerifyKey(required=True)


class OrganizationBootstrapRepSchema(BaseRepSchema):
    pass


organization_bootstrap_serializer = CmdSerializer(
    OrganizationBootstrapReqSchema, OrganizationBootstrapRepSchema
)


class OrganizationStatsReqSchema(BaseReqSchema):
    organization_id = OrganizationIDField(required=True)


class OrganizationStatsRepSchema(BaseRepSchema):
    data_size = fields.Integer(required=True)
    metadata_size = fields.Integer(required=True)
    users = fields.Integer(required=True)


organization_stats_serializer = CmdSerializer(
    OrganizationStatsReqSchema, OrganizationStatsRepSchema
)


class OrganizationStatusReqSchema(BaseReqSchema):
    organization_id = OrganizationIDField(required=True)


class OrganizationStatusRepSchema(BaseRepSchema):
    is_bootstrapped = fields.Boolean(required=True)
    expiration_date = fields.DateTime(allow_none=True, required=False)


organization_status_serializer = CmdSerializer(
    OrganizationStatusReqSchema, OrganizationStatusRepSchema
)


class OrganizationUpdateReqSchema(BaseReqSchema):
    organization_id = OrganizationIDField(required=True)
    expiration_date = fields.DateTime(allow_none=True, required=False)


class OrganizationUpdateRepSchema(BaseRepSchema):
    pass


organization_update_serializer = CmdSerializer(
    OrganizationUpdateReqSchema, OrganizationUpdateRepSchema
)
