# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS
from typing import Union


class SerdeError(Exception):
    pass


class SerdeValidationError(SerdeError):
    def __init__(self, errors: Union[dict, str]):
        self.errors = errors


class SerdePackingError(SerdeError):
    pass
