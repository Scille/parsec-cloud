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
    async def anonymous_server_scws_service_certificate(
        self,
        client_ctx: AnonymousServerClientContext,
        req: anonymous_server_cmds.latest.scws_service_certificate.Req,
    ) -> anonymous_server_cmds.latest.scws_service_certificate.Rep:
        cert = self._config.scws_service_certificate
        if cert:
            return anonymous_server_cmds.latest.scws_service_certificate.RepOk(cert)
        else:
            return anonymous_server_cmds.latest.scws_service_certificate.RepNotAvailable()

    @api
    async def anonymous_server_scws_service_challenge(
        self,
        client_ctx: AnonymousServerClientContext,
        req: anonymous_server_cmds.latest.scws_service_challenge.Req,
    ) -> anonymous_server_cmds.latest.scws_service_challenge.Rep:
        raise NotImplementedError()
