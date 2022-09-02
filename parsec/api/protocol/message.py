# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from parsec._parsec import MessageGetReq, MessageGetRep
from parsec.api.protocol.base import ApiCommandSerializer


__all__ = ("message_get_serializer",)


message_get_serializer = ApiCommandSerializer(MessageGetReq, MessageGetRep)
