# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations


class SerdeError(Exception):
    pass


class SerdeValidationError(SerdeError):
    def __init__(self, errors: dict[str, str] | str):
        self.errors = errors


class SerdePackingError(SerdeError):
    pass
