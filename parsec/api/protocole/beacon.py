# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.serde import UnknownCheckedSchema, fields
from parsec.api.protocole.base import BaseReqSchema, BaseRepSchema, CmdSerializer


__all__ = ("beacon_set_rights_serializer", "beacon_get_rights_serializer", "beacon_poll_serializer")


# TODO: rename {Get|Set}Rights => {Get|Set}UserRights (or change api to set all rights ?)


class BeaconSetRightsReqSchema(BaseReqSchema):
    id = fields.UUID(required=True)
    user = fields.UserID(required=True)
    admin_access = fields.Boolean(required=True)
    read_access = fields.Boolean(required=True)
    write_access = fields.Boolean(required=True)


class BeaconSetRightsRepSchema(BaseRepSchema):
    pass


beacon_set_rights_serializer = CmdSerializer(BeaconSetRightsReqSchema, BeaconSetRightsRepSchema)


class BeaconGetRightsReqSchema(BaseReqSchema):
    id = fields.UUID(required=True)


class BeaconUserRights(UnknownCheckedSchema):
    admin_access = fields.Boolean(required=True)
    read_access = fields.Boolean(required=True)
    write_access = fields.Boolean(required=True)


class BeaconGetRightsRepSchema(BaseRepSchema):
    users = fields.Map(fields.UserID(), fields.Nested(BeaconUserRights), required=True)


beacon_get_rights_serializer = CmdSerializer(BeaconGetRightsReqSchema, BeaconGetRightsRepSchema)


class BeaconPollReqSchema(BaseReqSchema):
    id = fields.UUID(required=True)
    last_checkpoint = fields.Integer(required=True)


class BeaconPollRepSchema(BaseRepSchema):
    changes = fields.Map(fields.UUID(), fields.Integer(), required=True)
    current_checkpoint = fields.Integer(required=True)


beacon_poll_serializer = CmdSerializer(BeaconPollReqSchema, BeaconPollRepSchema)
