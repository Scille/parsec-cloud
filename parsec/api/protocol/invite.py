# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from enum import Enum
from typing import Dict, cast, Type

from parsec.serde import BaseSchema, OneOfSchema, fields
from parsec.api.protocol.base import BaseReqSchema, BaseRepSchema, CmdSerializer
from parsec.api.protocol.types import HumanHandleField, UserIDField

from parsec._parsec import InvitationToken

__all__ = (
    "InvitationToken",
    "InvitationTokenField",
    "invite_new_serializer",
    "invite_delete_serializer",
    "invite_list_serializer",
    "invite_info_serializer",
    "invite_1_claimer_wait_peer_serializer",
    "invite_1_greeter_wait_peer_serializer",
    "invite_2a_claimer_send_hashed_nonce_serializer",
    "invite_2a_greeter_get_hashed_nonce_serializer",
    "invite_2b_greeter_send_nonce_serializer",
    "invite_2b_claimer_send_nonce_serializer",
    "invite_3a_greeter_wait_peer_trust_serializer",
    "invite_3b_claimer_wait_peer_trust_serializer",
    "invite_3a_claimer_signify_trust_serializer",
    "invite_3b_greeter_signify_trust_serializer",
    "invite_4_greeter_communicate_serializer",
    "invite_4_claimer_communicate_serializer",
)


class InvitationType(Enum):
    USER = "USER"
    DEVICE = "DEVICE"


InvitationTokenField: Type[fields.Field] = fields.uuid_based_field_factory(InvitationToken)
InvitationTypeField: Type[fields.BaseEnumField] = fields.enum_field_factory(InvitationType)


class InviteNewUserReqSchema(BaseReqSchema):
    type = fields.EnumCheckedConstant(InvitationType.USER, required=True)
    claimer_email = fields.String(required=True)
    send_email = fields.Boolean(required=True)


class InviteNewDeviceReqSchema(BaseReqSchema):
    type = fields.EnumCheckedConstant(InvitationType.DEVICE, required=True)
    send_email = fields.Boolean(required=True)


class InviteNewReqSchema(OneOfSchema):
    type_field = "type"
    type_schemas = {
        InvitationType.USER: InviteNewUserReqSchema(),
        InvitationType.DEVICE: InviteNewDeviceReqSchema(),
    }

    def get_obj_type(self, obj: Dict[str, object]) -> InvitationType:
        return cast(InvitationType, obj["type"])


class InvitationEmailSentStatus(Enum):
    SUCCESS = "SUCCESS"
    NOT_AVAILABLE = "NOT_AVAILABLE"
    BAD_RECIPIENT = "BAD_RECIPIENT"


InvitationEmailSentStatusField = fields.enum_field_factory(InvitationEmailSentStatus)


class InviteNewRepSchema(BaseRepSchema):
    token = InvitationTokenField(required=True)
    # Added in API 2.3 (Parsec v2.6.0)
    # Field used when the invitation is correctly created but the invitation email cannot be sent
    email_sent = InvitationEmailSentStatusField(required=False, allow_none=False)


invite_new_serializer = CmdSerializer(InviteNewReqSchema, InviteNewRepSchema)


class InvitationDeletedReason(Enum):
    FINISHED = "FINISHED"
    CANCELLED = "CANCELLED"
    ROTTEN = "ROTTEN"


InvitationDeletedReasonField = fields.enum_field_factory(InvitationDeletedReason)


class InviteDeleteReqSchema(BaseReqSchema):
    token = InvitationTokenField(required=True)
    reason = InvitationDeletedReasonField(required=True)


class InviteDeleteRepSchema(BaseRepSchema):
    pass


invite_delete_serializer = CmdSerializer(InviteDeleteReqSchema, InviteDeleteRepSchema)


class InviteListReqSchema(BaseReqSchema):
    pass


class InvitationStatus(Enum):
    IDLE = "IDLE"
    READY = "READY"  # TODO: rename to CLAIMER_ONLINE ?
    DELETED = "DELETED"


