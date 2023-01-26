# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from .common import Variant, ClientHandle


class ClientEvent(Variant):
    class ClientConnectionChanged:
        client: ClientHandle

    class WorkspaceReencryptionNeeded:
        pass

    class WorkspaceReencryptionStarted:
        pass

    class WorkspaceReencryptionEnded:
        pass
