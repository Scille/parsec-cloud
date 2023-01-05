# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Type

from marshmallow.fields import Field

from parsec._parsec import (
    InvitationDeletedReason,
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
from parsec.api.protocol.base import ApiCommandSerializer
from parsec.serde import fields

__all__ = (
    "InvitationDeletedReason",
    "InvitationStatus",
    "InvitationStatusField",
    "InvitationEmailSentStatus",
    "InvitationToken",
    "InvitationTokenField",
    "InvitationType",
    "InvitationTypeField",
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


InvitationTokenField: Type[Field[InvitationToken]] = fields.uuid_based_field_factory(
    InvitationToken
)
InvitationStatusField = fields.rust_enum_field_factory(InvitationStatus)
InvitationTypeField: Type[Field[InvitationType]] = fields.rust_enum_field_factory(InvitationType)


invite_new_serializer = ApiCommandSerializer(InviteNewReq, InviteNewRep)

invite_delete_serializer = ApiCommandSerializer(InviteDeleteReq, InviteDeleteRep)


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
