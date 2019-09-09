# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.serde import fields
from parsec.api.protocol.base import BaseReqSchema, BaseRepSchema, CmdSerializer
from parsec.api.protocol.types import OrganizationIDField


__all__ = "organization_bootstrap_serializer"


class OrganizationCreateReqSchema(BaseReqSchema):
    organization_id = OrganizationIDField(required=True)


class OrganizationCreateRepSchema(BaseRepSchema):
    bootstrap_token = fields.String(required=True)


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
