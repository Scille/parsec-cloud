# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import attr
from functools import wraps
from pendulum import DateTime
from typing import Dict, Type, cast, TypeVar, Union, Mapping, Any, Tuple, Sequence

from parsec.utils import BALLPARK_CLIENT_EARLY_OFFSET, BALLPARK_CLIENT_LATE_OFFSET
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


__all__ = ("ProtocolError", "BaseReqSchema", "BaseRepSchema", "CmdSerializer")


class ProtocolError(Exception):
    pass


class InvalidMessageError(SerdeValidationError, ProtocolError):
    pass


class MessageSerializationError(SerdePackingError, ProtocolError):
    pass


def packb(data: Mapping) -> bytes:  # type: ignore[type-arg]
    return _packb(data, MessageSerializationError)


def unpackb(data: bytes) -> dict:  # type: ignore[type-arg]
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


class RequireGreaterTimestampRepSchema(BaseRepSchema):
    status: fields.CheckedConstant = fields.CheckedConstant(
        "require_greater_timestamp", required=True
    )
    strictly_greater_than = fields.DateTime(required=True, allow_none=False)


class TimestampOutOfBallparkRepSchema(BaseRepSchema):
    """This schema has been added to API version 2.4.

    However, it re-uses the `bad_timestamp` status that was used for similar errors in previous backend versions.
    For compatibility purposes, this schema should be compatible with `ErrorRepSchema` in the sense that:
    - an `ErrorRepSchema` with status `bad_timestamp` should be able to deserialize into a `TimestampOutOfBallparkRepSchema`
    - a `TimestampOutOfBallparkRepSchema` should be able to deserialize into an `ErrorRepSchema` with status `bad_timestamp

    New clients who wishes to use those fields should check for their existence first.
    TODO: This backward compatibility should be removed once Parsec < 2.4 support is dropped
    """

    # `bad_timestamp` is kept for backward compatibility,
    # even though `timestamp_out_of_ballpark` would be more explicit
    status: fields.CheckedConstant = fields.CheckedConstant("bad_timestamp", required=True)
    ballpark_client_early_offset = fields.Float(required=False, missing=None)
    ballpark_client_late_offset = fields.Float(required=False, missing=None)
    client_timestamp = fields.DateTime(required=False, missing=None)
    backend_timestamp = fields.DateTime(required=False, missing=None)


class CmdSerializer:
    def __repr__(self) -> str:
        return (
            f"{self.__class__.__name__}("
            f"req_schema={self._req_serializer}, "
            f"rep_schema={self._rep_serializer})"
        )

    @classmethod
    def from_typed(
        cls, req_cls: Type["BaseReq"], rep_clss: Sequence[Type["BaseRep"]]
    ) -> "CmdSerializer":
        assert issubclass(req_cls, BaseReq)
        assert all(issubclass(x, BaseRep) for x in rep_clss)

        rep_schemas = {
            x.SCHEMA_CLS._declared_fields["status"].default: x.SCHEMA_CLS for x in rep_clss
        }
        rep_schema_ok = rep_schemas.pop("ok")
        return cls(
            req_schema_cls=req_cls.SCHEMA_CLS,
            rep_schema_cls=rep_schema_ok,
            extra_error_schemas=rep_schemas,
        )

    def __init__(
        self,
        req_schema_cls: Type[BaseSchema],
        rep_schema_cls: Type[BaseSchema],
        extra_error_schemas: Dict[str, Type[BaseSchema]] = {},
    ):
        self.rep_noerror_schema = rep_schema_cls()
        self.extra_error_schemas = extra_error_schemas

        class RepWithErrorSchema(OneOfSchemaLegacy):
            type_field = "status"
            fallback_type_schema = ErrorRepSchema
            type_schemas = {
                "ok": self.rep_noerror_schema,
                "require_greater_timestamp": RequireGreaterTimestampRepSchema,
                "bad_timestamp": TimestampOutOfBallparkRepSchema,
            }
            type_schemas.update(extra_error_schemas)

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

    def require_greater_timestamp_rep_dump(
        self, strictly_greater_than: DateTime
    ) -> Dict[str, object]:
        return self.rep_dump(
            {"status": "require_greater_timestamp", "strictly_greater_than": strictly_greater_than}
        )

    def timestamp_out_of_ballpark_rep_dump(
        self, backend_timestamp: DateTime, client_timestamp: DateTime
    ) -> Dict[str, object]:
        return self.rep_dump(
            {
                "status": "bad_timestamp",
                "reason": "Timestamp is out of date.",
                "ballpark_client_early_offset": BALLPARK_CLIENT_EARLY_OFFSET,
                "ballpark_client_late_offset": BALLPARK_CLIENT_LATE_OFFSET,
                "backend_timestamp": backend_timestamp,
                "client_timestamp": client_timestamp,
            }
        )


