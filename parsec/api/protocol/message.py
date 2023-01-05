# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import MessageGetRep, MessageGetReq
from parsec.api.protocol.base import ApiCommandSerializer

__all__ = ("message_get_serializer",)


message_get_serializer = ApiCommandSerializer(MessageGetReq, MessageGetRep)
