# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import anonymous_server_cmds
from parsec.api import api
from parsec.client_context import AnonymousServerClientContext
from parsec.config import BackendConfig


class ScwsComponent:
    def __init__(self, config: BackendConfig) -> None:
        self._config = config

    @api
    async def anonymous_server_scws_service_mutual_challenges(
        self,
        client_ctx: AnonymousServerClientContext,
        req: anonymous_server_cmds.latest.scws_service_mutual_challenges.Req,
    ) -> anonymous_server_cmds.latest.scws_service_mutual_challenges.Rep:
        raise NotImplementedError()
