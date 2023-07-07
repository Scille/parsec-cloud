# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import DataError
from parsec.serde import SerdeValidationError


class DataValidationError(SerdeValidationError, DataError):
    pass
