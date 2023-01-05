# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import BlockCreateRep, BlockCreateReq, BlockID, BlockReadRep, BlockReadReq
from parsec.api.protocol.base import ApiCommandSerializer

__all__ = (
    "BlockID",
    "block_create_serializer",
    "block_read_serializer",
)

block_create_serializer = ApiCommandSerializer(BlockCreateReq, BlockCreateRep)
block_read_serializer = ApiCommandSerializer(BlockReadReq, BlockReadRep)
