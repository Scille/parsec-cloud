# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from typing import Optional
import attr
from functools import wraps
from pendulum import DateTime
from typing import Dict, Type, cast, TypeVar, Mapping, Any, Tuple, Sequence

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


class BaseTypedReqSchema(BaseReqSchema):
    # Defined by CmdReqMeta
    TYPE: Type["BaseReq"]

    # Children must provide a checked constant `cmd` field

    @post_load
    def make_obj(  # type: ignore[misc]
        self, data: Dict[str, Any]
    ) -> "BaseReq":
        return self.TYPE(**data)  # type: ignore[call-arg]


class BaseRepSchema(BaseSchema):
    status = fields.CheckedConstant("ok", required=True)


class BaseTypedRepSchema(BaseRepSchema):
    # Defined by CmdReqMeta
    TYPE: Type["BaseRep"]

    # Children must provide a checked constant `status` field

    @post_load
    def make_obj(  # type: ignore[misc]
        self, data: Dict[str, Any]
    ) -> "BaseRep":
        data.pop("status")
        return self.TYPE(**data)  # type: ignore[call-arg]


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
    """This schema has been added to API version 2.4 (Parsec v2.7.0).

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
    ballpark_client_early_offset = fields.Float(required=False, allow_none=False)
    ballpark_client_late_offset = fields.Float(required=False, allow_none=False)
    client_timestamp = fields.DateTime(required=False, allow_none=False)
    backend_timestamp = fields.DateTime(required=False, allow_none=False)

    @post_load
    def make_obj(self, data: Dict[str, Any]) -> Dict[str, Any]:  # type: ignore[misc]
        # Cannot use `missing=None` with `allow_none=False`
        data.setdefault("ballpark_client_early_offset", None)
        data.setdefault("ballpark_client_late_offset", None)
        data.setdefault("client_timestamp", None)
        data.setdefault("backend_timestamp", None)
        return data


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
                "require_greater_timestamp": RequireGreaterTimestampRepSchema(),
                "bad_timestamp": TimestampOutOfBallparkRepSchema(),
                **{k: v() for k, v in extra_error_schemas.items()},
            }

            def get_obj_type(self, obj: Dict[str, object]) -> str:
                try:
                    return cast(str, obj["status"])
                except (TypeError, KeyError):
                    return "ok"

        RepWithErrorSchema.__name__ = f"ErrorOr{rep_schema_cls.__name__}"

        self._req_serializer = serializer_factory(req_schema_cls)
        self._rep_serializer = serializer_factory(RepWithErrorSchema)

        # WTF is this `_maybe_untype_wrapper` mess ???
        # Command handling in the backend is divided between
        # `BackendApp._handle_client_websocket_loop` that is responsible to dispatch
        # the cmd and the `api_...` functions for actually doing the commands.
        # Currently the two communicate by representing req/rep as dict, we want
        # to replace that by typed objects but this involve all lot of change.
        # Hence this workaround that allow to have an `api_...` function working
        # with typed req/rep while the rest of the application still use the legacy
        # dict interface.
        # Of course once all the `api_...` functions migrated, this hack must be removed ;-)

        def _maybe_untype_wrapper(  # type: ignore[no-untyped-def]
            fn
        ):
            @wraps(fn)
            def wrapper(  # type: ignore[no-untyped-def, misc]
                data
            ):
                if isinstance(data, (BaseReq, BaseRep)):
                    data = data.SERIALIZER.dump(data)

                ret = fn(data)

                if isinstance(ret, (BaseReq, BaseRep)):
                    ret = ret.SERIALIZER.dump(ret)

                return ret

            return wrapper

        self.req_load = _maybe_untype_wrapper(self._req_serializer.load)
        self.req_dump = _maybe_untype_wrapper(self._req_serializer.dump)
        self.rep_load = _maybe_untype_wrapper(self._rep_serializer.load)
        self.rep_dump = _maybe_untype_wrapper(self._rep_serializer.dump)
        self.req_loads = _maybe_untype_wrapper(self._req_serializer.loads)
        self.req_dumps = _maybe_untype_wrapper(self._req_serializer.dumps)
        self.rep_loads = _maybe_untype_wrapper(self._rep_serializer.loads)
        self.rep_dumps = _maybe_untype_wrapper(self._rep_serializer.dumps)

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


class CmdMeta(type):
    CLS_ATTR_COOKING = attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
    BASE_SCHEMA_CLS: type
    DISCRIMINANT_FIELD: str

    def __new__(  # type: ignore[no-untyped-def, misc]
        cls, name: str, bases: Tuple[type, ...], nmspc: Dict[str, Any]
    ):
        # Sanity checks
        if "SCHEMA_CLS" not in nmspc:
            raise RuntimeError("Missing attribute `SCHEMA_CLS` in class definition")
        if not issubclass(nmspc["SCHEMA_CLS"], cls.BASE_SCHEMA_CLS):
            raise RuntimeError(f"Attribute `SCHEMA_CLS` must inherit {cls.BASE_SCHEMA_CLS!r}")

        # Special case if we are defining `BaseReq/BaseRep`
        if nmspc["SCHEMA_CLS"] is not cls.BASE_SCHEMA_CLS:
            # Sanity check: discriminant field should be defined as check constant
            discriminant_field = nmspc["SCHEMA_CLS"]._declared_fields.get(cls.DISCRIMINANT_FIELD)
            if not discriminant_field or not isinstance(discriminant_field, fields.CheckedConstant):
                raise RuntimeError(
                    f"Attribute `SCHEMA_CLS` must contain a `{cls.DISCRIMINANT_FIELD}` field"
                )

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

        # Define attribute `BaseTypedReqSchema.TYPE`
        raw_cls.SCHEMA_CLS.TYPE = raw_cls  # type: ignore[attr-defined]

        return raw_cls


class CmdReqMeta(CmdMeta):
    BASE_SCHEMA_CLS = BaseTypedReqSchema
    DISCRIMINANT_FIELD = "cmd"


class CmdRepMeta(CmdMeta):
    BASE_SCHEMA_CLS = BaseTypedRepSchema
    DISCRIMINANT_FIELD = "status"


BaseReqTypeVar = TypeVar("BaseReqTypeVar", bound="BaseReq")


class BaseReq(metaclass=CmdReqMeta):
    # Must be overloaded by child classes
    SCHEMA_CLS = BaseTypedReqSchema

    def __eq__(  # type: ignore[misc]
        self, other: Any
    ) -> bool:
        if isinstance(other, type(self)):
            return attr.astuple(self).__eq__(attr.astuple(other))
        return NotImplemented

    def evolve(  # type: ignore[no-untyped-def]
        self, **kwargs
    ):
        return attr.evolve(self, **kwargs)

    def dump(self) -> bytes:
        return self.SERIALIZER.dumps(self)  # type: ignore[attr-defined]

    @classmethod
    def load(cls: Type[BaseReqTypeVar], raw: bytes) -> BaseReqTypeVar:
        return cls.SERIALIZER.loads(raw)  # type: ignore[attr-defined]


BaseRepTypeVar = TypeVar("BaseRepTypeVar", bound="BaseRep")


class BaseRep(metaclass=CmdRepMeta):
    # Must be overloaded by child classes
    SCHEMA_CLS = BaseTypedRepSchema

    def __eq__(  # type: ignore[misc]
        self, other: Any
    ) -> bool:
        if isinstance(other, type(self)):
            return attr.astuple(self).__eq__(attr.astuple(other))
        return NotImplemented

    def evolve(  # type: ignore[no-untyped-def]
        self, **kwargs
    ):
        return attr.evolve(self, **kwargs)

    def dump(self) -> bytes:
        return self.SERIALIZER.dumps(self)  # type: ignore[attr-defined]

    @classmethod
    def load(cls: Type[BaseReqTypeVar], raw: bytes) -> BaseReqTypeVar:  # type: ignore[misc]
        return cls.SERIALIZER.loads(raw)  # type: ignore[attr-defined]


def cmd_rep_error_type_factory(  # type: ignore[no-any-unimported]
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
    def make_obj(  # type: ignore[no-untyped-def, misc]
        self, data: Dict[str, Any]
    ) -> Type[BaseRep]:
        data.pop("status")
        return rep_cls(**data)

    schema_cls = type("SCHEMA_CLS", (BaseTypedRepSchema,), {"make_obj": make_obj, **schema_fields})
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


def cmd_rep_factory(  # type: ignore[no-untyped-def]
    name: str, *rep_types: Type[BaseRep]
):
    assert name.endswith("Rep")

    class RepSchema(OneOfSchemaLegacy):
        type_field = "status"
        type_schemas = {
            x.SCHEMA_CLS._declared_fields["status"].default: x.SCHEMA_CLS for x in rep_types
        }

        def get_obj_type(self, obj: Dict[str, object]) -> str:
            return cast(str, obj.get("status"))

    RepSchema.__name__ = f"{name}Schema"

    serializer = MsgpackSerializer(RepSchema, InvalidMessageError, MessageSerializationError)

    @classmethod  # type: ignore[misc]
    def load(cls, raw: bytes) -> Dict[Any, Any]:  # type: ignore[no-untyped-def, misc]
        return serializer.loads(raw)

    fields = {"load": load, "TYPES": rep_types}
    for rep_type in rep_types:
        assert rep_type.__name__.startswith(name)
        rep_type_name = rep_type.__name__[len(name) :]
        assert rep_type_name not in fields
        fields[rep_type_name] = rep_type

    return type(name, (), fields)


def any_cmd_req_factory(name: str, *req_types: Type[BaseReq]):  # type: ignore[no-untyped-def]
    assert name.endswith("Req")

    class AnyCmdReqSchema(OneOfSchemaLegacy):
        type_field = "cmd"
        type_schemas = {
            x.SCHEMA_CLS._declared_fields["cmd"].default: x.SCHEMA_CLS for x in req_types
        }

        def get_obj_type(self, obj: Dict[str, object]) -> Optional[str]:
            return cast(str, obj.get("cmd"))

    AnyCmdReqSchema.__name__ = f"{name}Schema"

    serializer = MsgpackSerializer(AnyCmdReqSchema, InvalidMessageError, MessageSerializationError)

    @classmethod  # type: ignore[misc]
    def load(cls, raw: bytes) -> Dict[Any, Any]:  # type: ignore[no-untyped-def, misc]
        return serializer.loads(raw)

    fields = {"load": load, "TYPES": req_types}
    suffix = "Req"
    for rep_type in req_types:
        assert rep_type.__name__.endswith(suffix)
        rep_type_name = rep_type.__name__[: -len(suffix)]
        assert rep_type_name not in fields
        fields[rep_type_name] = rep_type

    return type(name, (), fields)


# TODO: temporary hack that should be removed once all cmds are typed, at this point we
# will be able to do this handling directly into `BackendApp._handle_client_websocket_loop`
def api_typed_msg_adapter(req_cls, rep_cls):  # type: ignore[no-untyped-def]
    def _api_typed_msg_adapter(fn):  # type: ignore[no-untyped-def]
        @wraps(fn)
        async def wrapper(self, client_ctx, msg):  # type: ignore[no-untyped-def, misc]
            from parsec.api.protocol import AuthenticatedAnyCmdReq

            # Here packb&unpackb should never fail given they are only undoing
            # work we've just done in another layer
            typed_req = AuthenticatedAnyCmdReq.load(_packb(msg))
            assert isinstance(typed_req, req_cls)
            typed_rep = await fn(self, client_ctx, typed_req)
            return _unpackb(typed_rep.dump())

        return wrapper

    return _api_typed_msg_adapter