InvitationStatusField: Type[fields.BaseEnumField] = fields.enum_field_factory(InvitationStatus)


class InviteListItemUserSchema(BaseSchema):
    type = fields.EnumCheckedConstant(InvitationType.USER, required=True)
    token = InvitationTokenField(required=True)
    created_on = fields.DateTime(required=True)
    claimer_email = fields.String(required=True)
    status = InvitationStatusField(required=True)


class InviteListItemDeviceSchema(BaseSchema):
    type = fields.EnumCheckedConstant(InvitationType.DEVICE, required=True)
    token = InvitationTokenField(required=True)
    created_on = fields.DateTime(required=True)
    status = InvitationStatusField(required=True)


class InviteListItemSchema(OneOfSchema):
    type_field = "type"
    type_schemas = {
        InvitationType.USER: InviteListItemUserSchema(),
        InvitationType.DEVICE: InviteListItemDeviceSchema(),
    }

    def get_obj_type(self, obj: Dict[str, object]) -> InvitationType:
        return cast(InvitationType, obj["type"])


class InviteListRepSchema(BaseRepSchema):
    invitations = fields.List(fields.Nested(InviteListItemSchema), required=True)


invite_list_serializer = CmdSerializer(InviteListReqSchema, InviteListRepSchema)


class InviteInfoReqSchema(BaseReqSchema):
    pass


class InviteInfoUserRepSchema(BaseRepSchema):
    type = fields.EnumCheckedConstant(InvitationType.USER, required=True)
    claimer_email = fields.String(required=True)
    greeter_user_id = UserIDField(required=True)
    greeter_human_handle = HumanHandleField(allow_none=True, required=True)


class InviteInfoDeviceRepSchema(BaseRepSchema):
    type = fields.EnumCheckedConstant(InvitationType.DEVICE, required=True)
    greeter_user_id = UserIDField(required=True)
    greeter_human_handle = HumanHandleField(allow_none=True, required=True)


class InviteInfoRepSchema(OneOfSchema):
    type_field = "type"
    type_schemas = {
        InvitationType.USER: InviteInfoUserRepSchema(),
        InvitationType.DEVICE: InviteInfoDeviceRepSchema(),
    }

    def get_obj_type(self, obj: Dict[str, object]) -> InvitationType:
        return cast(InvitationType, obj["type"])


invite_info_serializer = CmdSerializer(InviteInfoReqSchema, InviteInfoRepSchema)


class Invite1ClaimerWaitPeerReqSchema(BaseReqSchema):
    claimer_public_key = fields.PublicKey(required=True)


class Invite1ClaimerWaitPeerRepSchema(BaseRepSchema):
    greeter_public_key = fields.PublicKey(required=True)


invite_1_claimer_wait_peer_serializer = CmdSerializer(
    Invite1ClaimerWaitPeerReqSchema, Invite1ClaimerWaitPeerRepSchema
)


class Invite1GreeterWaitPeerReqSchema(BaseReqSchema):
    token = InvitationTokenField(required=True)
    greeter_public_key = fields.PublicKey(required=True)


class Invite1GreeterWaitPeerRepSchema(BaseRepSchema):
    claimer_public_key = fields.PublicKey(required=True)


invite_1_greeter_wait_peer_serializer = CmdSerializer(
    Invite1GreeterWaitPeerReqSchema, Invite1GreeterWaitPeerRepSchema
)


class Invite2aClaimerSendHashedNonceHashNonceReqSchema(BaseReqSchema):
    claimer_hashed_nonce = fields.HashDigest(required=True)


class Invite2aClaimerSendHashedNonceHashNonceRepSchema(BaseRepSchema):
    greeter_nonce = fields.Bytes(required=True)


invite_2a_claimer_send_hashed_nonce_serializer = CmdSerializer(
    Invite2aClaimerSendHashedNonceHashNonceReqSchema,
    Invite2aClaimerSendHashedNonceHashNonceRepSchema,
)


