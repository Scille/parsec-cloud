# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

import abc
from typing import Any, BinaryIO

class BaseImage:
    @abc.abstractmethod
    def save(self, stream: BinaryIO, **kwargs: Any) -> None: ...
