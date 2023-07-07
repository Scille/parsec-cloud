# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from typing import Any, Iterable

def zxcvbn(password: str, user_inputs: Iterable[object] | None) -> Any: ...
