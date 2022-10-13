# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from enum import Enum
import attr
from functools import partial, wraps
from parsec._parsec import DateTime
from typing import (
    Callable,
    Dict,
    Type,
    Union,
    cast,
    TypeVar,
    Mapping,
    Any,
    Tuple,
    Sequence,
)

from parsec.api.version import ApiVersion
from parsec.utils import BALLPARK_CLIENT_EARLY_OFFSET, BALLPARK_CLIENT_LATE_OFFSET
from parsec.serde import (
    BaseSchema,
    fields,
    post_load,
    BaseSerializer,
    MsgpackSerializer,
    SerdeValidationError,
    SerdePackingError,
    packb as _packb,
    unpackb as _unpackb,
)
from parsec.serde.schema import OneOfSchemaLegacy


__all__ = ("ProtocolError", "BaseReqSchema", "BaseRepSchema", "CmdSerializer")

ClientType = Enum(
    "ClientType", "AUTHENTICATED INVITED ANONYMOUS APIV1_ANONYMOUS APIV1_ADMINISTRATION"
)


class ProtocolError(Exception):
    pass


class InvalidMessageError(SerdeValidationError, ProtocolError):
    pass


class MessageSerializationError(SerdePackingError, ProtocolError):
    pass


class IncompatibleAPIVersionsError(ProtocolError):
    def __init__(
        self,
        backend_versions: Sequence[ApiVersion],
        client_versions: Sequence[ApiVersion] = [],
    ):
        self.client_versions = client_versions
        self.backend_versions = backend_versions
        client_versions_str = "{" + ", ".join(map(str, client_versions)) + "}"
        backend_versions_str = "{" + ", ".join(map(str, backend_versions)) + "}"
        self.message = (
            f"No overlap between client API versions {client_versions_str} "
            f"and backend API versions {backend_versions_str}"
        )

    def __str__(self) -> str:
        return self.message


def settle_compatible_versions(
    backend_versions: Sequence[ApiVersion], client_versions: Sequence[ApiVersion]
) -> Tuple[ApiVersion, ApiVersion]:
    """
    Try to find compatible API version between the server and the client.

    raise an exception if no compatible version is found
    """
    # Try to use the newest version first
    for client_version in reversed(sorted(client_versions)):
        # No need to compare `revision` because only `version` field breaks compatibility
        compatible_backend_versions = filter(
            lambda bv: client_version.version == bv.version, backend_versions
        )
        backend_version = next(compatible_backend_versions, None)

        if backend_version:
            return backend_version, client_version
    raise IncompatibleAPIVersionsError(backend_versions, client_versions)


def packb(data: Mapping) -> bytes:  # type: ignore[type-arg]
    return _packb(data, MessageSerializationError)


def unpackb(data: bytes) -> dict:  # type: ignore[type-arg]
    return _unpackb(data, MessageSerializationError)


def serializer_factory(schema_cls: Type[BaseSchema]) -> MsgpackSerializer:
    return MsgpackSerializer(schema_cls, InvalidMessageError, MessageSerializationError)


T = TypeVar("T")


class BaseReqSchema(BaseSchema):
    # Typing is for inheritance overloading
    cmd: Union[fields.String, fields.CheckedConstant] = fields.String(required=True)

    @post_load
    def _drop_cmd_field(self, item: Dict[str, T]) -> Dict[str, T]:
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
    def make_obj(self, data: Dict[str, Any]) -> "BaseReq":
        return self.TYPE(**data)  # type: ignore[call-arg]


class BaseRepSchema(BaseSchema):
    # Typing is for inheritance overloading
    status: Union[fields.CheckedConstant, fields.String] = fields.CheckedConstant(
        "ok", required=True
    )


class BaseTypedRepSchema(BaseRepSchema):
    # Defined by CmdReqMeta
    TYPE: Type["BaseRep"]

    # Children must provide a checked constant `status` field

    @post_load
    def make_obj(self, data: Dict[str, Any]) -> "BaseRep":
        data.pop("status")
        return self.TYPE(**data)  # type: ignore[call-arg]


