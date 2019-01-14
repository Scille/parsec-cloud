from parsec.serde import fields
from parsec.api.protocole.base import BaseReqSchema, BaseRepSchema, CmdSerializer


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
    certified_user = fields.Bytes(required=True)
    certified_device = fields.Bytes(required=True)
    root_verify_key = fields.VerifyKey(required=True)


class OrganizationBootstrapRepSchema(BaseRepSchema):
    pass


organization_bootstrap_serializer = CmdSerializer(
    OrganizationBootstrapReqSchema, OrganizationBootstrapRepSchema
)