class Invite2aGreeterGetHashedNonceReqSchema(BaseReqSchema):
    token = InvitationTokenField(required=True)


class Invite2aGreeterGetHashedNonceRepSchema(BaseRepSchema):
    claimer_hashed_nonce = fields.HashDigest(required=True)


invite_2a_greeter_get_hashed_nonce_serializer = CmdSerializer(
    Invite2aGreeterGetHashedNonceReqSchema, Invite2aGreeterGetHashedNonceRepSchema
)


class Invite2bGreeterSendNonceReqSchema(BaseReqSchema):
    token = InvitationTokenField(required=True)
    greeter_nonce = fields.Bytes(required=True)


class Invite2bGreeterSendNonceRepSchema(BaseRepSchema):
    claimer_nonce = fields.Bytes(required=True)


invite_2b_greeter_send_nonce_serializer = CmdSerializer(
    Invite2bGreeterSendNonceReqSchema, Invite2bGreeterSendNonceRepSchema
)


class Invite2bClaimerSendNonceReqSchema(BaseReqSchema):
    claimer_nonce = fields.Bytes(required=True)


class Invite2bClaimerSendNonceRepSchema(BaseRepSchema):
    pass


invite_2b_claimer_send_nonce_serializer = CmdSerializer(
    Invite2bClaimerSendNonceReqSchema, Invite2bClaimerSendNonceRepSchema
)


class Invite3aGreeterWaitPeerTrustReqSchema(BaseReqSchema):
    token = InvitationTokenField(required=True)


class Invite3aGreeterWaitPeerTrustRepSchema(BaseRepSchema):
    pass


invite_3a_greeter_wait_peer_trust_serializer = CmdSerializer(
    Invite3aGreeterWaitPeerTrustReqSchema, Invite3aGreeterWaitPeerTrustRepSchema
)


class Invite3bClaimerWaitPeerTrustReqSchema(BaseReqSchema):
    pass


class Invite3bClaimerWaitPeerTrustRepSchema(BaseRepSchema):
    pass


invite_3b_claimer_wait_peer_trust_serializer = CmdSerializer(
    Invite3bClaimerWaitPeerTrustReqSchema, Invite3bClaimerWaitPeerTrustRepSchema
)


class Invite3bGreeterSignifyTrustReqSchema(BaseReqSchema):
    token = InvitationTokenField(required=True)


class Invite3bGreeterSignifyTrustRepSchema(BaseRepSchema):
    pass


invite_3b_greeter_signify_trust_serializer = CmdSerializer(
    Invite3bGreeterSignifyTrustReqSchema, Invite3bGreeterSignifyTrustRepSchema
)


class Invite3aClaimerSignifyTrustReqSchema(BaseReqSchema):
    pass


class Invite3aClaimerSignifyTrustRepSchema(BaseRepSchema):
    pass


invite_3a_claimer_signify_trust_serializer = CmdSerializer(
    Invite3aClaimerSignifyTrustReqSchema, Invite3aClaimerSignifyTrustRepSchema
)


class Invite4GreeterCommunicateReqSchema(BaseReqSchema):
    token = InvitationTokenField(required=True)
    payload = fields.Bytes(required=True)


class Invite4GreeterCommunicateRepSchema(BaseRepSchema):
    payload = fields.Bytes(required=True)


invite_4_greeter_communicate_serializer = CmdSerializer(
    Invite4GreeterCommunicateReqSchema, Invite4GreeterCommunicateRepSchema
)


class Invite4ClaimerCommunicateReqSchema(BaseReqSchema):
    payload = fields.Bytes(required=True)


class Invite4ClaimerCommunicateRepSchema(BaseRepSchema):
    payload = fields.Bytes(required=True)


invite_4_claimer_communicate_serializer = CmdSerializer(
    Invite4ClaimerCommunicateReqSchema, Invite4ClaimerCommunicateRepSchema
)
