# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS
from typing import Union


class SerdeError(Exception):
    pass


class SerdeValidationError(SerdeError):
    def __init__(self, errors: Union[dict, str]):
        self.errors = errors


class SerdePackingError(SerdeError):
    pass
