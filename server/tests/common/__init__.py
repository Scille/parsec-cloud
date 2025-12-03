# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from pytest import LogCaptureFixture as VanillaLogCaptureFixture

from parsec.cli.testbed import next_organization_id

# ruff: noqa: F403
from .account import *
from .administration import *
from .backend import *
from .client import *
from .data import *
from .letter_box import *
from .pki import *
from .postgresql import *
from .utils import *

next_organization_id = next_organization_id  # Re-export for convenience


# customized in `tests/conftest.py`
class LogCaptureFixture(VanillaLogCaptureFixture):  # type: ignore[misc]
    def assert_occurred(self, log: str) -> None: ...

    def assert_occurred_once(self, log: str) -> None: ...

    def assert_not_occurred(self, log: str) -> None: ...
