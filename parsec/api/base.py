from parsec.schema import fields, ValidationError
from parsec.types import parse_user_id


class APIValidationError(Exception):
    pass


class UserIDField(fields.Field):
    def _deserialize(self, value, attr, data):
        try:
            parse_user_id(value)
        except ValueError as exc:
            raise ValidationError(str(exc)) from exc


__all__ = ("APIValidationError", "UserIDField")
