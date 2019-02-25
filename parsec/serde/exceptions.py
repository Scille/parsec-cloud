# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


class SerdeError(Exception):
    pass


class SerdeValidationError(SerdeError):
    def __init__(self, errors: dict):
        self.errors = errors


class SerdePackingError(SerdeError):
    pass
