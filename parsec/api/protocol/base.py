# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from parsec.serde import (
    BaseSchema,
    fields,
    post_load,
    MsgpackSerializer,
    SerdeValidationError,
    SerdePackingError,
    packb as _packb,
    unpackb as _unpackb,
)


from parsec.serde.schema import OneOfSchemaLegacy

from typing import Dict, Type, cast, TypeVar, Union


__all__ = ("ProtocolError", "BaseReqSchema", "BaseRepSchema", "CmdSerializer")


class ProtocolError(Exception):
    pass


class InvalidMessageError(SerdeValidationError, ProtocolError):
    pass


class MessageSerializationError(SerdePackingError, ProtocolError):
    pass


def packb(data: Dict[str, object]) -> bytes:
    return _packb(data, MessageSerializationError)


def unpackb(data: bytes) -> Dict[str, object]:
    return _unpackb(data, MessageSerializationError)


def serializer_factory(schema_cls: Type[BaseSchema]) -> MsgpackSerializer:
    return MsgpackSerializer(schema_cls, InvalidMessageError, MessageSerializationError)


T = TypeVar("T")


class BaseReqSchema(BaseSchema):
    cmd = fields.String(required=True)

    @post_load
    def _drop_cmd_field(self, item: Dict[str, T]) -> Dict[str, T]:  # type: ignore[misc]
        if self.drop_cmd_field:
            item.pop("cmd")
        return item

    def __init__(self, drop_cmd_field: bool = True, **kwargs: object):
        super().__init__(**kwargs)
        self.drop_cmd_field = drop_cmd_field


class BaseRepSchema(BaseSchema):
    status: Union[str, fields.CheckedConstant] = fields.CheckedConstant("ok", required=True)


class ErrorRepSchema(BaseRepSchema):
    status = fields.String(required=True)
    reason = fields.String(allow_none=True)
    # TODO: should errors be better checked ?
    errors = fields.Dict(allow_none=True)


class CmdSerializer:
    def __repr__(self) -> str:
        return (
            f"{self.__class__.__name__}("
            f"req_schema={self._req_serializer}, "
            f"rep_schema={self._rep_serializer})"
        )

    def __init__(self, req_schema_cls: Type[BaseSchema], rep_schema_cls: Type[BaseSchema]):
        self.rep_noerror_schema = rep_schema_cls()

        class RepWithErrorSchema(OneOfSchemaLegacy):
            type_field = "status"
            fallback_type_schema = ErrorRepSchema
            type_schemas = {"ok": self.rep_noerror_schema}

            def get_obj_type(self, obj: Dict[str, object]) -> str:
                try:
                    return cast(str, obj["status"])
                except (TypeError, KeyError):
                    return "ok"

        RepWithErrorSchema.__name__ = f"ErrorOr{rep_schema_cls.__name__}"

        self._req_serializer = serializer_factory(req_schema_cls)
        self._rep_serializer = serializer_factory(RepWithErrorSchema)

        self.req_load = self._req_serializer.load
        self.req_dump = self._req_serializer.dump
        self.rep_load = self._rep_serializer.load
        self.rep_dump = self._rep_serializer.dump
        self.req_loads = self._req_serializer.loads
        self.req_dumps = self._req_serializer.dumps
        self.rep_loads = self._rep_serializer.loads
        self.rep_dumps = self._rep_serializer.dumps