class CmdReqMeta(type):
    BASE_SCHEMA_CLS = BaseReqSchema
    CLS_ATTR_COOKING = attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)

    def __new__(cls, name: str, bases: Tuple[type, ...], nmspc: Dict[str, Any]):
        # Sanity checks
        if "SCHEMA_CLS" not in nmspc:
            raise RuntimeError("Missing attribute `SCHEMA_CLS` in class definition")
        if not issubclass(nmspc["SCHEMA_CLS"], cls.BASE_SCHEMA_CLS):
            raise RuntimeError(f"Attribute `SCHEMA_CLS` must inherit {cls.BASE_SCHEMA_CLS!r}")

        raw_cls = type.__new__(cls, name, bases, nmspc)

        # Sanity checks: class SCHEMA_CLS needs to define parents SCHEMA_CLS fields
        schema_cls_fields = set(nmspc["SCHEMA_CLS"]._declared_fields)
        bases_schema_cls = (base for base in bases if hasattr(base, "SCHEMA_CLS"))
        for base in bases_schema_cls:
            assert (
                base.SCHEMA_CLS._declared_fields.keys()  # type: ignore[attr-defined]
                <= schema_cls_fields
            )

        # Sanity check: attr fields need to be defined in SCHEMA_CLS
        if "__attrs_attrs__" in nmspc:
            assert {att.name for att in nmspc["__attrs_attrs__"]} <= schema_cls_fields

        raw_cls.SERIALIZER = MsgpackSerializer(  # type: ignore[attr-defined]
            nmspc["SCHEMA_CLS"], InvalidMessageError, MessageSerializationError
        )

        return raw_cls


class CmdRepMeta(CmdReqMeta):
    BASE_SCHEMA_CLS = BaseRepSchema


BaseReqTypeVar = TypeVar("BaseReqTypeVar", bound="BaseReq")


class BaseReq(metaclass=CmdReqMeta):
    # Must be overloaded by child classes
    SCHEMA_CLS = BaseReqSchema

    def __eq__(self, other: Any) -> bool:
        if isinstance(other, type(self)):
            return attr.astuple(self).__eq__(attr.astuple(other))
        return NotImplemented

    def evolve(self, **kwargs):
        return attr.evolve(self, **kwargs)

    def dump(self) -> bytes:
        return self.SERIALIZER.dumps(self)  # type: ignore[attr-defined]

    @classmethod
    def load(cls: Type[BaseReqTypeVar], raw: bytes) -> BaseReqTypeVar:
        return cls.SERIALIZER.loads(raw)  # type: ignore[attr-defined]


BaseRepTypeVar = TypeVar("BaseRepTypeVar", bound="BaseRep")


class BaseRep(metaclass=CmdRepMeta):
    # Must be overloaded by child classes
    SCHEMA_CLS = BaseRepSchema

    def __eq__(self, other: Any) -> bool:
        if isinstance(other, type(self)):
            return attr.astuple(self).__eq__(attr.astuple(other))
        return NotImplemented

    def evolve(self, **kwargs):
        return attr.evolve(self, **kwargs)

    def dump(self) -> bytes:
        return self.SERIALIZER.dumps(self)  # type: ignore[attr-defined]

    @classmethod
    def load(cls: Type[BaseReqTypeVar], raw: bytes) -> BaseReqTypeVar:
        return cls.SERIALIZER.loads(raw)  # type: ignore[attr-defined]


def cmd_rep_error_type_factory(
    name: str, status: str, extra_fields: Dict[str, fields.Field] = {}
) -> Type[BaseRep]:
    """
    Shorthand for creating:

        class <name>(BaseRep):
            class SCHEMA_CLS(BaseRepSchema):
                status = fields.CheckedConstant(<status>, required=True)
                [...]  # extra_fields

                @post_load
                def make_obj(self, data):
                    data.pop("status")
                    return <name>(**data)

            status: str
            [...] # extra_fields

    """
    schema_fields = {**extra_fields, "status": fields.CheckedConstant(status, required=True)}

    @post_load
    def make_obj(self, data: Dict[str, Any]) -> Type[BaseRep]:
        data.pop("status")
        return rep_cls(**data)

    schema_cls = type("SCHEMA_CLS", (BaseRepSchema,), {"make_obj": make_obj, **schema_fields})
    rep_cls = attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)(
        type(
            name,
            (BaseRep,),
            {
                "SCHEMA_CLS": schema_cls,
                # Note static type check won't be ablet to check such dynamic code,
                # hence it's fine to use object as generic type here
                **{k: object for k in schema_fields.keys()},
            },
        )
    )
    return rep_cls


def cmd_rep_factory(name: str, *rep_types: Type[BaseRep]):
    assert name.endswith("Rep")

    class RepSchema(OneOfSchemaLegacy):
        type_field = "status"
        type_schemas = {
            x.SCHEMA_CLS._declared_fields["status"].default: x.SCHEMA_CLS for x in rep_types
        }

        def get_obj_type(self, obj: Dict[str, object]) -> str:
            try:
                return cast(str, obj["status"])
            except (TypeError, KeyError):
                return "ok"

    RepSchema.__name__ = f"{name}Schema"

    serializer = MsgpackSerializer(RepSchema, InvalidMessageError, MessageSerializationError)

    @classmethod
    def load(cls, raw: bytes) -> BaseRep:
        return serializer.loads(raw)

    fields = {"load": load, "TYPES": rep_types}
    for rep_type in rep_types:
        assert rep_type.__name__.startswith(name)
        rep_type_name = rep_type.__name__[len(name) :]
        assert rep_type_name not in fields
        fields[rep_type_name] = rep_type

    return type(name, (), fields)


# TODO: temporary hack that should be removed once all cmds are typed, at this point we
# will be able to do this handling directly into `BackendApp._handle_client_websocket_loop`
def api_typed_msg_adapter(rep_cls, req_cls):
    def _api_typed_msg_adapter(fn):
        @wraps(fn)
        async def wrapper(self, client_ctx, msg):
            # Here packb&unpackb should never fail given they are only undoing
            # work we've just done in another layer
            typed_msg = rep_cls.load(_packb(msg))
            typed_rep = await fn(self, client_ctx, typed_msg)
            return _unpackb(typed_rep.dump())

        return wrapper

    return _api_typed_msg_adapter