class ErrorRepSchema(BaseRepSchema):
    status = fields.String(required=True)
    reason = fields.String(allow_none=True)
    # TODO: should errors be better checked ?
    errors = fields.Dict(allow_none=True)


class RequireGreaterTimestampRepSchema(BaseRepSchema):
    status = fields.CheckedConstant("require_greater_timestamp", required=True)
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
    def make_obj(self, data: Dict[str, Any]) -> Dict[str, Any]:
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

        rep_schemas: Dict[str, Type[BaseSchema]] = {}
        for rep_cls in rep_clss:
            rep_cls_status = rep_cls.SCHEMA_CLS._declared_fields["status"].default
            assert isinstance(rep_cls_status, str)
            rep_schemas[rep_cls_status] = rep_cls.SCHEMA_CLS

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

        def _maybe_untype_wrapper(fn):  # type: ignore[no-untyped-def]
            @wraps(fn)
            def wrapper(data):  # type: ignore[no-untyped-def, misc]
                if isinstance(data, (BaseReq, BaseRep)):
                    data = data.SERIALIZER.dump(data)

                ret = fn(data)

                if isinstance(ret, (BaseReq, BaseRep)):
                    ret = ret.SERIALIZER.dump(ret)

                return ret

            return wrapper

        self.req_load: Callable[[Dict[str, Any]], Dict[str, Any]] = _maybe_untype_wrapper(
            self._req_serializer.load
        )
        self.req_dump: Callable[[Dict[str, Any]], Dict[str, Any]] = _maybe_untype_wrapper(
            self._req_serializer.dump
        )
        self.rep_load: Callable[[Dict[str, Any]], Dict[str, Any]] = _maybe_untype_wrapper(
            self._rep_serializer.load
        )
        self.rep_dump: Callable[[Dict[str, Any]], Dict[str, Any]] = _maybe_untype_wrapper(
            self._rep_serializer.dump
        )
        self.req_loads: Callable[[bytes], Dict[str, Any]] = _maybe_untype_wrapper(
            self._req_serializer.loads
        )
        self.req_dumps: Callable[[Dict[str, Any]], bytes] = _maybe_untype_wrapper(
            self._req_serializer.dumps
        )
        self.rep_loads: Callable[[bytes], Dict[str, Any]] = _maybe_untype_wrapper(
            self._rep_serializer.loads
        )
        self.rep_dumps: Callable[[Dict[str, Any]], bytes] = _maybe_untype_wrapper(
            self._rep_serializer.dumps
        )

    def require_greater_timestamp_rep_dump(
        self, strictly_greater_than: DateTime
    ) -> Dict[str, object]:
        return self.rep_dump(
            {
                "status": "require_greater_timestamp",
                "strictly_greater_than": strictly_greater_than,
            }
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
    SERIALIZER: BaseSerializer

    def __eq__(self, other: Any) -> bool:
        if isinstance(other, type(self)):
            return attr.astuple(self).__eq__(attr.astuple(other))
        return NotImplemented

    def evolve(self, **kwargs):  # type: ignore[no-untyped-def]
        return attr.evolve(self, **kwargs)

    def dump(self) -> bytes:
        return self.SERIALIZER.dumps(self)

    @classmethod
    def load(cls: Type[BaseReqTypeVar], raw: bytes) -> BaseReqTypeVar:
        return cls.SERIALIZER.loads(raw)


BaseRepTypeVar = TypeVar("BaseRepTypeVar", bound="BaseRep")


class BaseRep(metaclass=CmdRepMeta):
    # Must be overloaded by child classes
    SCHEMA_CLS = BaseTypedRepSchema
    SERIALIZER: BaseSerializer

    def __eq__(self, other: Any) -> bool:
        if isinstance(other, type(self)):
            return attr.astuple(self).__eq__(attr.astuple(other))
        return NotImplemented

    def evolve(self, **kwargs):  # type: ignore[no-untyped-def]
        return attr.evolve(self, **kwargs)

    def dump(self) -> bytes:
        return self.SERIALIZER.dumps(self)

    @classmethod
    def load(cls: Type[BaseRepTypeVar], raw: bytes) -> BaseRepTypeVar:
        return cls.SERIALIZER.loads(raw)


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
    schema_fields: Dict[str, fields.Field] = {
        **extra_fields,
        "status": fields.CheckedConstant(status, required=True),
    }

    @post_load
    def make_obj(self: BaseTypedRepSchema, data: Dict[str, Any]) -> BaseRep:
        data.pop("status")
        return rep_cls(**data)

    schema_cls = type("SCHEMA_CLS", (BaseTypedRepSchema,), {"make_obj": make_obj, **schema_fields})  # type: ignore[arg-type]
    rep_cls = attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)(
        type(
            name,
            (BaseRep,),
            {
                "SCHEMA_CLS": schema_cls,
                # Note static type check won't be able to check such dynamic code,
                # hence it's fine to use object as generic type here
                **{k: object for k in schema_fields.keys()},
            },
        )
    )
    return rep_cls


