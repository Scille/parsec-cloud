from msgpack import packb as msgpack_packb, unpackb as msgpack_unpackb
from msgpack.exceptions import ExtraData, FormatError, StackError

from parsec.schema import fields, UnknownCheckedSchema, OneOfSchema, post_load, ValidationError


__all__ = ("ProtocoleError", "BaseReqSchema", "BaseRepSchema", "CmdSerializer", "packb", "unpackb")


class ProtocoleError(Exception):
    pass


class MessageSerializationError(ProtocoleError):
    pass


class InvalidMessageError(ProtocoleError):
    def __init__(self, errors: dict):
        self.errors = errors


class BaseReqSchema(UnknownCheckedSchema):
    cmd = fields.String(required=True)

    @post_load
    def _drop_cmd_field(self, item):
        if self.drop_cmd_field:
            item.pop("cmd")
        return item

    def __init__(self, drop_cmd_field=True, **kwargs):
        super().__init__(**kwargs)
        self.drop_cmd_field = drop_cmd_field


class BaseRepSchema(UnknownCheckedSchema):
    status = fields.CheckedConstant("ok", required=True)


class ErrorRepSchema(BaseRepSchema):
    status = fields.String(required=True)
    reason = fields.String(allow_none=True)
    # TODO: should errors be better checked ?
    errors = fields.Dict(allow_none=True)


def packb(data: dict) -> bytes:
    """
    Raises:
        MessageSerializationError
    """
    try:
        return msgpack_packb(data, use_bin_type=True)

    except (ExtraData, ValueError, FormatError, StackError) as exc:
        raise MessageSerializationError(f"Invalid msgpack data: {exc}") from exc


def unpackb(raw_data: bytes) -> dict:
    """
    Raises:
        MessageSerializationError
    """
    try:
        return msgpack_unpackb(raw_data, raw=False)

    except (ExtraData, ValueError, FormatError, StackError) as exc:
        raise MessageSerializationError(f"Invalid msgpack data: {exc}") from exc


class Serializer:
    def __repr__(self):
        return f"{self.__class__.__name__}({self.req_schema.__class__.__name__})"

    def __init__(self, schema_cls):
        self.schema = schema_cls(strict=True)

    def load(self, data: dict):
        """
        Raises:
            ProtocoleError
        """
        try:
            return self.schema.load(data).data

        except (ValidationError,) as exc:
            raise InvalidMessageError(exc.messages) from exc

    def dump(self, data) -> dict:
        """
        Raises:
            ProtocoleError
        """
        try:
            return self.schema.dump(data).data

        except ValidationError as exc:
            raise InvalidMessageError(exc.messages) from exc

    def loads(self, data: bytes) -> dict:
        """
        Raises:
            ProtocoleError
        """
        return self.load(unpackb(data))

    def dumps(self, data: dict) -> bytes:
        """
        Raises:
            ProtocoleError
        """
        return packb(self.dump(data))


class CmdSerializer:
    def __repr__(self):
        return (
            f"{self.__class__.__name__}("
            f"{self.req_schema.__class__.__name__}, "
            f"{self.rep_noerror_schema.__class__.__name__})"
        )

    def __init__(self, req_schema_cls, rep_schema_cls):
        self.rep_noerror_schema = rep_schema_cls()

        class RepWithErrorSchema(OneOfSchema):
            type_field = "status"
            type_field_remove = False
            fallback_type_schema = ErrorRepSchema
            type_schemas = {"ok": self.rep_noerror_schema}

            def get_obj_type(self, obj):
                try:
                    return obj["status"]
                except (TypeError, KeyError):
                    return "ok"

        RepWithErrorSchema.__name__ = f"ErrorOr{rep_schema_cls.__name__}"

        self.req_schema = req_schema_cls(strict=True)
        self.rep_schema = RepWithErrorSchema(strict=True)

    def req_load(self, data: dict) -> dict:
        """
        Raises:
            ProtocoleError
        """
        try:
            return self.req_schema.load(data).data

        except (ValidationError,) as exc:
            raise InvalidMessageError(exc.messages) from exc

    def req_dump(self, data: dict) -> dict:
        """
        Raises:
            ProtocoleError
        """
        try:
            return self.req_schema.dump(data).data

        except ValidationError as exc:
            raise InvalidMessageError(exc.messages) from exc

    def rep_load(self, data: dict) -> dict:
        """
        Raises:
            ProtocoleError
        """
        try:
            return self.rep_schema.load(data).data

        except ValidationError as exc:
            raise InvalidMessageError(exc.messages) from exc

    def rep_dump(self, data: dict) -> dict:
        """
        Raises:
            ProtocoleError
        """
        try:
            return self.rep_schema.dump(data).data

        except ValidationError as exc:
            raise InvalidMessageError(exc.messages) from exc

    def req_loads(self, data: bytes) -> dict:
        """
        Raises:
            ProtocoleError
        """
        return self.req_load(unpackb(data))

    def req_dumps(self, data: dict) -> bytes:
        """
        Raises:
            ProtocoleError
        """
        return packb(self.req_dump(data))

    def rep_loads(self, data: bytes) -> dict:
        """
        Raises:
            ProtocoleError
        """
        return self.rep_load(unpackb(data))

    def rep_dumps(self, data: dict) -> bytes:
        """
        Raises:
            ProtocoleError
        """
        return packb(self.rep_dump(data))
