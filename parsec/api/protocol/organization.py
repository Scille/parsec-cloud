# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.serde import fields
from parsec.api.protocol.base import BaseReqSchema, BaseRepSchema, CmdSerializer
from parsec.api.protocol.types import OrganizationIDField


__all__ = "organization_bootstrap_serializer"


class APIV1_OrganizationCreateReqSchema(BaseReqSchema):
    organization_id = OrganizationIDField(required=True)
    expiration_date = fields.DateTime(allow_none=True, required=False)


class APIV1_OrganizationCreateRepSchema(BaseRepSchema):
    bootstrap_token = fields.String(required=True)
    expiration_date = fields.DateTime(allow_none=True, required=False)


apiv1_organization_create_serializer = CmdSerializer(
    APIV1_OrganizationCreateReqSchema, APIV1_OrganizationCreateRepSchema
)


class APIV1_OrganizationBootstrapReqSchema(BaseReqSchema):
    bootstrap_token = fields.String(required=True)
    user_certificate = fields.Bytes(required=True)
    device_certificate = fields.Bytes(required=True)
    root_verify_key = fields.VerifyKey(required=True)


class APIV1_OrganizationBootstrapRepSchema(BaseRepSchema):
    pass


# TODO: remove me once API v1 is deprecated
apiv1_organization_bootstrap_serializer = CmdSerializer(
    APIV1_OrganizationBootstrapReqSchema, APIV1_OrganizationBootstrapRepSchema
)


class OrganizationBootstrapReqSchema(BaseReqSchema):
    user_certificate = fields.Bytes(required=True)
    device_certificate = fields.Bytes(required=True)
    root_verify_key = fields.VerifyKey(required=True)


class OrganizationBootstrapRepSchema(BaseRepSchema):
    pass


organization_bootstrap_serializer = CmdSerializer(
    OrganizationBootstrapReqSchema, OrganizationBootstrapRepSchema
)


class APIV1_OrganizationStatsReqSchema(BaseReqSchema):
    organization_id = OrganizationIDField(required=True)


class APIV1_OrganizationStatsRepSchema(BaseRepSchema):
    data_size = fields.Integer(required=True)
    metadata_size = fields.Integer(required=True)
    users = fields.Integer(required=True)


apiv1_organization_stats_serializer = CmdSerializer(
    APIV1_OrganizationStatsReqSchema, APIV1_OrganizationStatsRepSchema
)


class APIV1_OrganizationStatusReqSchema(BaseReqSchema):
    organization_id = OrganizationIDField(required=True)


class APIV1_OrganizationStatusRepSchema(BaseRepSchema):
    is_bootstrapped = fields.Boolean(required=True)
    expiration_date = fields.DateTime(allow_none=True, required=False)


apiv1_organization_status_serializer = CmdSerializer(
    APIV1_OrganizationStatusReqSchema, APIV1_OrganizationStatusRepSchema
)


class APIV1_OrganizationUpdateReqSchema(BaseReqSchema):
    organization_id = OrganizationIDField(required=True)
    expiration_date = fields.DateTime(allow_none=True, required=False)


class APIV1_OrganizationUpdateRepSchema(BaseRepSchema):
    pass


apiv1_organization_update_serializer = CmdSerializer(
    APIV1_OrganizationUpdateReqSchema, APIV1_OrganizationUpdateRepSchema
)
