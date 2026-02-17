# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from collections.abc import Callable

from .common import (
    AccessToken,
    ApiVersion,
    DateTime,
    DeviceID,
    GreetingAttemptID,
    Handle,
    IndexInt,
    InvitationStatus,
    SizeInt,
    Variant,
    VlobID,
)


class ClientEvent(Variant):
    class ClientStarted:
        device_id: DeviceID

    class ClientStopped:
        device_id: DeviceID

    class Ping:
        ping: str

    class Offline:
        pass

    class Online:
        pass

    class ServerConfigChanged:
        pass

    class WorkspacesSelfListChanged:
        pass

    class WorkspaceLocallyCreated:
        pass

    class WorkspaceWatchedEntryChanged:
        realm_id: VlobID
        entry_id: VlobID

    class WorkspaceOpsOutboundSyncStarted:
        realm_id: VlobID
        entry_id: VlobID

    class WorkspaceOpsOutboundSyncProgress:
        realm_id: VlobID
        entry_id: VlobID
        blocks: IndexInt
        block_index: IndexInt
        blocksize: SizeInt

    class WorkspaceOpsOutboundSyncAborted:
        realm_id: VlobID
        entry_id: VlobID

    class WorkspaceOpsOutboundSyncDone:
        realm_id: VlobID
        entry_id: VlobID

    class WorkspaceOpsInboundSyncDone:
        realm_id: VlobID
        entry_id: VlobID

    class InvitationChanged:
        token: AccessToken
        status: InvitationStatus

    class AsyncEnrollmentUpdated:
        pass

    class GreetingAttemptReady:
        token: AccessToken
        greeting_attempt: GreetingAttemptID

    class GreetingAttemptCancelled:
        token: AccessToken
        greeting_attempt: GreetingAttemptID

    class GreetingAttemptJoined:
        token: AccessToken
        greeting_attempt: GreetingAttemptID

    class TooMuchDriftWithServerClock:
        server_timestamp: DateTime
        client_timestamp: DateTime
        ballpark_client_early_offset: float
        ballpark_client_late_offset: float

    class ExpiredOrganization:
        pass

    class OrganizationNotFound:
        pass

    class InvitationAlreadyUsedOrDeleted:
        pass

    class RevokedSelfUser:
        pass

    class FrozenSelfUser:
        pass

    class WebClientNotAllowedByOrganization:
        pass

    class MustAcceptTos:
        pass

    class IncompatibleServer:
        api_version: ApiVersion
        supported_api_version: list[ApiVersion]

    class ClientErrorResponse:
        error_type: str

    class ServerInvalidResponseStatus:
        status_code: str

    class ServerInvalidResponseContent:
        protocol_decode_error: str

    class InvalidCertificate:
        detail: str

    # class InvalidKeysBundle:
    #     detail: str

    # class InvalidManifest:
    #     detail: str


class OnClientEventCallback(Callable[[Handle, ClientEvent], None]):  # type: ignore[misc]
    ...
