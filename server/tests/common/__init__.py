# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from _pytest.logging import LogCaptureFixture as VanillaLogCaptureFixture

from .backend import *  # noqa
from .client import *  # noqa
from .postgresql import *  # noqa
from .realm import *  # noqa


# customized in `tests/conftest.py`
class LogCaptureFixture(VanillaLogCaptureFixture):  # type: ignore[misc]
    def assert_occurred(self, log: str) -> None:
        ...

    def assert_occurred_once(self, log: str) -> None:
        ...

    def assert_not_occurred(self, log: str) -> None:
        ...
