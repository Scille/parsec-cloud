# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import DataError
from parsec.serde import SerdeValidationError


class DataValidationError(SerdeValidationError, DataError):
    pass