def cmd_rep_factory(name: str, *rep_types: Type[BaseRep]):  # type: ignore[no-untyped-def]
    assert name.endswith("Rep")

    rep_schemas: Dict[Union[Enum, str], Union[BaseSchema, Type[BaseSchema]]] = {}
    for rep_type in rep_types:
        status = rep_type.SCHEMA_CLS._declared_fields["status"].default
        assert isinstance(status, (str, Enum))
        rep_schemas[status] = rep_type.SCHEMA_CLS

    class RepSchema(OneOfSchemaLegacy):
        type_field = "status"
        type_schemas = rep_schemas

        def get_obj_type(self, obj: Dict[str, object]) -> str:
            return cast(str, obj.get("status"))

    RepSchema.__name__ = f"{name}Schema"

    serializer = MsgpackSerializer(RepSchema, InvalidMessageError, MessageSerializationError)

    # Define the `CmdRep.load` classmethod
    load_meth = partial(serializer.loads)

    fields = {"load": load_meth, "TYPES": rep_types}
    for rep_type in rep_types:
        assert rep_type.__name__.startswith(name)
        rep_type_name = rep_type.__name__[len(name) :]
        assert rep_type_name not in fields
        fields[rep_type_name] = rep_type

    return type(name, (), fields)


# TODO: temporary hack that should be removed once all cmds are typed, at this point we
# will be able to do this handling directly into `BackendApp._handle_client_websocket_loop`
def api_typed_msg_adapter(req_cls, rep_cls):  # type: ignore[no-untyped-def]
    def _api_typed_msg_adapter(fn):  # type: ignore[no-untyped-def]
        @wraps(fn)
        async def wrapper(self, client_ctx, msg):  # type: ignore[no-untyped-def, misc]
            # Here packb&unpackb should never fail given they are only undoing
            # work we've just done in another layer
            if client_ctx.TYPE == ClientType.INVITED:
                from parsec.api.protocol import InvitedAnyCmdReq

                typed_req = InvitedAnyCmdReq.load(_packb(msg))
            else:
                from parsec.api.protocol import AuthenticatedAnyCmdReq

                typed_req = AuthenticatedAnyCmdReq.load(_packb(msg))

            assert isinstance(typed_req, req_cls)
            typed_rep = await fn(self, client_ctx, typed_req)
            return _unpackb(typed_rep.dump())

        return wrapper

    return _api_typed_msg_adapter


class ApiCommandSerializer:
    def __init__(self, req_schema: Any, rep_schema: Any) -> None:
        self.req_schema = req_schema
        self.rep_schema = rep_schema

    def req_dumps(self, req: dict[str, Any]) -> bytes:
        if "cmd" in req:
            req.pop("cmd")

        return self.req_schema(**req).dump()

    def rep_loads(self, raw: bytes) -> Any:
        return self.rep_schema.load(raw)

    # Temporary Used for generate_data
    def req_loads(self, raw: bytes) -> Any:
        from parsec._parsec import AuthenticatedAnyCmdReq, InvitedAnyCmdReq

        try:
            return AuthenticatedAnyCmdReq.load(raw)
        except ProtocolError:
            return InvitedAnyCmdReq.load(raw)

    def rep_dumps(self, rep: Any) -> bytes:
        return rep.dump()
