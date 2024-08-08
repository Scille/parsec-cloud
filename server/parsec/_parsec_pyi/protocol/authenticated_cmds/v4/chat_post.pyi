# /!\ Autogenerated by misc/gen_protocol_typings.py, any modification will be lost !

from __future__ import annotations

from parsec._parsec import DateTime


class Req:
    def __init__(self, messageContent: str) -> None: ...

    def dump(self) -> bytes: ...

    @property
    def messageContent(self) -> str: ...


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
    def __init__(self, ) -> None: ...


class RepInvalidMessage(Rep):
    def __init__(self, ) -> None: ...


class RepRecipientNotFound(Rep):
    def __init__(self, ) -> None: ...


class RepRecipientRevoked(Rep):
    def __init__(self, ) -> None: ...


class RepTimestampOutOfBallpark(Rep):
    def __init__(self, ballpark_client_early_offset: float,ballpark_client_late_offset: float,server_timestamp: DateTime,client_timestamp: DateTime) -> None: ...

    @property
    def ballpark_client_early_offset(self) -> float: ...

    @property
    def ballpark_client_late_offset(self) -> float: ...

    @property
    def client_timestamp(self) -> DateTime: ...

    @property
    def server_timestamp(self) -> DateTime: ...
