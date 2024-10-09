# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

# /!\ Autogenerated by misc/gen_protocol_typings.py, any modification will be lost !

from __future__ import annotations

from . import tos_accept, tos_get

class AnyCmdReq:
    @classmethod
    def load(cls, raw: bytes) -> tos_accept.Req | tos_get.Req: ...

__all__ = ["AnyCmdReq", "tos_accept", "tos_get"]