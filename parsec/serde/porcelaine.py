# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from marshmallow import ValidationError

from parsec.serde.exceptions import SerdeValidationError
from parsec.serde.packing import SerdePackingError, packb, unpackb


class Serializer:
    def __repr__(self):
        return f"{self.__class__.__name__}(schema={self.schema.__class__.__name__})"

    def __init__(
        self, schema_cls, validation_exc=SerdeValidationError, packing_exc=SerdePackingError
    ):
        if isinstance(validation_exc, SerdeValidationError):
            raise ValueError("validation_exc must subclass SerdeValidationError")
        if isinstance(packing_exc, SerdePackingError):
            raise ValueError("packing_exc must subclass SerdePackingError")

        self.validation_exc = validation_exc
        self.packing_exc = packing_exc
        self.schema = schema_cls(strict=True)

    def load(self, data: dict):
        """
        Raises:
            SerdeValidationError
        """
        try:
            return self.schema.load(data).data

        except (ValidationError,) as exc:
            raise self.validation_exc(exc.messages) from exc

    def dump(self, data) -> dict:
        """
        Raises:
            SerdeValidationError
        """
        try:
            return self.schema.dump(data).data

        except ValidationError as exc:
            raise SerdeValidationError(exc.messages) from exc

    def loads(self, data: bytes) -> dict:
        """
        Raises:
            SerdeValidationError
            SerdePackingError
        """
        return self.load(unpackb(data, self.packing_exc))

    def dumps(self, data: dict) -> bytes:
        """
        Raises:
            SerdeValidationError
            SerdePackingError
        """
        return packb(self.dump(data), self.packing_exc)
