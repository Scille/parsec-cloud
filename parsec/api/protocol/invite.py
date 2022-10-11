# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from enum import Enum
from typing import Optional, Type, Any
from marshmallow import ValidationError
from marshmallow.fields import Field

from parsec._parsec import (
    InvitationEmailSentStatus,
    InvitationStatus,
    InvitationToken,
    InvitationType,
    Invite1ClaimerWaitPeerRep,
    Invite1ClaimerWaitPeerReq,
    Invite1GreeterWaitPeerRep,
    Invite1GreeterWaitPeerReq,
    Invite2aClaimerSendHashedNonceRep,
    Invite2aClaimerSendHashedNonceReq,
    Invite2aGreeterGetHashedNonceRep,
    Invite2aGreeterGetHashedNonceReq,
    Invite2bClaimerSendNonceRep,
    Invite2bClaimerSendNonceReq,
    Invite2bGreeterSendNonceRep,
    Invite2bGreeterSendNonceReq,
    Invite3aClaimerSignifyTrustRep,
    Invite3aClaimerSignifyTrustReq,
    Invite3aGreeterWaitPeerTrustRep,
    Invite3aGreeterWaitPeerTrustReq,
    Invite3bClaimerWaitPeerTrustRep,
    Invite3bClaimerWaitPeerTrustReq,
    Invite3bGreeterSignifyTrustRep,
    Invite3bGreeterSignifyTrustReq,
    Invite4ClaimerCommunicateRep,
    Invite4ClaimerCommunicateReq,
    Invite4GreeterCommunicateRep,
    Invite4GreeterCommunicateReq,
    InviteDeleteRep,
    InviteDeleteReq,
    InviteInfoRep,
    InviteInfoReq,
    InviteListRep,
    InviteListReq,
    InviteNewRep,
    InviteNewReq,
)
from parsec.serde import fields
from parsec.api.protocol.base import ApiCommandSerializer


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


InvitationTokenField: Type[fields.Field] = fields.uuid_based_field_factory(InvitationToken)


class InvitationTypeField(Field):
    def _serialize(self, value: Any, attr: Any, obj: Any) -> Optional[str]:
        if value is None:
            return None

        if value == InvitationType.DEVICE:
            return "DEVICE"
        elif value == InvitationType.USER:
            return "USER"
        else:
            raise ValidationError(f"Not a InvitationType")

    def _deserialize(self, value: Any, attr: Any, data: Any) -> InvitationType:
        if not isinstance(value, str):
            raise ValidationError("Not string")

        if value == "DEVICE":
            return InvitationType.DEVICE
        elif value == "USER":
            return InvitationType.USER
        else:
            raise ValidationError(f"Invalid type `{value}`")


class InvitationEmailSentStatusField(Field):
    def _serialize(self, value: Any, attr: Any, obj: Any) -> Optional[str]:
        if value is None:
            return None
        elif isinstance(value, InvitationEmailSentStatus):
            return str(value)

        raise ValidationError(f"Invalid type `{value}`")

    def _deserialize(self, value: Any, attr: Any, data: Any) -> InvitationEmailSentStatus:
        if not isinstance(value, str):
            raise ValidationError("Not a string")

        try:
            return InvitationEmailSentStatus.from_str(value)
        except ValueError:
            raise ValidationError(f"Invalid type `{value}`")


invite_new_serializer = ApiCommandSerializer(InviteNewReq, InviteNewRep)


class InvitationDeletedReason(Enum):
    FINISHED = "FINISHED"
    CANCELLED = "CANCELLED"
    ROTTEN = "ROTTEN"


InvitationDeletedReasonField: Type[fields.BaseEnumField] = fields.enum_field_factory(
    InvitationDeletedReason
)


invite_delete_serializer = ApiCommandSerializer(InviteDeleteReq, InviteDeleteRep)


class InvitationStatusField(Field):
    def _serialize(self, value: Any, attr: Any, obj: Any) -> Optional[str]:
        if value is None:
            return None
        elif isinstance(value, InvitationStatus):
            return value.value

        raise ValidationError(f"Invalid type `{value}`")

    def _deserialize(self, value: Any, attr: Any, data: Any) -> InvitationStatus:
        if not isinstance(value, str):
            raise ValidationError("Not a string")

        try:
            return InvitationStatus.from_str(value)
        except ValueError:
            raise ValidationError(f"Invalid type `{value}`")


invite_list_serializer = ApiCommandSerializer(InviteListReq, InviteListRep)


invite_info_serializer = ApiCommandSerializer(InviteInfoReq, InviteInfoRep)


invite_1_claimer_wait_peer_serializer = ApiCommandSerializer(
    Invite1ClaimerWaitPeerReq, Invite1ClaimerWaitPeerRep
)


invite_1_greeter_wait_peer_serializer = ApiCommandSerializer(
    Invite1GreeterWaitPeerReq, Invite1GreeterWaitPeerRep
)


invite_2a_claimer_send_hashed_nonce_serializer = ApiCommandSerializer(
    Invite2aClaimerSendHashedNonceReq,
    Invite2aClaimerSendHashedNonceRep,
)

invite_2a_greeter_get_hashed_nonce_serializer = ApiCommandSerializer(
    Invite2aGreeterGetHashedNonceReq, Invite2aGreeterGetHashedNonceRep
)


invite_2b_greeter_send_nonce_serializer = ApiCommandSerializer(
    Invite2bGreeterSendNonceReq, Invite2bGreeterSendNonceRep
)


invite_2b_claimer_send_nonce_serializer = ApiCommandSerializer(
    Invite2bClaimerSendNonceReq, Invite2bClaimerSendNonceRep
)


invite_3a_greeter_wait_peer_trust_serializer = ApiCommandSerializer(
    Invite3aGreeterWaitPeerTrustReq, Invite3aGreeterWaitPeerTrustRep
)


invite_3b_claimer_wait_peer_trust_serializer = ApiCommandSerializer(
    Invite3bClaimerWaitPeerTrustReq, Invite3bClaimerWaitPeerTrustRep
)


invite_3b_greeter_signify_trust_serializer = ApiCommandSerializer(
    Invite3bGreeterSignifyTrustReq, Invite3bGreeterSignifyTrustRep
)


invite_3a_claimer_signify_trust_serializer = ApiCommandSerializer(
    Invite3aClaimerSignifyTrustReq, Invite3aClaimerSignifyTrustRep
)


invite_4_greeter_communicate_serializer = ApiCommandSerializer(
    Invite4GreeterCommunicateReq, Invite4GreeterCommunicateRep
)


invite_4_claimer_communicate_serializer = ApiCommandSerializer(
    Invite4ClaimerCommunicateReq, Invite4ClaimerCommunicateRep
)
