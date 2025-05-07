# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from pytest import LogCaptureFixture as VanillaLogCaptureFixture

from .backend import *  # noqa
from .client import *  # noqa
from .data import *  # noqa
from .letter_box import *  # noqa
from .postgresql import *  # noqa


# customized in `tests/conftest.py`
class LogCaptureFixture(VanillaLogCaptureFixture):  # type: ignore[misc]
    def assert_occurred(self, log: str) -> None: ...

    def assert_occurred_once(self, log: str) -> None: ...

    def assert_not_occurred(self, log: str) -> None: ...
