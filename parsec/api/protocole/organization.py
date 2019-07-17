# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.api.protocole.base import BaseRepSchema, BaseReqSchema, CmdSerializer
from parsec.serde import fields

__all__ = "organization_bootstrap_serializer"


class OrganizationCreateReqSchema(BaseReqSchema):
    organization_id = fields.OrganizationID(required=True)


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
