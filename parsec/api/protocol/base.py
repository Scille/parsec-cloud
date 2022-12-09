# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Any, Dict, Mapping, Sequence, Tuple, Type, TypeVar, Union, cast

from parsec._parsec import DateTime, ProtocolError
from parsec.api.version import ApiVersion
from parsec.serde import (
    BaseSchema,
    MsgpackSerializer,
    SerdePackingError,
    SerdeValidationError,
    fields,
    post_load,
)
from parsec.serde import packb as _packb
from parsec.serde import unpackb as _unpackb
from parsec.serde.schema import OneOfSchemaLegacy
from parsec.utils import BALLPARK_CLIENT_EARLY_OFFSET, BALLPARK_CLIENT_LATE_OFFSET

__all__ = (
    "BaseReqSchema",
    "BaseRepSchema",
    "CmdSerializer",
    "ApiCommandSerializer",
)


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


def packb(data: Mapping[str, Any]) -> bytes:
    return _packb(data, MessageSerializationError)


def unpackb(data: bytes) -> dict[str, Any]:
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


class BaseRepSchema(BaseSchema):
    # Typing is for inheritance overloading
    status: Union[fields.CheckedConstant, fields.String] = fields.CheckedConstant(
        "ok", required=True
    )


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

        self.req_load = self._req_serializer.load
        self.req_dump = self._req_serializer.dump
        self.rep_load = self._rep_serializer.load
        self.rep_dump = self._rep_serializer.dump
        self.req_loads = self._req_serializer.loads
        self.req_dumps = self._req_serializer.dumps
        self.rep_loads = self._rep_serializer.loads
        self.rep_dumps = self._rep_serializer.dumps

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
        from parsec._parsec import AnonymousAnyCmdReq, AuthenticatedAnyCmdReq, InvitedAnyCmdReq

        try:
            return AuthenticatedAnyCmdReq.load(raw)
        except ProtocolError:
            try:
                return AnonymousAnyCmdReq.load(raw)
            except ProtocolError:
                return InvitedAnyCmdReq.load(raw)

    def rep_dumps(self, rep: Any) -> bytes:
        return rep.dump()
