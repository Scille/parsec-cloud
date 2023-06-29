# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

class TimeProvider:
    """
    Taking the current time constitutes a side effect, on top of that we want to be able
    to simulate in our tests complex behavior where different Parsec client/server have
    shifting clocks.
    So the solution here is to force the current time to be taken from a non-global object
    (typically each client/server should have it own) that can be independently mocked.
    """

    def now(self) -> DateTime: ...
    async def sleep(self, time: float) -> None: ...
    def sleeping_stats(self) -> int: ...
    def new_child(self) -> "TimeProvider": ...
    def mock_time(
        self,
        freeze: DateTime | None = None,
        shift: float | None = None,
        realtime: bool = False,
        speed: float | None = None,
    ) -> None: ...

class DateTime:
    """
    A class representing DateTime
    """

    def __init__(
        self,
        year: int,
        month: int,
        day: int,
        hour: int = 0,
        minute: int = 0,
        second: int = 0,
        microsecond: int = 0,
    ) -> None: ...
    def __lt__(self, other: DateTime) -> bool: ...
    def __gt__(self, other: DateTime) -> bool: ...
    def __le__(self, other: DateTime) -> bool: ...
    def __ge__(self, other: DateTime) -> bool: ...
    def __hash__(self) -> int: ...
    def __sub__(self, other: DateTime) -> float: ...
    @property
    def year(self) -> int: ...
    @property
    def month(self) -> int: ...
    @property
    def day(self) -> int: ...
    @property
    def hour(self) -> int: ...
    @property
    def minute(self) -> int: ...
    @property
    def second(self) -> int: ...
    @property
    def microsecond(self) -> int: ...
    @staticmethod
    def now() -> DateTime: ...
    @staticmethod
    def from_timestamp(ts: float) -> DateTime: ...
    @staticmethod
    def from_rfc3339(value: str) -> DateTime: ...
    def timestamp(self) -> float: ...
    def add(
        self,
        days: int = 0,
        hours: int = 0,
        minutes: int = 0,
        seconds: int = 0,
        microseconds: int = 0,
    ) -> DateTime: ...
    def subtract(
        self,
        days: int = 0,
        hours: int = 0,
        minutes: int = 0,
        seconds: int = 0,
        microseconds: int = 0,
    ) -> DateTime: ...
    def to_local(self) -> LocalDateTime: ...
    def to_rfc3339(self) -> str: ...

class LocalDateTime:
    """
    A class representing LocalDateTime
    """

    def __init__(
        self,
        year: int,
        month: int,
        day: int,
        hour: int = 0,
        minute: int = 0,
        second: int = 0,
        microsecond: int = 0,
    ) -> None: ...
    @property
    def year(self) -> int: ...
    @property
    def month(self) -> int: ...
    @property
    def day(self) -> int: ...
    @property
    def hour(self) -> int: ...
    @property
    def minute(self) -> int: ...
    @property
    def second(self) -> int: ...
    @staticmethod
    def from_timestamp(ts: float) -> LocalDateTime: ...
    def timestamp(self) -> float: ...
    def to_utc(self) -> DateTime: ...
    def format(self, fmt: str) -> str: ...

def mock_time(time: DateTime | int | None) -> None: ...
