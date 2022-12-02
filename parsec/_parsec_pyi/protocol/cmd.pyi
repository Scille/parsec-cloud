# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

from typing import Any

class AuthenticatedAnyCmdReq:
    def dump(self) -> bytes: ...
    @classmethod
    def load(cls, buf: bytes) -> Any: ...

class InvitedAnyCmdReq:
    def dump(self) -> bytes: ...
    @classmethod
    def load(cls, buf: bytes) -> Any: ...

class AnonymousAnyCmdReq:
    def dump(self) -> bytes: ...
    @classmethod
    def load(cls, buf: bytes) -> Any: ...
