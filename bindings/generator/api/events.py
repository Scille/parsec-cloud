# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Callable

from .common import (
    DateTime,
    Handle,
    IndexInt,
    InvitationStatus,
    InvitationToken,
    GreetingAttemptID,
    SizeInt,
    Variant,
    VlobID,
)


class ClientEvent(Variant):
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
        token: InvitationToken
        status: InvitationStatus

    class GreetingAttemptReady:
        token: InvitationToken
        greeting_attempt: GreetingAttemptID

    class GreetingAttemptCancelled:
        token: InvitationToken
        greeting_attempt: GreetingAttemptID

    class GreetingAttemptJoined:
        token: InvitationToken
        greeting_attempt: GreetingAttemptID

    class TooMuchDriftWithServerClock:
        server_timestamp: DateTime
        client_timestamp: DateTime
        ballpark_client_early_offset: float
        ballpark_client_late_offset: float

    class ExpiredOrganization:
        pass

    class RevokedSelfUser:
        pass

    class MustAcceptTos:
        pass

    class IncompatibleServer:
        detail: str

    # class InvalidKeysBundle:
    #     detail: str

    # class InvalidCertificate:
    #     detail: str

    # class InvalidManifest:
    #     detail: str


class OnClientEventCallback(Callable[[Handle, ClientEvent], None]):  # type: ignore[misc]
    ...
