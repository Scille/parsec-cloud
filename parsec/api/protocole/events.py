from parsec.schema import UnknownCheckedSchema, fields
from parsec.api.protocole.base import BaseReqSchema, BaseRepSchema, CmdSerializer


class BackendEventPingRepSchema(UnknownCheckedSchema):
    status = fields.CheckedConstant("ok", required=True)
    event = fields.CheckedConstant("ping", required=True)
    ping = fields.String(required=True)


class BackendEventDeviceTryClaimSubmittedRepSchema(UnknownCheckedSchema):
    status = fields.CheckedConstant("ok", required=True)
    event = fields.CheckedConstant("device.try_claim_submitted", required=True)
    device_name = fields.String(required=True)
    config_try_id = fields.String(required=True)


class BackendEventBeaconUpdatedRepSchema(UnknownCheckedSchema):
    status = fields.CheckedConstant("ok", required=True)
    event = fields.CheckedConstant("beacon.updated", required=True)
    beacon_id = fields.UUID(required=True)
    index = fields.Integer(required=True)
    src_id = fields.UUID(required=True)
    src_version = fields.Integer(required=True)


class BackendEventMessageReceivedRepSchema(UnknownCheckedSchema):
    status = fields.CheckedConstant("ok", required=True)
    event = fields.CheckedConstant("message.received", required=True)
    index = fields.Integer(required=True)


class BackendEventListenRepSchema(OneOfSchema):
    type_field = "event"
    type_field_remove = False
    type_schemas = {
        "ping": BackendEventPingRepSchema(),
        "device.try_claim_submitted": BackendEventDeviceTryClaimSubmittedRepSchema(),
        "beacon.updated": BackendEventBeaconUpdatedRepSchema(),
        "message.received": BackendEventMessageReceivedRepSchema(),
    }

    def get_obj_type(self, obj):
        return obj["event"]
