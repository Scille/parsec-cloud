from parsec.schema import fields, ValidationError
from parsec.types import DeviceID, UserID, DeviceName


class APIValidationError(Exception):
    pass


class DeviceIDField(fields.Field):
    def _deserialize(self, value, attr, data):
        try:
            return DeviceID(value)
        except ValueError as exc:
            raise ValidationError(str(exc)) from exc


class UserIDField(fields.Field):
    def _deserialize(self, value, attr, data):
        try:
            return UserID(value)
        except ValueError as exc:
            raise ValidationError(str(exc)) from exc


class DeviceNameField(fields.Field):
    def _deserialize(self, value, attr, data):
        try:
            return DeviceName(value)
        except ValueError as exc:
            raise ValidationError(str(exc)) from exc


__all__ = ("APIValidationError", "DeviceIDField", "UserIDField", "DeviceNameField")
