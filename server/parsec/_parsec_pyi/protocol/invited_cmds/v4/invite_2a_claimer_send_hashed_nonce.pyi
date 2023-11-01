# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

# /!\ Autogenerated by misc/gen_protocol_typings.py, any modification will be lost !

from __future__ import annotations

from parsec._parsec import HashDigest

class Req:
    def __init__(self, claimer_hashed_nonce: HashDigest) -> None: ...
    def dump(self) -> bytes: ...
    @property
    def claimer_hashed_nonce(self) -> HashDigest: ...

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
    def __init__(self, greeter_nonce: bytes) -> None: ...
    @property
    def greeter_nonce(self) -> bytes: ...

class RepEnrollmentWrongState(Rep):
    def __init__(
        self,
    ) -> None: ...
