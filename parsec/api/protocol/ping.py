# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    AuthenticatedPingReq,
    AuthenticatedPingRep,
    InvitedPingReq,
    InvitedPingRep,
)
from parsec.api.protocol.base import ApiCommandSerializer


__all__ = (
    "invited_ping_serializer",
    "authenticated_ping_serializer",
)


authenticated_ping_serializer = ApiCommandSerializer(AuthenticatedPingReq, AuthenticatedPingRep)
invited_ping_serializer = ApiCommandSerializer(InvitedPingReq, InvitedPingRep)
