# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Union


class SerdeError(Exception):
    pass


class SerdeValidationError(SerdeError):
    def __init__(self, errors: Union[dict, str]):
        self.errors = errors


class SerdePackingError(SerdeError):
    pass
