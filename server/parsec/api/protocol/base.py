# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from typing import Any, Mapping, Sequence, Type

from parsec._parsec import ApiVersion, ProtocolError
from parsec.serde import BaseSchema, MsgpackSerializer, SerdePackingError, SerdeValidationError
from parsec.serde import packb as _packb
from parsec.serde import unpackb as _unpackb

__all__ = ("ApiCommandSerializer",)


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
) -> tuple[ApiVersion, ApiVersion]:
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


class ApiCommandSerializer:
    def __init__(self, req_type: Any, rep_type: Any) -> None:
        self.req_type = req_type
        self.rep_type = rep_type

    def req_dumps(self, req: dict[str, Any]) -> bytes:
        if "cmd" in req:
            req.pop("cmd")

        return self.req_type(**req).dump()

    def rep_loads(self, raw: bytes) -> Any:
        return self.rep_type.load(raw)

    # Temporary Used for generate_data
    def req_loads(self, raw: bytes) -> Any:
        from parsec._parsec import anonymous_cmds, authenticated_cmds, invited_cmds

        try:
            return authenticated_cmds.latest.AnyCmdReq.load(raw)
        except ProtocolError:
            try:
                return anonymous_cmds.latest.AnyCmdReq.load(raw)
            except ProtocolError:
                return invited_cmds.latest.AnyCmdReq.load(raw)

    def rep_dumps(self, rep: Any) -> bytes:
        return rep.dump()
