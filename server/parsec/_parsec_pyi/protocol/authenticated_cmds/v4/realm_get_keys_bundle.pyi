# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

# /!\ Autogenerated by misc/gen_protocol_typings.py, any modification will be lost !

from __future__ import annotations

from parsec._parsec import VlobID

class Req:
    def __init__(self, realm_id: VlobID, key_index: int) -> None: ...
    def dump(self) -> bytes: ...
    @property
    def realm_id(self) -> VlobID: ...
    @property
    def key_index(self) -> int: ...

class Rep:
    @staticmethod
    def load(raw: bytes) -> Rep: ...
    def dump(self) -> bytes: ...

class RepUnknownStatus(Rep):
    def __init__(self, status: str, reason: str | None) -> None: ...
    @property
    def status(self) -> str: ...
    @property
    def reason(self) -> str | None: ...

class RepOk(Rep):
    def __init__(self, keys_bundle_access: bytes, keys_bundle: bytes) -> None: ...
    @property
    def keys_bundle_access(self) -> bytes: ...
    @property
    def keys_bundle(self) -> bytes: ...

class RepAccessNotAvailableForAuthor(Rep):
    def __init__(
        self,
    ) -> None: ...

class RepAuthorNotAllowed(Rep):
    def __init__(
        self,
    ) -> None: ...

class RepBadKeyIndex(Rep):
    def __init__(
        self,
    ) -> None: ...
