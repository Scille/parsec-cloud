# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from parsec.api.protocol.types import DeviceID, OrganizationID, UserID


class HttpProtocolError(Exception):
    pass


class MissingHeaderError(HttpProtocolError):
    def __init__(self, header: str) -> None:
        self.header = header

    def __str__(self) -> str:
        return f"Missing header `{self.header}`"


class InvalidHeaderValueError(HttpProtocolError):
    def __init__(self, header: str, invalid_value: str) -> None:
        self.header = header
        self.invalid_value = invalid_value

    def __str__(self) -> str:
        return f"Invalid header value for header `{self.header}`"


class AuthenticationProtocolError(Exception):
    pass


class DeviceNotFoundError(AuthenticationProtocolError):
    def __init__(self, organization_id: OrganizationID, device_id: DeviceID) -> None:
        self.organization_id = organization_id
        self.device_id = device_id

    def __str__(self) -> str:
        return f"Could not find device {self.device_id} in organization {self.organization_id}"


class UserIsRevokedError(AuthenticationProtocolError):
    def __init__(self, organization_id: OrganizationID, user_id: UserID) -> None:
        self.organization_id = organization_id
        self.user_id = user_id

    def __str__(self) -> str:
        return f"User {self.user_id} from {self.organization_id} is revoked"


class CannotVerifySignatureError(AuthenticationProtocolError):
    def __init__(self, internal_exception: Exception) -> None:
        self.internal_exception = internal_exception

    def __str__(self) -> str:
        return f"Could not verify the signature, reason: {self.internal_exception}"
