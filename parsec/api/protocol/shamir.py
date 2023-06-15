# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

from parsec._parsec import (
    InviteShamirRecoveryRevealRep,
    InviteShamirRecoveryRevealReq,
    ShamirRecoveryOthersListRep,
    ShamirRecoveryOthersListReq,
    ShamirRecoverySelfInfoRep,
    ShamirRecoverySelfInfoReq,
    ShamirRecoverySetupRep,
    ShamirRecoverySetupReq,
)
from parsec.api.protocol.base import ApiCommandSerializer

__all__ = (
    "shamir_recovery_others_list_serializer",
    "shamir_recovery_self_info_serializer",
    "shamir_recovery_setup_serializer",
    "invite_shamir_recovery_reveal_serializer",
)

shamir_recovery_others_list_serializer = ApiCommandSerializer(
    ShamirRecoveryOthersListReq, ShamirRecoveryOthersListRep
)
shamir_recovery_self_info_serializer = ApiCommandSerializer(
    ShamirRecoverySelfInfoReq, ShamirRecoverySelfInfoRep
)
shamir_recovery_setup_serializer = ApiCommandSerializer(
    ShamirRecoverySetupReq, ShamirRecoverySetupRep
)
invite_shamir_recovery_reveal_serializer = ApiCommandSerializer(
    InviteShamirRecoveryRevealReq, InviteShamirRecoveryRevealRep
)
